//! Differential test of the Rust KaTeX tokenizer port against node:
//!
//!     tokdiff CORPUS.json [NODE]
//!
//! CORPUS is a JSON array of strings, each one the stdin text of one `node
//! preprocess_formula.js normalize` / `node preprocess_tabular.js tokenize` call. Both
//! scripts run on every entry. Entries without `\r` / `\n` are batched through one node
//! process per script (one input line gives one output line; the scripts keep no state
//! across lines); the others run one node process each. Prints the mismatch count per
//! script and the first mismatches.

use std::io::Write;
use std::process::{Command, Stdio};

use cdm::katex::{run, Script};

fn node_run(node: &str, script: Script, input: &str) -> String {
    let dir = cdm::__tokenize_dir();
    let (js, mode) = match script {
        Script::FormulaNormalize => ("preprocess_formula.js", "normalize"),
        Script::TabularTokenize => ("preprocess_tabular.js", "tokenize"),
    };
    let mut child = Command::new(node)
        .arg(format!("{dir}/{js}"))
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn node");
    let mut stdin = child.stdin.take().unwrap();
    let data = input.as_bytes().to_vec();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&data);
    });
    let out = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(out.status.success(), "node failed");
    String::from_utf8(out.stdout).expect("node output is UTF-8")
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw: Vec<String> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let node = a.get(2).cloned().unwrap_or_else(|| "node".into());
    let fuzz: usize = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(0);
    let corpus = build_corpus(&raw, fuzz);
    for script in [Script::FormulaNormalize, Script::TabularTokenize] {
        let (single, multi): (Vec<&String>, Vec<&String>) =
            corpus.iter().partition(|s| !s.contains('\n') && !s.contains('\r') && !s.is_empty());
        let mut bad = 0usize;
        let mut empty_out = 0usize;
        let mut shown = 0;
        let mut report = |inp: &str, want: &str, got: &str| {
            bad += 1;
            if shown < 15 {
                shown += 1;
                let cut = |s: &str| -> String { if s.chars().count() > 300 { format!("{}...<{} chars>", s.chars().take(120).collect::<String>(), s.chars().count()) } else { format!("{s:?}") } };
                println!("MISMATCH {script:?}\n  in:   {}\n  node: {}\n  rust: {}", cut(inp), cut(want), cut(got));
            }
        };
        // batched
        let t = std::time::Instant::now();
        let mut joined = String::new();
        for s in &single {
            joined.push_str(s);
            joined.push('\n');
        }
        let want = node_run(&node, script, &joined);
        let tn = t.elapsed();
        let want: Vec<&str> = want.split_terminator('\n').collect();
        assert_eq!(want.len(), single.len(), "node line count");
        let t = std::time::Instant::now();
        let mut got_all = Vec::with_capacity(single.len());
        for s in &single {
            got_all.push(run(s, script));
        }
        let tr = t.elapsed();
        for ((s, wnt), got) in single.iter().zip(&want).zip(&got_all) {
            let got = got.strip_suffix('\n').unwrap_or(got);
            if wnt.is_empty() {
                empty_out += 1;
            }
            if *wnt != got {
                report(s, wnt, got);
            }
        }
        for s in &multi {
            let wnt = node_run(&node, script, s);
            let got = run(s, script);
            if wnt != got {
                report(s, &wnt, &got);
            }
        }
        println!(
            "{script:?}: cases {} (batched {}, single {}), mismatches {bad}, node empty (error) outputs {empty_out}; node {:.1}s, rust {:.2}s",
            corpus.len(),
            single.len(),
            multi.len(),
            tn.as_secs_f64(),
            tr.as_secs_f64()
        );
    }
}

/// The corpus: for each raw formula the stdin text the pipeline gives node, the raw
/// string itself, `fuzz` random mutations per formula, and hand-written edge cases.
fn build_corpus(raw: &[String], fuzz: usize) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut add = |s: String, out: &mut Vec<String>| {
        if seen.insert(s.clone()) {
            out.push(s);
        }
    };
    for r in raw {
        if let Some((_, s)) = cdm::__node_stdin(r) {
            add(s, &mut out);
        }
        add(r.clone(), &mut out);
    }
    for s in EDGE {
        add(s.to_string(), &mut out);
        if let Some((_, p)) = cdm::__node_stdin(s) {
            add(p, &mut out);
        }
    }
    let mut rng: u64 = 0x9E3779B97F4A7C15;
    let mut next = move |n: usize| -> usize {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        (rng % n.max(1) as u64) as usize
    };
    let base: Vec<String> = out.clone();
    for k in 0..fuzz {
        for (bi, b) in base.iter().enumerate() {
            // line breaks only rarely: such inputs run one node process each
            let keep_breaks = (bi + k) % 97 == 0;
            let mut u: Vec<char> =
                b.chars().map(|c| if !keep_breaks && (c == '\n' || c == '\r') { ' ' } else { c }).collect();
            let ops = 1 + next(3);
            for _ in 0..ops {
                let p = next(u.len() + 1);
                match next(6) {
                    0 if !u.is_empty() => {
                        let n = 1 + next(3);
                        let e = (p + n).min(u.len());
                        u.drain(p.min(u.len())..e);
                    }
                    1 => u.truncate(p),
                    2 if p < u.len() => {
                        let e = (p + 1 + next(8)).min(u.len());
                        let dup: Vec<char> = u[p..e].to_vec();
                        let q = next(u.len() + 1);
                        u.splice(q..q, dup);
                    }
                    _ => {
                        let mut sn = SNIPPETS[next(SNIPPETS.len())];
                        if (sn.contains('\n') || sn.contains('\r')) && !keep_breaks {
                            sn = " ";
                        }
                        u.splice(p..p, sn.chars());
                    }
                }
            }
            add(u.into_iter().collect(), &mut out);
        }
    }
    out
}

const SNIPPETS: &[&str] = &[
    "{", "}", "{}", "\\\\", "&", "^", "_", "'", "''", "\\left(", "\\right)", "\\left.", "\\right\\|", "\\begin{array}{cc}",
    "\\begin{array}{c|l}", "\\end{array}", "\\begin{matrix}", "\\end{matrix}", "\\begin{pmatrix}", "\\end{pmatrix}",
    "\\begin{aligned}", "\\end{aligned}", "\\begin{cases}", "\\end{cases}", "\\begin{tabular}{|c|r|}", "\\end{tabular}",
    "\\hline", "\\matrix{", "\\pmatrix{", "\\cases{", "\\over", "\\choose", "\\limits", "\\nolimits", "\\rule{1em}{2pt}",
    "\\rule[-1.5ex]{.5em}{3 pt}", "\\color{red}", "\\color{#ff0}", "\\blue{x}", "\\text{", "\\text{a b}", "\\mbox{",
    "\\sqrt[", "\\sqrt[3]{x}", "]", "[", "$", "$$", "%", "\\%", "\\label{eq1}", "\\label{", "\\>", "\\~", "\\\\[2pt]",
    "\\\\[x]", "\\cr", "\\rm", "{\\rm ", "{ \\rm x}", "\\rm{", "\\mathrm{", "\\mathrm{a b c}", "\\operatorname{sin}",
    "\\operatorname*{arg\\,max}", "\\sin", "\\lim", "\\int", "\\sum", "\\displaystyle", "\\scriptstyle", "\\Large",
    "\\bf", "\\it", "\\cal", "\\frac", "\\frac{a}{b}", "\\dfrac12", "\\binom", "\\tbinom{n}{k}", "\\hat", "\\vec{x}",
    "\\bar", "\\overline{", "\\underline", "\\llap{x}", "\\rlap", "\\phantom{x}", "\\phantom", "\\KaTeX", "\\big(",
    "\\Bigr]", "\\bigl\\{", "\\biggm|", "\\Bigg", "\\mathbb{R}", "\\Bbb", "\\bold{x}", "\\frak", "\\mathcal",
    "\\hspace{1cm}", "\\hskip 2pt", "\\hskip1em", "\\quad", "\\,", r"\;","\\!", "\\ ", "~", "\\tag{1}", "\\nonumber",
    "\\notag", "\\end", "\\begin", "\\begin{foo}", "\\end{foo}", "\\begin{split}", "\\end{split}", "\\begin{align*}",
    "\\end{align*}", "\\begin{smallmatrix}", "\\end{smallmatrix}", " ", "  ", "\t", "\u{a0}", "\u{2028}", "\u{3000}",
    "\u{1d465}", "\u{1f600}", "é", "中文", "\u{e000}", "\u{1}", "\u{b}", "#", "--", "---", "\\", "\\\\\\", "\\-",
    "\\alpha", "\\matrix", "\\cases", "SSSSSS", " S S S S S S", "\\label { x }", "\\mathrm{\u{1d465}}", "\\text{\u{1f600}}",
    "\\begin{array}{{}c}", "\\begin{array}c", "\\begin{array}{c{c}}", "\\begin{{}array}", "\\begin{ array}",
    "\\color{", "\\rule{x}{y}", "\\sqrt", "\\left", "\\right", "\\\\\\\\", "\\cr\\cr", "&&", "^^", "__", "\\prime",
    "\\mathop{x}", "\\stackrel{a}{=}", "\\fbox{x}", "\\textrm{x}", "\\boldmath", "\\siptstyle", "\\vbox{x}", "\r", "\n",
    "\r\n",
];

const EDGE: &[&str] = &[
    "x^2", "\\frac{a}{b}", "\\sqrt", "", " ", "%", "%x", "%%", "a%b", "\\%", "\\\\%", "$x$", "\\label{a}x\\label{b}",
    "\\label{a\u{2028}}x", "\\>\\>\\>", "\\~x", "\\\\\\\\", "a\\\\b\\\\c", "\\begin{array}{cc}a&b\\\\c&d\\end{array}",
    "\\begin{tabular}{|c|c|}\\hline a & b \\\\ \\hline c & d \\\\ \\hline\\end{tabular}", "\\begin{tabular}{cc} a&b",
    "\\begin{tabular}{l} a \\hskip 1cm b \\hskip2pt c\\end{tabular}", "\\begin{tabular}{c} 50\\% % comment\\end{tabular}",
    "{\\rm abc}", "{ \\rm abc}", "\\rm{abc}", "\\mathrm{a b}", "\\mathrm{\\alpha\\beta}", "\\text{a b}", "a \\over b",
    "a \\over b \\over c", "{a \\choose b}", "\\sum\\limits_{i=1}^n", "\\int\\nolimits_0^1", "x\\limits", "\\sin\\limits_x",
    "\\rule{1.50em}{-.5pt}", "\\rule{1.em}{100000000000000000000000pt}", "\\rule{-0em}{0.000000123pt}",
    "\\color{red}{x}", "\\left(\\frac{a}{b}\\right)", "\\left\\{x\\right.", "\\left x \\right)", "\\bigl( x \\bigr)",
    "\\begin{matrix}a&b\\\\c&d\\end{matrix}", "\\begin{pmatrix}1\\end{pmatrix}", "\\begin{cases}a&b\\\\c&d\\end{cases}",
    "\\begin{aligned}a&=b\\\\&=c\\end{aligned}", "\\begin{align}a&=b\\end{align}", "\\begin{split}x\\end{split}",
    "\\begin{smallmatrix}a\\end{smallmatrix}", "\\matrix{a&b\\cr c&d}", "\\pmatrix{a}", "\\cases{a&b}", "x''", "x'^2",
    "a^b^c", "a_b_c", "^2", "_x", "x^", "\\frac{a}\\sqrt{b}", "\\frac\\sqrt12", "\\sqrt[3]", "\\sqrt[3]\\frac12",
    "\\text\\frac12", "\\overline\\frac12", "\\hat\\frac12", "\\mathbf\\text{x}", "\\begin{array}\\frac12 a\\end{array}",
    "\\begin\\frac{a}{b}", "\\begin{matrix}a\\end{pmatrix}", "\\displaystyle a \\over b", "\\Large a \\over b",
    "a\rb", "a\r\nb", "a\nb", "\ra", "a\r", "\u{feff}x", "\u{1d465}", "\\mathrm{\u{1d465}y}", "\\alpha\u{2028}",
    "\\\u{1f600}", "\u{e000}", "\\text{\u{2028}}", "\\phantom{x}", "\\KaTeX", "\\llap{a}", "\\rlap", "\\underline{a}",
    "\\operatorname{sin} x", "\\operatorname*{lim}", "SSSSSS", "a SSSSSS b S S S S S S", "\\label { a } b",
    "\\begin{array}{{}c}a&b\\end{array}", "\\begin{array}{c{c}}a\\end{array}", "\\begin{{}array}{c}a\\end{array}",
    "\\begin{tabular}{l}\\rule{1pt}{2pt}\\end{tabular}", "\\begin{tabular}{l}a\\\\[3pt]b\\end{tabular}",
];
