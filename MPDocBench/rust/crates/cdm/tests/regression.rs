//! End-to-end regression against scores and token-box files recorded from the Python
//! toolkit (`_process_single_cdm_sample`). `recorded_samples` renders with the external
//! backend (node/xelatex/magick), so it
//! needs the toolchain parity was proven with on PATH (TeX Live 2024, ImageMagick
//! 7.1.1-43 with the image's delegates via MAGICK_CONFIGURE_PATH, Ghostscript, node 20):
//!
//!     cargo test -p cdm --test regression -- --ignored
//!
//! `recorded_scores_native` checks only the scores with the in-process backend (Tectonic
//! bundle cached or reachable; `CDM_TECTONIC_CACHE` as for the scorer).

use std::path::{Path, PathBuf};

use serde_json::Value;

#[test]
#[ignore = "needs the CDM render toolchain on PATH"]
fn recorded_samples() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/regression");
    let cases: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(dir.join("cases.json")).unwrap()).unwrap();
    let work = std::env::temp_dir().join(format!("cdm-regression-{}", std::process::id()));
    std::fs::create_dir_all(&work).unwrap();
    let root = work.join("result/x/CDM");
    let mut bad = Vec::new();
    for c in &cases {
        let idx = c["idx"].as_u64().unwrap() as usize;
        let marathon = c["marathon"].as_bool().unwrap_or(false);
        let r = cdm::cdm_sample_with(
            idx,
            c["gt_raw"].as_str().unwrap(),
            c["pred_raw"].as_str().unwrap(),
            &root,
            marathon,
            &cdm::Backend::external(),
        )
        .unwrap();
        let (score, ty) = match r.score {
            cdm::Score::Int(i) => (i as f64, "int"),
            cdm::Score::Float(f) => (f, "float"),
        };
        if score != c["score"].as_f64().unwrap() || ty != c["type"].as_str().unwrap() {
            bad.push(format!("{idx}: got {ty} {score}, want {} {}", c["type"], c["score"]));
        }
        if r.gt != c["gt"].as_str().unwrap() || r.pred != c["pred"].as_str().unwrap() {
            bad.push(format!("{idx}: cleaning differs"));
        }
        for subset in ["gt", "pred"] {
            let want = dir.join(format!("{subset}_{idx}.jsonl"));
            let got = root.join(subset).join("bbox").join(format!("{idx}.jsonl"));
            let read = |p: &Path| std::fs::read_to_string(p).ok();
            if read(&want) != read(&got) {
                bad.push(format!("{idx}: {subset} bbox file differs"));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&work);
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
#[ignore = "needs the Tectonic bundle (cached or network)"]
fn recorded_scores_native() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/regression");
    let cases: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(dir.join("cases.json")).unwrap()).unwrap();
    let work = std::env::temp_dir().join(format!("cdm-regression-native-{}", std::process::id()));
    let root = work.join("result/x/CDM");
    let mut bad = Vec::new();
    for c in &cases {
        let idx = c["idx"].as_u64().unwrap() as usize;
        let marathon = c["marathon"].as_bool().unwrap_or(false);
        let r = cdm::cdm_sample_with(
            idx,
            c["gt_raw"].as_str().unwrap(),
            c["pred_raw"].as_str().unwrap(),
            &root,
            marathon,
            &cdm::Backend::native(),
        )
        .unwrap();
        if r.score.as_f64() != c["score"].as_f64().unwrap() {
            bad.push(format!("{idx}: got {}, want {}", r.score.as_f64(), c["score"]));
        }
    }
    let _ = std::fs::remove_dir_all(&work);
    assert!(bad.is_empty(), "{bad:#?}");
}
