"""Notation equivalence for table cells, applied to ground truth and prediction alike before TEDS.

A cell is compared by what its formulas render to, not by how they are written:

- A math span ($...$, $$...$$, \\(...\\), \\[...\\]) that holds LaTeX markup (a backslash, ^, _ or a
  brace) is replaced by its rendered characters, so the choice of delimiter and which characters
  sit inside the span do not matter: "250 $\\theta$", "$250 \\theta$" and "\\(250\\theta\\)" all
  become "250θ".
- Inside such a span, spaces are dropped (LaTeX ignores them), as are the spaces at its edges.
- Superscripts and subscripts are flattened to their content (^{a} = ^a = a), as clean_table
  already does for <sup> and <sub>.
- Text and font commands (\\text, \\mathrm, \\mathbf, ...) are reduced to their argument.
- Symbol commands are written as their Unicode character (\\pm = ±, \\theta = θ, ^{\\circ} = °).

Text outside math is left as it is, spaces included. A "$" without LaTeX markup (currency such as
"$194.1m" or "$12 to $15") is not treated as math. Commands without a character equivalent
(\\frac, \\sqrt, ...) are kept as written.
"""
import re

SYMBOLS = {
    'pm': '±', 'mp': '∓', 'times': '×', 'cdot': '·', 'div': '÷', 'leq': '≤', 'le': '≤', 'geq': '≥', 'ge': '≥',
    'neq': '≠', 'ne': '≠', 'approx': '≈', 'sim': '∼', 'infty': '∞', 'rightarrow': '→', 'to': '→', 'leftarrow': '←',
    'Rightarrow': '⇒', 'leftrightarrow': '↔', 'uparrow': '↑', 'downarrow': '↓', 'prime': '′', 'ldots': '…',
    'cdots': '⋯', 'dots': '…', 'circ': '°', 'degree': '°', 'partial': '∂', 'nabla': '∇', 'sum': '∑', 'prod': '∏',
    'int': '∫', 'in': '∈', 'notin': '∉', 'subset': '⊂', 'cup': '∪', 'cap': '∩', 'forall': '∀', 'exists': '∃',
    'propto': '∝', 'perp': '⊥', 'angle': '∠', 'triangle': '△', 'checkmark': '✓', 'star': '⋆', 'ast': '∗',
    'bullet': '•', '%': '%', '$': '$', '#': '#', '&': '&', '_': '_', '{': '{', '}': '}',
    'alpha': 'α', 'beta': 'β', 'gamma': 'γ', 'delta': 'δ', 'epsilon': 'ϵ', 'varepsilon': 'ε', 'zeta': 'ζ', 'eta': 'η',
    'theta': 'θ', 'vartheta': 'ϑ', 'iota': 'ι', 'kappa': 'κ', 'lambda': 'λ', 'mu': 'μ', 'nu': 'ν', 'xi': 'ξ',
    'pi': 'π', 'rho': 'ρ', 'sigma': 'σ', 'tau': 'τ', 'upsilon': 'υ', 'phi': 'ϕ', 'varphi': 'φ', 'chi': 'χ',
    'psi': 'ψ', 'omega': 'ω', 'Gamma': 'Γ', 'Delta': 'Δ', 'Theta': 'Θ', 'Lambda': 'Λ', 'Xi': 'Ξ', 'Pi': 'Π',
    'Sigma': 'Σ', 'Upsilon': 'Υ', 'Phi': 'Φ', 'Psi': 'Ψ', 'Omega': 'Ω',
}
SPACING = ('quad', 'qquad', ',', ';', ':', '!', ' ', 'left', 'right', 'displaystyle', 'textstyle')
FONTS = ('text', 'textrm', 'textbf', 'textit', 'textsf', 'texttt', 'mathrm', 'mathbf', 'mathit', 'mathsf',
         'mathtt', 'mathcal', 'mathbb', 'boldsymbol', 'bm', 'operatorname', 'rm', 'bf', 'it')
MATH_SPAN = re.compile(r'\s*(?:\$\$(.+?)\$\$|\$(.+?)\$|\\\((.+?)\\\)|\\\[(.+?)\\\])\s*', re.S)
MARKUP = re.compile(r'[\\^_{}]')
FONT = re.compile(r'\\(?:' + '|'.join(FONTS) + r')\s*\{([^{}]*)\}')
BARE_FONT = re.compile(r'\\(?:rm|bf|it)(?![A-Za-z])')
COMMAND = re.compile(r'\\([A-Za-z]+|[%$#&_{} ,;:!])')
SCRIPT = re.compile(r'[\^_]\s*(\{[^{}]*\}|\\[A-Za-z]+|.)')
CELL_TEXT = re.compile(r'>([^<>]+)<')


def render_math(body):
    """The characters a LaTeX math span renders to, without spaces."""
    previous = None
    while previous != body:  # nested font commands, e.g. \mathrm{\textbf{x}}
        previous = body
        body = FONT.sub(r'\1', body)
    body = BARE_FONT.sub('', body)
    body = SCRIPT.sub(lambda m: m[1][1:-1] if m[1].startswith('{') else m[1], body)
    body = COMMAND.sub(lambda m: '' if m[1] in SPACING else SYMBOLS.get(m[1], m[0]), body)
    body = body.replace('{', '').replace('}', '').replace('~', '').replace('−', '-')
    return re.sub(r'\s+', '', body)


def normalize_cell_notation(text):
    """One cell's text with each LaTeX math span replaced by what it renders to."""
    def span(m):
        body = next(group for group in m.groups() if group is not None)
        return render_math(body) if MARKUP.search(body) else m[0]
    return MATH_SPAN.sub(span, text)


def normalize_table_notation(table_html):
    """Apply normalize_cell_notation to the text between the tags of a normalized HTML table."""
    return CELL_TEXT.sub(lambda m: '>' + normalize_cell_notation(m[1]) + '<', table_html)
