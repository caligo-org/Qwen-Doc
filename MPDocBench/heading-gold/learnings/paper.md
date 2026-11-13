# paper learnings (v3)

## Decisions (recurring)
- L1 = paper title alone (authors, affiliations, journal header, column labels, CLC/doc codes, received dates are text).
- English title of a Chinese paper printed in its own English abstract block (separated from the Chinese title by
  authors / abstract / codes): repeat of L1 (title restated in translation), not a union (not "printed with it").
- Running heads "authors, 等：title" or the short title: repeat of L1.
- Numbered sections L2, 2.1 L3, 2.1.1 L4; 参考文献 / References L2 back matter.
- Headings of other articles sharing the pages (jumped "上接第49页" parts, end of previous article): not_headings.
- Column labels ("• 实验研究 •") above the title: text.
- Footnote markers on the title (*, ①) left out of the node text.
- Own-line numbered item labels ("1）目标优先级" on its own line, then text/equation): headings one level below the
  subsection, all siblings alike. Items run into a paragraph ("1) 设空间部署 n 颗...") are text (considered).
- A paper's own section jumped to a later page after another article's tail (上接第30页): normal node at its level.
- Lead-in sentences ending in a colon before a list ("基于人类健康标准：") are text.
- English title block placed after the references (or on a "Summary for <title>" page): repeat of L1 as well; the
  "Summary for ..." kicker is text.
- Bold run-in numbered subsections inside a paragraph ("3.1.1 上、中、下游流域年际变化趋势  简单的..."): text.
- Second article starting on a later page while this paper continues after it (paper_chn_a197458e p2): the other
  article's title/sections are not_headings; this paper's next section stays under its own L1.
- Journal subscription notices / ads at the end ("欢迎订阅...《煤炭学报》"): not_headings.
- Own-line subheads printed in body font ("1. 金融机构...在财务供应链管理中发挥的作用") whose siblings are headings:
  headings at the sibling level, even when gold missed them.
- Numbering beats gold flattening (1.1 under 1 is L3 even when gold put it at L2).
- Column labels in the title box (【水文·泥沙】 + title in one box): node text = the title only.
- English papers: own-line "Abstract" heading = L2 front matter; run-in "Abstract—" / "Abstract:" inside the paragraph = text.
- IEEE / AMS run-in subsubsections ("1) Open source projects: The evaluation...", italic lead in the paragraph) and bold
  run-in paragraph leads ("Numeric filter. The numeric...", "Acknowledgements This material..."): text (considered).
- Own-line lettered / numbered item headings with their own paragraphs below ("a. The Effect of ...", "1. Report delay
  and audit delay"): headings one level below their subsection.
- Unnumbered own-line subheads in body-size font ("Study Design", "Data Collection", italic caps "PERIOD OF THE
  STUDY", "REGION-WISE FDI INFLOWS IN INDIA") with their own paragraph: one level below the section, even when gold
  missed them.
- Journal logo / masthead printed big on the first page (e.g. a journal-name box) is a journal header: not_heading;
  the paper title is L1 (paper_eng_bba70704).
- Repository cover sheet (redalyc etc.) printing the title inside a citation record: considered text; L1 is the
  title on the paper's own first page (paper_eng_fd23041b). Article-type labels ("Original Article"): text.
- Subtitle / deck + authors under the title: text. Titled summary box ("Lessons Learned") on page 1 with its own
  block: L2 at top-section level (paper_eng_f30006b1).
- Unnumbered back matter in heading style (Declarations, Acknowledgements, Authors, 【作者简介】, Appendix X):
  L2. Contents title: L2 front matter.
- Reading order beats gold parenting: subsections after "Method" belong to Method even if gold hung them under the
  previous section (paper_eng_d76efc79).
- Subsections numbered 4.1 under "4 ..." printed smaller: L3 even when gold made them L2 (paper_eng_b8670bda).

## OPEN
- English title restated in the English abstract block / back-placed English summary: taken as a REPEAT of L1
  (title printed again, in translation). Alternatives: not_heading (translation is text) or an L2 back-matter
  "English abstract" node (no printed "Abstract" heading exists). 14 docs affected.
- paper_eng_ff3a3862: "Appendix" label then "A ...", "B ...", "C ..." printed at top-section size. Taken as
  Appendix L2 > A/B/C L3 > A.1 L4 (content: the label groups A-C). By look alone A-C would be L2 siblings.
- paper_eng_d054de2a: REFERENCES (gold L3 under ACKNOWLEDGEMENT) taken as L2, same look as ACKNOWLEDGEMENT.
- Own-line "1）... 2）..." item labels in body font (paper_chn_5650c4e6 3.2/3.3) and "（1）/（2）" labels
  (paper_chn_7d70eb40 4.2.1): taken as headings; they are body-styled.
- Foreign-article fragments (other articles' sections / references on shared journal pages): not_headings. The tree
  cannot give them their real (unprinted) parent; as L1 roots they would capture this paper's later sections.
- paper_chn_a197458e: second article's title on p2 while this paper continues after it: not_heading (gold had a
  second L1 root); this paper's "4 基地建设" stays under its own L1.

## Breadcrumb review round 2
38 docs, 539 breadcrumb paths read; no unseen paper pages listed. No tree changes: every path reads as a narrowing
breadcrumb, every L1 is the paper title alone, no FRONT/DEEP/DUP/NUM/ROOTS flags. All 7 flags are SEQ parser misses
(the skipped sibling's number is not parsed: no space after the digit, a lowercase / "of" title, or a title >= 40 chars);
the printed numbering is continuous in each case.
- ACCEPTED paper_chn_7d70eb40 #14 SEQ: "2自行车交通特征分析" (no space) is section 2; numbering 1-6 is continuous.
- ACCEPTED paper_chn_7d70eb40 #120 SEQ: "4自行车…" and "5厦门…" (no space) sit between 3 and 6.
- ACCEPTED paper_chn_ceb8680b #38 SEQ: "2煤炭安全评价指标体系构建" (no space) is section 2.
- ACCEPTED paper_chn_da20c59b #153 SEQ: section 3 "3 “蒸发悖论”的探讨" starts with a quote mark; 1-4 continuous.
- ACCEPTED paper_eng_20a93115 #359 SEQ: sections 2 and 3 exist ("2 Topological networks on…", "3 DeepONets on…").
- ACCEPTED paper_eng_4b1e42ed #34 SEQ: "3 Background and Notation", "4 Cohort and Data…" exist; 1-7 continuous.
- ACCEPTED paper_eng_b8670bda #301 SEQ: "6 Proof of Uniqueness" exists; 1-7 continuous.
Checked and kept (no fix):
- paper_eng_369ef675 p6: "Classification of Article Categorization" and the "Article Classification Based on …"
  heads have the same look: siblings at L3 (look decides; the first one's block is a method note, not a group label).
- paper_eng_d054de2a p2: "Airports" / "Air transport service" / "Gross Domestic Product" under "II. CIVIL AVIATION":
  printed as that section's subheads; kept (the section introduces the study's terms).
- paper_chn_a197458e p1-2: 3.1-3.6 and 4.1-4.2 are run-in / one-sentence items: text, like 3.1 (considered).
- paper_chn_509e934a 3.1.2 / 3.2.1 (numbered lead-in sentences ending in a list): numbered outline steps, kept.
OPEN: none new.

## Breadcrumb review round 3
38 docs, 542 breadcrumb lines read (all nodes); no unseen paper pages listed in review/unseen_pages.txt. Pages
viewed: a197458e p2, da20c59b p4/p12, 4b1e42ed p2-3, b8670bda p12/p15, fdabe9a2 p3, ff3a3862 p19-20, d054de2a p1-2.
No structural change: every node path reads as a narrowing breadcrumb; every L1 is the paper title alone; no node
text is a bare marker; no continuation without headline.
Fix class (why only, 4 entries): back-placed English title block after the references (b4b85a83 #108, be9986ff #288,
da20c59b #219, fdabe9a2 #88): kept as repeat of L1, why now states it is the restated whole-document title
(NOTATIONS "A restated title of the whole document inside it is a repeat of L1") and closes 参考文献, so the English
authors/abstract after it are not in the references' subtree.
- ACCEPTED paper_chn_da20c59b #153 SEQ: "3 “蒸发悖论”的探讨" (#66, p4) is printed as a section heading and is a node;
  the parser skips it because a quote mark follows the number. Numbering 1-4 continuous.
- ACCEPTED paper_eng_4b1e42ed #34 SEQ: "3 Background and Notation" (#17, p2) and "4 Cohort and Data Pre-Processing"
  (#28, p3) are printed headings and nodes; the parser drops titles with "and" as the second word. 1-7 continuous.
- ACCEPTED paper_eng_b8670bda #301 SEQ: "6 Proof of Uniqueness" (#224, p12) is printed and a node; the parser drops
  titles with "of". 1-7 continuous.
Resolved OPEN cases (path-related):
- English title of Chinese papers in its own English block (12 docs: 8 front, 4 back): REPEAT of L1, final. NOTATIONS
  v3: L1 ∪ translation only when "printed with it"; a title printed again later is a repeat; a restated whole-document
  title is a repeat of L1, not a child. Not a node, so it adds no path.
- paper_eng_ff3a3862 Appendix > A/B/C > A.1: kept. "Appendix" is a back-matter title (Usually headings), not a bare
  marker: it labels and contains A-C (only appendices follow it to the end); path "title › Appendix › A Details of VST
  Inference › A.1 Inference Prompt" narrows at each step.
- paper_eng_d054de2a REFERENCES L2: kept (same look as ACKNOWLEDGEMENT; sibling back matter).
- paper_chn_5650c4e6 / 7d70eb40 own-line item labels: kept; each path ("3.2 目标函数 › 1）目标优先级",
  "4.2.1 中心城区——改善为主 › （1）现状道路断面优化调整") narrows.
- Foreign-article fragments (a197458e p2, a8bfcf5f p3, 0945f03a p3, 5650c4e6 p6): stay not_headings (NOTATIONS paper
  note "Headings of another article sharing the pages are text"). Every node path stays consistent. Their blocks are
  not part of this document (like ads in a newspaper story), so the containment rule does not make this paper's open
  section ("3 可行性分析", "4 结语", 参考文献) text. Making the other article an L1 root would capture this paper's
  later sections ("4 基地建设") or force a fabricated continuation node on a text box.
- paper_chn_a197458e: the second article on p2 stays not_headings; "4 基地建设" stays L2 under this paper's L1.
- paper_eng_d054de2a p2 "II. CIVIL AVIATION › Gross Domestic Product": ACCEPTED path. The paper prints Airports / Air
  transport service / Gross Domestic Product as the same bold subheads inside section II (its term definitions); no
  unnumbered L2 style exists to lift GDP to, so look + printed outline decide.
OPEN: none.
