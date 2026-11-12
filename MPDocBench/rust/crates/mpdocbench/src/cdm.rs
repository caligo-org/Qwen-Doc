//! Formula CDM (`call_CDM.evaluate`) on top of the Rust CDM port (crate `cdm`), with a
//! bounded pool of threads. By default CDM is fully Rust-native (KaTeX tokenizer port,
//! Tectonic, hayro); `CDM_BACKEND=external` uses node/xelatex/magick as the toolkit does.

use crate::pdnum::py_mean;
use crate::pyval::{Map, V};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

pub struct CdmConfig {
    pub workers: usize,
    pub marathon: bool,
    /// directory the toolkit's relative `result/...` paths are resolved against
    pub base: PathBuf,
}

/// Returns (new samples, result); writes `{prefix}_per_sample_CDM.json` and `{prefix}_result.json`.
pub fn cdm_metric(cfg: &CdmConfig, samples: &[V], prefix: &str, output_root: &str) -> (Vec<V>, V) {
    let root = cfg.base.join(output_root);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(cfg.workers.max(1)).build().unwrap();
    let done = std::sync::atomic::AtomicUsize::new(0);
    let n = samples.len();
    let run = |i: usize, s: &V| {
        let r = cdm::cdm_sample(i, &s.get("gt").py_str(), &s.get("pred").py_str(), &root, cfg.marathon);
        let k = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if k % 200 == 0 || k == n {
            eprintln!("[CDM] {}/{}", k, n);
        }
        r
    };
    let mut results: Vec<Result<cdm::CdmScore, cdm::Error>> =
        pool.install(|| samples.par_iter().enumerate().map(|(i, s)| run(i, s)).collect());
    // failed samples are retried once, sequentially, as the original does
    for i in 0..n {
        if results[i].is_err() {
            eprintln!("[CDM] retrying sample {} sequentially", i);
            results[i] = cdm::cdm_sample(i, &samples[i].get("gt").py_str(), &samples[i].get("pred").py_str(), &root, cfg.marathon);
        }
    }
    let mut out = Vec::with_capacity(n);
    let mut per = Map::new();
    let mut scores = Vec::new();
    for (i, (s, r)) in samples.iter().zip(results).enumerate() {
        let mut c = s.clone();
        c.set("img_id_cdm", V::from(i.to_string()));
        let score = match r {
            Ok(r) => {
                c.set("gt", V::from(r.gt));
                c.set("pred", V::from(r.pred));
                match r.score {
                    cdm::Score::Int(x) => V::Int(x),
                    cdm::Score::Float(x) => V::Float(x),
                }
            }
            Err(e) => {
                eprintln!("Sample {} also failed sequentially: {}", i, e);
                V::Float(0.0)
            }
        };
        if !c.get("metric").truthy() {
            c.set("metric", V::empty_dict());
        }
        c.dict_mut().get_mut("metric").unwrap().set("CDM", score.clone());
        per.insert(format!("{}_{}", c.get("img_id").py_str(), c.get("gt_idx").py_str()), score.clone());
        scores.push(score.as_f64().unwrap());
        out.push(c);
    }
    crate::metrics::write_json(Path::new(&format!("{}_per_sample_CDM.json", prefix)), &V::from(per));
    crate::metrics::write_json(Path::new(&format!("{}_result.json", prefix)), &V::list(out.clone()));
    let mut r = Map::new();
    if !scores.is_empty() {
        r.insert("all".into(), V::Float(py_mean(&scores)));
    }
    let mut res = Map::new();
    res.insert("CDM".into(), V::from(r));
    (out, V::from(res))
}
