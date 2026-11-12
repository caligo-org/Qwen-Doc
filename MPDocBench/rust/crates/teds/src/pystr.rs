//! Python 3.11 string semantics the scorers depend on.

/// `str.isspace()` / `re` `\s` for `str` patterns / `Py_UNICODE_ISSPACE` (Unicode 14.0).
pub fn is_py_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{0b}' | '\u{0c}' | '\r' | '\u{1c}'..='\u{1f}' | ' ' | '\u{85}' | '\u{a0}'
            | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}'
            | '\u{205f}' | '\u{3000}'
    )
}

/// `str.strip()` with no arguments.
pub fn py_strip(s: &str) -> &str {
    s.trim_matches(is_py_space)
}

/// Code points of Unicode 14.0 decimal digit zeros (`unicodedata.decimal(c) == 0`);
/// each starts a run of ten consecutive digits.
const DECIMAL_ZEROS: [u32; 66] = [
    0x30, 0x660, 0x6f0, 0x7c0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xbe6, 0xc66, 0xce6, 0xd66,
    0xde6, 0xe50, 0xed0, 0xf20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80, 0x1a90,
    0x1b50, 0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xa9f0, 0xaa50, 0xabf0, 0xff10,
    0x104a0, 0x10d30, 0x11066, 0x110f0, 0x11136, 0x111d0, 0x112f0, 0x11450, 0x114d0, 0x11650,
    0x116c0, 0x11730, 0x118e0, 0x11950, 0x11c50, 0x11d50, 0x11da0, 0x16a60, 0x16ac0, 0x16b50,
    0x1d7ce, 0x1d7d8, 0x1d7e2, 0x1d7ec, 0x1d7f6, 0x1e140, 0x1e2f0, 0x1e950, 0x1fbf0,
];

/// `Py_UNICODE_TODECIMAL`.
fn to_decimal(c: char) -> Option<u8> {
    let v = c as u32;
    let i = DECIMAL_ZEROS.partition_point(|&z| z <= v);
    if i == 0 {
        return None;
    }
    let d = v - DECIMAL_ZEROS[i - 1];
    (d < 10).then_some(d as u8)
}

/// Python's default limit on the number of digits `int(str)` converts (`sys.get_int_max_str_digits()`).
const MAX_STR_DIGITS: usize = 4300;

/// `int(s)` (base 10) for a `str`, returned in canonical form (`-` sign only for negative
/// values, no leading zeros), or `None` where Python raises `ValueError`.
pub fn py_int(s: &str) -> Option<String> {
    // _PyUnicode_TransformDecimalAndSpaceToASCII: below 127 kept as is; other whitespace
    // becomes ' ', other decimal digits become ASCII digits, anything else ends the string
    // with '?' (which then fails to parse).
    let mut ascii: Vec<u8> = Vec::with_capacity(s.len());
    for c in s.chars() {
        if (c as u32) < 127 {
            ascii.push(c as u8);
        } else if is_py_space(c) {
            ascii.push(b' ');
        } else if let Some(d) = to_decimal(c) {
            ascii.push(b'0' + d);
        } else {
            return None;
        }
    }
    // PyLong_FromString with base 10.
    let is_space = |b: u8| matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
    let mut i = 0;
    while i < ascii.len() && is_space(ascii[i]) {
        i += 1;
    }
    let mut negative = false;
    if i < ascii.len() && (ascii[i] == b'+' || ascii[i] == b'-') {
        negative = ascii[i] == b'-';
        i += 1;
    }
    let mut digits = Vec::new();
    let mut prev_underscore = false;
    let start = i;
    while i < ascii.len() {
        let b = ascii[i];
        if b.is_ascii_digit() {
            digits.push(b);
            prev_underscore = false;
        } else if b == b'_' {
            if i == start || prev_underscore {
                return None;
            }
            prev_underscore = true;
        } else {
            break;
        }
        i += 1;
    }
    if digits.is_empty() || prev_underscore {
        return None;
    }
    while i < ascii.len() && is_space(ascii[i]) {
        i += 1;
    }
    if i != ascii.len() {
        return None;
    }
    if digits.len() > MAX_STR_DIGITS {
        return None;
    }
    let first_nonzero = digits.iter().position(|&b| b != b'0');
    let mut out = String::new();
    match first_nonzero {
        None => out.push('0'),
        Some(k) => {
            if negative {
                out.push('-');
            }
            out.push_str(std::str::from_utf8(&digits[k..]).unwrap());
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_parsing() {
        assert_eq!(py_int("2").as_deref(), Some("2"));
        assert_eq!(py_int(" +02 ").as_deref(), Some("2"));
        assert_eq!(py_int("-0").as_deref(), Some("0"));
        assert_eq!(py_int("1_0").as_deref(), Some("10"));
        assert_eq!(py_int("\u{663}").as_deref(), Some("3"));
        assert_eq!(py_int("\u{ff12}\u{3000}").as_deref(), Some("2"));
        assert_eq!(py_int(""), None);
        assert_eq!(py_int("_1"), None);
        assert_eq!(py_int("1_"), None);
        assert_eq!(py_int("1__0"), None);
        assert_eq!(py_int("\u{1c}1"), None);
        assert_eq!(py_int("\u{85}1").as_deref(), Some("1"));
        assert_eq!(py_int("2.0"), None);
        assert_eq!(py_int("0x2"), None);
    }
}
