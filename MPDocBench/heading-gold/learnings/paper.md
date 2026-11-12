# paper learnings

## Decisions (recurring)
- L1 = paper title (Chinese title for chn papers). The English title block (translated title + authors + abstract) is a
  translation of the title -> not_heading.
- Run-in labels in text blocks (摘要：, 关键词：, Abstract:, Key words:, "1.1 实验动物 30只..." run-in subsections) are
  text; there is no node for them.
- Numbered sections (0 前言, 1 ..., 2 ...) L2; 2.1 L3; 2.1.1 L4; 参考文献 / References L2 (back matter at top-section
  level).
- Journal headers (journal name, 全国中文核心期刊), column labels (• 实验研究 •), CLC/doc codes (中图分类号,
  文献标识码, 文章编号), received dates, author bios are text.
- Foreign-article fragments: journal pages often carry the end of the previous article (its 参考文献) or a jumped part
  of another article ("（上接第49页）" + its sections "3 ...", "4 ..."). These headings are not part of this document's
  outline -> text (not_headings). Reason: their parent title is not in the document, and as L1 roots they would
  capture this paper's later headings by the level rule (paper_chn_5650c4e6 p6: foreign 参考文献 above this paper's
  jumped 参考文献).
- A paper's own section jumped to a later page ("（下转第51页）" / "（上接第30页）") is a normal node at its level when it
  first appears there (no `continues`, there is no earlier node).
- Own-line numbered item labels ("1）目标优先级", "2）任务时间约束") printed on their own line before their content are
  headings one level below the subsection, all of them alike (same look = same level). Numbered items run into the
  paragraph are text.
- English title blocks placed at the end (English abstract after the references, or a "Summary for ..." page) are
  translations of the title -> not_heading, same as the front English block.
- Footnote markers on the title (①, *, ^1) are left out of the node text.
- A second article that starts on a later page of the document (its own title, authors, abstract) is treated like a
  foreign fragment (text) when the document's own paper continues after it on the same page
  (paper_chn_a197458e p2): the level rule would otherwise hang this paper's next section under the other title.
- Bold run-in subsection numbers ("3.1.1 上、中、下游流域年际变化趋势  简单的...") inside a paragraph are text.
- Journal subscription notices ("欢迎订阅2015年《煤炭学报》杂志") at the end are ads -> text.
- Gold sometimes misses own-line numbered subheadings printed in body font ("1.金融机构...", "2. 供应链金融的定义...")
  while their siblings are gold headings: add them (same look = same level).
- Section numbering decides the level when gold flattens it (paper_chn_fdabe9a2 "1.1" was L2 -> L3).
- Journal logos/mastheads printed big on the cover (e.g. a "Review of ... Research" logo box) are text even when gold
  made them L1; the paper title is L1 (paper_eng_bba70704: all levels shifted up by one).
- Numbered run-in subsections in AMS/math style ("1.1. Preamble. This paper ...", bold number + title starting the
  paragraph) are text (run-in label rule), even though they are numbered.
- Unnumbered "Declarations", "Acknowledgements", "Authors", "Appendix X ..." printed as heading lines are L2 back
  matter.
- Own-line subheadings in body-size font under a top section (e.g. "Study Design", "Data Collection" under
  "Material and Methods"; italic caps "PERIOD OF THE STUDY") are headings one level below, even when gold missed them.
- Bold run-in paragraph leads ("Stage 1: VST-SFT.", "Ablation on training schedule.", "Quantitative Analysis in
  Benchmarks") are text.

## Open questions
- Foreign-article fragment headings as text (see above): alternative is L1 roots; chosen text because the tree cannot
  give them their real (unprinted) parent.
- Own-line "1）... 2）..." item labels in body font (paper_chn_5650c4e6 3.2/3.3): taken as headings; they are printed in
  body style, not heading style.
- paper_chn_a197458e: gold had the second article (响水涧..., p2) as a second L1 root, with this paper's "4 基地建设"
  parented to the first title across it. Not expressible with reading-order parents; second article taken as text.
- Back-placed English abstract blocks: taken as text (translation). Alternative: an L2 back-matter heading
  ("Abstract" section) — there is no printed "Abstract" heading line, only the English title.
- paper_eng_ff3a3862: "Appendix" heading, then "A Details ...", "B ...", "C ..." printed at the same size as the body
  sections. Kept gold nesting (Appendix L2 > A L3 > A.1 L4) because the Appendix label groups A-C; by look alone A-C
  would be L2 siblings of Appendix.
- paper_eng_d054de2a: REFERENCES (gold L3 under ACKNOWLEDGEMENT) taken as L2 (same look as ACKNOWLEDGEMENT).

## Label rule pass
- 参考文献： / Abstract: = real section titles that end in a colon: kept L2.
- Numbered outline heading ending in a colon (paper_chn_509e934a 3.2.1 下列文件应在船上得到确认：) = not a label: kept.
- Body-style own-line lead-in with no sibling label (paper_chn_509e934a 基于人类健康标准：) = text, left as text.
