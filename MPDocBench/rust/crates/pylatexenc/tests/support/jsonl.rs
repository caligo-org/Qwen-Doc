//! Minimal reader for the differential-test JSONL files: one flat JSON object
//! per line whose values are strings (keys "i", "o", "e", "k").
#![allow(dead_code)]

/// One case: input, expected output (`Ok`) or Python exception class (`Err`).
pub struct Case {
    pub input: String,
    pub expected: Result<String, String>,
    pub kind: String,
}

fn parse_string(b: &[char], i: &mut usize) -> Option<String> {
    if b.get(*i) != Some(&'"') {
        return None;
    }
    *i += 1;
    let mut out = String::new();
    loop {
        let c = *b.get(*i)?;
        *i += 1;
        match c {
            '"' => return Some(out),
            '\\' => {
                let e = *b.get(*i)?;
                *i += 1;
                match e {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    '/' => out.push('/'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    'u' => {
                        let hex = |i: &mut usize| -> Option<u32> {
                            let s: String = b.get(*i..*i + 4)?.iter().collect();
                            *i += 4;
                            u32::from_str_radix(&s, 16).ok()
                        };
                        let u = hex(i)?;
                        if (0xD800..0xDC00).contains(&u) {
                            // surrogate pair
                            if b.get(*i) == Some(&'\\') && b.get(*i + 1) == Some(&'u') {
                                *i += 2;
                                let lo = hex(i)?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return None;
                                }
                                out.push(char::from_u32(
                                    0x10000 + ((u - 0xD800) << 10) + (lo - 0xDC00),
                                )?);
                            } else {
                                return None; // lone surrogate: not representable
                            }
                        } else {
                            out.push(char::from_u32(u)?);
                        }
                    }
                    _ => return None,
                }
            }
            c => out.push(c),
        }
    }
}

/// Parse one line; `None` if it is malformed or not representable in Rust.
pub fn parse_line(line: &str) -> Option<Case> {
    let b: Vec<char> = line.chars().collect();
    let mut i = 0;
    let ws = |i: &mut usize| {
        while *i < b.len() && b[*i].is_ascii_whitespace() {
            *i += 1;
        }
    };
    ws(&mut i);
    if b.get(i) != Some(&'{') {
        return None;
    }
    i += 1;
    let (mut inp, mut out, mut exc, mut kind) = (None, None, None, String::new());
    loop {
        ws(&mut i);
        if b.get(i) == Some(&'}') {
            break;
        }
        let k = parse_string(&b, &mut i)?;
        ws(&mut i);
        if b.get(i) != Some(&':') {
            return None;
        }
        i += 1;
        ws(&mut i);
        let v = parse_string(&b, &mut i)?;
        match k.as_str() {
            "i" => inp = Some(v),
            "o" => out = Some(v),
            "e" => exc = Some(v),
            "k" => kind = v,
            _ => {}
        }
        ws(&mut i);
        match b.get(i) {
            Some(',') => i += 1,
            Some('}') => break,
            _ => return None,
        }
    }
    let expected = match (out, exc) {
        (Some(o), None) => Ok(o),
        (None, Some(e)) => Err(e),
        _ => return None,
    };
    Some(Case {
        input: inp?,
        expected,
        kind,
    })
}

/// Read all cases of a JSONL file; returns (cases, number of skipped lines).
pub fn read_cases(path: &str) -> (Vec<Case>, usize) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {}", path, e));
    let mut cases = Vec::new();
    let mut skipped = 0;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match parse_line(line) {
            Some(c) => cases.push(c),
            None => skipped += 1,
        }
    }
    (cases, skipped)
}

/// JSON-escape a string (for reports).
pub fn quote(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}
