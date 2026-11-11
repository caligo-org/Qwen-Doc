//! Rust port of the MPDocBench Formula CDM metric: `metrics/cdm_metric.py`,
//! `metrics/cdm/modules/{latex2bbox_color,latex_processor,visual_matcher}.py`,
//! `metrics/cdm/modules/tokenize_latex/tokenize_latex.py` and the per-sample part of
//! `call_CDM` in `metrics/cal_metric.py`.
//!
//! External programs are spawned exactly as the Python code spawns them (through
//! `/bin/sh -c`): `node preprocess_formula.js normalize` / `node preprocess_tabular.js
//! tokenize` (the toolkit's JS with its vendored KaTeX), `perl` (tabular inputs only),
//! `xelatex` and ImageMagick `magick` (which calls Ghostscript). They are looked up on
//! PATH; the toolchain on PATH decides the renders, as for the Python scorer.
//!
//! Thread safety: each sample works in `output_root/temp_dir_{gt,pred}_{idx}` and writes
//! only files named after its idx, like the Python worker; no global state is mutated.

mod cdm;
mod latex_processor;
mod lsap;
mod matcher;
mod pil;
mod py;
mod render;
mod rng;
mod tokenize;

use std::path::Path;

pub use cdm::EvalResult;

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
pub fn evaluate(gt: &str, pred: &str, img_id: &str, output_root: &Path) -> Result<EvalResult, Error> {
    let root = output_root.to_str().ok_or(Error::BadPath)?;
    cdm::evaluate(root, gt, pred, img_id).map_err(|e| Error::Raised(e.0))
}

/// One sample of `call_CDM.evaluate`: the cleaning and `CDM.evaluate(...)["F1_score"]`
/// of `_process_single_cdm_sample`, with `img_id = str(idx)`. `output_root` is used as
/// the Python code uses it (relative paths resolve against the current directory).
/// `marathon` selects the MarathonDocBench patch, which scores an empty cleaned
/// prediction as int 0 without rendering. Err where Python raises out of evaluate (the
/// caller retries once, then records 0.0).
pub fn cdm_sample(idx: usize, gt: &str, pred: &str, output_root: &Path, marathon: bool) -> Result<CdmScore, Error> {
    let (gt, pred) = clean(gt, pred);
    let score = if marathon && py::strip(&pred).is_empty() {
        Score::Int(0)
    } else {
        match evaluate(&gt, &pred, &idx.to_string(), output_root)? {
            EvalResult::Zero => Score::Int(0),
            EvalResult::Scores { f1, .. } => Score::Float(f1),
        }
    };
    Ok(CdmScore { gt, pred, score })
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
