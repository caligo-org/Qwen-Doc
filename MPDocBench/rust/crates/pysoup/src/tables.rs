//! The MPDocBench table functions on top of the parser/DOM ports.

use crate::dom::{Id, Soup};
use crate::pystr::{self, is_space, nfkc, py_int, strip};
use crate::unescape::html_unescape;
use crate::Error;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

// ------------------------------------------------------------------ regex helpers
// All patterns below are hand-translated; delimiters are ASCII, so byte offsets
// on UTF-8 are safe, and Python `.` (no DOTALL) only excludes '\n'.

fn find_from(h: &str, pat: &str, from: usize) -> Option<usize> {
    if from > h.len() {
        return None;
    }
    h[from..].find(pat).map(|p| p + from)
}

/// `re.sub('( NAME=".*?")', "", s)` for a literal prefix like ` style="`.
fn sub_attr(s: &str, prefix: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    let mut p = 0;
    while let Some(q) = find_from(s, prefix, p) {
        let after = q + prefix.len();
        // lazy .*? then '"' : first '"' with no '\n' in between
        let rest = &s.as_bytes()[after..];
        let mut hit = None;
        for (k, &b) in rest.iter().enumerate() {
            if b == b'"' {
                hit = Some(after + k);
                break;
            }
            if b == b'\n' {
                break;
            }
        }
        match hit {
            Some(e) => {
                out.push_str(&s[last..q]);
                last = e + 1;
                p = e + 1;
            }
            None => p = q + 1,
        }
    }
    out.push_str(&s[last..]);
    out
}

/// `re.sub(OPEN + '.*?' + CLOSE, '', s)` without DOTALL (literal OPEN/CLOSE).
fn sub_lazy_nodotall(s: &str, open: &str, close: &str, repl: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    let mut p = 0;
    while let Some(q) = find_from(s, open, p) {
        let after = q + open.len();
        // first occurrence of `close` reachable without crossing '\n'
        let nl = find_from(s, "\n", after).unwrap_or(s.len());
        match find_from(s, close, after) {
            Some(c) if c <= nl => {
                out.push_str(&s[last..q]);
                out.push_str(repl);
                last = c + close.len();
                p = last;
            }
            _ => p = q + 1,
        }
        if p > s.len() {
            break;
        }
    }
    out.push_str(&s[last..]);
    out
}

/// `re.sub(r'\s+', " ", s)`
fn collapse_space(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_ws = false;
    for c in s.chars() {
        if is_space(c) {
            if !in_ws {
                out.push(' ');
                in_ws = true;
            }
        } else {
            in_ws = false;
            out.push(c);
        }
    }
    out
}

fn ascii_ci_at(s: &[u8], i: usize, pat: &str) -> bool {
    let p = pat.as_bytes();
    i + p.len() <= s.len() && s[i..i + p.len()].eq_ignore_ascii_case(p)
}

/// `''.join(re.findall(r'<table\b[^>]*>(.*)</table>', s, re.DOTALL | re.IGNORECASE))`
fn findall_table_body(s: &str) -> String {
    let b = s.as_bytes();
    // last `</table>` (case-insensitive)
    let mut last_close = None;
    if b.len() >= 8 {
        let mut i = b.len() - 8;
        loop {
            if ascii_ci_at(b, i, "</table>") {
                last_close = Some(i);
                break;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    let Some(lc) = last_close else { return String::new() };
    let mut p = 0;
    while p + 6 <= b.len() {
        if b[p] == b'<' && ascii_ci_at(b, p, "<table") {
            // \b after "table": next char must be a non-word char or end
            let next = s[p + 6..].chars().next();
            let boundary = match next {
                None => true,
                Some(c) => !pystr::is_word(c),
            };
            if boundary {
                match find_from(s, ">", p + 6) {
                    None => return String::new(),
                    Some(gt) => {
                        if lc > gt {
                            return s[gt + 1..lc].to_string();
                        }
                        return String::new();
                    }
                }
            }
        }
        p += 1;
    }
    String::new()
}

/// The shared first stage of both normalisers (inner `process_table_html`).
fn soup_stage(md: &str) -> Result<String, Error> {
    let (mut soup, root) = Soup::parse(md);
    soup.base_frames = CALLER_DEPTH_NORMALIZE.load(Ordering::Relaxed) + 3;
    for th in soup.find_all(root, &["th"])? {
        soup.set_name(th, "td");
    }
    for thead in soup.find_all(root, &["thead"])? {
        soup.unwrap(thead)?;
    }
    for m in soup.find_all(root, &["math"])? {
        let alt = match soup.get_attr(m, "alttext") {
            None => String::new(),
            Some(v) => match v.as_str() {
                Some(s) => s.to_string(),
                // a list-valued attribute would be formatted as a Python list
                None => return Err(Error::Python("unsupported list-valued alttext".into())),
            },
        };
        let alttext = format!("${}$", alt);
        soup.replace_with_text(m, &alttext)?;
    }
    for span in soup.find_all(root, &["span"])? {
        soup.unwrap(span)?;
    }
    soup.decode(root)
}

static CALLER_DEPTH_NORMALIZE: AtomicUsize = AtomicUsize::new(2);
static CALLER_DEPTH_MERGE: AtomicUsize = AtomicUsize::new(3);

/// Python frame depth of the code that calls `normalized_html_table` /
/// `merge_tables` (1 = called from module level). Only used to emulate the
/// `RecursionError` that bs4's recursive `str(soup)` raises on trees nested
/// ~490 levels deep; the defaults (2 and 3) match `gen/dump_cases.py`.
pub fn set_python_caller_depths(normalize: usize, merge: usize) {
    CALLER_DEPTH_NORMALIZE.store(normalize, Ordering::Relaxed);
    CALLER_DEPTH_MERGE.store(merge, Ordering::Relaxed);
}

/// Common middle part; `variant_dp` selects the data_preprocess.py tail.
fn process_table_html(md: &str, variant_dp: bool) -> Result<String, Error> {
    let squeezed: String = md.chars().filter(|&c| c != ' ').map(|c| if c == '\'' { '"' } else { c }).collect();
    if !squeezed.contains("<table") {
        return Ok(String::new());
    }
    let s = soup_stage(md)?;
    let t = html_unescape(&s).replace('\n', "");
    let t = strip(&nfkc(&t)).to_string();
    let mut t = findall_table_body(&t);
    for a in [" style=\"", " height=\"", " width=\"", " align=\"", " class=\""] {
        t = sub_attr(&t, a);
    }
    let t = t.replace("<tbody>", "").replace("</tbody>", "");
    if variant_dp {
        let t = collapse_space(&t);
        Ok(format!("<html><body><table border=\"1\" >{}</table></body></html>", t))
    } else {
        Ok(format!("<table border=\"1\" >{}</table>", t))
    }
}

fn clean_table(s: &str) -> String {
    let s = s.replace("<sup>", "").replace("</sup>", "");
    let s = s.replace("<sub>", "").replace("</sub>", "");
    let s = s.replace("<span>", "").replace("</span>", "");
    let s = s.replace("<div>", "").replace("</div>", "");
    let s = s.replace("<p>", "").replace("</p>", "");
    let s = s.replace("<spandata-span-identity=\"\">", "");
    sub_lazy_nodotall(&s, "<colgroup>", "</colgroup>", "")
}

/// `utils/data_preprocess.py::normalized_html_table`, returning Python's
/// exception (if any) as an error.
pub fn try_normalized_html_table(text: &str) -> Result<String, Error> {
    let t = process_table_html(text, true)?;
    Ok(clean_table(&t))
}

/// `utils/data_preprocess.py::normalized_html_table(text)`.
///
/// Panics only where Python would raise (never observed on real data; see
/// [`try_normalized_html_table`]).
pub fn normalized_html_table(text: &str) -> String {
    try_normalized_html_table(text).unwrap_or_else(|e| panic!("normalized_html_table raised: {}", e))
}

// ---------------------------------------------------------------- merge_tables.py variant

/// `re.sub(r'>\s+<', '><', s)`
fn sub_gt_ws_lt(s: &str) -> String {
    let cs: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '>' {
            let mut j = i + 1;
            while j < cs.len() && is_space(cs[j]) {
                j += 1;
            }
            if j > i + 1 && j < cs.len() && cs[j] == '<' {
                out.push_str("><");
                i = j + 1;
                continue;
            }
        }
        out.push(cs[i]);
        i += 1;
    }
    out
}

/// `re.sub(r'<td(.*?)>(.*?)</td>', lambda m: '<td'+g1+'>'+g2.strip()+'</td>', s, flags=re.DOTALL)`
fn sub_td(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    let mut p = 0;
    while let Some(q) = find_from(s, "<td", p) {
        let m = find_from(s, ">", q + 3).and_then(|gt| find_from(s, "</td>", gt + 1).map(|c| (gt, c)));
        match m {
            Some((gt, c)) => {
                out.push_str(&s[last..q]);
                out.push_str("<td");
                out.push_str(&s[q + 3..gt]);
                out.push('>');
                out.push_str(strip(&s[gt + 1..c]));
                out.push_str("</td>");
                last = c + 5;
                p = last;
            }
            None => break,
        }
    }
    out.push_str(&s[last..]);
    out
}

/// `re.sub(r'>(.*?)<', lambda m: '>'+g1.replace('\n','').strip()+'<', s, flags=re.DOTALL)`
fn sub_between(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    let mut p = 0;
    while let Some(q) = find_from(s, ">", p) {
        match find_from(s, "<", q + 1) {
            Some(c) => {
                out.push_str(&s[last..q]);
                out.push('>');
                out.push_str(strip(&s[q + 1..c].replace('\n', "")));
                out.push('<');
                last = c + 1;
                p = last;
            }
            None => break,
        }
    }
    out.push_str(&s[last..]);
    out
}

fn extract_and_clean_tables(text: &str) -> String {
    let mut text = text.to_string();
    if !text.contains("</table>") {
        text.push_str("</table>");
    }
    // re.findall(r'<table.*?>.*?</table>', text, re.DOTALL)
    let mut tables: Vec<&str> = Vec::new();
    let mut p = 0;
    while let Some(q) = find_from(&text, "<table", p) {
        let m = find_from(&text, ">", q + 6).and_then(|gt| find_from(&text, "</table>", gt + 1));
        match m {
            Some(c) => {
                tables.push(&text[q..c + 8]);
                p = c + 8;
            }
            None => break,
        }
    }
    if tables.len() > 1 {
        return String::new();
    }
    let mut out = String::new();
    for t in tables {
        let t = sub_lazy_nodotall(t, "<table", ">", "<table>");
        let t = sub_gt_ws_lt(&t);
        let t = sub_td(&t);
        let t = sub_between(&t);
        let t = t.replace('\n', "");
        out.push_str(strip(&t));
    }
    out
}

/// `utils/merge_tables.py::normalized_html_table` (used by relation_utils `_norm_table`).
pub fn try_normalized_html_table_relation(text: &str) -> Result<String, Error> {
    let t = process_table_html(text, false)?;
    Ok(clean_table(&extract_and_clean_tables(&t)))
}

/// See [`try_normalized_html_table_relation`]; panics where Python would raise.
pub fn normalized_html_table_relation(text: &str) -> String {
    try_normalized_html_table_relation(text).unwrap_or_else(|e| panic!("normalized_html_table raised: {}", e))
}

// ---------------------------------------------------------------- match.py

/// `[re.sub(r'\$\\cdot\$','',item.string).strip() for item in
///   BeautifulSoup(html,'html.parser').findAll('td') if item.string]`
pub fn try_td_strings(html: &str) -> Result<Vec<String>, Error> {
    let (soup, root) = Soup::parse(html);
    let mut out = Vec::new();
    for td in soup.find_all(root, &["td"])? {
        if let Some(s) = soup.string(td) {
            if !s.is_empty() {
                out.push(strip(&s.replace("$\\cdot$", "")).to_string());
            }
        }
    }
    Ok(out)
}

/// See [`try_td_strings`]; panics where Python would raise.
pub fn td_strings(html: &str) -> Vec<String> {
    try_td_strings(html).unwrap_or_else(|e| panic!("td_strings raised: {}", e))
}

// ---------------------------------------------------------------- merge_tables

fn full_to_half(s: &str) -> String {
    s.chars()
        .map(|c| {
            let u = c as u32;
            if (0xFF01..=0xFF5E).contains(&u) {
                char::from_u32(u - 0xFEE0).unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn int_attr(soup: &Soup, cell: Id, key: &str) -> Result<i128, Error> {
    match soup.get_attr(cell, key) {
        None => Ok(1),
        Some(v) => match v.as_str() {
            Some(s) => py_int(s).map_err(|m| Error::Python(format!("ValueError: {}", m))),
            None => Err(Error::Python("TypeError: int() argument must be a string, not 'list'".into())),
        },
    }
}

const CELLS: [&str; 2] = ["td", "th"];

fn detect_table_headers(soup: &Soup, s1: Id, s2: Id) -> Result<usize, Error> {
    let rows1 = soup.find_all(s1, &["tr"])?;
    let rows2 = soup.find_all(s2, &["tr"])?;
    let min_rows = rows1.len().min(rows2.len()).min(5);
    let mut header_rows = 0;
    for i in 0..min_rows {
        let cells1 = soup.find_all(rows1[i], &CELLS)?;
        let cells2 = soup.find_all(rows2[i], &CELLS)?;
        if cells1.len() != cells2.len() {
            break;
        }
        let mut matched = true;
        for (&c1, &c2) in cells1.iter().zip(cells2.iter()) {
            let t1 = pystr::remove_space(&full_to_half(&soup.get_text(c1)?));
            let t2 = pystr::remove_space(&full_to_half(&soup.get_text(c2)?));
            if t1 != t2 {
                matched = false;
                break;
            }
            let a = int_attr(soup, c1, "colspan")?;
            let b = int_attr(soup, c2, "colspan")?;
            if a != b {
                matched = false;
                break;
            }
        }
        if matched {
            header_rows += 1;
        } else {
            break;
        }
    }
    Ok(header_rows)
}

fn calculate_table_total_columns(soup: &Soup, root: Id) -> Result<i128, Error> {
    let rows = soup.find_all(root, &["tr"])?;
    if rows.is_empty() {
        return Ok(0);
    }
    let mut max_cols: i128 = 0;
    let mut occupied: HashMap<i128, HashSet<i128>> = HashMap::new();
    for (row_idx, &row) in rows.iter().enumerate() {
        let row_idx = row_idx as i128;
        let mut col_idx: i128 = 0;
        let cells = soup.find_all(row, &CELLS)?;
        occupied.entry(row_idx).or_default();
        for cell in cells {
            while occupied[&row_idx].contains(&col_idx) {
                col_idx += 1;
            }
            let colspan = int_attr(soup, cell, "colspan")?;
            let rowspan = int_attr(soup, cell, "rowspan")?;
            if rowspan.max(0).saturating_mul(colspan.max(0)) > 50_000_000 {
                return Err(Error::Python("span too large (Python would not finish in reasonable time)".into()));
            }
            let mut r = row_idx;
            while r < row_idx + rowspan {
                let e = occupied.entry(r).or_default();
                let mut c = col_idx;
                while c < col_idx + colspan {
                    e.insert(c);
                    c += 1;
                }
                r += 1;
            }
            col_idx += colspan;
            max_cols = max_cols.max(col_idx);
        }
    }
    Ok(max_cols)
}

fn calculate_row_columns(soup: &Soup, row: Id) -> Result<i128, Error> {
    let mut sum = 0i128;
    for c in soup.find_all(row, &CELLS)? {
        sum += int_attr(soup, c, "colspan")?;
    }
    Ok(sum)
}

fn check_rows_match(soup: &Soup, s1: Id, s2: Id) -> Result<bool, Error> {
    let rows1 = soup.find_all(s1, &["tr"])?;
    let rows2 = soup.find_all(s2, &["tr"])?;
    if rows1.is_empty() || rows2.is_empty() {
        return Ok(false);
    }
    let last_row = *rows1.last().unwrap();
    let header_count = detect_table_headers(soup, s1, s2)?;
    if rows2.len() <= header_count {
        return Ok(false);
    }
    let first_data_row = rows2[header_count];
    let last_cols = calculate_row_columns(soup, last_row)?;
    let first_cols = calculate_row_columns(soup, first_data_row)?;
    let last_visual = soup.find_all(last_row, &CELLS)?.len();
    let first_visual = soup.find_all(first_data_row, &CELLS)?.len();
    Ok(last_cols == first_cols || last_visual == first_visual)
}

fn perform_table_merge(soup: &mut Soup, prev: Id, curr: Id) -> Result<String, Error> {
    let header_count = detect_table_headers(soup, prev, curr)?;
    let rows_prev = soup.find_all(prev, &["tr"])?;
    let rows_curr = soup.find_all(curr, &["tr"])?;
    for &row in rows_curr.iter().skip(header_count) {
        soup.extract(row, None)?;
        let Some(&last) = rows_prev.last() else {
            return Err(Error::Python("IndexError: list index out of range".into()));
        };
        let Some(parent) = soup.nodes[last].parent else {
            return Err(Error::Python("AttributeError: 'NoneType' object has no attribute 'append'".into()));
        };
        soup.append(parent, row)?;
    }
    soup.decode(prev)
}

/// `utils/merge_tables.py::merge_tables(table_lists)` (its `print`s are omitted).
pub fn merge_tables(tables: &[String]) -> Result<String, Error> {
    if tables.len() == 1 {
        return Ok(tables[0].clone());
    }
    if tables.is_empty() {
        return Ok(String::new());
    }
    let mut curr_html = tables[tables.len() - 1].clone();
    for i in (0..tables.len() - 1).rev() {
        let prev_html = &tables[i];
        if prev_html.is_empty() || curr_html.is_empty() {
            continue;
        }
        let mut soup = Soup::new();
        soup.base_frames = CALLER_DEPTH_MERGE.load(Ordering::Relaxed) + 2;
        let sp = soup.parse_doc(prev_html);
        let sc = soup.parse_doc(&curr_html);
        let cols_prev = calculate_table_total_columns(&soup, sp)?;
        let cols_curr = calculate_table_total_columns(&soup, sc)?;
        let tables_match = cols_prev == cols_curr;
        let rows_match = check_rows_match(&soup, sp, sc)?;
        if tables_match || rows_match {
            curr_html = perform_table_merge(&mut soup, sp, sc)?;
        }
    }
    Ok(curr_html)
}
