//! Bit-exact port of
//!
//! ```python
//! from pylatexenc.latex2text import LatexNodes2Text
//! LatexNodes2Text().latex_to_text(s)
//! ```
//!
//! for pylatexenc 2.10 with its default configuration (default latexwalker
//! and latex2text contexts, tolerant parsing, `math_mode='text'`,
//! `strict_latex_spaces='macros'`, no comments, no fill, no braced groups),
//! as run by Python 3.11 (Unicode 14).
//!
//! [`latex_to_text`] returns `Err` exactly when the Python call raises. All
//! positions are counted in Unicode code points, as in Python.
//!
//! The specification tables and the Unicode tables (Python's `isspace`,
//! `isalpha`, regex `\w`, `upper`, NFC compositions) live in
//! `src/generated.rs`, produced by `tools/gen_specs.py`. Behaviour that is a
//! Python callable in pylatexenc is ported by hand below (see `Call`).
//!
//! Python raises `RecursionError` on deeply nested input. This port emulates
//! that by counting the Python frames the reference implementation would have
//! on the stack (see [`PY_RECURSION_LIMIT`] and [`PY_BASE_DEPTH`]); the
//! threshold therefore matches Python only up to a few frames, and in Python
//! itself it depends on the caller's stack depth.

use std::cell::Cell;
use std::fmt;

#[rustfmt::skip]
mod generated;
use generated::*;

/// Error returned when the Python call would raise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// Python exception class name that would have been raised.
    pub kind: &'static str,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

fn err(kind: &'static str, message: impl Into<String>) -> Error {
    Error {
        kind,
        message: message.into(),
    }
}

/// Python's default recursion limit.
pub const PY_RECURSION_LIMIT: usize = 1000;
/// Python stack depth (1 = module frame) of the
/// `LatexNodes2Text.latex_to_text` frame assumed by [`latex_to_text`]: 4 is
/// a script whose module code calls `f()`, which calls `g(s)`, which calls
/// `LatexNodes2Text().latex_to_text(s)`. With it, the first failing nesting
/// levels match Python 3.11 (e.g. 249 nested braces, 142 nested `\frac{`,
/// 330 nested environments, 331 nested `\(`); see `depth_tests`. A deeper
/// real caller fails slightly earlier in Python: use
/// [`latex_to_text_at_depth`] to model it.
pub const PY_BASE_DEPTH: usize = 4;

/// Convert LaTeX to text exactly like
/// `LatexNodes2Text().latex_to_text(s)` (pylatexenc 2.10, defaults).
pub fn latex_to_text(s: &str) -> Result<String, Error> {
    latex_to_text_at_depth(s, PY_BASE_DEPTH)
}

/// Same as [`latex_to_text`], with an explicit Python stack depth of the
/// `latex_to_text` frame (only affects when deep nesting raises
/// `RecursionError`).
pub fn latex_to_text_at_depth(s: &str, base_depth: usize) -> Result<String, Error> {
    // Unoptimized builds use far more stack per nesting level than release
    // builds (~25 KiB vs ~1.5 KiB); run potentially deep inputs on a thread
    // with a large stack there. In release builds the emulated recursion
    // limit keeps the native stack use below ~0.5 MiB.
    if cfg!(debug_assertions) && opener_count(s) > 24 {
        return std::thread::scope(|sc| {
            std::thread::Builder::new()
                .stack_size(256 << 20)
                .spawn_scoped(sc, || convert(s, base_depth).1)
                .expect("spawn")
                .join()
                .expect("latex_to_text thread panicked")
        });
    }
    convert(s, base_depth).1
}

/// Upper bound on the nesting depth of `s`.
fn opener_count(s: &str) -> usize {
    s.chars()
        .filter(|&c| matches!(c, '{' | '[' | '$' | '\\'))
        .count()
}

fn convert(s: &str, base_depth: usize) -> (usize, Result<String, Error>) {
    let chars: Vec<char> = s.chars().collect();
    let depth = Depth { max: Cell::new(0) };
    let r = (|| {
        let w = Walker {
            s: &chars,
            depth: &depth,
        };
        // LatexWalker(latex).get_latex_nodes() at base+1; parsing frames are
        // offset by PARSE_EXTRA to account for C-level recursion checks
        let (nodes, _, _) = w.get_latex_nodes(
            0,
            Stop::None,
            PState::default(),
            base_depth + 1 + PARSE_EXTRA,
        )?;
        let mut l = L2T {
            in_eq: false,
            title: None,
            author: None,
            date: None,
            depth: &depth,
        };
        l.nodelist_to_text(&nodes.iter().map(Some).collect::<Vec<_>>(), base_depth + 1)
    })();
    (depth.max.get(), r)
}

/// Maximum emulated Python stack depth reached while converting `s`
/// (for testing the recursion emulation), or the error.
#[doc(hidden)]
pub fn py_max_depth(s: &str, base_depth: usize) -> (usize, Result<String, Error>) {
    convert(s, base_depth)
}

// ---------------------------------------------------------------------------
// Emulated Python stack depth
// ---------------------------------------------------------------------------

/// Extra depth added to the parsing phase: Python 3.11 also counts some
/// C-level calls (e.g. class instantiation through `type.__call__`) against
/// the recursion limit; calibrated against measurements.
const PARSE_EXTRA: usize = 2;

struct Depth {
    max: Cell<usize>,
}

impl Depth {
    /// A Python frame at depth `d` is entered.
    #[inline]
    fn at(&self, d: usize) -> Result<(), Error> {
        if d > self.max.get() {
            self.max.set(d);
        }
        if d > PY_RECURSION_LIMIT {
            return Err(err("RecursionError", "maximum recursion depth exceeded"));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Python str semantics
// ---------------------------------------------------------------------------

/// Python `str.isspace()` for one char (also `\s` in `re` and `str.strip()`).
#[inline]
pub fn py_isspace(c: char) -> bool {
    let u = c as u32;
    if u < 0x80 {
        return matches!(u, 0x09..=0x0d | 0x1c..=0x20);
    }
    PY_SPACE.binary_search(&c).is_ok()
}

fn in_ranges(r: &[(u32, u32)], u: u32) -> bool {
    match r.binary_search_by(|&(a, _)| a.cmp(&u)) {
        Ok(_) => true,
        Err(0) => false,
        Err(i) => u <= r[i - 1].1,
    }
}

/// Python `str.isalpha()` for one char.
#[inline]
pub fn py_isalpha(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphabetic();
    }
    in_ranges(PY_ALPHA, c as u32)
}

/// Python `re` `\w` for one char (str pattern, default flags).
#[inline]
fn py_isword(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphanumeric() || c == '_';
    }
    in_ranges(PY_WORD, c as u32)
}

/// Python `str.strip()` without arguments.
pub fn py_strip(s: &str) -> &str {
    s.trim_matches(py_isspace)
}

/// Python `str.upper()`.
fn py_upper(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii() {
            out.push(c.to_ascii_uppercase());
        } else {
            match PY_UPPER.binary_search_by(|&(k, _)| k.cmp(&c)) {
                Ok(i) => out.push_str(PY_UPPER[i].1),
                Err(_) => out.push(c),
            }
        }
    }
    out
}

fn nchars(s: &str) -> usize {
    s.chars().count()
}

// ---------------------------------------------------------------------------
// Spec types (data in generated.rs)
// ---------------------------------------------------------------------------

/// latexwalker argument parser of a macro/environment.
#[derive(Debug)]
pub(crate) enum WArgs {
    /// `MacroStandardArgsParser(argspec, optional_arg_no_space, args_math_mode)`
    Std {
        argspec: &'static str,
        no_space: bool,
        math: Option<&'static [Option<bool>]>,
    },
    /// `VerbatimArgsParser('verb-macro')`
    VerbMacro,
    /// `VerbatimArgsParser('verbatim-environment')`
    VerbEnv,
}

/// latex2text `simplify_repl`.
#[derive(Debug)]
pub(crate) enum Repl {
    /// `None` or a falsy value
    None,
    Str(&'static str),
    Call(Call),
}

/// Hand-ported Python callables used as `simplify_repl` in the default
/// latex2text context.
#[derive(Debug)]
pub(crate) enum Call {
    Title,
    Author,
    Date,
    Maketitle,
    Item,
    Href,
    Part,
    Chapter,
    Section,
    Subsection,
    Subsubsection,
    Paragraph,
    Subparagraph,
    Texorpdfstring,
    Percent,
    Input,
    Today,
    Uebung,
    MathStyle(&'static str),
    Placeholder(&'static str),
    Accent(char),
    Matrix,
    Equation,
}

fn walker_macro(name: &str) -> Option<&'static WArgs> {
    WALKER_MACROS
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| &WALKER_MACROS[i].1)
}

fn walker_env(name: &str) -> Option<(&'static WArgs, bool)> {
    WALKER_ENVS
        .binary_search_by(|(n, _, _)| (*n).cmp(name))
        .ok()
        .map(|i| (&WALKER_ENVS[i].1, WALKER_ENVS[i].2))
}

fn l2t_macro(name: &str) -> Option<(&'static Repl, bool)> {
    L2T_MACROS
        .binary_search_by(|(n, _, _)| (*n).cmp(name))
        .ok()
        .map(|i| (&L2T_MACROS[i].1, L2T_MACROS[i].2))
}

fn l2t_env(name: &str) -> Option<(&'static Repl, bool)> {
    L2T_ENVS
        .binary_search_by(|(n, _, _)| (*n).cmp(name))
        .ok()
        .map(|i| (&L2T_ENVS[i].1, L2T_ENVS[i].2))
}

fn l2t_specials(chars: &str) -> Option<&'static str> {
    L2T_SPECIALS
        .iter()
        .find(|(n, _)| *n == chars)
        .map(|(_, r)| *r)
}

// ---------------------------------------------------------------------------
// latexwalker
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug)]
struct PState {
    in_math: bool,
    /// math_mode_delimiter == '$'
    dollar: bool,
}

impl PState {
    /// `sub_context(in_math_mode=False)` (sanitized: delimiter dropped)
    fn text(self) -> PState {
        PState {
            in_math: false,
            dollar: false,
        }
    }
}

#[derive(Debug)]
enum Tk {
    Char(String),
    Macro(String, String),
    /// comment post_space (the comment text itself is never output)
    Comment(String),
    BraceOpen(char),
    BraceClose(char),
    Math(&'static str),
    Specials(&'static str),
    BeginEnv(String),
    EndEnv(String),
}

#[derive(Debug)]
struct Token {
    tk: Tk,
    pos: usize,
    len: usize,
    pre: String,
}

enum PErr {
    /// LatexWalkerEndOfStream(final_space)
    Eos(String),
    /// LatexWalkerParseError
    Parse,
    /// any other exception: propagates to the caller of latex_to_text
    Fatal(Error),
}

impl From<Error> for PErr {
    fn from(e: Error) -> PErr {
        PErr::Fatal(e)
    }
}

#[derive(Debug)]
pub(crate) struct Args {
    spec: &'static str,
    list: Vec<Option<Node>>,
}

#[derive(Debug)]
pub(crate) enum Node {
    Chars(String),
    Group(Vec<Node>),
    Comment(String),
    Macro {
        name: String,
        post: String,
        args: Option<Args>,
        expr: bool,
    },
    Env {
        name: String,
        nodes: Vec<Node>,
        args: Option<Args>,
    },
    Specials(&'static str),
    Math {
        display: bool,
        nodes: Vec<Node>,
    },
}

/// Result of `do_read`: continue, stop (r_endnow=True), or end of stream.
enum Rd {
    Cont,
    End,
    Eos(String),
}

#[derive(Clone, Copy, PartialEq)]
enum Stop<'a> {
    None,
    Brace(char),
    Env(&'a str),
    Math(&'static str),
}

struct Walker<'a> {
    s: &'a [char],
    depth: &'a Depth,
}

fn collect(s: &[char]) -> String {
    s.iter().collect()
}

impl<'a> Walker<'a> {
    fn starts_with(&self, pos: usize, pat: &str) -> bool {
        let mut i = pos;
        for c in pat.chars() {
            if i >= self.s.len() || self.s[i] != c {
                return false;
            }
            i += 1;
        }
        true
    }

    /// `LatexWalker.get_token` (tolerant parsing). `d` is the Python frame
    /// depth of get_token.
    fn get_token(
        &self,
        pos0: usize,
        brackets: bool,
        environments: bool,
        ps: PState,
        d: usize,
    ) -> Result<Token, PErr> {
        let s = self.s;
        let n = s.len();
        let mut pos = pos0;
        self.depth.at(d)?;
        // LatexToken.__init__
        self.depth.at(d + 1)?;
        let mut space = String::new();
        let mut space_nl = 0usize; // trailing "\n" count is tracked through `space`
        while pos < n && py_isspace(s[pos]) {
            space.push(s[pos]);
            pos += 1;
            if s[pos - 1] == '\n' {
                space_nl += 1;
            } else {
                space_nl = 0;
            }
            if space_nl >= 2 {
                space.truncate(space.len() - 2);
                return Ok(Token {
                    tk: Tk::Char("\n\n".into()),
                    pos: pos - 2,
                    len: 2,
                    pre: space,
                });
            }
        }
        if pos >= n {
            return Err(PErr::Eos(space));
        }
        let c = s[pos];
        if c == '\\' {
            if pos + 1 >= n {
                return Err(PErr::Eos(String::new()));
            }
            let mut mac = String::new();
            mac.push(s[pos + 1]);
            let mut i = 2;
            let isalpha = py_isalpha(s[pos + 1]);
            if isalpha {
                while pos + i < n && py_isalpha(s[pos + i]) {
                    mac.push(s[pos + i]);
                    i += 1;
                }
            }
            match mac.as_str() {
                "[" => {
                    return Ok(Token {
                        tk: Tk::Math("\\["),
                        pos,
                        len: i,
                        pre: space,
                    })
                }
                "]" => {
                    return Ok(Token {
                        tk: Tk::Math("\\]"),
                        pos,
                        len: i,
                        pre: space,
                    })
                }
                "(" => {
                    return Ok(Token {
                        tk: Tk::Math("\\("),
                        pos,
                        len: i,
                        pre: space,
                    })
                }
                ")" => {
                    return Ok(Token {
                        tk: Tk::Math("\\)"),
                        pos,
                        len: i,
                        pre: space,
                    })
                }
                _ => {}
            }
            if environments && (mac == "begin" || mac == "end") {
                // re.match -> re._compile
                self.depth.at(d + 2)?;
                // r'^\s*\{([\w* ._-]+)\}' on s[pos+i:]
                let mut j = pos + i;
                while j < n && py_isspace(s[j]) {
                    j += 1;
                }
                let mut matched = None;
                if j < n && s[j] == '{' {
                    let k0 = j + 1;
                    let mut k = k0;
                    while k < n && (py_isword(s[k]) || matches!(s[k], '*' | ' ' | '.' | '_' | '-'))
                    {
                        k += 1;
                    }
                    if k > k0 && k < n && s[k] == '}' {
                        matched = Some((collect(&s[k0..k]), k + 1 - pos));
                    }
                }
                return match matched {
                    None => {
                        // _token_parse_error -> LatexWalkerParseError -> _dispstr -> _fmt_pos
                        self.depth.at(d + 4)?;
                        Ok(Token {
                            tk: Tk::Char(format!("\\{}", mac)),
                            pos,
                            len: i,
                            pre: space,
                        })
                    }
                    Some((name, len)) => Ok(Token {
                        tk: if mac == "begin" {
                            Tk::BeginEnv(name)
                        } else {
                            Tk::EndEnv(name)
                        },
                        pos,
                        len,
                        pre: space,
                    }),
                };
            }
            let mut post = String::new();
            if isalpha {
                let mut nl = 0;
                while pos + i < n && py_isspace(s[pos + i]) {
                    let ch = s[pos + i];
                    post.push(ch);
                    i += 1;
                    if ch == '\n' {
                        nl += 1;
                    } else {
                        nl = 0;
                    }
                    if nl >= 2 {
                        post.truncate(post.len() - 2);
                        i -= 2;
                        break;
                    }
                }
            }
            return Ok(Token {
                tk: Tk::Macro(mac, post),
                pos,
                len: i,
                pre: space,
            });
        }
        if c == '%' {
            // re.compile(...).search: re.compile -> re._compile
            self.depth.at(d + 2)?;
            let mut k = pos;
            while k < n && s[k] != '\n' && s[k] != '\r' {
                k += 1;
            }
            let (arglen, mlen, mspace) = if k < n {
                let mut e = k + 1;
                while e < n && py_isspace(s[e]) {
                    e += 1;
                }
                if k + 1 < e && (s[k + 1] == '\n' || s[k + 1] == '\r') {
                    (k - pos, k - pos, String::new())
                } else {
                    (k - pos, e - pos, collect(&s[k..e]))
                }
            } else {
                (n - pos, n - pos, String::new())
            };
            let _comment = &s[pos + 1..pos + arglen];
            return Ok(Token {
                tk: Tk::Comment(mspace),
                pos,
                len: mlen,
                pre: space,
            });
        }
        if c == '{' || (brackets && c == '[') {
            return Ok(Token {
                tk: Tk::BraceOpen(c),
                pos,
                len: 1,
                pre: space,
            });
        }
        if c == '}' || (brackets && c == ']') {
            return Ok(Token {
                tk: Tk::BraceClose(c),
                pos,
                len: 1,
                pre: space,
            });
        }
        if c == '$' {
            if pos + 1 < n && s[pos + 1] == '$' && !(ps.in_math && ps.dollar) {
                return Ok(Token {
                    tk: Tk::Math("$$"),
                    pos,
                    len: 2,
                    pre: space,
                });
            }
            return Ok(Token {
                tk: Tk::Math("$"),
                pos,
                len: 1,
                pre: space,
            });
        }
        // latex_context.test_for_specials
        self.depth.at(d + 1)?;
        let mut best: Option<&'static str> = None;
        let mut best_len = 0;
        for sp in WALKER_SPECIALS {
            let l = sp.chars().count();
            if l > best_len && self.starts_with(pos, sp) {
                best = Some(sp);
                best_len = l;
            }
        }
        if let Some(sp) = best {
            return Ok(Token {
                tk: Tk::Specials(sp),
                pos,
                len: best_len,
                pre: space,
            });
        }
        Ok(Token {
            tk: Tk::Char(c.to_string()),
            pos,
            len: 1,
            pre: space,
        })
    }

    /// make_node(...) at depth `d` for a node class; returns the leaf depth.
    fn make_node(&self, d: usize, macro_or_env: bool) -> Result<(), Error> {
        // make_node -> XNode.__init__ -> [ParsedMacroArgs() -> _get_legacy_attribs] / LatexNode.__init__
        self.depth.at(d + if macro_or_env { 3 } else { 2 })
    }

    /// `get_latex_expression(pos, strict_braces=False, parsing_state)`;
    /// returns (node, pos, len). `d`: depth of this frame.
    fn get_latex_expression(
        &self,
        pos: usize,
        ps: PState,
        d: usize,
    ) -> Result<(Node, usize, usize), PErr> {
        self.depth.at(d + 1)?; // _PushPropOverride
        let tok = self.get_token(pos, false, false, ps, d + 1)?;
        match tok.tk {
            Tk::Macro(name, post) => {
                self.make_node(d + 2, name != "end")?;
                if name == "end" {
                    return Ok((Node::Chars(String::new()), tok.pos, 0));
                }
                Ok((
                    Node::Macro {
                        name,
                        post,
                        args: None,
                        expr: true,
                    },
                    tok.pos,
                    tok.len,
                ))
            }
            Tk::Specials(sp) => {
                self.make_node(d + 2, false)?;
                Ok((Node::Specials(sp), tok.pos, tok.len))
            }
            Tk::Comment(_) => self.get_latex_expression(tok.pos + tok.len, ps, d + 1),
            Tk::BraceOpen(_) => self.get_latex_braced_group(tok.pos, '{', ps, d + 1),
            Tk::BraceClose(_) => {
                self.make_node(d + 2, false)?;
                Ok((Node::Chars(String::new()), tok.pos, 0))
            }
            Tk::Char(c) => {
                self.make_node(d + 2, false)?;
                Ok((Node::Chars(c), tok.pos, tok.len))
            }
            Tk::Math(arg) => {
                if arg.starts_with('\\') {
                    self.make_node(d + 2, true)?;
                    Ok((
                        Node::Macro {
                            name: arg.to_string(),
                            post: String::new(),
                            args: None,
                            expr: true,
                        },
                        tok.pos,
                        tok.len,
                    ))
                } else {
                    self.make_node(d + 2, false)?;
                    Ok((Node::Chars(arg.to_string()), tok.pos, tok.len))
                }
            }
            Tk::BeginEnv(_) | Tk::EndEnv(_) => unreachable!("environments=False"),
        }
    }

    /// `get_latex_maybe_optional_arg(pos, parsing_state)`.
    fn get_latex_maybe_optional_arg(
        &self,
        pos: usize,
        ps: PState,
        d: usize,
    ) -> Result<Option<(Node, usize, usize)>, PErr> {
        let tok = match self.get_token(pos, true, false, ps, d + 1) {
            Ok(t) => t,
            Err(PErr::Eos(_)) => return Ok(None),
            Err(e) => return Err(e),
        };
        if let Tk::BraceOpen('[') = tok.tk {
            return self.get_latex_braced_group(pos, '[', ps, d + 1).map(Some);
        }
        Ok(None)
    }

    /// `get_latex_braced_group(pos, brace_type, parsing_state)`.
    fn get_latex_braced_group(
        &self,
        pos: usize,
        brace: char,
        ps: PState,
        d: usize,
    ) -> Result<(Node, usize, usize), PErr> {
        self.depth.at(d)?;
        let first = self.get_token(pos, brace == '[', true, ps, d + 1)?;
        match first.tk {
            Tk::BraceOpen(c) if c == brace => {}
            _ => {
                // LatexWalkerParseError(...)
                self.depth.at(d + 3)?;
                return Err(PErr::Parse);
            }
        }
        let close = if brace == '[' { ']' } else { '}' };
        let (nodes, npos, nlen) =
            self.get_latex_nodes(first.pos + first.len, Stop::Brace(close), ps, d + 1)?;
        self.make_node(d + 2, false)?; // _mknodeposlen -> make_node
        self.depth.at(d + 5)?;
        Ok((Node::Group(nodes), first.pos, npos + nlen - first.pos))
    }

    /// `args_parser.parse_args(w, pos, parsing_state)` for a standard or
    /// verbatim parser. `d`: depth of the args_parser.parse_args frame.
    fn parse_args(
        &self,
        wa: &WArgs,
        pos: usize,
        ps: PState,
        d: usize,
    ) -> Result<(Args, usize, usize), PErr> {
        self.depth.at(d)?;
        let s = self.s;
        match wa {
            WArgs::Std {
                argspec,
                no_space,
                math,
            } => {
                // `from .. import latexwalker` -> importlib._bootstrap._handle_fromlist
                self.depth.at(d + 1)?;
                let mut list = Vec::with_capacity(argspec.len());
                let mut p = pos;
                for (j, argt) in argspec.chars().enumerate() {
                    let inner = |depth: &Depth| -> Result<PState, Error> {
                        // get_inner_parsing_state(j)
                        depth.at(d + 1)?;
                        Ok(match math {
                            None => ps,
                            Some(amm) => match amm[j] {
                                None => ps,
                                Some(m) if m == ps.in_math => ps,
                                Some(true) => {
                                    depth.at(d + 4)?; // sub_context -> ...
                                    PState {
                                        in_math: true,
                                        dollar: ps.dollar,
                                    }
                                }
                                Some(false) => {
                                    depth.at(d + 4)?;
                                    ps.text()
                                }
                            },
                        })
                    };
                    match argt {
                        '{' => {
                            let ips = inner(self.depth)?;
                            let (node, np, nl) = self.get_latex_expression(p, ips, d + 1)?;
                            p = np + nl;
                            list.push(Some(node));
                        }
                        '[' => {
                            if *no_space && p < s.len() && py_isspace(s[p]) {
                                list.push(None);
                                continue;
                            }
                            let ips = inner(self.depth)?;
                            match self.get_latex_maybe_optional_arg(p, ips, d + 1)? {
                                None => list.push(None),
                                Some((node, np, nl)) => {
                                    p = np + nl;
                                    list.push(Some(node));
                                }
                            }
                        }
                        '*' => {
                            // w.get_token(p): default parsing state
                            let tok = self.get_token(p, false, true, PState::default(), d + 1)?;
                            match &tok.tk {
                                Tk::Char(a) if a.starts_with('*') => {
                                    let ips = inner(self.depth)?;
                                    let _ = ips;
                                    self.make_node(d + 1, false)?;
                                    list.push(Some(Node::Chars("*".into())));
                                    p = tok.pos + 1;
                                }
                                _ => list.push(None),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                // ParsedMacroArgs(...) -> _get_legacy_attribs -> all(genexpr)
                self.depth.at(d + 3)?;
                Ok((
                    Args {
                        spec: argspec,
                        list,
                    },
                    pos,
                    p - pos,
                ))
            }
            WArgs::VerbEnv => {
                self.depth.at(d + 1)?;
                // w.s.find(r'\end{verbatim}', pos)
                let pat: Vec<char> = "\\end{verbatim}".chars().collect();
                let mut found = None;
                if s.len() >= pat.len() {
                    let mut k = pos;
                    while k + pat.len() <= s.len() {
                        if s[k..k + pat.len()] == pat[..] {
                            found = Some(k);
                            break;
                        }
                        k += 1;
                    }
                }
                let Some(endpos) = found else {
                    self.depth.at(d + 3)?;
                    return Err(PErr::Parse);
                };
                self.make_node(d + 1, false)?;
                let len = endpos - pos;
                Ok((
                    Args {
                        spec: "{",
                        list: vec![Some(Node::Chars(collect(&s[pos..endpos])))],
                    },
                    pos,
                    len,
                ))
            }
            WArgs::VerbMacro => {
                self.depth.at(d + 1)?;
                let mut pos = pos;
                loop {
                    if pos >= s.len() {
                        return Err(PErr::Fatal(err("IndexError", "string index out of range")));
                    }
                    if !py_isspace(s[pos]) {
                        break;
                    }
                    pos += 1;
                    if pos >= s.len() {
                        self.depth.at(d + 3)?;
                        return Err(PErr::Parse);
                    }
                }
                let delim = s[pos];
                let begin = pos + 1;
                let Some(off) = s[begin.min(s.len())..].iter().position(|&c| c == delim) else {
                    self.depth.at(d + 3)?;
                    return Err(PErr::Parse);
                };
                let endpos = begin + off;
                self.make_node(d + 1, false)?;
                Ok((
                    Args {
                        spec: "{",
                        list: vec![Some(Node::Chars(collect(&s[begin..endpos])))],
                    },
                    pos,
                    endpos + 1 - pos,
                ))
            }
        }
    }

    /// `get_latex_environment(pos, environmentname, parsing_state)`.
    fn get_latex_environment(
        &self,
        pos: usize,
        name: &str,
        ps: PState,
        d: usize,
    ) -> Result<(Node, usize, usize), PErr> {
        self.depth.at(d)?;
        let startpos = pos;
        let first = self.get_token(pos, false, true, ps, d + 1)?;
        let pos = first.pos + first.len;
        self.depth.at(d + 1)?; // get_environment_spec
        let (wa, is_math) = match walker_env(name) {
            Some((wa, m)) => (wa, m),
            None => (
                &WArgs::Std {
                    argspec: "",
                    no_space: false,
                    math: None,
                },
                false,
            ),
        };
        // env_spec.parse_args (d+1) -> args_parser.parse_args (d+2)
        self.depth.at(d + 1)?;
        let (argd, apos, alen) = match self.parse_args(wa, pos, ps, d + 2) {
            Ok((a, p, l)) => (Some(a), p, l),
            Err(PErr::Fatal(e)) => return Err(PErr::Fatal(e)),
            Err(_) => {
                // _exchandle_parse_subexpression -> LatexWalkerParseError -> _dispstr -> _fmt_pos
                self.depth.at(d + 4)?;
                (None, pos, 0)
            }
        };
        let pos = apos + alen;
        let inner = if is_math {
            self.depth.at(d + 3)?; // sub_context
            PState {
                in_math: true,
                dollar: false,
            }
        } else {
            ps
        };
        let (nodes, npos, nlen) = self.get_latex_nodes(pos, Stop::Env(name), inner, d + 1)?;
        self.make_node(d + 2, true)?;
        Ok((
            Node::Env {
                name: name.to_string(),
                nodes,
                args: argd,
            },
            startpos,
            npos + nlen - startpos,
        ))
    }

    /// `get_latex_nodes(pos, stop_upon_..., parsing_state)` in tolerant mode
    /// (never raises a LatexWalker error). `d`: depth of this frame.
    fn get_latex_nodes(
        &self,
        pos: usize,
        stop: Stop,
        ps: PState,
        d: usize,
    ) -> Result<(Vec<Node>, usize, usize), Error> {
        self.depth.at(d)?;
        let brackets = matches!(stop, Stop::Brace(']'));
        let origpos = pos;
        let mut nodes: Vec<Node> = Vec::new();
        let mut p = pos;
        let mut lastchars = String::new();
        let dr = d + 1; // do_read

        loop {
            // ---- do_read ----
            self.depth.at(dr)?;
            let r: Result<Rd, PErr> = (|| {
                let tok = match self.get_token(p, brackets, true, ps, dr + 1) {
                    Ok(t) => t,
                    Err(PErr::Eos(fs)) => return Ok(Rd::Eos(fs)),
                    Err(e) => return Err(e),
                };
                p = tok.pos + tok.len;
                if let Tk::Char(a) = &tok.tk {
                    // push_lastchars
                    lastchars.push_str(&tok.pre);
                    lastchars.push_str(a);
                    return Ok(Rd::Cont);
                }
                if !lastchars.is_empty() {
                    let mut chars = std::mem::take(&mut lastchars);
                    chars.push_str(&tok.pre);
                    self.make_node(dr + 1, false)?;
                    nodes.push(Node::Chars(chars));
                } else if !tok.pre.is_empty() {
                    self.make_node(dr + 1, false)?;
                    nodes.push(Node::Chars(tok.pre.clone()));
                }
                match tok.tk {
                    Tk::BraceClose(c) => {
                        if stop != Stop::Brace(c) {
                            self.depth.at(dr + 3)?;
                            return Err(PErr::Parse);
                        }
                        Ok(Rd::End)
                    }
                    Tk::EndEnv(name) => {
                        match stop {
                            Stop::Env(e) if e == name => {}
                            _ => {
                                self.depth.at(dr + 3)?;
                                return Err(PErr::Parse);
                            }
                        }
                        Ok(Rd::End)
                    }
                    Tk::Math(arg) => {
                        if let Stop::Math(sm) = stop {
                            if arg == sm {
                                return Ok(Rd::End);
                            }
                            if arg == "\\)" || arg == "\\]" {
                                self.depth.at(dr + 3)?;
                                return Err(PErr::Parse);
                            }
                        } else if arg == "\\)" || arg == "\\]" {
                            self.depth.at(dr + 3)?;
                            return Err(PErr::Parse);
                        }
                        let closing: &'static str = match arg {
                            "\\(" => "\\)",
                            "\\[" => "\\]",
                            other => other,
                        };
                        let display = !(arg == "\\(" || arg == "$");
                        self.depth.at(dr + 3)?; // sub_context
                        let inner = PState {
                            in_math: true,
                            dollar: arg == "$",
                        };
                        let (mnodes, mpos, mlen) =
                            self.get_latex_nodes(p, Stop::Math(closing), inner, dr + 1)?;
                        p = mpos + mlen;
                        self.make_node(dr + 1, false)?;
                        nodes.push(Node::Math {
                            display,
                            nodes: mnodes,
                        });
                        Ok(Rd::Cont)
                    }
                    Tk::Comment(post) => {
                        self.make_node(dr + 1, false)?;
                        nodes.push(Node::Comment(post));
                        Ok(Rd::Cont)
                    }
                    Tk::BraceOpen(c) => {
                        let (g, bpos, blen) =
                            self.get_latex_braced_group(tok.pos, c, ps, dr + 1)?;
                        p = bpos + blen;
                        nodes.push(g);
                        Ok(Rd::Cont)
                    }
                    Tk::BeginEnv(name) => {
                        let (e, epos, elen) =
                            self.get_latex_environment(tok.pos, &name, ps, dr + 1)?;
                        p = epos + elen;
                        nodes.push(e);
                        Ok(Rd::Cont)
                    }
                    Tk::Macro(name, post) => {
                        self.depth.at(dr + 1)?; // get_macro_spec
                        let wa = walker_macro(&name).unwrap_or(&WArgs::Std {
                            argspec: "",
                            no_space: false,
                            math: None,
                        });
                        let start = tok.pos + tok.len;
                        // mspec.parse_args (dr+1) -> args_parser.parse_args (dr+2)
                        self.depth.at(dr + 1)?;
                        let (args, mapos, malen) = match self.parse_args(wa, start, ps, dr + 2) {
                            Ok((a, ap, al)) => (Some(a), ap, al),
                            Err(PErr::Fatal(e)) => return Err(PErr::Fatal(e)),
                            Err(_) => {
                                self.depth.at(dr + 4)?;
                                (None, start, 0)
                            }
                        };
                        p = mapos + malen;
                        self.make_node(dr + 1, true)?;
                        nodes.push(Node::Macro {
                            name,
                            post,
                            args,
                            expr: false,
                        });
                        Ok(Rd::Cont)
                    }
                    Tk::Specials(sp) => {
                        self.depth.at(dr + 1)?; // sspec.parse_args
                        self.make_node(dr + 1, false)?;
                        nodes.push(Node::Specials(sp));
                        Ok(Rd::Cont)
                    }
                    Tk::Char(_) => unreachable!(),
                }
            })();
            // ---- loop body after do_read ----
            let final_space = match r {
                Ok(Rd::Cont) => continue,
                Ok(Rd::Eos(fs)) => Some(fs),
                Ok(Rd::End) => None,
                Err(PErr::Eos(_)) => unreachable!("tolerant do_read returns EOS"),
                Err(PErr::Parse) => {
                    // _report_ignore_parse_error -> logger.info -> isEnabledFor
                    self.depth.at(d + 3)?;
                    continue;
                }
                Err(PErr::Fatal(e)) => return Err(e),
            };
            if let Some(fs) = final_space {
                lastchars.push_str(&fs);
                p += nchars(&fs);
            }
            if !lastchars.is_empty() {
                self.make_node(d + 1, false)?;
                nodes.push(Node::Chars(std::mem::take(&mut lastchars)));
            }
            return Ok((nodes, origpos, p - origpos));
        }
    }
}

// ---------------------------------------------------------------------------
// latex2text
// ---------------------------------------------------------------------------

struct L2T<'a> {
    /// inside an equation context: strict_latex_spaces = 'based-on-source'
    /// (otherwise the default 'macros')
    in_eq: bool,
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    depth: &'a Depth,
}

/// (nodeoptarg, nodeargs) of a macro node; `None` nodeargs for nodes created
/// by get_latex_expression (Python: both are None there).
type Legacy<'n> = (Option<&'n Node>, Option<Vec<Option<&'n Node>>>);

fn legacy_of(args: &Option<Args>, expr: bool) -> Legacy<'_> {
    if expr {
        return (None, None);
    }
    let Some(a) = args else {
        return (None, Some(Vec::new()));
    };
    let spec = a.spec;
    let nskip = spec.chars().take_while(|&c| c == '*').count();
    let rest = &spec[nskip..];
    let list: Vec<Option<&Node>> = a.list.iter().map(|x| x.as_ref()).collect();
    if rest.starts_with('[') && rest[1..].chars().all(|c| c == '{') {
        (list[nskip], Some(list[nskip + 1..].to_vec()))
    } else {
        (None, Some(list))
    }
}

/// Python `%`-formatting arguments.
enum FmtArgs<'a> {
    Tuple(&'a [String]),
    Dict(&'a [(String, String)]),
}

enum FmtErr {
    /// TypeError / ValueError (caught by apply_simplify_repl)
    TypeOrValue,
    /// KeyError (propagates)
    Key(String),
}

/// `fmt % args` for format strings made of literal text, `%%`, `%s` and
/// `%(key)s` (the generator checks that the default specs only use these).
fn py_percent(fmt: &str, args: FmtArgs) -> Result<String, FmtErr> {
    let mut out = String::new();
    let mut it = fmt.chars().peekable();
    let mut argi = 0usize;
    while let Some(c) = it.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('%') => out.push('%'),
            Some('(') => {
                let mut key = String::new();
                let mut depth = 1;
                loop {
                    match it.next() {
                        None => return Err(FmtErr::TypeOrValue), // incomplete format key
                        Some('(') => {
                            depth += 1;
                            key.push('(');
                        }
                        Some(')') => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                            key.push(')');
                        }
                        Some(ch) => key.push(ch),
                    }
                }
                let FmtArgs::Dict(d) = &args else {
                    return Err(FmtErr::TypeOrValue); // format requires a mapping
                };
                if it.next() != Some('s') {
                    return Err(FmtErr::TypeOrValue);
                }
                match d.iter().find(|(k, _)| *k == key) {
                    Some((_, v)) => out.push_str(v),
                    None => return Err(FmtErr::Key(key)),
                }
            }
            Some('s') => match &args {
                FmtArgs::Tuple(t) => {
                    if argi >= t.len() {
                        return Err(FmtErr::TypeOrValue); // not enough arguments
                    }
                    out.push_str(&t[argi]);
                    argi += 1;
                }
                // a mapping used with a positional conversion formats the mapping
                // itself; never reached with the default specs
                FmtArgs::Dict(_) => return Err(FmtErr::TypeOrValue),
            },
            _ => return Err(FmtErr::TypeOrValue),
        }
    }
    if let FmtArgs::Tuple(t) = &args {
        if argi < t.len() {
            return Err(FmtErr::TypeOrValue); // not all arguments converted
        }
    }
    Ok(out)
}

/// `re.search('(^|[^%])(%%)*%s', s)`
fn has_percent_s(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    for k in 0..c.len().saturating_sub(1) {
        if c[k] == '%' && c[k + 1] == 's' {
            let mut r = 0;
            while r < k && c[k - 1 - r] == '%' {
                r += 1;
            }
            if r % 2 == 0 {
                return true;
            }
        }
    }
    false
}

fn indented_block(contents: &str, indent: &str) -> String {
    // _fmt_indented_block (fill_text is None)
    let mut s = String::with_capacity(contents.len() + 2 + indent.len());
    s.push('\n');
    s.push_str(indent);
    s.push_str(&contents.replace('\n', &format!("\n{}", indent)));
    s.push('\n');
    s
}

fn math_style_char(c: char, style: &str) -> char {
    let exc: &[(char, u32)] = match style {
        "italic" => &[('h', 0x210E)],
        "script" => &[
            ('B', 0x212C),
            ('E', 0x2130),
            ('F', 0x2131),
            ('H', 0x210B),
            ('I', 0x2110),
            ('L', 0x2112),
            ('M', 0x2133),
            ('R', 0x211B),
            ('e', 0x212F),
            ('g', 0x210A),
            ('o', 0x2134),
        ],
        "fraktur" => &[
            ('C', 0x212D),
            ('H', 0x210C),
            ('I', 0x2111),
            ('R', 0x211C),
            ('Z', 0x2128),
        ],
        "doublestruck" => &[
            ('C', 0x2102),
            ('H', 0x210D),
            ('N', 0x2115),
            ('P', 0x2119),
            ('Q', 0x211A),
            ('R', 0x211D),
            ('Z', 0x2124),
        ],
        _ => &[],
    };
    if let Some(&(_, z)) = exc.iter().find(|&&(k, _)| k == c) {
        return char::from_u32(z).unwrap();
    }
    let (up, lo) = match style {
        "bold" => (0x1D400, 0x1D41A),
        "italic" => (0x1D434, 0x1D44E),
        "bold-italic" => (0x1D468, 0x1D482),
        "script" => (0x1D49C, 0x1D4B6),
        "bold-script" => (0x1D4D0, 0x1D4EA),
        "fraktur" => (0x1D504, 0x1D51E),
        "doublestruck" => (0x1D538, 0x1D552),
        "bold-fraktur" => (0x1D56C, 0x1D586),
        "sans" => (0x1D5A0, 0x1D5BA),
        "sans-bold" => (0x1D5D4, 0x1D5EE),
        "sans-italic" => (0x1D608, 0x1D622),
        "sans-bold-italic" => (0x1D63C, 0x1D656),
        "monospace" => (0x1D670, 0x1D68A),
        _ => ('A' as u32, 'a' as u32),
    };
    let oc = c as u32;
    if c.is_ascii_uppercase() {
        return char::from_u32(up + oc - 'A' as u32).unwrap();
    }
    if c.is_ascii_lowercase() {
        return char::from_u32(lo + oc - 'a' as u32).unwrap();
    }
    c
}

fn accented(ch: char, mark: char) -> String {
    let ch = match ch {
        '\u{131}' => 'i',
        '\u{237}' => 'j',
        c => c,
    };
    match NFC_ACCENT.binary_search_by(|&(m, c, _)| (m, c).cmp(&(mark, ch))) {
        Ok(i) => NFC_ACCENT[i].2.to_string(),
        Err(_) => {
            let mut s = String::new();
            s.push(ch);
            s.push(mark);
            s
        }
    }
}

/// `'{dt:%B} {dt.day}, {dt.year}'.format(dt=datetime.datetime.now())`
fn latex_today() -> String {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let (y, m, d) = local_date();
    format!("{} {}, {}", MONTHS[(m - 1) as usize], d, y)
}

#[cfg(unix)]
fn local_date() -> (i64, u32, u32) {
    #[repr(C)]
    struct Tm {
        tm_sec: i32,
        tm_min: i32,
        tm_hour: i32,
        tm_mday: i32,
        tm_mon: i32,
        tm_year: i32,
        tm_wday: i32,
        tm_yday: i32,
        tm_isdst: i32,
        tm_gmtoff: std::ffi::c_long,
        tm_zone: *const std::ffi::c_char,
    }
    extern "C" {
        fn time(t: *mut i64) -> i64;
        fn localtime_r(t: *const i64, tm: *mut Tm) -> *mut Tm;
    }
    // SAFETY: plain libc calls with valid pointers to local storage.
    unsafe {
        let mut t: i64 = 0;
        time(&mut t);
        let mut tm: Tm = std::mem::zeroed();
        if localtime_r(&t, &mut tm).is_null() {
            return utc_date();
        }
        (
            tm.tm_year as i64 + 1900,
            tm.tm_mon as u32 + 1,
            tm.tm_mday as u32,
        )
    }
}

#[cfg(not(unix))]
fn local_date() -> (i64, u32, u32) {
    utc_date()
}

#[allow(dead_code)]
fn utc_date() -> (i64, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    // civil_from_days (H. Hinnant)
    let z = secs.div_euclid(86400) + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

impl<'a> L2T<'a> {
    fn at(&self, d: usize) -> Result<(), Error> {
        self.depth.at(d)
    }

    /// `nodelist_to_text(nodelist)`; `d`: depth of this frame.
    fn nodelist_to_text(&mut self, nodes: &[Option<&Node>], d: usize) -> Result<String, Error> {
        self.at(d)?;
        let mut s = String::new();
        let mut prev: Option<&Node> = None;
        for &node in nodes {
            // self._is_bare_macro_node(prev_node) and node.isNodeType(LatexCharsNode)
            self.at(d + 2)?;
            if self.is_bare_macro(prev)? {
                let Some(n) = node else {
                    return Err(err(
                        "AttributeError",
                        "'NoneType' object has no attribute 'isNodeType'",
                    ));
                };
                if let Node::Chars(_) = n {
                    if self.in_eq {
                        // 'between-macro-and-chars' is False in equations
                        if let Some(Node::Macro { post, .. }) = prev {
                            s.push_str(post);
                        }
                    }
                }
            }
            s.push_str(&self.node_to_text(node, d + 1)?);
            prev = node;
        }
        Ok(s)
    }

    fn is_bare_macro(&self, node: Option<&Node>) -> Result<bool, Error> {
        let Some(Node::Macro { args, expr, .. }) = node else {
            return Ok(false);
        };
        let (opt, nargs) = legacy_of(args, *expr);
        if opt.is_some() {
            return Ok(false);
        }
        match nargs {
            None => Err(err("TypeError", "object of type 'NoneType' has no len()")),
            Some(v) => Ok(v.is_empty()),
        }
    }

    /// `node_to_text(node)`; `d`: depth of this frame.
    fn node_to_text(&mut self, node: Option<&Node>, d: usize) -> Result<String, Error> {
        self.at(d)?;
        let Some(node) = node else {
            return Ok(String::new());
        };
        self.at(d + 1)?; // isNodeType
        match node {
            Node::Chars(c) => {
                self.at(d + 1)?;
                if !self.in_eq || !py_strip(c).is_empty() {
                    Ok(c.clone())
                } else {
                    Ok(String::new())
                }
            }
            Node::Comment(post) => {
                self.at(d + 1)?;
                Ok(post.clone())
            }
            Node::Group(nodes) => {
                // group_node_to_text -> _groupnodecontents_to_text -> nodelist_to_text
                self.at(d + 3)?;
                self.nodelist_to_text(&nodes.iter().map(Some).collect::<Vec<_>>(), d + 3)
            }
            Node::Macro { .. } => self.macro_node_to_text(node, d + 1),
            Node::Env { .. } => self.environment_node_to_text(node, d + 1),
            Node::Specials(sp) => {
                // specials_node_to_text -> get_specials_spec / get_specials_str_repl -> apply_simplify_repl
                self.at(d + 3)?;
                Ok(match l2t_specials(sp) {
                    Some(r) => r.to_string(),
                    None => sp.to_string(),
                })
            }
            Node::Math { .. } => self.math_node_to_text(node, d + 1),
        }
    }

    /// `_groupnodecontents_to_text(n)`; `d`: depth of this frame.
    fn group_contents(&mut self, node: Option<&Node>, d: usize) -> Result<String, Error> {
        self.at(d)?;
        match node {
            None => Ok(String::new()),
            Some(Node::Group(nodes)) => {
                self.nodelist_to_text(&nodes.iter().map(Some).collect::<Vec<_>>(), d + 1)
            }
            Some(n) => self.node_to_text(Some(n), d + 1),
        }
    }

    fn argnlist(node: &Node) -> &[Option<Node>] {
        match node {
            Node::Macro { args: Some(a), .. } | Node::Env { args: Some(a), .. } => &a.list,
            _ => &[],
        }
    }

    /// `node_arg_to_text(node, k)`; `d`: depth of this frame.
    fn node_arg_to_text(&mut self, node: &Node, k: usize, d: usize) -> Result<String, Error> {
        self.at(d)?;
        let list = Self::argnlist(node);
        if list.is_empty() {
            return Ok(String::new());
        }
        let Some(a) = list.get(k) else {
            return Err(err("IndexError", "list index out of range"));
        };
        self.group_contents(a.as_ref(), d + 1)
    }

    fn macro_node_to_text(&mut self, node: &Node, d: usize) -> Result<String, Error> {
        self.at(d + 1)?; // get_macro_spec
        let Node::Macro { name, .. } = node else {
            unreachable!()
        };
        let (repl, discard) = l2t_macro(name).unwrap_or((&Repl::None, true));
        // get_macro_str_repl at d+1
        if !matches!(repl, Repl::None) {
            return self.apply_simplify_repl(node, repl, d + 2);
        }
        if discard {
            return Ok(String::new());
        }
        let mut out = String::new();
        for a in Self::argnlist(node) {
            // listcomp (d+2) -> _groupnodecontents_to_text (d+3)
            out.push_str(&self.group_contents(a.as_ref(), d + 3)?);
        }
        Ok(out)
    }

    fn environment_node_to_text(&mut self, node: &Node, d: usize) -> Result<String, Error> {
        self.at(d + 1)?; // get_environment_spec
        let Node::Env { name, nodes, .. } = node else {
            unreachable!()
        };
        let (repl, discard) = l2t_env(name).unwrap_or((&Repl::None, false));
        if !matches!(repl, Repl::None) {
            return self.apply_simplify_repl(node, repl, d + 1);
        }
        if discard {
            return Ok(String::new());
        }
        self.nodelist_to_text(&nodes.iter().map(Some).collect::<Vec<_>>(), d + 1)
    }

    fn math_node_to_text(&mut self, node: &Node, d: usize) -> Result<String, Error> {
        // _PushEquationContext(self): type call (counted) -> __init__ (entered
        // from C: counted twice) -> _parse_strict_latex_spaces_dict / super().__init__
        self.at(d + 4)?;
        let (nodes, block) = match node {
            Node::Math { display, nodes } => (nodes, *display),
            Node::Env { nodes, .. } => (nodes, true),
            _ => unreachable!(),
        };
        let saved = self.in_eq;
        self.in_eq = true;
        let r = self.nodelist_to_text(&nodes.iter().map(Some).collect::<Vec<_>>(), d + 1);
        self.in_eq = saved;
        let content = r?;
        let content = py_strip(&content);
        if block {
            self.at(d + 1)?;
            Ok(indented_block(content, "    "))
        } else {
            Ok(content.to_string())
        }
    }

    /// `apply_simplify_repl(node, simplify_repl, what)`; `d`: depth of this frame.
    fn apply_simplify_repl(&mut self, node: &Node, repl: &Repl, d: usize) -> Result<String, Error> {
        self.at(d)?;
        match repl {
            Repl::None => unreachable!(),
            // a plain string in Python (evaluated at import time)
            Repl::Call(Call::Today) => Ok(latex_today()),
            Repl::Call(c) => {
                // inspect.getfullargspec(simplify_repl) (calibrated leaf depth)
                self.at(d + 8)?;
                self.call(node, c, d + 1)
            }
            Repl::Str(r) => {
                let r: &str = r;
                if !(r.contains('%') && nchars(r) != 1) {
                    return Ok(r.to_string());
                }
                self.at(d + 2)?; // re.search -> re._compile
                let list = Self::argnlist(node);
                let hps = has_percent_s(r);
                let res = if let Node::Env { nodes, .. } = node {
                    let refs: Vec<Option<&Node>> = nodes.iter().map(Some).collect();
                    if hps {
                        let body = self.nodelist_to_text(&refs, d + 1)?;
                        py_percent(r, FmtArgs::Tuple(&[body]))
                    } else {
                        let mut x = Vec::new();
                        for (j, a) in list.iter().enumerate() {
                            // dict(genexpr (d+1) <- genexpr (d+2) -> _gnct (d+3))
                            x.push(((j + 1).to_string(), self.group_contents(a.as_ref(), d + 3)?));
                        }
                        let body = self.nodelist_to_text(&refs, d + 1)?;
                        x.push(("body".to_string(), body));
                        py_percent(r, FmtArgs::Dict(&x))
                    }
                } else if hps {
                    let mut x = Vec::new();
                    for a in list {
                        // listcomp (d+1) -> _gnct (d+2)
                        x.push(self.group_contents(a.as_ref(), d + 2)?);
                    }
                    py_percent(r, FmtArgs::Tuple(&x))
                } else {
                    let mut x = Vec::new();
                    for (j, a) in list.iter().enumerate() {
                        x.push(((j + 1).to_string(), self.group_contents(a.as_ref(), d + 3)?));
                    }
                    py_percent(r, FmtArgs::Dict(&x))
                };
                match res {
                    Ok(s) => Ok(s),
                    Err(FmtErr::TypeOrValue) => {
                        // logger.warning(...): emitted, nothing raised
                        Ok(r.to_string())
                    }
                    Err(FmtErr::Key(k)) => Err(err("KeyError", format!("'{}'", k))),
                }
            }
        }
    }

    /// The hand-ported callables. `d`: depth of the callable's frame.
    fn call(&mut self, node: &Node, c: &Call, d: usize) -> Result<String, Error> {
        self.at(d)?;
        let none_attr = || {
            err(
                "AttributeError",
                "'NoneType' object has no attribute 'argnlist'",
            )
        };
        let (args, expr) = match node {
            Node::Macro { args, expr, .. } => (args, *expr),
            Node::Env { args, .. } => (args, false),
            _ => unreachable!(),
        };
        let out: String = match c {
            Call::Title | Call::Author | Call::Date => {
                // setattr(l2tobj, '_doc_X', l2tobj.nodelist_to_text(n.nodeargd.argnlist[0:1]))
                let a = args.as_ref().ok_or_else(none_attr)?;
                let sl: Vec<Option<&Node>> = a.list.iter().take(1).map(|x| x.as_ref()).collect();
                let t = self.nodelist_to_text(&sl, d + 1)?;
                match c {
                    Call::Title => self.title = Some(t),
                    Call::Author => self.author = Some(t),
                    _ => self.date = Some(t),
                }
                String::new()
            }
            Call::Maketitle => {
                let title = self
                    .title
                    .clone()
                    .unwrap_or_else(|| "[NO \\title GIVEN]".into());
                let author = self
                    .author
                    .clone()
                    .unwrap_or_else(|| "[NO \\author GIVEN]".into());
                let date = match &self.date {
                    Some(x) => x.clone(),
                    None => latex_today(),
                };
                let w = nchars(&title)
                    .max(4 + nchars(&author))
                    .max(4 + nchars(&date));
                format!(
                    "{}\n    {}\n    {}\n{}\n\n",
                    title,
                    author,
                    date,
                    "=".repeat(w)
                )
            }
            Call::Item => {
                let (opt, _) = legacy_of(args, expr);
                let t = match opt {
                    Some(o) => self.nodelist_to_text(&[Some(o)], d + 1)?,
                    None => "* ".to_string(),
                };
                format!("\n  {}", t)
            }
            Call::Href => {
                let a = args.as_ref().ok_or_else(none_attr)?;
                if a.list.len() < 2 {
                    return Err(err("IndexError", "list index out of range"));
                }
                let t1 = self.nodelist_to_text(&[a.list[1].as_ref()], d + 1)?;
                let t0 = self.nodelist_to_text(&[a.list[0].as_ref()], d + 1)?;
                format!("{} <{}>", t1, t0)
            }
            Call::Part => format!(
                "\n\nPART: {}\n",
                py_upper(&self.node_arg_to_text(node, 2, d + 1)?)
            ),
            Call::Chapter => {
                format!(
                    "\n\nCHAPTER: {}\n",
                    py_upper(&self.node_arg_to_text(node, 2, d + 1)?)
                )
            }
            Call::Section => {
                format!(
                    "\n\n\u{a7} {}\n",
                    py_upper(&self.node_arg_to_text(node, 2, d + 1)?)
                )
            }
            Call::Subsection => {
                format!(
                    "\n\n \u{a7}.\u{a7} {}\n",
                    self.node_arg_to_text(node, 2, d + 1)?
                )
            }
            Call::Subsubsection => format!(
                "\n\n  \u{a7}.\u{a7}.\u{a7} {}\n",
                self.node_arg_to_text(node, 2, d + 1)?
            ),
            Call::Paragraph => format!("\n\n  {}\n", self.node_arg_to_text(node, 2, d + 1)?),
            Call::Subparagraph => format!("\n\n    {}\n", self.node_arg_to_text(node, 2, d + 1)?),
            Call::Texorpdfstring => {
                let (_, nargs) = legacy_of(args, expr);
                let Some(v) = nargs else {
                    return Err(err("TypeError", "'NoneType' object is not subscriptable"));
                };
                let sl: Vec<Option<&Node>> = v.iter().skip(1).take(1).copied().collect();
                self.nodelist_to_text(&sl, d + 1)?
            }
            Call::Percent => "%".to_string(),
            Call::Today => latex_today(),
            Call::Input => {
                // fmt_input_macro -> _input_node_simplify_repl
                self.at(d + 1)?;
                let (_, nargs) = legacy_of(args, expr);
                let Some(v) = nargs else {
                    return Err(err("TypeError", "object of type 'NoneType' has no len()"));
                };
                let Some(&a0) = v.first() else {
                    return Err(err("IndexError", "list index out of range"));
                };
                let _ = self.nodelist_to_text(&[a0], d + 2)?;
                // read_input_file(): tex_input_directory is None -> ''
                self.at(d + 2)?;
                String::new()
            }
            Call::Uebung => {
                let (_, nargs) = legacy_of(args, expr);
                let Some(v) = nargs else {
                    return Err(err("TypeError", "'NoneType' object is not subscriptable"));
                };
                let Some(&a0) = v.first() else {
                    return Err(err("IndexError", "list index out of range"));
                };
                let mut s = format!("\n{}\n", self.nodelist_to_text(&[a0], d + 1)?);
                let Some(&optarg) = v.get(1) else {
                    return Err(err("IndexError", "list index out of range"));
                };
                if let Some(o) = optarg {
                    s.push_str(&format!(
                        "[{}]\n",
                        self.nodelist_to_text(&[Some(o)], d + 1)?
                    ));
                }
                s
            }
            Call::MathStyle(style) => {
                let t = self.node_arg_to_text(node, 0, d + 1)?;
                // fmt_math_text_style -> genexpr -> _fmt_math_style_char
                if !t.is_empty() {
                    self.at(d + 3)?;
                } else {
                    self.at(d + 2)?;
                }
                t.chars().map(|ch| math_style_char(ch, style)).collect()
            }
            Call::Placeholder(pht) => {
                // _do_fmt_placeholder_node -> _fmt_indented_block
                self.at(d + 2)?;
                let joined: Vec<String> = pht.chars().map(|c| c.to_string()).collect();
                let txt = format!("< {} >", joined.join(" "));
                indented_block(&txt, "    ")
            }
            Call::Accent(mark) => {
                // lambda -> make_accented_char (d+1)
                self.at(d + 1)?;
                let (_, nargs) = legacy_of(args, expr);
                let c = match nargs {
                    Some(v) if !v.is_empty() => {
                        let t = self.nodelist_to_text(&[v[0]], d + 2)?;
                        py_strip(&t).to_string()
                    }
                    _ => " ".to_string(),
                };
                if !c.is_empty() {
                    self.at(d + 3)?; // listcomp -> getaccented
                }
                c.chars().map(|ch| accented(ch, *mark)).collect()
            }
            Call::Matrix => {
                let Node::Env { nodes, .. } = node else {
                    unreachable!()
                };
                self.fmt_matrix(nodes, d)?
            }
            Call::Equation => {
                // fmt_equation_environment -> math_node_to_text
                self.math_node_to_text(node, d + 1)?
            }
        };
        Ok(out)
    }

    /// `fmt_matrix_environment_node`; `d`: depth of its frame.
    fn fmt_matrix(&mut self, nodes: &[Node], d: usize) -> Result<String, Error> {
        self.at(d + 1)?; // class StateType body / methods
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut this_col: Vec<String> = Vec::new();
        let mut buf: Vec<Option<&Node>> = Vec::new();
        // new_column called from the loop (d+1) or from new_row (d+2)
        fn new_column(
            l: &mut L2T,
            buf: &mut Vec<Option<&Node>>,
            col: &mut Vec<String>,
            d: usize,
        ) -> Result<(), Error> {
            if !buf.is_empty() {
                let t = l.nodelist_to_text(buf, d + 1)?;
                col.push(py_strip(&t).to_string());
            }
            buf.clear();
            Ok(())
        }
        for n in nodes {
            match n {
                Node::Specials("&") => {
                    new_column(self, &mut buf, &mut this_col, d + 1)?;
                    continue;
                }
                Node::Macro { name, .. } if name == "\\" => {
                    new_column(self, &mut buf, &mut this_col, d + 2)?;
                    rows.push(std::mem::take(&mut this_col));
                    continue;
                }
                _ => buf.push(Some(n)),
            }
        }
        new_column(self, &mut buf, &mut this_col, d + 2)?;
        rows.push(std::mem::take(&mut this_col));
        let width = rows.iter().flatten().map(|x| nchars(x)).max();
        let Some(width) = width else {
            return Err(err("ValueError", "max() arg is an empty sequence"));
        };
        self.at(d + 3)?; // join(genexpr(join(genexpr)))
        let row_strs: Vec<String> = rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| {
                        let n = nchars(x);
                        let mut s = " ".repeat(width.saturating_sub(n));
                        s.push_str(x);
                        s
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        Ok(format!("[ {} ]", row_strs.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(latex_to_text("\\alpha x \\beta  y").unwrap(), "αx βy");
        assert_eq!(latex_to_text("$\\alpha x$").unwrap(), "α x");
        assert_eq!(latex_to_text("{a  ").unwrap(), "a  ");
        assert_eq!(latex_to_text("a}b").unwrap(), "ab");
        assert_eq!(latex_to_text("\\begin x").unwrap(), "\\begin x");
        assert_eq!(latex_to_text("\\frac{a}").unwrap(), "%s/%sa");
        assert_eq!(latex_to_text("a \\").unwrap(), "a");
        assert_eq!(latex_to_text("\\frac 12").unwrap(), "1/2");
        assert_eq!(latex_to_text("\\[a\\]b").unwrap(), "\n    a\nb");
        assert_eq!(latex_to_text("% c\n  x").unwrap(), "\n  x");
        assert_eq!(latex_to_text("\\\\[2mm] a").unwrap(), "\n a");
        assert_eq!(latex_to_text("\\ss x").unwrap(), "ßx");
        for e in [
            "\\verb",
            "\\href{a}{b}",
            "\\sqrt",
            "\\frac\\sqrt{2}",
            "\\begin{array}\\end{array}",
            "\\exercise",
            "\\texorpdfstring\\title",
            "\\title",
        ] {
            assert!(latex_to_text(e).is_err(), "{}", e);
        }
    }
}

#[cfg(test)]
mod depth_tests {
    use super::*;

    /// First failing nesting level measured with Python 3.11 when
    /// latex_to_text is called from a function called from a function
    /// called at module level (latex_to_text frame at depth 4).
    #[test]
    fn recursion_thresholds() {
        type Family = (&'static str, fn(usize) -> String, usize);
        let fams: [Family; 4] = [
            (
                "brace",
                |n| format!("{}x{}", "{".repeat(n), "}".repeat(n)),
                249,
            ),
            (
                "frac",
                |n| format!("{}x{}", "\\frac{".repeat(n), "}{y}".repeat(n)),
                142,
            ),
            (
                "env",
                |n| format!("{}x{}", "\\begin{a}".repeat(n), "\\end{a}".repeat(n)),
                330,
            ),
            ("math", |n| "\\(".repeat(n), 331),
        ];
        for (name, f, thr) in fams {
            let ok = latex_to_text_at_depth(&f(thr - 1), 4);
            let bad = latex_to_text_at_depth(&f(thr), 4);
            assert!(ok.is_ok(), "{} at {}: {:?}", name, thr - 1, ok);
            assert_eq!(
                bad.as_ref().map_err(|e| e.kind),
                Err("RecursionError"),
                "{} at {}",
                name,
                thr
            );
        }
    }
}
