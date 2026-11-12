//! Port of the text/formula normalisation in `utils/data_preprocess.py`.

use crate::py;
use regex::Regex;
use std::sync::LazyLock;

static INLINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$(.*?)\$|\\\((.*?)\\\)").unwrap());
static ESC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\([\\_&%^])").unwrap());
static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\tag\{.*?\}").unwrap());
static HSPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\hspace\{.*?\}").unwrap());
static BEGIN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\begin\{.*?\}").unwrap());
static END: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\end\{.*?\}").unwrap());
static COLSEP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\arraycolsep.*?\}").unwrap());

/// Marathon patch: skip pylatexenc when brace nesting exceeds 10.
fn latex_nesting_too_deep(s: &str) -> bool {
    let mut depth: i64 = 0;
    for ch in s.chars() {
        if ch == '{' {
            depth += 1;
            if depth > 10 {
                return true;
            }
        } else if ch == '}' {
            depth -= 1;
        }
    }
    false
}

/// `textblock2unicode(text)`; `marathon` enables the MarathonDocBench patch's depth check.
pub fn textblock2unicode(text: &str, marathon: bool) -> String {
    let ci = py::CharIndex::new(text);
    let mut removals: Vec<(usize, usize, String)> = Vec::new();
    for c in INLINE.captures_iter(text) {
        let m = c.get(0).unwrap();
        let content = c.get(1).or_else(|| c.get(2)).map_or("", |g| g.as_str());
        let mut clean = ESC.replace_all(content, "").into_owned();
        if clean.contains(['\\', '^', '_']) && !(marathon && latex_nesting_too_deep(&clean)) {
            if clean.ends_with('\\') {
                clean.push(' ');
            }
            match pylatexenc::latex_to_text(&clean) {
                Ok(u) => removals.push((ci.char_of(m.start()), ci.char_of(m.end()), u)),
                Err(_) => continue,
            }
        }
    }
    if removals.is_empty() {
        return text.to_string();
    }
    // apply from the end (positions are ascending and non-overlapping)
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut cur = 0usize;
    for (start, end, u) in &removals {
        out.extend(&chars[cur..*start]);
        out.push_str(py::strip(u));
        cur = *end;
    }
    out.extend(&chars[cur..]);
    out
}

/// `clean_string(s)`: drop `\t \n` escapes/controls and `_`, keep `\w` and CJK.
pub fn clean_string(s: &str) -> String {
    let s = s
        .replace("\\t", "")
        .replace("\\n", "")
        .replace('\t', "")
        .replace('\n', "")
        .replace("/t", "")
        .replace("/n", "")
        .replace('_', "");
    s.chars().filter(|&c| py::is_word(c) || ('\u{4e00}'..='\u{9fff}').contains(&c)).collect()
}

/// `re.compile(r"\\\[(.+?)(?<!\\)\\\]").search(text)` → group 1.
fn bracket_display(text: &str) -> Option<String> {
    let ch: Vec<char> = text.chars().collect();
    let n = ch.len();
    let mut s = 0;
    while s + 1 < n {
        if ch[s] == '\\' && ch[s + 1] == '[' {
            // content: at least one non-newline char, ends right before `\]` not preceded by `\`
            let mut e = s + 3;
            while e < n + 1 {
                if ch[e - 1] == '\n' {
                    break;
                }
                if e + 1 < n && ch[e] == '\\' && ch[e + 1] == ']' && ch[e - 1] != '\\' {
                    return Some(ch[s + 2..e].iter().collect());
                }
                e += 1;
            }
        }
        s += 1;
    }
    None
}

const FORMULA_FILTER: [&str; 36] = [
    "\\mathbf", "\\mathrm", "\\mathnormal", "\\mathit", "\\mathbb", "\\mathcal", "\\mathscr", "\\mathfrak", "\\mathsf",
    "\\mathtt", "\\textbf", "\\text", "\\boldmath", "\\boldsymbol", "\\operatorname", "\\bm", "\\symbfit", "\\mathbfcal",
    "\\symbf", "\\scriptscriptstyle", "\\notag", "\\setlength", "\\coloneqq", "\\space", "\\thickspace", "\\thinspace",
    "\\medspace", "\\nobreakspace", "\\negmedspace", "\\quad", "\\qquad", "\\enspace", "\\substackw", " ", "$$", "\\left",
];
const FORMULA_FILTER_TAIL: [&str; 3] = ["\\right", "\\displaystyle", "\\text"];

/// `normalized_formula(text)`.
pub fn normalized_formula(text: &str) -> String {
    let t = py::strip(text);
    let t = py::strip_chars(t, "$");
    let t = py::strip_chars(t, "\n");
    let mut t = match bracket_display(t) {
        Some(g) => py::strip(&g).to_string(),
        None => t.to_string(),
    };
    for re in [&*TAG, &*HSPACE, &*BEGIN, &*END, &*COLSEP] {
        t = re.replace_all(&t, "").into_owned();
    }
    let mut t = py::strip_chars(&t, ".").to_string();
    for f in FORMULA_FILTER.iter().chain(FORMULA_FILTER_TAIL.iter()) {
        t = t.replace(f, "");
    }
    py::lower(&t)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bracket() {
        assert_eq!(bracket_display(r"a \[x\] b").as_deref(), Some("x"));
        assert_eq!(bracket_display(r"\[x\\]y\]").as_deref(), Some(r"x\\]y"));
        assert_eq!(bracket_display("\\[x\ny\\]"), None);
        assert_eq!(bracket_display(r"\[\]"), None);
        assert_eq!(bracket_display(r"\[\]\]").as_deref(), Some(r"\]"));
    }
}
