# researchReport learnings (v3)

## Decisions (carry over)
- L1 = report headline; when the company / stock code or "——xx深度研究/点评" line is printed in the same title block, it is
  part of the L1 (one box or a union).
- Cover summary title (投资要点：/核心观点/报告摘要...) = L2, same level as body chapters; body 1、= L2, 1.1、= L3.
- 内容目录 / 图表目录 = L2 front matter. Compliance / disclaimer titles (合规声明...) = L2 back matter.
- Cover sidebars: 相关报告 (related reports), analysts, market-data tables, charts = text (category note), even though
  the label has a list below it.
- Bold lead-in sentences starting a cover bullet (■ 上半年业绩下滑并非主营问题。...) = run-in text.
- Title union: company/stock banner printed atop the headline (通威股份（600438）, 洽洽食品（002557）, 长海股份 300196.SZ)
  and the ——report-type/subtitle line under it (——2020年年报点评, —家电行业周报20170724, 化工行业周报（dates）) are part
  of the L1 union. A bare industry banner (化工, 家电行业 行业研究/周报, 非银金融) is text when the subtitle line already
  names the industry; when nothing else names it and it sits directly on the headline bar (chn_641838eb 电子), it joins
  the union. A banner fused with the rating in one box (chn_b857707d 动力电池行业行业研究 买入（维持评级）) stays text.
- Company name only inside a mixed header box (date + type + company + rating, chn_d0eac218 #1) is not merged (box
  cannot be split); L1 = headline alone. OPEN: the owner is then missing from L1.
- Running heads that name the document (company + report type, or restate the headline) = repeats of L1
  (chn_a2f46550, chn_d0eac218). Running heads that are only the report type / series (公司点评报告, 行业周报 化工) = text.
- Periodical commentary with no headline (chn_1c78bccd 中國市場及A股點評): banner = L1; column rubric 大市導航 = L2
  section; its pieces L3. OPEN: whether the second piece (有色金屬) still belongs to the 大市導航 column.
- Morning brief (chn_2161890a 万联晨会): brief name L1; cover overview 概览 L2 with its boxes (incl. market-table boxes) L3;
  body banners L2 with stories L3.
- Product sheet (chn_5463fcf3): L1 = product series name 信达卓越推 ∪ industry 家电行业 (no headline); sidebar
  产品提示： box title L2; the own-line weekly-view line over the numbered items L2.
- Cover summary without a title: own-line bullet headings with their paragraphs are L2 (chn_25d1b14f).
- Own-line label naming the list below it (本周行业主要动态：, 受益标的组合：, ● 行业要闻：) = heading one level below its
  section; run-in sibling labels stay text.
- Table title bars (公司主要财务信息, 数据预测与估值：, 附表 captions) = text. Analyst name under 分析师声明 = text.
  Office address boxes and sales-contact table titles = text. Back-matter rating sub-blocks (股票投资评级：) = L3.
- Key-point bullets ending in a colon over a 1) 2) list, styled like whole-paragraph key-point bullets (chn_e960c7b3)
  = lead-ins, text. OPEN: they pass containment + substance; kept text as lead-ins.

## English (Goldman Sachs) families
- Banner title: covered market/series kicker box + headline box (Singapore + Equities Reform...) = L1 union. Running head
  repeating that kicker on every page (Singapore) = repeat of L1.
- Disclosure Appendix = L2 back matter; Reg AC / GS Factor Profile / ... = L3; parts under Regulatory disclosures = L4.
- Top of Mind (periodical): L1 masthead; L2 issue theme; L3 WHAT'S INSIDE box, page-banner articles (Macro news and views,
  editor lead article with the same words as the theme, Interview with X); L4 region boxes; L5 the two list labels
  (v3: repeated own-line labels with a block are headings); L4 interview questions "Allison Nathan: ...?" (own box,
  separate answer block; user decision). Running head "Top of Mind" = repeat of L1. Text: deck under Macro news and
  views, chart titles in region boxes, INTERVIEWS WITH: + names, contents entries, pull quotes, editor byline, bold
  run-in questions in an explainer page (Going public: the basics).
- Top of Mind special issue (eng_b3735ab5): L1 = masthead + issue title box; crossword and quotes page banners L2 with
  Across:/Down: L3 (labels with their clue lists); theme divider panel = badge "1" ∪ its first line, L2; the page
  banners inside that theme (pgs 5-17) L3. Exhibit title bars (Short history..., Which US banks...?) = chart/table
  titles, text. OPEN: divider panel has no real title (its lines form one sentence); union of badge + first line.
- Kicker/series line + headline (US Economics Analyst + 10 Questions..., GS SUSTAIN: ESG of the Future + headline,
  Oil + Squaring...) = L1 (one box or a union). Thematic covers (Carbonomics, ADAPTATION): big title ∪ subtitle box.
  Running heads that repeat the series/kicker/report name (US Economics Analyst, Carbonomics, Strategy Espresso, Oil,
  GS SUSTAIN: APAC ESG Regulation) = repeats of L1; a running series tag that is not in the title (GS SUSTAIN on the
  Adaptation report) = text. Headline restated at the top of the body (eng_af983a3b #20, eng_bfd076b2 #15,
  eng_d3abe75a #30) = repeat of L1 (v2 had it as a node or text).
- Numbered question headings on their own line with answer paragraphs (1. Will Congress...?, Q1: ...) = L2 sections;
  the same questions as bold run-ins in cover bullets = text.
- All Authors / AUTHORS list titles and analyst-name boxes = text. Exhibit panels inside a captioned exhibit
  (eng_4872e373 Exhibit 1 three P's / five metrics; eng_df933ffb Exhibit 14 De-carbonization technologies) = figure
  content, text; an uncaptioned full-page infographic with its own page title (eng_d3339752) = L2 with column titles L3.
- eng_df933ffb #150 (smaller bold subhead on a later page) kept L3 under #89. OPEN: may be a sibling L2 in smaller print.

## OPEN summary
- chn_1c78bccd: does 有色金屬 still belong to the 大市導航 column (made L3 under it)?
- chn_d0eac218: owner name only in a mixed header box; L1 = headline alone.
- chn_e960c7b3: colon-ended key-point bullets over 1) 2) lists kept as text lead-ins.
- chn_b857707d: industry banner fused with rating kept text; L1 = headline alone.
- Top of Mind: editor lead-article banner with the same words as the issue theme kept as an L3 node, not a repeat.
- eng_b3735ab5: theme divider panel = badge ∪ first line as L2 (no printed title).
- eng_df933ffb #150 level (resolved round 2: L3 kept).

## Breadcrumb review round 2
Checked all 37 docs (402 breadcrumb lines read, 3 automatic flags) and all 94 unseen pages (review/unseen_pages.txt)
with --tree views. No missed headings, wrong boxes or wrong parents found on the unseen pages (they are mostly
exhibit, TOC, interview and disclosure pages already consistent with the decisions above).

Fixes by class:
- level confirmed by content (OPEN resolved): 1 - eng_df933ffb #150 stays L3 under #89; its text narrows the
  cost-curve update that #89 announces (levels rule 3). Only its `why` changed.
- No level/parent, containment, title-union, repeat/continuation or description fixes were needed.

Flags accepted:
- ACCEPTED researchReport_eng_1588f970 #79 DUP: the editor lead-article page banner "All about bank(panic)s" prints the
  issue theme words again; it opens one article (not the whole issue), so it is a narrower L3 node, not a repeat.
- ACCEPTED researchReport_eng_5b6f3871 #85 DUP: same Top of Mind pattern (lead-article banner = theme words).
- ACCEPTED researchReport_eng_aaabf80f #84 DUP: same Top of Mind pattern (lead-article banner = theme words).

Checked and kept (no flag):
- chn_25d1b14f cover: 保险：/券商： stay L2 siblings of ● 周观点 (all printed with the same red ● bullet style;
  look decides on the cover, while the body nests them under 1、一周观点 by numbering).
- chn_4064c117 / chn_dd0407a2 / chn_d0eac218: captioned appendix tables and figures (附表：盈利预测, 财务预测表,
  附表：财务预测与估值) at the end fall inside the last section; they are exhibits, not containment failures.
  chn_a2f46550 附表：财务报表预测与比率分析 is a styled section title (same red style and rule as 分析师申明), L2.
- chn_a988638a 一、/二、/三、 are bold run-in lead sentences of paragraphs, text.

OPEN (round 2):
- chn_1fb28f59: the body (pp3-6) starts with no printed heading (only bold run-ins 事件：, 硅料出货高增…), so it falls
  into the subtree of front-matter 图表目录 (#42, p2). Kept 图表目录 as an L2 contents title (front-matter rule);
  containment fails for its subtree with no printed heading to close it.
- eng_b3735ab5: divider union "1 Markets worried first about growth" only names the first line of the theme
  sentence while its subtree (pgs 5-17) also covers rates, bank panics and credit (carried over).

## Breadcrumb review round 3
Independent review of all 37 docs: every breadcrumb line read (402 lines before fixes), NOTATIONS "Every path is a
breadcrumb" applied, all 94 unseen pages checked (listing of every non-body element with its tree status; page images
for chn_1fb28f59, chn_4064c117, chn_dd0407a2, chn_5463fcf3, chn_1c78bccd, chn_25d1b14f, chn_a2f46550, chn_a988638a,
chn_e960c7b3, chn_d0eac218, chn_b857707d, eng_1588f970, eng_b3735ab5). check.py: no ERROR; breadcrumbs --flags-only:
no flags left in the category.

Fixes by class:
- title union with owner from a mixed box (new NOTATIONS rule): 2
  - chn_d0eac218: L1 = header box #1 (heading part only: 宁德时代（300750）2019年年报点评) ∪ headline #3; node id now #1.
  - chn_b857707d: L1 = banner box #4 fused with the rating (heading part only: 动力电池行业行业研究) ∪ headline #10.
- periodical issue title union (Top of Mind DUP banners): 3 - eng_1588f970, eng_5b6f3871, eng_aaabf80f: L1 = masthead ∪
  issue theme box (as in eng_b3735ab5); the theme L2 node is gone, all lower levels move up one. The editor lead-article
  banner (same words as the theme) stays a node, now L2: a repeat of L1 would leave its page inside Macro news and
  views. The three DUP flags are gone. Rule: Top of Mind L1 = masthead ∪ issue title; articles L2, region boxes /
  interview questions L3, list labels L4.
- divider panel union: 1 - eng_b3735ab5 #114 = badge "1" ∪ every line of the panel sentence (growth, sticky inflation,
  higher-for-longer, bank panics, credit, CRE), so the node describes its whole subtree (pgs 5-17).
- containment, heading made text: 2 - chn_1fb28f59 图表目录 #42 (the unheaded body pp3-6 would fall into it) and
  投资要点 #8 (the figure list and the body would then fall into it); the tree is L1 only. Rule: when a body prints no
  heading at all, no earlier cover/front-matter title may hold it.
- containment, title made heading to close a section: 4 - appendix / table-section titles after the last body section:
  chn_4064c117 附表：盈利预测 #38 L2 (was under · 风险提示), chn_d0eac218 附表：财务预测与估值 #135 L2 (was under
  3 风险提示), chn_dd0407a2 数据预测与估值： #36 L2 (forecast table + p3 statements were under 风险提示),
  chn_5463fcf3 机构销售联系人 #30 L2 (sales table was under 研究团队简介). This replaces the round-1 "table title bars /
  附表 captions / sales-contact table titles = text" decision where the title is the only thing that can close the
  previous section; table titles inside a section stay text.

Resolved OPEN:
- chn_1c78bccd 有色金屬 stays L3 under 大市導航: the rubric names the market-guide column, the sector piece is a
  market pick within it and nothing closes the column before 免責聲明; path is consistent.
- chn_d0eac218 / chn_b857707d owner missing: fixed (title union above).
- Top of Mind DUP banners, eng_b3735ab5 divider, chn_1fb28f59 body under 图表目录: fixed above.
- eng_df933ffb #150: unchanged (round 2).

Checked and kept:
- Contact/colophon blocks at the end (chn_b857707d 上海/北京/深圳 office addresses, chn_a2f46550 东方证券研究所,
  chn_e960c7b3 国元证券研究所) stay text inside the last disclaimer section: NOTATIONS lists contact boxes and colophons
  as text.
- Cover sidebars (analysts, market data, related reports) under cover summary titles: category convention, kept.

OPEN (round 3):
- chn_25d1b14f: the cover's last bullet "● 风险提示：…" is a run-in (text), so it sits inside ● 受益标的组合； and
  chn_5463fcf3 "> 风险提示：…" / "本期【卓越推】暂不推荐" (p2) sit inside > 本周行业观点. Run-in rule vs containment;
  making the label above text would cascade the problem upward, so kept.
- chn_e960c7b3: colon-ended key-point bullets kept as text lead-ins (not a path issue; carried over).

## User decisions (2026-11-13)
- Owner printed only as a logo: the logo's text joins the L1 union when the layout has a box holding that text; a logo
  with no text box stays out. Goldman Sachs reports print the owner only as the logo in a header box on the cover
  (with the research-division label, which is dropped from the text). L1 = that box first (reading order) ∪ the
  former L1 boxes; text "Goldman Sachs " + old L1 text; node id moved to the logo box, children/repeats re-pointed.
  Changed (13): eng_02963ff9, 39be8c41, 4872e373, 53adef78, af983a3b (box #1), b36642e7, bcc8ee71 (box #1),
  bfd076b2, d3339752, d3abe75a, df933ffb, f653f866, f8406343 (box #2 unless noted).
  Unchanged: Top of Mind issues (eng_1588f970, 5b6f3871, aaabf80f, b3735ab5) are periodicals whose masthead is L1.
  Chinese reports: every broker logo is a header image with no text box, so no change (owner names already in the
  title stay as they are, e.g. chn_5463fcf3 信达, chn_2161890a 万联).

## Round 4 (full path review, 2026-11-13)
Every node of all 37 docs checked against every page (205 pages, --tree views). 366 node lines: 356 OK, 8 FIXED,
1 ADDED, 1 REMOVED (review/paths/researchReport.tsv). check.py: no ERROR.

Fixes:
- chn_1f9ad79f: L1 = headline #2 ∪ owner box #3 (联合资信 公用事业三部, heading part 联合资信 only) -> "机场行业研究报告 联合资信".
  The owner is printed as text on the cover (rule "The title": owner ∪ document type; owner in a box with other fields
  -> heading part only; same principle as the Goldman logo decision). Round 3 said no Chinese report prints the
  owner in a text box; this one does (also the header #1 "联合研究报告" logo, left as text since #3 names the owner in full).
- chn_25d1b14f: L1 = owner box #3 (vertical cover band 开源证券证券研究报告; heading part 开源证券) ∪ headline #16 ->
  "开源证券 3 季报符合预期，继续看好保险股投资机会 — —非银金融行业周报"; node id #16 -> #3, children re-pointed.
  Rule: user decision 2026-11-13 (owner logo/name with a text box joins L1) + "The title" (owner in a mixed box: heading
  part only). Round 3 said every Chinese broker logo has no text box; here the cover band names the broker in text.
- chn_4064c117 #11: text "- 疫情影响逐步消退…" -> "疫情影响逐步消退…": the leading "- " is the printed bullet mark, not part of the
  heading (siblings #15/#17/#19 already drop their "·"). Text-only fix.
- chn_7033db6d #52 ADDED L4 under 二、最新观点: the body prints point "1. 网上零售增速显著高于社会零售销售增速…" on its own line
  over paragraphs and figures 1-3 (pp4-5). Rule: every level of the body outline counts (own-line numbered point with
  its own block). The cover prints the same points as run-in paragraphs, so they stay L3 cover-summary items as before.
- chn_d07a67be L1 #2: owner box #10 (sidebar field "川财研究 |宏观策略部", heading part 川财研究) joins the union ->
  "全球流动性边际收紧趋势暂缓 ——欧洲央行7月议息会议点评 川财研究". The cover prints the owner only as an image logo and
  in this mixed field box. Rule: "The title" (owner in a box with other fields: heading part only) + user decision
  2026-11-13; same treatment as chn_1f9ad79f (联合资信 公用事业三部).
- chn_d0eac218 L1 #1: broker box #15 (sidebar banner "西南证券研究发展中心", heading part 西南证券) joins the union ->
  "宁德时代（300750）2019年年报点评 连续三年动力电池全球第一 西南证券". The logo #2 is an image; this banner is the only
  text box naming the broker. Rule: user decision 2026-11-13 (owner text joins L1; research-division part dropped, as
  in the Goldman logo box) + "The title" (owner in a mixed box: heading part only). #15 removed from not_headings.
- eng_5b6f3871 #118: node text carried the OCR LaTeX escape "\(25\%\)"; replaced by the printed "25%" (text matches the page).
- eng_af983a3b #88: text "7 Will Democrats…" -> "7. Will Democrats…": the page prints "7." like siblings 1.-6.; OCR dropped the period.

Rule note: where a Chinese cover prints the broker name in a text box (owner line under the headline, a cover band, a
sidebar department field or research-centre banner), its heading part joins L1 (user decision 2026-11-13 applied to
Chinese reports); covers whose broker appears only as a logo image are unchanged.

OPEN (round 4):
- chn_5463fcf3 #22 风险提示 / #23 本期【卓越推】 and chn_25d1b14f #26 风险提示: one-line run-in bullets in a
  mixed box are nodes (label part only) so that they close the section above (containment). The substance rule says a
  bullet item with nothing below it is never a heading. Kept; which rule wins?
- chn_e960c7b3: colon-ended key-point bullets kept as text lead-ins (carried over).
