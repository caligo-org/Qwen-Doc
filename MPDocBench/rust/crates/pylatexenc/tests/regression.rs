//! Regression fixture: cases recorded from Python 3.11 + pylatexenc 2.10
//! (`LatexNodes2Text().latex_to_text(s)`), see tools/make_fixture.py.
//! Set PYLATEXENC_CORPUS=/path/corpus.jsonl to also replay a full corpus.

mod support;
use support::jsonl;

fn replay(path: &str) -> (usize, Vec<String>) {
    let (cases, skipped) = jsonl::read_cases(path);
    assert_eq!(skipped, 0, "unreadable lines in {}", path);
    let mut bad = Vec::new();
    for c in &cases {
        let got = pylatexenc::latex_to_text(&c.input);
        let ok = match (&got, &c.expected) {
            (Ok(g), Ok(e)) => g == e,
            (Err(_), Err(_)) => true,
            _ => false,
        };
        if !ok {
            bad.push(format!(
                "input={} expected={:?} got={:?}",
                jsonl::quote(&c.input),
                c.expected,
                got
            ));
        }
    }
    (cases.len(), bad)
}

#[test]
fn regression_fixture() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/regression.jsonl"
    );
    let (n, bad) = replay(path);
    assert!(n >= 100, "fixture too small: {}", n);
    assert!(
        bad.is_empty(),
        "{} mismatches:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

#[test]
fn full_corpus_if_given() {
    let Ok(path) = std::env::var("PYLATEXENC_CORPUS") else {
        return;
    };
    let (n, bad) = replay(&path);
    assert!(
        bad.is_empty(),
        "{} of {} mismatch:\n{}",
        bad.len(),
        n,
        bad[..bad.len().min(20)].join("\n")
    );
}
