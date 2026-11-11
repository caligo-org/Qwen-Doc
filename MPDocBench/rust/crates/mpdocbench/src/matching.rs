//! Port of `utils/match.py` (get_gt_pred_lines, match_gt2pred_simple) and
//! `utils/match_quick.py` (match_gt2pred_quick). The original's quirks are kept on
//! purpose (they decide the published numbers): indices into unfiltered item lists
//! after filtering, `list.remove` while iterating, CPython set iteration order,
//! first-pred-only distances for multi-GT merges, and so on.

use crate::lev::{self, Pattern};
use crate::lsap::linear_sum_assignment;
use crate::preprocess::{clean_string, normalized_formula, textblock2unicode};
use crate::py;
use crate::pyset::IntSet;
use crate::pyval::{Map, V};
use std::collections::HashMap;

const IGNORE_THRES: f64 = 0.1;
const IGNORE_CATS: [&str; 11] = [
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
const TEXT_CATS: [&str; 16] = [
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    TextAll,
    HtmlTable,
    LatexTable,
    /// `None` as passed by match_gt2pred_quick
    Untyped,
}

pub struct Lines {
    pub gt_lines: Vec<String>,
    pub norm_gt: Vec<String>,
    pub gt_cat: Vec<String>,
    pub pred_lines: Vec<String>,
    pub norm_pred: Vec<String>,
}

fn cat_of(item: &V) -> String {
    let f = item.get("fine_category_type");
    if f.truthy() {
        f.py_str()
    } else {
        item.get("category_type").py_str()
    }
}

fn req_str(item: &V, key: &str) -> String {
    if !item.has(key) {
        panic!("KeyError: {:?} in {}", key, item.py_repr().chars().take(300).collect::<String>());
    }
    item.get(key).py_str()
}

/// `get_gt_pred_lines` (returns the filtered lines; the item lists stay unfiltered).
pub fn get_gt_pred_lines(gt_mix: &[V], pred_mix: &[V], line_type: LineType, marathon: bool) -> Lines {
    let mut gt_lines = Vec::new();
    let mut norm_gt = Vec::new();
    let mut gt_cat = Vec::new();
    let mut pred_lines = Vec::new();
    let mut norm_pred = Vec::new();
    if matches!(line_type, LineType::HtmlTable | LineType::LatexTable) {
        let mut norm_html = Vec::new();
        for item in gt_mix {
            gt_cat.push(cat_of(item));
            if item.get("content").truthy() {
                gt_lines.push(item.get("content").py_str());
                norm_html.push(item.get("content").py_str());
            } else if line_type == LineType::HtmlTable {
                gt_lines.push(req_str(item, "html"));
            } else {
                gt_lines.push(req_str(item, "html"));
                norm_html.push(req_str(item, "html"));
            }
        }
        pred_lines = pred_mix.iter().map(|i| req_str(i, "content")).collect();
        norm_gt = gt_lines.clone();
        norm_pred = pred_lines.clone();
        if line_type == LineType::LatexTable {
            gt_lines = norm_html;
        }
    } else {
        for item in pred_mix {
            let content = req_str(item, "content");
            let cat = item.get("category_type").as_str().unwrap_or("");
            let norm = if cat == "text_all" {
                clean_string(&textblock2unicode(&content, marathon))
            } else if cat == "equation_isolated" {
                normalized_formula(&content)
            } else {
                content.clone()
            };
            pred_lines.push(content);
            norm_pred.push(norm);
        }
        for item in gt_mix {
            let cat = item.get("category_type").as_str().unwrap_or("").to_string();
            if item.get("content").truthy() {
                let content = item.get("content").py_str();
                gt_lines.push(content.clone());
                if cat == "text_all" {
                    norm_gt.push(clean_string(&textblock2unicode(&content, marathon)));
                } else {
                    norm_gt.push(content);
                }
                gt_cat.push(cat_of(item));
            } else if TEXT_CATS.contains(&cat.as_str()) {
                let text = req_str(item, "text");
                norm_gt.push(clean_string(&textblock2unicode(&text, marathon)));
                gt_lines.push(text);
                gt_cat.push(cat_of(item));
            } else if cat == "equation_isolated" {
                let latex = req_str(item, "latex");
                norm_gt.push(normalized_formula(&latex));
                gt_lines.push(latex);
                gt_cat.push(cat_of(item));
            }
        }
    }
    // keep (a, b[, c]) where a and b are non-empty (zip truncates to the shortest)
    let n = gt_lines.len().min(norm_gt.len()).min(gt_cat.len());
    let mut l = Lines { gt_lines: vec![], norm_gt: vec![], gt_cat: vec![], pred_lines: vec![], norm_pred: vec![] };
    for i in 0..n {
        if !gt_lines[i].is_empty() && !norm_gt[i].is_empty() {
            l.gt_lines.push(std::mem::take(&mut gt_lines[i]));
            l.norm_gt.push(std::mem::take(&mut norm_gt[i]));
            l.gt_cat.push(std::mem::take(&mut gt_cat[i]));
        }
    }
    let n = pred_lines.len().min(norm_pred.len());
    for i in 0..n {
        if !pred_lines[i].is_empty() && !norm_pred[i].is_empty() {
            l.pred_lines.push(std::mem::take(&mut pred_lines[i]));
            l.norm_pred.push(std::mem::take(&mut norm_pred[i]));
        }
    }
    l
}

fn get_pred_category_type(idx: usize, pred_items: &[V]) -> String {
    cat_of(&pred_items[idx])
}

/// `[x.get('order') if x.get('order') else x.get('position', [""])[0]]` element.
fn gt_position(item: &V) -> V {
    let order = item.get("order");
    if order.truthy() {
        order.clone()
    } else if item.has("position") {
        match item.get("position") {
            V::List(l) => l[0].clone(),
            V::Str(s) => V::from(s.chars().next().unwrap().to_string()),
            other => panic!("TypeError: position {}", other.py_repr()),
        }
    } else {
        V::str("")
    }
}

fn pred_position(item: &V) -> V {
    item.get("position").as_list().expect("pred position")[0].clone()
}

fn attribute(item: &V) -> V {
    if item.has("attribute") {
        item.get("attribute").clone()
    } else {
        V::empty_dict()
    }
}

fn empty_list() -> V {
    V::list(vec![V::str("")])
}

fn idx_list(v: &[usize]) -> V {
    V::list(v.iter().map(|&i| V::from(i)).collect())
}

/// Normalised edit distance as `compute_edit_distance_matrix_new` (0 when both are empty).
fn norm_dist(a: &[u32], b: &[u32]) -> f64 {
    if a.is_empty() && b.is_empty() {
        0.0
    } else {
        lev::distance(a, b) as f64 / a.len().max(b.len()) as f64
    }
}

fn dist_matrix(gt: &[Vec<u32>], pred: &[Vec<u32>]) -> Vec<f64> {
    let mut m = Vec::with_capacity(gt.len() * pred.len());
    for g in gt {
        if g.is_empty() {
            for p in pred {
                m.push(norm_dist(g, p));
            }
            continue;
        }
        let pat = Pattern::new(g);
        for p in pred {
            if p.is_empty() {
                m.push(1.0);
            } else {
                m.push(pat.distance(p) as f64 / g.len().max(p.len()) as f64);
            }
        }
    }
    m
}

fn chars_of(lines: &[String]) -> Vec<Vec<u32>> {
    lines.iter().map(|s| lev::chars(s)).collect()
}

/// `match_gt2pred_simple` → (match list, unmatched-table-pred items or None).
pub fn match_gt2pred_simple(
    gt_items: &[V],
    pred_items: &[V],
    line_type: LineType,
    img_name: &str,
    marathon: bool,
) -> (Vec<V>, Option<Vec<V>>) {
    let l = get_gt_pred_lines(gt_items, pred_items, line_type, marathon);
    let mut out = Vec::new();
    if l.norm_gt.is_empty() && l.norm_pred.is_empty() {
        return (out, None);
    }
    if l.norm_gt.is_empty() {
        let idx: Vec<usize> = (0..l.norm_pred.len()).collect();
        let mut m = Map::new();
        m.insert("gt_idx".into(), empty_list());
        m.insert("gt".into(), V::str(""));
        m.insert("pred_idx".into(), idx_list(&idx));
        m.insert("pred".into(), V::from(l.pred_lines.concat()));
        m.insert("gt_position".into(), empty_list());
        m.insert("pred_position".into(), pred_position(&pred_items[0]));
        m.insert("norm_gt".into(), V::str(""));
        m.insert("norm_pred".into(), V::from(l.norm_pred.concat()));
        m.insert("gt_category_type".into(), V::str(""));
        m.insert("pred_category_type".into(), V::from(get_pred_category_type(0, pred_items)));
        m.insert("gt_attribute".into(), V::list(vec![V::empty_dict()]));
        m.insert("edit".into(), V::Int(1));
        m.insert("img_id".into(), V::str(img_name));
        out.push(V::from(m));
        return (out, None);
    }
    if l.norm_pred.is_empty() {
        for gi in 0..l.norm_gt.len() {
            let mut m = Map::new();
            m.insert("gt_idx".into(), idx_list(&[gi]));
            m.insert("gt".into(), V::from(l.gt_lines[gi].clone()));
            m.insert("pred_idx".into(), empty_list());
            m.insert("pred".into(), V::str(""));
            m.insert("gt_position".into(), V::list(vec![gt_position(&gt_items[gi])]));
            m.insert("pred_position".into(), V::str(""));
            m.insert("norm_gt".into(), V::from(l.norm_gt[gi].clone()));
            m.insert("norm_pred".into(), V::str(""));
            m.insert("gt_category_type".into(), V::from(l.gt_cat[gi].clone()));
            m.insert("pred_category_type".into(), V::str(""));
            m.insert("gt_attribute".into(), V::list(vec![attribute(&gt_items[gi])]));
            m.insert("edit".into(), V::Int(1));
            m.insert("img_id".into(), V::str(img_name));
            out.push(V::from(m));
        }
        return (out, None);
    }
    let gc = chars_of(&l.norm_gt);
    let pc = chars_of(&l.norm_pred);
    let (nr, nc) = (gc.len(), pc.len());
    let cost = dist_matrix(&gc, &pc);
    let (row_ind, col_ind) = linear_sum_assignment(&cost, nr, nc);
    for gi in 0..nr {
        let mut m = Map::new();
        let (pidx, pline, pnorm, edit): (Option<usize>, String, String, V) = match row_ind.iter().position(|&r| r == gi) {
            Some(ri) => {
                let p = col_ind[ri];
                (Some(p), l.pred_lines[p].clone(), l.norm_pred[p].clone(), V::Float(cost[gi * nc + p]))
            }
            None => (None, String::new(), String::new(), V::Int(1)),
        };
        m.insert("gt_idx".into(), idx_list(&[gi]));
        m.insert("gt".into(), V::from(l.gt_lines[gi].clone()));
        m.insert("norm_gt".into(), V::from(l.norm_gt[gi].clone()));
        m.insert("gt_category_type".into(), V::from(l.gt_cat[gi].clone()));
        m.insert("gt_position".into(), V::list(vec![gt_position(&gt_items[gi])]));
        m.insert("gt_attribute".into(), V::list(vec![attribute(&gt_items[gi])]));
        m.insert(
            "pred_idx".into(),
            V::list(vec![match pidx {
                Some(p) => V::from(p),
                None => V::str(""),
            }]),
        );
        m.insert("pred".into(), V::from(pline));
        m.insert("norm_pred".into(), V::from(pnorm));
        // `... if pred_idx else ""`: index 0 is falsy
        let truthy = matches!(pidx, Some(p) if p != 0);
        m.insert(
            "pred_category_type".into(),
            if truthy { V::from(get_pred_category_type(pidx.unwrap(), pred_items)) } else { V::str("") },
        );
        m.insert(
            "pred_position".into(),
            if truthy { pred_position(&pred_items[pidx.unwrap()]) } else { V::str("") },
        );
        m.insert("edit".into(), edit);
        m.insert("img_id".into(), V::str(img_name));
        out.push(V::from(m));
    }
    let unmatched: Vec<usize> = (0..nc).filter(|p| !col_ind.contains(p)).collect();
    if !unmatched.is_empty() {
        if matches!(line_type, LineType::HtmlTable | LineType::LatexTable) {
            let mut extra = Vec::new();
            for &i in &unmatched {
                let original = &pred_items[i];
                let content = original.get("content");
                let html = content.as_str().expect("table pred content");
                for text in pysoup::td_strings(html) {
                    let mut it = original.clone();
                    it.set("content", V::from(text));
                    it.set("category_type", V::str("text_all"));
                    extra.push(it);
                }
            }
            return (out, Some(extra));
        }
        let mut m = Map::new();
        m.insert("gt_idx".into(), empty_list());
        m.insert("gt".into(), V::str(""));
        m.insert("pred_idx".into(), idx_list(&unmatched));
        m.insert("pred".into(), V::from(unmatched.iter().map(|&p| l.pred_lines[p].as_str()).collect::<String>()));
        m.insert("gt_position".into(), empty_list());
        m.insert("pred_position".into(), pred_position(&pred_items[unmatched[0]]));
        m.insert("norm_gt".into(), V::str(""));
        m.insert("norm_pred".into(), V::from(unmatched.iter().map(|&p| l.norm_pred[p].as_str()).collect::<String>()));
        m.insert("gt_category_type".into(), V::str(""));
        m.insert("pred_category_type".into(), V::from(get_pred_category_type(unmatched[0], pred_items)));
        m.insert("gt_attribute".into(), V::list(vec![V::empty_dict()]));
        m.insert("edit".into(), V::Int(1));
        m.insert("img_id".into(), V::str(img_name));
        out.push(V::from(m));
    }
    (out, None)
}

// ------------------------------------------------------------------ quick match

/// Fuzzy window distances, memoised per (gt line, pred line).
struct Fuzzy<'a> {
    gt: &'a [Vec<u32>],
    pred: &'a [Vec<u32>],
    /// sub_pred_fuzzy_matching(gt[g], pred[p]) → None for Python's False
    sub_pred: HashMap<(usize, usize), Option<f64>>,
    pred_patterns: HashMap<usize, Pattern>,
}

impl<'a> Fuzzy<'a> {
    /// min over windows of gt (length len(pred)) of Lev(window, pred) / len(pred).
    fn sub_pred(&mut self, g: usize, p: usize) -> Option<f64> {
        if let Some(v) = self.sub_pred.get(&(g, p)) {
            return *v;
        }
        let gt = &self.gt[g];
        let pr = &self.pred[p];
        let r = if gt.len() >= pr.len() && !pr.is_empty() {
            let pat = self.pred_patterns.entry(p).or_insert_with(|| Pattern::new(pr));
            Some(min_window(gt, pr.len(), pat) as f64 / pr.len() as f64)
        } else {
            None
        };
        self.sub_pred.insert((g, p), r);
        r
    }
}

/// min_i Lev(text[i..i+w], pattern) for all windows of length w.
fn min_window(text: &[u32], w: usize, pat: &Pattern) -> usize {
    let mut best = usize::MAX;
    for i in 0..=(text.len() - w) {
        let d = pat.distance(&text[i..i + w]);
        if d < best {
            best = d;
            if d == 0 {
                break;
            }
        }
    }
    best
}

/// sub_gt_fuzzy_matching(pred, gt).0: min over windows of pred (length len(gt)).
fn sub_gt_fuzzy(pred: &[u32], gt: &[u32], gt_pat: &Pattern) -> f64 {
    if pred.len() >= gt.len() && !gt.is_empty() {
        min_window(pred, gt.len(), gt_pat) as f64 / gt.len() as f64
    } else {
        1.0
    }
}

fn join_chars(parts: &[&Vec<u32>]) -> Vec<u32> {
    let mut out = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            out.push(' ' as u32);
        }
        out.extend_from_slice(p);
    }
    out
}

fn ndist(a: &[u32], b: &[u32]) -> f64 {
    // Levenshtein(a, b) / max(len(a), len(b)); callers guarantee one is non-empty
    lev::distance(a, b) as f64 / a.len().max(b.len()) as f64
}

/// judge_pred_merge([gt], [norm_pred[k] for k in ks]) with threshold 0.6.
fn judge_pred_merge(fz: &mut Fuzzy, g: usize, ks: &[usize]) -> (bool, bool) {
    if ks.len() == 1 {
        return (false, false);
    }
    let gt = &fz.gt[g];
    let cur: Vec<&Vec<u32>> = ks[..ks.len() - 1].iter().map(|&k| &fz.pred[k]).collect();
    let all: Vec<&Vec<u32>> = ks.iter().map(|&k| &fz.pred[k]).collect();
    let cur_pred = join_chars(&cur);
    let merged_pred = join_chars(&all);
    let cur_dist = ndist(gt, &cur_pred);
    let merged_dist = ndist(gt, &merged_pred);
    if merged_dist > cur_dist {
        return (false, false);
    }
    for &k in &ks[..ks.len() - 1] {
        match fz.sub_pred(g, k) {
            None => return (false, false),
            Some(d) if d > 0.6 => return (false, false),
            _ => {}
        }
    }
    let add = match fz.sub_pred(g, *ks.last().unwrap()) {
        None => return (false, false),
        Some(d) => d,
    };
    (add < 0.6, merged_pred.len() <= fz.gt[g].len())
}

/// An entry of `final_pred_idx_list`: a pred index or a merged run of them.
#[derive(Clone, Debug, PartialEq)]
enum PredIdx {
    One(usize),
    Run(Vec<usize>),
}

/// deal_with_truncated + linear_sum_assignment (cal_final_match).
fn cal_final_match(
    cost: Vec<f64>,
    gt: &[Vec<u32>],
    pred: &[Vec<u32>],
) -> (Vec<PredIdx>, Vec<usize>, Vec<f64>) {
    let (nr, nc) = (gt.len(), pred.len());
    let mut masked_gt = vec![false; nr];
    let mut masked_pred = vec![false; nc];
    for i in 0..nr {
        for j in 0..nc {
            if cost[i * nc + j] < IGNORE_THRES {
                masked_gt[i] = true;
                masked_pred[j] = true;
            }
        }
    }
    let unmasked_gt: Vec<usize> = (0..nr).filter(|&i| !masked_gt[i]).collect();
    let unmasked_pred: Vec<usize> = (0..nc).filter(|&j| !masked_pred[j]).collect();
    let mut fz = Fuzzy { gt, pred, sub_pred: HashMap::new(), pred_patterns: HashMap::new() };
    let mut subset_certain: Vec<Vec<usize>> = Vec::new();
    let mut subset_cost: Vec<f64> = Vec::new();
    for &g in &unmasked_gt {
        let mut best: Option<(f64, Vec<usize>)> = None;
        let gpat = if unmasked_pred.is_empty() { None } else { Some(Pattern::new(&gt[g])) };
        for &p in &unmasked_pred {
            let mut step = 1;
            let mut ks = vec![p];
            loop {
                if p + step >= nc || masked_pred[p + step] {
                    break;
                }
                ks.push(p + step);
                let (flag, cont) = judge_pred_merge(&mut fz, g, &ks);
                if !flag {
                    break;
                }
                step += 1;
                if !cont {
                    break;
                }
            }
            let run: Vec<usize> = (p..p + step).collect();
            let line = join_chars(&run.iter().map(|&k| &pred[k]).collect::<Vec<_>>());
            let d = if gt[g].is_empty() && line.is_empty() {
                f64::NAN // ZeroDivisionError in Python; cannot happen (lines are non-empty)
            } else if line.len() >= gt[g].len() || true {
                gpat.as_ref().unwrap().distance(&line) as f64 / line.len().max(gt[g].len()) as f64
            } else {
                unreachable!()
            };
            // min() keeps the first minimum, like list.index(min(...))
            if best.as_ref().map_or(true, |(b, _)| d < *b) {
                best = Some((d, run));
            }
        }
        if let Some((c, run)) = best {
            if !run.is_empty() {
                subset_certain.push(run);
                subset_cost.push(c);
            }
        }
    }
    let final_subset = get_final_subset(&subset_certain, &subset_cost);
    let (cost, list): (Vec<f64>, Vec<PredIdx>) = if final_subset.is_empty() {
        (cost, (0..nc).map(PredIdx::One).collect())
    } else {
        let list = merge_lists_with_sublists(nc, &final_subset);
        let lines: Vec<Vec<u32>> = list
            .iter()
            .map(|x| match x {
                PredIdx::One(k) => pred[*k].clone(),
                PredIdx::Run(r) => join_chars(&(r[0]..=*r.last().unwrap()).map(|k| &pred[k]).collect::<Vec<_>>()),
            })
            .collect();
        (dist_matrix(gt, &lines), list)
    };
    let ncols = list.len();
    let (row_ind, col_ind) = linear_sum_assignment(&cost, nr, ncols);
    let cost_list: Vec<f64> = row_ind.iter().zip(col_ind.iter()).map(|(&r, &c)| cost[r * ncols + c]).collect();
    let matched: Vec<PredIdx> = col_ind.iter().map(|&c| list[c].clone()).collect();
    (matched, row_ind, cost_list)
}

fn merge_lists_with_sublists(n: usize, subs: &[Vec<usize>]) -> Vec<PredIdx> {
    let mut main: Vec<PredIdx> = (0..n).map(PredIdx::One).collect();
    for sub in subs {
        let pop_idx = main
            .iter()
            .position(|x| *x == PredIdx::One(sub[0]))
            .unwrap_or_else(|| panic!("ValueError: {} is not in list", sub[0]));
        for _ in sub {
            main.remove(pop_idx);
        }
        main.insert(pop_idx, PredIdx::Run(sub.clone()));
    }
    main
}

fn get_final_subset(subsets: &[Vec<usize>], costs: &[f64]) -> Vec<Vec<usize>> {
    if subsets.is_empty() || costs.is_empty() {
        return Vec::new();
    }
    let mut tuples: Vec<(Vec<usize>, f64)> = subsets.iter().cloned().zip(costs.iter().cloned()).collect();
    tuples.sort_by_key(|t| t.0[0]); // stable
    let mut groups: Vec<Vec<(Vec<usize>, f64)>> = vec![vec![tuples[0].clone()]];
    for item in tuples[1..].iter() {
        let last = groups.last().unwrap();
        let overlap = last.iter().any(|s| item.0.iter().any(|i| s.0.contains(i)));
        if overlap {
            groups.last_mut().unwrap().push(item.clone());
        } else {
            groups.push(vec![item.clone()]);
        }
    }
    let mut out = Vec::new();
    for group in groups {
        if group.len() == 1 {
            out.push(group[0].0.clone());
            continue;
        }
        // path_dict: insertion-ordered dict of paths
        let mut paths: Vec<Vec<(Vec<usize>, f64)>> = vec![vec![group[0].clone()]];
        for subset in group[1..].iter() {
            let mut new_path = true;
            for pi in 0..paths.len() {
                let mut is_dup = false;
                let mut is_same = false;
                // iterate over a snapshot of the path items, as the original mutates while iterating
                let mut idx = 0;
                while idx < paths[pi].len() {
                    let path_item = paths[pi][idx].clone();
                    if path_item.0 == subset.0 {
                        is_dup = true;
                        is_same = true;
                        if path_item.1 > subset.1 {
                            let pos = paths[pi].iter().position(|x| x.0 == path_item.0 && x.1 == path_item.1).unwrap();
                            paths[pi].remove(pos);
                            paths[pi].push(subset.clone());
                            // Python's for-loop over the mutated list: the element now at
                            // `idx` (shifted down) is skipped by the iterator advance below
                        }
                    } else if path_item.0.iter().any(|a| subset.0.contains(a)) {
                        is_dup = true;
                    }
                    idx += 1;
                }
                if !is_dup {
                    paths[pi].push(subset.clone());
                    new_path = false;
                }
                if is_same {
                    new_path = false;
                }
            }
            if new_path {
                paths.push(vec![subset.clone()]);
            }
        }
        let mut saved_cost = f64::INFINITY;
        let mut saved: Vec<Vec<usize>> = Vec::new();
        for path in &paths {
            let avg = path.iter().map(|x| x.1).sum::<f64>() / path.len() as f64;
            if avg < saved_cost {
                saved = path.iter().map(|x| x.0.clone()).collect();
                saved_cost = avg;
            }
        }
        out.extend(saved);
    }
    out
}

/// final_matches entry: (pred_key, gt_indices, edit_distance)
struct FinalMatch {
    pred_key: Vec<usize>,
    gt_indices: Vec<usize>,
    edit: V,
}

/// merge_matches(matches, matching_dict) with insertion-ordered dicts.
fn merge_matches(matches: &[(usize, Vec<usize>, V)], matching: &[(usize, Vec<usize>)]) -> Vec<FinalMatch> {
    let mut fin: Vec<FinalMatch> = Vec::new();
    let mut processed: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for (g, preds, edit) in matches {
        let mut key = preds.clone();
        key.sort();
        if let Some(f) = fin.iter_mut().find(|f| f.pred_key == key) {
            if !processed.contains(g) {
                f.gt_indices.push(*g);
                processed.insert(*g);
            }
        } else {
            fin.push(FinalMatch { pred_key: key, gt_indices: vec![*g], edit: edit.clone() });
            processed.insert(*g);
        }
    }
    for (p, gts) in matching {
        let key = vec![*p];
        if let Some(f) = fin.iter_mut().find(|f| f.pred_key == key) {
            for g in gts {
                if !processed.contains(g) {
                    f.gt_indices.push(*g);
                    processed.insert(*g);
                }
            }
        } else {
            let gi: Vec<usize> = gts.iter().cloned().filter(|g| !processed.contains(g)).collect();
            for g in &gi {
                processed.insert(*g);
            }
            fin.push(FinalMatch { pred_key: key, gt_indices: gi, edit: V::Null });
        }
    }
    fin
}

fn sorted_unique(v: &[usize]) -> Vec<usize> {
    let mut s = v.to_vec();
    s.sort();
    s.dedup();
    s
}

fn recalculate_edit_distances(fin: &mut [FinalMatch], gt: &[Vec<u32>], pred: &[Vec<u32>]) {
    for f in fin.iter_mut() {
        let gis = sorted_unique(&f.gt_indices);
        if gis.is_empty() {
            f.edit = V::Int(1);
            continue;
        }
        let (a, b): (Vec<u32>, Vec<u32>) = if gis.len() > 1 {
            let merged: Vec<u32> = gis.iter().flat_map(|&g| gt[g].iter().cloned()).collect();
            // only the first pred of the key (the original's isinstance check passes for ints)
            (merged, pred[f.pred_key[0]].clone())
        } else {
            (gt[gis[0]].clone(), join_chars(&f.pred_key.iter().map(|&k| &pred[k]).collect::<Vec<_>>()))
        };
        f.edit = if a.is_empty() && b.is_empty() { V::Int(1) } else { V::Float(ndist(&a, &b)) };
    }
}

/// converted result entry: gt_idx is None for Python's "" ; pred_idx None for ""
struct Converted {
    gt_idx: Option<usize>,
    gt: String,
    pred_idx: Option<Vec<usize>>,
    pred: String,
    edit: V,
}

fn convert_final_matches(fin: &[FinalMatch], gt_norm: &[String], pred_norm: &[String], gtc: &[Vec<u32>], pc: &[Vec<u32>]) -> Vec<Converted> {
    let mut out = Vec::new();
    let all_gt = IntSet::from_range(gt_norm.len());
    let all_pred = IntSet::from_range(pred_norm.len());
    for f in fin {
        let pred_content = f.pred_key.iter().map(|&k| pred_norm[k].as_str()).collect::<Vec<_>>().join(" ");
        for g in sorted_unique(&f.gt_indices) {
            out.push(Converted {
                gt_idx: Some(g),
                gt: gt_norm[g].clone(),
                pred_idx: Some(f.pred_key.clone()),
                pred: pred_content.clone(),
                edit: f.edit.clone(),
            });
        }
    }
    let matched_gt = IntSet::from_iter(fin.iter().flat_map(|f| f.gt_indices.iter().map(|&g| g as i64)));
    let unmatched_gt = all_gt.difference(&matched_gt);
    let matched_pred = IntSet::from_iter(fin.iter().flat_map(|f| f.pred_key.iter().map(|&k| k as i64)));
    let unmatched_pred = all_pred.difference(&matched_pred);
    if unmatched_pred.len() > 0 {
        if unmatched_gt.len() > 0 {
            let ug: Vec<usize> = unmatched_gt.iter().map(|x| x as usize).collect();
            let up: Vec<usize> = unmatched_pred.iter().map(|x| x as usize).collect();
            let mut m = Vec::with_capacity(ug.len() * up.len());
            for &g in &ug {
                for &p in &up {
                    m.push(ndist(&gtc[g], &pc[p]));
                }
            }
            let (ri, ci) = linear_sum_assignment(&m, ug.len(), up.len());
            for (i, j) in ri.into_iter().zip(ci) {
                out.push(Converted {
                    gt_idx: Some(ug[i]),
                    gt: gt_norm[ug[i]].clone(),
                    pred_idx: Some(vec![up[j]]),
                    pred: pred_norm[up[j]].clone(),
                    edit: V::Int(1),
                });
            }
        } else {
            let up: Vec<usize> = unmatched_pred.iter().map(|x| x as usize).collect();
            out.push(Converted {
                gt_idx: None,
                gt: String::new(),
                pred: up.iter().map(|&p| pred_norm[p].as_str()).collect::<Vec<_>>().join(" "),
                pred_idx: Some(up),
                edit: V::Int(1),
            });
        }
    } else {
        for g in unmatched_gt.iter() {
            let g = g as usize;
            out.push(Converted { gt_idx: Some(g), gt: gt_norm[g].clone(), pred_idx: None, pred: String::new(), edit: V::Int(1) });
        }
    }
    out
}

/// A merged result: gt_idx list (None = ""), pred_idx list (None = "").
struct Merged {
    gt_idx: Vec<Option<usize>>,
    pred_idx: Vec<Option<usize>>,
    edit: V,
    /// the `merged_entry['gt']` / `['pred']` strings (overwritten later unless the entry is skipped)
    gt: String,
    pred: String,
}

fn merge_duplicates_add_unmatched(conv: &[Converted], n_gt: usize, gt_ori: &[String]) -> Vec<Merged> {
    let mut out = Vec::new();
    let mut processed_pred: Vec<Vec<Option<usize>>> = Vec::new();
    let mut processed_gt: Vec<Option<usize>> = Vec::new();
    let key = |c: &Converted| -> Vec<Option<usize>> {
        match &c.pred_idx {
            Some(v) => v.iter().map(|&x| Some(x)).collect(),
            None => vec![None],
        }
    };
    for (i, e) in conv.iter().enumerate() {
        let k = key(e);
        if !processed_pred.contains(&k) && k != vec![None] {
            let mut m = Merged {
                gt_idx: vec![e.gt_idx],
                pred_idx: k.clone(),
                edit: e.edit.clone(),
                gt: e.gt.clone(),
                pred: e.pred.clone(),
            };
            for (j, o) in conv.iter().enumerate() {
                if j != i && key(o) == k {
                    m.gt_idx.push(o.gt_idx);
                    m.gt.push_str(&o.gt);
                    processed_gt.push(o.gt_idx);
                }
            }
            out.push(m);
            processed_pred.push(k);
            processed_gt.push(e.gt_idx);
        }
    }
    for g in 0..n_gt {
        if !processed_gt.contains(&Some(g)) {
            out.push(Merged { gt_idx: vec![Some(g)], pred_idx: vec![None], edit: V::Int(1), gt: gt_ori[g].clone(), pred: String::new() });
        }
    }
    out
}

/// Counter(xs).most_common(1)[0][0]: most frequent, first-seen on ties.
fn most_common(xs: &[&str]) -> String {
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for x in xs {
        match counts.iter_mut().find(|(k, _)| k == x) {
            Some(c) => c.1 += 1,
            None => counts.push((x, 1)),
        }
    }
    let mut best = counts[0];
    for c in &counts[1..] {
        if c.1 > best.1 {
            best = *c;
        }
    }
    best.0.to_string()
}

fn opt_list(v: &[Option<usize>]) -> V {
    V::list(v.iter().map(|x| x.map_or(V::str(""), V::from)).collect())
}

fn is_empty_marker(v: &[Option<usize>]) -> bool {
    v.len() == 1 && v[0].is_none()
}

/// One sub-problem's result lists (the "ignore" categories or the rest).
struct Side<'a> {
    gt_norm: &'a [String],
    gt_ori: &'a [String],
    gt_cat: &'a [String],
    gt_items: &'a [V],
    pred_norm: &'a [String],
    pred_ori: &'a [String],
    pred_items: &'a [V],
    /// map from local indices to the caller's indices
    gt_map: &'a [usize],
    pred_map: &'a [usize],
}

fn formula_join(lines: &[String], idx: &[Option<usize>]) -> String {
    let parts: Vec<String> = idx
        .iter()
        .map(|i| format!("{{{}}}", py::strip_chars(py::strip_chars(&lines[i.unwrap()], "$"), "\n")))
        .collect();
    format!("\\begin{{array}}{{l}} {} \\end{{array}}", parts.join(" \\\\ "))
}

fn concat(lines: &[String], idx: &[Option<usize>]) -> String {
    idx.iter().map(|i| lines[i.unwrap()].as_str()).collect()
}

/// The per-entry post-processing loops of match_gt2pred_quick. `ignore_side` selects the
/// field order/logic of the ignore-category loop (which differs slightly from the main loop).
fn finish_entries(merged: Vec<Merged>, s: &Side, img_name: &str, ignore_side: bool) -> Vec<V> {
    // start from the dicts merge_duplicates_add_unmatched builds
    let mut list: Vec<(V, Vec<Option<usize>>, Vec<Option<usize>>)> = merged
        .into_iter()
        .map(|m| {
            let mut d = Map::new();
            d.insert("gt_idx".into(), opt_list(&m.gt_idx));
            d.insert("gt".into(), V::from(m.gt));
            d.insert("pred_idx".into(), opt_list(&m.pred_idx));
            d.insert("pred".into(), V::from(m.pred));
            d.insert("edit".into(), m.edit);
            (V::from(d), m.gt_idx, m.pred_idx)
        })
        .collect();

    let ignore_types = IGNORE_CATS;
    let gt_cat_of = |gi: &[Option<usize>]| -> String {
        let clean: Vec<&str> =
            gi.iter().map(|i| s.gt_cat[i.unwrap()].as_str()).filter(|c| !ignore_types.contains(c)).collect();
        if !clean.is_empty() {
            most_common(&clean)
        } else {
            let all: Vec<&str> = gi.iter().map(|i| s.gt_cat[i.unwrap()].as_str()).collect();
            most_common(&all)
        }
    };
    let gt_positions = |gi: &[Option<usize>]| -> V {
        if is_empty_marker(gi) {
            empty_list()
        } else {
            V::list(gi.iter().map(|i| gt_position(&s.gt_items[i.unwrap()])).collect())
        }
    };
    let gt_attrs = |gi: &[Option<usize>]| -> V {
        if is_empty_marker(gi) {
            V::list(vec![V::empty_dict()])
        } else {
            V::list(gi.iter().map(|i| attribute(&s.gt_items[i.unwrap()])).collect())
        }
    };
    let pcat = |pi: &[Option<usize>]| -> String {
        if is_empty_marker(pi) {
            String::new()
        } else {
            get_pred_category_type(pi[0].unwrap(), s.pred_items)
        }
    };

    // Python `for entry in list:` with `list.remove(entry)` inside: the element after a
    // removed one is skipped (it keeps the raw fields above).
    let mut k = 0;
    while k < list.len() {
        let (mut e, gi, pi) = (list[k].0.clone(), list[k].1.clone(), list[k].2.clone());
        let mut removed = false;
        if ignore_side {
            e.set("gt_position", gt_positions(&gi));
            e.set(
                "pred_position",
                if is_empty_marker(&pi) { V::str("") } else { pred_position(&s.pred_items[pi[0].unwrap()]) },
            );
            e.set("gt", V::from(if is_empty_marker(&gi) { String::new() } else { concat(s.gt_ori, &gi) }));
            e.set("pred", V::from(if is_empty_marker(&pi) { String::new() } else { concat(s.pred_ori, &pi) }));
            e.set("norm_gt", V::from(if is_empty_marker(&gi) { String::new() } else { concat(s.gt_norm, &gi) }));
            e.set("norm_pred", V::from(if is_empty_marker(&pi) { String::new() } else { concat(s.pred_norm, &pi) }));
            if !is_empty_marker(&gi) {
                e.set("gt_category_type", V::from(gt_cat_of(&gi)));
            } else {
                e.set("gt_category_type", V::str(""));
                let pc = pcat(&pi);
                e.set("pred_category_type", V::from(pc.clone()));
                if pc == "equation_inline" {
                    removed = true;
                }
            }
            e.set("pred_category_type", V::from(pcat(&pi)));
            e.set("gt_attribute", gt_attrs(&gi));
            e.set("img_id", V::str(img_name));
        } else {
            if !is_empty_marker(&gi) {
                e.set("gt_category_type", V::from(gt_cat_of(&gi)));
            } else {
                e.set("gt_category_type", V::str(""));
                let pc = pcat(&pi);
                e.set("pred_category_type", V::from(pc.clone()));
                if pc == "equation_inline" {
                    removed = true;
                }
            }
            e.set("gt_position", gt_positions(&gi));
            e.set(
                "pred_position",
                if is_empty_marker(&pi) { V::str("") } else { pred_position(&s.pred_items[pi[0].unwrap()]) },
            );
            let gcat = e.get("gt_category_type").py_str();
            if gcat == "equation_isolated" && gi.len() > 1 {
                e.set("gt", V::from(formula_join(s.gt_ori, &gi)));
            } else {
                e.set("gt", V::from(if is_empty_marker(&gi) { String::new() } else { concat(s.gt_ori, &gi) }));
            }
            let pc = pcat(&pi);
            e.set("pred_category_type", V::from(pc.clone()));
            e.set("gt_attribute", gt_attrs(&gi));
            e.set("img_id", V::str(img_name));
            if pc.contains("equation") && pi.len() > 1 {
                e.set("pred", V::from(formula_join(s.pred_ori, &pi)));
            } else {
                e.set("pred", V::from(if is_empty_marker(&pi) { String::new() } else { concat(s.pred_ori, &pi) }));
            }
            e.set("norm_gt", V::from(if is_empty_marker(&gi) { String::new() } else { concat(s.gt_norm, &gi) }));
            e.set("norm_pred", V::from(if is_empty_marker(&pi) { String::new() } else { concat(s.pred_norm, &pi) }));
        }
        if removed {
            list.remove(k);
            k += 1; // the iterator advances past the element that moved into slot k
        } else {
            list[k].0 = e;
            k += 1;
        }
    }
    // remap local indices to the caller's
    list.into_iter()
        .map(|(mut e, gi, pi)| {
            if !is_empty_marker(&gi) {
                e.set("gt_idx", V::list(gi.iter().map(|i| V::from(s.gt_map[i.unwrap()])).collect()));
            }
            if !is_empty_marker(&pi) {
                e.set("pred_idx", V::list(pi.iter().map(|i| V::from(s.pred_map[i.unwrap()])).collect()));
            }
            e
        })
        .collect()
}

fn base_entry(img_name: &str) -> Map {
    let _ = img_name;
    Map::new()
}

/// `match_gt2pred_quick(gt_items, pred_items, 'text_all', img_name)`.
pub fn match_gt2pred_quick(gt_items: &[V], pred_items_in: &[V], img_name: &str, marathon: bool) -> Vec<V> {
    // sort_by_position_skip_inline
    let mut order: Vec<usize> = (0..pred_items_in.len()).collect();
    order.sort_by_key(|&i| {
        let it = &pred_items_in[i];
        let inline = it.get("fine_category_type").as_str() == Some("equation_inline");
        (inline, pred_position(it).as_i64().unwrap(), i)
    });
    let pred_items: Vec<V> = order.iter().map(|&i| pred_items_in[i].clone()).collect();
    let l = get_gt_pred_lines(gt_items, &pred_items, LineType::Untyped, marathon);
    let (ng, np) = (l.norm_gt.len(), l.norm_pred.len());

    if ng == 0 {
        return (0..np)
            .map(|p| {
                let mut m = base_entry(img_name);
                m.insert("gt_idx".into(), empty_list());
                m.insert("gt".into(), V::str(""));
                m.insert("pred_idx".into(), idx_list(&[p]));
                m.insert("pred".into(), V::from(l.pred_lines[p].clone()));
                m.insert("gt_position".into(), empty_list());
                m.insert("pred_position".into(), pred_position(&pred_items[p]));
                m.insert("norm_gt".into(), V::str(""));
                m.insert("norm_pred".into(), V::from(l.norm_pred[p].clone()));
                m.insert("gt_category_type".into(), V::str(""));
                m.insert("pred_category_type".into(), V::from(get_pred_category_type(p, &pred_items)));
                m.insert("gt_attribute".into(), V::list(vec![V::empty_dict()]));
                m.insert("edit".into(), V::Int(1));
                m.insert("img_id".into(), V::str(img_name));
                V::from(m)
            })
            .collect();
    }
    if np == 0 {
        return (0..ng)
            .map(|g| {
                let mut m = Map::new();
                m.insert("gt_idx".into(), idx_list(&[g]));
                m.insert("gt".into(), V::from(l.gt_lines[g].clone()));
                m.insert("pred_idx".into(), empty_list());
                m.insert("pred".into(), V::str(""));
                m.insert("gt_position".into(), V::list(vec![gt_position(&gt_items[g])]));
                m.insert("pred_position".into(), V::str(""));
                m.insert("norm_gt".into(), V::from(l.norm_gt[g].clone()));
                m.insert("norm_pred".into(), V::str(""));
                m.insert("gt_category_type".into(), V::from(l.gt_cat[g].clone()));
                m.insert("pred_category_type".into(), V::str(""));
                m.insert("gt_attribute".into(), V::list(vec![attribute(&gt_items[g])]));
                m.insert("edit".into(), V::Int(1));
                m.insert("img_id".into(), V::str(img_name));
                V::from(m)
            })
            .collect();
    }
    if ng == 1 && np == 1 {
        let d = lev::str_distance(&l.norm_gt[0], &l.norm_pred[0]) as f64
            / py::len(&l.norm_gt[0]).max(py::len(&l.norm_pred[0])) as f64;
        let mut m = Map::new();
        m.insert("gt_idx".into(), idx_list(&[0]));
        m.insert("gt".into(), V::from(l.gt_lines[0].clone()));
        m.insert("pred_idx".into(), idx_list(&[0]));
        m.insert("pred".into(), V::from(l.pred_lines[0].clone()));
        m.insert("gt_position".into(), V::list(vec![gt_position(&gt_items[0])]));
        m.insert("pred_position".into(), pred_position(&pred_items[0]));
        m.insert("norm_gt".into(), V::from(l.norm_gt[0].clone()));
        m.insert("norm_pred".into(), V::from(l.norm_pred[0].clone()));
        m.insert("gt_category_type".into(), V::from(l.gt_cat[0].clone()));
        m.insert("pred_category_type".into(), V::from(get_pred_category_type(0, &pred_items)));
        m.insert("gt_attribute".into(), V::list(vec![attribute(&gt_items[0])]));
        m.insert("edit".into(), V::Float(d));
        m.insert("img_id".into(), V::str(img_name));
        return vec![V::from(m)];
    }

    let gtc = chars_of(&l.norm_gt);
    let pc = chars_of(&l.norm_pred);

    // split GT into ignore categories and the rest
    let mut ig: Vec<usize> = Vec::new();
    let mut no: Vec<usize> = Vec::new();
    for i in 0..ng {
        if IGNORE_CATS.contains(&l.gt_cat[i].as_str()) {
            ig.push(i);
        } else {
            no.push(i);
        }
    }

    let mut ignore_pred_idxs: Vec<usize> = Vec::new();
    let mut merged_ignore: Vec<V> = Vec::new();
    if !ig.is_empty() {
        let ig_gtc: Vec<Vec<u32>> = ig.iter().map(|&i| gtc[i].clone()).collect();
        let m = dist_matrix(&ig_gtc, &pc);
        // np.argwhere(m < 0.1): row-major
        let mut matches: Vec<(usize, usize)> = Vec::new();
        for i in 0..ig.len() {
            for j in 0..np {
                if m[i * np + j] < IGNORE_THRES {
                    matches.push((i, j));
                }
            }
        }
        ignore_pred_idxs = matches.iter().map(|x| x.1).collect();
        let ig_pred_norm: Vec<String> = ignore_pred_idxs.iter().map(|&j| l.norm_pred[j].clone()).collect();
        let ig_pred_ori: Vec<String> = ignore_pred_idxs.iter().map(|&j| l.pred_lines[j].clone()).collect();
        let ig_pred_items: Vec<V> = ignore_pred_idxs.iter().map(|&j| pred_items[j].clone()).collect();
        let ig_pc: Vec<Vec<u32>> = ignore_pred_idxs.iter().map(|&j| pc[j].clone()).collect();
        // ignore_matches_dict: gt row -> (pred_indices=[idx of its last match], edit)
        let mut dict: Vec<(usize, Vec<usize>, V)> = Vec::new();
        for (idx, &(i, j)) in matches.iter().enumerate() {
            let e = V::Float(m[i * np + j]);
            match dict.iter_mut().find(|x| x.0 == i) {
                Some(x) => {
                    x.1 = vec![idx];
                    x.2 = e;
                }
                None => dict.push((i, vec![idx], e)),
            }
        }
        let ig_gt_norm: Vec<String> = ig.iter().map(|&i| l.norm_gt[i].clone()).collect();
        let ig_gt_ori: Vec<String> = ig.iter().map(|&i| l.gt_lines[i].clone()).collect();
        let ig_gt_cat: Vec<String> = ig.iter().map(|&i| l.gt_cat[i].clone()).collect();
        let ig_gt_items: Vec<V> = ig.iter().map(|&i| gt_items[i].clone()).collect();
        let mut fin = merge_matches(&dict, &[]);
        recalculate_edit_distances(&mut fin, &ig_gtc, &ig_pc);
        let conv = convert_final_matches(&fin, &ig_gt_norm, &ig_pred_norm, &ig_gtc, &ig_pc);
        let merged = merge_duplicates_add_unmatched(&conv, ig_gt_norm.len(), &ig_gt_ori);
        let side = Side {
            gt_norm: &ig_gt_norm,
            gt_ori: &ig_gt_ori,
            gt_cat: &ig_gt_cat,
            gt_items: &ig_gt_items,
            pred_norm: &ig_pred_norm,
            pred_ori: &ig_pred_ori,
            pred_items: &ig_pred_items,
            gt_map: &ig,
            pred_map: &ignore_pred_idxs,
        };
        merged_ignore = finish_entries(merged, &side, img_name, true);
    }

    let no_pred: Vec<usize> = (0..np).filter(|j| !ignore_pred_idxs.contains(j)).collect();
    let no_gt_norm: Vec<String> = no.iter().map(|&i| l.norm_gt[i].clone()).collect();
    let no_gt_ori: Vec<String> = no.iter().map(|&i| l.gt_lines[i].clone()).collect();
    let no_gt_cat: Vec<String> = no.iter().map(|&i| l.gt_cat[i].clone()).collect();
    let no_gt_items: Vec<V> = no.iter().map(|&i| gt_items[i].clone()).collect();
    let no_pred_norm: Vec<String> = no_pred.iter().map(|&j| l.norm_pred[j].clone()).collect();
    let no_pred_ori: Vec<String> = no_pred.iter().map(|&j| l.pred_lines[j].clone()).collect();
    let no_pred_items: Vec<V> = no_pred.iter().map(|&j| pred_items[j].clone()).collect();
    let no_gtc: Vec<Vec<u32>> = no.iter().map(|&i| gtc[i].clone()).collect();
    let no_pc: Vec<Vec<u32>> = no_pred.iter().map(|&j| pc[j].clone()).collect();

    let cost = dist_matrix(&no_gtc, &no_pc);
    let (matched_col, row_ind, cost_list) = cal_final_match(cost, &no_gtc, &no_pc);

    // process_matches
    let mut matches: Vec<(usize, Vec<usize>, V)> = Vec::new();
    let mut unmatched_gt: Vec<usize> = Vec::new();
    let mut unmatched_pred: Vec<usize> = Vec::new();
    for i in 0..no_gtc.len() {
        match row_ind.iter().position(|&r| r == i) {
            Some(idx) => {
                let range: Vec<usize> = match &matched_col[idx] {
                    PredIdx::One(p) => vec![*p],
                    PredIdx::Run(r) => (r[0]..=*r.last().unwrap()).collect(),
                };
                let edit = cost_list[idx];
                if edit > 0.7 {
                    unmatched_pred.extend(range);
                    unmatched_gt.push(i);
                } else {
                    for p in &range {
                        if let Some(pos) = unmatched_pred.iter().position(|x| x == p) {
                            unmatched_pred.remove(pos);
                        }
                    }
                    matches.push((i, range, V::Float(edit)));
                }
            }
            None => unmatched_gt.push(i),
        }
    }
    let _ = unmatched_pred;

    // fuzzy_match_unmatched_items
    let mut matching: Vec<(usize, Vec<usize>)> = Vec::new();
    if !unmatched_gt.is_empty() {
        let gpats: Vec<Pattern> = unmatched_gt.iter().map(|&g| Pattern::new(&no_gtc[g])).collect();
        for (p, pcont) in no_pc.iter().enumerate() {
            let mut idx = Vec::new();
            for (k, &g) in unmatched_gt.iter().enumerate() {
                if sub_gt_fuzzy(pcont, &no_gtc[g], &gpats[k]) < 0.4 {
                    idx.push(g);
                }
            }
            if !idx.is_empty() {
                matching.push((p, idx));
            }
        }
    }

    let mut fin = merge_matches(&matches, &matching);
    recalculate_edit_distances(&mut fin, &no_gtc, &no_pc);
    let conv = convert_final_matches(&fin, &no_gt_norm, &no_pred_norm, &no_gtc, &no_pc);
    let merged = merge_duplicates_add_unmatched(&conv, no_gt_norm.len(), &no_gt_ori);
    let side = Side {
        gt_norm: &no_gt_norm,
        gt_ori: &no_gt_ori,
        gt_cat: &no_gt_cat,
        gt_items: &no_gt_items,
        pred_norm: &no_pred_norm,
        pred_ori: &no_pred_ori,
        pred_items: &no_pred_items,
        gt_map: &no,
        pred_map: &no_pred,
    };
    let mut results = finish_entries(merged, &side, img_name, false);
    results.extend(merged_ignore);
    results
}
