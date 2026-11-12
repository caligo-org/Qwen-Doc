//! Batched rendering: many formulas typeset in one Tectonic run, one page each.
//!
//! Loading the template's preamble (fontspec, xeCJK, amsmath, ...) is most of the cost
//! of a single-formula render. A batch document has the template preamble once, then
//! per formula exactly the template's body (`\begin{displaymath} ... \end{displaymath}`)
//! inside `\begingroup ... \endgroup`, followed by `\clearpage`. Formulas are batched
//! only with formulas of the same paper size, so the page geometry is the template's.
//! Each page is then rasterised and boxed exactly as a single render's page.
//!
//! The TeX log tells which page belongs to which formula and whether TeX's state is
//! back to where a fresh document would be: before each formula `\immediate\write16`
//! logs its index and the page counter, after its `\clearpage` the page counter, the
//! group level and the conditional level. A formula's page is used if both markers are
//! present, it made exactly one page and TeX is back at group level 0 and conditional
//! level 0. TeX errors inside a formula (error recovery is local to the formula: the
//! same recovery happens in the single render) do not prevent that; with
//! `CDM_BATCH_KEEP_ERRORS=0` such formulas are rendered alone instead.
//!
//! Formulas after one that leaves an unclean state go into a fresh batch. A formula
//! that stops the run ("That makes 100 errors": TeX counts errors per paragraph, and
//! every formula is its own paragraph in both documents, so its single render stops
//! too, before shipping its page) is recorded as leaving no PDF, as its single render
//! does; the formulas before it are typeset again without it (a stopped run writes no
//! PDF) and the ones after it go into the next batch. Anything else (no closing marker
//! for another reason, several pages, a failed raster) is left without output files
//! and rendered alone by the per-sample evaluation: a safety net.

use std::collections::BTreeMap;

use crate::cdm::{mkdirs, prepare_directories, NoOutput};
use crate::py::{path_join, PyResult};
use crate::render::{gen_color_list, output_paths, prepare, template_body, template_head, write_boxes, Prepared};
use crate::{raster, tex};

/// One formula of a batch (a gt or pred side of a sample).
struct Job {
    subset: &'static str,
    output_path: String,
    basename: String,
    prep: Prepared,
}

/// Counts of what happened to the formulas handed to [`render_batched`].
#[derive(Debug, Default, Clone, Copy)]
pub struct BatchStats {
    /// formulas that went into a batch
    pub batched: usize,
    /// formulas whose batch page was used
    pub from_batch: usize,
    /// formulas whose single render is known to leave no PDF (too many TeX errors)
    pub no_output: usize,
    /// formulas left to the single render (safety net)
    pub fallback: usize,
    /// Tectonic runs for batches
    pub tex_runs: usize,
    /// fallbacks by reason: a marker missing (fatal stop, runaway), not exactly one page,
    /// group / conditional level not 0 afterwards, a TeX error, other (raster / write)
    pub fb_markers: usize,
    pub fb_pages: usize,
    pub fb_unclean: usize,
    pub fb_error: usize,
    pub fb_other: usize,
}

#[derive(Debug, Default, Clone)]
struct Marks {
    begin_page: Option<i64>,
    end: Option<(i64, i64, i64)>,
    error: bool,
}

/// Parses the batch log: per formula position, its markers and whether an error line
/// appeared between its begin marker and the next one.
fn parse_log(log: &str, n: usize) -> Vec<Marks> {
    let mut m = vec![Marks::default(); n];
    let mut cur: Option<usize> = None;
    for line in log.lines() {
        if let Some(r) = line.strip_prefix("CDMB ") {
            let mut it = r.split_whitespace();
            let (Some(i), Some(p)) = (it.next().and_then(|x| x.parse::<usize>().ok()), it.next().and_then(|x| x.parse().ok())) else {
                continue;
            };
            if i < n {
                m[i].begin_page = Some(p);
                cur = Some(i);
            }
        } else if let Some(r) = line.strip_prefix("CDME ") {
            let v: Vec<&str> = r.split_whitespace().collect();
            if v.len() == 4 {
                if let (Ok(i), Ok(p), Ok(g), Ok(c)) = (v[0].parse::<usize>(), v[1].parse(), v[2].parse(), v[3].parse()) {
                    if i < n {
                        m[i].end = Some((p, g, c));
                    }
                }
            }
        } else if line.starts_with('!') && !benign_error(line) {
            if let Some(i) = cur {
                m[i].error = true;
            }
        }
    }
    m
}

fn document(paper: u32, jobs: &[&Job]) -> String {
    let mut d = template_head(paper);
    for (i, j) in jobs.iter().enumerate() {
        d.push_str(&format!("\\immediate\\write16{{CDMB {i} \\the\\value{{page}}}}%\n\\begingroup\n"));
        d.push_str(&template_body(&j.prep.rgb_latex.replace(r"\mathcolor", r"\color")));
        d.push_str(&format!(
            "\\endgroup\n\\clearpage\n\\immediate\\write16{{CDME {i} \\the\\value{{page}} \\the\\currentgrouplevel\\space\\the\\currentiflevel}}%\n"
        ));
    }
    d.push_str("\\end{document}\n");
    d
}

/// TeX's "(That makes 100 errors; please try again.)": the run stops. TeX counts errors
/// per paragraph, and each formula is its own paragraph in a batch as in the single
/// document, so a formula that stops a batch this way stops its single render too,
/// before its page is shipped out: the single render leaves no PDF.
const TOO_MANY_ERRORS: &str = "(That makes 100 errors; please try again.)";

/// What happened to a formula of a batch.
enum Outcome {
    /// its page was boxed and the output files written
    Page,
    /// its single render leaves no PDF (no output files, as `latex2bbox_color`)
    NoOutput,
    /// left to the single render
    Fallback,
}

fn settle(j: &Job, outcome: Outcome, why: &str, st: &mut BatchStats, no_output: &mut NoOutput) {
    match outcome {
        Outcome::Page => st.from_batch += 1,
        Outcome::NoOutput => {
            st.no_output += 1;
            no_output.insert((j.subset, j.basename.clone()));
        }
        Outcome::Fallback => {
            // remove partial outputs so the single render redoes this formula
            let (b, v, base) = output_paths(&j.output_path, &j.basename);
            for p in [b, v, base] {
                let _ = std::fs::remove_file(p);
            }
            st.fallback += 1;
            match why {
                "markers" => st.fb_markers += 1,
                "pages" => st.fb_pages += 1,
                "unclean" => st.fb_unclean += 1,
                "error" => st.fb_error += 1,
                _ => st.fb_other += 1,
            }
            if let Ok(d) = std::env::var("CDM_BATCH_LOG_DIR") {
                use std::io::Write;
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!("{d}/fallbacks.txt")) {
                    let _ = writeln!(f, "{why}\t{}", j.prep.rgb_latex.chars().take(300).collect::<String>());
                }
            }
        }
    }
}

/// Renders one batch and settles a prefix of it; returns the length of that prefix
/// (at least 1). Formulas after one that leaves TeX in an unclean state, or after a
/// formula that stopped the run, are batched again.
fn run_batch(jobs: &[&Job], paper: u32, threshold: u8, keep_errors: bool, st: &mut BatchStats, no_output: &mut NoOutput) -> usize {
    st.tex_runs += 1;
    let doc = document(paper, jobs);
    let (pdf, log) = match tex::compile_with_log(&doc, true) {
        Ok(x) => x,
        Err(_) => {
            // setup failure: every formula is left to the single render (which reports it)
            for j in jobs {
                settle(j, Outcome::Fallback, "setup", st, no_output);
            }
            return jobs.len();
        }
    };
    let log = String::from_utf8_lossy(log.as_deref().unwrap_or(&[])).into_owned();
    if let Ok(d) = std::env::var("CDM_BATCH_LOG_DIR") {
        let _ = std::fs::write(format!("{d}/batch{}.log", st.tex_runs), &log);
        let _ = std::fs::write(format!("{d}/batch{}.tex", st.tex_runs), &doc);
    }
    let marks = parse_log(&log, jobs.len());

    // A formula without its closing marker stopped the run; TeX then writes no PDF, so
    // the formulas before it are typeset again without it.
    if let Some(k) = marks.iter().position(|m| m.begin_page.is_some() && m.end.is_none()) {
        let seg_start = log.find(&format!("\nCDMB {k} ")).unwrap_or(0);
        let fatal_too_many = log[seg_start..].contains(TOO_MANY_ERRORS);
        let mut s = 0;
        while s < k {
            s += run_batch(&jobs[s..k], paper, threshold, keep_errors, st, no_output);
        }
        if fatal_too_many {
            settle(jobs[k], Outcome::NoOutput, "", st, no_output);
        } else {
            settle(jobs[k], Outcome::Fallback, "markers", st, no_output);
        }
        return k + 1;
    }

    let parsed = pdf.as_deref().and_then(|p| raster::parse(p).ok());
    let pages = parsed.as_ref().map(|p| p.pages());
    for (i, (j, m)) in jobs.iter().zip(&marks).enumerate() {
        let clean_after = matches!(m.end, Some((_, 0, 0)));
        let (outcome, why) = match (m.begin_page, m.end) {
            (Some(b), Some((e, ..))) if e - b != 1 => (Outcome::Fallback, "pages"),
            (Some(b), Some(_)) => {
                if m.error && !keep_errors {
                    (Outcome::Fallback, "error")
                } else {
                    let ok = pages.as_ref().and_then(|ps| ps.get((b - 1) as usize)).is_some_and(|page| {
                        raster::rasterize_page(page, threshold).ok().is_some_and(|opened| {
                            let colors = gen_color_list(5800, 15);
                            let (bb, v, base) = output_paths(&j.output_path, &j.basename);
                            write_boxes(&opened, &j.prep.token_list, &colors[..j.prep.n_colors], &bb, &v, &base).is_ok()
                        })
                    });
                    if ok {
                        (Outcome::Page, "")
                    } else {
                        (Outcome::Fallback, "other")
                    }
                }
            }
            _ => (Outcome::Fallback, "markers"),
        };
        settle(j, outcome, why, st, no_output);
        if !clean_after {
            return i + 1;
        }
    }
    jobs.len()
}

/// Typesets the formulas of `items` (cleaned gt / pred strings with their img_id) in
/// batches of up to `batch_size` and writes the per-formula output files the single
/// render would write. Formulas whose outputs already exist, or whose preprocessing
/// fails, are skipped (the per-sample evaluation handles them as before). Returns the
/// statistics and the formulas whose single render leaves no PDF.
pub(crate) fn render_batched(
    root: &str,
    items: &[(String, String, String)],
    batch_size: usize,
    threshold: u8,
    keep_errors: bool,
) -> PyResult<(BatchStats, NoOutput)> {
    prepare_directories(root)?;
    let colors = gen_color_list(5800, 15);
    let mut early_no_output = NoOutput::new();
    let mut by_paper: BTreeMap<u32, Vec<Job>> = BTreeMap::new();
    for (img_id, gt, pred) in items {
        for (subset, latex) in [("gt", gt), ("pred", pred)] {
            let output_path = path_join(root, subset);
            let (b, v, base) = output_paths(&output_path, img_id);
            if [&b, &v, &base].iter().all(|p| std::path::Path::new(p).exists()) {
                continue;
            }
            let temp_dir = path_join(root, &format!("temp_dir_{subset}_{img_id}"));
            mkdirs(&temp_dir)?;
            let prep = prepare(latex, img_id, &temp_dir, &colors, crate::Tokenizer::Native);
            let _ = std::fs::remove_dir_all(&temp_dir);
            if let Ok(prep) = prep {
                if max_line_chars(&prep.rgb_latex.replace(r"\mathcolor", r"\color")) >= LINE_LIMIT {
                    // XeTeX aborts on reading the line; the single render leaves no PDF
                    settle_no_output(subset, img_id, &mut early_no_output);
                    continue;
                }
                by_paper.entry(prep.paper_size).or_default().push(Job {
                    subset,
                    output_path,
                    basename: img_id.replace(".jpg", ""),
                    prep,
                });
            }
        }
    }
    let mut st = BatchStats::default();
    st.no_output += early_no_output.len();
    let mut no_output = early_no_output;
    for (paper, jobs) in &by_paper {
        st.batched += jobs.len();
        let refs: Vec<&Job> = jobs.iter().collect();
        let mut start = 0;
        while start < refs.len() {
            let end = (start + batch_size.max(1)).min(refs.len());
            start += run_batch(&refs[start..end], *paper, threshold, keep_errors, &mut st, &mut no_output);
        }
    }
    Ok((st, no_output))
}

/// Debug hook: the batch document for `latexes` and its TeX log.
pub(crate) fn debug(latexes: &[&str]) -> (String, String) {
    let colors = gen_color_list(5800, 15);
    let jobs: Vec<Job> = latexes
        .iter()
        .filter_map(|l| prepare(l, "x", ".", &colors, crate::Tokenizer::Native).ok())
        .map(|prep| Job { subset: "gt", output_path: String::new(), basename: String::new(), prep })
        .collect();
    let refs: Vec<&Job> = jobs.iter().collect();
    let doc = document(5, &refs);
    let log = tex::compile_with_log(&doc, true).ok().and_then(|x| x.1).unwrap_or_default();
    (doc, String::from_utf8_lossy(&log).into_owned())
}

/// TeX errors that do not make a batch page differ from the single render:
/// - the template's CJK font ("Source Han Sans SC") cannot be loaded; XeTeX reports it
///   when the first display sets up the CJK math fonts (once per document, so in a batch
///   only at its first formula, in a single render at every formula) and ignores the
///   font;
/// - `\textfont N is undefined`: a CJK character in math with that font missing; TeX
///   drops the character, as in the single render.
fn benign_error(line: &str) -> bool {
    (line.starts_with("! Font TU/SourceHanSans") && line.contains("not load")) || (line.starts_with("! \\textfont ") && line.contains("is undefined"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_markers() {
        let log = "pre\n! Package fontspec Error: x\nCDMB 0 1\n! Font TU/SourceHanSansSC(0)/m/n/12=\"Source Han Sans SC/OT\" at 12.0pt not loada\nble\n[1\n]\nCDME 0 2 0 0\nCDMB 1 2\n! Missing } inserted.\n[2]\nCDME 1 3 0 0\nCDMB 2 3\n[3] [4]\nCDME 2 5 1 0\n";
        let m = parse_log(log, 4);
        assert_eq!((m[0].begin_page, m[0].end, m[0].error), (Some(1), Some((2, 0, 0)), false));
        assert!(m[1].error);
        assert_eq!(m[2].end, Some((5, 1, 0)));
        assert_eq!((m[3].begin_page, m[3].end), (None, None));
    }
}

/// XeTeX's input line buffer (`buf_size` in Tectonic's engine): a longer input line
/// aborts the run when it is read ("unable to read an entire line"), before the
/// formula's page is shipped out, in a batch as in the single render.
const LINE_LIMIT: usize = 200_000;

fn max_line_chars(s: &str) -> usize {
    s.split(['\n', '\r']).map(|l| l.chars().count()).max().unwrap_or(0)
}

fn settle_no_output(subset: &'static str, img_id: &str, no_output: &mut NoOutput) {
    no_output.insert((subset, img_id.replace(".jpg", "")));
}
