//! Port of `metrics/cdm_metric.py` (class CDM).

use std::fs;

use crate::matcher::{hungarian_match, ransac_inliers, BoxTok};
use crate::pil::{self, RgbImage};
use crate::py::{path_join, round3, PyErr, PyResult};
use crate::render::{gen_color_list, latex2bbox_color};
use crate::Backend;

const MAX_ITER: usize = 3;
const MIN_SAMPLES: usize = 3;

/// The dict `CDM.evaluate` returns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalResult {
    /// `{"recall": 0, "precision": 0, "F1_score": 0}` (the `except:` branch, ints)
    Zero,
    Scores { recall: f64, precision: f64, f1: f64 },
}

fn mkdirs(p: &str) -> PyResult<()> {
    fs::create_dir_all(p).map_err(|e| PyErr::new(format!("OSError: {p}: {e}")))
}

fn prepare_directories(root: &str) -> PyResult<()> {
    for sub in ["gt/bbox", "gt/vis", "pred/bbox", "pred/vis", "vis_match"] {
        let mut p = root.to_string();
        for part in sub.split('/') {
            p = path_join(&p, part);
        }
        mkdirs(&p)?;
    }
    Ok(())
}

fn generate_bboxes(root: &str, gt: &str, pred: &str, img_id: &str, backend: &Backend) -> PyResult<()> {
    let colors = gen_color_list(5800, 15);
    for (subset, latex) in [("gt", gt), ("pred", pred)] {
        let output_path = path_join(root, subset);
        let temp_dir = path_join(root, &format!("temp_dir_{subset}_{img_id}"));
        mkdirs(&temp_dir)?;
        latex2bbox_color(latex, img_id, &output_path, &temp_dir, &colors, backend)?;
        fs::remove_dir_all(&temp_dir).map_err(|e| PyErr::new(format!("OSError: {temp_dir}: {e}")))?;
    }
    Ok(())
}

fn load_box_file(path: &str) -> PyResult<Vec<BoxTok>> {
    let b = fs::read(path).map_err(|e| PyErr::new(format!("FileNotFoundError: {path}: {e}")))?;
    let t = String::from_utf8(b).map_err(|_| PyErr::new("UnicodeDecodeError"))?;
    let t = t.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = Vec::new();
    for line in t.split_inclusive('\n') {
        let v: serde_json::Value = serde_json::from_str(line).map_err(|e| PyErr::new(format!("JSONDecodeError: {e}")))?;
        let bbox = v.get("bbox").ok_or_else(|| PyErr::new("KeyError: bbox"))?;
        let truthy = match bbox {
            serde_json::Value::Array(a) => !a.is_empty(),
            serde_json::Value::Null => false,
            serde_json::Value::Bool(b) => *b,
            serde_json::Value::Number(n) => n.as_f64() != Some(0.0),
            serde_json::Value::String(s) => !s.is_empty(),
            serde_json::Value::Object(o) => !o.is_empty(),
        };
        if !truthy {
            continue;
        }
        let arr = bbox.as_array().ok_or_else(|| PyErr::new("TypeError: bbox"))?;
        if arr.len() != 4 {
            return Err(PyErr::new("ValueError: unpack bbox"));
        }
        let mut bb = [0i64; 4];
        for (k, x) in arr.iter().enumerate() {
            bb[k] = x.as_i64().ok_or_else(|| PyErr::new("TypeError: bbox value"))?;
        }
        let token = v.get("token").and_then(|x| x.as_str()).ok_or_else(|| PyErr::new("KeyError: token"))?.to_string();
        out.push(BoxTok { bbox: bb, token });
    }
    Ok(out)
}

fn update_inliers(ori: &[bool], sub: &[bool]) -> Vec<bool> {
    let mut inliers = ori.to_vec();
    let mut sub_idx: i64 = -1;
    for i in 0..ori.len() {
        if !ori[i] {
            sub_idx += 1;
            if sub[sub_idx as usize] {
                inliers[i] = true;
            }
        }
    }
    inliers
}

/// Matched (gt, pred) index pairs and their inlier flags.
pub(crate) type MatchResult = (Vec<(usize, usize)>, Vec<bool>);

/// `CDM._match_boxes`; None where the matcher raises (an empty box list).
pub(crate) fn match_boxes(
    box_gt: &[BoxTok],
    box_pred: &[BoxTok],
    gt_size: (usize, usize),
    pred_size: (usize, usize),
) -> Option<MatchResult> {
    let m = hungarian_match(box_gt, box_pred, gt_size, pred_size)?;
    let src: Vec<[f64; 2]> = m
        .pairs
        .iter()
        .map(|&(a, _)| {
            let b = box_gt[a].bbox;
            [(b[1] + b[3]) as f64 / 2.0, (b[0] + b[2]) as f64 / 2.0]
        })
        .collect();
    let dst: Vec<[f64; 2]> = m
        .pairs
        .iter()
        .map(|&(_, p)| {
            let b = box_pred[p].bbox;
            [(b[1] + b[3]) as f64 / 2.0, (b[0] + b[2]) as f64 / 2.0]
        })
        .collect();
    let n = m.pairs.len();
    let mut inliers;
    if n <= MIN_SAMPLES {
        inliers = vec![true; n];
    } else {
        inliers = vec![false; n];
        for _ in 0..MAX_ITER {
            let rest: Vec<usize> = (0..n).filter(|&k| !inliers[k]).collect();
            if rest.len() <= MIN_SAMPLES {
                break;
            }
            let s: Vec<[f64; 2]> = rest.iter().map(|&k| src[k]).collect();
            let d: Vec<[f64; 2]> = rest.iter().map(|&k| dst[k]).collect();
            match ransac_inliers(&s, &d) {
                Some(in1) if in1.iter().any(|&b| b) => inliers = update_inliers(&inliers, &in1),
                _ => break,
            }
            if inliers.iter().filter(|&&b| b).count() >= n {
                break;
            }
        }
    }
    for (k, &(a, b)) in m.pairs.iter().enumerate() {
        if inliers[k] && m.token_cost[a * m.n_pred + b] == 1.0 {
            inliers[k] = false;
        }
    }
    Some((m.pairs, inliers))
}

struct Loaded {
    box_gt: Vec<BoxTok>,
    box_pred: Vec<BoxTok>,
    img_gt: RgbImage,
    img_pred: RgbImage,
    pairs: Vec<(usize, usize)>,
    inliers: Vec<bool>,
}

fn match_and_load(root: &str, gt: &str, pred: &str, img_id: &str, backend: &Backend) -> PyResult<Loaded> {
    prepare_directories(root)?;
    generate_bboxes(root, gt, pred, img_id, backend)?;
    let box_gt = load_box_file(&path_join(&path_join(&path_join(root, "gt"), "bbox"), &format!("{img_id}.jsonl")))?;
    let box_pred = load_box_file(&path_join(&path_join(&path_join(root, "pred"), "bbox"), &format!("{img_id}.jsonl")))?;
    let img_gt = pil::open_png(&path_join(&path_join(&path_join(root, "gt"), "vis"), &format!("{img_id}_base.png")))?.to_rgb();
    let img_pred = pil::open_png(&path_join(&path_join(&path_join(root, "pred"), "vis"), &format!("{img_id}_base.png")))?.to_rgb();

    let (pairs, inliers) = match_boxes(&box_gt, &box_pred, (img_gt.w, img_gt.h), (img_pred.w, img_pred.h))
        .ok_or_else(|| PyErr::new("IndexError: empty box list"))?;
    Ok(Loaded { box_gt, box_pred, img_gt, img_pred, pairs, inliers })
}

fn visualize(root: &str, l: &Loaded, img_id: &str) -> PyResult<()> {
    let gap = 5usize;
    let (w1, h1) = (l.img_gt.w, l.img_gt.h);
    let (w2, h2) = (l.img_pred.w, l.img_pred.h);
    let h = h1 + h2 + gap;
    let w = w1.max(w2);
    let mut vis = RgbImage::new(w, h, [255, 255, 255]);
    vis.paste(&l.img_gt, 0, 0);
    vis.paste(&RgbImage::new(w, gap, [120, 120, 120]), 0, h1 as i64);
    vis.paste(&l.img_pred, 0, (h1 + gap) as i64);
    let mut m = vis.clone();
    // {a: flag for (a, b), flag in zip(...)}: the last pair for a key wins (keys are unique)
    let mut gt_flag: Vec<Option<bool>> = vec![None; l.box_gt.len()];
    let mut pred_flag: Vec<Option<bool>> = vec![None; l.box_pred.len()];
    for (&(a, b), &f) in l.pairs.iter().zip(&l.inliers) {
        gt_flag[a] = Some(f);
        pred_flag[b] = Some(f);
    }
    const GREEN: [u8; 3] = [0, 128, 0];
    const RED: [u8; 3] = [255, 0, 0];
    for (i, bx) in l.box_gt.iter().enumerate() {
        let c = if gt_flag[i] == Some(true) { GREEN } else { RED };
        let b = bx.bbox;
        m.rectangle_outline(b[0] - 1, b[1] - 1, b[2] + 1, b[3] + 1, c, 2)?;
    }
    let off = (h1 + gap) as i64;
    for (i, bx) in l.box_pred.iter().enumerate() {
        let c = if pred_flag[i] == Some(true) { GREEN } else { RED };
        let b = bx.bbox;
        m.rectangle_outline(b[0] - 1, b[1] - 1 + off, b[2] + 1, b[3] + 1 + off, c, 2)?;
    }
    let dir = path_join(root, "vis_match");
    vis.save_png(&path_join(&dir, &format!("{img_id}_base.png")))?;
    m.save_png(&path_join(&dir, &format!("{img_id}.png")))?;
    Ok(())
}

/// `CDM(output_root).evaluate(gt_latex, pred_latex, img_id)`. Err where Python raises out
/// of evaluate (after the `try:` block).
pub fn evaluate(root: &str, gt: &str, pred: &str, img_id: &str, backend: &Backend) -> PyResult<EvalResult> {
    let l = match match_and_load(root, gt, pred, img_id, backend) {
        Ok(l) => l,
        Err(_) => return Ok(EvalResult::Zero),
    };
    let k = l.inliers.iter().filter(|&&b| b).count() as f64;
    let (ng, np_) = (l.box_gt.len(), l.box_pred.len());
    if ng == 0 || np_ == 0 {
        return Err(PyErr::new("ZeroDivisionError: division by zero"));
    }
    let recall = round3(k / ng as f64);
    let precision = round3(k / np_ as f64);
    let f1 = round3(2.0 * k / (ng + np_) as f64);
    visualize(root, &l, img_id)?;
    Ok(EvalResult::Scores { recall, precision, f1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `CDM._match_boxes` + `_calculate_metrics` (Hungarian matching, RANSAC with
    /// random_state=42, token filter) on random box sets, against Python.
    /// `CDM_MATCH_REF=path` runs a larger reference file.
    #[test]
    fn match_boxes_matches_python() {
        let txt = match std::env::var("CDM_MATCH_REF") {
            Ok(p) => std::fs::read_to_string(p).unwrap(),
            Err(_) => include_str!("../tests/data/match_ref.json").to_string(),
        };
        let cases: Vec<serde_json::Value> = serde_json::from_str(&txt).unwrap();
        let boxes = |v: &serde_json::Value| -> Vec<BoxTok> {
            v.as_array()
                .unwrap()
                .iter()
                .map(|b| {
                    let bb: Vec<i64> = b["bbox"].as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect();
                    BoxTok { bbox: [bb[0], bb[1], bb[2], bb[3]], token: b["token"].as_str().unwrap().to_string() }
                })
                .collect()
        };
        let size = |v: &serde_json::Value| (v[0].as_u64().unwrap() as usize, v[1].as_u64().unwrap() as usize);
        for (k, c) in cases.iter().enumerate() {
            let (g, p) = (boxes(&c["g"]), boxes(&c["p"]));
            let (pairs, inl) = match_boxes(&g, &p, size(&c["gs"]), size(&c["ps"])).unwrap();
            let want_pairs: Vec<(usize, usize)> = c["m"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| (x[0].as_u64().unwrap() as usize, x[1].as_u64().unwrap() as usize))
                .collect();
            let want_inl: Vec<bool> = c["i"].as_array().unwrap().iter().map(|x| x.as_bool().unwrap()).collect();
            assert_eq!(pairs, want_pairs, "case {k}");
            assert_eq!(inl, want_inl, "case {k}");
            let n = inl.iter().filter(|&&b| b).count() as f64;
            let f = [
                round3(n / g.len() as f64),
                round3(n / p.len() as f64),
                round3(2.0 * n / (g.len() + p.len()) as f64),
            ];
            let want_f: Vec<f64> = c["f"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
            assert_eq!(f.to_vec(), want_f, "case {k}");
        }
    }
}
