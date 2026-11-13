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
  report": 广东纳睿雷达科技股份有限公司 ∪ 招股说明书; when the owner is printed only as a logo, the logo box joins).
- **Levels:** same look = same level among outline headings; numbering settles ties; where look and content disagree,
  content decides. Every parent comes earlier in reading order, so every tree can be written as Markdown levels.
- **A heading printed again:** a **repeat** (its section is still open: running head, "(continued)") is not a node;
  a **continuation** (other content came in between: a jumped story, a resumed section) is a new node with
  `continues` pointing to the original.
- **Breaks:** where a section ends without a new printed heading (a label's block ends and the main text resumes, a
  story ends before an ad, a list ends before a closing paragraph about the whole list), a break closes it and the
  content after it belongs to the parent. Breaks change where content sits, never the heading tree; they are annotated
  for later use and not scored. The reader's reading order is never corrected: content stays where the layout's
  reading order puts it.
- **Decisions on doubtful kinds** (all in `NOTATIONS.md`): repeated own-line labels with their own block are headings;
  interview questions with an answer are headings; exam questions are text; national-standard term entries are
  headings; a closing paragraph after a numbered list stays under the last item instead of dropping the list.

## Files

| Path | What |
|---|---|
| `NOTATIONS.md` | The notation and every decision. |
| `trees/<doc>.json` | The gold, schema v3 (`tools/tree.py`): `nodes` (`boxes` = the union, level, parent, text, `why`, optional `continues`), `repeats` (`boxes`, `of`, `why`), `breaks` (`before`, `closes`, `why`), `not_headings` and `considered` (text, with `why`). Every entry carries its reason. |
| `MPDocBench.roletrees.json` | `MPDocBench.json` with the trees written in: each node's first box becomes the title with the node text and the union's box; other union boxes, repeats and text decisions become text blocks; title relations are the tree's parents. Built by `to_mpdocbench.py`. |
| `to_mpdocbench.py` | Builds `MPDocBench.roletrees.json` from `../MPDocBench.json` and `trees/` (validates every tree first). |
| `score_tree.py` | Scores explicit trees (`{doc: [{id, text, parent, boxes?, continues?} \| {id, text, repeat_of}]}`): Heading TEDS on the tree without Markdown flattening, plus precision/recall of unions, repeats and `continues`. |
| `tools/check.py` | Validates one tree or all (`--all`): boxes used once, parents reachable in reading order, repeat vs continuation, breaks close an open section, every gold title accounted for, a `why` on every entry. |
| `tools/roundtrip.py` | Writes every tree as Markdown levels and scores it with the unchanged HeadTEDS against `MPDocBench.roletrees.json` (expect 1.0 on all). |
| `tools/breadcrumbs.py` | Prints every breadcrumb of a tree and flags paths that look inconsistent (numbering that does not nest, siblings out of order, a heading repeating its ancestor, front matter under a body section, several roots). |
| `tools/listing.py`, `tools/show.py`, `tools/suggest_repeats.py`, `tools/coverage.py` | Review tools: element listing, page renders with every box labelled (`MPDOCBENCH_PAGES` = folder of `<doc>/page_<n>.jpg`), repeat candidates, pages not yet reviewed. |
| `path_review/<type>.tsv`, `break_review/<type>.tsv`, `logo_owner.tsv` | Round 4: a verdict for every node, checked on its page. Round 5: every break and restored heading, and a line for every document checked without one. Owner-logo pass: one line per report-type document. `tools/path_coverage.py` checks that every node has a verdict. |
| `final_paths/*.tsv` | Final independent check of the finished trees: one fresh reviewer per document type viewed every page and gave every node a verdict (5,697 OK, 11 fixed, 4 added, 2 removed). |
| `third_pass/*.tsv` | Third independent page check of every node (one fresh reviewer per document type) and `fixes.tsv` with what it changed. |
| `final_check/*.tsv` | Independent final check on the page of every round-5 change: 92 changed nodes (restored headings, owner-logo titles) and all breaks. |
| `ACCEPTED_FLAGS.json` | Checker flags reviewed on the page, by kind, with the reason each flagged path is consistent. |
| `learnings/<type>.md` | Decision log per document type: rounds of annotation and breadcrumb review, accepted checker flags with reasons, open cases. |

## How it was made

Every document was annotated from scratch by one reviewer per document type, then reviewed four more times: two
breadcrumb rounds, a round that checked every node's path on its rendered page (round 4), and a break round that went
through every page again (round 5), an independent check of every round-5 change, and a final independent check of
every path in the finished trees. Remaining checker flags are cases the documents print themselves (two consecutive "1.3" slides,
a list numbered inside a section, titleless excerpts with several top-level sections); each is logged as ACCEPTED with
its reason in `learnings/`.

## Numbers

| | Original gold | These trees |
|---|---:|---:|
| Headings | 6,309 titles | 5,715 nodes |
| Multi-box headings (unions) | — | 309 |
| Repeats / continuations | — | 1,212 / 56 |
| Breaks | — | 342 |
| Titles with no relation | 21 | 0 |
| Parents not expressible as Markdown levels | 116 | 0 |

- Every tree, written as Markdown with its levels, scores exactly 1.0 against `MPDocBench.roletrees.json` with the
  unchanged Heading TEDS. Fed back as explicit trees, `score_tree.py` gives TEDS 100.00 and all unions, repeats and
  continuations correct.
- A stored third-party parser output (uocr2) moves from 60.12 to 59.09 Heading TEDS (held-out half 60.58 → 58.54),
  scored with the Markdown-expressible gold tree of caligo-org/Qwen-Doc#17.

## Use

```
python3 tools/check.py --all
python3 tools/breadcrumbs.py --all       # flags; every remaining one is in ACCEPTED_FLAGS.json
python3 tools/path_coverage.py           # every node has a verdict
python3 tools/roundtrip.py               # Markdown levels round-trip: 1.0 on every document
python3 to_mpdocbench.py                 # rebuild MPDocBench.roletrees.json after editing trees
python3 score_tree.py pred.json          # explicit-tree predictions
```
