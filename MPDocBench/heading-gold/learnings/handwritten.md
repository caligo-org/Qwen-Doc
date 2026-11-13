# handwritten learnings (v3)

## Decisions
- 01fba5b3: separate sheets with different own titles (itinerary vs guide) -> each its own L1. A sheet title restated
  at the top of later sheets of the same guide is a repeat of that L1. A topic restated at the top of the next sheet
  while its numbered list just continues (注意事项) is a REPEAT in v3 (nothing came in between), not a continuation.
- Labels drawn inside a figure element (DAY1..DAY5 in an itinerary drawing) have no box: no node.
- Run-in labels (首站：/机场：/火车站：) are text, even the one that wraps its content to the next line.
- 048c9a89: a lesson-plan doc with a second lesson titled on its own sheet (第二课时): the plan title (教案) is L1 and the
  second lesson title is L2 under it (breadcrumb: plan › lesson 2), sibling of lesson 1's 一/二/阶梯训练 parts.
  Lesson 1 has no printed 第一课时 heading, so its parts sit directly under L1.
- Worked problem statements (2. 如图…, 6. 在锐角…) are exam questions: text, even when gold had them as titles.
  Own-line exercise labels (差的最大值经典习题(1)) and a numbered topic title ((5) 架桥选址问题) over a problem + analysis
  block are headings.
- OPEN 048c9a89: lesson 2 runs (3) and (4) as problem statements (text) and then (5) 架桥选址问题 as a titled topic;
  (5) sits as a sibling of the 经典习题 labels (L3) because (3)/(4) are not headings.
- 1ae487be (printed drill workbook filled by hand, no overall title): round banners (第二轮…/第三轮…) L1; day markers
  (第N天) L2 everywhere (same look); 类型/考法 titles L3; unnumbered sub-type titles L4. Round 1's banner is not
  printed, so its days are L2 under root (role skip). An appended notebook sheet with its own title is another L1.
- OPEN 1ae487be: types run across day boundaries; 盐与盐的反应 resumes type 四 under 第5天 and sits L4 directly under
  the L2 day (level skip, type not printed there). Alternative would be days as text.
- Margin notes in another ink (除铁锈) and notebook "Date" headers are text.
- 375d58b7: a printed form where each page is one table element: no nodes (headings inside a table have no box).
- 50d71ae2 (course notes): 第八章 L1; section 7.3 L2 under root (chapter 7 not printed); own-line Chinese-numbered
  titles (四. 常用傅氏变换对, 一. 概念) L3 in both chapters (same look; chapter 8 prints no 8.x level).
- Numbered property series: if any member is an own-line title (2. 位移性质：), the whole series is headings, including
  members whose first formula shares the title line (1. 线性性质: F[…], with examples below). A series whose members
  are all run-in statements (1. 定义：… 5. 性质：…) is text. Numbered formula-only cases (一. ①当 f(t)=δ(t)) are text.
  Node text keeps the whole box (formula included).
- A starred own-line topic with its formula + examples (拉普拉斯逆变换：) is a heading at the property level (OPEN:
  sits between properties 1 and 2 as a sibling; it could be read as a sub-topic of 1. 线性性质).
- Watermarks (@…), side notes in brackets, margin notes in another ink, ★ exercise statements are text.
- 65cdc9d3: sheets that each hold printed exam problems with handwritten solutions, no title: no nodes. Problem
  statements and their (Ⅰ)/(Ⅱ) sub-questions are exam questions (text), even when one fills a sheet. Brand/date
  running headers are text.
- Lab reports (6a1cedac): the 实验项目 line (field label + subject, one box) is the L1 title; the form's 实验项目 field
  restated at the top of a later sheet is a repeat of L1; an empty 实验项目 field is text. 一、二… L2; own-line numbered
  titles (1. 内容, 2. 结果, 3./4. 思考题) L3; own-line (1)/(2) L4. The 实验内容：一、二、三 list under the title is text.
- Run-in lead-ins that start a prose paragraph or a one-line statement (1. 原理：…, 2. 操作：…, 3. 结果：…) are text even
  when a sibling (4. 思考题) is an own-line title. Contrast 50d71ae2, where the run-in member carries only a formula
  and its examples sit in separate blocks below.
- OPEN: 思考题 questions with written answers: kept as text (answers are mostly one or two lines, about as long as
  the question; substance fails on average). Long-answer Q&A would qualify as headings by the interview/FAQ rule.
- 71864fb7: math essay: title L1, Ⅰ/Ⅱ/Ⅲ parts L2; theorem labels set before their formula (Newton公式, Waring公式) are
  run-in labels like "Definition 2.1": text.
- 79c34515: same as 375d58b7 (each page one table element): no nodes.
- 92741860: essay with photos out of order (title on photo 3, continuation pages before it): the essay title is the only node, L1 under root, though it comes after text in reading order.
- 92bf52bc (lab report): 实验名称 line L1; 一…七 L2; own-line 4. 离心技术 with a large block L3 while its siblings
  1.-3. are run-in principle statements (text). Bulleted run-in labels (·注意事项：, 离心机的类型：) are text.
- 94af97ba (answered exam paper, no title): question groups 一、二、三 L1. Question 21 (计算题：with four parts) is an
  individual question: text (v3 exam rule; v2 had it L2). Sub-questions are text.
- 9ad83bc1 (essay): essay title L1; centred part subtitles L2 (gold had them as extra L1). The closing paragraph stays under the last part (no own heading).
- Study notes without title (a6f48a98): top Chinese-numbered topics L1; a restarted numbering (二. 价格 after 四. 纸币)
  starts a new L1, not a child. Curve titles 1.-7. on the later sheet sit under 二. 价格 (L2).
- Numbered series, decided per series (all or none, so containment holds): a series is headings when most members are
  own-line titles with blocks (纸币: 1. 含义 / 2. 纸币的职能：statement / 4. 发行规律 -> all L2); text when most are run-in
  statements ((1) 物品交换的4个阶段 own-line but (2)-(4) run-in -> all text). OPEN: the per-series majority call.
- d1ec88bb: several printed worksheets, each with its own title -> each L1; exercises, student name and 解题思路 rule lines are text.
- eac2d8c2: chapter L1, 3.x L2, (n) L3; own-line principle labels with their own block (最低能量原理（焓变判据）：, Gibbs 函数(变)判据：) L3 one level below their section. Lead-ins before a formula (对于一般的化学反应：, 任意状态下：) and 注： callouts are text.
- eac3f5a2 (annotated textbook lesson): the lesson title (26 西门豹治邺) is the only node; annotation callouts ([中心思想], 近义词/反义词), margin notes and watermarks are text.
- edc60213 (untitled study notes, photos out of order: notebook pages 5, ?, 3): bracketed section labels ([方法突破])
  L1; numbered topics L2. Topics whose section label is not on the photographed pages (4./2./3. 集合问题与…思想) are L2
  under root (role skip). Callouts 启示：, [例] are text.
- OPEN edc60213: reading order follows the photos, so topic 4 precedes topics 2-3 and [方法突破] (notebook page 3)
  comes last.

## Breadcrumb review round 2
Docs 18 (3 with no nodes: 375d58b7, 79c34515, 65cdc9d3), 131 breadcrumb paths read (124 after the fix); unseen pages for handwritten: none listed.
- FIX (chart titles / containment, 1 doc, 7 nodes): a6f48a98 curve titles 1.-7. (需求曲线 … 平衡价格) are titles of
  drawn graphs = chart titles -> text (not_headings). As headings, 7. 平衡价格 also held the last sheet's income /
  consumption lines (no later heading closes it) and failed containment. Everything now sits under 二. 价格.
- Clarified series rule (6a1cedac / 92bf52bc vs a6f48a98 are consistent): an own-line numbered member is a heading
  unless later run-in siblings would fall into its subtree. So a LAST own-line member after run-in siblings stays a
  heading (6a1cedac 4. 思考题, 92bf52bc 4. 离心技术); a FIRST own-line member followed by run-in siblings is text
  (a6f48a98 (1) 物品交换的4个阶段); a run-in member between own-line ones is a heading so that its content does not fall
  under the previous member (a6f48a98 2. 纸币的职能).
- ACCEPTED handwritten_chn_01fba5b3 #7 ROOTS: itinerary sheet and guide sheets carry two different own titles.
- ACCEPTED handwritten_chn_1ae487be #19 ROOTS: round 1 banner not printed, so days 1-5 are L2 under root; round 2/3
  banners and the appended notebook sheet title are L1.
- ACCEPTED handwritten_chn_50d71ae2 #85 ROOTS: chapter 7 not printed (7.3 L2 under root) + 第八章 L1.
- ACCEPTED handwritten_chn_50d71ae2 #65 NUM: the page prints "3. 推理："; the box OCR reads 三.
- ACCEPTED handwritten_chn_94af97ba #63 ROOTS: untitled answered exam; question groups 一/二/三 are the top level.
- ACCEPTED handwritten_chn_a6f48a98 #9 ROOTS: untitled study notes; numbered topics are the top level.
- ACCEPTED handwritten_chn_a6f48a98 #19 SEQ: the page prints "三." (OCR box text reads 二.).
- ACCEPTED handwritten_chn_a6f48a98 #44 SEQ: follows from #19 (三 -> 四 as printed).
- ACCEPTED handwritten_chn_a6f48a98 #75 SEQ: the notes restart their numbering with 二. 价格 (a new unit; no 一 on the photographed pages).
- ACCEPTED handwritten_chn_a6f48a98 #55 SEQ: the notes print 4. 纸币的发行规律 right after 2.; there is no 3.
- ACCEPTED handwritten_chn_d1ec88bb #15 ROOTS: three worksheets, each with its own title.
- ACCEPTED handwritten_chn_edc60213 #22 ROOTS + SEQ: photos are out of notebook order (pages 5, ?, 3); [方法突破]'s
  topics 2-4 come before it, and topic 4 comes before 2.
- OPEN (still): 1ae487be 盐与盐的反应 L4 directly under the 第5天 day (type 四 not restated); days cut across types.
- OPEN: edc60213 p3 top lines (empty-set facts) end the previous topic 3. 集合问题与等价转化思想 by photo order only.
- OPEN: 048c9a89 lesson 2 (3)/(4) model problems stay text; (5) 架桥选址问题 L3 beside the 经典习题 labels.

## Breadcrumb review round 3
Docs 18 (3 with no nodes), 125 breadcrumb paths read (115 after the fixes); unseen pages for handwritten: none listed.
Every page of every doc with nodes viewed with --tree.
- FIX (pacing markers cut across the content outline, 1 doc, 10 nodes): 1ae487be day markers 第1天…第10天 -> text
  (not_headings). They are schedule marks, not descriptions of content, and they cut across the type/考法 titles
  (第2天 starts inside 类型一 at item 9, 第5天 inside 类型四, 第8天/第9天 inside 考法二). As headings they failed
  containment (类型一 did not contain items 9-11) and broke the breadcrumb (第5天 › 盐与盐的反应 skipped its type).
  New outline: 类型一…五 L2 under root (round-1 banner not printed, role skip), sub-types L3; 盐与盐的反应 is now L3
  under 类型四 (resolves the round-1/2 OPEN); round banners L1 › 考法一/二 L2; round 3 L1 with no sub-heading;
  appended sheet L1 › 1./二./三. L2. Rule: a marker of when/where to study (day, session, week) that cuts across
  the topic titles is text; if it coincides with the topics it may stay a heading.
- FIX (breadcrumb step not narrower, 1 doc, 1 node): 50d71ae2 #122 拉普拉斯逆变换 L4 sibling of the properties ->
  L5 under 1. 线性性质 (inserted, starred, inside property 1; its 例3 splits an inverse transform by linearity).
  "性质 › 拉普拉斯逆变换" was not a narrower step (it is not a property); L3 would capture 2. 位移性质 (containment).
  Resolves the round-1 OPEN.
- RESOLVED 048c9a89 lesson 2: paths are consistent. Each photo is one sheet: lesson-2 intro + model problems (3)/(4)
  (text), 经典习题(1)-(3) each on its own sheet, (5) 架桥选址问题 alone on the last sheet; every L3 subtree holds only
  its own sheet. (5) is the last own-line member after text siblings (series rule) -> heading, L3 under lesson 2.
- RESOLVED edc60213: the topic paths are consistent. The two p3 top lines (subset/empty-set facts) fall under
  3. 集合问题与等价转化思想 only because the photos are out of notebook order; no tree fixes this (making topic 3
  text moves its own example and those lines under topic 2), so the tree stays. Same reasoning, kept from rounds
  1-2: 9ad83bc1 closing paragraph under the last part; a6f48a98 headingless 外汇/汇率 lines (p5) under 4. 纸币的发行规律
  and the last-sheet income lines (p9) under 二. 价格: the notes print no heading for them, and demoting the open
  heading would put them under an even less fitting one.
- ACCEPTED handwritten_chn_1ae487be #26 ROOTS: untitled drill workbook; round-1 banner not printed, so its types
  类型一…五 are L2 under root; round 2/3 banners and the appended notebook sheet title are L1.
- ACCEPTED handwritten_chn_50d71ae2 #85 ROOTS, #65 NUM: as round 2 (chapter 7 not printed; page prints "3. 推理：").
- ACCEPTED handwritten_chn_01fba5b3 #7 ROOTS, 94af97ba #63 ROOTS, a6f48a98 #9 ROOTS / #19 #44 #55 #75 SEQ,
  d1ec88bb #15 ROOTS, edc60213 #22 ROOTS + SEQ: as round 2 (re-checked on the pages).
- OPEN: none that concerns a path.

## Round 4 (full path review)
Docs 18 (3 with no nodes: 375d58b7, 79c34515, 65cdc9d3, each re-checked on every page: one table element per
page or exam problems only). 116 nodes, every page of every doc viewed with --tree. 111 OK, 5 FIXED, 0 ADDED,
0 REMOVED. All paths read as breadcrumbs; no missing heading found. The fixes are node texts that copied an OCR
error instead of what the page prints:
- FIX 1ae487be #238: "1. 化合反应" -> "一. 化合反应" (page prints 一., like its siblings 二./三.).
- FIX 6a1cedac #40: "三、草兰染色" -> "三、革兰染色".
- FIX a6f48a98 #54: "2. 低币的职能：低币…" -> "2. 纸币的职能：纸币…".
- FIX eac2d8c2 #1: "第三章 化学反应的自发性." -> "第3章 化学反应的自发性." (the page prints the Arabic 3).
- FIX eac2d8c2 #61: "(2) 反应商}" -> "(2) 反应商" (stray brace from OCR).
- Kept as printed by the box: d1ec88bb titles with the dash read as 一一 / -一 (the glyphs are the same strokes);
  eac3f5a2 #2 keeps its pinyin ruby (bào yè), which sits inside the title box.
- Earlier decisions re-checked on the pages and kept: 50d71ae2 7.3 cases 一-三 text / 四-五 headings (series rule:
  last own-line members); a6f48a98 run-in 2. 纸币的职能 a heading; 92bf52bc / 6a1cedac 思考题 Q&A text.

## Round 5 (breaks)
Docs 18, every page rendered with `show.py --tree` and viewed. 9 breaks in 6 docs, 1 node restored; check.py: no ERROR.
- BREAK 048c9a89 before #4 closes #2: the red teaching-goals box (第一课时教学目标) is about the lesson, not 一：由来.
- BREAK 1ae487be before #188 closes #148: items 20-22 (equations from conversion diagrams) are not about 考法二
  根据原理、用途书写; no 考法 title is printed for them, so they sit under the round-2 banner.
- BREAK 9ad83bc1 before #15 closes #10: the essay's closing paragraph (round 1-3 kept it under the last part
  because no heading closed it; the break resolves that).
- RESTORED a6f48a98 #21 (1) 物品交换的4个阶段 (L2 under 三. 货币的产生的过程): own-line title with its own block; it
  was text only because the run-in siblings (2)-(4) would fall under it (series rule, containment). BREAK before
  #24 closes it. The round-2 series rule "a FIRST own-line member followed by run-in siblings is text" is
  superseded by breaks; a LAST own-line member after run-in siblings was already a heading.
- BREAK a6f48a98 before #67 closes #44 (外汇/汇率 lines, not about 纸币) and before #127 closes #75 (income /
  consumption lines, not about 价格). Both groups now sit at root; round 3 kept them under the open heading only
  because no heading closed them.
- BREAK eac2d8c2 before #49 closes #44: ΔG=ΔH-TΔS and T转 resume 3.4's main text after the Gibbs 判据 label block.
- BREAK edc60213 before #21 closes #11 and before #34 closes #30: photo-order spill-over (an element-property
  example end; subset / empty-set facts) under topics 4 and 3. Resolves the round-2 OPEN on the p3 top lines.
- Kept without break: 50d71ae2 ★ exercises under 三. 推理 (they apply 推理/位移); 92bf52bc 离心机的类型 under
  4. 离心技术; eac2d8c2 #14 焓变只是…之一 under the 焓变判据 label (it qualifies that criterion); 1ae487be day
  markers and a6f48a98 curve titles stay text (pacing marks / chart titles, not containment).
- OPEN 71864fb7 p5: the σ_2i(cos jπ/n) identity after III. 组合反演公式 may apply the Waring formula rather than the
  inversion formula; not decidable from the page, no break.
- OPEN a6f48a98 #24: NOTATIONS' exception bullet (book_eng_dc66edfa "Star Topology") says a run-in sibling's
  content sits under the previous own-line label (no break); the Breaks section and the round-5 brief say a break
  closes it. I followed the brief (break added); confirm which applies.
