# researchReport learnings

## Decisions (carry over)
- L1 = report headline. Merge into L1 (one node, `merge`) the lines printed as part of the same title block:
  the "——subtitle" line under the headline (——公司（代码）深度研究, ——2020年年报点评, 化工行业周报（日期）),
  and a company/stock banner printed directly on top of the headline as part of the title block (通威股份（600438）,
  长海股份 300196.SZ). A company name inside the date/rating info box or in the running head is NOT merged (text).
- Report-type / industry banners on the cover (行业月度报告, 电子, 化工, 非银金融, 家电行业 行业研究/周报,
  "动力电池行业行业研究 买入（维持评级）") are text. Exception: when no headline exists and the large banner is the
  only title of the issue (researchReport_chn_5463fcf3 家电行业; chn_1c78bccd 中國市場及A股點評 periodic commentary),
  that banner is L1.
- Generic "证券研究报告" lines are text.
- Cover summary title (投资要点 / 投资要点： / 核心观点 / 报告要点 / 报告摘要 / 事件 / 点评 / Investment highlights) = L2,
  same level as the body chapters. Sub-heading lines inside it (bulleted lines printed on their own line) = L3.
  When the cover has no summary title but its bullet heading lines are printed as own lines (chn_25d1b14f), they are L2.
- Bold run-in lead-ins (■ 上半年业绩下滑并非主营问题。..., 风险提示：..., 一、市场加速下跌，... bold sentence at paragraph start)
  = text.
- Sidebar box labels on the cover = text: 股价走势 / 相对指数表现 / 行业涨跌幅比较 (chart labels), 市场数据 / 基础数据 /
  基本数据 / 主要数据 (data-table labels), 相关报告 / 相关研究 (related-reports lists), 分析师 / 证券分析师 / 研究助理 /
  联系人 / 报告作者 / 执业分析师 (analyst lists), research-centre banner over analysts, the rating (增持 ——维持).
- Table titles (公司主要财务信息, 数据预测与估值：, 附表 table title) and table banner labels = text.
- 内容目录 / 正文目录 / 目录 / 图表目录 = L2.
- Back matter: disclaimer/analyst-statement/rating-definition titles (免责声明, 分析师声明, 投资评级说明, 合规声明,
  研究团队简介, 特别声明, 公司评级说明：...) = L2; their sub-blocks (股票投资评级：/行业投资评级：) = L3.
  Office address boxes (东方证券研究所, 上海/北京/深圳, 国元证券研究所, 川财研究所), sales-contact table titles
  (机构销售联系人) and an analyst's name under 分析师声明 = text.
- Numbered body: 1、/1./一、 = L2, 1.1/（一） = L3, 1、 under （一） = L4.
- Morning/daily brief (chn_2161890a 万联晨会): brief name L1; cover overview box 概览 L2 with its boxes L3; body banners L2.

## English (Goldman Sachs) families
- Banner title box often holds a series kicker line + the headline (Global Strategy Views / Diversify to Amplify;
  Japan Economics Analyst / 2023 Outlook...; Oil / Squaring...). Kept the gold box as L1 with `text` = the headline only
  (kicker is text). Where the kicker is its own box above the headline (Singapore, US Economics Analyst, GS SUSTAIN: ESG of
  the Future) it is a not_heading / text.
- Cover-style thematic reports (Carbonomics, Adaptation): big title + subtitle box merged into one L1 (the report cites
  itself as "Title: Subtitle").
- Headline restated at the top of the body (p2) = text (title repeated later).
- AUTHORS / All Authors list titles and analyst-name boxes = text.
- Bold run-in lead-ins / questions starting a bullet or paragraph = text; standalone Q1:/numbered question lines = L2.
- Exhibit infographics: panel titles inside a figure under an "Exhibit N:" caption = figure content (text). A full-page
  infographic section with its own page title and no exhibit caption (eng_d3339752 "Impact through Affordability") = L2
  with its column titles L3.
- Disclosure Appendix family (eng_02963ff9): Disclosure Appendix L2, Reg AC/... L3, underlined sub-parts L4.
- Top of Mind (periodical): L1 masthead "TOP of MIND" (special issue: masthead + issue title in one box = L1),
  L2 issue theme, L3 contents box (WHAT'S INSIDE) and page-banner articles, L4 region boxes (US/Europe/Japan/EM),
  L5 the two labelled lists. Deck under Macro news and views, chart titles in the boxes, editor byline,
  INTERVIEWS WITH: label and every "Allison Nathan: ...?" interview question = text (Q&A turns, not headings).
  Special issue: Across:/Down: crossword labels, theme-number badge, exhibit/table title bars = text; page banners L2.

## Open questions
- chn_1c78bccd: small blue column rubric 大市導航 above the first headline treated as a rubric (text); banner 中國市場及A股點評 as L1.
- chn_5463fcf3: weekly product sheet; L1 = 家电行业 (largest title), product line 信达卓越推/推荐公司精选 text; 产品提示 box label text.
- chn_7033db6d: 核心观点（10.8-10.12）: was a text_block in gold; made L2 summary title with 1./2./3. under it at L3.
- chn_e960c7b3: blue bold summary bullets — only those printed as short lines heading a 1) 2) list made L3; whole-paragraph key-point bullets in the same style kept as text (same look, different role).
- chn_ae3bbb85: bullet label 本周行业主要动态： printed on its own line made L3 while the sibling labels (本周板块行情：, 投资建议：) run in and are text.
- Top of Mind interview questions (33 gold L4 titles in eng_5b6f3871/eng_aaabf80f) made text as Q&A turns; they are bold
  coloured lines and could be argued as interview subheads. eng_1588f970 gold already had them as text.
- Top of Mind special issue (eng_b3735ab5): divider panel "1 Markets worried first about growth..." has only a number
  badge; badge made text, no heading for the panel.
- eng_df933ffb: #150 (smaller bold subhead under the big section heading #89 on a later page) set L3 under #89; it may be
  a sibling L2 printed in a smaller style.
- eng_d3339752: #28 infographic page title kept L2 (no exhibit caption) while eng_4872e373 Exhibit 1 panel titles were
  made text — distinction is whether the page is captioned as an exhibit.
- Merged L1 vs kicker: when kicker and headline are one gold box, the node text drops the kicker (box cannot be split).

## Label rule pass
- Cover summary titles ending in a colon (投资要点：, 报告要点：, 报告摘要：, 核心观点（…）：, 事件：/点评：) = section
  titles of the summary, not labels: kept L2.
- Back-matter titles ending in a colon (公司评级说明：, 分析师声明：, 免责声明：, 本公司具备证券投资咨询业务资格的说明：,
  特别声明：) and their rating sub-blocks (股票投资评级：, 行业投资评级：, 行业投资评级的量化标准：) name specific content:
  kept.
- Own-line bullet labels naming the list under them (受益标的组合：, 行业要闻：, 本周行业主要动态：) = specific labels: kept.
- Key-point sentences ending in a colon over a 1) 2) list (chn_e960c7b3 blue bullets) = lead-ins: text (4 nodes
  removed; same style as the whole-paragraph bullets that were already text).
- Top of Mind region boxes: the two list labels repeated in every box (Latest GS proprietary datapoints/major changes in
  views, Datapoints/trends we're focused on) = generic labels: text in eng_1588f970, eng_5b6f3871, eng_aaabf80f
  (8 nodes each; "Latest GS..." was not a candidate but is the same label class, removed with its sibling).
- False kind-A hits kept: issue theme vs same-named lead article (ALL ABOUT BANK(PANIC)S / All about bank(panic)s,
  THE IPO SPAC-TACLE); morning-brief overview boxes vs body sections of the same name (chn_2161890a 核心观点/财经要闻).
- Body-style lead-in labels left as text: 关键假设： (chn_d0eac218), INTERVIEWS WITH: (names list).
