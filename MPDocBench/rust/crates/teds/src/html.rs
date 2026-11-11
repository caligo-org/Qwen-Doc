//! `lxml.html.fromstring(s, parser=HTMLParser(remove_comments=True, encoding='utf-8'))`
//! (lxml 5.3.0) followed by `xpath('body/table')[0]`, reproduced over libxml2's HTML parser.

use crate::pystr::is_py_space;
use crate::Error;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::sync::Once;

mod ffi {
    use std::ffi::{c_char, c_int, c_void};
    extern "C" {
        pub fn teds_init();
        pub fn teds_libxml_version() -> c_int;
        pub fn teds_parse(buf: *const c_char, len: c_int, encoding: *const c_char) -> *mut c_void;
        pub fn teds_free_doc(doc: *mut c_void);
        pub fn teds_doc_root(doc: *mut c_void) -> *mut c_void;
        pub fn teds_node_type(n: *mut c_void) -> c_int;
        pub fn teds_node_children(n: *mut c_void) -> *mut c_void;
        pub fn teds_node_next(n: *mut c_void) -> *mut c_void;
        pub fn teds_node_name(n: *mut c_void) -> *const c_char;
        pub fn teds_node_content(n: *mut c_void) -> *const c_char;
        pub fn teds_node_has_ns(n: *mut c_void) -> c_int;
        pub fn teds_node_ns_prefix(n: *mut c_void) -> *const c_char;
        pub fn teds_get_prop(n: *mut c_void, name: *const c_char) -> *mut c_char;
        pub fn teds_free(p: *mut c_void);
    }
}

const ELEMENT_NODE: c_int = 1;
const ENTITY_REF_NODE: c_int = 5;
const TEXT_NODE: c_int = 3;
const CDATA_SECTION_NODE: c_int = 4;
const PI_NODE: c_int = 7;
const COMMENT_NODE: c_int = 8;
const XINCLUDE_START: c_int = 19;
const XINCLUDE_END: c_int = 20;

static INIT: Once = Once::new();

/// Version of the libxml2 linked at run time, e.g. 20913.
pub fn libxml_version() -> i32 {
    INIT.call_once(|| unsafe { ffi::teds_init() });
    unsafe { ffi::teds_libxml_version() }
}

/// What lxml's `.tag` is for the nodes `getchildren()` returns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Element(String),
    /// `etree.ProcessingInstruction` (the factory function is the tag).
    Pi,
    /// `etree.Comment` (not produced: comments are removed while parsing).
    Comment,
    /// `etree.Entity`.
    Entity,
}

/// An lxml element as the TEDS code sees it.
#[derive(Debug)]
pub struct Elem {
    pub tag: Tag,
    /// `.text`
    pub text: Option<String>,
    /// `.tail`
    pub tail: Option<String>,
    /// `getchildren()`
    pub children: Vec<usize>,
    /// `attrib.get('colspan')` / `attrib.get('rowspan')` (elements only)
    pub colspan: Option<String>,
    pub rowspan: Option<String>,
}

/// The table `xpath('body/table')[0]` selects: `elems[0]` is the table itself.
#[derive(Debug)]
pub struct Table {
    pub elems: Vec<Elem>,
    /// `len(table.xpath('.//*'))`
    pub descendant_elements: usize,
}

struct Doc(*mut c_void);

impl Drop for Doc {
    fn drop(&mut self) {
        unsafe { ffi::teds_free_doc(self.0) }
    }
}

type Node = *mut c_void;

unsafe fn node_type(n: Node) -> c_int {
    ffi::teds_node_type(n)
}

unsafe fn cstr(p: *const c_char) -> String {
    if p.is_null() {
        return String::new();
    }
    String::from_utf8_lossy(CStr::from_ptr(p).to_bytes()).into_owned()
}

/// The element name as libxml2 >= 2.11 stores it: older versions split `o:p` into a
/// namespace prefix and `p` even in HTML, so the prefix is put back.
unsafe fn elem_name(n: Node) -> String {
    let name = crate::compat::restore_name(&cstr(ffi::teds_node_name(n)));
    if ffi::teds_node_has_ns(n) != 0 {
        let prefix = ffi::teds_node_ns_prefix(n);
        if !prefix.is_null() {
            return format!("{}:{name}", cstr(prefix));
        }
    }
    name
}

unsafe fn is_named_element(n: Node, name: &str) -> bool {
    node_type(n) == ELEMENT_NODE && elem_name(n) == name
}

/// lxml `_textNodeOrSkip`
unsafe fn text_node_or_skip(mut n: Node) -> Node {
    while !n.is_null() {
        match node_type(n) {
            TEXT_NODE | CDATA_SECTION_NODE => return n,
            XINCLUDE_START | XINCLUDE_END => n = ffi::teds_node_next(n),
            _ => return std::ptr::null_mut(),
        }
    }
    n
}

/// lxml `_collectText`: the text of consecutive text nodes from `n`, `None` if there are none.
unsafe fn collect_text(n: Node) -> Option<String> {
    let mut cur = text_node_or_skip(n);
    if cur.is_null() {
        return None;
    }
    let mut bytes = Vec::new();
    while !cur.is_null() {
        let c = ffi::teds_node_content(cur);
        if !c.is_null() {
            bytes.extend_from_slice(CStr::from_ptr(c).to_bytes());
        }
        cur = text_node_or_skip(ffi::teds_node_next(cur));
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// lxml `_isElement`
unsafe fn is_lxml_element(n: Node) -> bool {
    matches!(node_type(n), ELEMENT_NODE | COMMENT_NODE | ENTITY_REF_NODE | PI_NODE)
}

unsafe fn get_prop(n: Node, name: &CStr) -> Option<String> {
    let p = ffi::teds_get_prop(n, name.as_ptr());
    if p.is_null() {
        return None;
    }
    let s = cstr(p);
    ffi::teds_free(p as *mut c_void);
    Some(s)
}

unsafe fn count_descendant_elements(n: Node) -> usize {
    let mut count = 0;
    let mut stack = vec![ffi::teds_node_children(n)];
    while let Some(c) = stack.pop() {
        if c.is_null() {
            continue;
        }
        stack.push(ffi::teds_node_next(c));
        if node_type(c) == ELEMENT_NODE {
            count += 1;
            stack.push(ffi::teds_node_children(c));
        }
    }
    count
}

unsafe fn build(table: Node) -> Table {
    let mut elems: Vec<Elem> = Vec::new();
    let mut stack: Vec<(Node, usize)> = vec![(table, usize::MAX)];
    while let Some((n, parent)) = stack.pop() {
        let idx = elems.len();
        let ty = node_type(n);
        let (tag, colspan, rowspan, text) = match ty {
            ELEMENT_NODE => (
                Tag::Element(elem_name(n)),
                get_prop(n, c"colspan"),
                get_prop(n, c"rowspan"),
                collect_text(ffi::teds_node_children(n)),
            ),
            // _ProcessingInstruction / __ContentOnlyElement .text
            PI_NODE => (Tag::Pi, None, None, Some(cstr(ffi::teds_node_content(n)))),
            COMMENT_NODE => (Tag::Comment, None, None, Some(cstr(ffi::teds_node_content(n)))),
            _ => (Tag::Entity, None, None, None),
        };
        elems.push(Elem {
            tag,
            text,
            tail: collect_text(ffi::teds_node_next(n)),
            children: Vec::new(),
            colspan,
            rowspan,
        });
        if parent != usize::MAX {
            elems[parent].children.push(idx);
        }
        if ty == ELEMENT_NODE {
            let mut kids = Vec::new();
            let mut c = ffi::teds_node_children(n);
            while !c.is_null() {
                if is_lxml_element(c) {
                    kids.push(c);
                }
                c = ffi::teds_node_next(c);
            }
            for k in kids.into_iter().rev() {
                stack.push((k, idx));
            }
        }
    }
    Table { descendant_elements: count_descendant_elements(table), elems }
}

/// lxml `__HAS_XML_ENCODING`:
/// `^(<\?xml[^>]+)\s+encoding\s*=\s*["\'][^"\']*["\'](\s*\?>|)`
fn has_encoding_declaration(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("<?xml") else {
        return false;
    };
    let chars: Vec<char> = rest.chars().collect();
    let tail_matches = |mut i: usize| -> bool {
        let start = i;
        while i < chars.len() && is_py_space(chars[i]) {
            i += 1;
        }
        if i == start {
            return false;
        }
        for c in "encoding".chars() {
            if chars.get(i) != Some(&c) {
                return false;
            }
            i += 1;
        }
        while i < chars.len() && is_py_space(chars[i]) {
            i += 1;
        }
        if chars.get(i) != Some(&'=') {
            return false;
        }
        i += 1;
        while i < chars.len() && is_py_space(chars[i]) {
            i += 1;
        }
        if !matches!(chars.get(i), Some('"') | Some('\'')) {
            return false;
        }
        i += 1;
        while i < chars.len() && chars[i] != '"' && chars[i] != '\'' {
            i += 1;
        }
        matches!(chars.get(i), Some('"') | Some('\''))
    };
    // `[^>]+` takes at least one character; the rest must start right after it.
    let mut k = 0;
    while k < chars.len() && chars[k] != '>' {
        k += 1;
        if tail_matches(k) {
            return true;
        }
    }
    false
}

/// lxml.html `_looks_like_full_html_unicode`: `^\s*<(?:html|!doctype)` with `re.I`.
fn looks_like_full_html(s: &str) -> bool {
    let t = s.trim_start_matches(is_py_space);
    let Some(t) = t.strip_prefix('<') else {
        return false;
    };
    let starts_ci = |p: &str| {
        t.len() >= p.len() && t.as_bytes()[..p.len()].eq_ignore_ascii_case(p.as_bytes())
    };
    starts_ci("html") || starts_ci("!doctype")
}

/// How lxml hands a Python `str` to libxml2 (`_parseUnicodeDoc`): the PEP 393 buffer as is,
/// with an encoding named after the string's kind.
fn python_buffer(s: &str) -> (Vec<u8>, &'static CStr) {
    let max = s.chars().map(|c| c as u32).max().unwrap_or(0);
    if max <= 127 {
        (s.as_bytes().to_vec(), c"UTF-8")
    } else if max <= 0xff {
        (s.chars().map(|c| c as u8).collect(), c"ISO-8859-1")
    } else if max <= 0xffff {
        let mut v = Vec::with_capacity(s.len() * 2);
        for c in s.chars() {
            v.extend_from_slice(&(c as u16).to_le_bytes());
        }
        (v, c"UTF-16LE")
    } else {
        let mut v = Vec::with_capacity(s.len() * 4);
        for c in s.chars() {
            v.extend_from_slice(&(c as u32).to_le_bytes());
        }
        (v, c"UTF-32LE")
    }
}

/// Parses `s` like `lxml.html.fromstring` and returns `xpath('body/table')[0]` of the result,
/// or `None` when that selects nothing. `Err` where lxml raises.
pub fn parse_table(s: &str) -> Result<Option<Table>, Error> {
    INIT.call_once(|| unsafe { ffi::teds_init() });
    if has_encoding_declaration(s) {
        return Err(Error::Parse("Unicode strings with encoding declaration are not supported"));
    }
    let emulated;
    let input = if libxml_version() < 21200 {
        emulated = crate::compat::emulate_libxml2_2_12(s);
        emulated.as_str()
    } else {
        s
    };
    let (buf, encoding) = python_buffer(input);
    let len: c_int = buf.len().try_into().map_err(|_| Error::Parse("input too large"))?;
    unsafe {
        let doc = ffi::teds_parse(buf.as_ptr() as *const c_char, len, encoding.as_ptr());
        if doc.is_null() {
            return Err(Error::Parse("libxml2 returned no document"));
        }
        let doc = Doc(doc);
        let root = ffi::teds_doc_root(doc.0);
        if root.is_null() {
            return Err(Error::Parse("Document is empty"));
        }
        // lxml.html.fromstring returns the document's root element only for full documents
        // or when a <head> was produced; otherwise it returns a body child or the body (renamed),
        // on which xpath('body/table') selects nothing (libxml2 never nests a body).
        let mut has_body = false;
        let mut has_head = false;
        let mut c = ffi::teds_node_children(root);
        while !c.is_null() {
            has_body |= is_named_element(c, "body");
            has_head |= is_named_element(c, "head");
            c = ffi::teds_node_next(c);
        }
        if !(looks_like_full_html(s) || has_head || !has_body) {
            return Ok(None);
        }
        let mut c = ffi::teds_node_children(root);
        while !c.is_null() {
            if is_named_element(c, "body") {
                let mut t = ffi::teds_node_children(c);
                while !t.is_null() {
                    if is_named_element(t, "table") {
                        return Ok(Some(build(t)));
                    }
                    t = ffi::teds_node_next(t);
                }
            }
            c = ffi::teds_node_next(c);
        }
        Ok(None)
    }
}
