//! Replays the small differential fixture produced by `gen/dump_cases.py`
//! (selected by `gen/make_fixture.py`) against the Rust port.

use serde_json::Value;
use std::io::BufRead;

fn run(f: &str, input: &Value) -> Result<Value, pysoup::Error> {
    let s = || input.as_str().unwrap();
    match f {
        "unescape" => Ok(Value::String(pysoup::html_unescape(s()))),
        "soup" => pysoup::soup_str(s()).map(Value::String),
        "norm" => pysoup::try_normalized_html_table(s()).map(Value::String),
        "norm_rel" => pysoup::try_normalized_html_table_relation(s()).map(Value::String),
        "td" => pysoup::try_td_strings(s()).map(|v| Value::Array(v.into_iter().map(Value::String).collect())),
        "merge" => {
            let v: Vec<String> = input.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
            pysoup::merge_tables(&v).map(Value::String)
        }
        _ => panic!("unknown function {}", f),
    }
}

#[test]
fn fixture_matches_python() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/fixture.jsonl");
    let fh = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut n = 0;
    let mut bad = Vec::new();
    for line in fh.lines() {
        let rec: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let f = rec["f"].as_str().unwrap();
        let got = run(f, &rec["in"]);
        let ok = match (&got, rec.get("out")) {
            (Ok(v), Some(want)) => v == want,
            (Err(_), None) => rec.get("err").is_some(),
            _ => false,
        };
        n += 1;
        if !ok {
            bad.push(format!("{} {} -> want {:?} got {:?}", f, rec["in"], rec.get("out").or(rec.get("err")), got));
        }
    }
    assert!(n > 100, "fixture too small: {}", n);
    assert!(bad.is_empty(), "{} of {} mismatches:\n{}", bad.len(), n, bad.join("\n"));
}

#[test]
fn spot_checks() {
    assert_eq!(pysoup::html_unescape("&notit; &amp &#x41;&#128;&#0;"), "¬it; & A€\u{fffd}");
    assert_eq!(pysoup::soup_str("<br><br/>x").unwrap(), "<br/><br>x</br>");
    assert_eq!(pysoup::td_strings("<td> a $\\cdot$</td><td><b>b</b></td><td></td>"), vec!["a", "b"]);
    assert_eq!(
        pysoup::normalized_html_table("<table><thead><tr><th>a</th></tr></thead></table>"),
        "<html><body><table border=\"1\" ><tr><td>a</td></tr></table></body></html>"
    );
}
