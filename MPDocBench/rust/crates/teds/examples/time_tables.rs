//! Times `teds` on each {"pred","gt"} line: `cargo run --release --example time_tables -- PAIRS.jsonl`
use std::time::Instant;
fn main() {
    let path = std::env::args().nth(1).unwrap();
    for line in std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        let d: serde_json::Value = serde_json::from_str(line).unwrap();
        for so in [false, true] {
            let t0 = Instant::now();
            let r = teds::teds(d["pred"].as_str().unwrap(), d["gt"].as_str().unwrap(), so);
            println!(
                "cells {} structure_only {so}: {r:?} in {:.3}s",
                d["cells"],
                t0.elapsed().as_secs_f64()
            );
        }
    }
}
