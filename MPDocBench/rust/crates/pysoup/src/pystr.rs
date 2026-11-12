//! Python `str` semantics needed by the port (Unicode 14.0 as in Python 3.11).

use crate::generated::{PY_ALNUM_RANGES, PY_DECIMAL_RANGES, PY_LOWER, PY_UNASSIGNED_RANGES};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `str.isspace()` (also Python `re` `\s` for str patterns).
#[inline]
pub fn is_space(c: char) -> bool {
    matches!(
        c as u32,
        0x09..=0x0d
            | 0x1c..=0x20
            | 0x85
            | 0xa0
            | 0x1680
            | 0x2000..=0x200a
            | 0x2028
            | 0x2029
            | 0x202f
            | 0x205f
            | 0x3000
    )
}

/// `str.strip()` without arguments.
pub fn strip(s: &str) -> &str {
    s.trim_matches(is_space)
}

/// `str.split()` without arguments, joined with "" (i.e. remove all whitespace).
pub fn remove_space(s: &str) -> String {
    s.chars().filter(|&c| !is_space(c)).collect()
}

fn in_ranges(c: u32, ranges: &[(u32, u32)]) -> bool {
    ranges.binary_search_by(|&(a, b)| {
        if b < c {
            std::cmp::Ordering::Less
        } else if a > c {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }).is_ok()
}

/// `str.isalnum()` for one character.
pub fn is_alnum(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphanumeric();
    }
    in_ranges(c as u32, PY_ALNUM_RANGES)
}

/// Python `re` `\w` (Unicode str pattern).
pub fn is_word(c: char) -> bool {
    c == '_' || is_alnum(c)
}

/// Unassigned (category Cn) in Unicode 14.0.
pub fn is_unassigned_14(c: char) -> bool {
    in_ranges(c as u32, PY_UNASSIGNED_RANGES)
}

fn lower_map() -> &'static HashMap<char, &'static str> {
    static M: OnceLock<HashMap<char, &'static str>> = OnceLock::new();
    M.get_or_init(|| PY_LOWER.iter().copied().collect())
}

/// `str.lower()`.
pub fn lower(s: &str) -> String {
    if s.is_ascii() {
        return s.to_ascii_lowercase();
    }
    let m = lower_map();
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii() {
            out.push(c.to_ascii_lowercase());
        } else if c == '\u{3a3}' {
            out.push(if final_sigma(&chars, i) { '\u{3c2}' } else { '\u{3c3}' });
        } else if let Some(l) = m.get(&c) {
            out.push_str(l);
        } else {
            out.push(c);
        }
    }
    out
}

// Approximation of Unicode Cased / Case_Ignorable for the Final_Sigma rule
// (only reached for names containing U+03A3).
fn cased(c: char) -> bool {
    c.is_lowercase() || c.is_uppercase()
}
fn case_ignorable(c: char) -> bool {
    matches!(c, '\'' | '.' | ':' | '^' | '`' | '\u{ad}' | '\u{b7}' | '\u{2019}')
        || ('\u{300}'..='\u{36f}').contains(&c)
}
fn final_sigma(chars: &[char], i: usize) -> bool {
    let mut j = i;
    let mut before = false;
    while j > 0 {
        j -= 1;
        if case_ignorable(chars[j]) {
            continue;
        }
        before = cased(chars[j]);
        break;
    }
    if !before {
        return false;
    }
    for &c in &chars[i + 1..] {
        if case_ignorable(c) {
            continue;
        }
        return !cased(c);
    }
    true
}

fn decimal_value(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        return Some(c as u32 - '0' as u32);
    }
    let u = c as u32;
    let i = PY_DECIMAL_RANGES.partition_point(|&(_, b, _)| b < u);
    if let Some(&(a, b, v)) = PY_DECIMAL_RANGES.get(i) {
        if a <= u && u <= b {
            return Some(v + (u - a));
        }
    }
    None
}

/// Python `int(s)` for a `str` argument (base 10). Values outside i128 are
/// reported as an error (Python would produce a big integer).
pub fn py_int(s: &str) -> Result<i128, String> {
    let err = || format!("invalid literal for int() with base 10: {:?}", s);
    let t = strip(s);
    let mut chars = t.chars().peekable();
    let mut neg = false;
    if let Some(&c) = chars.peek() {
        if c == '+' || c == '-' {
            neg = c == '-';
            chars.next();
        }
    }
    let mut val: i128 = 0;
    let mut ndigits = 0;
    let mut prev_underscore = false;
    for c in chars {
        if c == '_' {
            if ndigits == 0 || prev_underscore {
                return Err(err());
            }
            prev_underscore = true;
            continue;
        }
        let d = decimal_value(c).ok_or_else(err)?;
        prev_underscore = false;
        ndigits += 1;
        val = val
            .checked_mul(10)
            .and_then(|v| v.checked_add(d as i128))
            .ok_or_else(|| format!("int() value out of supported range: {:?}", s))?;
    }
    if ndigits == 0 || prev_underscore {
        return Err(err());
    }
    Ok(if neg { -val } else { val })
}

/// NFKC as `unicodedata.normalize('NFKC', s)` in Python 3.11 (Unicode 14.0).
/// Code points unassigned in 14.0 are passed through untouched and act as
/// normalisation boundaries, as Python treats them.
pub fn nfkc(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    if s.is_ascii() {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut seg_start = 0;
    for (i, c) in s.char_indices() {
        if !c.is_ascii() && is_unassigned_14(c) {
            out.extend(s[seg_start..i].nfkc());
            out.push(c);
            seg_start = i + c.len_utf8();
        }
    }
    out.extend(s[seg_start..].nfkc());
    out
}
