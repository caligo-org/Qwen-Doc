"""Notation around a display formula that does not change what it says.

Applied to the ground truth and the prediction alike before CDM renders them.
"""
import re

_DISPLAY_DELIMITERS = re.compile(r'^\s*\\\[(.*)\\\]\s*$', re.DOTALL)


def strip_display_delimiters(line):
    """Return the body of one formula line written as `\\[ ... \\]`.

    Lines joined into one `array` must not keep their own display delimiters:
    `\\[` inside the `displaymath` environment the renderer wraps around the
    formula is a LaTeX error.
    """
    match = _DISPLAY_DELIMITERS.match(line)
    return match.group(1).strip() if match else line


_TAG = re.compile(r'\\tag\*?\s*\{[^{}]*\}')
_SPACING = r'(?:\\q?quad|\\hfill|\\[,;:]|~|\s)*'
# (1), (2.4), (2-2), (A.1): digits, optionally after one capital letter and a dot
_NUMBER = r'\(\s*(?:[A-Z]\.)?[0-9]+(?:[.\-][0-9]+)*\s*\)'
_ELLIPSIS = r'(?:\.{3}|…|\\[lc]?dots(?![A-Za-z]))'
_LEADER = r'(?:\.{4,}|(?:' + _ELLIPSIS + r'\s*){2,})'
_LONG_LEADER = r'(?:\.{6,}|(?:' + _ELLIPSIS + r'\s*){3,})'
_TRAILING_NUMBER = re.compile(
    _SPACING + r'(?:' + _LONG_LEADER + r'\s*(?:' + _NUMBER + r')?'
    + r'|' + _LEADER + r'\s*' + _NUMBER
    + r'|(?:\\q?quad|\\hfill)\s*' + _NUMBER + r')\s*$'
)


def strip_equation_number(latex):
    """Remove the equation number of a display formula.

    An equation number labels the formula; it is not part of it. The ground
    truth writes it as `\\tag{n}`, as a leader (a run of dots or ellipses) with
    or without `(n)`, or not at all, and the matcher's own normalization
    (`normalized_formula`) already ignores `\\tag`. The number itself is
    ignored too: `\\tag{3}` and `\\tag{7}` score the same. Removed here:

    - every `\\tag{..}` / `\\tag*{..}`;
    - a trailing `(n)` after a leader of two or more ellipses (`...`, `…`,
      `\\cdots`, `\\ldots`, `\\dots`) or four or more dots;
    - a trailing leader of three or more ellipses or six or more dots, with
      or without `(n)`;
    - a trailing `(n)` set off by `\\quad`, `\\qquad` or `\\hfill`.

    `(n)` is a number such as `(1)`, `(2.4)`, `(2-2)` or `(A.1)`. A shorter
    trailing run of dots or ellipses without `(n)` (`x_1, x_2, \\cdots`,
    `0.3333....`) and a parenthesized term that is not a number
    (`\\quad (1-x)`, `\\quad (N > 15)`) are content and are kept.
    """
    latex = _TAG.sub('', latex)
    return _TRAILING_NUMBER.sub('', latex).strip()
