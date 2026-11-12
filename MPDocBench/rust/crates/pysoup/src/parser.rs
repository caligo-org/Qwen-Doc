//! Port of CPython 3.11.18 `html/parser.py` (`HTMLParser`) as used by
//! BeautifulSoup's `html.parser` builder, i.e. with `convert_charrefs=False`
//! and `scripting=False`, driven as `feed(markup)` followed by `close()`.
//!
//! The input is handled as a sequence of code points (Python `str` indexing);
//! every regular expression of the original is hand-translated, including its
//! backtracking behaviour.

use crate::pystr;
use crate::unescape::html_unescape;

/// Event sink (the overridable `handle_*` methods).
pub trait Sink {
    fn handle_starttag(&mut self, tag: String, attrs: Vec<(String, Option<String>)>);
    fn handle_startendtag(&mut self, tag: String, attrs: Vec<(String, Option<String>)>);
    fn handle_endtag(&mut self, tag: String);
    fn handle_data(&mut self, data: String);
    fn handle_charref(&mut self, name: String);
    fn handle_entityref(&mut self, name: String);
    fn handle_comment(&mut self, data: String);
    fn handle_decl(&mut self, decl: String);
    fn handle_pi(&mut self, data: String);
    fn unknown_decl(&mut self, data: String);
}

const CDATA_CONTENT_ELEMENTS: [&str; 6] = ["script", "style", "xmp", "iframe", "noembed", "noframes"];
const RCDATA_CONTENT_ELEMENTS: [&str; 2] = ["textarea", "title"];

#[inline]
fn is_ws5(c: char) -> bool {
    // [\t\n\r\f ]
    matches!(c, '\t' | '\n' | '\r' | '\x0c' | ' ')
}
#[inline]
fn is_ws5_slash(c: char) -> bool {
    // [\t\n\r\f /]
    matches!(c, '\t' | '\n' | '\r' | '\x0c' | ' ' | '/')
}

fn collect(s: &[char]) -> String {
    s.iter().collect()
}

pub struct HtmlParser {
    raw: Vec<char>,
    /// start of the unprocessed part of `raw` (Python's `self.rawdata`)
    start: usize,
    cdata_elem: Option<String>,
    escapable: bool,
}

impl HtmlParser {
    /// `p = HTMLParser(convert_charrefs=False); p.feed(markup); p.close()`
    pub fn run<S: Sink>(markup: &str, sink: &mut S) {
        let mut p = HtmlParser {
            raw: markup.chars().collect(),
            start: 0,
            cdata_elem: None,
            escapable: true,
        };
        if !p.raw.is_empty() {
            // feed(): threshold is 1, so a non-empty chunk is parsed at once.
            p.goahead(false, sink);
        }
        p.goahead(true, sink);
    }

    #[inline]
    fn at(&self, i: usize) -> Option<char> {
        self.raw.get(i).copied()
    }

    fn startswith(&self, i: usize, pat: &str) -> bool {
        let mut k = i;
        for c in pat.chars() {
            if self.at(k) != Some(c) {
                return false;
            }
            k += 1;
        }
        true
    }

    fn find_char(&self, c: char, from: usize) -> Option<usize> {
        if from >= self.raw.len() {
            return None;
        }
        self.raw[from..].iter().position(|&x| x == c).map(|p| p + from)
    }

    fn find_str(&self, pat: &str, from: usize) -> Option<usize> {
        let p: Vec<char> = pat.chars().collect();
        let n = self.raw.len();
        if from > n || n - from < p.len() {
            return None;
        }
        (from..=n - p.len()).find(|&i| self.raw[i..i + p.len()] == p[..])
    }

    fn set_cdata_mode(&mut self, elem: &str, escapable: bool) {
        self.cdata_elem = Some(pystr::lower(elem));
        self.escapable = escapable;
    }

    fn clear_cdata_mode(&mut self) {
        self.cdata_elem = None;
        self.escapable = true;
    }

    /// `</elem(?=[\t\n\r\f />])`, IGNORECASE|ASCII, at position i
    fn match_cdata_end(&self, i: usize, elem: &str) -> bool {
        if !self.startswith(i, "</") {
            return false;
        }
        let mut k = i + 2;
        for e in elem.chars() {
            match self.at(k) {
                Some(c) if c.is_ascii() && c.eq_ignore_ascii_case(&e) => {}
                Some(c) if c == e => {}
                _ => return false,
            }
            k += 1;
        }
        matches!(self.at(k), Some(c) if is_ws5_slash(c) || c == '>')
    }

    /// `self.interesting.search(rawdata, i)` -> start of match
    fn interesting_search(&self, i: usize) -> Option<usize> {
        let n = self.raw.len();
        match &self.cdata_elem {
            None => (i..n).find(|&k| matches!(self.raw[k], '&' | '<')),
            Some(elem) if elem == "plaintext" => Some(n), // \Z
            Some(elem) => {
                let amp = self.escapable; // escapable and not convert_charrefs
                (i..n).find(|&k| (amp && self.raw[k] == '&') || (self.raw[k] == '<' && self.match_cdata_end(k, elem)))
            }
        }
    }

    fn goahead<S: Sink>(&mut self, end: bool, sink: &mut S) {
        let n = self.raw.len();
        let mut i = self.start;
        while i < n {
            let j = match self.interesting_search(i) {
                Some(j) => j,
                None => {
                    if self.cdata_elem.is_some() {
                        break;
                    }
                    n
                }
            };
            if i < j {
                sink.handle_data(collect(&self.raw[i..j]));
            }
            i = j;
            if i == n {
                break;
            }
            if self.at(i) == Some('<') {
                let k: isize;
                if matches!(self.at(i + 1), Some(c) if c.is_ascii_alphabetic()) {
                    k = self.parse_starttag(i, sink);
                } else if self.startswith(i, "</") {
                    k = self.parse_endtag(i, sink);
                } else if self.startswith(i, "<!--") {
                    k = self.parse_comment(i, sink);
                } else if self.startswith(i, "<?") {
                    k = self.parse_pi(i, sink);
                } else if self.startswith(i, "<!") {
                    k = self.parse_html_declaration(i, sink);
                } else if i + 1 < n || end {
                    sink.handle_data("<".to_string());
                    k = (i + 1) as isize;
                } else {
                    break;
                }
                let k = if k < 0 {
                    if !end {
                        break;
                    }
                    if matches!(self.at(i + 1), Some(c) if c.is_ascii_alphabetic()) {
                        // pass
                    } else if self.startswith(i, "</") {
                        if i + 2 == n {
                            sink.handle_data("</".to_string());
                        } else if matches!(self.at(i + 2), Some(c) if c.is_ascii_alphabetic()) {
                            // pass
                        } else {
                            sink.handle_comment(collect(&self.raw[i + 2..]));
                        }
                    } else if self.startswith(i, "<!--") {
                        let mut j = n;
                        let tail = &self.raw[i + 4..];
                        for suffix in ["--!", "--", "-"] {
                            let s: Vec<char> = suffix.chars().collect();
                            if tail.ends_with(&s) {
                                j -= s.len();
                                break;
                            }
                        }
                        sink.handle_comment(collect(&self.raw[i + 4..j]));
                    } else if self.startswith(i, "<![CDATA[") {
                        sink.unknown_decl(collect(&self.raw[i + 3..]));
                    } else if self.lower_slice_eq(i, 9, "<!doctype") {
                        sink.handle_decl(collect(&self.raw[i + 2..]));
                    } else if self.startswith(i, "<!") {
                        sink.handle_comment(collect(&self.raw[i + 2..]));
                    } else if self.startswith(i, "<?") {
                        sink.handle_pi(collect(&self.raw[i + 2..]));
                    } else {
                        unreachable!("we should not get here!");
                    }
                    n
                } else {
                    k as usize
                };
                i = k;
            } else if self.startswith(i, "&#") {
                if let Some((name_end, mend)) = self.match_charref(i) {
                    let name = collect(&self.raw[i + 2..name_end]);
                    sink.handle_charref(name);
                    let mut k = mend;
                    if self.raw[k - 1] != ';' {
                        k -= 1;
                    }
                    i = k;
                    continue;
                } else {
                    if self.raw[i..].contains(&';') {
                        sink.handle_data(collect(&self.raw[i..i + 2]));
                        i += 2;
                    }
                    break;
                }
            } else if self.at(i) == Some('&') {
                if let Some((name_end, mend)) = self.match_entityref(i) {
                    let name = collect(&self.raw[i + 1..name_end]);
                    sink.handle_entityref(name);
                    let mut k = mend;
                    if self.raw[k - 1] != ';' {
                        k -= 1;
                    }
                    i = k;
                    continue;
                }
                // incomplete: &[a-zA-Z#]
                if matches!(self.at(i + 1), Some(c) if c.is_ascii_alphabetic() || c == '#') {
                    if end && i + 2 == n {
                        i += 1;
                    }
                    break;
                } else if i + 1 < n {
                    sink.handle_data("&".to_string());
                    i += 1;
                } else {
                    break;
                }
            } else {
                unreachable!("interesting.search() lied");
            }
        }
        if end && i < n {
            sink.handle_data(collect(&self.raw[i..n]));
            i = n;
        }
        self.start = i;
    }

    fn lower_slice_eq(&self, i: usize, len: usize, target: &str) -> bool {
        let e = (i + len).min(self.raw.len());
        pystr::lower(&collect(&self.raw[i..e])) == target
    }

    /// charref: `&#(?:[0-9]+|[xX][0-9a-fA-F]+)[^0-9a-fA-F]` -> (end of name, end of match)
    fn match_charref(&self, i: usize) -> Option<(usize, usize)> {
        let n = self.raw.len();
        let mut k = i + 2;
        while k < n && self.raw[k].is_ascii_digit() {
            k += 1;
        }
        if k > i + 2 {
            // backtracking cannot help: the char after a shorter run is a digit
            if k < n && !self.raw[k].is_ascii_hexdigit() {
                return Some((k, k + 1));
            }
            return None;
        }
        if matches!(self.at(i + 2), Some('x' | 'X')) {
            let mut k = i + 3;
            while k < n && self.raw[k].is_ascii_hexdigit() {
                k += 1;
            }
            if k > i + 3 && k < n && !self.raw[k].is_ascii_hexdigit() {
                return Some((k, k + 1));
            }
        }
        None
    }

    /// entityref: `&([a-zA-Z][-.a-zA-Z0-9]*)[^a-zA-Z0-9]` -> (end of name, end of match)
    fn match_entityref(&self, i: usize) -> Option<(usize, usize)> {
        let n = self.raw.len();
        if !matches!(self.at(i + 1), Some(c) if c.is_ascii_alphabetic()) {
            return None;
        }
        let name_char = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.';
        let mut k = i + 2;
        while k < n && name_char(self.raw[k]) {
            k += 1;
        }
        if k < n {
            return Some((k, k + 1));
        }
        // backtrack: shorter name followed by '-' or '.'
        let mut l = k;
        while l > i + 2 {
            l -= 1;
            if matches!(self.raw[l], '-' | '.') {
                return Some((l, l + 1));
            }
        }
        None
    }

    /// The optional value group shared by attrfind_tolerant and locatetagend:
    /// `[\t\n\r\f ]*=[\t\n\r\f ]*('[^']*'|"[^"]*"|(?!['"])[^>\t\n\r\f ]*)`
    /// Returns (value_start, value_end) i.e. the span of the inner group;
    /// the whole group ends at value_end.
    fn match_value(&self, p: usize) -> Option<(usize, usize)> {
        let n = self.raw.len();
        let mut q = p;
        while q < n && is_ws5(self.raw[q]) {
            q += 1;
        }
        if self.at(q) != Some('=') {
            return None;
        }
        let r0 = q + 1;
        let mut r = r0;
        while r < n && is_ws5(self.raw[r]) {
            r += 1;
        }
        // try at r, then (backtracking) r-1, ..., r0
        let try_at = |r: usize| -> Option<usize> {
            match self.at(r) {
                Some(qc @ ('\'' | '"')) => self.find_char(qc, r + 1).map(|e| e + 1),
                _ => {
                    let mut e = r;
                    while e < n && !matches!(self.raw[e], '>' | '\t' | '\n' | '\r' | '\x0c' | ' ') {
                        e += 1;
                    }
                    Some(e)
                }
            }
        };
        let mut rr = r;
        loop {
            if let Some(e) = try_at(rr) {
                return Some((rr, e));
            }
            if rr == r0 {
                return None;
            }
            rr -= 1;
        }
    }

    /// attribute name with lookbehind: `(?<=['"\t\n\r\f /])[^\t\n\r\f />][^\t\n\r\f /=>]*`
    fn match_attr_name(&self, k: usize) -> Option<usize> {
        if k == 0 {
            return None;
        }
        let prev = self.raw[k - 1];
        if !(prev == '\'' || prev == '"' || is_ws5_slash(prev)) {
            return None;
        }
        match self.at(k) {
            Some(c) if !(is_ws5_slash(c) || c == '>') => {}
            _ => return None,
        }
        let n = self.raw.len();
        let mut e = k + 1;
        while e < n && !(is_ws5_slash(self.raw[e]) || self.raw[e] == '=' || self.raw[e] == '>') {
            e += 1;
        }
        Some(e)
    }

    /// `locatetagend.match(rawdata, p).end()`
    fn locatetagend(&self, p: usize) -> usize {
        let n = self.raw.len();
        // [a-zA-Z][^\t\n\r\f />]*
        debug_assert!(matches!(self.at(p), Some(c) if c.is_ascii_alphabetic()));
        let mut k = p + 1;
        while k < n && !(is_ws5_slash(self.raw[k]) || self.raw[k] == '>') {
            k += 1;
        }
        while k < n && is_ws5_slash(self.raw[k]) {
            k += 1;
        }
        loop {
            let Some(e) = self.match_attr_name(k) else { break };
            k = e;
            if let Some((_, ve)) = self.match_value(k) {
                k = ve;
            }
            while k < n && is_ws5_slash(self.raw[k]) {
                k += 1;
            }
        }
        if self.at(k) == Some('>') {
            k += 1;
        }
        k
    }

    /// trailing `(?:[\t\n\r\f ]|/(?!>))*`
    fn skip_ws_slash_not_gt(&self, mut k: usize) -> usize {
        let n = self.raw.len();
        while k < n {
            let c = self.raw[k];
            if is_ws5(c) || (c == '/' && self.at(k + 1) != Some('>')) {
                k += 1;
            } else {
                break;
            }
        }
        k
    }

    /// tagfind_tolerant: returns (name_end, match_end)
    fn tagfind(&self, p: usize) -> (usize, usize) {
        let n = self.raw.len();
        let mut k = p + 1;
        while k < n && !(is_ws5_slash(self.raw[k]) || self.raw[k] == '>') {
            k += 1;
        }
        (k, self.skip_ws_slash_not_gt(k))
    }

    fn parse_starttag<S: Sink>(&mut self, i: usize, sink: &mut S) -> isize {
        let endpos = self.locatetagend(i + 1);
        if self.raw[endpos - 1] != '>' {
            return -1;
        }
        let (name_end, mut k) = self.tagfind(i + 1);
        let tag = pystr::lower(&collect(&self.raw[i + 1..name_end]));
        let mut attrs: Vec<(String, Option<String>)> = Vec::new();
        while k < endpos {
            let Some(name_e) = self.match_attr_name(k) else { break };
            let attrname = collect(&self.raw[k..name_e]);
            let mut m_end = name_e;
            let attrvalue: Option<String> = match self.match_value(name_e) {
                None => None,
                Some((vs, ve)) => {
                    m_end = ve;
                    let v = &self.raw[vs..ve];
                    let v = if v.len() >= 2
                        && ((v[0] == '\'' && v[v.len() - 1] == '\'') || (v[0] == '"' && v[v.len() - 1] == '"'))
                    {
                        &v[1..v.len() - 1]
                    } else if v.len() == 1 && (v[0] == '\'' || v[0] == '"') {
                        // attrvalue[:1] == "'" == attrvalue[-1:] holds for a lone quote
                        &v[1..1]
                    } else {
                        v
                    };
                    let s = collect(v);
                    Some(if s.is_empty() { s } else { html_unescape(&s) })
                }
            };
            attrs.push((pystr::lower(&attrname), attrvalue));
            k = self.skip_ws_slash_not_gt(m_end);
        }
        let end_s = if k < endpos { collect(&self.raw[k..endpos]) } else { String::new() };
        let end_t = pystr::strip(&end_s);
        if end_t != ">" && end_t != "/>" {
            sink.handle_data(collect(&self.raw[i..endpos]));
            return endpos as isize;
        }
        if end_t.ends_with("/>") {
            sink.handle_startendtag(tag, attrs);
        } else {
            let t = tag.clone();
            sink.handle_starttag(tag, attrs);
            if CDATA_CONTENT_ELEMENTS.contains(&t.as_str()) || t == "plaintext" {
                self.set_cdata_mode(&t, false);
            } else if RCDATA_CONTENT_ELEMENTS.contains(&t.as_str()) {
                self.set_cdata_mode(&t, true);
            }
        }
        endpos as isize
    }

    fn parse_endtag<S: Sink>(&mut self, i: usize, sink: &mut S) -> isize {
        if self.find_char('>', i + 2).is_none() {
            return -1;
        }
        if !matches!(self.at(i + 2), Some(c) if c.is_ascii_alphabetic()) {
            if self.at(i + 2) == Some('>') {
                return (i + 3) as isize;
            } else {
                return self.parse_bogus_comment(i, sink);
            }
        }
        let j = self.locatetagend(i + 2);
        if self.raw[j - 1] != '>' {
            return -1;
        }
        let (name_end, _) = self.tagfind(i + 2);
        let tag = pystr::lower(&collect(&self.raw[i + 2..name_end]));
        sink.handle_endtag(tag);
        self.clear_cdata_mode();
        j as isize
    }

    fn parse_comment<S: Sink>(&mut self, i: usize, sink: &mut S) -> isize {
        let n = self.raw.len();
        // commentclose: --!?>
        let mut p = i + 4;
        while p + 2 < n {
            if self.raw[p] == '-' && self.at(p + 1) == Some('-') {
                if self.at(p + 2) == Some('>') {
                    sink.handle_comment(collect(&self.raw[i + 4..p]));
                    return (p + 3) as isize;
                }
                if self.at(p + 2) == Some('!') && self.at(p + 3) == Some('>') {
                    sink.handle_comment(collect(&self.raw[i + 4..p]));
                    return (p + 4) as isize;
                }
            }
            p += 1;
        }
        // commentabruptclose: -?> (match at i+4)
        if self.at(i + 4) == Some('>') {
            sink.handle_comment(String::new());
            return (i + 5) as isize;
        }
        if self.at(i + 4) == Some('-') && self.at(i + 5) == Some('>') {
            sink.handle_comment(String::new());
            return (i + 6) as isize;
        }
        -1
    }

    fn parse_bogus_comment<S: Sink>(&mut self, i: usize, sink: &mut S) -> isize {
        match self.find_char('>', i + 2) {
            None => -1,
            Some(pos) => {
                sink.handle_comment(collect(&self.raw[i + 2..pos]));
                (pos + 1) as isize
            }
        }
    }

    fn parse_pi<S: Sink>(&mut self, i: usize, sink: &mut S) -> isize {
        match self.find_char('>', i + 2) {
            None => -1,
            Some(j) => {
                sink.handle_pi(collect(&self.raw[i + 2..j]));
                (j + 1) as isize
            }
        }
    }

    fn parse_html_declaration<S: Sink>(&mut self, i: usize, sink: &mut S) -> isize {
        if self.startswith(i, "<!--") {
            self.parse_comment(i, sink)
        } else if self.startswith(i, "<![CDATA[") {
            match self.find_str("]]>", i + 9) {
                None => -1,
                Some(j) => {
                    sink.unknown_decl(collect(&self.raw[i + 3..j]));
                    (j + 3) as isize
                }
            }
        } else if self.lower_slice_eq(i, 9, "<!doctype") {
            match self.find_char('>', i + 9) {
                None => -1,
                Some(gt) => {
                    sink.handle_decl(collect(&self.raw[i + 2..gt]));
                    (gt + 1) as isize
                }
            }
        } else if self.startswith(i, "<![") {
            match self.find_char('>', i + 3) {
                None => -1,
                Some(j) => {
                    if self.raw[j - 1] == ']' {
                        sink.unknown_decl(collect(&self.raw[i + 3..j - 1]));
                    } else {
                        sink.handle_comment(collect(&self.raw[i + 2..j]));
                    }
                    (j + 1) as isize
                }
            }
        } else {
            self.parse_bogus_comment(i, sink)
        }
    }
}
