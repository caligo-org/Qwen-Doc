# mpdocbench-score: the MPDocBench end-to-end scorer in Rust

A port of `pdf_validation.py --config <end2end config>` (dataset `end2end_dataset`,
`quick_match`, task `end2end_eval`). It reads the same YAML config and inputs, and it writes
the same files to `<out>/result/<save_name>/`: `metric_result.json`,
`<element>_result.json`, and the per-page, per-table and per-sample score files. The
Python scorer in this directory stays the reference.

No Python runs at score time. Formula CDM runs the same external programs the Python code
runs: `node` with the toolkit's KaTeX tokenizer, `xelatex`, ImageMagick `magick` and
Ghostscript, plus `perl` for tabular input. Put the same toolchain on `PATH` as for the
Python scorer.

## Build and run

```bash
cd MPDocBench/rust
cargo build --release
./target/release/mpdocbench-score --config ../configs/end2end.yaml --out .. --jobs 16 --cdm-jobs 16
```

Options:

- `--variant marathon` applies the semantics of MarathonDocBench's evaluator patch
  (`utils/mpdocbench_patch`): wider truncated-text merging, `_fix_span_attrs` before TEDS,
  skipping pylatexenc for nesting deeper than 10 braces, and empty predictions scoring
  0 in CDM.
- `--jobs` sets document-level parallelism (default `MPDOC_EVAL_NPROC`, else all cores).
  `--cdm-jobs` sets how many formulas are rendered at once (default `MPDOC_CDM_NPROC`).
  Each worker is a single thread. The Rust scorer starts no extra thread pools.
- `--simple-match FILE` and `--teds-zero FILE` reproduce a Python run whose wall-clock
  limits fired:
  - `--simple-match` takes the documents the 30 s `func_timeout` sent to
    `match_gt2pred_simple`. They appear in the Python log as
    `Time out for plain text match of X`.
  - `--teds-zero` takes the tables `MPDOC_TEDS_TIMEOUT` scored as 0, one
    `element/<img_id>_<gt_idx>` per line.

  Python's results then depend on machine load. Rust has no wall-clock limits, so it
  always runs the full algorithm.

If no requested element needs the text match (for example a heading-only config), the
text match is skipped.

## Layout

| crate | ports |
|---|---|
| `mpdocbench` | CLI; `dataset/end2end_dataset.py`; `utils/match_quick.py` and `utils/match.py`; `utils/extract.py`, `utils/data_preprocess.py` and `table_utils`; Edit_dist, TEDS, HeadTEDS, FigureF1 and CDM drivers (`metrics/cal_metric.py`); `metrics/show_result.py`. Also numpy's pairwise and pandas' Kahan summation orders, scipy `linear_sum_assignment`, CPython set iteration order, and Python's `json.dump` format |
| `pylatexenc` | `LatexNodes2Text().latex_to_text` (pylatexenc 2.10, default configuration) |
| `pysoup` | `html.parser` (CPython 3.11.18) and BeautifulSoup 4.11.1, as far as table normalisation and `merge_tables` use them; `html.unescape` |
| `teds` | table TEDS (lxml/libxml2 HTML parsing through a C shim) and Heading TEDS; a line-by-line port of `apted` 1.0.3, whose results are not always the optimal edit distance |
| `cdm` | `metrics/cdm_metric.py` and `metrics/cdm/modules/*`; skimage 0.20 `ransac` with numpy's PCG64 random stream; the Pillow operations used |

The matcher's quirks are kept on purpose, because they decide the published numbers.
Examples: indices into unfiltered item lists after filtering, `list.remove` while
iterating, and first-pred-only distances for multi-GT merges. Exactness is checked by
differential tests in each crate (`cargo test`). The `cdm` end-to-end test needs the
render toolchain: `cargo test -p cdm --test regression -- --ignored`.

One part is exact but not literal. The matcher's sliding-window fuzzy distances are only
ever compared with fixed thresholds. The port computes them with a Sellers lower bound
and evaluates exactly only the windows that could fall under the threshold. Every
comparison gives the same outcome, at a fraction of the cost.

## Known limits

- Inputs Python itself cannot represent the same way are not handled identically: lone
  surrogates, and recursion depth limits. These only approximate Python's
  `RecursionError` cut-offs.
- The TEDS HTML parser links the system libxml2 (2.9.13 on macOS). lxml 5.3.0 bundles
  2.12.9. `teds` rewrites the input where the two versions build different trees, which
  covers all benchmark data. 28 of 55,754 adversarial fuzz snippets still differ. Pointing
  `TEDS_LIBXML2_INCLUDE` / `TEDS_LIBXML2_LIB` at a 2.12.9 build removes the difference.
- Unicode tables follow Python 3.11 (Unicode 14) where it matters: NFKC, `isspace`,
  `\w`, and `lower`.
