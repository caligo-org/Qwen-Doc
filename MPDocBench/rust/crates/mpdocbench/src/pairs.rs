//! `--export-pairs DIR`: one JSON line per scored pair, with the gold elements it contains.
//! Read-only over the scored samples: no score changes.

use crate::pyval::{to_json_line, Map, V, INTERNAL_KEY};
use std::io::Write;
use std::path::Path;

pub const CATEGORIES: [&str; 4] = ["text_block", "merged_text_block", "table", "merged_table"];

/// The layout elements of one gold item: the parts of a merged truncated chain, or the item.
fn parts(item: &V) -> Vec<V> {
    match item.get("merge_list").as_list() {
        Some(l) if !l.is_empty() => l.to_vec(),
        _ => vec![item.clone()],
    }
}

pub fn export(dir: &Path, category: &str, samples: &[V]) {
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join(format!("{}_pairs.jsonl", category));
    let mut f = std::io::BufWriter::new(std::fs::File::create(&path).unwrap_or_else(|e| panic!("{}: {}", path.display(), e)));
    let table = category.ends_with("table");
    for s in samples {
        let gold: Vec<V> = s.get(INTERNAL_KEY).as_list().map(|l| l.iter().flat_map(parts).collect()).unwrap_or_default();
        let img = s.get("img_id").py_str();
        let mut m = Map::new();
        m.insert("doc".into(), V::from(img.strip_suffix(".pdf").unwrap_or(&img).to_string()));
        m.insert("category".into(), V::str(category));
        m.insert("gold_anno_ids".into(), V::list(gold.iter().map(|g| g.get("anno_id").clone()).collect()));
        m.insert("gold_pages".into(), V::list(gold.iter().map(|g| g.get("page_id").clone()).collect()));
        m.insert("gold_polys".into(), V::list(gold.iter().map(|g| g.get("poly").clone()).collect()));
        for k in ["gt", "pred", "norm_gt", "norm_pred"] {
            m.insert(k.into(), s.get(k).clone());
        }
        let metric = s.get("metric");
        if table {
            m.insert("teds".into(), metric.get("TEDS").clone());
            m.insert("teds_structure_only".into(), metric.get("TEDS_structure_only").clone());
        } else {
            m.insert("edit_num".into(), s.get("Edit_num").clone());
            m.insert("upper_len".into(), s.get("upper_len").clone());
            m.insert("edit".into(), metric.get("Edit_dist").clone());
        }
        m.insert("pred_position".into(), s.get("pred_position").clone());
        let unmatched = s.get("pred_idx").as_list().map_or(true, |l| l.len() == 1 && l[0].as_str() == Some(""));
        m.insert("matched".into(), V::Bool(!unmatched));
        writeln!(f, "{}", to_json_line(&V::from(m))).unwrap();
    }
}
