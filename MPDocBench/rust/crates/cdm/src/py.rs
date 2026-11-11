//! Small Python semantics helpers (str whitespace, list indexing/slicing, os.path.join,
//! round(x, 3), json.dumps).

/// Python exception raised inside the ported code (message only; the type rarely matters).
#[derive(Debug, Clone)]
pub struct PyErr(pub String);

impl PyErr {
    pub fn new(s: impl Into<String>) -> PyErr {
        PyErr(s.into())
    }
}

pub type PyResult<T> = Result<T, PyErr>;

/// `str.isspace()` for one character (Python 3.11, Unicode 14).
pub fn is_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\x0b' | '\x0c' | '\r' | '\x1c' | '\x1d' | '\x1e' | '\x1f' | ' ' | '\u{85}' | '\u{a0}'
            | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}'
    )
}

/// `str.strip()`
pub fn strip(s: &str) -> &str {
    s.trim_matches(is_space)
}

/// `str.split()` (no argument)
pub fn split_ws(s: &str) -> Vec<&str> {
    s.split(is_space).filter(|t| !t.is_empty()).collect()
}

/// Python list index `l[i]` with negative wrap; IndexError otherwise.
pub fn idx<T>(v: &[T], i: i64) -> PyResult<&T> {
    let n = v.len() as i64;
    let j = if i < 0 { i + n } else { i };
    if j < 0 || j >= n {
        return Err(PyErr::new("IndexError: list index out of range"));
    }
    Ok(&v[j as usize])
}

fn clamp_bound(i: i64, n: i64) -> i64 {
    if i < 0 {
        (i + n).max(0)
    } else {
        i.min(n)
    }
}

/// Python slice `l[a:b]` (step 1).
pub fn slice<T: Clone>(v: &[T], a: i64, b: i64) -> Vec<T> {
    let n = v.len() as i64;
    let a = clamp_bound(a, n);
    let b = clamp_bound(b, n);
    if a >= b {
        Vec::new()
    } else {
        v[a as usize..b as usize].to_vec()
    }
}

/// `l[a:]`
pub fn slice_from<T: Clone>(v: &[T], a: i64) -> Vec<T> {
    slice(v, a, v.len() as i64)
}

/// `l[:b]`
pub fn slice_to<T: Clone>(v: &[T], b: i64) -> Vec<T> {
    slice(v, 0, b)
}

/// `os.path.join(a, b)` (POSIX)
pub fn path_join(a: &str, b: &str) -> String {
    if b.starts_with('/') {
        return b.to_string();
    }
    if a.is_empty() || a.ends_with('/') {
        format!("{a}{b}")
    } else {
        format!("{a}/{b}")
    }
}

/// Python `round(x, 3)` for a finite float: the double nearest to the correctly rounded
/// (round-half-even on exact binary ties) 3-digit decimal.
pub fn round3(x: f64) -> f64 {
    format!("{x:.3}").parse().unwrap()
}

/// `json.dumps(s, ensure_ascii=False)` for a str.
pub fn json_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round3_half_even_and_nearest() {
        // exact binary ties
        assert_eq!(round3(0.0625), 0.062);
        assert_eq!(round3(0.1875), 0.188);
        assert_eq!(round3(2.0 / 3.0), 0.667);
        assert_eq!(round3(0.0005), 0.001); // 0.0005 is slightly above the tie in binary
        assert_eq!(round3(1.0), 1.0);
    }

    #[test]
    fn slices() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(slice(&v, 1, -1), vec![2, 3]);
        assert_eq!(slice(&v, 0, 0), Vec::<i32>::new());
        assert_eq!(slice_from(&v, -1), vec![4]);
        assert_eq!(slice(&v, 3, 1), Vec::<i32>::new());
        assert_eq!(*idx(&v, -1).unwrap(), 4);
        assert!(idx(&v, 4).is_err());
    }
}

#[cfg(test)]
mod round_tests {
    /// Python `round(n/m, 3)` and `round(2n/(n+m), 3)` for small n, m.
    #[test]
    fn round3_matches_python() {
        let want: Vec<f64> = serde_json::from_str(include_str!("../tests/data/round_ref.json")).unwrap();
        let mut k = 0;
        for n in 0..120u32 {
            for m in 1..120u32 {
                assert_eq!(super::round3(n as f64 / m as f64), want[k], "{n}/{m}");
                assert_eq!(super::round3(2.0 * n as f64 / (m + n) as f64), want[k + 1]);
                k += 2;
            }
        }
    }
}
