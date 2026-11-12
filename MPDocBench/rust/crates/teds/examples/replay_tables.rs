//! Replays Python TEDS references: `cargo run --release --example replay_tables -- REFS.jsonl [--show N]`
//! Each line: {"pred","gt","teds","teds_s","t_full","t_s"}; scores are floats or "ERR:<type>".
use serde_json::Value;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let show: usize = args
        .iter()
        .position(|a| a == "--show")
        .map(|i| args[i + 1].parse().unwrap())
        .unwrap_or(10);
    let text = std::fs::read_to_string(path).unwrap();
    let (mut n, mut exact, mut close, mut bad, mut skipped) = (0, 0, 0, 0, 0);
    let mut max_diff = 0f64;
    let (mut t_py, mut t_rs) = (0f64, 0f64);
    let mut slowest = (0f64, 0usize);
    for (lineno, line) in text.lines().enumerate() {
        let d: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => {
                skipped += 1; // e.g. lone surrogates, which a Rust &str cannot hold
                continue;
            }
        };
        let pred = d["pred"].as_str().unwrap();
        let gt = d["gt"].as_str().unwrap();
        t_py += d["t_full"].as_f64().unwrap_or(0.0) + d["t_s"].as_f64().unwrap_or(0.0);
        for (key, so) in [("teds", false), ("teds_s", true)] {
            n += 1;
            let t0 = Instant::now();
            let got = teds::teds(pred, gt, so);
            let dt = t0.elapsed().as_secs_f64();
            t_rs += dt;
            if dt > slowest.0 {
                slowest = (dt, lineno);
            }
            let want = &d[key];
            let ok = match (&got, want.as_f64()) {
                (Ok(g), Some(w)) => {
                    let diff = (g - w).abs();
                    if *g == w {
                        exact += 1;
                        true
                    } else if diff <= 1e-12 {
                        close += 1;
                        max_diff = max_diff.max(diff);
                        true
                    } else {
                        false
                    }
                }
                (Err(_), None) => {
                    exact += 1;
                    true
                }
                _ => false,
            };
            if !ok {
                bad += 1;
                if bad <= show {
                    println!(
                        "MISMATCH line {lineno} {key}: rust={got:?} python={want} pred={:?} gt={:?}",
                        &pred.chars().take(300).collect::<String>(),
                        &gt.chars().take(300).collect::<String>()
                    );
                }
            }
        }
    }
    println!(
        "{path}: {n} scores ({} pairs, {skipped} lines skipped): exact {exact}, within 1e-12 {close} (max abs diff {max_diff:e}), mismatches {bad}",
        n / 2
    );
    println!(
        "time: python {t_py:.1}s (sum of per-call times), rust {t_rs:.3}s; slowest rust call {:.3}s (line {})",
        slowest.0, slowest.1
    );
    println!("libxml2 runtime version {}", teds::html::libxml_version());
}
