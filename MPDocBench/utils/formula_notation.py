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

