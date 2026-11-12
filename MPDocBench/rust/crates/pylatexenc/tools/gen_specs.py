#!/usr/bin/env python3
"""Generate ``src/generated.rs`` from the installed pylatexenc (must be 2.10)
and the running Python's ``unicodedata`` / ``str`` methods (Python 3.11,
Unicode 14).

Usage:  python tools/gen_specs.py > src/generated.rs

What is generated:
  * latexwalker default context (macros / environments / specials argument
    specs), flattened with LatexContextDb lookup semantics (first category
    wins; inside one category the last definition wins);
  * latex2text default context (macro / environment / specials text specs),
    flattened the same way. Specs whose ``simplify_repl`` is a Python callable
    are emitted as a ``Call::...`` variant; the behaviour of each variant is
    hand-ported in ``src/lib.rs``. An unknown callable aborts generation;
  * Unicode tables reproducing the Python semantics used by pylatexenc:
    ``str.isspace`` (also used for ``\\s`` and ``str.strip``), ``str.isalpha``,
    regex ``\\w`` (``str.isalnum`` or ``_``), ``str.upper`` and the NFC
    composition ``unicodedata.normalize('NFC', ch + combining)`` for every
    combining mark used by the accent macros.
"""

import sys
import unicodedata

import pylatexenc
from pylatexenc import latexwalker, latex2text, macrospec
from pylatexenc.macrospec import VerbatimArgsParser, MacroStandardArgsParser

assert pylatexenc.__version__ == '2.10', pylatexenc.__version__
assert sys.version_info[:2] == (3, 11), sys.version_info
assert unicodedata.unidata_version == '14.0.0', unicodedata.unidata_version


def rs(s):
    """Rust string literal."""
    out = ['"']
    for ch in s:
        o = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == '\\':
            out.append('\\\\')
        elif 0x20 <= o < 0x7f:
            out.append(ch)
        else:
            out.append('\\u{%x}' % o)
    out.append('"')
    return ''.join(out)


def rc(ch):
    o = ord(ch)
    if ch == "'":
        return "'\\''"
    if ch == '\\':
        return "'\\\\'"
    if 0x20 <= o < 0x7f:
        return "'%s'" % ch
    return "'\\u{%x}'" % o


def all_names(db, kind):
    names = set()
    for c in db.category_list:
        names |= set(db.d[c][kind])
    return sorted(names)


def walker_args(parser):
    if type(parser) is VerbatimArgsParser:
        if parser.verbatim_arg_type == 'verb-macro':
            return 'WArgs::VerbMacro'
        assert parser.verbatim_arg_type == 'verbatim-environment'
        return 'WArgs::VerbEnv'
    assert type(parser) is MacroStandardArgsParser, parser
    assert not parser._like_pylatexenc1x_ignore_leading_star
    amm = parser.args_math_mode
    if amm is None:
        amm_s = 'None'
    else:
        assert len(amm) == len(parser.argspec)
        amm_s = 'Some(&[%s])' % ', '.join(
            'None' if x is None else ('Some(true)' if x else 'Some(false)') for x in amm)
    return 'WArgs::Std { argspec: %s, no_space: %s, math: %s }' % (
        rs(parser.argspec), 'true' if parser.optional_arg_no_space else 'false', amm_s)


def classify_callable(name, fn, what):
    q = fn.__qualname__
    mod = fn.__module__
    if what == 'env':
        if fn is latex2text.fmt_matrix_environment_node:
            return 'Call::Matrix'
        if fn is latex2text.fmt_equation_environment:
            return 'Call::Equation'
        raise SystemExit('unknown env callable %r %r' % (name, fn))
    if fn is latex2text.fmt_input_macro:
        return 'Call::Input'
    if q == '_format_uebung':
        return 'Call::Uebung'
    if q == '_mathxx_formatter.<locals>.formatter':
        (style,) = fn.__defaults__
        return 'Call::MathStyle(%s)' % rs(style)
    if q == 'placeholder_node_formatter.<locals>.<lambda>':
        (pht,) = fn.__defaults__
        assert fn.__closure__ is not None
        # closure holds block=True
        cells = dict(zip(fn.__code__.co_freevars, [c.cell_contents for c in fn.__closure__]))
        assert cells == {'block': True}, cells
        return 'Call::Placeholder(%s)' % rs(pht)
    if q == '<lambda>' and fn.__code__.co_varnames[:3] == ('x', 'l2tobj', 'c'):
        (c,) = fn.__defaults__
        assert len(c) == 1
        return 'Call::Accent(%s)' % rc(c)
    lambdas = {
        'title': 'Call::Title', 'author': 'Call::Author', 'date': 'Call::Date',
        'maketitle': 'Call::Maketitle', 'item': 'Call::Item', 'href': 'Call::Href',
        'part': 'Call::Part', 'chapter': 'Call::Chapter', 'section': 'Call::Section',
        'subsection': 'Call::Subsection', 'subsubsection': 'Call::Subsubsection',
        'paragraph': 'Call::Paragraph', 'subparagraph': 'Call::Subparagraph',
        'texorpdfstring': 'Call::Texorpdfstring', '%': 'Call::Percent',
    }
    if q == '<lambda>' and name in lambdas:
        return lambdas[name]
    raise SystemExit('unknown macro callable %r %r' % (name, fn))


def repl_of(name, r, what):
    if what == 'macro' and name == 'today':
        # evaluated once at import time from datetime.now(); ported as a call
        from pylatexenc.latex2text._defaultspecs import _latex_today
        assert r == _latex_today(), r
        return 'Repl::Call(Call::Today)'
    if callable(r):
        return 'Repl::Call(%s)' % classify_callable(name, r, what)
    if r:  # only truthy strings matter (`if mac.simplify_repl:`)
        assert isinstance(r, str)
        if '%' in r and len(r) != 1:
            # the hand-ported formatter only knows %s, %(key)s and %%
            import re
            rest = re.sub(r'%%|%s|%\(\d+\)s', '', r)
            assert '%' not in rest, r
        return 'Repl::Str(%s)' % rs(r)
    return 'Repl::None'


def ranges(pred):
    out = []
    start = None
    for c in range(0x110000):
        ok = pred(c)
        if ok and start is None:
            start = c
        elif not ok and start is not None:
            out.append((start, c - 1))
            start = None
    if start is not None:
        out.append((start, 0x10ffff))
    return out


def main():
    w = []
    p = w.append
    p('// @generated by tools/gen_specs.py from pylatexenc %s, Python %s, Unicode %s.'
      % (pylatexenc.__version__, sys.version.split()[0], unicodedata.unidata_version))
    p('// Do not edit by hand.')
    p('#![allow(clippy::all)]')
    p('use crate::{WArgs, Repl, Call};')
    p('')

    # ---------------- latexwalker ----------------
    wdb = latexwalker.get_default_latex_context_db()
    p('/// latexwalker macros: (name, args parser), sorted by name.')
    p('pub static WALKER_MACROS: &[(&str, WArgs)] = &[')
    for n in sorted(all_names(wdb, 'macros'), key=lambda x: x.encode('utf-8')):
        spec = wdb.get_macro_spec(n)
        p('    (%s, %s),' % (rs(n), walker_args(spec.args_parser)))
    p('];')
    p('/// latexwalker environments: (name, args parser, is_math_mode), sorted by name.')
    p('pub static WALKER_ENVS: &[(&str, WArgs, bool)] = &[')
    for n in sorted(all_names(wdb, 'environments'), key=lambda x: x.encode('utf-8')):
        spec = wdb.get_environment_spec(n)
        p('    (%s, %s, %s),' % (rs(n), walker_args(spec.args_parser),
                                 'true' if spec.is_math_mode else 'false'))
    p('];')
    p('/// latexwalker specials (none of them parses arguments).')
    p('pub static WALKER_SPECIALS: &[&str] = &[')
    for n in all_names(wdb, 'specials'):
        assert wdb.get_specials_spec(n).args_parser is None
        p('    %s,' % rs(n))
    p('];')
    p('')

    # ---------------- latex2text ----------------
    ldb = latex2text.get_default_latex_context_db()
    p('/// latex2text macros: (name, simplify_repl, discard), sorted by name.')
    p('pub static L2T_MACROS: &[(&str, Repl, bool)] = &[')
    for n in sorted(all_names(ldb, 'macros'), key=lambda x: x.encode('utf-8')):
        spec = ldb.get_macro_spec(n)
        p('    (%s, %s, %s),' % (rs(n), repl_of(n, spec.simplify_repl, 'macro'),
                                 'true' if spec.discard else 'false'))
    p('];')
    p('/// latex2text environments: (name, simplify_repl, discard), sorted by name.')
    p('pub static L2T_ENVS: &[(&str, Repl, bool)] = &[')
    for n in sorted(all_names(ldb, 'environments'), key=lambda x: x.encode('utf-8')):
        spec = ldb.get_environment_spec(n)
        p('    (%s, %s, %s),' % (rs(n), repl_of(n, spec.simplify_repl, 'env'),
                                 'true' if spec.discard else 'false'))
    p('];')
    p('/// latex2text specials: (chars, simplify_repl).')
    p('pub static L2T_SPECIALS: &[(&str, &str)] = &[')
    for n in all_names(ldb, 'specials'):
        r = ldb.get_specials_spec(n).simplify_repl
        # all default specials have a non-empty string replacement without '%'
        assert isinstance(r, str) and r and '%' not in r, (n, r)
        p('    (%s, %s),' % (rs(n), rs(r)))
    p('];')
    p('')

    # ---------------- Unicode tables ----------------
    def cp_ok(c):
        return not (0xD800 <= c < 0xE000)

    sp = [c for c in range(0x110000) if cp_ok(c) and chr(c).isspace()]
    p('/// Code points for which Python str.isspace() is true.')
    p('pub static PY_SPACE: &[char] = &[%s];' % ', '.join(rc(chr(c)) for c in sp))

    al = ranges(lambda c: cp_ok(c) and chr(c).isalpha())
    p('/// Ranges for which Python str.isalpha() is true.')
    p('pub static PY_ALPHA: &[(u32, u32)] = &[')
    for a, b in al:
        p('    (0x%x, 0x%x),' % (a, b))
    p('];')

    wr = ranges(lambda c: cp_ok(c) and (chr(c).isalnum() or chr(c) == '_'))
    p('/// Ranges matched by Python re `\\w` (str patterns): isalnum() or "_".')
    p('pub static PY_WORD: &[(u32, u32)] = &[')
    for a, b in wr:
        p('    (0x%x, 0x%x),' % (a, b))
    p('];')

    up = [(c, chr(c).upper()) for c in range(0x110000)
          if cp_ok(c) and chr(c).upper() != chr(c)]
    p('/// Python str.upper() for every char it changes, sorted by char.')
    p('pub static PY_UPPER: &[(char, &str)] = &[')
    for c, u in up:
        p('    (%s, %s),' % (rc(chr(c)), rs(u)))
    p('];')

    marks = sorted(set(c.__defaults__[0]
                       for n in all_names(ldb, 'macros')
                       for c in [ldb.get_macro_spec(n).simplify_repl]
                       if callable(c) and c.__qualname__ == '<lambda>'
                       and c.__code__.co_varnames[:3] == ('x', 'l2tobj', 'c')))
    p('/// unicodedata.normalize("NFC", ch + mark) for every (mark, ch) where it')
    p('/// differs from ch + mark; sorted by (mark, ch).')
    p('pub static NFC_ACCENT: &[(char, char, &str)] = &[')
    for m in marks:
        for c in range(0x110000):
            if not cp_ok(c):
                continue
            s = chr(c) + m
            n = unicodedata.normalize('NFC', s)
            if n != s:
                p('    (%s, %s, %s),' % (rc(m), rc(chr(c)), rs(n)))
    p('];')

    sys.stdout.write('\n'.join(w) + '\n')


if __name__ == '__main__':
    main()
