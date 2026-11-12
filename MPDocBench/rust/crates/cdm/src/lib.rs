//! Rust port of the MPDocBench Formula CDM metric: `metrics/cdm_metric.py`,
//! `metrics/cdm/modules/{latex2bbox_color,latex_processor,visual_matcher}.py`,
//! `metrics/cdm/modules/tokenize_latex/tokenize_latex.py` and the per-sample part of
//! `call_CDM` in `metrics/cal_metric.py`.
//!
//! By default ([`Backend::native`]) nothing outside the process runs:
//! - the KaTeX based tokenizers (`preprocess_formula.js normalize`,
//!   `preprocess_tabular.js tokenize` with the toolkit's vendored KaTeX) are ported to
//!   Rust ([`katex`]), output identical to node's; perl's `hskip` rewrite is a regex;
//! - the coloured formula is compiled with Tectonic's XeTeX engine linked in as a
//!   library (`xelatex` before), TeX support files from Tectonic's cached bundle;
//! - the PDF is rasterised at 200 dpi with hayro (`magick`/Ghostscript before).
//!
//! Renders therefore differ slightly from the Python toolkit's (another TeX Live
//! snapshot, another rasteriser). [`Backend::external`] keeps the original external
//! programs, spawned exactly as the Python code spawns them (through `/bin/sh -c`, looked
//! up on PATH): `node`, `perl`, `xelatex` and ImageMagick `magick`. `CDM_BACKEND` and
//! the per-stage variables of [`Backend::from_env`] select them at run time.
//!
//! Thread safety: each sample works in `output_root/temp_dir_{gt,pred}_{idx}` and writes
//! only files named after its idx, like the Python worker; no global state is mutated
//! (immutable lookup tables are built once). Tectonic serialises its engine runs
//! internally (the XeTeX engine has global state), so native renders of concurrent
//! samples queue on one lock.

mod cdm;
pub mod katex;
mod latex_processor;
mod lsap;
mod matcher;
mod pil;
mod py;
mod raster;
mod render;
mod rng;
mod tex;
mod tokenize;

use std::path::Path;

pub use cdm::EvalResult;
pub use tokenize::Tokenizer;

/// LaTeX -> PDF
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tex {
    /// Tectonic's XeTeX engine in-process
    Tectonic,
    /// `xelatex -interaction=nonstopmode` on PATH
    Xelatex,
}

/// PDF -> pixels
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raster {
    /// hayro in-process
    Native,
    /// `magick -density 200 ... -alpha set` on PATH
    Magick,
}

/// Which implementation runs each external stage of the CDM pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Backend {
    pub tokenizer: Tokenizer,
    pub tex: Tex,
    pub raster: Raster,
    /// Coverage (of 255) above which the native rasteriser paints a pixel.
    pub coverage_threshold: u8,
}

impl Backend {
    /// Everything in-process (the default).
    pub fn native() -> Backend {
        Backend { tokenizer: Tokenizer::Native, tex: Tex::Tectonic, raster: Raster::Native, coverage_threshold: raster::COVERAGE_THRESHOLD }
    }

    /// The Python toolkit's external programs: node, perl, xelatex, magick.
    pub fn external() -> Backend {
        Backend { tokenizer: Tokenizer::External, tex: Tex::Xelatex, raster: Raster::Magick, coverage_threshold: raster::COVERAGE_THRESHOLD }
    }

    /// `CDM_BACKEND=native|external` (default native), then per stage
    /// `CDM_TOKENIZER=native|node`, `CDM_TEX=tectonic|xelatex`, `CDM_RASTER=native|magick`
    /// and `CDM_RASTER_THRESHOLD=0..255`.
    pub fn from_env() -> Backend {
        let var = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let mut b = match var("CDM_BACKEND").as_deref() {
            Some("external") => Backend::external(),
            _ => Backend::native(),
        };
        match var("CDM_TOKENIZER").as_deref() {
            Some("native") => b.tokenizer = Tokenizer::Native,
            Some("node") | Some("external") => b.tokenizer = Tokenizer::External,
            _ => {}
        }
        match var("CDM_TEX").as_deref() {
            Some("tectonic") => b.tex = Tex::Tectonic,
            Some("xelatex") => b.tex = Tex::Xelatex,
            _ => {}
        }
        match var("CDM_RASTER").as_deref() {
            Some("native") => b.raster = Raster::Native,
            Some("magick") => b.raster = Raster::Magick,
            _ => {}
        }
        if let Some(t) = var("CDM_RASTER_THRESHOLD").and_then(|t| t.parse().ok()) {
            b.coverage_threshold = t;
        }
        b
    }
}

/// A per-sample score with the Python type it would have.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Score {
    /// Python `int` (0 from the `except:` branch of `CDM.evaluate`, or the MarathonDocBench
    /// short-circuit for empty predictions)
    Int(i64),
    /// Python `float` (`F1_score`, rounded to 3 digits)
    Float(f64),
}

impl Score {
    pub fn as_f64(&self) -> f64 {
        match *self {
            Score::Int(i) => i as f64,
            Score::Float(f) => f,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CdmScore {
    /// gt after the cleaning in `_process_single_cdm_sample`
    pub gt: String,
    /// pred after the cleaning in `_process_single_cdm_sample`
    pub pred: String,
    pub score: Score,
}

#[derive(Debug, Clone)]
pub enum Error {
    /// An exception escaped `CDM.evaluate` (Python: the sample fails, is retried
    /// sequentially, and finally scores 0.0).
    Raised(String),
    /// The output root is not valid UTF-8.
    BadPath,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Raised(s) => write!(f, "{s}"),
            Error::BadPath => write!(f, "output_root is not valid UTF-8"),
        }
    }
}

impl std::error::Error for Error {}

fn strip_dollars(s: &str) -> &str {
    // .lstrip("$$").rstrip("$$").strip(), then .lstrip("$").rstrip("$").strip()
    let s = py::strip(s.trim_start_matches('$').trim_end_matches('$'));
    py::strip(s.trim_start_matches('$').trim_end_matches('$'))
}

/// `re.sub(r'^OPEN|CLOSE$', '', s.strip()).strip()` for a stripped string
fn strip_delims(s: &str, open: &str, close: &str) -> String {
    let s = py::strip(s);
    let mut start = 0;
    let mut end = s.len();
    if s.starts_with(open) {
        start = open.len();
    }
    // `$` matches at the very end (s is stripped, so there is no trailing newline); the
    // suffix match must not overlap the prefix match.
    if s.ends_with(close) && end - close.len() >= start {
        end -= close.len();
    }
    py::strip(&s[start..end]).to_string()
}

/// The gt / pred cleaning of `_process_single_cdm_sample`.
pub fn clean(gt: &str, pred: &str) -> (String, String) {
    let gt = strip_dollars(gt);
    let pred = pred.split("```latex").last().unwrap();
    let pred = pred.split("```").next().unwrap();
    let pred = strip_dollars(pred);
    let gt = strip_delims(gt, "\\[", "\\]");
    let pred = strip_delims(pred, "\\[", "\\]");
    let gt = strip_delims(&gt, "\\(", "\\)");
    let pred = strip_delims(&pred, "\\(", "\\)");
    (gt, pred)
}

/// `CDM(output_root).evaluate(gt, pred, img_id)` on already cleaned strings.
pub fn evaluate(gt: &str, pred: &str, img_id: &str, output_root: &Path, backend: &Backend) -> Result<EvalResult, Error> {
    let root = output_root.to_str().ok_or(Error::BadPath)?;
    cdm::evaluate(root, gt, pred, img_id, backend).map_err(|e| Error::Raised(e.0))
}

/// One sample of `call_CDM.evaluate`: the cleaning and `CDM.evaluate(...)["F1_score"]`
/// of `_process_single_cdm_sample`, with `img_id = str(idx)`. `output_root` is used as
/// the Python code uses it (relative paths resolve against the current directory).
/// `marathon` selects the MarathonDocBench patch, which scores an empty cleaned
/// prediction as int 0 without rendering. Err where Python raises out of evaluate (the
/// caller retries once, then records 0.0). The backend comes from the environment
/// ([`Backend::from_env`]).
pub fn cdm_sample(idx: usize, gt: &str, pred: &str, output_root: &Path, marathon: bool) -> Result<CdmScore, Error> {
    cdm_sample_with(idx, gt, pred, output_root, marathon, &Backend::from_env())
}

/// [`cdm_sample`] with an explicit backend.
pub fn cdm_sample_with(
    idx: usize,
    gt: &str,
    pred: &str,
    output_root: &Path,
    marathon: bool,
    backend: &Backend,
) -> Result<CdmScore, Error> {
    let (gt, pred) = clean(gt, pred);
    let score = if marathon && py::strip(&pred).is_empty() {
        Score::Int(0)
    } else {
        match evaluate(&gt, &pred, &idx.to_string(), output_root, backend)? {
            EvalResult::Zero => Score::Int(0),
            EvalResult::Scores { f1, .. } => Score::Float(f1),
        }
    };
    Ok(CdmScore { gt, pred, score })
}

/// Test hook: `normalize_latex` + the `token_add_color_RGB` loop + colour substitution of
/// `latex2bbox_color` on an already tokenized string. Returns (rgb_latex, token_list).
#[doc(hidden)]
pub fn __normalize_and_color(latex: &str) -> Result<(String, Vec<String>), String> {
    render::normalize_and_color(latex).map_err(|e| e.0)
}

/// Test hook: the coloured TeX document for `latex` and its token colour count.
#[doc(hidden)]
pub fn __tex_document(latex: &str) -> Result<(String, usize), String> {
    render::tex_document(latex).map_err(|e| e.0)
}

/// Test hook: Tectonic compile (None: no PDF).
#[doc(hidden)]
pub fn __compile(tex: &str) -> Result<Option<Vec<u8>>, String> {
    tex::compile(tex)
}

/// Test hook: native rasterisation, RGBA pixels.
#[doc(hidden)]
pub fn __rasterize(pdf: &[u8], threshold: u8) -> Result<(usize, usize, Vec<[u8; 4]>), String> {
    let o = raster::rasterize(pdf, threshold).map_err(|e| e.0)?;
    match o.px {
        pil::Decoded::Rgba(v) => Ok((o.w, o.h, v)),
        _ => unreachable!(),
    }
}

/// Test hook: the 5800 token colours.
#[doc(hidden)]
pub fn __colors() -> Vec<[u8; 3]> {
    render::gen_color_list(5800, 15)
}

/// Test hook: the directory of the toolkit's tokenizer scripts.
#[doc(hidden)]
pub fn __tokenize_dir() -> &'static str {
    tokenize::tokenize_dir()
}

/// Test hook: the stdin text node gets for `latex` in `latex2bbox_color` (after the
/// `\n` / `\%` replacements and `tokenize_latex`'s pre-processing; perl's rewrite applied
/// for tabular input), and which script reads it. None for an empty string.
#[doc(hidden)]
pub fn __node_stdin(latex: &str) -> Option<(katex::Script, String)> {
    let latex = latex.replace('\n', " ").replace("\\%", "<PERCENTAGETOKEN>");
    if latex.is_empty() {
        return None;
    }
    Some(if latex.contains("tabular") {
        (katex::Script::TabularTokenize, tokenize::hskip_rewrite(&tokenize::tabular_pre(&latex).1))
    } else {
        (katex::Script::FormulaNormalize, tokenize::formula_prepre(&latex))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleaning() {
        assert_eq!(clean("$$ x^2 $$", "```latex\n$$y$$\n```"), ("x^2".into(), "y".into()));
        assert_eq!(clean("\\[a\\]", "\\(b\\)"), ("a".into(), "b".into()));
        assert_eq!(clean("\\]", "\\["), ("".into(), "".into()));
        assert_eq!(clean("$\\[a$", "x```"), ("a".into(), "x".into()));
    }
}
