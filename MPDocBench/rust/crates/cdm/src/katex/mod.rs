//! Rust port of the toolkit's KaTeX based tokenizers: `preprocess_formula.js normalize`
//! and `preprocess_tabular.js tokenize` (metrics/cdm/modules/tokenize_latex) together
//! with the parts of the vendored (and locally modified) KaTeX they reach: `Lexer.js`,
//! `Parser.js` (including its `global_str` token log), `functions.js`,
//! `environments.js` and `symbols.js` (tables generated into `tables.rs`).
//!
//! The port reproduces the JavaScript behaviour, quirks included: strings are UTF-16
//! code unit sequences (output lone surrogates become U+FFFD, as `console.log` writes
//! them), `var` hoisting in `parseArguments` (an argument the parser skips repeats the
//! previous one), JS `String()` of values where the code concatenates or compares them,
//! and every exception is caught per input line and prints an empty line.
//!
//! [`run`] maps the bytes node reads on stdin to the text it writes on stdout.

mod tables;

use std::collections::HashMap;
use std::sync::OnceLock;

/// A JavaScript string (UTF-16 code units).
type W = Vec<u16>;

fn w(s: &str) -> W {
    s.encode_utf16().collect()
}

fn eq(a: &[u16], b: &str) -> bool {
    a.len() == b.len() && a.iter().copied().eq(b.encode_utf16())
}

fn push(out: &mut W, s: &str) {
    out.extend(s.encode_utf16());
}

/// Any exception thrown inside the `try` of the line handler (ParseError, TypeError,
/// ReferenceError, RangeError): the line prints as an empty line.
#[derive(Debug)]
pub struct JsError;

type R<T> = Result<T, JsError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Group {
    Accent,
    Bin,
    Close,
    Inner,
    Mathord,
    Op,
    Open,
    Punct,
    Rel,
    Spacing,
    Textord,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ArgType {
    Text,
    Color,
    Size,
    Original,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Handler {
    Sqrt,
    Text,
    Color,
    Overline,
    Underline,
    Rule,
    Katex,
    Phantom,
    ColorFn,
    OpNoLimitsNoSymbol,
    OpLimitsNoSymbol,
    OpNoLimitsSymbol,
    OpLimitsSymbol,
    Genfrac,
    Lap,
    Delim,
    Null,
    Font,
    Accent,
    Infix,
    Cr,
    Environment,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum EnvHandler {
    Array,
    Tabular,
    Matrix,
    Cases,
    Aligned,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FuncSpec {
    num_args: u32,
    num_optional: u32,
    greediness: u32,
    arg_types: Option<&'static [ArgType]>,
    handler: Handler,
}

struct Tables {
    math: HashMap<W, Group>,
    text: HashMap<W, Group>,
    functions: HashMap<W, FuncSpec>,
    envs: HashMap<W, (EnvHandler, u32)>,
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| Tables {
        math: tables::MATH_SYMBOLS.iter().map(|(k, g)| (w(k), *g)).collect(),
        text: tables::TEXT_SYMBOLS.iter().map(|(k, g)| (w(k), *g)).collect(),
        functions: tables::FUNCTIONS.iter().map(|(k, f)| (w(k), *f)).collect(),
        envs: tables::ENVIRONMENTS.iter().map(|(k, h, n)| (w(k), (*h, *n))).collect(),
    })
}

// ---------------------------------------------------------------------------------------
// Parse tree

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ty {
    Mathord,
    Textord,
    Bin,
    Rel,
    Open,
    Close,
    Inner,
    Punct,
    Spacing,
    Accent,
    Op,
    Ordgroup,
    Supsub,
    Sizing,
    Styling,
    Leftright,
    Array,
    Sqrt,
    Text,
    Color,
    Overline,
    Underline,
    Rule,
    Katex,
    Phantom,
    Genfrac,
    Llap,
    Rlap,
    Delimsizing,
    Font,
    Infix,
    Cr,
    Environment,
    Size,
}

impl From<Group> for Ty {
    fn from(g: Group) -> Ty {
        match g {
            Group::Accent => Ty::Accent,
            Group::Bin => Ty::Bin,
            Group::Close => Ty::Close,
            Group::Inner => Ty::Inner,
            Group::Mathord => Ty::Mathord,
            Group::Op => Ty::Op,
            Group::Open => Ty::Open,
            Group::Punct => Ty::Punct,
            Group::Rel => Ty::Rel,
            Group::Spacing => Ty::Spacing,
            Group::Textord => Ty::Textord,
        }
    }
}

#[derive(Clone, Debug)]
struct SizeData {
    number: f64,
    unit: W,
}

/// A column of an array (`{type: "align", align}` or `{type: "separator"}`); the align
/// value keeps the JS value it was taken from (a one-letter string, or an empty array
/// from an empty `{}` group, which is truthy and prints as "").
#[derive(Clone, Debug)]
enum Col {
    Align(W),
    AlignEmptyArray,
    Separator,
}

type Opt = Option<Box<Node>>;

/// `ParseNode.value`
#[derive(Clone, Debug)]
enum V {
    /// symbols: the token text
    Str(W),
    /// ordgroup: the array of nodes
    List(Vec<Node>),
    SupSub { base: Opt, sup: Opt, sub: Opt },
    /// sizing / styling
    Implicit { original: W, value: Vec<Node> },
    /// leftright from `\left ... \right` or a delimited environment
    LeftRight { body: Vec<Node>, left: W, right: W },
    /// `\left` / `\right` called as functions, and delimsizing
    Delim { value: W, func_name: W },
    Array { cols: Option<Vec<Option<Col>>>, body: Vec<Vec<Node>> },
    Sqrt { body: Opt, index: Opt },
    /// text body, color value, phantom value
    Inner(Vec<Node>),
    /// overline, underline, llap, rlap
    Body(Opt),
    Font { font: W, body: Opt },
    Rule { width: SizeData, height: SizeData },
    Empty,
    Op { limits: bool, symbol: bool, body: W },
    Genfrac { numer: Opt, denom: Opt, bar: bool },
    Accent { accent: W, base: Opt },
    Infix { replace_with: &'static str },
    Env { name: W },
    SizeData(SizeData),
}

#[derive(Clone, Debug)]
struct Node {
    ty: Ty,
    v: V,
}

fn node(ty: Ty, v: V) -> Node {
    Node { ty, v }
}

/// JS `String(value)` of a ParseNode value: strings as is, arrays (of ParseNodes) joined
/// with "," as "[object Object]" each, other objects "[object Object]".
fn js_string(v: &V) -> W {
    match v {
        V::Str(s) => s.clone(),
        V::List(l) => {
            let mut o = W::new();
            for (i, _) in l.iter().enumerate() {
                if i > 0 {
                    push(&mut o, ",");
                }
                push(&mut o, "[object Object]");
            }
            o
        }
        _ => w("[object Object]"),
    }
}

/// `ParseFuncOrArgument`
enum Pfa {
    Func(W),
    Arg(Node),
}

// ---------------------------------------------------------------------------------------
// Lexer

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Math,
    Text,
    Color,
    Size,
}

#[derive(Clone, Debug)]
struct Token {
    text: W,
    data: Option<SizeData>,
    position: usize,
}

/// JS `\s` (WhiteSpace and LineTerminator)
fn js_space(c: u16) -> bool {
    matches!(c, 0x09..=0x0d | 0x20 | 0xa0 | 0x1680 | 0x2000..=0x200a | 0x2028 | 0x2029 | 0x202f | 0x205f | 0x3000 | 0xfeff)
}

/// `.` in a non-unicode JS regex: anything but a line terminator
fn js_dot(c: u16) -> bool {
    !matches!(c, 0x0a | 0x0d | 0x2028 | 0x2029)
}

fn is_ascii_letter(c: u16) -> bool {
    (b'a' as u16..=b'z' as u16).contains(&c) || (b'A' as u16..=b'Z' as u16).contains(&c)
}

fn is_digit(c: u16) -> bool {
    (b'0' as u16..=b'9' as u16).contains(&c)
}

fn is_surrogate(c: u16) -> bool {
    (0xd800..=0xdfff).contains(&c)
}

struct Lexer<'a> {
    input: &'a [u16],
}

impl Lexer<'_> {
    fn inner_lex(&self, mut pos: usize, ignore_ws: bool) -> R<Token> {
        let s = self.input;
        loop {
            if pos == s.len() {
                return Ok(Token { text: w("EOF"), data: None, position: pos });
            }
            let c = s[pos];
            if matches!(c, 0x20 | 0x0d | 0x0a | 0x09) {
                let mut e = pos;
                while e < s.len() && matches!(s[e], 0x20 | 0x0d | 0x0a | 0x09) {
                    e += 1;
                }
                if ignore_ws {
                    pos = e;
                    continue;
                }
                return Ok(Token { text: w(" "), data: None, position: e });
            }
            let len = if c == b'-' as u16 && s.get(pos + 1) == Some(&(b'-' as u16)) {
                if s.get(pos + 2) == Some(&(b'-' as u16)) {
                    3
                } else {
                    2
                }
            } else if (0x21..=0x5b).contains(&c)
                || (0x5d..=0x2027).contains(&c)
                || (0x202a..=0xd7ff).contains(&c)
                || (0xf900..=0xffff).contains(&c)
            {
                1
            } else if (0xd800..=0xdbff).contains(&c) && s.get(pos + 1).is_some_and(|&d| (0xdc00..=0xdfff).contains(&d)) {
                2
            } else if c == b'\\' as u16 {
                match s.get(pos + 1) {
                    Some(&d) if is_ascii_letter(d) => {
                        let mut e = pos + 1;
                        while e < s.len() && is_ascii_letter(s[e]) {
                            e += 1;
                        }
                        e - pos
                    }
                    Some(&d) if !is_surrogate(d) => 2,
                    _ => return Err(JsError), // Unexpected character
                }
            } else {
                return Err(JsError); // Unexpected character
            };
            return Ok(Token { text: s[pos..pos + len].to_vec(), data: None, position: pos + len });
        }
    }

    fn skip_space(&self, mut pos: usize) -> usize {
        while pos < self.input.len() && js_space(self.input[pos]) {
            pos += 1;
        }
        pos
    }

    /// `/#[a-z0-9]+|[a-z]+/i` at pos, after `\s*`
    fn lex_color(&self, pos: usize) -> R<Token> {
        let s = self.input;
        let pos = self.skip_space(pos);
        let alnum = |c: u16| is_ascii_letter(c) || is_digit(c);
        let mut e = pos;
        if s.get(pos) == Some(&(b'#' as u16)) {
            e = pos + 1;
            while e < s.len() && alnum(s[e]) {
                e += 1;
            }
            if e == pos + 1 {
                e = pos; // `#` alone: the second alternative cannot match at `#` either
            }
        } else {
            while e < s.len() && is_ascii_letter(s[e]) {
                e += 1;
            }
        }
        if e == pos {
            return Err(JsError); // Invalid color
        }
        Ok(Token { text: s[pos..e].to_vec(), data: None, position: e })
    }

    /// `/(-?)\s*(\d+(?:\.\d*)?|\.\d+)\s*([a-z]{2})/` at pos, after `\s*`
    fn lex_size(&self, pos: usize) -> R<Token> {
        let s = self.input;
        let start = self.skip_space(pos);
        let mut p = start;
        let neg = s.get(p) == Some(&(b'-' as u16));
        if neg {
            p += 1;
        }
        p = self.skip_space(p);
        let ns = p;
        let dot = b'.' as u16;
        if s.get(p).is_some_and(|&c| is_digit(c)) {
            while p < s.len() && is_digit(s[p]) {
                p += 1;
            }
            if s.get(p) == Some(&dot) {
                p += 1;
                while p < s.len() && is_digit(s[p]) {
                    p += 1;
                }
            }
        } else if s.get(p) == Some(&dot) && s.get(p + 1).is_some_and(|&c| is_digit(c)) {
            p += 1;
            while p < s.len() && is_digit(s[p]) {
                p += 1;
            }
        } else {
            return Err(JsError); // Invalid size
        }
        let num: String = String::from_utf16_lossy(&s[ns..p]);
        p = self.skip_space(p);
        let lower = |c: u16| (b'a' as u16..=b'z' as u16).contains(&c);
        if !(s.get(p).is_some_and(|&c| lower(c)) && s.get(p + 1).is_some_and(|&c| lower(c))) {
            return Err(JsError);
        }
        let unit = s[p..p + 2].to_vec();
        p += 2;
        // +(match[1] + match[2])
        let mut lit = String::new();
        if num.starts_with('.') {
            lit.push('0');
        }
        lit.push_str(&num);
        if lit.ends_with('.') {
            lit.push('0');
        }
        let mut number: f64 = lit.parse().map_err(|_| JsError)?;
        if neg {
            number = -number;
        }
        Ok(Token { text: s[start..p].to_vec(), data: Some(SizeData { number, unit }), position: p })
    }

    fn lex(&self, pos: usize, mode: Mode) -> R<Token> {
        match mode {
            Mode::Math => self.inner_lex(pos, true),
            Mode::Text => self.inner_lex(pos, false),
            Mode::Color => self.lex_color(pos),
            Mode::Size => self.lex_size(pos),
        }
    }
}

// ---------------------------------------------------------------------------------------
// Parser

/// Approximation of V8's stack limit: the JS code throws a RangeError (caught: an empty
/// line) on very deep nesting. Each ported JS function adds a weight to a stack budget;
/// the weights are fitted to the depths at which node 22 starts failing for nested
/// `{x}` (914 levels ok), `\frac{x}{y}` (740), `a^{x}` (1325), `\left(x\right)` (1360),
/// `\sqrt[x]{y}` (740) and `\text{x}` (629). Parsing and rendering have separate budgets. (Node's own limit moves by a few
/// levels between runs, and is higher once the code is JIT-optimised.)
const MAX_FRAMES: usize = 10_005_000;
const W_EXPR: usize = 2450;
const W_ATOM: usize = 2450;
const W_IMPLICIT: usize = 2450;
const W_FUNCTION: usize = 1800;
const W_GROUP: usize = 1785;
const W_OPTIONAL: usize = 1785;
const W_ARGUMENTS: usize = 2569;
const W_SPECIAL: usize = 2381;
const W_SUPSUB: usize = 859;
const W_ARRAY: usize = 2500;
const B_EXPRESSION: usize = 1000;
const B_GROUP: usize = 3000;

const END_OF_EXPRESSION: [&str; 6] = ["}", "\\end", "\\right", "&", "\\\\", "\\cr"];

const SIZE_FUNCS: [&str; 17] = [
    "\\tiny", "\\scriptsize", "\\footnotesize", "\\small", "\\normalsize", "\\large", "\\Large", "\\LARGE", "\\huge",
    "\\Huge", "\\textrm", "\\rm", "\\cal", "\\bf", "\\siptstyle", "\\boldmath", "\\it",
];

const STYLE_FUNCS: [&str; 4] = ["\\displaystyle", "\\textstyle", "\\scriptstyle", "\\scriptscriptstyle"];

const DELIMITERS: [&str; 41] = [
    "(", ")", "[", "\\lbrack", "]", "\\rbrack", "\\{", "\\lbrace", "\\}", "\\rbrace", "\\lfloor", "\\rfloor", "\\lceil",
    "\\rceil", "<", ">", "\\langle", "\\rangle", "\\lt", "\\gt", "\\lvert", "\\rvert", "\\lVert", "\\rVert", "\\lgroup",
    "\\rgroup", "\\lmoustache", "\\rmoustache", "/", "\\backslash", "|", "\\vert", "\\|", "\\Vert", "\\uparrow",
    "\\Uparrow", "\\downarrow", "\\Downarrow", "\\updownarrow", "\\Updownarrow", ".",
];

struct Parser<'a> {
    lexer: Lexer<'a>,
    t: &'static Tables,
    mode: Mode,
    pos: usize,
    next: Token,
    /// `global_str` (only kept when `log` is set)
    gs: W,
    log: bool,
    frames: usize,
}

fn greediness(t: &Tables, f: &[u16]) -> u32 {
    t.functions[f].greediness
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u16], log: bool) -> Self {
        Parser {
            lexer: Lexer { input },
            t: tables(),
            mode: Mode::Math,
            pos: 0,
            next: Token { text: W::new(), data: None, position: 0 },
            gs: W::new(),
            log,
            frames: 0,
        }
    }

    fn enter(&mut self, n: usize) -> R<()> {
        self.frames += n;
        if self.frames > MAX_FRAMES {
            return Err(JsError);
        }
        Ok(())
    }

    fn leave(&mut self, n: usize) {
        self.frames -= n;
    }

    fn expect(&mut self, text: &str, consume: bool) -> R<()> {
        if !eq(&self.next.text, text) {
            return Err(JsError);
        }
        if consume {
            self.consume()?;
        }
        Ok(())
    }

    fn consume(&mut self) -> R<()> {
        self.pos = self.next.position;
        if self.log {
            push(&mut self.gs, " ");
            self.gs.extend_from_slice(&self.next.text);
        }
        self.next = self.lexer.lex(self.pos, self.mode)?;
        Ok(())
    }

    fn parse(&mut self) -> R<Vec<Node>> {
        self.mode = Mode::Math;
        self.pos = 0;
        self.next = self.lexer.lex(self.pos, self.mode)?;
        let e = self.parse_expression(false, None)?;
        self.expect("EOF", false)?;
        Ok(e)
    }

    fn parse_expression(&mut self, break_on_infix: bool, break_on_token: Option<&str>) -> R<Vec<Node>> {
        self.enter(W_EXPR)?;
        let r = stacker::maybe_grow(64 * 1024, 4 * 1024 * 1024, || self.parse_expression_(break_on_infix, break_on_token));
        self.leave(W_EXPR);
        r
    }

    fn parse_expression_(&mut self, break_on_infix: bool, break_on_token: Option<&str>) -> R<Vec<Node>> {
        let mut body = Vec::new();
        loop {
            let lex = self.next.clone();
            let pos = self.pos;
            if END_OF_EXPRESSION.iter().any(|e| eq(&lex.text, e)) {
                break;
            }
            if let Some(b) = break_on_token {
                if eq(&lex.text, b) {
                    break;
                }
            }
            let atom = match self.parse_atom()? {
                Some(a) => a,
                None => break,
            };
            if break_on_infix && atom.ty == Ty::Infix {
                self.pos = pos;
                self.next = lex;
                break;
            }
            body.push(atom);
        }
        self.handle_infix_nodes(body)
    }

    fn handle_infix_nodes(&mut self, body: Vec<Node>) -> R<Vec<Node>> {
        let mut over: Option<usize> = None;
        let mut func = "";
        for (i, n) in body.iter().enumerate() {
            if n.ty == Ty::Infix {
                if over.is_some() {
                    return Err(JsError); // only one infix operator per group
                }
                over = Some(i);
                if let V::Infix { replace_with } = n.v {
                    func = replace_with;
                }
            }
        }
        let Some(oi) = over else { return Ok(body) };
        let mut body = body;
        let denom_body: Vec<Node> = body.split_off(oi + 1);
        body.pop();
        let numer_body = body;
        let wrap = |b: Vec<Node>| -> Node {
            if b.len() == 1 && b[0].ty == Ty::Ordgroup {
                b.into_iter().next().unwrap()
            } else {
                node(Ty::Ordgroup, V::List(b))
            }
        };
        let numer = wrap(numer_body);
        let denom = wrap(denom_body);
        let n = self.call_function(&w(func), vec![Some(numer), Some(denom)])?;
        Ok(vec![n])
    }

    fn handle_sup_subscript(&mut self) -> R<Option<Node>> {
        self.enter(W_SUPSUB)?;
        let r = self.handle_sup_subscript_();
        self.leave(W_SUPSUB);
        r
    }

    fn handle_sup_subscript_(&mut self) -> R<Option<Node>> {
        self.consume()?;
        match self.parse_group()? {
            None => Ok(None),
            Some(Pfa::Func(f)) => {
                if greediness(self.t, &f) > 1 {
                    self.parse_function(Some(Pfa::Func(f)))
                } else {
                    Err(JsError)
                }
            }
            Some(Pfa::Arg(n)) => Ok(Some(n)),
        }
    }

    fn parse_atom(&mut self) -> R<Option<Node>> {
        self.enter(W_ATOM)?;
        let r = self.parse_atom_();
        self.leave(W_ATOM);
        r
    }

    fn parse_atom_(&mut self) -> R<Option<Node>> {
        let mut base = self.parse_implicit_group()?;
        if self.mode == Mode::Text {
            return Ok(base);
        }
        let mut sup: Option<Node> = None;
        let mut sub: Option<Node> = None;
        loop {
            let t = &self.next.text;
            if eq(t, "\\limits") || eq(t, "\\nolimits") {
                let limits = eq(t, "\\limits");
                match base.as_mut() {
                    Some(b) if b.ty == Ty::Op => {
                        // a string value (an op from the symbol table) ignores the assignment
                        if let V::Op { limits: l, .. } = &mut b.v {
                            *l = limits;
                        }
                    }
                    _ => return Err(JsError), // Limit controls must follow a math operator
                }
                self.consume()?;
            } else if eq(t, "^") {
                sup = self.handle_sup_subscript()?;
            } else if eq(t, "_") {
                sub = self.handle_sup_subscript()?;
            } else if eq(t, "'") {
                let prime = node(Ty::Textord, V::Str(w("\\prime")));
                let mut primes = vec![prime.clone()];
                self.consume()?;
                while eq(&self.next.text, "'") {
                    primes.push(prime.clone());
                    self.consume()?;
                }
                sup = Some(node(Ty::Ordgroup, V::List(primes)));
            } else {
                break;
            }
        }
        if sup.is_some() || sub.is_some() {
            Ok(Some(node(Ty::Supsub, V::SupSub { base: base.map(Box::new), sup: sup.map(Box::new), sub: sub.map(Box::new) })))
        } else {
            Ok(base)
        }
    }

    /// `global_str.substring(0, global_str.length - (name.length * 2 + 2)) + name + "}"`
    fn begin_hack(&mut self, name: &[u16]) {
        if !self.log {
            return;
        }
        let cut = self.gs.len() as i64 - (name.len() as i64 * 2 + 2);
        let cut = cut.max(0) as usize;
        self.gs.truncate(cut);
        self.gs.extend_from_slice(name);
        push(&mut self.gs, "}");
    }

    fn parse_implicit_group(&mut self) -> R<Option<Node>> {
        self.enter(W_IMPLICIT)?;
        let r = self.parse_implicit_group_();
        self.leave(W_IMPLICIT);
        r
    }

    fn parse_implicit_group_(&mut self) -> R<Option<Node>> {
        let start = match self.parse_symbol()? {
            None => return self.parse_function(None),
            Some(s) => s,
        };
        match start {
            Pfa::Func(f) if eq(&f, "\\left") => {
                let left = self.parse_function(Some(Pfa::Func(f)))?.ok_or(JsError)?;
                let body = self.parse_expression(false, None)?;
                self.expect("\\right", false)?;
                let right = self.parse_function(None)?.ok_or(JsError)?;
                let lv = match left.v {
                    V::Delim { value, .. } => value,
                    _ => return Err(JsError),
                };
                let rv = match right.v {
                    V::Delim { value, .. } => value,
                    _ => return Err(JsError),
                };
                Ok(Some(node(Ty::Leftright, V::LeftRight { body, left: lv, right: rv })))
            }
            Pfa::Func(f) if eq(&f, "\\begin") => {
                let begin = self.parse_function(Some(Pfa::Func(f)))?.ok_or(JsError)?;
                let env_name = match &begin.v {
                    V::Env { name } => name.clone(),
                    _ => return Err(JsError),
                };
                self.begin_hack(&env_name);
                let Some(&(handler, num_args)) = self.t.envs.get(&env_name) else {
                    return Err(JsError); // No such environment
                };
                let spec = FuncSpec { num_args, num_optional: 0, greediness: 1, arg_types: None, handler: Handler::Null };
                let args = self.parse_arguments(&spec)?;
                let result = self.env_handler(handler, &env_name, &args)?;
                self.expect("\\end", false)?;
                let end = self.parse_function(None)?.ok_or(JsError)?;
                self.begin_hack(&env_name);
                match &end.v {
                    V::Env { name } if *name == env_name => {}
                    _ => return Err(JsError), // Mismatch
                }
                Ok(Some(result))
            }
            Pfa::Arg(n) if matches!(&n.v, V::Str(s) if eq(s, "\\matrix") || eq(s, "\\pmatrix") || eq(s, "\\cases")) => {
                let V::Str(s) = &n.v else { unreachable!() };
                let env_name = s[1..].to_vec();
                let &(handler, _) = self.t.envs.get(&env_name).ok_or(JsError)?;
                self.expect("{", true)?;
                let result = self.env_handler(handler, &env_name, &[])?;
                self.expect("}", true)?;
                Ok(Some(result))
            }
            Pfa::Func(f) if SIZE_FUNCS.iter().any(|s| eq(&f, s)) => {
                let body = self.parse_expression(false, None)?;
                Ok(Some(node(Ty::Sizing, V::Implicit { original: f, value: body })))
            }
            Pfa::Func(f) if STYLE_FUNCS.iter().any(|s| eq(&f, s)) => {
                let body = self.parse_expression(true, None)?;
                Ok(Some(node(Ty::Styling, V::Implicit { original: f, value: body })))
            }
            start => self.parse_function(Some(start)),
        }
    }

    fn parse_function(&mut self, base: Option<Pfa>) -> R<Option<Node>> {
        self.enter(W_FUNCTION)?;
        let r = self.parse_function_(base);
        self.leave(W_FUNCTION);
        r
    }

    fn parse_function_(&mut self, base: Option<Pfa>) -> R<Option<Node>> {
        let base = match base {
            Some(b) => Some(b),
            None => self.parse_group()?,
        };
        match base {
            None => Ok(None),
            Some(Pfa::Func(f)) => {
                let spec = self.t.functions[&f];
                let args = self.parse_arguments(&spec)?;
                Ok(Some(self.call_function(&f, args)?))
            }
            Some(Pfa::Arg(n)) => Ok(Some(n)),
        }
    }

    fn parse_arguments(&mut self, spec: &FuncSpec) -> R<Vec<Option<Node>>> {
        self.enter(W_ARGUMENTS)?;
        let r = self.parse_arguments_(spec);
        self.leave(W_ARGUMENTS);
        r
    }

    fn parse_arguments_(&mut self, spec: &FuncSpec) -> R<Vec<Option<Node>>> {
        let total = spec.num_args + spec.num_optional;
        let mut args = Vec::new();
        // `var argNode` is function scoped: an argument that is a function with too low
        // a greediness leaves the previous argument's node in place.
        let mut arg_node: Option<Node> = None;
        for i in 0..total {
            let arg_type = spec.arg_types.and_then(|a| a.get(i as usize).copied());
            let arg = if i < spec.num_optional {
                let a = match arg_type {
                    Some(t) => self.parse_special_group(t, true)?,
                    None => self.parse_optional_group()?,
                };
                match a {
                    None => {
                        args.push(None);
                        continue;
                    }
                    Some(a) => a,
                }
            } else {
                let a = match arg_type {
                    Some(t) => self.parse_special_group(t, false)?,
                    None => self.parse_group()?,
                };
                a.ok_or(JsError)? // Expected group after ...
            };
            match arg {
                Pfa::Func(f) => {
                    if greediness(self.t, &f) > spec.greediness {
                        arg_node = self.parse_function(Some(Pfa::Func(f)))?;
                    }
                }
                Pfa::Arg(n) => arg_node = Some(n),
            }
            args.push(arg_node.clone());
        }
        Ok(args)
    }

    fn parse_special_group(&mut self, inner: ArgType, optional: bool) -> R<Option<Pfa>> {
        self.enter(W_SPECIAL)?;
        let r = self.parse_special_group_(inner, optional);
        self.leave(W_SPECIAL);
        r
    }

    fn parse_special_group_(&mut self, inner: ArgType, optional: bool) -> R<Option<Pfa>> {
        let outer = self.mode;
        let inner_mode = match inner {
            ArgType::Original => outer,
            ArgType::Text => Mode::Text,
            ArgType::Color => Mode::Color,
            ArgType::Size => Mode::Size,
        };
        if matches!(inner_mode, Mode::Color | Mode::Size) {
            if optional && !eq(&self.next.text, "[") {
                return Ok(None);
            }
            self.mode = inner_mode;
            self.expect(if optional { "[" } else { "{" }, true)?;
            let tok = self.next.clone();
            self.mode = outer;
            let (ty, v) = if inner_mode == Mode::Color {
                (Ty::Color, V::Str(tok.text))
            } else {
                (Ty::Size, V::SizeData(tok.data.ok_or(JsError)?))
            };
            self.consume()?;
            self.expect(if optional { "]" } else { "}" }, true)?;
            return Ok(Some(Pfa::Arg(node(ty, v))));
        } else if inner_mode == Mode::Text {
            self.pos = self.lexer.skip_space(self.pos);
        }
        self.mode = inner_mode;
        self.next = self.lexer.lex(self.pos, inner_mode)?;
        let res = if optional { self.parse_optional_group()? } else { self.parse_group()? };
        self.mode = outer;
        self.next = self.lexer.lex(self.pos, outer)?;
        Ok(res)
    }

    fn parse_group(&mut self) -> R<Option<Pfa>> {
        self.enter(W_GROUP)?;
        let r = self.parse_group_();
        self.leave(W_GROUP);
        r
    }

    fn parse_group_(&mut self) -> R<Option<Pfa>> {
        if eq(&self.next.text, "{") {
            self.consume()?;
            let e = self.parse_expression(false, None)?;
            self.expect("}", true)?;
            Ok(Some(Pfa::Arg(node(Ty::Ordgroup, V::List(e)))))
        } else {
            self.parse_symbol()
        }
    }

    fn parse_optional_group(&mut self) -> R<Option<Pfa>> {
        self.enter(W_OPTIONAL)?;
        let r = self.parse_optional_group_();
        self.leave(W_OPTIONAL);
        r
    }

    fn parse_optional_group_(&mut self) -> R<Option<Pfa>> {
        if eq(&self.next.text, "[") {
            self.consume()?;
            let e = self.parse_expression(false, Some("]"))?;
            self.expect("]", true)?;
            Ok(Some(Pfa::Arg(node(Ty::Ordgroup, V::List(e)))))
        } else {
            Ok(None)
        }
    }

    fn parse_symbol(&mut self) -> R<Option<Pfa>> {
        let text = self.next.text.clone();
        if self.t.functions.contains_key(&text) {
            self.consume()?;
            return Ok(Some(Pfa::Func(text)));
        }
        let syms = match self.mode {
            Mode::Math => &self.t.math,
            Mode::Text => &self.t.text,
            _ => return Err(JsError),
        };
        if let Some(&g) = syms.get(&text) {
            self.consume()?;
            return Ok(Some(Pfa::Arg(node(g.into(), V::Str(text)))));
        }
        if eq(&text, "EOF") || eq(&text, "{") {
            return Ok(None);
        }
        self.consume()?;
        // symbols["math"]["\\sigma"].group
        Ok(Some(Pfa::Arg(node(Ty::Mathord, V::Str(text)))))
    }

    // -- functions.js handlers --------------------------------------------------------

    fn call_function(&mut self, name: &[u16], args: Vec<Option<Node>>) -> R<Node> {
        let spec = self.t.functions[name];
        let mut args = args.into_iter();
        let mut arg = || args.next().flatten();
        /// `body.type === "ordgroup" ? body.value : [body]` (TypeError on undefined)
        fn inner(body: Option<Node>) -> R<Vec<Node>> {
            let b = body.ok_or(JsError)?;
            if b.ty == Ty::Ordgroup {
                match b.v {
                    V::List(l) => Ok(l),
                    _ => Err(JsError),
                }
            } else {
                Ok(vec![b])
            }
        }
        let bx = |n: Option<Node>| n.map(Box::new);
        Ok(match spec.handler {
            Handler::Sqrt => {
                let index = arg();
                let body = arg();
                node(Ty::Sqrt, V::Sqrt { body: bx(body), index: bx(index) })
            }
            Handler::Text => node(Ty::Text, V::Inner(inner(arg())?)),
            Handler::Color => {
                let color = arg().ok_or(JsError)?;
                let _ = color;
                node(Ty::Color, V::Inner(inner(arg())?))
            }
            Handler::Overline => node(Ty::Overline, V::Body(bx(arg()))),
            Handler::Underline => node(Ty::Underline, V::Body(bx(arg()))),
            Handler::Rule => {
                let _shift = arg();
                let size = |n: Option<Node>| -> R<SizeData> {
                    match n.map(|n| n.v) {
                        Some(V::SizeData(d)) => Ok(d),
                        _ => Err(JsError),
                    }
                };
                let width = size(arg())?;
                let height = size(arg())?;
                node(Ty::Rule, V::Rule { width, height })
            }
            Handler::Katex => node(Ty::Katex, V::Empty),
            Handler::Phantom => node(Ty::Phantom, V::Inner(inner(arg())?)),
            Handler::ColorFn => node(Ty::Color, V::Inner(inner(arg())?)),
            Handler::OpNoLimitsNoSymbol => node(Ty::Op, V::Op { limits: false, symbol: false, body: name.to_vec() }),
            Handler::OpLimitsNoSymbol => node(Ty::Op, V::Op { limits: true, symbol: false, body: name.to_vec() }),
            Handler::OpNoLimitsSymbol => node(Ty::Op, V::Op { limits: false, symbol: true, body: name.to_vec() }),
            Handler::OpLimitsSymbol => node(Ty::Op, V::Op { limits: true, symbol: true, body: name.to_vec() }),
            Handler::Genfrac => {
                let numer = arg();
                let denom = arg();
                let bar = ["\\dfrac", "\\frac", "\\tfrac"].iter().any(|s| eq(name, s));
                node(Ty::Genfrac, V::Genfrac { numer: bx(numer), denom: bx(denom), bar })
            }
            Handler::Lap => {
                let ty = if eq(name, "\\llap") { Ty::Llap } else { Ty::Rlap };
                node(ty, V::Body(bx(arg())))
            }
            Handler::Delim => {
                let delim = arg().ok_or(JsError)?;
                let value = match delim.v {
                    V::Str(s) if DELIMITERS.iter().any(|d| eq(&s, d)) => s,
                    _ => return Err(JsError), // Invalid delimiter
                };
                if eq(name, "\\left") || eq(name, "\\right") {
                    node(Ty::Leftright, V::Delim { value, func_name: name.to_vec() })
                } else {
                    node(Ty::Delimsizing, V::Delim { value, func_name: name.to_vec() })
                }
            }
            Handler::Null => return Err(JsError), // handler is not a function
            Handler::Font => {
                let body = arg();
                let func = if eq(name, "\\Bbb") {
                    w("\\mathbb")
                } else if eq(name, "\\bold") {
                    w("\\mathbf")
                } else if eq(name, "\\frak") {
                    w("\\mathfrak")
                } else {
                    name.to_vec()
                };
                node(Ty::Font, V::Font { font: func[1..].to_vec(), body: bx(body) })
            }
            Handler::Accent => node(Ty::Accent, V::Accent { accent: name.to_vec(), base: bx(arg()) }),
            Handler::Infix => {
                let replace_with = if eq(name, "\\over") { "\\frac" } else { "\\binom" };
                node(Ty::Infix, V::Infix { replace_with })
            }
            Handler::Cr => node(Ty::Cr, V::Empty),
            Handler::Environment => {
                let g = arg().ok_or(JsError)?;
                if g.ty != Ty::Ordgroup {
                    return Err(JsError); // Invalid environment name
                }
                let V::List(l) = &g.v else { return Err(JsError) };
                let mut name = W::new();
                for n in l {
                    name.extend(js_string(&n.v));
                }
                node(Ty::Environment, V::Env { name })
            }
        })
    }

    // -- environments.js ----------------------------------------------------------------

    fn parse_array(&mut self, cols: Option<Vec<Option<Col>>>) -> R<Node> {
        self.enter(W_ARRAY)?;
        let r = self.parse_array_(cols);
        self.leave(W_ARRAY);
        r
    }

    fn parse_array_(&mut self, cols: Option<Vec<Option<Col>>>) -> R<Node> {
        let mut body: Vec<Vec<Node>> = vec![Vec::new()];
        loop {
            let cell = self.parse_expression(false, None)?;
            body.last_mut().unwrap().push(node(Ty::Ordgroup, V::List(cell)));
            let next = &self.next.text;
            if eq(next, "&") {
                self.consume()?;
            } else if eq(next, "\\end") || eq(next, "}") {
                break;
            } else if eq(next, "\\\\") || eq(next, "\\cr") {
                let _cr = self.parse_function(None)?.ok_or(JsError)?;
                body.push(Vec::new());
            } else {
                return Err(JsError); // Expected & or \\ or \end
            }
        }
        Ok(node(Ty::Array, V::Array { cols, body }))
    }

    fn env_handler(&mut self, h: EnvHandler, name: &[u16], args: &[Option<Node>]) -> R<Node> {
        match h {
            EnvHandler::Array | EnvHandler::Tabular => {
                let colalign = args.first().cloned().flatten().ok_or(JsError)?;
                let nodes = match colalign.v {
                    V::List(l) => l,
                    _ => vec![colalign],
                };
                let lcr = w("lcr");
                let cols = nodes
                    .iter()
                    .map(|n| {
                        let ca = js_string(&n.v);
                        let found = ca.is_empty() || lcr.windows(ca.len().max(1)).any(|x| x == ca.as_slice());
                        let found = found && ca.len() <= 3;
                        if found {
                            Some(match &n.v {
                                V::Str(s) => Col::Align(s.clone()),
                                _ => Col::AlignEmptyArray,
                            })
                        } else if matches!(&n.v, V::Str(s) if eq(s, "|")) {
                            Some(Col::Separator)
                        } else {
                            None
                        }
                    })
                    .collect();
                self.parse_array(Some(cols))
            }
            EnvHandler::Matrix => {
                let res = self.parse_array(None)?;
                let delims = match String::from_utf16_lossy(name).as_str() {
                    "pmatrix" => Some(("(", ")")),
                    "bmatrix" => Some(("[", "]")),
                    "Bmatrix" => Some(("\\{", "\\}")),
                    "vmatrix" => Some(("|", "|")),
                    "Vmatrix" => Some(("\\Vert", "\\Vert")),
                    _ => None,
                };
                Ok(match delims {
                    Some((l, r)) => node(Ty::Leftright, V::LeftRight { body: vec![res], left: w(l), right: w(r) }),
                    None => res,
                })
            }
            EnvHandler::Cases => {
                let cols = vec![Some(Col::Align(w("l"))), Some(Col::Align(w("l")))];
                let res = self.parse_array(Some(cols))?;
                Ok(node(Ty::Leftright, V::LeftRight { body: vec![res], left: w("\\{"), right: w(".") }))
            }
            EnvHandler::Aligned => {
                let mut res = self.parse_array(Some(Vec::new()))?;
                let V::Array { cols, body } = &mut res.v else { unreachable!() };
                let mut num_cols = 0;
                for row in body.iter_mut() {
                    let mut i = 1;
                    while i < row.len() {
                        if let V::List(l) = &mut row[i].v {
                            l.insert(0, node(Ty::Ordgroup, V::List(Vec::new())));
                        }
                        i += 2;
                    }
                    num_cols = num_cols.max(row.len());
                }
                let cols = cols.as_mut().unwrap();
                for i in 0..num_cols {
                    let a = if i % 2 == 1 { "l" } else { "r" };
                    let c = Some(Col::Align(w(a)));
                    if i < cols.len() {
                        cols[i] = c;
                    } else {
                        cols.push(c);
                    }
                }
                Ok(res)
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// The LaTeX renderer of preprocess_formula.js (`norm_str`)

struct Builder {
    out: W,
    frames: usize,
}

/// `options.font == "mathrm"` is all the renderer asks of its options.
#[derive(Clone, Copy, PartialEq)]
enum Font {
    Other,
    Mathrm,
}

fn with_font(f: &[u16]) -> Font {
    if eq(f, "mathrm") {
        Font::Mathrm
    } else {
        Font::Other
    }
}

/// `String(number)` for a JS number
fn js_number(x: f64) -> String {
    let mut b = ryu_js::Buffer::new();
    b.format(x).to_string()
}

impl Builder {
    fn s(&mut self, s: &str) {
        push(&mut self.out, s);
    }

    fn enter(&mut self, n: usize) -> R<()> {
        self.frames += n;
        if self.frames > MAX_FRAMES {
            return Err(JsError);
        }
        Ok(())
    }

    fn expression(&mut self, e: &[Node], font: Font) -> R<()> {
        self.enter(B_EXPRESSION)?;
        for g in e {
            self.group(Some(g), font)?;
        }
        self.frames -= B_EXPRESSION;
        Ok(())
    }

    fn group(&mut self, g: Option<&Node>, font: Font) -> R<()> {
        self.enter(B_GROUP)?;
        let r = stacker::maybe_grow(64 * 1024, 4 * 1024 * 1024, || self.group_(g, font));
        self.frames -= B_GROUP;
        r
    }

    fn opt(&mut self, g: &Opt, font: Font) -> R<()> {
        self.group(g.as_deref(), font)
    }

    fn group_(&mut self, g: Option<&Node>, font: Font) -> R<()> {
        let g = g.ok_or(JsError)?;
        match (g.ty, &g.v) {
            (Ty::Mathord, V::Str(s)) => {
                if font == Font::Mathrm {
                    for &c in s {
                        self.out.push(c);
                        if c == b' ' as u16 {
                            self.s("; ");
                        } else {
                            self.s(" ");
                        }
                    }
                } else {
                    self.out.extend_from_slice(s);
                    self.s(" ");
                }
            }
            (Ty::Textord | Ty::Bin | Ty::Rel | Ty::Open | Ty::Close | Ty::Inner | Ty::Punct, V::Str(s)) => {
                self.out.extend_from_slice(s);
                self.s(" ");
            }
            (Ty::Ordgroup, V::List(l)) => {
                self.s("{ ");
                self.expression(l, font)?;
                self.s("} ");
            }
            (Ty::Text, V::Inner(body)) => {
                self.s("\\mathrm { ");
                self.expression(body, font)?;
                self.s("} ");
            }
            (Ty::Supsub, V::SupSub { base, sup, sub }) => {
                self.opt(base, font)?;
                if let Some(sub) = sub {
                    self.s("_ ");
                    if sub.ty != Ty::Ordgroup {
                        self.s(" { ");
                        self.group(Some(sub), font)?;
                        self.s("} ");
                    } else {
                        self.group(Some(sub), font)?;
                    }
                }
                if let Some(sup) = sup {
                    self.s("^ ");
                    if sup.ty != Ty::Ordgroup {
                        self.s(" { ");
                        self.group(Some(sup), font)?;
                        self.s("} ");
                    } else {
                        self.group(Some(sup), font)?;
                    }
                }
            }
            (Ty::Genfrac, V::Genfrac { numer, denom, bar }) => {
                self.s(if *bar { "\\frac " } else { "\\binom " });
                self.opt(numer, font)?;
                self.opt(denom, font)?;
            }
            (Ty::Array, V::Array { cols, body }) => {
                self.s("\\begin{array} { ");
                match cols {
                    Some(cols) => {
                        for c in cols {
                            match c {
                                Some(Col::Align(a)) => {
                                    self.out.extend_from_slice(a);
                                    self.s(" ");
                                }
                                Some(Col::AlignEmptyArray) => self.s(" "),
                                _ => {}
                            }
                        }
                    }
                    None => {
                        for _ in &body[0] {
                            self.s("l ");
                        }
                    }
                }
                self.s("} ");
                for row in body {
                    let nonempty = row.iter().any(|c| matches!(&c.v, V::List(l) if !l.is_empty()));
                    if nonempty {
                        for cell in row {
                            self.group(Some(cell), font)?;
                            let n = self.out.len();
                            if n > 4 && eq(&self.out[n - 4..], "{ } ") {
                                self.out.truncate(n - 4);
                            }
                            self.s("& ");
                        }
                        let n = self.out.len();
                        self.out.truncate(n.saturating_sub(2));
                        self.s("\\\\ ");
                    }
                }
                self.s("\\end{array} ");
            }
            (Ty::Sqrt, V::Sqrt { body, index }) => {
                if let Some(index) = index {
                    self.s("\\sqrt [ ");
                    match &index.v {
                        V::List(l) => self.expression(l, font)?,
                        _ => return Err(JsError),
                    }
                    self.s("] ");
                    self.opt(body, font)?;
                } else {
                    self.s("\\sqrt ");
                    self.opt(body, font)?;
                }
            }
            (Ty::Leftright, V::LeftRight { body, left, right }) => {
                self.s("\\left");
                self.out.extend_from_slice(left);
                self.s(" ");
                self.expression(body, font)?;
                self.s("\\right");
                self.out.extend_from_slice(right);
                self.s(" ");
            }
            (Ty::Accent, V::Accent { accent, base }) => {
                let b = base.as_deref().ok_or(JsError)?;
                self.out.extend_from_slice(accent);
                if b.ty != Ty::Ordgroup {
                    self.s(" { ");
                    self.group(Some(b), font)?;
                    self.s("} ");
                } else {
                    self.s(" ");
                    self.group(Some(b), font)?;
                }
            }
            (Ty::Spacing, V::Str(s)) => {
                if eq(s, " ") {
                    self.s("~ ");
                } else {
                    self.out.extend_from_slice(s);
                    self.s(" ");
                }
            }
            (Ty::Op, V::Op { limits, symbol, body }) => {
                if *symbol {
                    self.out.extend_from_slice(body);
                    self.s(" ");
                } else {
                    self.s(if !*limits { "\\operatorname { " } else { "\\operatorname* { " });
                    for &c in body.iter().skip(1) {
                        self.out.push(c);
                        self.s(" ");
                    }
                    self.s("} ");
                }
            }
            (Ty::Font, V::Font { font: f, body }) => {
                self.s("\\");
                self.out.extend_from_slice(f);
                self.s(" ");
                self.opt(body, with_font(f))?;
            }
            (Ty::Delimsizing, V::Delim { value, func_name }) => {
                self.out.extend_from_slice(func_name);
                self.s(" ");
                self.out.extend_from_slice(value);
                self.s(" ");
            }
            (Ty::Styling, V::Implicit { original, value }) => {
                self.s(" ");
                self.out.extend_from_slice(original);
                self.s(" ");
                self.expression(value, font)?;
            }
            (Ty::Sizing, V::Implicit { original, value }) => {
                if eq(original, "\\rm") {
                    self.s("\\mathrm { ");
                    self.expression(value, Font::Mathrm)?;
                    self.s("} ");
                } else {
                    self.s(" ");
                    self.out.extend_from_slice(original);
                    self.s(" ");
                    self.expression(value, font)?;
                }
            }
            (Ty::Overline, V::Body(b)) => {
                self.s("\\overline { ");
                self.opt(b, font)?;
                self.s("} ");
            }
            (Ty::Underline, V::Body(b)) => {
                self.s("\\underline { ");
                self.opt(b, font)?;
                self.s("} ");
            }
            (Ty::Rule, V::Rule { width, height }) => {
                let s = format!(
                    "\\rule {{ {} {}  }} {{ {} {} }} ",
                    js_number(width.number),
                    String::from_utf16_lossy(&width.unit),
                    js_number(height.number),
                    String::from_utf16_lossy(&height.unit)
                );
                self.s(&s);
            }
            (Ty::Llap, V::Body(b)) => {
                self.s("\\llap ");
                self.opt(b, font)?;
            }
            (Ty::Rlap, V::Body(b)) => {
                self.s("\\rlap ");
                self.opt(b, font)?;
            }
            (Ty::Phantom, V::Inner(v)) => {
                self.s("\\phantom { ");
                self.expression(v, font)?;
                self.s("} ");
            }
            // color (mathMLTree is not defined), katex, and the types without a renderer
            // (environment, infix, cr, size), ops / accents from the symbol table and
            // `\left` / `\right` used as plain functions all throw.
            _ => return Err(JsError),
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------
// String helpers for the line preprocessing (first-match JS `replace`)

fn find(h: &[u16], n: &[u16], from: usize) -> Option<usize> {
    if n.is_empty() {
        return Some(from.min(h.len()));
    }
    if h.len() < n.len() {
        return None;
    }
    (from..=h.len() - n.len()).find(|&i| h[i..i + n.len()] == *n)
}

fn contains(h: &[u16], n: &str) -> bool {
    find(h, &w(n), 0).is_some()
}

/// `s.replace(pat, rep)` for a literal pattern (first occurrence); true if replaced.
fn replace_first(s: &mut W, pat: &[u16], rep: &[u16]) -> bool {
    match find(s, pat, 0) {
        Some(i) => {
            s.splice(i..i + pat.len(), rep.iter().copied());
            true
        }
        None => false,
    }
}

/// `s.replace(/PREFIX.*?SUFFIX/, "")` with `.` not matching line terminators.
fn remove_lazy(s: &mut W, prefix: &[u16], suffix: &[u16]) -> bool {
    let mut from = 0;
    while let Some(i) = find(s, prefix, from) {
        let mut j = i + prefix.len();
        loop {
            if j + suffix.len() <= s.len() && s[j..j + suffix.len()] == *suffix {
                s.drain(i..j + suffix.len());
                return true;
            }
            if j >= s.len() || !js_dot(s[j]) {
                break;
            }
            j += 1;
        }
        from = i + 1;
    }
    false
}

/// The per-line work of the `rl.on('line', ...)` handler; returns what `console.log`
/// prints (without the newline).
fn process_line(line: &[u16], tabular: bool) -> W {
    let mut line: W = line.to_vec();
    if line.first() == Some(&(b'%' as u16)) {
        line.remove(0);
    }
    if !tabular {
        if let Some(i) = find(&line, &w("%"), 0) {
            line.truncate(i);
        }
    }
    // line.split('\\~').join(' ')
    {
        let pat = w("\\~");
        let mut out = W::with_capacity(line.len());
        let mut i = 0;
        while i < line.len() {
            if line[i..].starts_with(&pat) {
                out.push(b' ' as u16);
                i += 2;
            } else {
                out.push(line[i]);
                i += 1;
            }
        }
        line = out;
    }
    let (gt, dollar, label_open, label_close) = (w("\\>"), w("$"), w("\\label{"), w("}"));
    let sp = w(" ");
    for _ in 0..300 {
        let mut changed = replace_first(&mut line, &gt, &sp);
        if !tabular {
            changed |= replace_first(&mut line, &dollar, &sp);
        }
        changed |= remove_lazy(&mut line, &label_open, &label_close);
        if !changed {
            break;
        }
    }
    if !contains(&line, "matrix") && !contains(&line, "cases") && !contains(&line, "array") && !contains(&line, "begin") {
        let (bb, bc) = (w("\\\\"), w("\\,"));
        for _ in 0..300 {
            if !replace_first(&mut line, &bb, &bc) {
                break;
            }
        }
    }
    line.push(b' ' as u16);
    let result = if tabular { tokenize(&line) } else { normalize(line) };
    result.unwrap_or_default()
}

fn tokenize(line: &[u16]) -> R<W> {
    let mut p = Parser::new(line, true);
    p.parse()?;
    let mut out = p.gs;
    remove_lazy(&mut out, &w("\\label { "), &w(" }"));
    Ok(out)
}

fn normalize(mut line: W) -> R<W> {
    let (a, b, c, rep) = (w("{\\rm"), w("{ \\rm"), w("\\rm{"), w("\\mathrm{"));
    for _ in 0..300 {
        let mut changed = replace_first(&mut line, &a, &rep);
        changed |= replace_first(&mut line, &b, &rep);
        changed |= replace_first(&mut line, &c, &rep);
        if !changed {
            break;
        }
    }
    let tree = {
        let mut p = Parser::new(&line, false);
        p.parse()?
    };
    let mut b = Builder { out: W::new(), frames: 0 };
    b.expression(&tree, Font::Other)?;
    let mut out = b.out;
    let (s6, s6s, dol) = (w("SSSSSS"), w(" S S S S S S"), w("$"));
    for _ in 0..300 {
        let mut changed = replace_first(&mut out, &s6, &dol);
        changed |= replace_first(&mut out, &s6s, &dol);
        if !changed {
            break;
        }
    }
    remove_lazy(&mut out, &w("\\label { "), &w(" }"));
    Ok(out)
}

/// Which script: `preprocess_formula.js normalize` or `preprocess_tabular.js tokenize`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Script {
    FormulaNormalize,
    TabularTokenize,
}

/// What `node <script> <mode>` writes to stdout for the given stdin text: readline
/// splits the input on `\r\n`, `\n` and `\r` (a last line without terminator counts
/// when non-empty), and each line prints one line.
pub fn run(input: &str, script: Script) -> String {
    let u: W = w(input);
    let mut lines: Vec<&[u16]> = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < u.len() {
        let c = u[i];
        if c == 0x0a || c == 0x0d {
            lines.push(&u[start..i]);
            if c == 0x0d && u.get(i + 1) == Some(&0x0a) {
                i += 1;
            }
            start = i + 1;
        }
        i += 1;
    }
    if start < u.len() {
        lines.push(&u[start..]);
    }
    let mut out = String::new();
    for l in lines {
        let r = process_line(l, script == Script::TabularTokenize);
        out.push_str(&String::from_utf16_lossy(&r));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(s: &str) -> String {
        run(s, Script::FormulaNormalize)
    }

    #[test]
    fn basics() {
        assert_eq!(n("x^2"), "x ^  { 2 } \n");
        assert_eq!(n("\\frac{a}{b}"), "\\frac { a } { b } \n");
        assert_eq!(n("\\sqrt"), "\n");
    }
}
