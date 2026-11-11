//! Port of `dataset/end2end_dataset.py` (End2EndDataset): per-document matching and the
//! assembly of the per-element sample lists.

use crate::extract::{self, md_tex_filter};
use crate::matching::{match_gt2pred_quick, match_gt2pred_simple, LineType};
use crate::preprocess::clean_string;
use crate::py;
use crate::pyval::{Map, V};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Opts {
    /// MarathonDocBench evaluator patch semantics
    pub marathon: bool,
    /// no requested element uses the text match (text_block, merged_text_block,
    /// display_formula, reading_order): skip it; the other elements do not depend on it
    pub skip_text: bool,
}

/// Hashable form of a GT `anno_id` (ints in both benchmarks).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Int(i64),
    Str(String),
    Other(String),
}

pub fn key_of(v: &V) -> Key {
    match v {
        V::Int(i) => Key::Int(*i),
        V::Bool(b) => Key::Int(*b as i64),
        V::Float(f) if f.fract() == 0.0 => Key::Int(*f as i64),
        V::Str(s) => Key::Str(s.to_string()),
        other => Key::Other(other.py_repr()),
    }
}

fn py_sort_by_order(items: &mut Vec<V>, key: impl Fn(&V) -> V) {
    let keys: Vec<V> = items.iter().map(&key).collect();
    let mut idx: Vec<usize> = (0..items.len()).collect();
    idx.sort_by(|&a, &b| {
        keys[a].py_cmp(&keys[b]).unwrap_or_else(|_| {
            panic!("TypeError: cannot order {} and {}", keys[a].py_repr(), keys[b].py_repr())
        })
    });
    let sorted: Vec<V> = idx.iter().map(|&i| items[i].clone()).collect();
    *items = sorted;
}

/// `x.get('order') or 0`
fn order_or_zero(x: &V) -> V {
    let o = x.get("order");
    if o.truthy() {
        o.clone()
    } else {
        V::Int(0)
    }
}

fn remove_consecutive_duplicates(arr: &[Key]) -> Vec<Key> {
    let mut out: Vec<Key> = Vec::new();
    for (i, k) in arr.iter().enumerate() {
        if i == 0 || *k != arr[i - 1] {
            out.push(k.clone());
        }
    }
    out
}

/// Ordered category → items map (a Python defaultdict(list)).
pub type Elements = Vec<(String, Vec<V>)>;

fn push_cat(d: &mut Elements, cat: &str, item: V) {
    match d.iter_mut().find(|(c, _)| c == cat) {
        Some((_, v)) => v.push(item),
        None => d.push((cat.to_string(), vec![item])),
    }
}

fn get_cat<'a>(d: &'a Elements, cat: &str) -> &'a [V] {
    d.iter().find(|(c, _)| c == cat).map(|(_, v)| v.as_slice()).unwrap_or(&[])
}

/// `get_page_elements`: merge truncated groups, bucket elements by category.
pub fn get_page_elements(sample: &V, opts: Opts) -> Elements {
    let mut saved: Elements = Vec::new();
    let mut related: Vec<Vec<Key>> = Vec::new();
    let mut truncated_all: HashMap<Key, Option<V>> = HashMap::new();
    let empty = Vec::new();
    let relations = sample.get("extra").get("relation").as_list().unwrap_or(&empty);
    for r in relations.iter() {
        if r.get("relation_type").as_str() == Some("truncated") {
            let s = key_of(r.get("source_anno_id"));
            let t = key_of(r.get("target_anno_id"));
            truncated_all.insert(s.clone(), None);
            truncated_all.insert(t.clone(), None);
            let mut exist = false;
            for ml in related.iter_mut() {
                if ml.contains(&s) || ml.contains(&t) {
                    ml.push(s.clone());
                    ml.push(t.clone());
                    exist = true;
                }
            }
            if !exist {
                related.push(vec![s, t]);
            }
        }
    }
    for item in sample.get("layout_dets").as_list().unwrap_or(&empty).iter() {
        let k = key_of(item.get("anno_id"));
        if let Some(slot) = truncated_all.get_mut(&k) {
            *slot = Some(item.clone());
        } else {
            push_cat(&mut saved, &item.get("category_type").py_str(), item.clone());
        }
    }
    for ml in related {
        let ml = remove_consecutive_duplicates(&ml);
        let mut blocks: Vec<V> = ml
            .iter()
            .map(|k| truncated_all[k].clone().unwrap_or_else(|| panic!("truncated relation to missing anno_id {:?}", k)))
            .collect();
        py_sort_by_order(&mut blocks, order_or_zero);
        let mut page_ids: Vec<Key> = Vec::new();
        for b in &blocks {
            if !b.has("page_id") {
                panic!("KeyError: 'page_id'");
            }
            let k = key_of(b.get("page_id"));
            if !page_ids.contains(&k) {
                page_ids.push(k);
            }
        }
        let first = blocks[0].clone();
        let cat = first.get("category_type").py_str();
        let mut m = Map::new();
        if cat == "table" {
            let html = if !first.get("merged_html").is_null() {
                first.get("merged_html").clone()
            } else {
                let img = sample.get("page_info").get("image_path").py_str();
                eprintln!(
                    "{} has a merged table group {} with no merged_html annotation.",
                    img,
                    blocks.iter().map(|b| b.get("anno_id").py_str()).collect::<Vec<_>>().join("_")
                );
                let tables: Vec<String> = blocks.iter().map(|b| b.get("html").py_str()).collect();
                V::from(pysoup::merge_tables(&tables).unwrap_or_else(|e| panic!("merge_tables failed: {:?}", e)))
            };
            m.insert("category_type".into(), first.get("category_type").clone());
            m.insert("order".into(), first.get("order").clone());
            m.insert("anno_id".into(), first.get("anno_id").clone());
            m.insert("html".into(), html);
            m.insert("merge_list".into(), V::list(blocks.clone()));
            m.insert("is_merged".into(), V::str("table"));
            let mut a = Map::new();
            a.insert("table_merge_cross_pages".into(), V::from(page_ids.len().to_string()));
            m.insert("attribute".into(), V::from(a));
        } else {
            let mut text = String::new();
            for b in &blocks {
                match b.get("text") {
                    V::Str(s) => text.push_str(s),
                    V::Null if !b.has("text") => {}
                    _ => {
                        // `text += block.get('text', "")` raised TypeError: the loop stops
                        eprintln!("non-str text in truncated group");
                        break;
                    }
                }
            }
            let merged_cats: &[&str] = if opts.marathon {
                &["text_block", "title", "reference", "code_txt", "code_txt_caption"]
            } else {
                &["text_block"]
            };
            let is_merged = if merged_cats.contains(&cat.as_str()) { "text_block" } else { "unknown" };
            m.insert("category_type".into(), first.get("category_type").clone());
            m.insert("order".into(), first.get("order").clone());
            m.insert("anno_id".into(), first.get("anno_id").clone());
            m.insert("text".into(), V::from(text));
            m.insert("merge_list".into(), V::list(blocks.clone()));
            m.insert("is_merged".into(), V::str(is_merged));
            if is_merged == "text_block" {
                let mut a = Map::new();
                a.insert("text_merge_cross_pages".into(), V::from(page_ids.len().to_string()));
                m.insert("attribute".into(), V::from(a));
            }
        }
        push_cat(&mut saved, &cat, V::from(m));
    }
    saved
}

/// `get_document_heads`: (parent_son relations, title nodes keyed by anno_id in insertion order).
pub fn get_document_heads(sample: &V) -> (Vec<V>, Vec<(Key, V)>) {
    let mut head_nodes: Vec<(Key, V)> = Vec::new();
    let mut index: HashMap<Key, usize> = HashMap::new();
    let empty = Vec::new();
    for item in sample.get("layout_dets").as_list().unwrap_or(&empty).iter() {
        if item.get("category_type").as_str() == Some("title") {
            let k = key_of(item.get("anno_id"));
            match index.get(&k) {
                Some(&i) => head_nodes[i].1 = item.clone(),
                None => {
                    index.insert(k.clone(), head_nodes.len());
                    head_nodes.push((k, item.clone()));
                }
            }
        }
    }
    let mut rels = Vec::new();
    for r in sample.get("extra").get("relation").as_list().unwrap_or(&empty).iter() {
        if r.get("relation_type").as_str() == Some("parent_son") {
            let s = r.get("source_anno_id");
            let t = key_of(r.get("target_anno_id"));
            let s_ok = index.contains_key(&key_of(s)) || s.as_str() == Some("root");
            if s_ok && index.contains_key(&t) {
                rels.push(r.clone());
            }
        }
    }
    (rels, head_nodes)
}

fn sorted_text_list(items: &[V]) -> Vec<V> {
    let mut v = items.to_vec();
    py_sort_by_order(&mut v, order_or_zero);
    v
}

pub struct HeadSample {
    /// JSON form written to head_result.json: {"gt": [relations, nodes], "pred": [...], ...}
    pub sample: V,
    pub relations: Vec<V>,
    pub nodes: Vec<(Key, V)>,
    pub pred: Vec<String>,
}

pub struct MainResult {
    pub plain_text: Vec<V>,
    pub merged_plain_text: Vec<V>,
    pub display_formula: Vec<V>,
    pub latex_table: Vec<V>,
    pub merged_latex_table: Vec<V>,
    pub html_table: Vec<V>,
    pub merged_html_table: Vec<V>,
    pub order: Option<V>,
    pub head: HeadSample,
}

pub struct DocResult {
    pub figure: Option<V>,
    pub main: Option<MainResult>,
}

fn not_gt_empty(x: &V) -> bool {
    let g = x.get("gt_idx");
    !(g.as_list().map_or(false, |l| l.len() == 1 && l[0].as_str() == Some("")))
}

fn compute_iou(a: [f64; 4], b: [f64; 4]) -> f64 {
    let x_left = a[0].max(b[0]);
    let y_top = a[1].max(b[1]);
    let x_right = a[2].min(b[2]);
    let y_bottom = a[3].min(b[3]);
    let inter = (x_right - x_left).max(0.0) * (y_bottom - y_top).max(0.0);
    let area_a = (a[2] - a[0]) * (a[3] - a[1]);
    let area_b = (b[2] - b[0]) * (b[3] - b[1]);
    let union = area_a + area_b - inter;
    if union > 0.0 {
        inter / union
    } else {
        0.0
    }
}

/// `get_figure_matched`
pub fn get_figure_matched(sample: &V, pred: &[extract::FigureBox], img_name: &str) -> Option<V> {
    let page_info = sample.get("page_info");
    let empty = Vec::new();
    let widths = page_info.get("width").as_list().unwrap_or(&empty);
    let heights = page_info.get("height").as_list().unwrap_or(&empty);
    let mut gt: Vec<(i64, [f64; 4], V)> = Vec::new();
    for det in sample.get("layout_dets").as_list().unwrap_or(&empty).iter() {
        if det.get("category_type").as_str() != Some("figure") {
            continue;
        }
        let page_id = det.get("page_id");
        let poly = det.get("poly").as_list().cloned().unwrap_or_default();
        if page_id.is_null() || poly.len() < 6 {
            continue;
        }
        let page_id = page_id.as_i64().expect("page_id");
        let page_idx = page_id - 1;
        if page_idx < 0 || page_idx as usize >= widths.len() || page_idx as usize >= heights.len() {
            continue;
        }
        let (pw, ph) = (&widths[page_idx as usize], &heights[page_idx as usize]);
        if !pw.truthy() || !ph.truthy() {
            continue;
        }
        let (pw, ph) = (pw.as_f64().unwrap(), ph.as_f64().unwrap());
        let p = |i: usize| poly[i].as_f64().expect("poly");
        let b = [p(0) / pw * 1000.0, p(1) / ph * 1000.0, p(4) / pw * 1000.0, p(5) / ph * 1000.0];
        let label = if det.has("label") { det.get("label").clone() } else { V::str("figure") };
        gt.push((page_id, b, label));
    }
    if gt.is_empty() {
        return None;
    }
    let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
    // gt_by_page iterates pages in first-appearance order; pairs within (gi, pi) order
    let mut pages: Vec<i64> = Vec::new();
    for g in &gt {
        if !pages.contains(&g.0) {
            pages.push(g.0);
        }
    }
    for &page in &pages {
        for (gi, g) in gt.iter().enumerate().filter(|(_, g)| g.0 == page) {
            for (pi, p) in pred.iter().enumerate().filter(|(_, p)| p.page_id == page) {
                let pb = [p.bbox[0] as f64, p.bbox[1] as f64, p.bbox[2] as f64, p.bbox[3] as f64];
                let iou = compute_iou(g.1, pb);
                if iou >= 0.5 {
                    pairs.push((gi, pi, iou));
                }
            }
        }
    }
    pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    let mut matched_gt = vec![false; gt.len()];
    let mut matched_pred = vec![false; pred.len()];
    let mut matched = 0usize;
    for (gi, pi, _) in pairs {
        if !matched_gt[gi] && !matched_pred[pi] {
            matched_gt[gi] = true;
            matched_pred[pi] = true;
            matched += 1;
        }
    }
    let mut label_stats: Vec<(String, i64, i64)> = Vec::new();
    for (i, g) in gt.iter().enumerate() {
        let label = match &g.2 {
            V::Str(s) => s.to_string(),
            V::Null => "null".to_string(),
            other => other.py_str(),
        };
        let pos = match label_stats.iter().position(|x| x.0 == label) {
            Some(p) => p,
            None => {
                label_stats.push((label, 0, 0));
                label_stats.len() - 1
            }
        };
        label_stats[pos].1 += 1;
        if matched_gt[i] {
            label_stats[pos].2 += 1;
        }
    }
    let mut ls = Map::new();
    for (l, g, m) in label_stats {
        let mut d = Map::new();
        d.insert("gt_count".into(), V::Int(g));
        d.insert("matched_count".into(), V::Int(m));
        ls.insert(l, V::from(d));
    }
    let mut m = Map::new();
    m.insert("img_id".into(), V::str(img_name));
    m.insert("gt_count".into(), V::from(gt.len()));
    m.insert("pred_count".into(), V::from(pred.len()));
    m.insert("matched_count".into(), V::from(matched));
    m.insert("gt".into(), V::str(""));
    m.insert("pred".into(), V::str(""));
    m.insert(
        "gt_attribute".into(),
        V::list(vec![if page_info.has("page_attribute") { page_info.get("page_attribute").clone() } else { V::empty_dict() }]),
    );
    m.insert("metric".into(), V::empty_dict());
    m.insert("label_stats".into(), V::from(ls));
    Some(V::from(m))
}

fn get_order_paired(items: &[V], img_name: &str) -> Option<V> {
    let empty_marker = |v: &V| v.as_list().map_or(false, |l| l.len() == 1 && l[0].as_str() == Some(""));
    let mut matched: Vec<(V, V)> = Vec::new();
    let mut gt_all: Vec<V> = Vec::new();
    for it in items {
        let gp = it.get("gt_position");
        let pp = it.get("pred_position");
        if !empty_marker(gp) && !(pp.as_str() == Some("")) {
            matched.push((gp.clone(), pp.clone()));
        }
        if !empty_marker(gp) {
            gt_all.push(gp.clone());
        }
    }
    // sorted(matched, key=pred_position): stable
    let mut idx: Vec<usize> = (0..matched.len()).collect();
    idx.sort_by(|&a, &b| matched[a].1.py_cmp(&matched[b].1).expect("pred_position order"));
    let mut gt: Vec<V> = gt_all.iter().flat_map(|l| l.as_list().unwrap().iter().cloned()).filter(|x| x.truthy()).collect();
    gt.sort_by(|a, b| a.py_cmp(b).expect("gt order"));
    let pred: Vec<V> = idx
        .iter()
        .flat_map(|&i| matched[i].0.as_list().unwrap().iter().cloned().collect::<Vec<_>>())
        .filter(|x| x.truthy())
        .collect();
    if pred.is_empty() && gt.is_empty() {
        return None;
    }
    // Levenshtein over element sequences: map equal values to equal symbols
    let mut syms: Vec<V> = Vec::new();
    let mut sym = |v: &V| -> u32 {
        match syms.iter().position(|s| s.py_eq(v)) {
            Some(p) => p as u32,
            None => {
                syms.push(v.clone());
                (syms.len() - 1) as u32
            }
        }
    };
    let gs: Vec<u32> = gt.iter().map(&mut sym).collect();
    let ps: Vec<u32> = pred.iter().map(&mut sym).collect();
    let edit = crate::lev::distance(&gs, &ps) as f64 / gt.len().max(pred.len()) as f64;
    let mut m = Map::new();
    m.insert("gt".into(), V::list(gt));
    m.insert("pred".into(), V::list(pred));
    m.insert("img_id".into(), V::str(img_name));
    m.insert("edit".into(), V::Float(edit));
    Some(V::from(m))
}

const MIX_CATS: [&str; 17] = [
    "text_block",
    "title",
    "code_txt",
    "code_txt_caption",
    "reference",
    "equation_caption",
    "figure_caption",
    "figure_footnote",
    "table_caption",
    "table_footnote",
    "code_algorithm",
    "code_algorithm_caption",
    "header",
    "footer",
    "page_footnote",
    "page_number",
    "equation_isolated",
];
const PLAIN_CATS: [&str; 16] = [
    "text_block",
    "title",
    "code_txt",
    "code_txt_caption",
    "reference",
    "equation_caption",
    "figure_caption",
    "figure_footnote",
    "table_caption",
    "table_footnote",
    "code_algorithm",
    "code_algorithm_caption",
    "header",
    "footer",
    "page_footnote",
    "page_number",
];
const TEXT_IGNORE: [&str; 11] = [
    "figure_caption",
    "figure_footnote",
    "table_caption",
    "table_footnote",
    "code_algorithm",
    "code_algorithm_caption",
    "header",
    "footer",
    "page_footnote",
    "page_number",
    "equation_caption",
];

/// `process_get_matched_elements` (quick_match).
pub fn process_get_matched_elements(sample: &V, pred_content: &str, img_name: &str, opts: Opts, simple: bool) -> MainResult {
    let mk = opts.marathon;
    let pred_dataset = md_tex_filter(pred_content);
    let gt_elements = get_page_elements(sample, opts);
    let (rels, head_nodes) = get_document_heads(sample);

    let mut pred_mix: Vec<V> = Vec::new();
    for (cat, items) in &pred_dataset {
        if !["html_table", "latex_table", "md2html_table"].contains(&cat.as_str()) {
            pred_mix.extend(items.iter().cloned());
        }
    }
    let mut gt_mix: Vec<V> = Vec::new();
    for c in MIX_CATS {
        gt_mix.extend(get_cat(&gt_elements, c).iter().cloned());
    }
    if !gt_mix.is_empty() {
        gt_mix = sorted_text_list(&gt_mix);
    }

    let mut latex_table = Vec::new();
    let mut merged_latex_table = Vec::new();
    let mut html_table = Vec::new();
    let mut merged_html_table = Vec::new();
    let tables = get_cat(&gt_elements, "table");
    if !tables.is_empty() {
        let gt_table = sorted_text_list(tables);
        let merged_gt_table: Vec<V> =
            gt_table.iter().filter(|t| t.get("is_merged").as_str() == Some("table")).cloned().collect();
        let latex = extract::category(&pred_dataset, "latex_table");
        let html = extract::category(&pred_dataset, "html_table");
        let unmatch;
        if latex.is_empty() && html.is_empty() {
            let (m, u) = match_gt2pred_simple(&gt_table, &[], LineType::HtmlTable, img_name, mk);
            html_table = m.into_iter().filter(not_gt_empty).collect();
            unmatch = u;
            let (m, _) = match_gt2pred_simple(&merged_gt_table, &[], LineType::HtmlTable, img_name, mk);
            merged_html_table = m.into_iter().filter(not_gt_empty).collect();
        } else if latex.len() > html.len() {
            let (m, u) = match_gt2pred_simple(&gt_table, latex, LineType::LatexTable, img_name, mk);
            latex_table = m.into_iter().filter(not_gt_empty).collect();
            unmatch = u;
            let (m, _) = match_gt2pred_simple(&merged_gt_table, latex, LineType::LatexTable, img_name, mk);
            merged_latex_table = m.into_iter().filter(not_gt_empty).collect();
        } else {
            let (m, u) = match_gt2pred_simple(&gt_table, html, LineType::HtmlTable, img_name, mk);
            html_table = m.into_iter().filter(not_gt_empty).collect();
            unmatch = u;
            let (m, _) = match_gt2pred_simple(&merged_gt_table, html, LineType::HtmlTable, img_name, mk);
            merged_html_table = m.into_iter().filter(not_gt_empty).collect();
        }
        if let Some(u) = unmatch {
            pred_mix.extend(u);
        }
    }

    // get_head_paired sorts pred_mix in place (stable) before the text match uses it
    let mut idx: Vec<usize> = (0..pred_mix.len()).collect();
    idx.sort_by_key(|&i| pred_mix[i].get("position").as_list().unwrap()[0].as_i64().unwrap());
    pred_mix = idx.iter().map(|&i| pred_mix[i].clone()).collect();
    let md_heads: Vec<String> = pred_mix
        .iter()
        .map(|i| py::strip(i.get("content").as_str().unwrap_or("")).to_string())
        .filter(|c| c.starts_with('#'))
        .collect();
    let mut nodes_json = Map::new();
    for (k, v) in &head_nodes {
        let ks = match k {
            Key::Int(i) => i.to_string(),
            Key::Str(s) => s.clone(),
            Key::Other(s) => s.clone(),
        };
        nodes_json.insert(ks, v.clone());
    }
    let mut hs = Map::new();
    hs.insert("gt".into(), V::list(vec![V::list(rels.clone()), V::from(nodes_json)]));
    hs.insert("pred".into(), V::list(md_heads.iter().map(|s| V::from(s.as_str())).collect()));
    hs.insert("img_id".into(), V::str(img_name));
    hs.insert("edit".into(), V::Float(1.0));
    let head = HeadSample { sample: V::from(hs), relations: rels, nodes: head_nodes, pred: md_heads };

    // `simple`: the original's func_timeout fallback (match_gt2pred_simple) for this document
    let matched = if opts.skip_text {
        Vec::new()
    } else if simple {
        match_gt2pred_simple(&gt_mix, &pred_mix, LineType::TextAll, img_name, mk).0
    } else {
        match_gt2pred_quick(&gt_mix, &pred_mix, img_name, mk)
    };

    let merged_gt_mix: Vec<V> =
        gt_mix.iter().filter(|g| g.get("is_merged").as_str() == Some("text_block")).cloned().collect();
    let (m, _) = if opts.skip_text {
        (Vec::new(), None)
    } else {
        match_gt2pred_simple(&merged_gt_mix, &pred_mix, LineType::TextAll, img_name, mk)
    };
    let merged_plain_text: Vec<V> = m.into_iter().filter(not_gt_empty).collect();

    let mut plain_s = Vec::new();
    let mut display = Vec::new();
    for it in matched {
        let c = it.get("gt_category_type");
        match c.as_str() {
            Some(c) if PLAIN_CATS.contains(&c) => plain_s.push(it),
            Some("equation_isolated") => display.push(it),
            _ => {}
        }
    }
    let display: Vec<V> = display.into_iter().filter(not_gt_empty).collect();
    let plain: Vec<V> = plain_s
        .into_iter()
        .filter(|i| !TEXT_IGNORE.contains(&i.get("gt_category_type").as_str().unwrap_or("")))
        .collect();
    let order = if !plain.is_empty() { get_order_paired(&plain, img_name) } else { None };

    MainResult {
        plain_text: plain,
        merged_plain_text,
        display_formula: display,
        latex_table,
        merged_latex_table,
        html_table,
        merged_html_table,
        order,
        head,
    }
}

/// Python `open(path, 'r', encoding='utf-8').read()` (universal newlines).
pub fn read_md_file(path: &std::path::Path) -> std::io::Result<String> {
    let bytes = std::fs::read(path)?;
    let s = String::from_utf8(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(if s.contains('\r') { s.replace("\r\n", "\n").replace('\r', "\n") } else { s })
}

/// Prediction path lookup of the original (md, nougat .mmd, marker, mineru).
pub fn find_pred(pred_folder: &std::path::Path, img_name: &str) -> Option<std::path::PathBuf> {
    let stem: String = img_name.chars().take(img_name.chars().count().saturating_sub(4)).collect();
    let cands = [
        format!("{}.md", stem),
        format!("{}.mmd", stem.replace(".pdf", "")),
        format!("{}.md", stem.replace(".pdf", "")),
        format!("{}.md", img_name),
    ];
    cands.iter().map(|c| pred_folder.join(c)).find(|p| p.exists())
}

/// Per-document work (`_process_one_doc`).
pub fn process_doc(sample: &V, pred_folder: &std::path::Path, opts: Opts, need_main: bool, need_figure: bool, simple: bool) -> Option<DocResult> {
    let image_path = sample.get("page_info").get("image_path").py_str();
    let img_name = image_path.rsplit('/').next().unwrap_or(&image_path).to_string();
    let path = match find_pred(pred_folder, &img_name) {
        Some(p) => p,
        None => {
            eprintln!("!!!WARNING: No prediction for {}", img_name);
            return None;
        }
    };
    let content = read_md_file(&path).unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e));
    let figure = if need_figure { get_figure_matched(sample, &extract::extract_figure_boxes(&content), &img_name) } else { None };
    let main = if need_main { Some(process_get_matched_elements(sample, &content, &img_name, opts, simple)) } else { None };
    Some(DocResult { figure, main })
}

/// The display-formula → text conversion of get_matched_elements for formulas matched to
/// non-formula predictions; returns (kept display formulas, converted text samples).
pub fn split_display_formulas(items: Vec<V>, opts: Opts) -> (Vec<V>, Vec<V>) {
    let mut clean = Vec::new();
    let mut others = Vec::new();
    for mut it in items {
        let pc = it.get("pred_category_type").clone();
        let keep = matches!(pc.as_str(), Some("equation_inline") | Some("equation_isolated") | Some(""));
        if !keep {
            let gt = it.get("gt").py_str();
            let text = match pylatexenc::latex_to_text(&gt) {
                Ok(t) => t,
                Err(e) => {
                    if !opts.marathon {
                        eprintln!("warning: pylatexenc failed on a display formula (the original scorer stops here): {:?}", e);
                    }
                    gt.clone()
                }
            };
            let norm = clean_string(&text);
            it.set("gt", V::from(text));
            it.set("norm_gt", V::from(norm));
            others.push(it);
        } else {
            clean.push(it);
        }
    }
    (clean, others)
}
