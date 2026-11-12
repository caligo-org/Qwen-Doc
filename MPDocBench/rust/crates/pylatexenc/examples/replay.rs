//! Replay a differential corpus produced against Python pylatexenc 2.10 and
//! report mismatches.
//!
//! Usage: cargo run --release -p pylatexenc --example replay -- corpus.jsonl [max_shown]
//!
//! Each line: {"i": input, "o": expected output} or {"i": input, "e": "ExcClass"}.

#[path = "../tests/support/jsonl.rs"]
mod jsonl;

use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("usage: replay corpus.jsonl [max_shown]");
    let max_shown: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(20);
    let (cases, skipped) = jsonl::read_cases(path);
    let t0 = Instant::now();
    let mut mismatches = 0usize;
    let mut per_kind: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut n_err = 0usize;
    for c in &cases {
        let got = pylatexenc::latex_to_text(&c.input);
        let ok = match (&got, &c.expected) {
            (Ok(g), Ok(e)) => g == e,
            (Err(_), Err(_)) => true,
            _ => false,
        };
        if c.expected.is_err() {
            n_err += 1;
        }
        let e = per_kind.entry(c.kind.clone()).or_default();
        e.0 += 1;
        if !ok {
            e.1 += 1;
            mismatches += 1;
            if mismatches <= max_shown {
                println!("MISMATCH [{}] input={}", c.kind, jsonl::quote(&c.input));
                match &c.expected {
                    Ok(o) => println!("  python: {}", jsonl::quote(o)),
                    Err(e) => println!("  python: raises {}", e),
                }
                match &got {
                    Ok(o) => println!("  rust:   {}", jsonl::quote(o)),
                    Err(e) => println!("  rust:   Err({})", e),
                }
            }
        }
    }
    let dt = t0.elapsed();
    for (k, (n, m)) in &per_kind {
        println!("{:>14}: {:>7} cases, {:>5} mismatches", k, n, m);
    }
    println!(
        "total: {} cases ({} expected exceptions, {} unreadable lines skipped), {} mismatches, {:.3}s",
        cases.len(),
        n_err,
        skipped,
        mismatches,
        dt.as_secs_f64()
    );
    if mismatches > 0 {
        std::process::exit(1);
    }
}
