//! `metrics/head_metric.py::HeadTEDS().evaluate(pred, gold)` for a Markdown heading list
//! `pred` and an annotated gold `[parent_son_list, head_nodes]`.

use crate::lev::Pattern;
use crate::pystr::{is_py_space, py_strip};
use crate::Error;
use std::collections::HashMap;

/// A relation endpoint / `head_nodes` key: an `anno_id` or the string `"root"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HeadId {
    Root,
    Int(i64),
}

/// One `head_nodes` entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadNode {
    /// The `head_nodes` key.
    pub id: HeadId,
    /// `node["text"]`; `None` when it is not a `str` (Python then raises `TypeError` if the
    /// node is reached from the root).
    pub text: Option<String>,
    /// `node["anno_id"]`, the reading-order key children are sorted by.
    pub anno_id: i64,
}

/// A heading tree as `TedsHeadNode`s: node 0 is the root.
struct Tree {
    content: Vec<Vec<u32>>,
    children: Vec<Vec<usize>>,
}

impl Tree {
    fn new_root() -> Tree {
        Tree {
            content: vec![chars("root")],
            children: vec![Vec::new()],
        }
    }

    fn add(&mut self, parent: usize, content: Vec<u32>) -> usize {
        let id = self.content.len();
        self.content.push(content);
        self.children.push(Vec::new());
        self.children[parent].push(id);
        id
    }
}

fn chars(s: &str) -> Vec<u32> {
    s.chars().map(|c| c as u32).collect()
}

/// Adds `child` under the nearest node on `path` with a smaller depth (the rule of
/// `build_md_head_tree` and `markdown_head_tree`); `path` holds (node, depth).
fn push_on_path(tree: &mut Tree, path: &mut Vec<(usize, i64)>, content: Vec<u32>, depth: i64) {
    while depth <= path.last().unwrap().1 {
        path.pop();
    }
    let parent = path.last().unwrap().0;
    let id = tree.add(parent, content);
    path.push((id, depth));
}

/// `HeadTEDS.build_md_head_tree`
fn build_md_head_tree(node_list: &[String]) -> Result<Tree, Error> {
    let mut tree = Tree::new_root();
    let mut path: Vec<(usize, i64)> = vec![(0, 0)];
    for raw in node_list {
        let node = py_strip(raw);
        let hashes = node.chars().take_while(|&c| c == '#').count();
        let (level, title) = if hashes > 0 && node.chars().nth(hashes).is_some_and(is_py_space) {
            // re.match(r'^#+\s', node): label, title = node.split(' ', 1)
            let (label, title) = node.split_once(' ').ok_or(Error::HeadingSplit)?;
            (label.chars().count() as i64, py_strip(title).to_owned())
        } else {
            let level = node.chars().filter(|&c| c == '#').count() as i64;
            if level == 0 {
                continue;
            }
            (level, py_strip(&node.replace('#', "")).to_owned())
        };
        push_on_path(&mut tree, &mut path, chars(&title), level);
    }
    Ok(tree)
}

/// One `re.sub` of `remove_latex_style`: `\<name>` [`\*?`] [`\s*`] `{` [`\s*`] `(.*?)` [`\s*`] `}`
/// replaced by the group. Without `dotall`, `.` does not match `\n`.
fn sub_wrapped(
    s: &[char],
    name: &str,
    opt_star: bool,
    ws_before_brace: bool,
    ws_inside: bool,
    dotall: bool,
) -> Vec<char> {
    let name: Vec<char> = name.chars().collect();
    let mut out = Vec::with_capacity(s.len());
    let mut i = 0;
    'scan: while i < s.len() {
        if s[i] == '\\' && s[i + 1..].starts_with(&name) {
            let mut j = i + 1 + name.len();
            if opt_star && s.get(j) == Some(&'*') {
                j += 1;
            }
            if ws_before_brace {
                while j < s.len() && is_py_space(s[j]) {
                    j += 1;
                }
            }
            if s.get(j) == Some(&'{') {
                j += 1;
                if ws_inside {
                    while j < s.len() && is_py_space(s[j]) {
                        j += 1;
                    }
                }
                let a = j;
                let mut p = a;
                while p < s.len() && s[p] != '}' && (dotall || s[p] != '\n') {
                    p += 1;
                }
                if p < s.len() && s[p] == '}' {
                    let mut q = p;
                    if ws_inside {
                        while q > a && is_py_space(s[q - 1]) {
                            q -= 1;
                        }
                    }
                    out.extend_from_slice(&s[a..q]);
                    i = p + 1;
                    continue 'scan;
                }
            }
        }
        out.push(s[i]);
        i += 1;
    }
    out
}

/// `utils/extract.py::remove_latex_style`
fn remove_latex_style(text: &str) -> Vec<char> {
    let s: Vec<char> = text.chars().collect();
    let s = sub_wrapped(&s, "title", false, false, false, false);
    let s = sub_wrapped(&s, "title", false, true, true, true);
    let s = sub_wrapped(&s, "text", false, true, true, true);
    let s = sub_wrapped(&s, "section", true, false, false, false);
    sub_wrapped(&s, "section", true, false, true, true)
}

/// `HeadTEDS.markdown_text`: `re.sub(r'\s*\n\s*', ' ', remove_latex_style(text)).strip()`
pub fn markdown_text(text: &str) -> String {
    let s = remove_latex_style(text);
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        if is_py_space(s[i]) {
            let mut j = i;
            let mut newline = false;
            while j < s.len() && is_py_space(s[j]) {
                newline |= s[j] == '\n';
                j += 1;
            }
            if newline {
                out.push(' ');
            } else {
                out.extend(&s[i..j]);
            }
            i = j;
        } else {
            out.push(s[i]);
            i += 1;
        }
    }
    py_strip(&out).to_owned()
}

/// `TedsHeadNode` of `build_json_head_tree`.
struct JsonNode {
    text: Option<String>,
    order: i64,
    children: Vec<usize>,
    parent: Option<usize>,
}

/// `HeadTEDS.build_json_head_tree`; node 0 is the root. The result can be a DAG.
fn build_json_head_tree(
    relations: &[(HeadId, HeadId)],
    nodes: &[HeadNode],
) -> Result<Vec<JsonNode>, Error> {
    let mut items: HashMap<HeadId, &HeadNode> = HashMap::new();
    for n in nodes {
        items.insert(n.id, n); // a later duplicate key wins, as in a dict
    }
    let mut arena = vec![JsonNode {
        text: Some("root".to_owned()),
        order: 0,
        children: Vec::new(),
        parent: None,
    }];
    let mut map: HashMap<HeadId, usize> = HashMap::new();
    let mut map_order: Vec<HeadId> = vec![HeadId::Root];
    map.insert(HeadId::Root, 0);
    for &(s, t) in relations {
        for id in [s, t] {
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(id) {
                let item = items.get(&id).ok_or(Error::MissingNode)?;
                e.insert(arena.len());
                map_order.push(id);
                arena.push(JsonNode {
                    text: item.text.clone(),
                    order: item.anno_id,
                    children: Vec::new(),
                    parent: None,
                });
            }
        }
    }
    for &(s, t) in relations {
        let (p, c) = (map[&s], map[&t]);
        arena[p].children.push(c);
        arena[c].parent = Some(p);
    }
    for id in &map_order {
        if *id == HeadId::Root {
            continue;
        }
        let k = map[id];
        if arena[k].parent.is_none() {
            arena[0].children.push(k);
            arena[k].parent = Some(0);
        }
    }
    for k in 0..arena.len() {
        let mut ch = std::mem::take(&mut arena[k].children);
        ch.sort_by_key(|&c| arena[c].order); // stable, like list.sort
        arena[k].children = ch;
    }
    Ok(arena)
}

/// `HeadTEDS.get_max_depth` on the (possibly DAG-shaped) annotated tree. A cycle reachable
/// from the root makes Python recurse until `RecursionError`.
fn get_max_depth(arena: &[JsonNode]) -> Result<i64, Error> {
    if arena[0].children.is_empty() {
        return Ok(0);
    }
    // 0 = unvisited, 1 = on the DFS path, 2 = done
    let mut state = vec![0u8; arena.len()];
    let mut depth = vec![0i64; arena.len()];
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    state[0] = 1;
    while let Some(top) = stack.last_mut() {
        let (n, next) = *top;
        if next < arena[n].children.len() {
            top.1 += 1;
            let c = arena[n].children[next];
            match state[c] {
                0 => {
                    state[c] = 1;
                    stack.push((c, 0));
                }
                1 => return Err(Error::Cycle),
                _ => {}
            }
        } else {
            stack.pop();
            state[n] = 2;
            depth[n] = 1 + arena[n]
                .children
                .iter()
                .map(|&c| depth[c])
                .max()
                .unwrap_or(0);
        }
    }
    Ok(arena[0].children.iter().map(|&c| depth[c]).max().unwrap())
}

/// `HeadTEDS.markdown_head_tree`
fn markdown_head_tree(arena: &[JsonNode]) -> Result<Tree, Error> {
    let mut nodes: Vec<(usize, i64)> = Vec::new();
    let mut stack: Vec<(usize, i64)> = vec![(0, 0)];
    while let Some((n, d)) = stack.pop() {
        if n != 0 {
            nodes.push((n, d));
        }
        stack.extend(arena[n].children.iter().map(|&c| (c, d + 1)));
    }
    nodes.sort_by_key(|&(n, _)| arena[n].order);
    let mut tree = Tree::new_root();
    let mut path: Vec<(usize, i64)> = vec![(0, 0)];
    for (n, d) in nodes {
        let text = arena[n].text.as_deref().ok_or(Error::NonStringText)?;
        push_on_path(&mut tree, &mut path, chars(&markdown_text(text)), d);
    }
    Ok(tree)
}

/// HeadTEDS between Markdown heading lines `pred` and the annotated gold
/// `[relations, nodes]`: returns `(score, gold tree depth)` as `HeadTEDS().evaluate` does,
/// or `Err` where it raises.
pub fn head_teds(
    pred: &[String],
    relations: &[(HeadId, HeadId)],
    nodes: &[HeadNode],
) -> Result<(f64, i64), Error> {
    let pred_tree = build_md_head_tree(pred)?;
    let annotated = build_json_head_tree(relations, nodes)?;
    let gold_depth = get_max_depth(&annotated)?;
    let gold_tree = markdown_head_tree(&annotated)?;
    let patterns: Vec<Pattern> = pred_tree.content.iter().map(|c| Pattern::new(c)).collect();
    let distance = crate::apted::apted(&pred_tree.children, 0, &gold_tree.children, 0, |i, j| {
        let (a, b) = (&pred_tree.content[i], &gold_tree.content[j]);
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }
        let d = if a == b { 0 } else { patterns[i].distance(b) };
        d as f64 / a.len().max(b.len()) as f64
    })?;
    let pred_count = pred_tree.content.len();
    let gold_count = gold_tree.content.len();
    Ok((
        1.0 - distance / pred_count.max(gold_count) as f64,
        gold_depth,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latex_style() {
        assert_eq!(markdown_text("\\title{ A }"), " A ".trim());
        assert_eq!(markdown_text("x \\text { a\n b } y"), "x a b y");
        assert_eq!(markdown_text("\\section*{Intro}"), "Intro");
        assert_eq!(markdown_text("a \n\t b"), "a b");
        assert_eq!(markdown_text("a  b"), "a  b");
    }
}
