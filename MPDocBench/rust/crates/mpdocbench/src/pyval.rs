//! A dynamic value with Python semantics (dicts keep insertion order, int and float
//! are distinct, floats may be NaN) and a JSON writer that reproduces Python's
//! `json.dump(obj, f, indent=4, ensure_ascii=False)` byte for byte.

use indexmap::IndexMap;
use std::fmt::Write as _;
use std::sync::Arc;

pub type Map = IndexMap<String, V>;

#[derive(Clone, Debug)]
pub enum V {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(Arc<str>),
    List(Arc<Vec<V>>),
    Dict(Arc<Map>),
}

impl Default for V {
    fn default() -> Self {
        V::Null
    }
}

impl From<&str> for V {
    fn from(s: &str) -> V {
        V::Str(Arc::from(s))
    }
}
impl From<String> for V {
    fn from(s: String) -> V {
        V::Str(Arc::from(s))
    }
}
impl From<i64> for V {
    fn from(i: i64) -> V {
        V::Int(i)
    }
}
impl From<usize> for V {
    fn from(i: usize) -> V {
        V::Int(i as i64)
    }
}
impl From<f64> for V {
    fn from(f: f64) -> V {
        V::Float(f)
    }
}
impl From<bool> for V {
    fn from(b: bool) -> V {
        V::Bool(b)
    }
}
impl From<Vec<V>> for V {
    fn from(v: Vec<V>) -> V {
        V::List(Arc::new(v))
    }
}
impl From<Map> for V {
    fn from(m: Map) -> V {
        V::Dict(Arc::new(m))
    }
}

impl V {
    pub fn str(s: &str) -> V {
        V::from(s)
    }
    pub fn list(v: Vec<V>) -> V {
        V::from(v)
    }
    pub fn empty_dict() -> V {
        V::Dict(Arc::new(Map::new()))
    }

    /// Python truthiness.
    pub fn truthy(&self) -> bool {
        match self {
            V::Null => false,
            V::Bool(b) => *b,
            V::Int(i) => *i != 0,
            V::Float(f) => *f != 0.0,
            V::Str(s) => !s.is_empty(),
            V::List(l) => !l.is_empty(),
            V::Dict(d) => !d.is_empty(),
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            V::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_dict(&self) -> Option<&Map> {
        match self {
            V::Dict(d) => Some(d),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&Vec<V>> {
        match self {
            V::List(l) => Some(l),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            V::Int(i) => Some(*i),
            V::Bool(b) => Some(*b as i64),
            _ => None,
        }
    }
    /// Numeric value (int, bool or float) as f64.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            V::Int(i) => Some(*i as f64),
            V::Bool(b) => Some(*b as i64 as f64),
            V::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn is_null(&self) -> bool {
        matches!(self, V::Null)
    }

    /// `d.get(key)` for dicts; Null otherwise / when missing.
    pub fn get(&self, key: &str) -> &V {
        static NULL: V = V::Null;
        match self {
            V::Dict(d) => d.get(key).unwrap_or(&NULL),
            _ => &NULL,
        }
    }
    pub fn has(&self, key: &str) -> bool {
        match self {
            V::Dict(d) => d.contains_key(key),
            _ => false,
        }
    }
    pub fn dict_mut(&mut self) -> &mut Map {
        match self {
            V::Dict(d) => Arc::make_mut(d),
            _ => panic!("not a dict"),
        }
    }
    pub fn set(&mut self, key: &str, value: V) {
        let d = self.dict_mut();
        if let Some(slot) = d.get_mut(key) {
            *slot = value;
        } else {
            d.insert(key.to_string(), value);
        }
    }

    /// Python `len()` for str (code points), list, dict.
    pub fn py_len(&self) -> usize {
        match self {
            V::Str(s) => s.chars().count(),
            V::List(l) => l.len(),
            V::Dict(d) => d.len(),
            _ => 0,
        }
    }

    /// Python `==` (ints and floats compare numerically; bool is an int).
    pub fn py_eq(&self, other: &V) -> bool {
        match (self, other) {
            (V::Null, V::Null) => true,
            (V::Str(a), V::Str(b)) => a == b,
            (V::List(a), V::List(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.py_eq(y)),
            (V::Dict(a), V::Dict(b)) => {
                a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).map_or(false, |w| v.py_eq(w)))
            }
            (a, b) => match (a.num(), b.num()) {
                (Some(x), Some(y)) => x == y,
                _ => false,
            },
        }
    }

    fn num(&self) -> Option<f64> {
        match self {
            V::Int(i) => Some(*i as f64),
            V::Bool(b) => Some(*b as i64 as f64),
            V::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Python `<` ordering for the value kinds that appear as sort keys.
    /// Err(()) where Python raises TypeError.
    pub fn py_cmp(&self, other: &V) -> Result<std::cmp::Ordering, ()> {
        use std::cmp::Ordering;
        match (self, other) {
            (V::Str(a), V::Str(b)) => Ok(a.as_ref().cmp(b.as_ref())),
            (V::Int(a), V::Int(b)) => Ok(a.cmp(b)),
            (V::List(a), V::List(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    if !x.py_eq(y) {
                        return x.py_cmp(y);
                    }
                }
                Ok(a.len().cmp(&b.len()))
            }
            (a, b) => match (a.num(), b.num()) {
                (Some(x), Some(y)) => Ok(x.partial_cmp(&y).unwrap_or(Ordering::Equal)),
                _ => Err(()),
            },
        }
    }

    /// Python `str(v)`.
    pub fn py_str(&self) -> String {
        match self {
            V::Str(s) => s.to_string(),
            _ => self.py_repr(),
        }
    }

    /// Python `repr(v)`.
    pub fn py_repr(&self) -> String {
        match self {
            V::Null => "None".into(),
            V::Bool(true) => "True".into(),
            V::Bool(false) => "False".into(),
            V::Int(i) => i.to_string(),
            V::Float(f) => float_repr(*f),
            V::Str(s) => str_repr(s),
            V::List(l) => {
                let parts: Vec<String> = l.iter().map(|x| x.py_repr()).collect();
                format!("[{}]", parts.join(", "))
            }
            V::Dict(d) => {
                let parts: Vec<String> = d.iter().map(|(k, v)| format!("{}: {}", str_repr(k), v.py_repr())).collect();
                format!("{{{}}}", parts.join(", "))
            }
        }
    }
}

/// Python `repr()` of a str.
pub fn str_repr(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') { '"' } else { '\'' };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c)
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                let _ = write!(out, "\\x{:02x}", c as u32);
            }
            c if !crate::py::isprintable(c) => {
                let u = c as u32;
                if u <= 0xff {
                    let _ = write!(out, "\\x{:02x}", u);
                } else if u <= 0xffff {
                    let _ = write!(out, "\\u{:04x}", u);
                } else {
                    let _ = write!(out, "\\U{:08x}", u);
                }
            }
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

/// Python `repr(float)` (shortest round-trip digits, Python's notation rules).
pub fn float_repr(f: f64) -> String {
    if f.is_nan() {
        return "nan".into();
    }
    if f.is_infinite() {
        return if f > 0.0 { "inf".into() } else { "-inf".into() };
    }
    if f == 0.0 {
        return if f.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    // Rust's `{:e}` is the shortest representation that round-trips: d[.ddd]e[-]X
    let s = format!("{:e}", f);
    let (mant, exp) = s.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let neg = mant.starts_with('-');
    let digits: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    // Python: repr uses 'r' format: scientific if exp < -4 or exp >= 16
    if exp < -4 || exp >= 16 {
        out.push_str(&digits[..1]);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        if exp < 0 {
            out.push('-');
        } else {
            out.push('+');
        }
        let _ = write!(out, "{:02}", exp.abs());
    } else if exp < 0 {
        out.push_str("0.");
        for _ in 0..(-exp - 1) {
            out.push('0');
        }
        out.push_str(&digits);
    } else {
        let e = exp as usize;
        if digits.len() > e + 1 {
            out.push_str(&digits[..e + 1]);
            out.push('.');
            out.push_str(&digits[e + 1..]);
        } else {
            out.push_str(&digits);
            for _ in 0..(e + 1 - digits.len()) {
                out.push('0');
            }
            out.push_str(".0");
        }
    }
    out
}

/// `json.dumps` float formatting (`float.__repr__`, but NaN/Infinity spelled as JSON extensions).
fn json_float(f: f64) -> String {
    if f.is_nan() {
        "NaN".into()
    } else if f.is_infinite() {
        if f > 0.0 { "Infinity".into() } else { "-Infinity".into() }
    } else {
        float_repr(f)
    }
}

fn json_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Internal per-sample annotation (the gold items of a pair, for `--export-pairs`); never
/// written to the toolkit's result files.
pub const INTERNAL_KEY: &str = "__gold";

/// One-line JSON (for .jsonl exports): `json.dumps(v, ensure_ascii=False)`.
pub fn to_json_line(v: &V) -> String {
    let mut out = String::new();
    write_compact(&mut out, v);
    out
}

fn write_compact(out: &mut String, v: &V) {
    match v {
        V::List(l) => {
            out.push('[');
            for (i, x) in l.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_compact(out, x);
            }
            out.push(']');
        }
        V::Dict(d) => {
            out.push('{');
            let mut first = true;
            for (k, x) in d.iter() {
                if k == INTERNAL_KEY {
                    continue;
                }
                if !first {
                    out.push_str(", ");
                }
                first = false;
                json_str(out, k);
                out.push_str(": ");
                write_compact(out, x);
            }
            out.push('}');
        }
        other => write_json(out, other, 0),
    }
}

/// Python `json.dumps(v, indent=4, ensure_ascii=False)`.
pub fn to_json_indent4(v: &V) -> String {
    let mut out = String::new();
    write_json(&mut out, v, 0);
    out
}

fn newline_indent(out: &mut String, level: usize) {
    out.push('\n');
    for _ in 0..level {
        out.push_str("    ");
    }
}

fn write_json(out: &mut String, v: &V, level: usize) {
    match v {
        V::Null => out.push_str("null"),
        V::Bool(true) => out.push_str("true"),
        V::Bool(false) => out.push_str("false"),
        V::Int(i) => {
            let _ = write!(out, "{}", i);
        }
        V::Float(f) => out.push_str(&json_float(*f)),
        V::Str(s) => json_str(out, s),
        V::List(l) => {
            if l.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, x) in l.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline_indent(out, level + 1);
                write_json(out, x, level + 1);
            }
            newline_indent(out, level);
            out.push(']');
        }
        V::Dict(d) => {
            if d.is_empty() {
                out.push_str("{}");
                return;
            }
            if d.keys().all(|k| k == INTERNAL_KEY) {
                out.push_str("{}");
                return;
            }
            out.push('{');
            let mut first = true;
            for (k, x) in d.iter() {
                if k == INTERNAL_KEY {
                    continue;
                }
                if !first {
                    out.push(',');
                }
                first = false;
                newline_indent(out, level + 1);
                json_str(out, k);
                out.push_str(": ");
                write_json(out, x, level + 1);
            }
            newline_indent(out, level);
            out.push('}');
        }
    }
}

/// Parse JSON with Python's `json.load` typing (int vs float, key order kept).
pub fn from_json(text: &str) -> Result<V, String> {
    let v: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    Ok(from_serde(v))
}

pub fn from_serde(v: serde_json::Value) -> V {
    use serde_json::Value as J;
    match v {
        J::Null => V::Null,
        J::Bool(b) => V::Bool(b),
        J::Number(n) => {
            if let Some(i) = n.as_i64() {
                V::Int(i)
            } else if let Some(u) = n.as_u64() {
                V::Float(u as f64)
            } else {
                V::Float(n.as_f64().unwrap())
            }
        }
        J::String(s) => V::Str(Arc::from(s)),
        J::Array(a) => V::List(Arc::new(a.into_iter().map(from_serde).collect())),
        J::Object(o) => {
            let mut m = Map::with_capacity(o.len());
            for (k, x) in o {
                m.insert(k, from_serde(x));
            }
            V::Dict(Arc::new(m))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn floats() {
        assert_eq!(float_repr(1.0), "1.0");
        assert_eq!(float_repr(0.1), "0.1");
        assert_eq!(float_repr(1e-5), "1e-05");
        assert_eq!(float_repr(0.0001), "0.0001");
        assert_eq!(float_repr(1e16), "1e+16");
        assert_eq!(float_repr(1.5e16), "1.5e+16");
        assert_eq!(float_repr(123456789012345.6), "123456789012345.6");
        assert_eq!(float_repr(2.0 / 3.0), "0.6666666666666666");
        assert_eq!(float_repr(1e22), "1e+22");
        assert_eq!(float_repr(-0.5), "-0.5");
        assert_eq!(float_repr(1234.0), "1234.0");
    }
}
