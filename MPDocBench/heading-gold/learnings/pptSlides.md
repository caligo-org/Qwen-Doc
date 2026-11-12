# pptSlides learnings

## Decisions (carry over)
- Levels: L1 deck title; L2 contents title / section dividers / slides before the first divider; L3 slide titles
  under a divider; L4 titled blocks inside a slide. In a deck with NO divider slides, slide titles are L2 and block
  titles L3 (no level skip; parents are the same either way).
- Cover: lecturer, email, university/company logo, date, "出品机构/研究团队" lines, slogans are text. A course/book
  (+ chapter) line printed above a larger lecture title (《书名》/第7章 over "7.2 xxx"; 信号分析与处理 over 第二章 ...)
  is a series line -> text; the lecture title is L1 (chn_0616b8b7, chn_d81f00a8, chn_60917576, chn_ca2c7356,
  chn_7c522d1e). A volume line that names this part of a series ("{个性表达赛道}篇") merges into the title (chn_eba02de5).
- Agenda divider slides (contents slide repeated with the current entry highlighted, others greyed, or marked by an
  arrow): node = the list text_block id, text = the current entry only (number + title). When entries are
  progressively darkened, the current one is the last dark entry. The repeated "目录" box on a divider slide is text.
  The first contents slide keeps its title as the L2 contents node; when it also highlights entry 01, it gets the
  divider node as well (chn_746216ac, eng_1e8e2f71).
- "Part 01." + part title on a divider slide: one node with merge (chn_b30fedce, chn_eba02de5). A contents title with
  its English translation (目录 + CONTENTS) merges.
- A contents-style slide that lists one part's own topics ("一、个性化美妆" with 01-04 entries; "三、离散傅立叶变换"
  with sub-topics; "连续信号的时域描述和分析") is the divider for that part; its entries are TOC text.
- A divider that prints the deck title above the section name: the deck-title repeat is text, the section name is
  the divider (eng_5bb58cde). A first content slide titled exactly like the deck is a repeated title -> text
  (eng_52c9fd59).
- Repeated slide titles (same text on consecutive slides, "(cont.)", builds of the same template) are continuation
  nodes at the slide-title level with `continues` -> first occurrence.
- Two consecutive slides with the same numbered prefix but different titles (1.3 ...：传统 / 1.3 ...：新型) are
  siblings, not parent/child.
- Slide headline under a numbered slide title (green-bar key-message line) is one level below the slide title.
- Block titles inside a slide that head their own content are headings: bold/banner/pill/arrow-marked block labels,
  box titles in multi-box slides (SWOT boxes, column boxes), PEST quadrant labels, 方案3/方案4 labels.
- Text, not headings: chart/table/panel titles (including KPI-panel banners and titles inside embedded report
  images), figure captions and figure-group labels, infographic icon labels, diagram centre labels, run-in labels with
  a colon (趋势调研：, Definition:, Which means?), callout labels (定义 box label, Key points), question-prompt lines,
  first-level bullet items in the standard bullet style even when short (eng_a04b6aef ◉ items; chn_60917576 ➢ items),
  teaser-slide keyword labels next to pictures (eng_24541ef2 p2).
- Gold titles that swallowed a neighbouring box (author name, figure label) keep the node; text is the heading only.

## Open questions
- Series line (book/course + chapter) treated as text, not merged into the L1 lecture title (5 USTC/ZJU decks).
- Agenda-divider node text is only the highlighted part of a list box that holds all entries.
- chn_23e6f440 (a report laid out as a slide spread): overview panel categories L3 and their item labels L4 were
  kept as headings; the "绿色低碳倡导场景" box icon labels were made text. Borderline either way.
- chn_ca2c7356: "普通信号：正弦信号" slides are semantically under the block title "普通信号的时域描述", but reading
  order forces them to be siblings of the block titles under "(一) 时域描述".
- eng_a04b6aef: 8 first-level bullet items (Sensory input…, Motor output, CNS, PNS, divisions, one "(continued)")
  were gold headings; made text by the same-look rule. The "(continued)" marker suggests the author saw them as
  topic heads — reviewer may prefer them as L3 headings.
- eng_f280f799: "ALSO FROM 2015" companion stat slides kept as L4 under the preceding chart slide (smaller title);
  could be L3 siblings. The p4 "METHODOLOGY" slide title kept as a child of the "METHODOLOGY" divider.
- chn_d81f00a8: "口 人是怎么下棋的" kept as a block title (bold boxed bullet, distinct from the rule list);
  "如何让机器实现上述方案？" made text.
- chn_b30fedce: "趋势调研：" (box label with colon) made text.
- chn_eba02de5: "美容护肤品类" KPI banner made text.

## Label rule pass
- Own-line specific labels inside a slide that name the list under them become L4 block titles even in body style:
  "井字棋规则说明：" (p4, p7) and "○方，第n+1步时的策略：" (p8) in chn_d81f00a8. This supersedes "run-in labels
  with a colon are text" when the label is on its own line and specific.
- Lead-in sentences ending in a colon are text, even when styled as a block title: chn_23e6f440 "截至2022年底，
  “蚂蚁森林”助力各地生态建设：" removed; "3 aspects of response:", "Startups solving 3 vertical-specific
  challenges:" stay text. Generic "Definition:" stays text.
- Icon-grid item labels keep the look of their siblings: chn_23e6f440 "“绿色能量行动”合作伙伴：" stays text like
  绿色出行/减纸减塑.
- Kind-A repeats that are real slide/section titles are kept: "绿色低碳" (overview category vs section), "METHODOLOGY"
  (divider vs slide title), "ALSO FROM 2015" (companion slide titles), slide title "WE ARE REVIEWING CHEMISTRY
  BECAUSE:".
- eng_312b78ba "STRATEGIC DIRECTION(:)" on p5 and p6 kept as block titles (same style as PRODUCTION / COMPETITIVE
  POSITIONING); it repeats across sections, so it is close to a generic label (open).
