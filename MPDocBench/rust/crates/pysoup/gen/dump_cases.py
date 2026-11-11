"""Build differential-test cases for the pysoup crate.

Runs the *reference* Python implementation (CPython 3.11 + bs4 4.11.1 + the
MPDocBench toolkit) and writes JSONL lines
    {"f": <function>, "in": <input>, "out": <output>}   or
    {"f": <function>, "in": <input>, "err": <exception class name>}

Functions: unescape (html.unescape), soup (str(BeautifulSoup(x,'html.parser'))),
norm (utils/data_preprocess.normalized_html_table), norm_rel
(utils/merge_tables.normalized_html_table), td (match.py cell strings),
merge (utils/merge_tables.merge_tables, input is a list of strings).

Usage:
  python gen/dump_cases.py --toolkit <MPDocBench dir> --out <dir> [--gt a.json ...]
         [--pred-dir d ...] [--fuzz N] [--seed S]
"""
import argparse
import html
import io
import json
import os
import random
import re
import sys
import contextlib

ap = argparse.ArgumentParser()
ap.add_argument('--toolkit', required=True)
ap.add_argument('--out', required=True)
ap.add_argument('--gt', nargs='*', default=[])
ap.add_argument('--pred-dir', nargs='*', default=[])
ap.add_argument('--fuzz', type=int, default=0)
ap.add_argument('--seed', type=int, default=1234)
ap.add_argument('--pairs', type=int, default=4000)
ap.add_argument('--only', nargs='*', default=None, help='restrict to these corpora: gt pred adv fuzz')
args = ap.parse_args()

sys.path.insert(0, args.toolkit)
from bs4 import BeautifulSoup  # noqa: E402
from utils.data_preprocess import normalized_html_table as norm_dp  # noqa: E402
from utils.merge_tables import normalized_html_table as norm_rel, merge_tables  # noqa: E402

os.makedirs(args.out, exist_ok=True)
rng = random.Random(args.seed)


def td_strings(h):
    soup = BeautifulSoup(h, 'html.parser')
    return [re.sub(r'\$\\cdot\$', '', item.string).strip() for item in soup.findAll('td') if item.string]


def soup_str(h):
    return str(BeautifulSoup(h, 'html.parser'))


def quiet_merge(lst):
    with contextlib.redirect_stdout(io.StringIO()):
        return merge_tables(lst)


FUNCS = {
    'unescape': html.unescape,
    'soup': soup_str,
    'norm': norm_dp,
    'norm_rel': norm_rel,
    'td': td_strings,
    'merge': quiet_merge,
}

SURR = re.compile('[\ud800-\udfff]')


def has_surrogate(x):
    if isinstance(x, str):
        return bool(SURR.search(x))
    if isinstance(x, list):
        return any(has_surrogate(y) for y in x)
    return False


stats = {}


def emit(fh, f, inp):
    try:
        out = FUNCS[f](inp)
        rec = {'f': f, 'in': inp, 'out': out}
    except RecursionError:
        rec = {'f': f, 'in': inp, 'err': 'RecursionError'}
    except Exception as e:  # noqa: BLE001
        rec = {'f': f, 'in': inp, 'err': type(e).__name__}
    key = (fh.name, f)
    if has_surrogate(rec.get('out')) or has_surrogate(inp):
        stats[key + ('surrogate-skipped',)] = stats.get(key + ('surrogate-skipped',), 0) + 1
        return
    stats[key] = stats.get(key, 0) + 1
    fh.write(json.dumps(rec, ensure_ascii=False) + '\n')


def want(name):
    return args.only is None or name in args.only


# ----------------------------------------------------------------------------- GT
def remove_consecutive_duplicates(arr):
    if not arr:
        return []
    result = [arr[0]]
    for i in range(1, len(arr)):
        if arr[i] != arr[i - 1]:
            result.append(arr[i])
    return result


gt_tables = []
groups = []
n_trunc_groups = [0]
for path in args.gt:
    docs = json.load(open(path))
    for doc in docs:
        doc_tables = []
        for x in doc['layout_dets']:
            if x['category_type'] == 'table':
                for k in ('html', 'merged_html'):
                    if isinstance(x.get(k), str):
                        gt_tables.append(x[k])
                if isinstance(x.get('html'), str):
                    doc_tables.append(x['html'])
        # truncated groups exactly as get_page_elements builds them
        related, trunc = [], {}
        for rel in doc['extra']['relation']:
            if rel['relation_type'] == 'truncated':
                trunc[rel['source_anno_id']] = ''
                trunc[rel['target_anno_id']] = ''
                hit = False
                for ml in related:
                    if rel['source_anno_id'] in ml or rel['target_anno_id'] in ml:
                        ml.append(rel['source_anno_id'])
                        ml.append(rel['target_anno_id'])
                        hit = True
                if not hit:
                    related.append([rel['source_anno_id'], rel['target_anno_id']])
        for item in doc['layout_dets']:
            if item['anno_id'] in trunc:
                trunc[item['anno_id']] = item
        for ml in related:
            ml = remove_consecutive_duplicates(ml)
            blocks = [trunc[k] for k in ml]
            if any(not isinstance(b, dict) for b in blocks):
                continue
            sb = sorted(blocks, key=lambda b: b['order'])
            # get_page_elements only calls merge_tables when merged_html is missing
            # (never the case in the current GT files); run it on every truncated
            # table group, which is exactly the input it would get.
            if sb[0]['category_type'] == 'table':
                if all(isinstance(b.get('html'), str) for b in sb):
                    groups.append([b['html'] for b in sb])
                    n_trunc_groups[0] += 1
        # consecutive pairs/triples inside the document
        for i in range(len(doc_tables) - 1):
            if rng.random() < 0.3:
                groups.append(doc_tables[i:i + 2 + (rng.random() < 0.3)])

if want('gt') and gt_tables:
    with open(os.path.join(args.out, 'gt.jsonl'), 'w') as fh:
        for t in gt_tables:
            for f in ('unescape', 'soup', 'norm', 'norm_rel', 'td'):
                emit(fh, f, t)
    with open(os.path.join(args.out, 'merge.jsonl'), 'w') as fh:
        for g in groups:
            emit(fh, 'merge', g)
        for _ in range(args.pairs):
            k = 2 if rng.random() < 0.7 else 3
            emit(fh, 'merge', [rng.choice(gt_tables) for _ in range(k)])
    print('gt tables', len(gt_tables), 'merge groups', len(groups), 'of which truncated groups', n_trunc_groups[0], file=sys.stderr)

# ----------------------------------------------------------------------------- predictions
TABLE_RE = re.compile(r'<table[\s\S]*?</table>', re.I)
if want('pred') and args.pred_dir:
    pred_tables = []
    windows = []
    for d in args.pred_dir:
        for fn in sorted(os.listdir(d)):
            p = os.path.join(d, fn)
            if not os.path.isfile(p):
                continue
            txt = open(p, encoding='utf-8', errors='surrogateescape').read()
            if SURR.search(txt):
                txt = SURR.sub('�', txt)
            for m in TABLE_RE.finditer(txt):
                pred_tables.append(m.group())
            # unclosed / malformed: <table ... up to the next blank line
            for m in re.finditer(r'<table', txt, re.I):
                end = txt.find('\n\n', m.start())
                seg = txt[m.start(): end if end >= 0 else len(txt)]
                if '</table>' not in seg.lower():
                    pred_tables.append(seg)
                # random window around a table start (cuts tags in half)
                if rng.random() < 0.2:
                    a = max(0, m.start() - rng.randint(0, 200))
                    windows.append(txt[a: a + rng.randint(50, 3000)])
    seen = set()
    uniq = []
    for t in pred_tables:
        if t not in seen:
            seen.add(t)
            uniq.append(t)
    with open(os.path.join(args.out, 'pred.jsonl'), 'w') as fh:
        for t in uniq + windows:
            for f in ('unescape', 'soup', 'norm', 'norm_rel', 'td'):
                emit(fh, f, t)
    with open(os.path.join(args.out, 'merge_pred.jsonl'), 'w') as fh:
        for _ in range(args.pairs // 2):
            k = 2 if rng.random() < 0.7 else 3
            emit(fh, 'merge', [rng.choice(uniq) for _ in range(k)])
    print('pred tables', len(uniq), 'windows', len(windows), file=sys.stderr)

# ----------------------------------------------------------------------------- adversarial
ADV = [
    '', ' ', '\n', '<', '&', '<!', '</', '</>', '<?', '<!--', '<!---', '<!---->', '<!-->', '<!--->', '<!-- a --!>b',
    '<!-- unclosed', '<!-- unclosed -', '<!-- unclosed --', '<!-- unclosed --!', '<![CDATA[x]]>y', '<![CDATA[x',
    '<![if x]>y<![endif]>', '<![foo>', '<!DOCTYPE html><p>x', '<!doctype', '<!DocType html', '<!x>', '<!x',
    '<?php echo 1 ?>z', '<?x', '</ a>', '</1>', '</a b c', '</td foo="bar">', '</td\n>', '<a b=\'c"d\'>x</a>',
    '<a b="c\'d">', '<a b="c\'d" e=\'f"g\'>', '<a b=c"d>', '<a b= "c>', "<a b= 'c>", '<a b=\'c>', '<a b ="c" >',
    '<a b=>', '<a =b>', '<a b/c>', '<a b/>', '<a/b>', '<a / >', '<a b c d>', '<a b="1" b="2">', '<A HREF="x">X</A>',
    '<TD>x</Td>', '<td class=" a  b ">x</td>', '<td class="">x</td>', '<td headers="h1 h2">', '<p class="a\x0cb">',
    '<script>a<b>c</b>&amp;</script>d', '<script>x</script >y', '<script>x</scriptx>y</script>z', '<script>x',
    '<script>x</script', '<style>p{}</style><p>a&b</p>', '<title>a&amp;b<i>c</i></title>', '<title>x &amp',
    '<textarea>  </textarea>', '<textarea> a <b> </textarea>', '<pre>  \n </pre>', '<pre> <b> </b> </pre>',
    '<td>   </td>', '<td>\n \n</td>', '<td> \x1c </td>', '<td>\xa0</td>', '<xmp><b></xmp>', '<plaintext><b>x</b>',
    '<iframe><td></iframe>', '<noembed>a</noembed>', '<noframes>a</noframes>', '<noscript><b>x</b></noscript>',
    '<br></br>', '<br/><br>', '<br><br/>x', '<br/>', '<br />', '<br>x</br>', '<img src=x>y', '<td><br>a</td>',
    '<hr/><hr></hr>', '<input value="a&amp;b">', '<a title="&lt;&gt;&amp;&quot;">', '<a title="&notit;">',
    'a &amp b', 'a &amp', '&amp', '&a', '&', '& ', '&#', '&#;', '&#x;', '&#65', '&#65;', '&#65a;', '&#x41g;',
    '&#x41', '&#128;', '&#129;', '&#0;', '&#x110000;', '&#xD800;', '&#99999999999999999999;', '&#1114111;',
    '&nbsp;&nbsp', '&notit;', '&notin;', '&amp.x;', '&amp-x', '&am', '&AMP;', '&Amp;', 'x&y=z', '&lt&gt',
    'a & b ; c', '&#12a; <b>x</b>', '&#12a; <b>x</b> &#12a; <i>y</i>', '<td>&#12a;</td><td>b</td>',
    '<table><tr><th>a</th></tr></table>', '<table><thead><tr><th>a</th></tr></thead><tbody><tr><td>1</td></tr></tbody></table>',
    '<table><tr><td><math alttext="x^2"><mi>x</mi></math></td></tr></table>',
    '<table><tr><td><math><mi>x</mi></math></td></tr></table>',
    '<table><tr><td><span>a<span>b</span></span></td></tr></table>',
    '<table><thead><thead><tr><th>a</th></tr></thead></thead></table>',
    '<table><tr><td><math alttext="a"><math alttext="b"></math></math></td></tr></table>',
    '<table style="x"><tr><td style="color:red" width="3" class="c">a</td></tr></table>',
    '<TABLE><TR><TD>a</TD></TR></TABLE>', '<table><tr><td>a</td></tr></table><table><tr><td>b</td></tr></table>',
    '<table><tr><td>a</td></tr>', '<table border=1><tr><td colspan=2>a</td></tr></table>',
    '<table><colgroup><col></colgroup><tr><td>a</td></tr></table>',
    '<table><tr><td>$\\cdot$ x</td><td><b>y</b></td><td><!--c--></td><td></td><td>a<b>b</b></td></tr></table>',
    '<table><tr><td> a </td>\n <td>b\n</td></tr></table>', '<table><tr><td><sup>1</sup><sub>2</sub><p>p</p><div>d</div></td></tr></table>',
    '<meta charset="latin1"><table><tr><td>a</td></tr></table>',
    '<meta http-equiv="Content-Type" content="text/html; charset=latin1"><table></table>',
    '<table><tr><td>Ａ　Ⅰ</td></tr></table>', '<table><tr><td>é ﬁ</td></tr></table>',
    '<tableé><tr><td>x</td></tr></table>', '<table_x><td>a</td></table>', '<tablex>a</tablex></table>',
    '<table><td>a<td>b<td>c</table>', '<td><tr><table>', '<table><tr><td>a</td></tr></table >', '<table>a</TABLE>',
    '<ruby>a<rt>b</rt><rp>(</rp></ruby>', '<template><td>a</td></template>', '<td><script>s</script>t</td>',
    '<a b="\n">', '<a\nb\n=\n"c"\n>', '<a\tb\x0c=c>', '<a b=c\x0bd>', '<a\x0bb>', '<a\x1cb>', '<a b=\'\'>', '<a b="">',
    '<p<p>', '<p =>', '<p ==a>', '<p a==b>', "<p a='b'c='d'>", '<p a="b"c>', '<p a=\'b>c\'>', '<p a="b>c">',
    '<p/ a>', '<p a/ b>', '<p a=/>', '<p a=b/>', '<p a="b"/>', '<p a/>', '<p//>', '<td/><td>', '<td/>x',
    '<p></p></p></p>', '<b><i></b></i>', '<div><p>a</div>b', 'Å<é>', '<é>', '<aé>x</aé>',
    '<Aİ>x</aİ>', '<a K=1>', '<a K=1>', '<I>x</i>', '<td>\r\n</td>', '<td>\r</td>', 'a\rb',
]
NAMES = ['td', 'tr', 'th', 'table', 'thead', 'tbody', 'span', 'math', 'br', 'b', 'p', 'pre', 'textarea', 'script',
         'style', 'title', 'img', 'sup', 'colgroup', 'col', 'div', 'TD', 'Tr', 'a', 'x-y', 'plaintext', 'rt', 'meta']
ATTRS = ['colspan', 'rowspan', 'class', 'style', 'alttext', 'width', 'headers', 'id', 'COLSPAN', 'b', 'charset']
PIECES = ['<', '>', '</', '/>', '/', '=', '"', "'", ' ', '  ', '\n', '\t', '\x0c', '\r', '\x0b', '\x1c', '\xa0', '&',
          '&amp;', '&amp', '&lt;', '&#', '&#12;', '&#x41;', '&#65a;', '&nbsp', '&notin;', '&notit', ';', '<!--', '-->',
          '--!>', '<!', '<?', '<![CDATA[', ']]>', '<!DOCTYPE ', 'a', 'b', '1', '2', 'x y', 'é', '中', '１',
          '$\\cdot$', 'K', '-', '--', '.', ':']


def fuzz_one():
    out = []
    for _ in range(rng.randint(1, 30)):
        r = rng.random()
        if r < 0.35:
            nm = rng.choice(NAMES)
            s = '<' + nm
            for _ in range(rng.choice([0, 0, 1, 1, 2, 3])):
                s += rng.choice([' ', '  ', '\n', '/', '\t', ''])
                s += rng.choice(ATTRS)
                if rng.random() < 0.7:
                    s += rng.choice(['=', ' = ', '= ', ' ='])
                    q = rng.choice(['"', "'", '', '"', ''])
                    v = rng.choice(['1', '2', ' 3 ', 'a b', '', 'x&amp;y', 'x>y', '"', "'", '&#65;', '２', 'a"b'])
                    s += q + v + (q if rng.random() < 0.9 else '')
            s += rng.choice(['>', '>', '>', '/>', ' />', '', ' >'])
            out.append(s)
        elif r < 0.6:
            out.append('</' + rng.choice(NAMES) + rng.choice(['>', '>', ' >', '', 'x>', ' a="b">']))
        elif r < 0.85:
            out.append(rng.choice(['a', 'text', ' ', '  ', '\n', '1', '中文', 'x y z', 'Ａ']))
        else:
            out.append(rng.choice(PIECES))
    return ''.join(out)


def fuzz_table():
    rows = []
    for _ in range(rng.randint(1, 4)):
        cells = []
        for _ in range(rng.randint(1, 4)):
            tag = rng.choice(['td', 'td', 'th'])
            attrs = ''
            if rng.random() < 0.2:
                attrs += ' colspan="%s"' % rng.choice(['1', '2', '3', ' 2', '0', '-1', 'x', '', '２', '1_0'])
            if rng.random() < 0.1:
                attrs += ' rowspan="%s"' % rng.choice(['1', '2', '3'])
            cells.append('<%s%s>%s</%s>' % (tag, attrs, rng.choice(['a', 'b', '1', '', ' a ', 'H', 'Ａ']), tag))
        rows.append('<tr>' + ''.join(cells) + '</tr>')
    head = rng.random() < 0.3
    return '<table>' + ('<thead>' if head else '') + ''.join(rows) + ('</thead>' if head else '') + '</table>'


if want('adv'):
    with open(os.path.join(args.out, 'adv.jsonl'), 'w') as fh:
        for a in ADV:
            for f in ('unescape', 'soup', 'norm', 'norm_rel', 'td'):
                emit(fh, f, a)
                emit(fh, f, '<table><tr><td>' + a + '</td></tr></table>')
        # deep nesting (RecursionError emulation) and wide tables
        for depth in (100, 300, 480, 490, 495, 497, 500, 600):
            emit(fh, 'soup', '<b>' * depth + 'x')
            emit(fh, 'norm', '<table><tr>' + '<td>a' * depth + '</table>')
            emit(fh, 'norm_rel', '<table><tr>' + '<td>a' * depth + '</table>')
        for a in ADV[:60]:
            emit(fh, 'merge', ['<table><tr><td>h</td></tr><tr><td>1</td></tr></table>', '<table><tr><td>h</td></tr>' + a + '</table>'])

if want('fuzz') and args.fuzz:
    with open(os.path.join(args.out, 'fuzz.jsonl'), 'w') as fh:
        for _ in range(args.fuzz):
            s = fuzz_one()
            for f in ('unescape', 'soup', 'norm', 'norm_rel', 'td'):
                emit(fh, f, s)
            emit(fh, 'soup', '<table>' + s + '</table>')
            emit(fh, 'norm', '<table>' + s + '</table>')
            emit(fh, 'norm_rel', '<table>' + s + '</table>')
        for _ in range(args.fuzz // 3):
            k = rng.choice([2, 2, 3])
            g = [fuzz_table() for _ in range(k)]
            if rng.random() < 0.3:
                g[rng.randrange(k)] = fuzz_one()
            emit(fh, 'merge', g)
    # character-level fuzz over a tokenizer-heavy alphabet
    alpha = list('<>/!-?&#;=\'" \n\t\x0c\rabtdrxXA1[]_.') + ['CDATA[', 'td', 'tr', 'table', 'th', 'span',
                                                          'script', 'title', 'br', 'pre', 'amp', 'lt', '\u00e9']
    with open(os.path.join(args.out, 'charfuzz.jsonl'), 'w') as fh:
        for _ in range(args.fuzz * 2):
            s = ''.join(rng.choice(alpha) for _ in range(rng.randint(1, 40)))
            for f in ('unescape', 'soup', 'td'):
                emit(fh, f, s)
            emit(fh, 'norm_rel', '<table>' + s)
            emit(fh, 'norm', '<table><tr><td>' + s + '</td></tr></table>')

for k, v in sorted(stats.items()):
    print(os.path.basename(k[0]), *k[1:], v, file=sys.stderr)
