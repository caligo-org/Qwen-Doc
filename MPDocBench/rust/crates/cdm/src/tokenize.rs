//! Port of `metrics/cdm/modules/tokenize_latex/tokenize_latex.py`. The KaTeX based
//! tokenizers (`preprocess_formula.js`, `preprocess_tabular.js`) run as the Rust port in
//! `crate::katex` ([`Tokenizer::Native`]), or as the toolkit's own JavaScript with `node`
//! (and `perl` for tabular input) through `/bin/sh`, with the Python code's command lines
//! ([`Tokenizer::External`]).

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use regex::Regex;

use crate::katex::{self, Script};
use crate::py::{self, PyErr, PyResult};

/// Directory holding `preprocess_formula.js` / `preprocess_tabular.js` and `third_party/`.
/// `MPD_CDM_TOKENIZE_DIR` overrides the toolkit copy next to this crate.
pub fn tokenize_dir() -> &'static str {
    static D: OnceLock<String> = OnceLock::new();
    D.get_or_init(|| {
        if let Ok(d) = std::env::var("MPD_CDM_TOKENIZE_DIR") {
            if !d.is_empty() {
                return d;
            }
        }
        let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../metrics/cdm/modules/tokenize_latex");
        let p = p.canonicalize().unwrap_or(p);
        p.to_string_lossy().into_owned()
    })
}

/// `subprocess.call(cmd, shell=True)`: returns the exit status as Python reports it
/// (negative signal number when killed by a signal).
pub fn shell_call(cmd: &str) -> PyResult<i32> {
    let st = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map_err(|e| PyErr::new(format!("OSError: {e}")))?;
    Ok(exit_code(&st))
}

pub fn exit_code(st: &std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    match st.code() {
        Some(c) => c,
        None => -st.signal().unwrap_or(0),
    }
}

fn write_text(path: &str, s: &str) -> PyResult<()> {
    fs::write(path, s.as_bytes()).map_err(|e| PyErr::new(format!("OSError: {path}: {e}")))
}

fn remove(path: &str) -> PyResult<()> {
    fs::remove_file(path).map_err(|e| PyErr::new(format!("FileNotFoundError: {path}: {e}")))
}

/// Lines of a text file as Python's `for line in open(path, 'r')` yields them
/// (UTF-8 strict, universal newlines).
pub fn read_lines(path: &str) -> PyResult<Vec<String>> {
    let b = fs::read(path).map_err(|e| PyErr::new(format!("OSError: {path}: {e}")))?;
    let t = String::from_utf8(b).map_err(|_| PyErr::new("UnicodeDecodeError"))?;
    let t = t.replace("\r\n", "\n").replace('\r', "\n");
    Ok(t.split_inclusive('\n').map(|x| x.to_string()).collect())
}

fn ops_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        let names = [
            "arccos", "arcsin", "arctan", "arg", "cos", "cosh", "cot", "coth", "csc", "deg", "det", "dim", "exp", "gcd",
            "hom", "inf", "injlim", "ker", "lg", "lim", "liminf", "limsup", "ln", "log", "max", "min", "Pr", "projlim",
            "sec", "sin", "sinh", "sup", "tan", "tanh",
        ]
        .join("|");
        // '\s?'.join(names): the separator goes between every character
        let ops: Vec<String> = names.chars().map(|c| c.to_string()).collect();
        let ops = ops.join(r"\s?");
        Regex::new(&format!(r"\\operatorname \{{({ops})\}}")).unwrap()
    })
}

/// `re.sub(r'\\begin{(NAMES)\*?}(.+?)\\end{\1\*?}', r'BEGIN\2END', s, flags=re.S)`,
/// matched by hand with the backtracking semantics of Python's `re`.
fn sub_env(s: &str, names: &[&str], begin: &str, end: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::new();
    let mut last = 0;
    let mut pos = 0;
    'outer: while let Some(off) = s[pos..].find("\\begin{") {
        let p = pos + off;
        let after = p + 7;
        for name in names {
            if !s[after..].starts_with(name) {
                continue;
            }
            let q = after + name.len();
            let cs = if s[q..].starts_with("*}") {
                q + 2
            } else if b.get(q) == Some(&b'}') {
                q + 1
            } else {
                continue;
            };
            // `.+?`: at least one character, then the earliest `\end{name\*?}`
            let first = match s[cs..].chars().next() {
                Some(c) => c.len_utf8(),
                None => continue,
            };
            let pat = format!("\\end{{{name}");
            let mut e = cs + first;
            while let Some(o) = s[e..].find(&pat) {
                let at = e + o;
                let r = at + pat.len();
                let mend = if s[r..].starts_with("*}") {
                    Some(r + 2)
                } else if b.get(r) == Some(&b'}') {
                    Some(r + 1)
                } else {
                    None
                };
                if let Some(mend) = mend {
                    out.push_str(&s[last..p]);
                    out.push_str(begin);
                    out.push_str(&s[cs..at]);
                    out.push_str(end);
                    last = mend;
                    pos = mend;
                    continue 'outer;
                }
                e = at + 1;
            }
        }
        pos = p + 1;
    }
    out.push_str(&s[last..]);
    out
}

/// Lines of a text as Python's `for line in open(path, 'r')` yields them (universal
/// newlines).
fn text_lines(t: &str) -> Vec<String> {
    let t = t.replace("\r\n", "\n").replace('\r', "\n");
    t.split_inclusive('\n').map(|x| x.to_string()).collect()
}

/// The tabular `perl -pe 's|hskip(.*?)(cm\|in\|pt\|mm\|em)|hspace{\1\2}|g'` (inside the
/// single-quoted perl program `\|` is alternation). The input has no newlines.
pub fn hskip_rewrite(s: &str) -> String {
    static R: OnceLock<Regex> = OnceLock::new();
    let r = R.get_or_init(|| Regex::new(r"hskip(.*?)(cm|in|pt|mm|em)").unwrap());
    r.replace_all(s, |c: &regex::Captures| format!("hspace{{{}{}}}", &c[1], &c[2])).into_owned()
}

/// Which tokenizer runs the toolkit's KaTeX scripts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tokenizer {
    /// The Rust port (`crate::katex`), perl's rewrite as a Rust regex.
    Native,
    /// `node` and `perl` on PATH through `/bin/sh`, as the Python code runs them.
    External,
}

fn formula_post(lines: Vec<String>) -> PyResult<(bool, String)> {
    let mut post: Option<String> = None;
    for line in lines {
        let joined = py::split_ws(&line).join(" ");
        let p = ops_regex()
            .replace_all(&joined, |c: &regex::Captures| format!("\\{}", c[1].replace(' ', "")))
            .into_owned();
        post = Some(p.replace(r"\\ \end{array}", r"\end{array}"));
    }
    match post {
        Some(p) => Ok((true, p)),
        None => Err(PyErr::new("UnboundLocalError: post")),
    }
}

fn tabular_post(lines: Vec<String>) -> PyResult<(bool, String)> {
    let mut post: Option<String> = None;
    for line in lines {
        post = Some(py::split_ws(&line).join(" "));
    }
    match post {
        Some(p) => Ok((true, p)),
        None => Err(PyErr::new("UnboundLocalError: post")),
    }
}

/// The formula pre-processing before node: `\begin{split|align|...}` -> aligned,
/// smallmatrix -> matrix.
pub fn formula_prepre(latex_code: &str) -> String {
    let prepre = sub_env(
        latex_code,
        &["split", "align", "alignedat", "alignat", "eqnarray"],
        "\\begin{aligned}",
        "\\end{aligned}",
    );
    sub_env(&prepre, &["smallmatrix"], "\\begin{matrix}", "\\end{matrix}")
}

/// The tabular pre-processing (`%` comments, missing `\end{tabular}`): returns the
/// returned-on-failure code and the text written for perl.
pub fn tabular_pre(latex_code: &str) -> (String, String) {
    let mut lc = latex_code.replace("\\\\%", "\\\\ %");
    lc = lc.replace("\\%", "<PERCENTAGE_TOKEN>");
    lc = lc.split('%').next().unwrap().to_string();
    lc = lc.replace("<PERCENTAGE_TOKEN>", "\\%");
    if !lc.contains("\\end{tabular}") {
        lc.push_str("\\end{tabular}");
    }
    let file = lc.replace(['\r', '\n'], " ");
    (lc, file)
}

/// `tokenize_latex(latex_code, latex_type="", middle_file=...)`
pub fn tokenize_latex(latex_code: &str, middle_file: &str, tok: Tokenizer) -> PyResult<(bool, String)> {
    if latex_code.is_empty() {
        return Ok((false, latex_code.to_string()));
    }
    let tabular = latex_code.contains("tabular");
    if tok == Tokenizer::Native {
        return if !tabular {
            let out = katex::run(&formula_prepre(latex_code), Script::FormulaNormalize);
            formula_post(text_lines(&out))
        } else {
            let (_lc, file) = tabular_pre(latex_code);
            let out = katex::run(&hskip_rewrite(&file), Script::TabularTokenize);
            tabular_post(text_lines(&out))
        };
    }

    let temp_file = format!("{middle_file}.tmp");
    let dir = tokenize_dir();
    if !tabular {
        write_text(&temp_file, &formula_prepre(latex_code))?;
        let js = py::path_join(dir, "preprocess_formula.js");
        let cmd = format!("cat {temp_file} | node {js} normalize > {middle_file} ");
        let ret = shell_call(&cmd)?;
        remove(&temp_file)?;
        if ret != 0 {
            return Ok((false, latex_code.to_string()));
        }
        let lines = read_lines(middle_file)?;
        remove(middle_file)?;
        formula_post(lines)
    } else {
        let (lc, file) = tabular_pre(latex_code);
        write_text(middle_file, &file)?;
        let cmd = format!(
            "perl -pe 's|hskip(.*?)(cm\\|in\\|pt\\|mm\\|em)|hspace{{\\1\\2}}|g' {middle_file} > {temp_file}"
        );
        let ret = shell_call(&cmd)?;
        if ret != 0 {
            return Ok((false, lc));
        }
        remove(middle_file)?;
        let js = py::path_join(dir, "preprocess_tabular.js");
        let cmd = format!("cat {temp_file} | node {js} tokenize > {middle_file} ");
        let ret = shell_call(&cmd)?;
        remove(&temp_file)?;
        if ret != 0 {
            return Ok((false, lc));
        }
        let lines = read_lines(middle_file)?;
        remove(middle_file)?;
        tabular_post(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hskip() {
        assert_eq!(hskip_rewrite("x hskip 1.5 cm y \\hskip2pt z hskipabc"), "x hspace{ 1.5 cm} y \\hspace{2pt} z hskipabc");
        assert_eq!(hskip_rewrite("hskip|in"), "hspace{|in}");
    }
}
