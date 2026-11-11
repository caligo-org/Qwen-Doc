//! Compare the emulated Python stack depth with measurements from
//! Python (`need` = smallest recursion-limit headroom, relative to the caller
//! of latex_to_text, that avoids RecursionError).
//!
//! Input: JSONL lines {"i": input, "o": "<need>", "k": "<exception or None>"}.
//! Prints `need - emulated` per input (the calibration offset should be constant).

#[path = "../tests/support/jsonl.rs"]
mod jsonl;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: depth_cmp depth.jsonl");
    let (cases, _) = jsonl::read_cases(&path);
    let mut hist = std::collections::BTreeMap::new();
    for c in &cases {
        let need: i64 = c.expected.as_ref().unwrap().parse().unwrap();
        let (d, _) = pylatexenc::py_max_depth(&c.input, 1);
        let diff = need - d as i64;
        *hist.entry(diff).or_insert(0) += 1;
        println!(
            "{:>4} {:>5} {:>5} {}",
            diff,
            need,
            d,
            jsonl::quote(&c.input.chars().take(80).collect::<String>())
        );
    }
    eprintln!("histogram of need - emulated: {:?}", hist);
}
