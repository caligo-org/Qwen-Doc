//! Timing of pathological inputs.
use std::time::Instant;
fn main() {
    let cases: Vec<(&str, String)> = vec![
        ("deep braces 100k", "{".repeat(100_000)),
        ("deep env 50k", "\\begin{a}".repeat(50_000)),
        ("dollars 100k", "$".repeat(100_000)),
        ("\\( 100k", "\\(".repeat(100_000)),
        ("flat groups 300k", "{a}".repeat(100_000)),
        ("frac chain", "\\frac12".repeat(50_000)),
        ("comments", "%c\n".repeat(50_000)),
        (
            "matrix",
            format!(
                "\\begin{{pmatrix}}{}\\end{{pmatrix}}",
                "a & b \\\\ ".repeat(20_000)
            ),
        ),
        ("long text", "word ".repeat(200_000)),
        ("unclosed verb", "\\verb|".repeat(2_000)),
    ];
    for (name, s) in cases {
        let t = Instant::now();
        let r = pylatexenc::latex_to_text(&s);
        println!(
            "{:<18} {:>8.3} ms  {}",
            name,
            t.elapsed().as_secs_f64() * 1e3,
            match &r {
                Ok(o) => format!("ok len {}", o.len()),
                Err(e) => format!("Err {}", e.kind),
            }
        );
    }
}
