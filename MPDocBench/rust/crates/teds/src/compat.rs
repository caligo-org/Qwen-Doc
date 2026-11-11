//! Input rewriting that makes an older libxml2 HTML parser (validated: 2.9.13, as shipped
//! with macOS) build the tree libxml2 2.12.9 (bundled with lxml 5.3.0) builds.
//!
//! The rewrite scans the input the way the HTML parser tokenises it (start tags with
//! attribute values, end tags, comments, processing instructions, `<!...>`, script/style raw
//! text) and only changes markup in content:
//!
//! * `<` not followed by an ASCII letter, `/`, `!` or `?` is text in 2.12 but starts a
//!   bogus tag (dropped up to the next `>`) in 2.9: it becomes `&lt;`.
//! * Comments end where 2.12 ends them (`-->`, `--!>`, and the abrupt `<!-->` / `<!--->`);
//!   each complete comment becomes `<!---->` (comments are dropped from the tree anyway).
//! * A DOCTYPE declaration becomes `<!---->` (2.9 resynchronises differently after one).
//! * 2.9 drops the prefix of element names like `o:p` (it splits them as qualified names);
//!   2.12 keeps `o:p`. A `:` in a tag name becomes [`COLON`], which [`restore_name`] maps back.

/// Stands for `:` in element names (made of HTML name characters, so the parser keeps it).
pub const COLON: &str = "_-_-_";

/// The element name 2.12 would have produced from a name parsed out of rewritten input.
pub fn restore_name(name: &str) -> String {
    name.replace(COLON, ":")
}

/// Copies a tag name, replacing `:` by [`COLON`].
fn push_name(out: &mut String, c: &[char], from: usize, to: usize) {
    for &ch in &c[from..to] {
        if ch == ':' {
            out.push_str(COLON);
        } else {
            out.push(ch);
        }
    }
}

fn is_blank(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

fn is_name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == ':' || c == '.'
}

fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, ':' | '-' | '_' | '.')
}

/// Index just past the next `>` at or after `j`, or the end.
fn past_gt(c: &[char], mut j: usize) -> usize {
    while j < c.len() {
        if c[j] == '>' {
            return j + 1;
        }
        j += 1;
    }
    j
}

/// End of a comment starting at `i` (`<!--`) by libxml2 2.12's rules, or `None` if it is
/// not terminated (it then swallows the rest of the input).
fn comment_end(c: &[char], i: usize) -> Option<usize> {
    let s = &c[i + 4..];
    let base = i + 4;
    match s.first() {
        None => return None,
        Some('>') => return Some(base + 1),
        _ => {}
    }
    match s.get(1) {
        None => return None,
        Some('>') if s[0] == '-' => return Some(base + 2),
        _ => {}
    }
    let mut k = 2;
    while k < s.len() {
        if s[k] == '>' && s[k - 1] == '-' && s[k - 2] == '-' {
            return Some(base + k + 1);
        }
        if s[k] == '!' && s[k - 1] == '-' && s[k - 2] == '-' && s.get(k + 1) == Some(&'>') {
            return Some(base + k + 2);
        }
        k += 1;
    }
    None
}

/// Scans a start tag at `i` (`<` + ASCII letter) as libxml2's `htmlParseStartTag` does.
/// Returns (end index, lowercased name, self-closing).
fn start_tag(c: &[char], i: usize) -> (usize, String, bool) {
    let mut j = i + 1;
    let mut name = String::new();
    while j < c.len() && is_name_char(c[j]) {
        name.push(c[j].to_ascii_lowercase());
        j += 1;
    }
    loop {
        while j < c.len() && is_blank(c[j]) {
            j += 1;
        }
        if j >= c.len() {
            return (j, name, false);
        }
        if c[j] == '>' {
            return (j + 1, name, false);
        }
        if c[j] == '/' && c.get(j + 1) == Some(&'>') {
            return (j + 2, name, true);
        }
        if is_name_start(c[j]) {
            while j < c.len() && is_name_char(c[j]) {
                j += 1;
            }
            while j < c.len() && is_blank(c[j]) {
                j += 1;
            }
            if c.get(j) == Some(&'=') {
                j += 1;
                while j < c.len() && is_blank(c[j]) {
                    j += 1;
                }
                match c.get(j) {
                    Some(&q) if q == '"' || q == '\'' => {
                        j += 1;
                        while j < c.len() && c[j] != q {
                            j += 1;
                        }
                        if j < c.len() {
                            j += 1;
                        }
                    }
                    _ => {
                        while j < c.len() && !is_blank(c[j]) && c[j] != '>' {
                            j += 1;
                        }
                    }
                }
            }
        } else {
            // bogus attribute: dumped up to a blank or the end of the tag
            while j < c.len()
                && !is_blank(c[j])
                && c[j] != '>'
                && !(c[j] == '/' && c.get(j + 1) == Some(&'>'))
            {
                j += 1;
            }
        }
    }
}

/// End of the HTML name starting at `j`.
fn name_end(c: &[char], mut j: usize) -> usize {
    while j < c.len() && is_name_char(c[j]) {
        j += 1;
    }
    j
}

fn starts_with_ci(c: &[char], at: usize, word: &str) -> bool {
    let w: Vec<char> = word.chars().collect();
    at + w.len() <= c.len()
        && c[at..at + w.len()]
            .iter()
            .zip(&w)
            .all(|(a, b)| a.to_ascii_lowercase() == *b)
}

/// Elements libxml2 never keeps open (`htmlNoContentElements`-like empty elements).
const VOID: &[&str] = &[
    "area", "base", "basefont", "br", "col", "embed", "frame", "hr", "img", "input", "isindex",
    "keygen", "link", "meta", "param", "source", "track", "wbr",
];

pub fn emulate_libxml2_2_12(s: &str) -> String {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    let mut out = String::with_capacity(s.len() + 16);
    let mut raw: Option<&'static str> = None;
    let mut open: Vec<String> = Vec::new();
    let mut i = 0;
    let push = |out: &mut String, from: usize, to: usize| out.extend(&c[from..to]);
    while i < n {
        if let Some(name) = raw {
            // The content loop looks for an end tag before it hands over to the script
            // parser, so `</` right at the start of the raw text is markup.
            if c[i] == '<' && c.get(i + 1) == Some(&'/') {
                if c.get(i + 2).is_some_and(|&x| is_name_start(x)) {
                    // An end tag of the script/style or of an ancestor closes it; one for
                    // an element that is not open is ignored and the raw text goes on.
                    // (`open` is a rough model of the parser's element stack.)
                    let name_end = name_end(&c, i + 2);
                    let end = past_gt(&c, name_end);
                    out.push_str("</");
                    push_name(&mut out, &c, i + 2, name_end);
                    push(&mut out, name_end, end);
                    let tag: String = c[i + 2..name_end]
                        .iter()
                        .map(|x| x.to_ascii_lowercase())
                        .collect();
                    if let Some(k) = open.iter().rposition(|t| *t == tag) {
                        open.truncate(k);
                        raw = None;
                    }
                    i = end;
                } else {
                    push(&mut out, i, i + 2);
                    i += 2;
                }
                continue;
            }
            // script/style content runs to `</` + the element name (recovery mode).
            let mut j = i;
            while j < n
                && !(c[j] == '<' && c.get(j + 1) == Some(&'/') && starts_with_ci(&c, j + 2, name))
            {
                j += 1;
            }
            push(&mut out, i, j);
            i = j;
            if i >= n {
                break;
            }
            // the end tag; raw text resumes unless it closes the element
            let mut k = i + 2;
            let mut tag = String::new();
            while k < n && is_name_char(c[k]) {
                tag.push(c[k].to_ascii_lowercase());
                k += 1;
            }
            let end = past_gt(&c, k);
            push(&mut out, i, end);
            i = end;
            if tag == name {
                raw = None;
                if let Some(k) = open.iter().rposition(|t| t == name) {
                    open.truncate(k);
                }
            }
            continue;
        }
        if c[i] != '<' {
            out.push(c[i]);
            i += 1;
            continue;
        }
        match c.get(i + 1).copied() {
            Some('!') => {
                if c.get(i + 2) == Some(&'-') && c.get(i + 3) == Some(&'-') {
                    match comment_end(&c, i) {
                        Some(end) => {
                            out.push_str("<!---->");
                            i = end;
                        }
                        None => {
                            push(&mut out, i, n);
                            i = n;
                        }
                    }
                } else if starts_with_ci(&c, i + 2, "doctype") {
                    // 2.12 skips a (misplaced) DOCTYPE up to the next `>`; 2.9 resynchronises
                    // differently. It adds nothing to the tree, so an empty comment stands in.
                    out.push_str("<!---->");
                    i = past_gt(&c, i + 2);
                } else {
                    let end = past_gt(&c, i + 2);
                    push(&mut out, i, end);
                    i = end;
                }
            }
            Some('?') => {
                let start = c
                    .get(i + 2)
                    .is_some_and(|&x| x == '_' || x == ':' || x.is_alphabetic());
                let end = if start { past_gt(&c, i + 2) } else { i + 2 };
                push(&mut out, i, end);
                i = end;
            }
            Some('/') => {
                if c.get(i + 2).is_some_and(|&x| is_name_start(x)) {
                    let name_end = name_end(&c, i + 2);
                    let end = past_gt(&c, name_end);
                    out.push_str("</");
                    push_name(&mut out, &c, i + 2, name_end);
                    push(&mut out, name_end, end);
                    let tag: String = c[i + 2..name_end]
                        .iter()
                        .map(|x| x.to_ascii_lowercase())
                        .collect();
                    if let Some(k) = open.iter().rposition(|t| *t == tag) {
                        open.truncate(k);
                    }
                    i = end;
                } else {
                    push(&mut out, i, i + 2);
                    i += 2;
                }
            }
            Some(x) if x.is_ascii_alphabetic() => {
                let (end, name, self_closing) = start_tag(&c, i);
                let name_end = name_end(&c, i + 1);
                out.push('<');
                push_name(&mut out, &c, i + 1, name_end);
                push(&mut out, name_end, end);
                i = end;
                if !self_closing && !VOID.contains(&name.as_str()) {
                    open.push(name.clone());
                }
                if !self_closing && end <= n && c[end - 1] == '>' {
                    raw = match name.as_str() {
                        "script" => Some("script"),
                        "style" => Some("style"),
                        _ => None,
                    };
                }
            }
            _ => {
                out.push_str("&lt;");
                i += 1;
            }
        }
    }
    out
}
