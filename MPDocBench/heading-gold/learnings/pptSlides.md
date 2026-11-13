# pptSlides learnings (v3, from scratch)

## Decisions (carry over)
- Levels: L1 deck title; L2 contents title, section dividers, and slide titles that come before the first divider;
  L3 slide titles under a divider; L4 titled blocks inside a slide. A deck without dividers: slide titles L2, blocks L3.
- Lecture decks: course/book line (+ chapter line) on the cover ∪ lecture title = L1 union (owner of the lecture +
  which lecture; "7.2 X" alone fails the breadcrumb). (chn_0616b8b7)  OPEN: v2 made the course line text.
- Agenda divider (contents slide reprinted with the current entry highlighted): node = the list box, text overridden
  to the current entry only, L2. The reprinted 目录 title on the divider -> not_heading. (chn_0616b8b7)
- "定义" box label heading its own definition box = labelled block, L4 (own-line label with its own block).
- Report laid out as slide spreads (chn_23e6f440): owner+type in one box = L1; running head "<owner><report>" = repeat
  of L1; overview spread title L2, its row labels L3 and item labels with their own text L4; chapter L2, subsection L3,
  pill box titles L4 (incl. one ending in a colon, same look as its siblings and heading its stats box); a panel title
  over an icon grid L4; icon-grid labels with 2-line lists, infographic labels, logo/program names -> text.
- Deck with no dividers but slides titled "1）X / 2）Y" after a slide "Z" that introduces them: numbered slides L3 under
  Z (content beats the identical slide-title look). (chn_39aa7ee1)
- Short labels placed beside a single figure (example name next to its drawing) are figure labels -> text, even when a
  heading above says "two examples". (chn_39aa7ee1)  OPEN
- Same slide title on the next slide with nothing in between (回顾/回顾; X/X with a new block) = repeat of the first.
  Blocks on the second slide are children of the first slide's node. (chn_60917576)
- ➢ bullet labels that each head their own equation/figure block (矩形脉冲信号 + formula) are labelled blocks,
  one level below the block title (v2 made them text). (chn_60917576)
- Lecturer, institution, e-mail, logos, footers with the institution name -> text.
- A contents slide that also highlights Part 01 (and the following slides are all Part 01): contents title union
  (目录 ∪ CONTENTS) L2 plus a Part 01 divider node on the list box L2; numbered slide titles L3; the key-message
  headline under each slide title L4. (chn_746216ac)
- Two consecutive slides "1.3 X：A" / "1.3 X：B" are siblings. PEST quadrant labels = box titles one level below
  the headline (even the ones printed under their box); the diagram centre slogan -> text. Figure captions -> text.
- Research-report-style decks: publisher credit lines (出品机构/研究团队/发布时间) and the subtitle line -> text; L1 is the
  printed report title.
- The course name on a lecture cover can be tagged header; it still joins the L1 union when printed only on the cover.
- A section title slide listing its own sub-topics (三、离散傅立叶变换 + ➲ list) = divider L2; list entries -> text;
  following slide titles L3. Same slide title again after a different slide = continuation (continues -> first).
  (chn_7c522d1e)
- "DFS:" own-line label over its equations = labelled block (one level below the slide title). Diagram arrow labels
  -> text.
- An untitled slide belongs to the previous slide's title (no node). Unnumbered slide after "1、X" with the same look
  stays a sibling (chn_b0123c13 大气窗口) — content could argue for a child; OPEN.
- "Part 01." + part title on a divider = one union L2 (chn_b30fedce). A box label "趋势调研：" alone on its line over
  the preface text box = labelled block one level below 前言 (v2 had text).
- Small bar-marked panel titles inside a slide (话题&热词, 商业笔记声量, 小红书笔记) are chart/panel titles -> text, also
  when they sit over a panel of stat pills; they share the figure-caption style. (chn_b30fedce)
- A run of slides with the same title (普通信号：正弦信号 x4) = one node + repeats. An overview slide naming the
  chapter's topics (连续信号的时域描述和分析 with ➲ entries) = divider L2; its entries text. (chn_ca2c7356)
- ➲ labels with their own short list of item names (普通信号的时域描述: 正弦信号/指数信号) kept as labelled blocks;
  later slides "普通信号：正弦信号" stay siblings of those labels (reading order). OPEN (v2 same).
- Agenda divider with entries progressively darkened: the current entry is the last dark one. An agenda divider on
  the last page with nothing after it (excerpt ends) fails substance -> text/not_heading. (chn_d81f00a8) OPEN
- Own-line specific labels ending in a colon that name the list below ("井字棋规则说明：", "○方，第n+1步时的策略：")
  = labelled blocks; generic lead-in sentences ("以下规则按照优先级排列") and callouts ("建模方式并不唯一") = text;
  "如何让机器实现上述方案？" question prompt without its own answer = text. 方案3/方案4… labels with own lists = L4.
- Volume line naming this part of a series ("{个性表达赛道}篇") joins the L1 union; running heads/footers printing
  "<report title> <volume>" = repeats of L1; footer navigation tags (赛道说明/赛道热度) -> text. "PART 01." + topic
  title = union one level below the part (一、个性化美妆 contents slide = part divider). KPI panel banner -> text.
  (chn_eba02de5)
- Contents slide with an arrow at one entry = contents L2 + divider node (that entry) L2. (eng_1e8e2f71)
- Teaser slide with keywords beside pictures (no own block) -> not headings. A question used as a slide title is
  the slide title. "Key points" own-line label over its a)-c) list = labelled block one level below the slide title.
  Small diagram-tree labels (Elicitors/General/Race specific) and lead-ins ("3 aspects of response:") -> text.
  (eng_24541ef2)
- Bold own-line labels each heading a bullet block ("Definition:", "Which means?", "Example, please.") = labelled
  blocks one level below the slide title (v2 had text). Column box titles over bullet columns = blocks. Figure labels
  above logo clouds (Horizontal/Vertical) and lead-in lines ending in ":" -> text. (eng_26f4ebca)
- "(cont.)" slide title right after its slide = repeat; blocks on it are children of the original slide node.
  (eng_312b78ba)
- Titles of charts/tables/data panels (QUICK STATS, COMPANY RANKING, Net debt, Operating profitability…) -> text;
  bold labels heading slide text (Company Background, STRATEGIC DIRECTION, PRODUCTION) and box titles (SWOT boxes,
  4-box grids) = L3 blocks; SWOT centre words and embedded report headers -> text; a label in a row of labels with no
  list under it (THE BODY SHOP) fails substance. (eng_312b78ba)
- First content slide titled exactly like the deck = repeat of L1. Labels over a row of figures (Purines /
  Pyrimidines + structure drawings) = figure-group labels -> text. (eng_52c9fd59)
- Divider slide printing the deck title above the section name: deck title = repeat of L1, section name = L2
  divider. (eng_5bb58cde)
- Question slide with only a picture followed by its answer slide: both slide titles L2 siblings (eng_6d3fdf72).
- Gold title box that swallowed the author name: node keeps the box, text = title only. Running footer with the deck
  title = repeat of L1. A bullet item "Skills" over a word cloud stays text (bullet style). (eng_bd47b74b)
- Build slides (same title, the canvas filled in step by step) = repeats of the first. A divider-like title on the
  last page with nothing after it -> text (substance), like the trailing agenda divider. Canvas/diagram labels -> text.
  (eng_82a06813)
- ◉ first-level bullets that are short topic labels each heading their own sub-bullets (Sensory input…, CNS, PNS,
  Sensory/Motor division) = L3 blocks; the author's "Motor (efferent) division (continued)" confirms they are topic
  heads. Its resumption after other slides = continuation (continues -> first). Full-sentence bullets stay text.
  Slide title back after a different slide = continuation. (eng_a04b6aef) OPEN (v2 made them text)
- Cover with series line + title + "Supplementary Report": all three = L1 union (owner/series + title + document
  type). First slide after a divider titled exactly like the divider = repeat of the divider. Smaller "ALSO FROM
  2015" companion slides = one level below the chart slide they extend (OPEN: could be siblings). Stat panel labels
  -> text. (eng_f280f799)

## OPEN cases
- Lecture decks: course/book line (+ chapter) on the cover joined to the lecture title as the L1 union
  (chn_0616b8b7, chn_d81f00a8, chn_60917576, chn_ca2c7356, chn_7c522d1e); v2 made it text.
- Agenda-divider node text is only the highlighted entry of a list box that prints all entries.
- Trailing divider/agenda slide on the last page with nothing after it made text by substance (chn_d81f00a8 03,
  eng_82a06813 "The Children and Many Faces…").
- ➢/➲/◉ bullet labels heading their own equation/list blocks made headings (chn_60917576, chn_ca2c7356,
  eng_a04b6aef); full-sentence bullets and "Skills" (eng_bd47b74b) stay text. Borderline with "bullet item".
- Generic labels with their own block made headings (定义, Key points, Definition:/Which means?/Example, please.,
  趋势调研：); v2 had text.
- chn_39aa7ee1 example labels beside drawings made figure labels (text); chn_b0123c13 大气窗口 kept sibling of
  "1、红外辐射"; chn_ca2c7356 "普通信号：正弦信号" slides siblings of the label they belong under (reading order).
- eng_f280f799 "ALSO FROM 2015" companion slides one level below the chart slide (could be siblings).
- chn_b30fedce / eng_312b78ba panel and chart titles over stat pills made text (gold had them as headings).
- eng_312b78ba "Local research and adaptation of products" (subhead inside a pasted report excerpt) kept L4;
  "THE BODY SHOP" label with no list made text by substance.

## Breadcrumb review round 2
22 docs, 254 breadcrumb lines read; no unseen pptSlides pages listed in review/unseen_pages.txt.
- ACCEPTED pptSlides_chn_746216ac #64 SEQ: the deck prints two consecutive slides both numbered "1.3 科技产业生态体系：…"
  (传统 / 新型 布局方式); siblings by the round-1 decision.
- Fix class "label restating the slide title" (1): chn_7c522d1e #48 "1、DFT 的引出" (the pasted figure's own label) under
  slide "DFT的引出" is not narrower -> repeat of #46.
- Fix class "lead-in sentence as slide title" (1): chn_7c522d1e #51 "能量有限、时间长度为 L 的有限长序列的 DTFT 为" runs into
  the equation, not a description -> text (considered); the slide stays under "DFT的引出".
- Fix class "continuation" (1): eng_5bb58cde #66 "Step 1: Request VM Provisioning via UI/CLI" resumes Step 1 (#37,
  printed with different wording) after two Horizon slides -> continues #37.
- Fix class "bullet glyph in node text" (5): eng_a04b6aef #10/#28/#31/#36 drop the OCR "\(\odot\)" bullet;
  chn_d81f00a8 #23 drops the printed □ ("口"). Node keeps its box; only the text is cleaned.
- Checked and kept: chn_60917576 "非周期信号的频谱分析 › 常用周期信号的傅立叶变换" (the deck prints it under that running
  slide title); eng_312b78ba "North American" panel title; chn_23e6f440 p7 lead-in #97 (沙棘 intro) falls under
  "探索滨海湿地生态修复" by reading order — OPEN (containment leak, no heading in between to close it).

## Breadcrumb review round 3
22 docs, 251 breadcrumb lines read (independent second review); no pptSlides pages in review/unseen_pages.txt.
- ACCEPTED pptSlides_chn_746216ac #64 SEQ: the deck prints two consecutive slides both numbered "1.3 科技产业生态体系：…"
  (传统 / 新型 布局方式); siblings (round-1 decision).
- Fix class "containment, box titles -> text" (chn_23e6f440 p7, 3 nodes; resolves round-2 OPEN): the free paragraph #97
  (蚂蚁森林 as 绿色基建 / eco products, the lead-in to the 沙棘 box) follows "探索滨海湿地生态修复" in reading order with
  no heading before #98 -> #95 text; then #96/#97 would sit under the COP15 title -> #91 text; #83 "截至2022年底，…：" is
  a colon lead-in over a stats column (same as #129 on p8, text) -> text. The page's content sits under L3
  "协同多元社会力量，助力生态保护与修复"; "沙棘模式…" (#98) keeps L4 (its box closes the page).
- Fix class "overview entries -> text" (chn_ca2c7356 p3, 2 nodes; resolves the 普通信号 OPEN): "普通信号的时域描述" /
  "奇异信号的描述" are ➲ entries of the (一) 时域描述 overview slide with item names only; the topic slides
  "普通信号：正弦信号" can never sit under #22 (#25 comes between) -> both text, like the p2 overview entries. The
  正弦信号 slides stay L4 under (一) 时域描述.
- Fix class "kicker as node text -> text" (eng_f280f799, 2 nodes; resolves the ALSO FROM 2015 OPEN): "ALSO FROM 2015"
  is a banner, not a description; its stat panels extend the chart slide before it and stay in that slide's subtree.
- Fix class "slide title restating its divider -> repeat" (chn_d81f00a8 #10 "井字棋" under "01 井字棋：介绍", the only
  slide of section 01): repeat of #9; "井字棋规则说明：" now L3 under #9.
- Fix class "level by numbering" (chn_b0123c13 #17 大气窗口; resolves round-1 OPEN): unnumbered, same look as
  "1、红外辐射", content is a property of infrared radiation -> L3 under 1、红外辐射.
- Fix class "panel title not narrower / chart title" (eng_312b78ba p8, 2 nodes): "North American" only restates
  "Overview : North America" -> text; then the ranking panel would fall under the pie-chart title "NA Sales as a % of
  Total BPC" (a chart title, round-1 rule) -> text too; both panels sit under "Key Financial Figures NA-Q1-12".
- Resolved (kept) agenda-divider node texts: node = the reprinted list box, text = the highlighted entry only; this is
  the NOTATIONS rule for a heading printed in a box that also holds other fields (node uses the box, text = heading
  part). chn_0616b8b7 #17, chn_d81f00a8 #9/#21, eng_1e8e2f71 #5, chn_746216ac #12 all read as consistent breadcrumbs.
- Resolved (kept) lecture-deck L1 unions with the course/chapter line (whose document + what document).
- Checked and kept: eng_26f4ebca p12 column titles (Financial Services Industry / Vertical APIs / FinTech Startups) are
  the three parts of "Vertical APIs in Finance"; eng_312b78ba SWOT box titles; chn_746216ac PEST labels under the
  key-message headline; eng_a04b6aef continuations.
- OPEN: none concerning a path. Non-path round-1 OPENs (bullet labels as headings, generic labels, figure labels,
  trailing agenda dividers as text) unchanged.

## Round 4 (full path review, every node on its page)
22 docs, 172 pages rendered with the tree, 241 nodes reviewed (review/paths/pptSlides.tsv): 238 OK, 2 FIXED, 1 REMOVED.
- Fix class "label restating the slide title -> repeat" (eng_312b78ba #42): bold label "RESEARCH & DEVELOPMENT" on p4
  restates the slide title "L'ORÉAL – R&D, a Key to Success" (not narrower) -> repeat of #41; the p4 text and tables sit
  directly under the slide title; "STRATEGIC DIRECTION:" (p5) stays its L3 child. Same class as round-2 chn_7c522d1e #48.
- Fix class "node text as printed" (2): chn_d81f00a8 #54 restores the printed "○" ("○方，第n+1步时的策略：" — the O player,
  not a bullet glyph); eng_312b78ba #170 "LUXURY PRODUCTS" (OCR dropped the space).
- OPEN: does a repeated slide title (same title on the next slide, `repeats`) close the open block headings of the
  previous slide? If not, the next slide's content falls under the last block label: chn_ca2c7356 p5-p7 under
  "取虚部则为正弦信号" (#38), eng_24541ef2 p12 figures under "Key points" (#59), eng_a04b6aef p4 diagram under
  "Motor output" (#13). Trees kept.
