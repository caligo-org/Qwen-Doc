//! Test driver: reads a JSON list of strings, writes [[rgb_latex, tokens] | {"err": ..}].
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let items: Vec<String> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let out: Vec<serde_json::Value> = items
        .iter()
        .map(|s| match cdm::__normalize_and_color(s) {
            Ok((l, t)) => serde_json::json!([l, t]),
            Err(_) => serde_json::json!({"err": true}),
        })
        .collect();
    std::fs::write(&a[2], serde_json::to_string(&out).unwrap()).unwrap();
}
