//! Differential-test driver: `cdm_run INPUT.json OUTPUT.json OUTPUT_ROOT [marathon]`.
//! INPUT is `[{"idx": .., "gt": .., "pred": ..}, ...]`; samples run sequentially.
//! Writes `{idx: {"score", "type", "err", "time", "gt", "pred"}}`.

use std::path::Path;
use std::time::Instant;

use serde_json::{json, Map, Value};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let items: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let root = Path::new(&a[3]);
    let marathon = a.get(4).map(|s| s == "1").unwrap_or(false);
    let mut out = Map::new();
    for it in items {
        let idx = it["idx"].as_u64().unwrap() as usize;
        let t = Instant::now();
        let r = cdm::cdm_sample(idx, it["gt"].as_str().unwrap(), it["pred"].as_str().unwrap(), root, marathon);
        let dt = t.elapsed().as_secs_f64();
        let v = match r {
            Ok(s) => {
                let (score, ty) = match s.score {
                    cdm::Score::Int(i) => (json!(i), "int"),
                    cdm::Score::Float(f) => (json!(f), "float"),
                };
                json!({"score": score, "type": ty, "err": null, "time": dt, "gt": s.gt, "pred": s.pred})
            }
            Err(e) => json!({"score": null, "type": "NoneType", "err": e.to_string(), "time": dt}),
        };
        eprintln!("{idx} {} {:.2}", v["score"], dt);
        out.insert(idx.to_string(), v);
    }
    std::fs::write(&a[2], serde_json::to_string_pretty(&Value::Object(out)).unwrap()).unwrap();
}
