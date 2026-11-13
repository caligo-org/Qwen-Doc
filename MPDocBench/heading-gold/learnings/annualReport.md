# annualReport learnings (v3)

Decisions that recur (applied in every tree; OPEN = not settled by the test).

- Title: L1 = owner ∪ document type (金科地产集团股份有限公司 ∪ 2019年年度报告), one node even when both sit in one box.
  Logo, slogan, date line, cninfo footer are text (considered when heading-like).
- Running header naming the document (…年度报告全文) = repeat of L1 on every page after the cover. The one printed on
  the cover page itself sits above the title in reading order -> listed as considered (cannot repeat a node not yet open).
- 第一节 重要提示、目录和释义: 目录 and 释义 on their own pages are L3 under 第一节 (the section title names them).
- 重要提示 (SSE template): one-sentence items (一、…四、) are text (considered); items printed as a heading line with
  content below (五、利润分配预案, 六、, 九、, 十、其他, and the questions 七、八、 answered 否) are L3 under 重要提示
  (user decision 2026-11-12: headings when content sits under them). OPEN under v3 substance: 否 / 不适用 is far less
  than "much more content than heading".
- SSE/SZSE body: 第X节 L2, 一、 L3, （一） L4, 1. L5, （1） L6 — every numbered line printed on its own line with content
  below. Numbered template items stay headings even when only 适用/不适用 follows (the numbering is the outline).
- Unnumbered template label lines: heading when a real block follows ("报告期末公司前三年主要会计数据和财务指标的说明" +
  numbered explanations -> L4, sibling of (一)/(二) because it explains all of 七's data); text when only
  适用/不适用 follows ("季度数据与已披露定期报告数据差异说明") (substance).
- Stock-code header (股票代码：…), logo headers: text. Lead-in sentences before tables: text.
- Cover with a logo lockup (company name + English name beside the logo, gold L1 boxes) above a title block
  "公司名 / 20xx年年度报告": L1 = the title block; the lockup lines are not_headings (logo text).
- SZSE (cninfo) template: table-caption subheads 一、公司信息 … are L3 (numbered outline). Unnumbered lines that only
  name the single table right below (公司聘请的会计师事务所, 公司聘请的…财务顾问 + 适用 + table, 研发投入情况表,
  营业收入整体情况, 公司主要销售客户情况) are table captions -> text. Unnumbered labels whose block has its own
  discussion (赔款支出 + table + paragraphs; 报告期末…财务指标的说明 + numbered explanations) are headings one level
  below. Labels with only 适用/不适用 or a one-word 是/否 answer are text (substance). OPEN: caption vs label is a
  judgement on whether the block is just the table.
- Numbered captions (1、主要资产重大变化情况 / (1). 主营业务分行业… printed as a table caption) stay headings: the
  numbering puts them in the outline.
- Numbering ladder with skipped levels: 1、 directly under 一、 (no （一） printed) is one level below 一、 (levels never
  skip). A、/B、/C、 business-line heads inside （二） sit one below it, their 1、 and （1） below that (L7 reached).
- Text-block outline lines (（一）优化产业结构… on its own line, paragraph below) are headings even though the gold
  leaves them as text.
- Running head "2019年年度报告" (document type only, no owner) still names the document -> repeat of L1.
- Lead-in sentences ending in a colon before a table ("保险业务收入分险种如下：", "保险业务主要会计数据和财务指标：")
  are text even when the gold marks them. Bold own-line labels with their own table + discussion (赔款支出,
  保险业务手续费支出) are headings one level below their numbered item; 研发投入情况表 (caption of one table) and
  情况说明 with only 不适用 below are text.
- Gold sometimes roots 第X节 at L1 and leaves 第一节/目录 without relation (b20666fd): re-rooted under the cover title.
- Bold run-in （一）…。 / （1）…： lead-ins starting a paragraph are text; （一） printed on its own line with a block
  below is a heading.
- Cover where company name and report title are separate blocks (address + logo between them, baa53c78): one union
  L1 (owner ∪ document type) over the two boxes; address/logo/codes stay text.
- Running head split into two header boxes (公司名 | 2019年年度报告, bd2f20cd): one repeat with both boxes.
- Numbered risk items (1、原材料价格波动…风险 + 应对措施 block) inside 第一节 重要提示 are L3 under 第一节.
- EN glossy wrapper + Form 10-K (Cray): the owner is printed only as a logo -> L1 = "2018 ANNUAL REPORT" alone (logo is
  text); cover slogans are not_headings. Embedded "FORM 10-K" = L2 union with the registrant-name box
  (CRAY INC., a mixed cover-field box; node text = "FORM 10-K CRAY INC."); the SEC name/address above it is the
  issuing body (not_heading). 10-K: front-matter titles + PART = L3, Item = L4, bold heads L5, bold-italic subheads L6; italic or bold
  run-in lead-ins and bold risk sentences are text.
- ASX Appendix 4E bundle (Abacus): L1 = "Appendix 4E" ∪ "Abacus Group" (doc type + owner on page 1); the p1
  subtitle "Annual Financial Report / For the year ended" is text. The attached report's cover title "ANNUAL FINANCIAL
  REPORT" = L2 node; its title page restating it = repeat. Section title printed at the top of every Directors' Report
  page = node on first page (L3), repeats after. "X (continued)" with the section still open = repeat (not continuation).
- Standalone Form 10-K (Bloom): L1 = FORM 10-K ∪ registrant name box (text "FORM 10-K BLOOM ENERGY CORPORATION",
  the "(Exact name of registrant…)" caption dropped). SEC name = not_heading. Front-matter titles + Part = L2, Item L3,
  bold heads L4, bold-italic subheads L5, and content beats look for same-look subheads (Value Proposition /
  Competition of a product sit one below that product). Header "Table of Contents / Index to Financial Statements" is
  navigation text.
- Owner printed only as a logo (Lam, Cray): L1 = the printed document-type line alone.
- Back-page investor-information boxes whose layout box holds heading + body: node uses that box with text = the
  printed heading line only (t= override).
- Section-only running header + body title (alstria "Combined Management Report" header from the detail-index page on,
  body "A. COMBINED MANAGEMENT REPORT"): header = L2 node on its first page, body title L3 below it (600 Group rule),
  the detail index under the same header is L3 too; later headers = repeats. OPEN: header starts one page before the
  body section (on its detail index).
- Running footer "<owner> Annual Report 2024" = repeat of L1 (names the document).
- Board/executive name lists (Alaska): names are not_headings even though gold marks them; "Committees:" legend label
  too. Corporate Information box titles stay L3.
- Title box that also holds the title's translation (Annual Report 2024 / Pūrongo-ā-tau …): v3 makes title +
  translation printed with it one union -> node keeps the full box text.
- Cover year split from "Annual Report" in two boxes (Gogo "2024" / "Annual Report") = one union.

## OPEN
- 重要提示 yes/no questions (七、八、) answered by a single 否: kept as headings per user decision, but v3 substance
  ("much more content than heading") argues against.
- Caption vs label: unnumbered template lines naming the one table below (公司聘请的会计师事务所, 研发投入情况表) made
  text; labels with table + discussion made headings. Boundary is a judgement.
- Owner printed only as a logo (Lam, Cray, MSCI, alstria, Gogo, Fonterra): L1 has no owner part; running owner
  wordmarks in headers are text.
- alstria: section-only running header begins one page before its body title (on the detail index page).
- Embedded/standalone 10-K: registrant name taken from a mixed cover-field box ("(Exact name of registrant…)"
  caption dropped via text override); Cray's box also holds the file number.
- Cover-page running head printed above the title (…年度报告全文 on page 1) cannot be a repeat (L1 not yet open) ->
  considered.

## Breadcrumb review round 2

Fix classes (37 docs, 853 paths read, 51 unseen pages viewed):
- Missed numbered outline items (1): c8425a15 1、/2、 under 七、境内外会计准则下会计数据差异 (#54, #57) added as L4, as in
  every sibling SZSE tree (numbered template items stay headings even with only 不适用 below).
- Section-only running header listed after its page's body headings (2 docs): the gold leaves header boxes unordered,
  so a header node must be listed before the headings it sits above. 600 Group (778bc822): Strategic report #60 now
  parents the p7 subheads (were under Chairman's statement, which ends with its signature on p6); CFO Statement #159
  parents Chief Financial Officer Statement #160 (was under Strategic report); Corporate governance #176 parents the
  p13 subheads (were under CFO Statement). alstria (c58ec05f): DETAIL INDEX #19 under Combined Management Report #18
  (was under CONTENT).
- Contents title box that restates the document title (2 docs): Cray #54, Bloom #38 keep the box, text = the printed
  heading line only (INDEX / Table of Contents); the restated owner/form/year lines are a repeat, so the breadcrumb
  no longer restates its parent.
- Containment (1): 77918496 二、未出席董事情况 (+ small table) made text: the one-sentence 重要提示 items 三、/四、 after it
  are not about it; now treated like the other items 一、–四、.

ACCEPTED (flags left in place):
- ACCEPTED 00114ac3 #20, 16acace3 #21, 1737c8bb #19, 588028f9 #18, 679b76a4 #17, 787462d7 #18, 7de80556 #18,
  7f487dea #19, b20666fd #18, baa53c78 #40, bad1d0cc #18, c8425a15 #17, cee67928 #17, dbf02372 #17, f0e6067b #36
  FRONT: 目录 sits under 第一节 重要提示、目录和释义, whose title names it (SZSE template).
- ACCEPTED 588028f9 #90 #94 #99 #119 #121 NUM: the document really prints a second 一、/二、/三、 ladder (bold, smaller,
  own block) inside 一、报告期内公司从事的主要业务 and 一、概述.
- ACCEPTED eng_0c770734 #54 (INDEX), eng_5a2b9698 #35 (CONTENTS), eng_a7bf8c0c #133 (TABLE OF CONTENTS),
  eng_dd7d06a3 #40 (INDEX) FRONT: contents of the embedded 10-K / attached financial report, inside that L2 node.

OPEN (round 2):
- Last numbered risk item in 第一节 (baa53c78 5、疫情冲击风险, f0e6067b 3、技术储备不足的风险): the template's
  利润分配预案 paragraph follows it with no heading before 目录, so it sits in that item's subtree. Kept the items as
  headings (each has its own risk + 应对措施 block); the trailing paragraph is a containment exception.

## Breadcrumb review round 3

Independent review. 37/37 docs checked, all 854 breadcrumb paths read (819 after the fixes), plus every
non-node element in reading order (scratchpad outline) and the tail of every subtree whose next node is shallower or
starts a different numbering series (containment scan). check.py: no ERROR on any doc.

Fix classes:
- Containment, trailing parent-level content after the last item of a numbered series -> the whole series is text
  (all or none: if the last item is text, the item before it would contain it). 4 docs, 20 nodes -> considered:
  - baa53c78 1、–5、 risk items (#14 #17 #24 #27 #33) and f0e6067b 1、–3、 (#20 #23 #26): the 利润分配预案 paragraph
    (#36 / #32) followed the last risk item with no heading before 目录. Resolves the round-2 OPEN / KNOWN case. The
    paths are now L1 › 第一节 › 目录 / 释义.
  - 16acace3 第四节 一、概述 （一）–（八） (#207 #209 #211 #213 #219 #221 #223 #225): closing paragraph #227
    (报告期，得益于上述各项工作…) of 概述 fell under （八）.
  - b20666fd 三、核心竞争力分析 1.–4. (#117 #119 #126 #129): closing sentence #131 (报告期内，公司核心竞争力未发生重大变化。)
    fell under 4. 质量管理优势.
- Containment, front-matter labels with no closing heading (1 doc, 8 nodes -> considered): 600 Group (778bc822)
  Who We Are #10 + Position Statement / Mission / Vision (#14 #16 #18) and We Stand For #41 + Service / Innovation /
  Quality (#42 #44 #46). Vision held the brand logos, the p3 adviser list and the board photo; Quality and both L2
  labels held the untitled p5 contents list; nothing closes them before the Chairman's statement header (p6). Same
  outcome as financialReport d1c80a8f 我們的宗旨 (round 2).
- Restated section title (7 docs, 7 nodes -> repeats): SSE template 第一节 释义 › 一、释义 is not a narrower step;
  一、释义 is now a repeat of 第一节 释义 (1695038a #48, 170f7a9e #33, 1c29d148 #33, 35b8e075 #32, 77918496 #34,
  bd2f20cd #39, ebe06969 #36), like the restated-title rule. The lead-in sentence and the glossary table sit under
  第一节 释义.

Checked and kept:
- Signature / date lines closing a statement under its last subhead (600 Group "Paul Dupee / Chairman" after Other
  risks and uncertainties, "Rui Lopes / CFO" after Conclusion): attribution, like a byline; they do not break
  containment.
- SEC name line printed above an embedded FORM 10-K title (Cray #21, Alaska #103, Gogo #6) falls before the 10-K node
  in reading order: it is the kicker-like issuing line of that cover, not content of the previous section.
- 七、近三年主要会计数据和财务指标 › （二）主要财务指标 followed by the 说明 label + 不适用 (1c29d148, 77918496): the
  label is text and its one-word answer is about the indicators; no leak.
- alstria "Combined Management Report › A. COMBINED MANAGEMENT REPORT": section-only running header + body title,
  the NOTATIONS 600 Group rule; kept.

ACCEPTED (flags re-judged, all correct):
- ACCEPTED 00114ac3 #20, 16acace3 #21, 1737c8bb #19, 588028f9 #18, 679b76a4 #17, 787462d7 #18, 7de80556 #18,
  7f487dea #19, b20666fd #18, baa53c78 #40, bad1d0cc #18, c8425a15 #17, cee67928 #17, dbf02372 #17, f0e6067b #36
  FRONT: 目录 under 第一节 重要提示、目录和释义, whose title names it; the step is narrower.
- ACCEPTED 588028f9 #90 #94 #99 #119 #121 NUM: the document prints a second 一、二、三、 ladder (bold, own blocks)
  inside 一、报告期内公司从事的主要业务 and 一、概述; each step is a narrower description.
- ACCEPTED eng_0c770734 #54 INDEX, eng_5a2b9698 #35 CONTENTS, eng_a7bf8c0c #133 TABLE OF CONTENTS, eng_dd7d06a3 #40
  INDEX FRONT: contents of the embedded 10-K / attached financial report, inside that L2 node.

Unseen pages viewed (28 images, all 51 listed pages of this category: c8425a15 1-8,11,13; 7f487dea 5-8;
f0e6067b 1,4-9; 679b76a4 3-6,9; dbf02372 2-6,8; 7de80556 3-4; baa53c78 5; bad1d0cc 2-4; 77918496 3-4;
ebe06969 2-6,8; bd2f20cd 2-5; eng_70d0bf2b 3): no missed headings or wrong boxes.

Round-1/2 OPEN cases:
- (closed) Last risk item + 利润分配预案 paragraph: fixed above.
- (closed) alstria header one page before the body title: the detail index under it is about that report, so the path
  is consistent.
- (closed) Embedded 10-K registrant name from a mixed cover-field box: v3 "owner only in a box with other fields" rule.
- (closed, not a path) cover-page running head above the title = considered; caption vs label; 重要提示 七、/八、 with 否
  (user decision 2026-11-12 keeps them).
- OPEN (not an inconsistency): owner printed only as a logo (Lam, Cray, MSCI, alstria, Gogo, Fonterra). L1 is the
  document type alone, so the breadcrumb lacks "whose", but every step is still narrower. MSCI's logo wordmark is OCR'd
  as a header text box ("MSCI"); a union MSCI ∪ 2024 Annual Report would give the owner but contradicts "logo and
  brand text are text".

## User decisions (2026-11-13)
- Numbered list items stay headings when a closing/summary paragraph follows the last item; the paragraph sits under
  the last item. Restored 20 items (baa53c78 1、–5、, f0e6067b 1、–3、, 16acace3 （一）–（八）, b20666fd 1、–4.). Rule in NOTATIONS.md.
- Owner printed only as a logo: the logo's text joins the L1 union when the layout has a box holding that text; a logo
  with no text box stays out (rule in NOTATIONS.md "The title"). Supersedes the round-3 OPEN and the "Lam, Cray: L1 =
  document type alone" note above.
  - Changed: eng_b3c337f8 L1 "2024 Annual Report" -> "MSCI 2024 Annual Report" (header box #1 "MSCI" joined, removed
    from considered; node id #3 -> #1, children/repeats re-pointed).
  - Unchanged (logo is an image with no text box): eng_0c770734 (Cray, figure #4), eng_824dec82 (Lam, figure #1),
    eng_c58ec05f (alstria, header_image #1), eng_dd7d06a3 (Gogo, header_image #1), eng_fc735d9d (Fonterra,
    header_image #2; the "Dairy for life" header is the slogan, not the owner). Abacus' attached-report L2 keeps its
    title (L1 already names Abacus Group).

## Round 4 (full path review, 2026-11-13)

Every node of all 37 documents was checked against its page with the tree drawn (all 343 pages viewed; a7bf8c0c and
c58ec05f have no page 2 in the set). 839 nodes: 839 OK, 0 FIXED / ADDED / REMOVED. check.py: no ERROR on any doc.
Per-node lines: review/paths/annualReport.tsv.

Checked and kept (all covered by NOTATIONS.md or earlier decisions):
- SSE 重要提示 七、/八、 answered 否 stay L3 (user decision 2026-11-12); numbered 1、–5、 risk items and 16acace3
  （一）–（八）, b20666fd 1、–4. hold the trailing paragraph (user decision 2026-11-13).
- 588028f9 second 一、二、三、 ladder inside 一、 (printed, bold, own blocks); 77918496 第四节 一、经营情况讨论与分析 is
  a sibling of 二、 with bold run-in (一)–(六) paragraphs, not a restated title.
- Unnumbered 说明 labels with 适用 + numbered explanations stay L4 (bd2f20cd #69, ebe06969 #65, 1695038a #79,
  170f7a9e #60, 35b8e075 #62); 差异说明 / 情况说明 with 不适用 stay text.
- Embedded/standalone 10-K covers (Cray, Gogo, Bloom, Acura, Alaska): FORM 10-K ∪ registrant-name box; contents
  boxes that restate owner/form keep only the heading line.
- Section-only running headers (alstria Combined Management Report, 600 Group Strategic report / CFO Statement):
  node on the first page, body title one level below.
- Node texts that carry the box's OCR as printed in the layout (alstria "\*\*\*" / "*****" footnote stars, Acura
  "LIMITx? Technology" for ™, Alaska "PARTI", "10\%" escapes) were left as the box text.

OPEN: none.

## Round 5 (breaks, 2026-11-13)

All 37 documents, every page rendered with `show.py --tree` and viewed (100 images, review/breaks/annualReport_tmp/img),
plus a reading-order outline per document. 13 breaks, 9 restored nodes. check.py: no ERROR on any doc.
Lines: review/breaks/annualReport.tsv.

- Closing paragraph after a numbered list (user decision 2026-11-13 kept the items as headings): break before the
  paragraph closes the last item. baa53c78 #36 (closes 5、#33) and f0e6067b #32 (closes 3、#26): the 利润分配预案
  paragraph after the 第一节 risk list; 16acace3 #227 (closes （八）#225); b20666fd #131 (closes 4. #129);
  Acura ea53b553 #147 (sentence about both non-clinical studies, closes APT-RDR-301 #144).
- Contents ends and untitled text follows: break closes the contents node. Bloom #40 and Alaska #135 (definition of
  "we / the Company" below the 10-K table of contents), Cray #56 (trademark notice below INDEX), Abacus #37
  (read-in-conjunction note below CONTENTS, now under the attached report's L2).
- Abacus #21: the cover image of the attached Annual Financial Report precedes its title in reading order; break
  closes Distribution Reinvestment Plan (#19) so the image is not under it.
- Restored (were text only by containment): 77918496 二、未出席董事情况 #9 (L3, numbered item + its own table; break
  before 三、#11). 600 Group (778bc822) Who We Are #10 (L2) with Position Statement / Mission / Vision #14 #16 #18 (L3),
  We Stand For #41 (L2) with Service / Innovation / Quality #42 #44 #46 (L3). Breaks: #20 closes Vision (the group and
  brand logos stay under Who We Are), #25 closes Who We Are (company-information list and board photo go to L1), #50
  closes We Stand For (untitled p5 contents list goes to L1). Supersedes the round-3 containment fix.
- No break: signature / date blocks after letters and statements (attribution), lead-in sentences, 适用/不适用 answers,
  table captions and notes, running heads, decorative full-page photos with location captions (Fonterra, Lam chip
  spread), cover taglines under L1.
- OPEN: Abacus #45, an intro paragraph to the operating and financial review sits under PRINCIPAL ACTIVITIES AND
  STRUCTURE (#43); a break is impossible because #43's own L5 child follows it.
