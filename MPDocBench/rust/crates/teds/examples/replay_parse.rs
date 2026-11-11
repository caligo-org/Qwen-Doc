//! Compares the parse (what `load_html_tree` sees) with Python's dump:
//! `cargo run --release --example replay_parse -- DUMP.jsonl [--show N]`
//! Each line: {"html": str, "r": "ERR:<type>" | "NOTABLE" | {"n": int, "nodes": [[depth, tag, colspan, rowspan, tokens|null]]}}
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let show: usize = args
        .iter()
        .position(|a| a == "--show")
        .map(|i| args[i + 1].parse().unwrap())
        .unwrap_or(5);
    let text = std::fs::read_to_string(&args[1]).unwrap();
    let (mut n, mut bad, mut skipped) = (0, 0, 0);
    for (lineno, line) in text.lines().enumerate() {
        let Ok(d) = serde_json::from_str::<Value>(line) else {
            skipped += 1;
            continue;
        };
        n += 1;
        let html = d["html"].as_str().unwrap();
        let got = match teds::debug_parse(html) {
            Err(e) if e.starts_with("ERR") => json!("ERR"),
            Err(e) => json!(e),
            Ok((cnt, nodes)) => json!({"n": cnt, "nodes": nodes}),
        };
        let mut want = d["r"].clone();
        if want.as_str().is_some_and(|s| s.starts_with("ERR")) {
            want = json!("ERR");
        }
        if got != want {
            bad += 1;
            if bad <= show {
                println!("MISMATCH line {lineno}: html={:?}", html.chars().take(400).collect::<String>());
                if let (Some(g), Some(w)) = (got["nodes"].as_array(), want["nodes"].as_array()) {
                    println!("  n rust={} python={}", got["n"], want["n"]);
                    for (k, (a, b)) in g.iter().zip(w.iter()).enumerate() {
                        if a != b {
                            println!("  first diff at node {k}:\n    rust   {a}\n    python {b}");
                            break;
                        }
                    }
                    if g.len() != w.len() {
                        println!("  node count rust={} python={}", g.len(), w.len());
                    }
                } else {
                    println!("  rust={} python={}", got.to_string().chars().take(200).collect::<String>(), want.to_string().chars().take(200).collect::<String>());
                }
            }
        }
    }
    println!("{}: {n} documents ({skipped} skipped), {bad} parse mismatches", args[1]);
}
