//! Port of `utils/extract.py` (md_tex_filter, extract_figure_boxes, remove_latex_style)
//! and the Markdown-table conversion of `utils/table_utils.py`.
//! Positions are Python code point indices.

use crate::py;
use crate::pyval::{Map, V};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! re {
    ($name:ident, $pat:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pat).unwrap());
    };
}

// Python `\s` / `\w`
const S: &str = r"[\s\x1c-\x1f]";
const W: &str = r"[\p{L}\p{N}_]";

re!(IMG, r"!\[.*?\]\(.*?\)");
re!(FIG_BBOX, &format!(r"page(\d+)_(\d+)_(\d+)_(\d+)_(\d+)\.{W}+"));
re!(FENCE_MD, r"(?m)^```markdown\n?");
re!(FENCE_HTML, r"(?m)^```html\n?");
re!(FENCE_LATEX, r"(?m)^```latex\n?");
re!(FENCE_END, r"(?m)```\n?$");
re!(US4, r"_{4,}");
re!(SP4, r" {4,}");
re!(DISPLAY, r"(?s)\$\$(.*?)\$\$|\\\[(.*?)\\\]|\$(.*?)\$|\\\((.*?)\\\)");
re!(DOLLAR, r"(?s)\$\$(.*?)\$\$|\$(.*?)\$|\\\((.*?)\\\)");
re!(MD_TABLE, &format!(r"(?s)\|{S}*.*?{S}*\|\n"));
re!(HTML_TABLE, r"(?s)(<table.*?</table>)");
re!(CODE_BLOCK, &format!(r"(?s)```({W}+)\n(.*?)```"));
re!(TEX_TABLE, r"(?s)\\begin\{table\}(.*?)\\end\{table\}");
re!(HTML_BEGIN, r"<table(?:[^>]*)>");
re!(TITLE1, r"\\title\{(.*?)\}");
re!(TITLE2, &format!(r"(?s)\\title{S}*\{{{S}*(.*?){S}*\}}"));
re!(TEXT2, &format!(r"(?s)\\text{S}*\{{{S}*(.*?){S}*\}}"));
re!(SECTION1, r"\\section\*?\{(.*?)\}");
re!(SECTION2, &format!(r"(?s)\\section\*?\{{{S}*(.*?){S}*\}}"));
// table_utils
re!(COMMENT, r"<!--[\s\S]*?-->");
re!(TU_TABLE_BLOCK, r"<table>[\s\S]*?</table>");
re!(TU_TABLE_HEAD_BODY, r"</?t(able|head|body)>");

/// Python `re.sub(pattern, r'\1', s)` (group 1 always participates in these patterns).
fn sub_group1(re: &Regex, s: &str) -> String {
    re.replace_all(s, |c: &regex::Captures| c.get(1).map_or(String::new(), |m| m.as_str().to_string())).into_owned()
}

/// `remove_latex_style` (extract.py; also used by head_metric.py for gold heading text).
pub fn remove_latex_style(content: &str) -> String {
    let c = sub_group1(&TITLE1, content);
    let c = sub_group1(&TITLE2, &c);
    let c = sub_group1(&TEXT2, &c);
    let c = sub_group1(&SECTION1, &c);
    sub_group1(&SECTION2, &c)
}

pub fn remove_markdown_fences(content: &str) -> String {
    let c = FENCE_MD.replace_all(content, "");
    let c = FENCE_HTML.replace_all(&c, "");
    let c = FENCE_LATEX.replace_all(&c, "");
    FENCE_END.replace_all(&c, "").into_owned()
}

pub fn replace_repeated_chars(s: &str) -> String {
    let c = US4.replace_all(s, "____");
    SP4.replace_all(&c, "    ").into_owned()
}

/// Value of a string of Unicode decimal digits (Python `int()` of a `\d+` match).
fn py_int(s: &str) -> i64 {
    use unicode_general_category::{get_general_category, GeneralCategory};
    let mut v: i64 = 0;
    for c in s.chars() {
        let d = if c.is_ascii_digit() {
            c as i64 - '0' as i64
        } else {
            // Nd characters come in runs of ten starting at digit zero
            let mut start = c as u32;
            while start > 0 {
                match char::from_u32(start - 1) {
                    Some(p) if get_general_category(p) == GeneralCategory::DecimalNumber => start -= 1,
                    _ => break,
                }
            }
            ((c as u32 - start) % 10) as i64
        };
        v = v.saturating_mul(10).saturating_add(d);
    }
    v
}

pub struct FigureBox {
    pub page_id: i64,
    pub bbox: [i64; 4],
}

pub fn extract_figure_boxes(content: &str) -> Vec<FigureBox> {
    let mut boxes = Vec::new();
    for m in IMG.find_iter(content) {
        if let Some(c) = FIG_BBOX.captures(m.as_str()) {
            boxes.push(FigureBox {
                page_id: py_int(&c[1]),
                bbox: [py_int(&c[2]), py_int(&c[3]), py_int(&c[4]), py_int(&c[5])],
            });
        }
    }
    boxes
}

/// Char-indexed text buffer: Python string positions and same-length space replacement.
struct Buf {
    chars: Vec<char>,
}

impl Buf {
    fn new(s: &str) -> Buf {
        Buf { chars: s.chars().collect() }
    }
    fn string(&self) -> String {
        self.chars.iter().collect()
    }
    fn blank(&mut self, a: usize, b: usize) {
        let b = b.min(self.chars.len());
        for c in &mut self.chars[a..b] {
            *c = ' ';
        }
    }
}

/// Find `needle` (literal) in `hay[from..]`, returning the char index.
fn find_chars(hay: &[char], needle: &[char], from: usize) -> Option<usize> {
    if needle.is_empty() {
        return Some(from);
    }
    if hay.len() < needle.len() {
        return None;
    }
    (from..=hay.len() - needle.len()).find(|&i| hay[i..i + needle.len()] == *needle)
}

/// `extract_tabular` (latex) — a bracket matcher with the original's position arithmetic.
fn extract_tabular(text: &[char]) -> (Vec<String>, Vec<(usize, usize)>) {
    let begin: Vec<char> = r"\begin{tabular}".chars().collect();
    let end: Vec<char> = r"\end{tabular}".chars().collect();
    // len() of the raw *patterns* in the Python source (escaped backslash counts twice)
    let len_begin_pat = 16;
    let len_end_pat = 14;
    let mut tabulars = Vec::new();
    let mut positions = Vec::new();
    let mut cur = 0usize;
    let mut stack: Vec<usize> = Vec::new();
    while cur < text.len() {
        let b = find_chars(text, &begin, cur);
        let e = find_chars(text, &end, cur);
        if b.is_none() && e.is_none() {
            break;
        }
        if b.is_some() && (e.is_none() || b.unwrap() < e.unwrap()) {
            let b = b.unwrap();
            stack.push(b);
            cur = b + len_end_pat;
        } else if let Some(e) = e {
            if let Some(start) = stack.pop() {
                if stack.is_empty() {
                    let end_pos = e + len_end_pat;
                    tabulars.push(text[start..end_pos.min(text.len())].iter().collect());
                    positions.push((start, end_pos));
                }
            }
            cur = e + len_end_pat;
        } else {
            cur += 1;
        }
    }
    if !stack.is_empty() {
        let new_start = stack[0] + len_begin_pat;
        if new_start <= text.len() {
            let (t, p) = extract_tabular(&text[new_start..]);
            tabulars.extend(t);
            positions.extend(p.into_iter().map(|(s, e)| (s + new_start, e + new_start)));
        } else {
            let (t, p) = extract_tabular(&[]);
            tabulars.extend(t);
            positions.extend(p);
        }
    }
    let _ = end;
    (tabulars, positions)
}

fn extract_tex_table(content: &[char]) -> Vec<((usize, usize), String)> {
    let s: String = content.iter().collect();
    let ci = py::CharIndex::new(&s);
    let mut tables = Vec::new();
    let mut work = content.to_vec();
    for m in TEX_TABLE.find_iter(&s) {
        let (a, b) = (ci.char_of(m.start()), ci.char_of(m.end()));
        tables.push(((a, b), m.as_str().to_string()));
        for c in &mut work[a..b] {
            *c = ' ';
        }
    }
    let (tabs, pos) = extract_tabular(&work);
    let mut all: Vec<((usize, usize), String)> = tables;
    all.extend(pos.into_iter().zip(tabs));
    all.sort_by_key(|x| x.0 .0); // stable, like Python's sorted
    all
}

/// `extract_html_table` — same bracket matcher with a regex opening tag.
fn extract_html_table(text: &[char]) -> (Vec<String>, Vec<(usize, usize)>) {
    let s: String = text.iter().collect();
    let ci = py::CharIndex::new(&s);
    let end: Vec<char> = "</table>".chars().collect();
    let len_begin_pat = 16; // len(r'<table(?:[^>]*)>')
    let len_end_pat = 8;
    let mut tabulars = Vec::new();
    let mut positions = Vec::new();
    let mut cur = 0usize;
    let mut stack: Vec<usize> = Vec::new();
    while cur < text.len() {
        let bstart = py::byte_of_char(&s, cur);
        let b = HTML_BEGIN.find_at(&s, bstart).map(|m| ci.char_of(m.start()));
        let e = find_chars(text, &end, cur);
        if b.is_none() && e.is_none() {
            break;
        }
        if b.is_some() && (e.is_none() || b.unwrap() < e.unwrap()) {
            let b = b.unwrap();
            stack.push(b);
            cur = b + len_end_pat;
        } else if let Some(e) = e {
            if let Some(start) = stack.pop() {
                if stack.is_empty() {
                    let end_pos = e + len_end_pat;
                    tabulars.push(text[start..end_pos.min(text.len())].iter().collect());
                    positions.push((start, end_pos));
                }
            }
            cur = e + len_end_pat;
        } else {
            cur += 1;
        }
    }
    if !stack.is_empty() {
        let new_start = (stack[0] + len_begin_pat).min(text.len());
        let (t, p) = extract_html_table(&text[new_start..]);
        tabulars.extend(t);
        positions.extend(p.into_iter().map(|(s, e)| (s + new_start, e + new_start)));
    }
    (tabulars, positions)
}

// ---------------------------------------------------------------- table_utils.py

fn markdown_to_html(markdown_table: &str) -> String {
    let rows: Vec<&str> = py::strip(markdown_table).split('\n').map(py::strip).collect();
    let mut html = String::from("<table>\n  <thead>\n    <tr>\n");
    let cells = |row: &str| -> Vec<String> {
        let parts: Vec<&str> = row.split('|').collect();
        if parts.len() < 2 {
            return Vec::new();
        }
        parts[1..parts.len() - 1].iter().map(|c| py::strip(c).to_string()).collect()
    };
    for c in cells(rows[0]) {
        html.push_str(&format!("      <th>{}</th>\n", c));
    }
    html.push_str("    </tr>\n  </thead>\n  <tbody>\n");
    for row in rows.iter().skip(2) {
        html.push_str("    <tr>\n");
        for c in cells(row) {
            html.push_str(&format!("      <td>{}</td>\n", c));
        }
        html.push_str("    </tr>\n");
    }
    html.push_str("  </tbody>\n</table>\n");
    html
}

fn find_md_table_mode(line: &str) -> bool {
    // re.search(r'-*?:') / r'---' / r':-*?'
    line.contains(':') || line.contains("---")
}

fn tu_merge_tables(input: &str) -> String {
    let input = COMMENT.replace_all(input, "");
    let mut output_lines: Vec<String> = Vec::new();
    for block in TU_TABLE_BLOCK.find_iter(&input) {
        let mut lines: Vec<String> = block.as_str().split('\n').map(|l| l.to_string()).collect();
        for l in lines.iter_mut() {
            if l.contains("<th>") {
                *l = l.replace("<th>", "<td>").replace("</th>", "</td>");
            }
        }
        let final_tr: Vec<String> = lines.into_iter().filter(|l| !TU_TABLE_HEAD_BODY.is_match(l)).collect();
        if final_tr.len() > 2 {
            output_lines.extend(final_tr);
        }
    }
    format!("\n\n<table>\n{}\n</table>\n\n", output_lines.join("\n"))
}

fn replace_table_with_placeholder(input: &str) -> String {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut output_lines: Vec<String> = Vec::new();
    let mut in_table_block = false;
    let mut temp_block = String::new();
    let mut last_line: &str = "";
    for line in lines.iter() {
        if line.contains("<table>") {
            in_table_block = true;
            temp_block.push_str(last_line);
        } else if in_table_block {
            if !find_md_table_mode(last_line) && !last_line.contains("</thead>") {
                temp_block.push('\n');
                temp_block.push_str(last_line);
            }
            if last_line.contains("</table>") && !line.contains("<table>") {
                in_table_block = false;
                output_lines.push(tu_merge_tables(&temp_block));
                temp_block.clear();
            }
        } else {
            output_lines.push(last_line.to_string());
        }
        last_line = line;
    }
    if !last_line.is_empty() {
        if in_table_block || last_line.contains("</table>") {
            temp_block.push('\n');
            temp_block.push_str(last_line);
            output_lines.push(tu_merge_tables(&temp_block));
        } else {
            output_lines.push(last_line.to_string());
        }
    }
    output_lines.join("\n")
}

fn convert_markdown_to_html(content: &str) -> String {
    let mut md = content.replace('\r', "");
    md.push('\n');
    let matches: Vec<String> = MD_TABLE.find_iter(&md).map(|m| m.as_str().to_string()).collect();
    for m in matches {
        let html = markdown_to_html(&m);
        md = py::replace_first(&md, &m, &html);
    }
    replace_table_with_placeholder(&md)
        .replace("<table>", "<table border=\"1\" >")
        .replace("<td>", "<td colspan=\"1\" rowspan=\"1\">")
}

// ---------------------------------------------------------------- md_tex_filter

fn item(category: &str, pos: (usize, usize), content: &str, extra: &[(&str, V)]) -> V {
    let mut m = Map::new();
    m.insert("category_type".into(), V::str(category));
    m.insert("position".into(), V::list(vec![V::from(pos.0), V::from(pos.1)]));
    m.insert("content".into(), V::str(content));
    for (k, v) in extra {
        m.insert((*k).into(), v.clone());
    }
    V::from(m)
}

/// Python `re.sub(DOLLAR, template, s)` where the template is `\\[<groups>\\]`.
fn dollar_sub(s: &str, groups: &[usize]) -> String {
    DOLLAR
        .replace_all(s, |c: &regex::Captures| {
            let mut out = String::from("\\[");
            for &g in groups {
                if let Some(m) = c.get(g) {
                    out.push_str(m.as_str());
                }
            }
            out.push_str("\\]");
            out
        })
        .into_owned()
}

/// `md_tex_filter(content)`: returns the categories in first-appearance order (a Python
/// dict/defaultdict keeps that order), each with its items sorted by position.
pub fn md_tex_filter(content: &str) -> Vec<(String, Vec<V>)> {
    let content = IMG.replace_all(content, "");
    let content = remove_markdown_fences(&content);
    let content = replace_repeated_chars(&content);
    let content = content.replace("<html>", "").replace("</html>", "").replace("<body>", "").replace("</body>", "");

    let mut pred_all: Vec<V> = Vec::new();
    let mut buf = Buf::new(&content);

    // latex tables
    for ((p0, _), table) in extract_tex_table(&buf.chars) {
        let p1 = p0 + py::len(&table);
        pred_all.push(item("latex_table", (p0, p1), &table, &[]));
        buf.blank(p0, p1);
    }
    // html tables
    let (tables, positions) = extract_html_table(&buf.chars);
    for (table, (p0, _)) in tables.iter().zip(positions) {
        let p1 = p0 + py::len(table);
        pred_all.push(item("html_table", (p0, p1), table, &[]));
        buf.blank(p0, p1);
    }
    // display / inline formulas (iterating over a snapshot, as finditer does)
    let snapshot = buf.string();
    let ci = py::CharIndex::new(&snapshot);
    for m in DISPLAY.find_iter(&snapshot) {
        let matched = m.as_str();
        if matched.is_empty() {
            continue;
        }
        let single_line = py::strip(matched).split('\n').collect::<Vec<_>>().join(" ");
        let pos = (ci.char_of(m.start()), ci.char_of(m.end()));
        match DOLLAR.captures(&single_line) {
            None => {
                buf.blank(pos.0, pos.1);
                pred_all.push(item("equation_isolated", pos, &single_line, &[]));
            }
            Some(c) if c.get(1).map_or(false, |g| !g.as_str().is_empty()) => {
                let sl = dollar_sub(&single_line, &[1]);
                buf.blank(pos.0, pos.1);
                pred_all.push(item("equation_isolated", pos, &sl, &[]));
            }
            Some(_) => {
                let sl = dollar_sub(&single_line, &[2, 3]);
                pred_all.push(item(
                    "equation_isolated",
                    pos,
                    &sl,
                    &[("fine_category_type", V::str("equation_inline"))],
                ));
            }
        }
    }
    let mut content = buf.string();

    // Markdown tables
    let probe = format!("{}\n", content);
    if MD_TABLE.find_iter(&probe).take(2).count() >= 2 {
        content = convert_markdown_to_html(&content);
        let mut b = Buf::new(&content);
        let ci = py::CharIndex::new(&content);
        for m in HTML_TABLE.find_iter(&content) {
            let pos = (ci.char_of(m.start()), ci.char_of(m.end()));
            b.blank(pos.0, pos.1);
            pred_all.push(item(
                "html_table",
                pos,
                py::strip(m.as_str()),
                &[("fine_category_type", V::str("md2html_table"))],
            ));
        }
        content = b.string();
    }

    // code blocks
    {
        let mut b = Buf::new(&content);
        let ci = py::CharIndex::new(&content);
        for c in CODE_BLOCK.captures_iter(&content) {
            let m = c.get(0).unwrap();
            let pos = (ci.char_of(m.start()), ci.char_of(m.end()));
            let language = c.get(1).unwrap().as_str();
            let code = py::strip(c.get(2).unwrap().as_str());
            b.blank(pos.0, pos.1);
            pred_all.push(item(
                "text_all",
                pos,
                code,
                &[("language", V::str(language)), ("fine_category_type", V::str("code"))],
            ));
        }
        content = b.string();
    }

    let content = remove_latex_style(&content);

    // texts
    let mut res: Vec<&str> = content.split("\n\n").collect();
    if res.len() == 1 {
        res = content.split('\n').collect();
    }
    let mut content_position = 0usize;
    for text in res {
        let n = py::len(text);
        let position = (content_position, content_position + n);
        content_position += n;
        let t = py::strip(text);
        let t = py::strip_chars(t, "\n");
        let t: Vec<&str> = t.split('\n').filter(|l| !py::strip(l).is_empty()).map(py::strip).collect();
        let t = t.join("\n");
        if !t.is_empty() {
            if t.starts_with("<table") && t.ends_with("</table>") {
                pred_all.push(item("html_table", position, &t, &[]));
            } else if t.starts_with('$') && t.ends_with('$') {
                if !py::strip(&t.replace('$', "")).is_empty() {
                    pred_all.push(item("equation_isolated", position, py::strip(&t), &[]));
                }
            } else {
                let t = py::strip(&t);
                if !t.is_empty() {
                    pred_all.push(item("text_all", position, t, &[("fine_category_type", V::str("text_block"))]));
                }
            }
        }
    }

    pred_all.sort_by_key(|x| x.get("position").as_list().unwrap()[0].as_i64().unwrap());
    let mut out: Vec<(String, Vec<V>)> = Vec::new();
    for it in pred_all {
        let cat = it.get("category_type").as_str().unwrap().to_string();
        match out.iter_mut().find(|(c, _)| *c == cat) {
            Some((_, v)) => v.push(it),
            None => out.push((cat, vec![it])),
        }
    }
    out
}

pub fn category<'a>(pd: &'a [(String, Vec<V>)], name: &str) -> &'a [V] {
    pd.iter().find(|(c, _)| c == name).map(|(_, v)| v.as_slice()).unwrap_or(&[])
}
