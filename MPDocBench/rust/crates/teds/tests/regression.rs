//! Replays Python references (lxml 5.3.0 / libxml2 2.12.9, apted 1.0.3, Levenshtein 0.25.1)
//! kept in `tests/fixtures`. Scores must be bit-identical; "ERR:<type>" means Python raised.
use serde_json::Value;
use teds::{head_teds, teds, HeadId, HeadNode};

fn lines(name: &str) -> Vec<Value> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

#[test]
fn table_teds_matches_python() {
    let mut bad = Vec::new();
    for (k, d) in lines("tables.jsonl").iter().enumerate() {
        let (pred, gt) = (d["pred"].as_str().unwrap(), d["gt"].as_str().unwrap());
        for (key, so) in [("teds", false), ("teds_s", true)] {
            let got = teds(pred, gt, so);
            let ok = match (&got, d[key].as_f64()) {
                (Ok(g), Some(w)) => *g == w,
                (Err(_), None) => true,
                _ => false,
            };
            if !ok {
                bad.push(format!("case {k} {key}: rust {got:?} python {}", d[key]));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

fn id(v: &Value) -> HeadId {
    match v {
        Value::String(s) if s == "root" => HeadId::Root,
        v => HeadId::Int(v.as_i64().unwrap()),
    }
}

#[test]
fn head_teds_matches_python() {
    let mut bad = Vec::new();
    for (k, d) in lines("heads.jsonl").iter().enumerate() {
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
        let got = head_teds(&pred, &rels, &nodes);
        let ok = match (&got, d["ref"].as_array()) {
            (Ok((s, dep)), Some(w)) => {
                *s == w[0].as_f64().unwrap() && *dep == w[1].as_i64().unwrap()
            }
            (Err(_), None) => true,
            _ => false,
        };
        if !ok {
            bad.push(format!(
                "case {k} ({}): rust {got:?} python {}",
                d["img"], d["ref"]
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
