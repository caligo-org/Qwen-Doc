//! Python `str` semantics that differ from Rust's: whitespace (`str.isspace`, `re` `\s`),
//! word characters (`re` `\w`), `strip`/`split`, code-point indexing.

use unicode_general_category::{get_general_category, GeneralCategory as G};

/// Python `str.isspace()` for one character (also what `re` uses for `\s`).
pub fn isspace(c: char) -> bool {
    matches!(c,
        '\u{9}'..='\u{d}' | '\u{1c}'..='\u{20}' | '\u{85}' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}'
        | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}')
}

/// Python `str.isalnum()` for one character (`re` `\w` is this plus `_`).
pub fn isalnum(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphanumeric();
    }
    matches!(
        get_general_category(c),
        G::UppercaseLetter
            | G::LowercaseLetter
            | G::TitlecaseLetter
            | G::ModifierLetter
            | G::OtherLetter
            | G::DecimalNumber
            | G::LetterNumber
            | G::OtherNumber
    )
}

pub fn is_word(c: char) -> bool {
    c == '_' || isalnum(c)
}

/// Python `str.isprintable()` for one character (used by `repr`).
pub fn isprintable(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    !matches!(
        get_general_category(c),
        G::Control
            | G::Format
            | G::Surrogate
            | G::PrivateUse
            | G::Unassigned
            | G::LineSeparator
            | G::ParagraphSeparator
            | G::SpaceSeparator
    )
}

pub fn strip(s: &str) -> &str {
    s.trim_matches(isspace)
}
/// Python `s.strip(chars)`.
pub fn strip_chars<'a>(s: &'a str, chars: &str) -> &'a str {
    s.trim_matches(|c| chars.contains(c))
}


/// Python `len(s)`.
pub fn len(s: &str) -> usize {
    s.chars().count()
}

/// Map from byte offsets of `s` to code point indices (only valid at char boundaries).
pub struct CharIndex {
    /// byte offset of each char, plus the total length at the end
    starts: Vec<usize>,
    ascii: bool,
}

impl CharIndex {
    pub fn new(s: &str) -> CharIndex {
        if s.is_ascii() {
            return CharIndex { starts: Vec::new(), ascii: true };
        }
        let mut starts: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
        starts.push(s.len());
        CharIndex { starts, ascii: false }
    }
    pub fn char_of(&self, byte: usize) -> usize {
        if self.ascii {
            byte
        } else {
            self.starts.binary_search(&byte).expect("not a char boundary")
        }
    }
}

/// Byte offset of the code point index `ci` in `s` (len if past the end).
pub fn byte_of_char(s: &str, ci: usize) -> usize {
    s.char_indices().nth(ci).map(|(b, _)| b).unwrap_or(s.len())
}


/// Python `s.replace(old, new, 1)`.
pub fn replace_first(s: &str, old: &str, new: &str) -> String {
    if old.is_empty() {
        return format!("{}{}", new, s);
    }
    match s.find(old) {
        Some(i) => {
            let mut out = String::with_capacity(s.len() + new.len());
            out.push_str(&s[..i]);
            out.push_str(new);
            out.push_str(&s[i + old.len()..]);
            out
        }
        None => s.to_string(),
    }
}

/// Python `s.lower()` (full case mapping incl. final sigma, as Rust's `to_lowercase`).
pub fn lower(s: &str) -> String {
    s.to_lowercase()
}


pub const RE_S: &str = r"[\s\x1c-\x1f]";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ws() {
        assert!(isspace('\u{1c}'));
        assert!(!isspace('\u{200b}'));
        assert_eq!(strip("\u{1f} a \u{3000}"), "a");
        assert!(isalnum('²'));
        assert!(isalnum('一'));
        assert!(!isalnum('\u{94d}')); // devanagari virama (Mn)
    }
}
