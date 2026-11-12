//! Scores formula pairs with `cdm_batch` and compares with reference per-sample scores:
//!
//!     batch_compare PAIRS.json REF.json WORKDIR BATCH_SIZES [--out OUT.json]
//!
//! REF is the `--out` file of `cdm_compare` (field "a": the per-sample native score of
//! pair k, null where it raised). BATCH_SIZES is a comma list, e.g. 8,64,256; each size
//! runs on a fresh WORKDIR/b<size>. Prints differing scores, batch statistics and the
//! wall time per pair (render + boxes + matching), sequential.

use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let pairs: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let refs: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(&a[2]).unwrap()).unwrap();
    let work = std::path::PathBuf::from(&a[3]);
    let sizes: Vec<usize> = a[4].split(',').map(|s| s.parse().unwrap()).collect();
    let out = a.iter().position(|x| x == "--out").map(|i| a[i + 1].clone());
    let items: Vec<(usize, &str, &str)> =
        pairs.iter().enumerate().map(|(k, p)| (k, p["gt"].as_str().unwrap(), p["pred"].as_str().unwrap())).collect();
    let ref_time: f64 = refs.iter().map(|r| r["time_a"].as_f64().unwrap_or(0.0)).sum();
    println!("pairs {}; reference (per-sample native) time per pair {:.3}s", items.len(), ref_time / items.len() as f64);
    let mut dump = Vec::new();
    for size in sizes {
        let root = work.join(format!("b{size}"));
        let _ = std::fs::remove_dir_all(&root);
        let t = Instant::now();
        let (res, st) = cdm::cdm_batch_with(&items, &root, true, &cdm::Backend::native(), size);
        let dt = t.elapsed().as_secs_f64();
        let mut differ = Vec::new();
        let mut raised = (0, 0);
        for (k, r) in res.iter().enumerate() {
            let got = r.as_ref().ok().map(|s| s.score.as_f64());
            let want = refs[k]["a"].as_f64();
            raised.0 += got.is_none() as usize;
            raised.1 += want.is_none() as usize;
            if got != want {
                differ.push((k, want, got));
            }
            dump.push(serde_json::json!({"batch": size, "idx": k, "score": got}));
        }
        println!(
            "batch {size}: {:.3}s per pair ({:.1}s total); differing scores {} {:?}; raised batch/ref {}/{}; {:?}",
            dt / items.len() as f64,
            dt,
            differ.len(),
            &differ[..differ.len().min(10)],
            raised.0,
            raised.1,
            st
        );
    }
    if let Some(o) = out {
        std::fs::write(o, serde_json::to_string(&dump).unwrap()).unwrap();
    }
}
