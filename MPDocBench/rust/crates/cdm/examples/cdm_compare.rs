//! Scores formula pairs with two CDM backends and reports how the scores differ:
//!
//!     cdm_compare PAIRS.json WORKDIR [--a SPEC] [--b SPEC] [--jobs N] [--out OUT.json]
//!
//! PAIRS is `[{"gt": .., "pred": .., ...}, ...]` (raw strings, cleaned as
//! `_process_single_cdm_sample` cleans them; MarathonDocBench empty-prediction rule on).
//! SPEC is `native` (default for A), `external` (default for B: node, perl, xelatex,
//! magick on PATH) or a comma list of stage settings starting from native, e.g.
//! `tok=node,tex=tectonic,raster=magick,thr=12`. A "score" of `err` is a sample whose
//! evaluation raised (the scorer retries it and records 0.0).
//!
//! The external backend needs the toolchain the Python scorer ran with on PATH (node 20,
//! TeX Live 2024 xelatex, ImageMagick 7 with the `pngalpha` delegate via
//! MAGICK_CONFIGURE_PATH, Ghostscript).

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use cdm::{Backend, Raster, Tex, Tokenizer};
use serde_json::{json, Value};

fn parse_spec(s: &str) -> Backend {
    match s {
        "native" => return Backend::native(),
        "external" => return Backend::external(),
        _ => {}
    }
    let mut b = Backend::native();
    for kv in s.split(',') {
        let (k, v) = kv.split_once('=').unwrap_or_else(|| panic!("bad spec item {kv}"));
        match (k, v) {
            ("tok", "native") => b.tokenizer = Tokenizer::Native,
            ("tok", "node") => b.tokenizer = Tokenizer::External,
            ("tex", "tectonic") => b.tex = Tex::Tectonic,
            ("tex", "xelatex") => b.tex = Tex::Xelatex,
            ("raster", "native") => b.raster = Raster::Native,
            ("raster", "magick") => b.raster = Raster::Magick,
            ("thr", t) => b.coverage_threshold = t.parse().unwrap(),
            _ => panic!("bad spec item {kv}"),
        }
    }
    b
}

fn score(i: usize, gt: &str, pred: &str, root: &PathBuf, b: &Backend) -> (Option<f64>, f64) {
    let t = Instant::now();
    let r = cdm::cdm_sample_with(i, gt, pred, root, true, b);
    (r.ok().map(|s| s.score.as_f64()), t.elapsed().as_secs_f64())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pairs: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let work = PathBuf::from(&args[2]);
    let mut a = Backend::native();
    let mut b = Backend::external();
    let mut jobs = 4usize;
    let mut out: Option<String> = None;
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--a" => a = parse_spec(&args[i + 1]),
            "--b" => b = parse_spec(&args[i + 1]),
            "--jobs" => jobs = args[i + 1].parse().unwrap(),
            "--out" => out = Some(args[i + 1].clone()),
            x => panic!("unknown argument {x}"),
        }
        i += 2;
    }
    eprintln!("A = {a:?}\nB = {b:?}");
    let n = pairs.len();
    let results: Mutex<Vec<Option<Value>>> = Mutex::new(vec![None; n]);
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                if k >= n {
                    break;
                }
                let p = &pairs[k];
                let (gt, pred) = (p["gt"].as_str().unwrap(), p["pred"].as_str().unwrap());
                let (sa, ta) = score(k, gt, pred, &work.join("a"), &a);
                let (sb, tb) = score(k, gt, pred, &work.join("b"), &b);
                eprintln!("{k}: A {sa:?} ({ta:.2}s)  B {sb:?} ({tb:.2}s)");
                let v = json!({"idx": k, "kind": p.get("kind"), "a": sa, "b": sb, "time_a": ta, "time_b": tb, "gt": gt, "pred": pred});
                results.lock().unwrap()[k] = Some(v);
            });
        }
    });
    let results: Vec<Value> = results.into_inner().unwrap().into_iter().map(|v| v.unwrap()).collect();
    let mut differ = 0;
    let mut sum_abs = 0.0;
    let mut max_abs: f64 = 0.0;
    let mut err_mismatch = 0;
    let (mut ta, mut tb) = (0.0, 0.0);
    let mut by_kind: std::collections::BTreeMap<String, (usize, usize, f64, f64)> = Default::default();
    for r in &results {
        ta += r["time_a"].as_f64().unwrap();
        tb += r["time_b"].as_f64().unwrap();
        // a raising sample ends up 0.0 in the scorer
        let sa = r["a"].as_f64();
        let sb = r["b"].as_f64();
        if sa.is_none() != sb.is_none() {
            err_mismatch += 1;
        }
        let (x, y) = (sa.unwrap_or(0.0), sb.unwrap_or(0.0));
        let d = (x - y).abs();
        if d > 0.0 {
            differ += 1;
        }
        sum_abs += d;
        max_abs = max_abs.max(d);
        let kind = r["kind"].as_str().unwrap_or("-").to_string();
        let e = by_kind.entry(kind).or_default();
        e.0 += 1;
        if d > 0.0 {
            e.1 += 1;
        }
        e.2 += x;
        e.3 += y;
    }
    println!("pairs {n}: scores differ in {differ}, mean |A-B| {:.4}, max |A-B| {:.3}, raised in one backend only {err_mismatch}", sum_abs / n as f64, max_abs);
    for (k, (c, d, x, y)) in &by_kind {
        println!("  {k}: {c} pairs, {d} differ, mean A {:.4}, mean B {:.4}", x / *c as f64, y / *c as f64);
    }
    println!("time per pair: A {:.2}s, B {:.2}s (wall clock per call, {jobs} threads)", ta / n as f64, tb / n as f64);
    if let Some(o) = out {
        std::fs::write(o, serde_json::to_string_pretty(&results).unwrap()).unwrap();
    }
}
