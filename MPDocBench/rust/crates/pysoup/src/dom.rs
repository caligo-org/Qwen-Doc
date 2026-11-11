//! Arena port of the parts of BeautifulSoup 4.11.1 (`bs4/__init__.py`,
//! `bs4/element.py`, `bs4/formatter.py`, `bs4/builder/_htmlparser.py`)
//! reached by the MPDocBench table code, for the `html.parser` builder.
//!
//! All five element links (`parent`, `previous_element`, `next_element`,
//! `previous_sibling`, `next_sibling`) are maintained exactly as bs4 does,
//! because `find_all` walks the `next_element` chain, which bs4's tree
//! surgery can leave inconsistent with `contents`.

use crate::parser::{HtmlParser, Sink};
use crate::pystr;
use crate::unescape::bs4_entity;
use crate::Error;

pub type Id = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrType {
    Navigable,
    Comment,
    CData,
    ProcessingInstruction,
    Declaration,
    Doctype,
    Stylesheet,
    Script,
    TemplateString,
    RubyText,
    RubyParenthesis,
}

impl StrType {
    fn preformatted(self) -> Option<(&'static str, &'static str)> {
        match self {
            StrType::Comment => Some(("<!--", "-->")),
            StrType::CData => Some(("<![CDATA[", "]]>")),
            StrType::ProcessingInstruction => Some(("<?", ">")),
            StrType::Declaration => Some(("<?", "?>")),
            StrType::Doctype => Some(("<!DOCTYPE ", ">\n")),
            _ => None,
        }
    }
}

/// `builder.string_containers` (HTMLTreeBuilder.DEFAULT_STRING_CONTAINERS)
fn string_container_for(name: &str) -> Option<StrType> {
    match name {
        "rt" => Some(StrType::RubyText),
        "rp" => Some(StrType::RubyParenthesis),
        "style" => Some(StrType::Stylesheet),
        "script" => Some(StrType::Script),
        "template" => Some(StrType::TemplateString),
        _ => None,
    }
}

fn is_void(name: &str) -> bool {
    matches!(
        name,
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "keygen" | "link" | "menuitem"
            | "meta" | "param" | "source" | "track" | "wbr" | "basefont" | "bgsound" | "command" | "frame"
            | "image" | "isindex" | "nextid" | "spacer"
    )
}

fn is_cdata_list_attr(tag: &str, attr: &str) -> bool {
    if matches!(attr, "class" | "accesskey" | "dropzone") {
        return true;
    }
    let specific: &[&str] = match tag {
        "a" | "link" => &["rel", "rev"],
        "td" | "th" => &["headers"],
        "form" => &["accept-charset"],
        "object" => &["archive"],
        "area" => &["rel"],
        "icon" => &["sizes"],
        "iframe" => &["sandbox"],
        "output" => &["for"],
        _ => &[],
    };
    specific.contains(&attr)
}

#[derive(Clone, Debug)]
pub enum AttrVal {
    Str(String),
    List(Vec<String>),
    /// CharsetMetaAttributeValue(original)
    CharsetMeta(String),
    /// ContentMetaAttributeValue(original)
    ContentMeta(String),
}

impl AttrVal {
    /// The value as Python code would see it through `tag.get()` when it is
    /// a `str`; lists are returned joined only for diagnostic purposes.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttrVal::Str(s) | AttrVal::CharsetMeta(s) | AttrVal::ContentMeta(s) => Some(s),
            AttrVal::List(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Kind {
    Tag,
    Str(StrType),
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: Kind,
    pub name: String,
    pub attrs: Vec<(String, AttrVal)>,
    pub text: String,
    pub parent: Option<Id>,
    pub prev_el: Option<Id>,
    pub next_el: Option<Id>,
    pub prev_sib: Option<Id>,
    pub next_sib: Option<Id>,
    pub contents: Vec<Id>,
    pub can_be_empty: bool,
    pub hidden: bool,
    /// interesting_string_types fixed at construction time
    pub interesting: Option<StrType>,
}

impl Node {
    fn new_tag(name: String) -> Node {
        Node {
            kind: Kind::Tag,
            can_be_empty: is_void(&name),
            interesting: string_container_for(&name),
            name,
            attrs: Vec::new(),
            text: String::new(),
            parent: None,
            prev_el: None,
            next_el: None,
            prev_sib: None,
            next_sib: None,
            contents: Vec::new(),
            hidden: false,
        }
    }
    fn new_str(t: StrType, text: String) -> Node {
        Node {
            kind: Kind::Str(t),
            name: String::new(),
            attrs: Vec::new(),
            text,
            parent: None,
            prev_el: None,
            next_el: None,
            prev_sib: None,
            next_sib: None,
            contents: Vec::new(),
            can_be_empty: false,
            hidden: false,
            interesting: None,
        }
    }
    pub fn is_tag(&self) -> bool {
        matches!(self.kind, Kind::Tag)
    }
}

const ROOT_TAG_NAME: &str = "[document]";

/// Deepest Python frame that does not raise RecursionError under the default
/// recursion limit of 1000 (calibrated), see `Soup::decode`.
pub const PY_MAX_FRAME_DEPTH: usize = 998;

pub struct Soup {
    pub nodes: Vec<Node>,
    // tree-building state
    current_data: Vec<String>,
    tag_stack: Vec<Id>,
    open_tag_counter: std::collections::HashMap<String, i64>,
    preserve_ws_stack: Vec<Id>,
    string_container_stack: Vec<Id>,
    most_recent: Option<Id>,
    already_closed_empty_element: Vec<String>,
    /// Python frames already on the stack when `str(soup)` is called; used to
    /// emulate `RecursionError` in the recursive `decode`.
    pub base_frames: usize,
}

impl Soup {
    /// An empty arena; documents are added with [`Soup::parse_doc`].
    pub fn new() -> Soup {
        Soup {
            nodes: Vec::new(),
            current_data: Vec::new(),
            tag_stack: Vec::new(),
            open_tag_counter: Default::default(),
            preserve_ws_stack: Vec::new(),
            string_container_stack: Vec::new(),
            most_recent: None,
            already_closed_empty_element: Vec::new(),
            base_frames: 0,
        }
    }

    /// `BeautifulSoup(markup, 'html.parser')` in a fresh arena; returns it with the root id.
    pub fn parse(markup: &str) -> (Soup, Id) {
        let mut s = Soup::new();
        let r = s.parse_doc(markup);
        (s, r)
    }

    /// `BeautifulSoup(markup, 'html.parser')`, building the document in this
    /// arena (so that nodes can later be moved between documents). Returns the
    /// id of the `BeautifulSoup` object (the hidden `[document]` root).
    pub fn parse_doc(&mut self, markup: &str) -> Id {
        let mut root = Node::new_tag(ROOT_TAG_NAME.to_string());
        root.hidden = true;
        root.can_be_empty = false;
        root.interesting = None;
        let root = self.new_node(root);
        self.current_data.clear();
        self.tag_stack = vec![root];
        self.open_tag_counter.clear();
        self.preserve_ws_stack.clear();
        self.string_container_stack.clear();
        self.most_recent = None;
        self.already_closed_empty_element.clear();
        HtmlParser::run(markup, self);
        self.already_closed_empty_element.clear();
        self.end_data(None);
        while self.nodes[self.current_tag()].name != ROOT_TAG_NAME {
            self.pop_tag();
        }
        self.tag_stack.clear();
        root
    }

    fn current_tag(&self) -> Id {
        *self.tag_stack.last().unwrap()
    }

    // ---------------------------------------------------------------- building

    fn pop_tag(&mut self) -> Id {
        let tag = self.tag_stack.pop().unwrap();
        let name = self.nodes[tag].name.clone();
        if let Some(c) = self.open_tag_counter.get_mut(&name) {
            *c -= 1;
        }
        if self.preserve_ws_stack.last() == Some(&tag) {
            self.preserve_ws_stack.pop();
        }
        if self.string_container_stack.last() == Some(&tag) {
            self.string_container_stack.pop();
        }
        self.current_tag_or(tag)
    }

    fn current_tag_or(&self, popped: Id) -> Id {
        self.tag_stack.last().copied().unwrap_or(popped)
    }

    fn push_tag(&mut self, tag: Id) {
        let cur = self.current_tag();
        self.nodes[cur].contents.push(tag);
        self.tag_stack.push(tag);
        let name = self.nodes[tag].name.clone();
        if name != ROOT_TAG_NAME {
            *self.open_tag_counter.entry(name.clone()).or_insert(0) += 1;
        }
        if name == "pre" || name == "textarea" {
            self.preserve_ws_stack.push(tag);
        }
        if string_container_for(&name).is_some() {
            self.string_container_stack.push(tag);
        }
    }

    fn string_container(&self, base: Option<StrType>) -> StrType {
        let container = base.unwrap_or(StrType::Navigable);
        if container == StrType::Navigable {
            if let Some(&top) = self.string_container_stack.last() {
                if let Some(t) = string_container_for(&self.nodes[top].name) {
                    return t;
                }
            }
        }
        container
    }

    fn end_data(&mut self, container: Option<StrType>) {
        if self.current_data.is_empty() {
            return;
        }
        let mut data: String = self.current_data.concat();
        if self.preserve_ws_stack.is_empty() {
            let strippable = data.chars().all(|c| matches!(c, ' ' | '\n' | '\t' | '\x0c' | '\r'));
            if strippable {
                data = if data.contains('\n') { "\n".to_string() } else { " ".to_string() };
            }
        }
        self.current_data.clear();
        let t = self.string_container(container);
        let o = self.new_node(Node::new_str(t, data));
        self.object_was_parsed(o);
    }

    fn new_node(&mut self, n: Node) -> Id {
        self.nodes.push(n);
        self.nodes.len() - 1
    }

    /// PageElement.setup(parent, previous_element) with next_* None
    fn setup(&mut self, o: Id, parent: Option<Id>, previous_element: Option<Id>) {
        self.nodes[o].parent = parent;
        self.nodes[o].prev_el = previous_element;
        if let Some(p) = previous_element {
            self.nodes[p].next_el = Some(o);
        }
        self.nodes[o].next_el = None;
        self.nodes[o].next_sib = None;
        let mut previous_sibling = None;
        if let Some(p) = parent {
            if let Some(&last) = self.nodes[p].contents.last() {
                previous_sibling = Some(last);
            }
        }
        self.nodes[o].prev_sib = previous_sibling;
        if let Some(ps) = previous_sibling {
            self.nodes[ps].next_sib = Some(o);
        }
    }

    fn object_was_parsed(&mut self, o: Id) {
        let parent = self.current_tag();
        let previous_element = self.most_recent;
        let fix = self.nodes[parent].next_el.is_some();
        self.setup(o, Some(parent), previous_element);
        self.most_recent = Some(o);
        self.nodes[parent].contents.push(o);
        if fix {
            self.linkage_fixer(parent);
        }
    }

    fn linkage_fixer(&mut self, el: Id) {
        let first = self.nodes[el].contents[0];
        let child = *self.nodes[el].contents.last().unwrap();
        let mut descendant = child;
        if child == first && self.nodes[el].parent.is_some() {
            self.nodes[el].next_el = Some(child);
            let prev_el = self.nodes[child].prev_el;
            if let Some(pe) = prev_el {
                if pe != el {
                    self.nodes[pe].next_el = None;
                }
            }
            self.nodes[child].prev_el = Some(el);
            self.nodes[child].prev_sib = None;
        }
        self.nodes[child].next_sib = None;
        if self.nodes[child].is_tag() && !self.nodes[child].contents.is_empty() {
            descendant = self.last_descendant_uninit(child);
        }
        self.nodes[descendant].next_el = None;
        self.nodes[descendant].next_sib = None;
        let mut target = Some(el);
        while let Some(t) = target {
            if let Some(ns) = self.nodes[t].next_sib {
                self.nodes[descendant].next_el = Some(ns);
                self.nodes[ns].prev_el = Some(child);
                break;
            }
            target = self.nodes[t].parent;
        }
    }

    fn sink_starttag(&mut self, name: String, attrs: Vec<(String, Option<String>)>, handle_empty_element: bool) {
        // BeautifulSoupHTMLParser.handle_starttag: build the attribute dict
        let mut attr_dict: Vec<(String, String)> = Vec::new();
        for (k, v) in attrs {
            let v = v.unwrap_or_default();
            if let Some(slot) = attr_dict.iter_mut().find(|(ek, _)| *ek == k) {
                slot.1 = v;
            } else {
                attr_dict.push((k, v));
            }
        }
        // BeautifulSoup.handle_starttag
        self.end_data(None);
        let mut node = Node::new_tag(name.clone());
        // Tag.__init__: cdata list attributes
        let lname = pystr::lower(&name);
        node.attrs = attr_dict
            .into_iter()
            .map(|(k, v)| {
                if is_cdata_list_attr(&lname, &k) {
                    let vals: Vec<String> =
                        v.split(|c: char| pystr::is_space(c)).filter(|s| !s.is_empty()).map(String::from).collect();
                    (k, AttrVal::List(vals))
                } else {
                    (k, AttrVal::Str(v))
                }
            })
            .collect();
        // HTMLTreeBuilder.set_up_substitutions
        if name == "meta" {
            let get = |n: &Node, key: &str| -> Option<AttrVal> {
                n.attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
            };
            let http_equiv = get(&node, "http-equiv");
            let content = get(&node, "content");
            let charset = get(&node, "charset");
            if let Some(cs) = charset {
                if let AttrVal::Str(s) = cs {
                    let slot = node.attrs.iter_mut().find(|(k, _)| k == "charset").unwrap();
                    slot.1 = AttrVal::CharsetMeta(s);
                }
            } else if let (Some(AttrVal::Str(c)), Some(AttrVal::Str(he))) = (content, http_equiv) {
                if pystr::lower(&he) == "content-type" && charset_re_search(&c) {
                    let slot = node.attrs.iter_mut().find(|(k, _)| k == "content").unwrap();
                    slot.1 = AttrVal::ContentMeta(c);
                }
            }
        }
        let parent = self.current_tag();
        let previous = self.most_recent;
        let tag = self.new_node(node);
        self.setup(tag, Some(parent), previous);
        if let Some(m) = self.most_recent {
            self.nodes[m].next_el = Some(tag);
        }
        self.most_recent = Some(tag);
        self.push_tag(tag);
        // back in BeautifulSoupHTMLParser.handle_starttag
        if self.nodes[tag].contents.is_empty() && self.nodes[tag].can_be_empty && handle_empty_element {
            self.sink_endtag(name.clone(), false);
            self.already_closed_empty_element.push(name);
        }
    }

    fn sink_endtag(&mut self, name: String, check_already_closed: bool) {
        if check_already_closed {
            if let Some(pos) = self.already_closed_empty_element.iter().position(|n| *n == name) {
                self.already_closed_empty_element.remove(pos);
                return;
            }
        }
        self.end_data(None);
        self.pop_to_tag(&name);
    }

    fn pop_to_tag(&mut self, name: &str) {
        if name == ROOT_TAG_NAME {
            return;
        }
        let stack_size = self.tag_stack.len();
        let mut i = stack_size - 1;
        while i > 0 {
            if self.open_tag_counter.get(name).copied().unwrap_or(0) == 0 {
                break;
            }
            let t = self.tag_stack[i];
            if self.nodes[t].name == name {
                self.pop_tag();
                break;
            }
            self.pop_tag();
            i -= 1;
        }
    }

    fn emit_special(&mut self, data: String, t: StrType) {
        self.end_data(None);
        self.current_data.push(data);
        self.end_data(Some(t));
    }

    // ---------------------------------------------------------------- tree API

    pub fn is_tag(&self, id: Id) -> bool {
        self.nodes[id].is_tag()
    }

    pub fn name(&self, id: Id) -> &str {
        &self.nodes[id].name
    }

    pub fn set_name(&mut self, id: Id, name: &str) {
        self.nodes[id].name = name.to_string();
    }

    pub fn get_attr(&self, id: Id, key: &str) -> Option<&AttrVal> {
        self.nodes[id].attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// `_last_descendant(is_initialized=True, accept_self=True)`
    fn last_descendant(&self, id: Id) -> Result<Option<Id>, Error> {
        if let Some(ns) = self.nodes[id].next_sib {
            return Ok(self.nodes[ns].prev_el);
        }
        Ok(Some(self.last_descendant_uninit(id)))
    }

    /// `_last_descendant(is_initialized=False)` (accept_self=True)
    fn last_descendant_uninit(&self, id: Id) -> Id {
        let mut last = id;
        while self.nodes[last].is_tag() && !self.nodes[last].contents.is_empty() {
            last = *self.nodes[last].contents.last().unwrap();
        }
        last
    }

    /// `tag.descendants` (follows the next_element chain)
    pub fn descendants(&self, id: Id) -> Result<Vec<Id>, Error> {
        let mut out = Vec::new();
        if self.nodes[id].contents.is_empty() {
            return Ok(out);
        }
        let stop = match self.last_descendant(id)? {
            Some(l) => self.nodes[l].next_el,
            None => return Err(Error::Python("AttributeError: 'NoneType' object has no attribute 'next_element'".into())),
        };
        let mut current = Some(self.nodes[id].contents[0]);
        let limit = self.nodes.len() * 4 + 16;
        while current != stop {
            match current {
                None => {
                    return Err(Error::Python(
                        "AttributeError: 'NoneType' object has no attribute 'next_element'".into(),
                    ))
                }
                Some(c) => {
                    out.push(c);
                    if out.len() > limit {
                        return Err(Error::Python("descendants: next_element cycle (Python would not terminate)".into()));
                    }
                    current = self.nodes[c].next_el;
                }
            }
        }
        Ok(out)
    }

    /// `tag.find_all(names)` for a tag-name filter (string or list of strings).
    pub fn find_all(&self, id: Id, names: &[&str]) -> Result<Vec<Id>, Error> {
        Ok(self
            .descendants(id)?
            .into_iter()
            .filter(|&d| self.nodes[d].is_tag() && names.contains(&self.nodes[d].name.as_str()))
            .collect())
    }

    /// `parent.index(element)`
    fn index(&self, parent: Id, el: Id) -> Result<usize, Error> {
        self.nodes[parent]
            .contents
            .iter()
            .position(|&c| c == el)
            .ok_or_else(|| Error::Python("ValueError: Tag.index: element not in tag".into()))
    }

    /// `PageElement.extract(_self_index)`
    pub fn extract(&mut self, id: Id, self_index: Option<usize>) -> Result<(), Error> {
        if let Some(p) = self.nodes[id].parent {
            let idx = match self_index {
                Some(i) => i,
                None => self.index(p, id)?,
            };
            self.nodes[p].contents.remove(idx);
        }
        let last_child = match self.last_descendant(id)? {
            Some(l) => l,
            None => {
                return Err(Error::Python("AttributeError: 'NoneType' object has no attribute 'next_element'".into()))
            }
        };
        let next_element = self.nodes[last_child].next_el;
        let prev_el = self.nodes[id].prev_el;
        if let Some(pe) = prev_el {
            if Some(pe) != next_element {
                self.nodes[pe].next_el = next_element;
            }
        }
        if let Some(ne) = next_element {
            if Some(ne) != prev_el {
                self.nodes[ne].prev_el = prev_el;
            }
        }
        self.nodes[id].prev_el = None;
        self.nodes[last_child].next_el = None;
        self.nodes[id].parent = None;
        let ps = self.nodes[id].prev_sib;
        let ns = self.nodes[id].next_sib;
        if let Some(p) = ps {
            if Some(p) != ns {
                self.nodes[p].next_sib = ns;
            }
        }
        if let Some(n) = ns {
            if Some(n) != ps {
                self.nodes[n].prev_sib = ps;
            }
        }
        self.nodes[id].prev_sib = None;
        self.nodes[id].next_sib = None;
        Ok(())
    }

    /// `PageElement.insert(position, new_child)`
    pub fn insert(&mut self, this: Id, position: usize, new_child: Id) -> Result<(), Error> {
        if new_child == this {
            return Err(Error::Python("ValueError: Cannot insert a tag into itself.".into()));
        }
        let mut position = position.min(self.nodes[this].contents.len());
        if let Some(p) = self.nodes[new_child].parent {
            if p == this {
                let current_index = self.index(this, new_child)?;
                if current_index < position {
                    position -= 1;
                }
            }
            self.extract(new_child, None)?;
        }
        self.nodes[new_child].parent = Some(this);
        if position == 0 {
            self.nodes[new_child].prev_sib = None;
            self.nodes[new_child].prev_el = Some(this);
        } else {
            let previous_child = self.nodes[this].contents[position - 1];
            self.nodes[new_child].prev_sib = Some(previous_child);
            self.nodes[previous_child].next_sib = Some(new_child);
            // previous_child._last_descendant(False): is_initialized=False, accept_self=True
            self.nodes[new_child].prev_el = Some(self.last_descendant_uninit(previous_child));
        }
        if let Some(pe) = self.nodes[new_child].prev_el {
            self.nodes[pe].next_el = Some(new_child);
        }
        let new_last = self.last_descendant_uninit(new_child);
        if position >= self.nodes[this].contents.len() {
            self.nodes[new_child].next_sib = None;
            let mut parent = Some(this);
            let mut parents_next_sibling = None;
            while parents_next_sibling.is_none() {
                let Some(p) = parent else { break };
                parents_next_sibling = self.nodes[p].next_sib;
                parent = self.nodes[p].parent;
                if parents_next_sibling.is_some() {
                    break;
                }
            }
            self.nodes[new_last].next_el = parents_next_sibling;
        } else {
            let next_child = self.nodes[this].contents[position];
            self.nodes[new_child].next_sib = Some(next_child);
            self.nodes[next_child].prev_sib = Some(new_child);
            self.nodes[new_last].next_el = Some(next_child);
        }
        if let Some(ne) = self.nodes[new_last].next_el {
            self.nodes[ne].prev_el = Some(new_last);
        }
        self.nodes[this].contents.insert(position, new_child);
        Ok(())
    }

    pub fn append(&mut self, this: Id, child: Id) -> Result<(), Error> {
        let n = self.nodes[this].contents.len();
        self.insert(this, n, child)
    }

    /// `PageElement.unwrap()`
    pub fn unwrap(&mut self, id: Id) -> Result<(), Error> {
        let Some(my_parent) = self.nodes[id].parent else {
            return Err(Error::Python(
                "ValueError: Cannot replace an element with its contents when thatelement is not part of a tree."
                    .into(),
            ));
        };
        let my_index = self.index(my_parent, id)?;
        self.extract(id, Some(my_index))?;
        let children: Vec<Id> = self.nodes[id].contents.clone();
        for &child in children.iter().rev() {
            self.insert(my_parent, my_index, child)?;
        }
        Ok(())
    }

    /// `PageElement.replace_with(str)` with a plain Python `str`.
    pub fn replace_with_text(&mut self, id: Id, text: &str) -> Result<(), Error> {
        let Some(old_parent) = self.nodes[id].parent else {
            return Err(Error::Python(
                "ValueError: Cannot replace one element with another when the element to be replaced is not part of a tree."
                    .into(),
            ));
        };
        let my_index = self.index(old_parent, id)?;
        self.extract(id, Some(my_index))?;
        let s = self.new_node(Node::new_str(StrType::Navigable, text.to_string()));
        self.insert(old_parent, my_index, s)
    }

    /// `tag.string` (None, or the text of the single string reached)
    pub fn string(&self, id: Id) -> Option<&str> {
        let mut cur = id;
        loop {
            let n = &self.nodes[cur];
            if !n.is_tag() {
                return Some(&n.text);
            }
            if n.contents.len() != 1 {
                return None;
            }
            cur = n.contents[0];
        }
    }

    /// `tag.get_text()` (default separator, no strip, default types)
    pub fn get_text(&self, id: Id) -> Result<String, Error> {
        let types = self.nodes[id].interesting;
        let mut out = String::new();
        for d in self.descendants(id)? {
            if let Kind::Str(t) = self.nodes[d].kind {
                let ok = match types {
                    None => matches!(t, StrType::Navigable | StrType::CData),
                    Some(only) => t == only,
                };
                if ok {
                    out.push_str(&self.nodes[d].text);
                }
            }
        }
        Ok(out)
    }

    // ---------------------------------------------------------------- output

    /// `str(soup)` i.e. `decode()` with the "minimal" formatter.
    ///
    /// bs4 4.11.1 renders recursively (two Python frames per tag level), so very
    /// deep trees raise `RecursionError` in Python. This is emulated: with `D =
    /// base_frames` (the Python frame depth of the function that calls
    /// `str(soup)`), the tag at nesting level k has its `decode` frame at
    /// `D+3+2k`; the deepest frames below it were calibrated against CPython
    /// 3.11.18 (see `gen/calibrate_recursion.py`): +2 for any tag, +4/+6 for
    /// attributes without/with `&<>`, +6/+8 for a child string without/with an
    /// entity substitution. Python raises once a frame would exceed depth 998.
    pub fn decode(&self, id: Id) -> Result<String, Error> {
        let mut out = String::new();
        self.decode_into(id, &mut out, self.base_frames + 3)?;
        Ok(out)
    }

    fn decode_into(&self, id: Id, out: &mut String, depth: usize) -> Result<(), Error> {
        let check = |extra: usize| -> Result<(), Error> {
            if depth + extra > PY_MAX_FRAME_DEPTH {
                Err(Error::Python("RecursionError: maximum recursion depth exceeded".into()))
            } else {
                Ok(())
            }
        };
        let n = &self.nodes[id];
        check(2)?;
        let is_empty = n.contents.is_empty() && n.can_be_empty;
        if !n.hidden {
            out.push('<');
            out.push_str(&n.name);
            let mut attrs: Vec<&(String, AttrVal)> = n.attrs.iter().collect();
            attrs.sort_by(|a, b| a.0.cmp(&b.0));
            for (k, v) in attrs {
                let val: String = match v {
                    AttrVal::Str(s) => s.clone(),
                    AttrVal::List(l) => l.join(" "),
                    AttrVal::CharsetMeta(_) => "utf-8".to_string(),
                    AttrVal::ContentMeta(orig) => charset_re_sub(orig, "utf-8"),
                };
                check(if val.contains(['&', '<', '>']) { 6 } else { 4 })?;
                out.push(' ');
                out.push_str(k);
                out.push('=');
                quoted_attribute_value(&substitute_xml(&val), out);
            }
            if is_empty {
                out.push('/');
            }
            out.push('>');
        }
        for &c in &n.contents {
            let cn = &self.nodes[c];
            match cn.kind {
                Kind::Tag => self.decode_into(c, out, depth + 2)?,
                Kind::Str(t) => {
                    let raw = match cn.parent {
                        Some(p) => matches!(self.nodes[p].name.as_str(), "script" | "style"),
                        None => false,
                    };
                    let substitutes = !raw && cn.text.contains(['&', '<', '>']);
                    check(if substitutes { 8 } else { 6 })?;
                    if let Some((pre, suf)) = t.preformatted() {
                        out.push_str(pre);
                        out.push_str(&cn.text);
                        out.push_str(suf);
                    } else if raw {
                        out.push_str(&cn.text);
                    } else {
                        out.push_str(&substitute_xml(&cn.text));
                    }
                }
            }
        }
        if !n.hidden && !is_empty {
            out.push_str("</");
            out.push_str(&n.name);
            out.push('>');
        }
        Ok(())
    }
}

/// `EntitySubstitution.substitute_xml`
pub fn substitute_xml(s: &str) -> String {
    if !s.contains(['&', '<', '>']) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

/// `EntitySubstitution.quoted_attribute_value`
fn quoted_attribute_value(value: &str, out: &mut String) {
    if value.contains('"') {
        if value.contains('\'') {
            out.push('"');
            out.push_str(&value.replace('"', "&quot;"));
            out.push('"');
        } else {
            out.push('\'');
            out.push_str(value);
            out.push('\'');
        }
    } else {
        out.push('"');
        out.push_str(value);
        out.push('"');
    }
}

/// `ContentMetaAttributeValue.CHARSET_RE = re.compile(r"((^|;)\s*charset=)([^;]*)", re.M)`
/// Returns the matches as (start, end_of_group1, end).
fn charset_re_matches(s: &str) -> Vec<(usize, usize, usize)> {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let n = chars.len();
    let byte = |i: usize| if i < n { chars[i].0 } else { s.len() };
    let mut res = Vec::new();
    let mut p = 0;
    while p <= n {
        let try_from = |q: usize| -> Option<usize> {
            let mut q = q;
            while q < n && pystr::is_space(chars[q].1) {
                q += 1;
            }
            for e in "charset=".chars() {
                if q < n && chars[q].1 == e {
                    q += 1;
                } else {
                    return None;
                }
            }
            Some(q)
        };
        let at_line_start = p == 0 || (p <= n && chars[p - 1].1 == '\n');
        let mut m = None;
        if at_line_start {
            m = try_from(p);
        }
        if m.is_none() && p < n && chars[p].1 == ';' {
            m = try_from(p + 1);
        }
        if let Some(g1) = m {
            let mut e = g1;
            while e < n && chars[e].1 != ';' {
                e += 1;
            }
            res.push((byte(p), byte(g1), byte(e)));
            p = if e > p { e } else { p + 1 };
        } else {
            p += 1;
        }
    }
    res
}

fn charset_re_search(s: &str) -> bool {
    !charset_re_matches(s).is_empty()
}

fn charset_re_sub(s: &str, enc: &str) -> String {
    let mut out = String::new();
    let mut last = 0;
    for (a, g1, e) in charset_re_matches(s) {
        out.push_str(&s[last..a]);
        out.push_str(&s[a..g1]);
        out.push_str(enc);
        last = e;
    }
    out.push_str(&s[last..]);
    out
}

/// Python `int(name)` / `int(name.lstrip('x'), 16)` as used by
/// BeautifulSoupHTMLParser.handle_charref; returns the replacement text.
fn charref_text(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let (digits, radix) = if name.starts_with('x') || name.starts_with('X') {
        (&chars[1..], 16)
    } else {
        (&chars[..], 10)
    };
    let n = crate::unescape::parse_saturating(digits, radix);
    if n < 256 {
        // bytearray([n]).decode('windows-1252'), falling back to chr(n)
        if (0x80..=0x9f).contains(&n) {
            if let Some(c) = crate::unescape::invalid_charref(n as u32) {
                return c.to_string();
            }
        }
        return char::from_u32(n as u32).unwrap().to_string();
    }
    match char::from_u32(n as u32) {
        Some(c) if n <= 0x10FFFF => c.to_string(),
        // chr() of a surrogate succeeds in Python (lone surrogate); not
        // representable in Rust -> U+FFFD (documented difference).
        _ => "\u{fffd}".to_string(),
    }
}

impl Sink for Soup {
    fn handle_starttag(&mut self, tag: String, attrs: Vec<(String, Option<String>)>) {
        self.sink_starttag(tag, attrs, true);
    }
    fn handle_startendtag(&mut self, tag: String, attrs: Vec<(String, Option<String>)>) {
        self.sink_starttag(tag.clone(), attrs, false);
        self.sink_endtag(tag, true);
    }
    fn handle_endtag(&mut self, tag: String) {
        self.sink_endtag(tag, true);
    }
    fn handle_data(&mut self, data: String) {
        self.current_data.push(data);
    }
    fn handle_charref(&mut self, name: String) {
        let t = charref_text(&name);
        self.current_data.push(t);
    }
    fn handle_entityref(&mut self, name: String) {
        let data = match bs4_entity(&name) {
            Some(c) => c.to_string(),
            None => format!("&{}", name),
        };
        self.current_data.push(data);
    }
    fn handle_comment(&mut self, data: String) {
        self.emit_special(data, StrType::Comment);
    }
    fn handle_decl(&mut self, data: String) {
        let d: String = data.chars().skip("DOCTYPE ".len()).collect();
        self.emit_special(d, StrType::Doctype);
    }
    fn handle_pi(&mut self, data: String) {
        self.emit_special(data, StrType::ProcessingInstruction);
    }
    fn unknown_decl(&mut self, data: String) {
        let up6: String = data.chars().take(6).collect::<String>();
        if up6.len() == 6 && up6.eq_ignore_ascii_case("CDATA[") {
            let d: String = data.chars().skip(6).collect();
            self.emit_special(d, StrType::CData);
        } else {
            self.emit_special(data, StrType::Declaration);
        }
    }
}
