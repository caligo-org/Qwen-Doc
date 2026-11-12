#!/usr/bin/env python3
"""Build tests/fixtures/regression.jsonl.

Usage: python tools/make_fixture.py OUT.jsonl CORPUS.jsonl [CORPUS.jsonl ...]

CORPUS files are differential-test dumps with one JSON object per line,
{"i": input, "o": output} or {"i": input, "e": "ExceptionClass"}, plus "k"
(kind). The expected values are recomputed here with the running pylatexenc
(must be 2.10 on Python 3.11), so the fixture never depends on stale dumps.

Selection (deterministic): all hand-written adversarial cases, a sample of
every exception class, and a sample of every input kind, limited to inputs of
at most 400 characters.
"""
import json
import random
import sys
import logging

import pylatexenc
from pylatexenc.latex2text import LatexNodes2Text

assert pylatexenc.__version__ == '2.10'
assert sys.version_info[:2] == (3, 11)
logging.disable(logging.CRITICAL)

HAND = [
    r'\begin{matrix}\end{matrix}', r'\begin{pmatrix}\end{pmatrix}', r'\begin{array}{cc}\end{array}',
    r'\begin{array}{cc} a & b \\ c & d \end{array}', r'\begin{pmatrix} 1 & 22 \\ 333 & 4 \end{pmatrix}',
    '{{{', '}}}', '{a}}', r'\frac{', r'\frac{a}{b', r'\frac{a}', r'\sqrt[3]{x}', r'\sqrt[3]', r'\sqrt',
    r'\unknownmacro{x}', '% comment', 'a % c\nb', 'a %c\n\n b', '100%', 'a~b', '``q\'\'', '--- -- -',
    r"\'e \`a \^o \"u \c c \~n \H o \k a \=a \b b \.z \d d \r a \u g \v s", r'\vec x \hat{y} \bar{\i}',
    r'a \\ b', r'a\\[1em]b', r'a \\ [b]', r'\\', '\\', 'a \\', 'a \\ ', r'\begin{align} a &= b \\ c &= d \end{align}',
    r'\begin{itemize}\item a \item[b] c\end{itemize}', r'\section{Hello ß}', r'\subsection{a}',
    r'\title{T}\author{A}\date{D}\maketitle', r'\title', r'\href{u}{t}', r'\url{http://x}',
    r'\footnote{f}', r'\footnote', r'\mathbf{AbZ}', r'\mathcal{BEFxyz}', r'\mathbb{RNZ1}',
    r'\verb|x y|', r'\verb', r'\verb  ', r'\verb|abc', r'\begin{verbatim} \x{ \end{verbatim}',
    r'$a$', r'$$a$$', r'\(a\)', r'\[a\]b', r'$a', r'a\)', r'\text{$x$}', r'\begin{a}\begin{b}\end{a}\end{b}',
    r'\begin x', r'\begin{}', r'\input{f}', r'\input', r'\texorpdfstring{a}{b}', r'\texorpdfstring\title',
    r'\exercise{a}[b]', r'\exercise', r'\frac\sqrt{2}', r'\frac 12', r'\alpha x \beta  y', '$\\alpha x$',
    '{a  ', 'a}b', r'\ss x', r'\中文 x', r'\é x', '\\alpha\n\nb', '%c\r\n  \n', '\x0c\x1c\u2028\u3000x',
    r'\, \; \: \! \ \quad \qquad', r'\ket{\psi}', r'\braket{a}{b}', r'\not=', r'\CYRA \textasciicircum',
    '{' * 60 + 'x' + '}' * 60, r'\frac{' * 30 + 'x' + '}{y}' * 30,
]


def run(s):
    try:
        return {'i': s, 'o': LatexNodes2Text().latex_to_text(s)}
    except Exception as e:  # noqa
        return {'i': s, 'e': type(e).__name__}


def main():
    out = sys.argv[1]
    rng = random.Random(20261111)
    by_kind, by_exc = {}, {}
    for path in sys.argv[2:]:
        for line in open(path, encoding='utf-8'):
            d = json.loads(line)
            if len(d['i']) > 400:
                continue
            if 'e' in d:
                by_exc.setdefault(d['e'], []).append(d['i'])
            else:
                by_kind.setdefault(d.get('k', '?'), []).append(d['i'])
    chosen = list(HAND)
    for e, xs in sorted(by_exc.items()):
        chosen += rng.sample(xs, min(15, len(xs)))
    for k, xs in sorted(by_kind.items()):
        chosen += rng.sample(xs, min(40, len(xs)))
    seen = set()
    with open(out, 'w', encoding='utf-8') as f:
        for s in chosen:
            if s in seen:
                continue
            seen.add(s)
            f.write(json.dumps(run(s), ensure_ascii=False) + '\n')
    print(len(seen), 'cases', file=sys.stderr)


if __name__ == '__main__':
    main()
