//! Replays JSONL cases written by `gen/dump_cases.py` against the Rust port.
//!
//!     cargo run --release -p pysoup --example replay -- [--show N] [--dump-mismatches FILE] cases/*.jsonl

use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{BufRead, Write};

/// Ok(output as JSON) or Err(exception)
pub fn run(f: &str, input: &Value) -> Result<Value, String> {
    let s = || input.as_str().unwrap();
    let r = match f {
        "unescape" => Ok(Value::String(pysoup::html_unescape(s()))),
        "soup" => pysoup::soup_str(s()).map(Value::String),
        "norm" => pysoup::try_normalized_html_table(s()).map(Value::String),
        "norm_rel" => pysoup::try_normalized_html_table_relation(s()).map(Value::String),
        "td" => pysoup::try_td_strings(s()).map(|v| Value::Array(v.into_iter().map(Value::String).collect())),
        "merge" => {
            let v: Vec<String> = input.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
            pysoup::merge_tables(&v).map(Value::String)
        }
        "nfkc" => Ok(Value::String(pysoup::pystr::nfkc(s()))),
        "lower" => Ok(Value::String(pysoup::pystr::lower(s()))),
        "isspace" => Ok(Value::String(
            (if pysoup::pystr::is_space(s().chars().nth(1).unwrap()) { "True" } else { "False" }).to_string(),
        )),
        _ => panic!("unknown function {}", f),
    };
    r.map_err(|e| e.to_string())
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut show = 3usize;
    let mut dump: Option<std::fs::File> = None;
    let mut files = Vec::new();
    while !args.is_empty() {
        let a = args.remove(0);
        if a == "--show" {
            show = args.remove(0).parse().unwrap();
        } else if a == "--dump-mismatches" {
            dump = Some(std::fs::File::create(args.remove(0)).unwrap());
        } else {
            files.push(a);
        }
    }
    let mut total: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut shown = 0;
    for path in files {
        let fh = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let base = std::path::Path::new(&path).file_name().unwrap().to_string_lossy().to_string();
        for line in fh.lines() {
            let line = line.unwrap();
            let rec: Value = serde_json::from_str(&line).unwrap();
            let f = rec["f"].as_str().unwrap();
            let input = &rec["in"];
            let got = std::panic::catch_unwind(|| run(f, input));
            let ok = match (&got, rec.get("out"), rec.get("err")) {
                (Ok(Ok(v)), Some(want), None) => v == want,
                (Ok(Err(_)), None, Some(_)) => true,
                _ => false,
            };
            let e = total.entry(format!("{}:{}", base, f)).or_default();
            e.0 += 1;
            if !ok {
                e.1 += 1;
                let gotv = match &got {
                    Ok(Ok(v)) => serde_json::json!({"out": v}),
                    Ok(Err(m)) => serde_json::json!({"err": m}),
                    Err(_) => serde_json::json!({"err": "PANIC"}),
                };
                if shown < show {
                    shown += 1;
                    eprintln!("--- MISMATCH {} {}\n in:   {}\n want: {}\n got:  {}", base, f, input,
                        rec.get("out").or(rec.get("err")).unwrap(), gotv);
                }
                if let Some(d) = dump.as_mut() {
                    writeln!(d, "{}", serde_json::json!({"file": base, "f": f, "in": input, "want": rec, "got": gotv})).unwrap();
                }
            }
        }
    }
    let mut all = (0, 0);
    for (k, (n, m)) in &total {
        println!("{:40} cases {:8}  mismatches {:6}", k, n, m);
        all.0 += n;
        all.1 += m;
    }
    println!("{:40} cases {:8}  mismatches {:6}", "TOTAL", all.0, all.1);
}
