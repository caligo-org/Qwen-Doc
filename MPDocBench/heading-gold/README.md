# Heading gold trees for MPDocBench

A re-annotation of the heading structure of all 420 MPDocBench documents, written to one notation
(`NOTATIONS.md`). Each document's headings are one tree file in `trees/<doc>.json`. `MPDocBench.roletrees.json` is
the same trees in MPDocBench's own format, so the unchanged scorer can use it as the gold for Heading TEDS.

## Why

The heading gold in `MPDocBench.json` tags the same kind of line differently from document to document and from page
to page: kickers as the parent of their own headline, one of three identical teasers, callout bands at different
levels, a heading's parent placed after it in reading order, and 21 titles with no relation (the scorer drops them).
116 annotated parents cannot be written as Markdown levels at all, because a continued story or a resumed section is
annotated as the child of the original.

## The notation in short

- A line is a heading only if its entry in the breadcrumb tells you something you need to know deep inside the
  document. On a cover that is the document's title; issuer names, logos, addresses and banners are text.
- Same look = same level among outline headings; numbering settles ties; where look and content disagree, the content
  decides (a label naming the items of a section sits one level below that section).
- A heading printed again has two cases:
  - **repeat** (nothing in between: the next page with "(continued)", a running head): not a node, listed in
    `repeats` with the heading it repeats;
  - **continuation** (other sections or stories came in between: a jumped story, a resumed section): a new node where
    it reappears, with `continues` pointing to the original.
- Every parent is earlier in reading order, so every tree can be written as Markdown levels.

## Files

| Path | What |
|---|---|
| `NOTATIONS.md` | The notation, with the decisions per document type. |
| `trees/<doc>.json` | The gold: `nodes` (anno, level, parent, text, optional `merge`, `continues`), `repeats`, `not_headings` (gold titles that are text). |
| `MPDocBench.roletrees.json` | `MPDocBench.json` with the trees written in: nodes are titles with the tree's parents, merged boxes and not-headings are text blocks, repeats are text. Built by `to_mpdocbench.py`. |
| `to_mpdocbench.py` | Builds `MPDocBench.roletrees.json` from `../MPDocBench.json` and `trees/` (validates every tree first). |
| `score_tree.py` | Scores explicit trees (`{doc: [{id, text, parent, continues?} \| {id, text, repeat_of}]}`): Heading TEDS on the tree without Markdown flattening, plus precision/recall of `continues` links and repeats. |
| `tools/check.py` | Validates one tree or all (`--all`): parents reachable in reading order, repeat vs continuation, every gold title accounted for. |
| `tools/listing.py`, `tools/show.py` | Element listing and page renders with every box labelled (`show.py` needs `MPDOCBENCH_PAGES` = folder of `<doc>/page_<n>.jpg`). |
| `tools/label_candidates.py` | Finds candidates for the label rule. |
| `learnings/<type>.md` | Decision log per document type, including the cases left open. |

## Numbers

| | Original gold | These trees |
|---|---:|---:|
| Headings | 6,309 titles | 5,315 nodes |
| Titles with no relation | 21 | 0 |
| Parents not expressible as Markdown levels | 116 | 0 |
| `repeats` / `continues` links | — | 451 / 48 |

- Every tree, written as Markdown with its levels, scores exactly 1.0 against `MPDocBench.roletrees.json` with the
  unchanged Heading TEDS. Fed back as explicit trees, `score_tree.py` gives TEDS 100.00 and 451/451 repeats,
  48/48 continuations.
- A stored third-party parser output (uocr2) moves from 60.12 to 59.35 Heading TEDS (held-out half 60.58 → 59.58),
  scored with the Markdown-expressible gold tree of caligo-org/Qwen-Doc#17.

## Use

```
python3 tools/check.py --all
python3 to_mpdocbench.py                 # rebuild MPDocBench.roletrees.json after editing trees
python3 score_tree.py pred.json          # explicit-tree predictions
```
