//! Port of CPython 3.11 `html.unescape`.

use crate::generated::{BS4_ENTITY_TO_CHARACTER, HTML5};
use std::collections::HashMap;
use std::sync::OnceLock;

pub(crate) fn html5() -> &'static HashMap<&'static str, &'static str> {
    static M: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    M.get_or_init(|| HTML5.iter().copied().collect())
}

/// bs4 `EntitySubstitution.HTML_ENTITY_TO_CHARACTER`.
pub(crate) fn bs4_entity(name: &str) -> Option<&'static str> {
    static M: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    M.get_or_init(|| BS4_ENTITY_TO_CHARACTER.iter().copied().collect())
        .get(name)
        .copied()
}

/// `html._invalid_charrefs`
pub(crate) fn invalid_charref(n: u32) -> Option<char> {
    Some(match n {
        0x00 => '\u{fffd}',
        0x0d => '\r',
        0x80 => '\u{20ac}',
        0x81 => '\u{81}',
        0x82 => '\u{201a}',
        0x83 => '\u{0192}',
        0x84 => '\u{201e}',
        0x85 => '\u{2026}',
        0x86 => '\u{2020}',
        0x87 => '\u{2021}',
        0x88 => '\u{02c6}',
        0x89 => '\u{2030}',
        0x8a => '\u{0160}',
        0x8b => '\u{2039}',
        0x8c => '\u{0152}',
        0x8d => '\u{8d}',
        0x8e => '\u{017d}',
        0x8f => '\u{8f}',
        0x90 => '\u{90}',
        0x91 => '\u{2018}',
        0x92 => '\u{2019}',
        0x93 => '\u{201c}',
        0x94 => '\u{201d}',
        0x95 => '\u{2022}',
        0x96 => '\u{2013}',
        0x97 => '\u{2014}',
        0x98 => '\u{02dc}',
        0x99 => '\u{2122}',
        0x9a => '\u{0161}',
        0x9b => '\u{203a}',
        0x9c => '\u{0153}',
        0x9d => '\u{9d}',
        0x9e => '\u{017e}',
        0x9f => '\u{0178}',
        _ => return None,
    })
}

fn invalid_codepoint(n: u32) -> bool {
    matches!(n, 0x1..=0x8 | 0xe..=0x1f | 0x7f..=0x9f | 0xfdd0..=0xfdef | 0xb)
        || (n & 0xfffe == 0xfffe && n <= 0x10ffff)
}

/// Parse digits (already validated) in `radix`, saturating far above 0x10FFFF.
pub(crate) fn parse_saturating(digits: &[char], radix: u32) -> u64 {
    let mut v: u64 = 0;
    for &c in digits {
        let d = c.to_digit(radix).unwrap() as u64;
        v = v.saturating_mul(radix as u64).saturating_add(d);
        if v > 0xFFFF_FFFF {
            v = 0xFFFF_FFFF;
        }
    }
    v
}

fn numeric_replacement(num: u64, out: &mut String) {
    if num <= 0x9f {
        if let Some(c) = invalid_charref(num as u32) {
            out.push(c);
            return;
        }
    }
    if (0xD800..=0xDFFF).contains(&num) || num > 0x10FFFF {
        out.push('\u{fffd}');
        return;
    }
    if invalid_codepoint(num as u32) {
        return;
    }
    out.push(char::from_u32(num as u32).unwrap());
}

/// Python's `html.unescape(s)`.
pub fn html_unescape(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < n {
        let c = chars[i];
        if c != '&' {
            out.push(c);
            i += 1;
            continue;
        }
        // Try the three alternatives of `_charref` at i.
        let j = i + 1;
        if j < n && chars[j] == '#' {
            // #[0-9]+;?
            let mut k = j + 1;
            while k < n && chars[k].is_ascii_digit() {
                k += 1;
            }
            if k > j + 1 {
                let num = parse_saturating(&chars[j + 1..k], 10);
                if k < n && chars[k] == ';' {
                    k += 1;
                }
                numeric_replacement(num, &mut out);
                i = k;
                continue;
            }
            // #[xX][0-9a-fA-F]+;?
            if j + 1 < n && (chars[j + 1] == 'x' || chars[j + 1] == 'X') {
                let mut k = j + 2;
                while k < n && chars[k].is_ascii_hexdigit() {
                    k += 1;
                }
                if k > j + 2 {
                    let num = parse_saturating(&chars[j + 2..k], 16);
                    if k < n && chars[k] == ';' {
                        k += 1;
                    }
                    numeric_replacement(num, &mut out);
                    i = k;
                    continue;
                }
            }
            // third alternative cannot start with '#'
            out.push('&');
            i += 1;
            continue;
        }
        // [^\t\n\f <&#;]{1,32};?
        let mut k = j;
        while k < n && k - j < 32 && !matches!(chars[k], '\t' | '\n' | '\x0c' | ' ' | '<' | '&' | '#' | ';')
        {
            k += 1;
        }
        if k == j {
            out.push('&');
            i += 1;
            continue;
        }
        if k < n && chars[k] == ';' {
            k += 1;
        }
        let name: String = chars[j..k].iter().collect();
        let table = html5();
        if let Some(v) = table.get(name.as_str()) {
            out.push_str(v);
        } else {
            let ncount = k - j;
            let mut done = false;
            let mut x = ncount - 1;
            while x > 1 {
                let prefix: String = chars[j..j + x].iter().collect();
                if let Some(v) = table.get(prefix.as_str()) {
                    out.push_str(v);
                    out.extend(chars[j + x..k].iter());
                    done = true;
                    break;
                }
                x -= 1;
            }
            if !done {
                out.push('&');
                out.push_str(&name);
            }
        }
        i = k;
    }
    out
}
