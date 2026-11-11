//! Debug helper: prints `debug_parse` of each argument (and the compat rewrite).
fn main() {
    for a in std::env::args().skip(1) {
        match teds::debug_parse(&a) {
            Err(e) => println!("{e}"),
            Ok((n, nodes)) => {
                println!("n={n}");
                for (d, tag, c, r, toks) in nodes {
                    println!("{}{tag} {c:?} {r:?} {}", "  ".repeat(d), toks.map(|t| t.concat()).unwrap_or_default());
                }
            }
        }
    }
}
