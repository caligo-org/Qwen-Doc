//! Replays Python HeadTEDS references:
//! `cargo run --release --example replay_heads -- REFS.jsonl [--show N]`
//! Each line: {"pred": [str], "relations": [[src, dst]], "nodes": [{"id", "text", "anno_id"}],
//! "ref": [score, depth] | "ERR:<type>", "t": seconds}; ids are ints or "root".
use serde_json::Value;
use std::time::Instant;
use teds::{head_teds, HeadId, HeadNode};

fn id(v: &Value) -> HeadId {
    match v {
        Value::String(s) if s == "root" => HeadId::Root,
        v => HeadId::Int(v.as_i64().expect("int id")),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let show: usize = args
        .iter()
        .position(|a| a == "--show")
        .map(|i| args[i + 1].parse().unwrap())
        .unwrap_or(10);
    let text = std::fs::read_to_string(&args[1]).unwrap();
    let (mut n, mut exact, mut close, mut bad, mut skipped) = (0, 0, 0, 0, 0);
    let mut max_diff = 0f64;
    let (mut t_py, mut t_rs) = (0f64, 0f64);
    for (lineno, line) in text.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let Ok(d) = serde_json::from_str::<Value>(line) else {
            skipped += 1;
            continue;
        };
        n += 1;
        let pred: Vec<String> = d["pred"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect();
        let rels: Vec<(HeadId, HeadId)> = d["relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| (id(&r[0]), id(&r[1])))
            .collect();
        let nodes: Vec<HeadNode> = d["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| HeadNode {
                id: id(&x["id"]),
                text: x["text"].as_str().map(str::to_owned),
                anno_id: x["anno_id"].as_i64().unwrap(),
            })
            .collect();
        t_py += d["t"].as_f64().unwrap_or(0.0);
        let t0 = Instant::now();
        let got = head_teds(&pred, &rels, &nodes);
        t_rs += t0.elapsed().as_secs_f64();
        let want = &d["ref"];
        let ok = match (&got, want.as_array()) {
            (Ok((s, dep)), Some(w)) => {
                let (ws, wd) = (w[0].as_f64().unwrap(), w[1].as_i64().unwrap());
                if *dep != wd {
                    false
                } else if *s == ws {
                    exact += 1;
                    true
                } else if (s - ws).abs() <= 1e-12 {
                    close += 1;
                    max_diff = max_diff.max((s - ws).abs());
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
                    "MISMATCH line {lineno} {} {}: rust={got:?} python={want}",
                    d["img"], d["kind"]
                );
            }
        }
    }
    println!(
        "{}: {n} cases ({skipped} skipped): exact {exact}, within 1e-12 {close} (max abs diff {max_diff:e}), mismatches {bad}",
        args[1]
    );
    println!("time: python {t_py:.2}s, rust {t_rs:.3}s");
}
