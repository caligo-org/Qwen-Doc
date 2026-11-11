//! `metrics/table_metric.py::TEDS(structure_only).evaluate(pred, true)`.

use crate::html::{parse_table, Elem, Table, Tag};
use crate::lev::Pattern;
use crate::pystr::py_int;
use crate::ted::unit_tree_edit_distance;
use crate::Error;
use std::collections::HashMap;

#[derive(Default)]
struct Interner(HashMap<String, u32>);

impl Interner {
    fn id(&mut self, s: &str) -> u32 {
        if let Some(&v) = self.0.get(s) {
            return v;
        }
        let v = self.0.len() as u32;
        self.0.insert(s.to_owned(), v);
        v
    }
}

/// `TableTree` node in the form the rename cost needs.
struct TNode {
    tag: u32,
    /// For `td`: interned canonical `int(colspan)`, `int(rowspan)`.
    spans: Option<(u32, u32)>,
    /// For `td`: the cell tokens (`__tokens__[1:-1]`), interned.
    content: Vec<u32>,
}

/// Token for an lxml tag in `'<%s>' % node.tag` / `'</%s>' % node.tag`; also used as the
/// interned tag identity. Non-element tags are lxml factory functions; their `str()` cannot
/// collide with an element name.
fn tag_str(tag: &Tag) -> &str {
    match tag {
        Tag::Element(s) => s,
        Tag::Pi => "\u{0}<cyfunction ProcessingInstruction>",
        Tag::Comment => "\u{0}<cyfunction Comment>",
        Tag::Entity => "\u{0}<cyfunction Entity>",
    }
}

fn is_tag(tag: &Tag, name: &str) -> bool {
    matches!(tag, Tag::Element(s) if s == name)
}

/// `TEDS.tokenize` (iterative: cells can nest deeply).
fn tokenize(elems: &[Elem], idx: usize, interner: &mut Interner, out: &mut Vec<u32>) {
    enum Step {
        Enter(usize),
        Exit(usize),
    }
    let mut buf = [0u8; 4];
    let mut stack = vec![Step::Enter(idx)];
    while let Some(step) = stack.pop() {
        match step {
            Step::Enter(i) => {
                let e = &elems[i];
                out.push(interner.id(&format!("<{}>", tag_str(&e.tag))));
                if let Some(text) = &e.text {
                    out.extend(text.chars().map(|c| interner.id(c.encode_utf8(&mut buf))));
                }
                stack.push(Step::Exit(i));
                for &c in e.children.iter().rev() {
                    stack.push(Step::Enter(c));
                }
            }
            Step::Exit(i) => {
                let e = &elems[i];
                if !is_tag(&e.tag, "unk") {
                    out.push(interner.id(&format!("</{}>", tag_str(&e.tag))));
                }
                if !is_tag(&e.tag, "td") {
                    if let Some(tail) = &e.tail {
                        out.extend(tail.chars().map(|c| interner.id(c.encode_utf8(&mut buf))));
                    }
                }
            }
        }
    }
}

/// `TEDS.load_html_tree`: returns child lists (node 0 is the table) and the node data.
fn load_tree(
    table: &Table,
    structure_only: bool,
    interner: &mut Interner,
) -> Result<(Vec<Vec<usize>>, Vec<TNode>), Error> {
    let elems = &table.elems;
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut nodes: Vec<TNode> = Vec::new();
    // (elem index, parent node) in pre-order, as Python recurses.
    let mut stack: Vec<(usize, usize)> = vec![(0, usize::MAX)];
    while let Some((ei, parent)) = stack.pop() {
        let e = &elems[ei];
        let id = nodes.len();
        let tag = interner.id(tag_str(&e.tag));
        if is_tag(&e.tag, "td") {
            let content = if structure_only {
                Vec::new()
            } else {
                let mut toks = Vec::new();
                tokenize(elems, ei, interner, &mut toks);
                toks[1..toks.len() - 1].to_vec()
            };
            let colspan = py_int(e.colspan.as_deref().unwrap_or("1"))
                .ok_or(Error::InvalidInt("colspan"))?;
            let rowspan = py_int(e.rowspan.as_deref().unwrap_or("1"))
                .ok_or(Error::InvalidInt("rowspan"))?;
            let spans = (interner.id(&colspan), interner.id(&rowspan));
            nodes.push(TNode { tag, spans: Some(spans), content });
        } else {
            nodes.push(TNode { tag, spans: None, content: Vec::new() });
        }
        children.push(Vec::new());
        if parent != usize::MAX {
            children[parent].push(id);
        }
        if !is_tag(&e.tag, "td") {
            for &c in e.children.iter().rev() {
                stack.push((c, id));
            }
        }
    }
    Ok((children, nodes))
}

/// TEDS score between a predicted and a ground-truth HTML table, as
/// `TEDS(structure_only=structure_only).evaluate(pred, gt)` computes it.
///
/// `Err` exactly where the Python code raises (lxml parse errors, `int()` of a malformed
/// `colspan`/`rowspan`, division by zero for two tables without any element inside).
pub fn teds(pred: &str, gt: &str, structure_only: bool) -> Result<f64, Error> {
    if pred.is_empty() || gt.is_empty() {
        return Ok(0.0);
    }
    let pred_table = parse_table(pred)?;
    let gt_table = parse_table(gt)?;
    let (Some(pred_table), Some(gt_table)) = (pred_table, gt_table) else {
        return Ok(0.0);
    };
    let n_nodes = pred_table.descendant_elements.max(gt_table.descendant_elements);
    let mut interner = Interner::default();
    let (c1, n1) = load_tree(&pred_table, structure_only, &mut interner)?;
    let (c2, n2) = load_tree(&gt_table, structure_only, &mut interner)?;
    if n_nodes == 0 {
        // Python computes the (cheap) distance first and then divides by zero.
        return Err(Error::ZeroDivision);
    }
    let patterns: Vec<Option<Pattern>> = n1
        .iter()
        .map(|n| (n.spans.is_some() && !n.content.is_empty()).then(|| Pattern::new(&n.content)))
        .collect();
    let distance = unit_tree_edit_distance(&c1, 0, &c2, 0, |i, j| {
        let (a, b) = (&n1[i], &n2[j]);
        if a.tag != b.tag || a.spans != b.spans {
            return 1.0;
        }
        if a.spans.is_some() && (!a.content.is_empty() || !b.content.is_empty()) {
            let max = a.content.len().max(b.content.len());
            let d = match &patterns[i] {
                Some(p) => {
                    if a.content == b.content {
                        0
                    } else {
                        p.distance(&b.content)
                    }
                }
                None => b.content.len(),
            };
            return d as f64 / max as f64;
        }
        0.0
    });
    Ok(1.0 - distance / n_nodes as f64)
}

/// One node of [`debug_parse`]: depth, tag, raw `colspan`/`rowspan` and cell tokens (`td` only).
pub type DebugNode = (usize, String, Option<String>, Option<String>, Option<Vec<String>>);

/// Debug view of the parse: the nodes `load_html_tree` visits in pre-order with the raw span
/// attributes and the (full-mode) cell tokens of `td`s, plus `len(.//*)`.
/// `Err("ERR ...")` where lxml raises, `Err("NOTABLE")` when `xpath('body/table')` is empty.
pub fn debug_parse(html: &str) -> Result<(usize, Vec<DebugNode>), String> {
    let table = match parse_table(html) {
        Err(e) => return Err(format!("ERR {e:?}")),
        Ok(None) => return Err("NOTABLE".to_owned()),
        Ok(Some(t)) => t,
    };
    let mut interner = Interner::default();
    let mut out = Vec::new();
    let mut stack = vec![(0usize, 0usize)];
    let mut toks_by_elem = Vec::new();
    while let Some((n, d)) = stack.pop() {
        let e = &table.elems[n];
        let td = is_tag(&e.tag, "td");
        let toks = td.then(|| {
            let mut t = Vec::new();
            tokenize(&table.elems, n, &mut interner, &mut t);
            toks_by_elem.push(t.clone());
            t[1..t.len() - 1].to_vec()
        });
        out.push((d, tag_str(&e.tag).to_owned(), e.colspan.clone(), e.rowspan.clone(), toks));
        if !td {
            for &c in e.children.iter().rev() {
                stack.push((c, d + 1));
            }
        }
    }
    let mut names: Vec<String> = vec![String::new(); interner.0.len()];
    for (k, &v) in &interner.0 {
        names[v as usize] = k.clone();
    }
    let out = out
        .into_iter()
        .map(|(d, t, c, r, toks)| {
            (d, t, c, r, toks.map(|v| v.iter().map(|&x| names[x as usize].clone()).collect()))
        })
        .collect();
    Ok((table.descendant_elements, out))
}
