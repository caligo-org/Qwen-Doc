//! Port of `metrics/cdm/modules/latex_processor.py` (the parts reached by CDM:
//! `normalize_latex`, `token_add_color_RGB`, `find_matching_brace`).
//!
//! List operations follow Python semantics exactly (negative indices wrap, slices clamp,
//! out-of-range indexing raises), because malformed LaTeX reaches those paths.

use std::sync::OnceLock;

use regex::Regex;

use crate::py::{self, idx, slice, slice_from, slice_to, PyErr, PyResult};

const SKIP_TOKENS: &[&str] = &[
    "\\", "\\\\", "\\index", "\\a", "&", "$", "\\multirow", "\\def", "\\raggedright", "\\url", "\\cr", "\\ensuremath",
    "\\left", "\\right", "\\mathchoice", "\\scriptstyle", "\\displaystyle", "\\qquad", "\\quad", "\\,", "\\!", "~",
    "\\boldmath",
];
const PHANTOM_TOKENS: &[&str] = &["\\fontfamily", "\\vphantom", "\\phantom", "\\rowcolor", "\\ref"];
const TWO_TAIL_TOKENS: &[&str] = &["\\frac", "\\binom"];
const AB_TAIL_TOKENS: &[&str] = &["\\xrightarrow", "\\xleftarrow", "\\sqrt"];
const TWO_TAIL_INVISB_TOKENS: &[&str] = &["\\overset", "\\underset", "\\stackrel"];
const ONE_TAIL_TOKENS: &[&str] = &[
    "\\widetilde", "\\overline", "\\hat", "\\widehat", "\\tilde", "\\Tilde", "\\dot", "\\bar", "\\vec", "\\underline",
    "\\underbrace", "\\check", "\\breve", "\\Bar", "\\Vec", "\\mathring", "\\ddot",
];
const ONE_TAIL_INVISB_TOKENS: &[&str] = &[
    "\\boldsymbol", "\\pmb", "\\textbf", "\\mathrm", "\\mathbf", "\\mathbb", "\\mathcal", "\\textmd", "\\texttt",
    "\\textnormal", "\\text", "\\textit", "\\textup", "\\mathop", "\\mathbin", "\\smash", "\\operatorname", "\\textrm",
    "\\mathfrak", "\\emph", "\\textsf", "\\textsc",
];

fn is_in(t: &str, list: &[&str]) -> bool {
    list.contains(&t)
}

/// Python's default recursion limit is 1000; `token_add_color_RGB` recursing deeper than
/// what is left of it raises RecursionError, which the caller catches like any other
/// exception. The frames below the first `token_add_color_RGB` call (cal_metric worker,
/// CDM.evaluate, latex2bbox_color, ...) are counted approximately.
const MAX_COLOR_DEPTH: usize = 985;

macro_rules! re {
    ($s:expr) => {{
        static R: OnceLock<Regex> = OnceLock::new();
        R.get_or_init(|| Regex::new($s).unwrap())
    }};
}

/// SKIP_PATTERNS, each applied with `re.match` (anchored at the start).
fn skip_pattern_match(token: &str) -> bool {
    let r = re!(r"^(?:\{|\}|[\[\]]|\\begin\{.*?\}|\\end\{.*?\}|\^|_|\\.*rule|\\.*line|\[[\-.0-9]+[epm][xtm]\])");
    r.is_match(token)
}

pub fn find_matching_brace(sequence: &[String], start_index: i64, brace: (&str, &str)) -> PyResult<i64> {
    let (left, right) = brace;
    let mut depth: i64 = 0;
    let sub = slice_from(sequence, start_index);
    for (k, ch) in sub.iter().enumerate() {
        let i = start_index + k as i64;
        if ch == left {
            depth += 1;
        } else if ch == right {
            depth -= 1;
            if depth == 0 {
                return Ok(i);
            }
        }
    }
    if depth > 0 {
        return Err(PyErr::new("ValueError: Warning! found no matching brace in sequence !"));
    }
    Ok(-1)
}

const CB: (&str, &str) = ("{", "}");

/// `re.findall(pattern, l)` then `l = l.replace(bef, f(bef))` for each match in order.
fn findall_replace(l: &mut String, r: &Regex, f: impl Fn(&str) -> String) {
    let olds: Vec<String> = r.find_iter(l).map(|m| m.as_str().to_string()).collect();
    for bef in olds {
        let aft = f(&bef);
        *l = l.replace(&bef, &aft);
    }
}

fn s(x: &str) -> String {
    x.to_string()
}

fn join(v: &[String], sep: &str) -> String {
    v.join(sep)
}

fn split_sp(l: &str) -> Vec<String> {
    l.split(' ').map(|x| x.to_string()).collect()
}

macro_rules! cat {
    ($($e:expr),* $(,)?) => {{
        let mut v: Vec<String> = Vec::new();
        $( v.extend($e); )*
        v
    }};
}

pub fn normalize_latex(l: &str) -> PyResult<String> {
    let tabular = l.contains("tabular");
    let mut l = py::strip(l).replace(r"\pmatrix", r"\mypmatrix").replace(r"\matrix", r"\mymatrix");
    for item in ["\\raggedright", "\\arraybackslash"] {
        l = l.replace(item, "");
    }
    for item in ["\\lowercase", "\\uppercase"] {
        l = l.replace(item, "");
    }

    findall_replace(&mut l, re!(r"\\[hv]space \{ [.0-9a-z ]+ \}"), |item| {
        if tabular {
            String::new()
        } else {
            item.replace(' ', "")
        }
    });

    if tabular {
        l = l.replace("\\begin {tabular}", "\\begin{tabular}");
        l = l.replace("\\end {tabular}", "\\end{tabular}");
        l = l.replace("\\begin {array}", "\\begin{array}");
        l = l.replace("\\end {array}", "\\end{array}");
        let mut ls = split_sp(&l);
        let mut i: i64 = 0;
        while i < ls.len() as i64 {
            if ls[i as usize] == "\\begin{tabular}" {
                let sub_idx = i + 1;
                let end_idx = find_matching_brace(&ls, sub_idx, CB)?;
                let new_token = slice(&ls, i, end_idx + 1).concat();
                ls = cat!(slice_to(&ls, i), [new_token], slice_from(&ls, end_idx + 1));
                break;
            }
            i += 1;
        }
        l = join(&ls, " ");

        let mut ls = split_sp(&l);
        let mut i: i64 = 0;
        while i < ls.len() as i64 {
            let token = ls[i as usize].clone();
            if token == "\\cmidrule" || token == "\\cline" {
                let sub_idx = i + 1;
                let end_idx = if idx(&ls, sub_idx)? == "(" {
                    let mid_end = find_matching_brace(&ls, sub_idx, ("(", ")"))?;
                    find_matching_brace(&ls, mid_end + 1, CB)?
                } else {
                    find_matching_brace(&ls, sub_idx, CB)?
                };
                let new_token = slice(&ls, i, end_idx + 1).concat();
                ls = cat!(slice_to(&ls, i), [new_token], slice_from(&ls, end_idx + 1));
            }
            i += 1;
        }
        l = join(&ls, " ");
    }

    findall_replace(&mut l, re!(r"\\begin\{array\} \{ [lrc ]+ \}"), |item| {
        item.replace("\\begin{array} ", "<s>").replace(' ', "").replace("<s>", "\\begin{array} ")
    });
    findall_replace(&mut l, re!(r"\\not [<>+=\-]"), |item| item.replace(' ', ""));

    l = format!(" {l} ");
    l = l.replace(" \\ldots ", " . . . ");
    l = l.replace(" \\cdots ", " . . . ");
    l = l.replace(" \\dots ", " . . . ");
    l = l.replace(" \\dotsb ", " . . . ");
    l = l.replace(" \\log ", " \\mathrm { l o g } ");
    l = l.replace(" \\exp ", " \\mathrm { e x p } ");
    l = l.replace(" \\sin ", " \\mathrm { s i n } ");
    l = l.replace(" \\cos ", " \\mathrm { c o s } ");
    l = l.replace(" \\tan ", " \\mathrm { t a n } ");
    l = l.replace(" \\tanh ", " \\mathrm { t a n h } ");
    l = l.replace(" \\cosh ", " \\mathrm { c o s h } ");
    l = l.replace(" \\sinh ", " \\mathrm { s i n h } ");

    findall_replace(&mut l, re!(r"\\[Bb]ig[g]?[glrm]? [(){}|\[\]] "), |item| item.replace(' ', "") + " ");
    findall_replace(&mut l, re!(r"(?s)\\[Bb]ig[g]?[glrm]? \\.*? "), |item| item.replace(' ', "") + " ");
    findall_replace(&mut l, re!(r"\\operatorname \*"), |_| "\\operatorname".to_string());
    l = l.replace("\\lefteqn", "");
    l = l.replace("\\footnote ", "^ ");
    findall_replace(&mut l, re!(r"\\' [^{] "), |item| item.replace(' ', "") + " ");
    if tabular {
        findall_replace(&mut l, re!(r"\[ [\-.0-9 ]+[exptcm ]+ \]"), |item| item.replace(' ', ""));
    }
    findall_replace(&mut l, re!(r"\\parbox \{[^{]+\}"), |item| item.replace(' ', ""));
    findall_replace(&mut l, re!(r"\\raisebox \{[^{]+\} [\[\]0-9 exptcm]+\{"), |item| {
        let aft = item.replace(' ', "");
        // aft[0:-1] + " {"  (aft ends with '{', one byte)
        format!("{} {{", &aft[..aft.len() - 1])
    });
    findall_replace(&mut l, re!(r"\{ \\char[0-9' ]+\}"), |item| {
        let aft = item.replace(' ', "");
        // "{ " + aft[1:-1] + " }"  (aft starts with '{' and ends with '}')
        format!("{{ {} }}", &aft[1..aft.len() - 1])
    });
    findall_replace(&mut l, re!(r"\\not [\\=<>][^ ]+ "), |item| item.replace(' ', "") + " ");
    findall_replace(&mut l, re!(r"\\specialrule \{[ .0-9a-z]+\} \{[ .0-9a-z]+\} \{[ .0-9a-z]+\}"), |item| {
        item.replace(' ', "")
    });
    findall_replace(
        &mut l,
        re!(concat!(
            r"\\colorbox[ \[\]RGBrgb]+\{ [A-Za-z 0-9,!]+ \} |",
            r"\\color[ \[\]RGBrgb]+\{ [A-Za-z 0-9,!]+ \} |",
            r"\\textcolor[ \[\]RGBrgb]+\{ [A-Za-z 0-9,!]+ \} |",
            r"\\cellcolor[ \[\]RGBrgb]+\{ [A-Za-z 0-9,!]+ \} "
        )),
        |_| String::new(),
    );

    // filling the missing brace [] and {} according to token.
    let one_any = |t: &str| is_in(t, ONE_TAIL_TOKENS) || is_in(t, ONE_TAIL_INVISB_TOKENS);
    let mut ls = split_sp(&l);
    let mut i: i64 = 0;
    while i < ls.len() as i64 {
        let token = ls[i as usize].clone();
        if one_any(&token) {
            let mut sub_idx = i + 1;
            while sub_idx < ls.len() as i64 && one_any(&ls[sub_idx as usize]) {
                sub_idx += 1;
            }
            let mut new_split = slice(&ls, 0, i);
            for ii in i..sub_idx {
                new_split.push(ls[ii as usize].clone());
                new_split.push(s("{"));
            }
            let n_close = (sub_idx - i).max(0) as usize;
            if idx(&ls, sub_idx)? != "{" {
                new_split.push(ls[sub_idx as usize].clone());
                new_split.extend(std::iter::repeat_n(s("}"), n_close));
                ls = cat!(new_split, slice_from(&ls, sub_idx + 1));
            } else {
                let end_idx = find_matching_brace(&ls, sub_idx, CB)?;
                new_split.extend(slice(&ls, sub_idx + 1, end_idx));
                new_split.extend(std::iter::repeat_n(s("}"), n_close));
                ls = cat!(new_split, slice_from(&ls, end_idx + 1));
            }
        } else if is_in(&token, AB_TAIL_TOKENS) {
            let next = idx(&ls, i + 1)?.clone();
            if next != "[" && next != "{" {
                ls = cat!(slice_to(&ls, i + 1), [s("{"), next, s("}")], slice_from(&ls, i + 2));
            } else {
                let end1 = if next == "[" { find_matching_brace(&ls, i + 1, ("[", "]"))? } else { i };
                let after = idx(&ls, end1 + 1)?.clone();
                if after != "{" {
                    ls = cat!(slice_to(&ls, end1 + 1), [s("{"), after, s("}")], slice_from(&ls, end1 + 2));
                }
            }
        } else if is_in(&token, TWO_TAIL_TOKENS) || is_in(&token, TWO_TAIL_INVISB_TOKENS) {
            let next = idx(&ls, i + 1)?.clone();
            if next != "{" {
                ls = cat!(slice_to(&ls, i + 1), [s("{"), next, s("}")], slice_from(&ls, i + 2));
            }
            let end1 = find_matching_brace(&ls, i + 1, CB)?;
            let after = idx(&ls, end1 + 1)?.clone();
            if after != "{" {
                ls = cat!(slice_to(&ls, end1 + 1), [s("{"), after, s("}")], slice_from(&ls, end1 + 2));
            }
        }
        i += 1;
    }
    Ok(join(&ls, " "))
}

fn color_token(n: usize) -> String {
    format!("\\mathcolor[RGB]{{<color_{n}>}}{{")
}

/// `token_add_color_RGB(l_split, idx, token_list, brace_color)`; returns next_idx.
pub fn token_add_color_rgb(
    ls: &mut Vec<String>,
    i: i64,
    token_list: &mut Vec<String>,
    brace_color: bool,
    depth: usize,
) -> PyResult<i64> {
    if depth > MAX_COLOR_DEPTH {
        return Err(PyErr::new("RecursionError: maximum recursion depth exceeded"));
    }
    let token = idx(ls, i)?.clone();
    let next_idx;
    if token.is_empty() {
        next_idx = i + 1;
    } else if is_in(&token, PHANTOM_TOKENS) {
        let brace_end = if idx(ls, i + 1)? == "{" { find_matching_brace(ls, i + 1, CB)? } else { i + 1 };
        next_idx = brace_end + 1;
    } else if is_in(&token, TWO_TAIL_TOKENS) {
        let num_start = i + 1;
        let num_end = find_matching_brace(ls, num_start, CB)?;
        let den_start = num_end + 1;
        let den_end = find_matching_brace(ls, den_start, CB)?;
        let ct = color_token(token_list.len());
        *ls = cat!(
            slice_to(ls, i),
            [ct + &token],
            slice(ls, i + 1, den_end + 1),
            [s("}")],
            slice_from(ls, den_end + 1)
        );
        token_list.push(token);
        next_idx = i + 1;
    } else if is_in(&token, ONE_TAIL_TOKENS) {
        let num_start = i + 1;
        let num_end = find_matching_brace(ls, num_start, CB)?;
        let ct = color_token(token_list.len());
        if token != "\\underbrace" && num_end + 1 < ls.len() as i64 && idx(ls, num_end + 1)? == "_" {
            *ls = cat!(
                slice_to(ls, i),
                [format!("{{{ct}{token}")],
                slice(ls, i + 1, num_end + 1),
                [s("}}")],
                slice_from(ls, num_end + 1)
            );
        } else {
            *ls = cat!(
                slice_to(ls, i),
                [ct + &token],
                slice(ls, i + 1, num_end + 1),
                [s("}")],
                slice_from(ls, num_end + 1)
            );
        }
        token_list.push(token);
        next_idx = i + 1;
    } else if is_in(&token, ONE_TAIL_INVISB_TOKENS) {
        let num_start = i + 1;
        let num_end = find_matching_brace(ls, num_start, CB)?;
        let mut sub_idx = num_start + 1;
        if num_end - num_start == 2 {
            let ct = color_token(token_list.len());
            let inner = idx(ls, num_start + 1)?.clone();
            token_list.push(inner.clone());
            *ls = cat!(slice_to(ls, num_start + 1), [ct + &inner + "}"], slice_from(ls, num_end));
        } else {
            while sub_idx < num_end {
                sub_idx = token_add_color_rgb(ls, sub_idx, token_list, false, depth + 1)?;
            }
        }
        next_idx = num_end + 1;
    } else if is_in(&token, AB_TAIL_TOKENS) {
        let nxt = idx(ls, i + 1)?.clone();
        if nxt == "{" {
            let num_start = i + 1;
            let num_end = find_matching_brace(ls, num_start, CB)?;
            let ct = color_token(token_list.len());
            *ls = cat!(
                slice_to(ls, i),
                [ct + &token],
                slice(ls, i + 1, num_end + 1),
                [s("}")],
                slice_from(ls, num_end + 1)
            );
            token_list.push(token);
            let mut sub_idx = num_start + 1;
            while sub_idx < num_end {
                sub_idx = token_add_color_rgb(ls, sub_idx, token_list, false, depth + 1)?;
            }
            next_idx = num_end + 1;
        } else if nxt == "[" {
            let num_start = i + 1;
            let num_end = find_matching_brace(ls, num_start, ("[", "]"))?;
            let den_start = num_end + 1;
            let den_end = find_matching_brace(ls, den_start, CB)?;
            let ct = color_token(token_list.len());
            *ls = cat!(
                slice_to(ls, i),
                [ct + &token],
                slice(ls, i + 1, den_end + 1),
                [s("}")],
                slice_from(ls, den_end + 1)
            );
            token_list.push(token);
            let mut sub_idx = num_start + 1;
            while sub_idx < num_end {
                sub_idx = token_add_color_rgb(ls, sub_idx, token_list, true, depth + 1)?;
            }
            let mut sub_idx = den_start + 1;
            while sub_idx < den_end {
                sub_idx = token_add_color_rgb(ls, sub_idx, token_list, false, depth + 1)?;
            }
            next_idx = den_end + 1;
        } else {
            return Err(PyErr::new("UnboundLocalError: next_idx"));
        }
    } else if token == "\\multicolumn" || token == "\\multirow" {
        let first_start = i + 1;
        let first_end = find_matching_brace(ls, first_start, CB)?;
        let second_start = first_end + 1;
        let second_end = find_matching_brace(ls, second_start, CB)?;
        let third_start = second_end + 1;
        let third_end = find_matching_brace(ls, third_start, CB)?;
        let mut sub_idx = third_start + 1;
        while sub_idx < third_end {
            sub_idx = token_add_color_rgb(ls, sub_idx, token_list, false, depth + 1)?;
        }
        next_idx = third_end + 1;
    } else if is_in(&token, SKIP_TOKENS) || is_in(&token, TWO_TAIL_INVISB_TOKENS) || skip_pattern_match(&token) {
        let colored = (token == "[" && idx(ls, i - 1)? != "\\sqrt")
            || (token == "]" && i >= 3 && idx(ls, i - 3)? != "\\sqrt");
        if colored {
            let ct = color_token(token_list.len());
            let cur = idx(ls, i)?.clone();
            *ls = cat!(slice_to(ls, i), [ct + &cur + "}"], slice_from(ls, i + 1));
            token_list.push(token);
        }
        next_idx = i + 1;
    } else {
        let ct = color_token(token_list.len());
        let cur = idx(ls, i)?.clone();
        if brace_color || (i > 1 && idx(ls, i - 1)? == "_") {
            *ls = cat!(slice_to(ls, i), [format!("{{{ct}{cur}}}}}")], slice_from(ls, i + 1));
        } else {
            *ls = cat!(slice_to(ls, i), [ct + &cur + "}"], slice_from(ls, i + 1));
        }
        token_list.push(token);
        next_idx = i + 1;
    }
    Ok(next_idx)
}
