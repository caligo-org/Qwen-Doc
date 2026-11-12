# annualReport learnings

## Decisions
- Cover (CN, cninfo style): company name + "20xx 年年度报告" printed as one title block -> one L1 node with both lines.
  Logo, slogan, date line (2020年03月), running header "...年度报告全文", cninfo footer logo are text.
- CN 第一节 重要提示、目录和释义: 目录 and 释义 printed on their own pages under 第一节 are L3 children of 第一节 (the
  section title names them), even though they are printed in a similar size. 第X节 = L2.
- CN 重要提示 page (SSE template): items 一、…十、 that are one-sentence statements with nothing under them are text;
  items printed as a heading line with content below (五、利润分配预案, 六、前瞻性陈述的风险声明, 九、重大风险提示, 十、其他,
  and the yes/no questions 七、八、 answered 否 below) are L3 under 重要提示. OPEN: the series shares one look/numbering, so
  one could argue all ten are list items or all ten are headings.
- CN 目录 on its own page with no 第一节 wrapper -> L2 (sibling of 重要提示).
- SSE template body: 第X节 L2, 一、 L3, (一)/（一） L4, 1. L5. Bold (一)/(二)/(三) lines under 八、境内外会计准则下会计数据差异
  with 适用/不适用 below are L4 headings even when the gold missed them.
- Unbolded template label lines followed by 适用/不适用 ("报告期末公司前三年主要会计数据和财务指标的说明",
  "季度数据与已披露定期报告数据差异说明") are text: printed as body lines, not heading lines. OPEN (borderline).
- Cover with a logo lockup (company name + English name beside the logo) above the title block "公司名 20xx年年度报告":
  the title block is L1; the lockup lines are text.
- SZSE (cninfo) template: section subheads printed as table captions (一、公司信息, 二、联系人和联系方式 …) are headings
  (L3). Small labels inside a subsection (公司聘请的会计师事务所, 公司是否需追溯调整…, 上述财务指标…是否存在重大差异) are
  text. Deep numbered run of heading lines (一、 > （一） > 1、/A、 > 1、 > （1）) all count when each is printed on its
  own line with content below; the gold often stops at （一）.
- Numbered run-in-looking paragraphs inside 经营情况概述 ((一)优化产业结构… on their own line, then a paragraph) are L4
  headings.
- Lead-in sentences ending in a colon that introduce a table ("保险业务收入分险种如下：") are text, even when bold.
  Table captions (利润表及现金流量表相关科目变动分析表) are text. SSE template sub-items printed as bold lines
  (研发投入情况表, 情况说明, 赔款支出) stay headings.
- Bold run-in lead-ins that start a paragraph ("（一）机制纸产销量创历史最好水平。全年…") are text (run-in labels),
  unlike （一） lines printed on their own line.
- Numbering beats the gold's flattening: under （一）主营业务分析 the "1./2./3." items are children of （一）, and
  "(1)./(2)." children of those (gold had 2. as a sibling of （一）).
- Cover where company name and "20xx年年度报告" are separate blocks (address/logo between them): L1 = the report title
  line only, company name is text (NOTATIONS rule). When they form one stacked title block: one L1 with both.
- Gold sometimes roots 第X节 at L1 or leaves 第一节/目录 without relation: re-rooted under the cover title.
- EN glossy wrapper + Form 10-K: cover slogans (MORE DISCOVERY…, KEEP ASKING) are text; L1 = "2018 ANNUAL REPORT".
  "UNITED STATES SECURITIES AND EXCHANGE COMMISSION Washington…" is the issuing body -> text; "FORM 10-K" = L2 (the
  embedded filing); DOCUMENTS INCORPORATED BY REFERENCE, INDEX block, Forward-Looking Statements, PART I = L3;
  Item N = L4; bold section heads = L5; bold-italic subheads = L6. Bold sentence lead-ins in Risk Factors are run-in text.
  OPEN: the 10-K INDEX title box also restates "CRAY INC. FORM 10-K For Fiscal Year…" (kept as one node, printed text).
- ASX Appendix 4E bundle: L1 = "Appendix 4E"; issuer name and the "Annual Financial Report / For the year ended" subtitle
  on that page are text; the attached report's title page "ANNUAL FINANCIAL REPORT" = L2 sibling of "Results for
  announcement to the market". Section title repeated at the top of every page ("DIRECTORS' REPORT" + date) is a
  running head -> text. "X (continued)" subheads = continuation nodes at the original's level.
- Cover with only the fund/company name (big) and "Annual Report <date>" (small) and nothing else between: merged into one
  L1 (title split over two boxes). OPEN: vs baa53c78 where address+logo separate them and only the report line is L1.
- When a top section's title is printed only as the running head (UK AIM report: "Strategic report", "Chairman's
  statement", "Corporate governance" in the page header, no title in the body), the running head on the section's first
  page is the L2 node (newspaper-style section tag rule); its repeats on later pages are text. When the body prints the
  title too ("Chief Financial Officer Statement"), the body title is the node. OPEN.
- Directory label/value pairs (SECRETARY, REGISTERED OFFICE, AUDITOR …) are form-field labels -> text. "(continued)"
  lines without a title are text.
- Glossy EN reports: chart titles (R&D Spend (Millions)…) are text; intro-page headlines and letter titles are L2.
  Back-page investor-information titles (STOCK LISTING, INVESTOR RELATIONS, ANNUAL MEETING…) are L2 siblings; when the
  layout box also holds the body, the node uses that box with only the printed heading line as text.
- Standalone Form 10-K: L1 = "FORM 10-K" (the document title); the SEC name/address above it is the issuing body and
  the registrant name is a cover field -> text. Top front-matter titles (DOCUMENTS INCORPORATED BY REFERENCE, TOC title,
  forward-looking note) and "Part I" are L2; Item = L3; bold heads L4; italic heads L5. Bold-italic run-in bullet
  lead-ins are text. OPEN: whether the registrant name should merge into the L1.
- Board/executive name lists (name in bold + role below) are lists of names -> text (NOTATIONS), even though each name
  is styled like a subhead. "Committees:" label -> text. Corporate Information box titles (Annual Meeting, Transfer
  Agent…) are L3 under "Corporate Information".
- Wrapper + 10-K (Alaska): FORM 10-K = L2; report title restated above "TABLE OF CONTENTS" -> text; TOC title, cautionary
  note, PART I = L3 under FORM 10-K.
- OPEN: 10-K subheads with identical bold-italic look (LTX-03 Development / Study 301 / Manufacturing…) are kept as
  siblings (same look = same level, no numbering), although the gold and the content nest them.
- OPEN: when the title box also holds the title's translation (Annual Report 2024 / Pūrongo-ā-tau …), node text keeps
  only the original-language title line (translation = text per NOTATIONS); the box cannot be split.

## User decisions (2026-11-12)
- A running header printed only on one section's pages (600 Group: "CFO Statement") is that section's heading (L2),
  even when the body also prints a title; the body title ("Chief Financial Officer Statement") is L3 under it and the
  section's subheads L4. Later general rule (not yet applied corpus-wide): furniture repeated in part but not all of
  the document is a candidate section heading; first occurrence = node, later occurrences = repeats.
- CONFIRMED by user: L1 = the report title only; the company/fund name is text even when printed in the same title
  block (27 trees changed; 70d0bf2b: the report line #2 is the node, the fund name #1 text). Same reasoning as the
  prospectus cover decision.
- CONFIRMED: content beats look for 10-K subheads (ea53b553: Study 301 … AD Pharma Amended Agreement are children of
  LTX-03 Development). 重要提示 items: headings when content sits under them, one-line statements text (as done).

## Label rule pass

- Numbered A-share subsection "(三)境内外会计准则差异的说明：" ends in a colon but is a numbered outline heading
  beside (一)/(二) -> kept (7 docs).
- Kind A segment/brand subheads repeated under different sections ("Industrial laser systems" under trends and
  activity; ALASKA AIRLINES / Alaska Airlines under business and loyalty) name specific content -> kept.
- 10-K cover "(Mark One):" is a form field label -> stays text.
