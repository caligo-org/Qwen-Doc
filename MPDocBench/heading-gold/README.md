# Heading gold trees for MPDocBench (v3)

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

- **Test:** a line is a heading only if it describes everything in its subtree (containment), every path from it to
  the title reads as a breadcrumb (each step narrower than the one above), and it has a substantial block of its own
  (substance). Everything else is text, however it is printed.
- **Unions:** a heading is a union of one or more printed boxes: number + title, chapter label + title, a title split
  over boxes, a title + its translation, and on covers **owner + document type** ("section 5 of the Naruida annual
  report": 广东纳睿雷达科技股份有限公司 ∪ 招股说明书; a logo's text joins when the owner is printed only as a logo).
- **Levels:** same look = same level among outline headings; numbering settles ties; where look and content disagree,
  content decides. Every parent comes earlier in reading order, so every tree can be written as Markdown levels.
- **A heading printed again:** a **repeat** (its section is still open: running head, "(continued)") is not a node;
  a **continuation** (other content came in between: a jumped story, a resumed section) is a new node with
  `continues` pointing to the original.
- **Decisions on doubtful kinds** (all in `NOTATIONS.md`): repeated own-line labels with their own block are headings;
  interview questions with an answer are headings; exam questions are text; national-standard term entries are
  headings; a closing paragraph after a numbered list stays under the last item instead of dropping the list.

## Files

| Path | What |
|---|---|
| `NOTATIONS.md` | The notation and every decision. |
| `trees/<doc>.json` | The gold, schema v3 (`tools/tree.py`): `nodes` (`boxes` = the union, level, parent, text, `why`, optional `continues`), `repeats` (`boxes`, `of`, `why`), `not_headings` and `considered` (text, with `why`). Every entry carries its reason. |
| `MPDocBench.roletrees.json` | `MPDocBench.json` with the trees written in: each node's first box becomes the title with the node text and the union's box; other union boxes, repeats and text decisions become text blocks; title relations are the tree's parents. Built by `to_mpdocbench.py`. |
| `to_mpdocbench.py` | Builds `MPDocBench.roletrees.json` from `../MPDocBench.json` and `trees/` (validates every tree first). |
| `score_tree.py` | Scores explicit trees (`{doc: [{id, text, parent, boxes?, continues?} \| {id, text, repeat_of}]}`): Heading TEDS on the tree without Markdown flattening, plus precision/recall of unions, repeats and `continues`. |
| `tools/check.py` | Validates one tree or all (`--all`): boxes used once, parents reachable in reading order, repeat vs continuation, every gold title accounted for, a `why` on every entry. |
| `tools/breadcrumbs.py` | Prints every breadcrumb of a tree and flags paths that look inconsistent (numbering that does not nest, siblings out of order, a heading repeating its ancestor, front matter under a body section, several roots). |
| `tools/listing.py`, `tools/show.py`, `tools/suggest_repeats.py`, `tools/coverage.py` | Review tools: element listing, page renders with every box labelled (`MPDOCBENCH_PAGES` = folder of `<doc>/page_<n>.jpg`), repeat candidates, pages not yet reviewed. |
| `ACCEPTED_FLAGS.json` | Checker flags reviewed on the page, by kind, with the reason each flagged path is consistent. |
| `learnings/<type>.md` | Decision log per document type: rounds of annotation and breadcrumb review, accepted checker flags with reasons, open cases. |

## How it was made

Every document was annotated from scratch by one reviewer per document type, then reviewed twice more by independent
reviewers who read every breadcrumb of every document and checked doubtful paths on the page; every page of the
3,135 was looked at. Remaining checker flags are cases the documents print themselves (two consecutive "1.3" slides,
a list numbered inside a section, titleless excerpts with several top-level sections); each is logged as ACCEPTED with
its reason in `learnings/`.

## Numbers

| | Original gold | These trees |
|---|---:|---:|
| Headings | 6,309 titles | 5,666 nodes |
| Multi-box headings (unions) | — | 290 |
| Repeats / continuations | — | 1,212 / 55 |
| Titles with no relation | 21 | 0 |
| Parents not expressible as Markdown levels | 116 | 0 |

- Every tree, written as Markdown with its levels, scores exactly 1.0 against `MPDocBench.roletrees.json` with the
  unchanged Heading TEDS. Fed back as explicit trees, `score_tree.py` gives TEDS 100.00 and all unions, repeats and
  continuations correct.
- A stored third-party parser output (uocr2) moves from 60.12 to 58.81 Heading TEDS (held-out half 60.58 → 58.28),
  scored with the Markdown-expressible gold tree of caligo-org/Qwen-Doc#17.

## Use

```
python3 tools/check.py --all
python3 tools/breadcrumbs.py --all       # flags; every remaining one is ACCEPTED in learnings/
python3 to_mpdocbench.py                 # rebuild MPDocBench.roletrees.json after editing trees
python3 score_tree.py pred.json          # explicit-tree predictions
```
