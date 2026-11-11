//! Port of the metrics in `metrics/cal_metric.py` (Edit_dist, TEDS, HeadTEDS, FigureF1;
//! CDM runs in Python, see `cdm.rs`) and of `metrics/show_result.py` aggregation.

use crate::lev;
use crate::pdnum::{pd_group_mean, pd_mean, pd_sum, py_mean};
use crate::pyval::{Map, V};
use rayon::prelude::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::sync::LazyLock;

/// `sample['img_id'] if it ends with .jpg/.png/.pdf else '_'.join(img_id.split('_')[:-1])`
pub fn image_name(img_id: &str) -> String {
    if img_id.ends_with(".jpg") || img_id.ends_with(".png") || img_id.ends_with(".pdf") {
        img_id.to_string()
    } else {
        let parts: Vec<&str> = img_id.split('_').collect();
        parts[..parts.len() - 1].join("_")
    }
}

/// `sample['norm_gt'] if sample.get('norm_gt') else sample['gt']` (same for pred)
fn gt_of(s: &V) -> &V {
    if s.get("norm_gt").truthy() {
        s.get("norm_gt")
    } else {
        s.get("gt")
    }
}
fn pred_of(s: &V) -> &V {
    if s.get("norm_pred").truthy() {
        s.get("norm_pred")
    } else {
        s.get("pred")
    }
}

/// Levenshtein.distance on two str or two lists (elements compared with ==).
fn py_lev(a: &V, b: &V) -> usize {
    match (a, b) {
        (V::Str(x), V::Str(y)) => lev::str_distance(x, y),
        _ => {
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
            let seq = |v: &V, f: &mut dyn FnMut(&V) -> u32| -> Vec<u32> {
                match v {
                    V::List(l) => l.iter().map(|x| f(x)).collect(),
                    V::Str(s) => s.chars().map(|c| f(&V::from(c.to_string()))).collect(),
                    _ => panic!("Levenshtein on {}", v.py_repr()),
                }
            };
            let xa = seq(a, &mut sym);
            let xb = seq(b, &mut sym);
            lev::distance(&xa, &xb)
        }
    }
}

fn set_metric(s: &mut V, name: &str, value: V) {
    if !s.get("metric").truthy() {
        s.set("metric", V::empty_dict());
    }
    let d = s.dict_mut();
    d.get_mut("metric").unwrap().set(name, value);
}

pub fn write_json(path: &std::path::Path, v: &V) {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).unwrap();
    }
    std::fs::write(path, crate::pyval::to_json_indent4(v)).unwrap_or_else(|e| panic!("write {}: {}", path.display(), e));
}

fn dict(entries: Vec<(&str, V)>) -> V {
    let mut m = Map::new();
    for (k, v) in entries {
        m.insert(k.to_string(), v);
    }
    V::from(m)
}

/// A float result value: Python writes NaN for empty pandas reductions.
fn fv(x: f64) -> V {
    V::Float(x)
}

/// `call_Edit_dist.evaluate`. Writes `{prefix}_per_page_edit.json`.
pub fn edit_dist(samples: &mut [V], prefix: &str) -> V {
    let mut rows: Vec<(String, Option<f64>, f64)> = Vec::with_capacity(samples.len()); // (image_name, Edit_num, upper_len)
    let results: Vec<(String, usize, Option<usize>)> = samples
        .par_iter()
        .map(|s| {
            let img = image_name(&s.get("img_id").py_str());
            let gt = gt_of(s);
            let pred = pred_of(s);
            let upper = gt.py_len().max(pred.py_len());
            let ed = if pred.py_len() > 0 || gt.py_len() > 0 { Some(py_lev(pred, gt)) } else { None };
            (img, upper, ed)
        })
        .collect();
    for (s, (img, upper, ed)) in samples.iter_mut().zip(results) {
        s.set("image_name", V::from(img.clone()));
        s.set("upper_len", V::from(upper));
        if let Some(e) = ed {
            set_metric(s, "Edit_dist", V::Float(e as f64 / upper as f64));
            s.set("Edit_num", V::from(e));
        }
        rows.push((img, ed.map(|e| e as f64), upper as f64));
    }
    if samples.is_empty() {
        return dict(vec![("Edit_dist", dict(vec![("ALL_page_avg", V::str("NaN"))]))]);
    }
    // groupby("image_name") sorted by key; value = Edit_num.sum() / upper_len.sum()
    let mut groups: BTreeMap<&str, (Vec<f64>, Vec<f64>)> = BTreeMap::new();
    for (img, e, u) in &rows {
        let g = groups.entry(img.as_str()).or_default();
        g.0.push(e.unwrap_or(f64::NAN));
        g.1.push(*u);
    }
    let any_nan = rows.iter().any(|r| r.1.is_none());
    let mut per_img = Map::new();
    let mut page_vals = Vec::new();
    for (img, (e, u)) in &groups {
        // int columns sum exactly; with NaNs the column is float (pairwise, still exact for ints)
        let es = if any_nan { pd_sum(e) } else { e.iter().sum::<f64>() };
        let us: f64 = u.iter().sum();
        let v = es / us;
        per_img.insert(img.to_string(), fv(v));
        page_vals.push(v);
    }
    write_json(std::path::Path::new(&format!("{}_per_page_edit.json", prefix)), &V::from(per_img));
    let e_all: Vec<f64> = rows.iter().map(|r| r.1.unwrap_or(f64::NAN)).collect();
    let e_sum = if any_nan { pd_sum(&e_all) } else { e_all.iter().sum::<f64>() };
    let u_sum: f64 = rows.iter().map(|r| r.2).sum();
    let edit_whole = e_sum / u_sum;
    let ratios: Vec<f64> = rows.iter().map(|r| r.1.map_or(f64::NAN, |e| e / r.2)).collect();
    dict(vec![(
        "Edit_dist",
        dict(vec![
            ("ALL_page_avg", fv(pd_mean(&page_vals))),
            ("edit_whole", fv(edit_whole)),
            ("edit_sample_avg", fv(pd_mean(&ratios))),
        ]),
    )])
}

static SPAN_ATTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\b(row|col)span\s*=\s*([^\s\x1c-\x1f>]*)").unwrap());
static DIGITS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

/// Marathon patch `_fix_span_attrs`.
pub fn fix_span_attrs(html: &str) -> String {
    if html.is_empty() || !html.contains("span") {
        return html.to_string();
    }
    SPAN_ATTR
        .replace_all(html, |c: &regex::Captures| {
            let d = DIGITS.find(c.get(2).unwrap().as_str()).map_or("1".to_string(), |m| m.as_str().to_string());
            format!("{}span=\"{}\"", &c[1], d)
        })
        .into_owned()
}

/// `call_TEDS.evaluate`. Writes `{prefix}_per_table_TEDS.json`.
pub fn teds_metric(samples: &mut [V], prefix: &str, element: &str, marathon: bool, zero_keys: &std::collections::HashSet<String>) -> V {
    let scores: Vec<(V, V)> = samples
        .par_iter()
        .map(|s| {
            let gt = gt_of(s).py_str();
            let pred = pred_of(s).py_str();
            let (gt, pred) = if marathon { (fix_span_attrs(&gt), fix_span_attrs(&pred)) } else { (gt, pred) };
            let key = format!("{}_{}", s.get("img_id").py_str(), s.get("gt_idx").py_str());
            if zero_keys.contains(&format!("{}/{}", element, key)) {
                return (V::Int(0), V::Int(0));
            }
            let one = |so: bool| match teds::teds(&pred, &gt, so) {
                Ok(x) => V::Float(x),
                Err(_) => {
                    eprintln!(
                        "TEDS{} score error for table {} in {}. The score is set to 0.",
                        if so { "_structure_only" } else { "" },
                        s.get("gt_idx").py_str(),
                        s.get("img_id").py_str()
                    );
                    V::Int(0)
                }
            };
            (one(false), one(true))
        })
        .collect();
    let mut per_table = Map::new();
    let mut all = Vec::new();
    let mut all_so = Vec::new();
    for (s, (sc, so)) in samples.iter_mut().zip(scores) {
        set_metric(s, "TEDS", sc.clone());
        set_metric(s, "TEDS_structure_only", so.clone());
        let key = format!("{}_{}", s.get("img_id").py_str(), s.get("gt_idx").py_str());
        per_table.insert(key, dict(vec![("TEDS", sc.clone()), ("TEDS_structure_only", so.clone())]));
        all.push(sc.as_f64().unwrap());
        all_so.push(so.as_f64().unwrap());
    }
    write_json(std::path::Path::new(&format!("{}_per_table_TEDS.json", prefix)), &V::from(per_table));
    let mut r = Map::new();
    let mut r_so = Map::new();
    if !all.is_empty() {
        r.insert("all".into(), fv(py_mean(&all)));
        r_so.insert("all".into(), fv(py_mean(&all_so)));
    }
    dict(vec![("TEDS", V::from(r)), ("TEDS_structure_only", V::from(r_so))])
}

/// `call_HeadTEDS.evaluate`. `inputs[i]` is the structured gold/pred of `samples[i]`.
pub fn head_teds_metric(samples: &mut [V], inputs: &[crate::dataset::HeadSample], prefix: &str) -> V {
    let results: Vec<(V, i64)> = inputs
        .par_iter()
        .map(|h| {
            let rels: Vec<(teds::HeadId, teds::HeadId)> = h
                .relations
                .iter()
                .map(|r| (head_id(r.get("source_anno_id")), head_id(r.get("target_anno_id"))))
                .collect();
            let nodes: Vec<teds::HeadNode> = h
                .nodes
                .iter()
                .map(|(k, item)| teds::HeadNode {
                    id: match k {
                        crate::dataset::Key::Int(i) => teds::HeadId::Int(*i),
                        other => panic!("non-int title anno_id {:?}", other),
                    },
                    text: item.get("text").as_str().map(|s| s.to_string()),
                    anno_id: item.get("anno_id").as_i64().expect("int anno_id"),
                })
                .collect();
            match teds::head_teds(&h.pred, &rels, &nodes) {
                Ok((score, depth)) => (V::Float(score), depth),
                Err(e) => {
                    eprintln!("HeadTEDS score error for sample: {:?}. The score is set to 0.", e);
                    (V::Int(0), 0)
                }
            }
        })
        .collect();
    let mut per = Map::new();
    let mut all = Vec::new();
    for (s, (score, depth)) in samples.iter_mut().zip(results) {
        set_metric(s, "HeadTEDS", score.clone());
        let mut a = Map::new();
        a.insert("heading tree depth".into(), V::Int(depth));
        s.set("gt_attribute", V::list(vec![V::from(a)]));
        per.insert(s.get("img_id").py_str(), score.clone());
        all.push(score.as_f64().unwrap());
    }
    write_json(std::path::Path::new(&format!("{}_per_sample_HeadTEDS.json", prefix)), &V::from(per));
    let mut r = Map::new();
    if !all.is_empty() {
        r.insert("all".into(), fv(py_mean(&all)));
    }
    dict(vec![("HeadTEDS", V::from(r))])
}

fn head_id(v: &V) -> teds::HeadId {
    match v {
        V::Str(s) if s.as_ref() == "root" => teds::HeadId::Root,
        V::Int(i) => teds::HeadId::Int(*i),
        other => panic!("unexpected relation id {}", other.py_repr()),
    }
}

/// `call_FigureF1.evaluate`.
pub fn figure_f1(samples: &mut [V], prefix: &str) -> V {
    let mut per = Map::new();
    let mut all = Vec::new();
    for s in samples.iter_mut() {
        let gt = s.get("gt_count").as_i64().unwrap_or(0);
        let pc = s.get("pred_count").as_i64().unwrap_or(0);
        let mc = s.get("matched_count").as_i64().unwrap_or(0);
        let precision = if pc > 0 { V::Float(mc as f64 / pc as f64) } else { V::Int(0) };
        let recall = if gt > 0 { V::Float(mc as f64 / gt as f64) } else { V::Int(0) };
        let (p, r) = (precision.as_f64().unwrap(), recall.as_f64().unwrap());
        let f1 = if p + r > 0.0 { V::Float(2.0 * p * r / (p + r)) } else { V::Int(0) };
        set_metric(s, "FigureF1", f1.clone());
        per.insert(
            s.get("img_id").py_str(),
            dict(vec![
                ("f1", f1.clone()),
                ("precision", precision),
                ("recall", recall),
                ("gt_count", V::Int(gt)),
                ("pred_count", V::Int(pc)),
                ("matched_count", V::Int(mc)),
            ]),
        );
        all.push(f1.as_f64().unwrap());
    }
    write_json(std::path::Path::new(&format!("{}_per_doc_FigureF1.json", prefix)), &V::from(per));
    let mut r = Map::new();
    if !all.is_empty() {
        r.insert("all".into(), fv(py_mean(&all)));
    }
    dict(vec![("FigureF1", V::from(r))])
}

fn sort_nested(v: &V) -> V {
    match v {
        V::Dict(d) => {
            let mut keys: Vec<&String> = d.keys().collect();
            keys.sort();
            let mut m = Map::new();
            for k in keys {
                m.insert(k.clone(), sort_nested(&d[k]));
            }
            V::from(m)
        }
        other => other.clone(),
    }
}

/// `get_full_labels_results(samples)`; `is_list` says whether the metric returned a
/// plain list (empty → `{}`) or a dataset object (always iterated).
pub fn full_labels_results(samples: &[V], is_list: bool) -> V {
    if is_list && samples.is_empty() {
        return V::empty_dict();
    }
    // label -> metric -> scores, insertion ordered
    let mut groups: Vec<(String, Vec<(String, Vec<f64>)>)> = Vec::new();
    for s in samples {
        let ga = s.get("gt_attribute");
        if !ga.truthy() {
            continue;
        }
        let mut labels: Vec<String> = Vec::new();
        for anno in ga.as_list().unwrap().iter() {
            for (k, v) in anno.as_dict().expect("gt_attribute entry").iter() {
                labels.push(format!("{}: {}", k, v.py_str()));
            }
        }
        labels.sort();
        labels.dedup();
        let metric = s.get("metric").as_dict().unwrap_or_else(|| panic!("KeyError: 'metric'"));
        for l in labels {
            let gi = match groups.iter().position(|g| g.0 == l) {
                Some(i) => i,
                None => {
                    groups.push((l, Vec::new()));
                    groups.len() - 1
                }
            };
            for (m, score) in metric.iter() {
                let g = &mut groups[gi].1;
                let mi = match g.iter().position(|x| x.0 == *m) {
                    Some(i) => i,
                    None => {
                        g.push((m.clone(), Vec::new()));
                        g.len() - 1
                    }
                };
                g[mi].1.push(score.as_f64().expect("numeric score"));
            }
        }
    }
    let mut result = Map::new();
    result.insert("sample_count".into(), V::empty_dict());
    for (attr, metrics) in &groups {
        for (m, scores) in metrics {
            let mean = py_mean(scores);
            if !result.get(m).map_or(false, |x| x.truthy()) {
                result.insert(m.clone(), V::empty_dict());
            }
            result.get_mut(m).unwrap().set(attr, fv(mean));
            result.get_mut("sample_count").unwrap().set(attr, V::from(scores.len()));
        }
    }
    sort_nested(&V::from(result))
}

/// `get_page_split(samples, page_info)`
pub fn page_split(samples: &[V], page_info: &Map) -> V {
    if page_info.is_empty() {
        return V::empty_dict();
    }
    // metric -> rows (image_name, attribute, score, upper_len), metric insertion ordered
    let mut lists: Vec<(String, Vec<(String, String, f64, f64)>)> = Vec::new();
    for s in samples {
        let img = image_name(&s.get("img_id").py_str());
        let pinfo = page_info.get(&img).unwrap_or_else(|| panic!("KeyError: {}", img));
        if !s.get("metric").truthy() {
            continue;
        }
        let upper = gt_of(s).py_len().max(pred_of(s).py_len()) as f64;
        for (m, score) in s.get("metric").as_dict().unwrap().iter() {
            let score = score.as_f64().expect("numeric score");
            let li = match lists.iter().position(|x| x.0 == *m) {
                Some(i) => i,
                None => {
                    lists.push((m.clone(), Vec::new()));
                    lists.len() - 1
                }
            };
            let rows = &mut lists[li].1;
            rows.push((img.clone(), "ALL".into(), score, upper));
            if let Some(attrs) = pinfo.as_dict() {
                for (k, v) in attrs.iter() {
                    if let V::List(issues) = v {
                        for si in issues.iter() {
                            let si = si.py_str();
                            if !si.contains("table") {
                                rows.push((img.clone(), si, score, upper));
                            }
                        }
                    } else {
                        rows.push((img.clone(), format!("{}: {}", k, v.py_str()), score, upper));
                    }
                }
            }
        }
    }
    let mut result = Map::new();
    let agg = |rows: &[(String, String, f64, f64)], edit: bool| -> V {
        // groupby([image_name, attribute]) sorted; then groupby(attribute).mean() (Kahan)
        let mut g: BTreeMap<(&str, &str), Vec<usize>> = BTreeMap::new();
        for (i, r) in rows.iter().enumerate() {
            g.entry((r.0.as_str(), r.1.as_str())).or_default().push(i);
        }
        let mut by_attr: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
        for ((_, attr), idx) in &g {
            let v = if edit {
                let prod: Vec<f64> = idx.iter().map(|&i| rows[i].2 * rows[i].3).collect();
                let num = pd_sum(&prod);
                let den: f64 = idx.iter().map(|&i| rows[i].3).sum();
                num / den
            } else {
                let sc: Vec<f64> = idx.iter().map(|&i| rows[i].2).collect();
                pd_mean(&sc)
            };
            by_attr.entry(attr).or_default().push(v);
        }
        let mut m = Map::new();
        for (attr, vals) in by_attr {
            m.insert(attr.to_string(), fv(pd_group_mean(&vals)));
        }
        V::from(m)
    };
    if let Some((_, rows)) = lists.iter().find(|x| x.0 == "Edit_dist") {
        if !rows.is_empty() {
            result.insert("Edit_dist".into(), agg(rows, true));
        }
    }
    for (m, rows) in &lists {
        if m == "Edit_dist" {
            continue;
        }
        result.insert(m.clone(), agg(rows, false));
    }
    sort_nested(&V::from(result))
}

/// `get_figure_label_results(samples)`
pub fn figure_label_results(samples: &[V]) -> V {
    if samples.is_empty() {
        return V::empty_dict();
    }
    let mut totals: BTreeMap<String, (i64, i64)> = BTreeMap::new();
    let mut per_doc: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    let (mut gm, mut gp) = (0i64, 0i64);
    for s in samples {
        let dm = s.get("matched_count").as_i64().unwrap_or(0);
        let dp = s.get("pred_count").as_i64().unwrap_or(0);
        let prec = if dp > 0 { dm as f64 / dp as f64 } else { 0.0 };
        gm += dm;
        gp += dp;
        if let Some(ls) = s.get("label_stats").as_dict() {
            for (label, st) in ls.iter() {
                let g = st.get("gt_count").as_i64().unwrap_or(0);
                let m = st.get("matched_count").as_i64().unwrap_or(0);
                let t = totals.entry(label.clone()).or_insert((0, 0));
                t.0 += g;
                t.1 += m;
                if g > 0 {
                    let rec = m as f64 / g as f64;
                    let f1 = if prec + rec > 0.0 { 2.0 * prec * rec / (prec + rec) } else { 0.0 };
                    per_doc.entry(label.clone()).or_default().push(f1);
                }
            }
        }
    }
    let micro_p = if gp > 0 { gm as f64 / gp as f64 } else { 0.0 };
    let mut micro = Map::new();
    let mut macro_ = Map::new();
    let mut count = Map::new();
    for (label, (g, m)) in &totals {
        if *g > 0 {
            let rec = *m as f64 / *g as f64;
            let f1 = if micro_p + rec > 0.0 { V::Float(2.0 * micro_p * rec / (micro_p + rec)) } else { V::Int(0) };
            micro.insert(label.clone(), f1);
        } else {
            micro.insert(label.clone(), V::str("NaN"));
        }
        match per_doc.get(label) {
            Some(f) if !f.is_empty() => macro_.insert(label.clone(), fv(py_mean(f))),
            _ => macro_.insert(label.clone(), V::str("NaN")),
        };
        count.insert(label.clone(), V::Int(*g));
    }
    dict(vec![
        ("FigureF1_by_label(micro)", V::from(micro)),
        ("FigureF1_by_label(macro)", V::from(macro_)),
        ("gt_count_by_label", V::from(count)),
    ])
}
