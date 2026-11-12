# newspaper learnings

## Decisions
- Magazine supplement covers (eng_79250562): masthead = brand line + "MAGAZINE" box merged into one L1. Cover lines
  (teasers with "By ...") are text. Full-page ads: every line text, incl. ad slogans the gold marked as titles.
- Contents page without a printed contents title: no node. A boxed feature on it with its own title
  ("FAB FIVE: ...") = L3 under the masthead (no section printed).

- Weekly news magazine (eng_b6de1de8, Guardian Weekly): cover lines incl. the big cover story line = teasers (text,
  they point to a page). Contents page with no "Contents" word: the repeated masthead/tagline and every TOC entry
  (big page numbers + section names) are text. Colophon logo (GW) text.
- Briefs keyed to a map ("1 EUROPE", "2 GERMANY" location labels above each brief headline): labels are kickers
  (text); brief headlines are L3 under the section banner ("Global report" L2). Deck under the banner is text.
- Photo-spread rubric ("Eyewitness Italy") is a rubric (text); the photo story headline ("The rat pack") is L3.
- Split masthead (THE | crest | TIMES as two boxes): one L1 with merge.
- Front-page "IN THE NEWS" box where every item ends in a page pointer ("Page 6"): teaser/pointer box, label and items
  text. (OPEN: if such a box has no page pointers it is a briefs column: label L3, items L4.)
- "TODAY'S EDITION" index box, podcast/app promos, "THE WEATHER": text.
- Jump in a text_block "CONTINUED FROM PAGE 1 <short head>": node on that element with text = the short head only,
  `continues` the front-page story, under the current section.
- Running section tag on p2 ("News") = L2 (section changes from the implicit front-page section).
- The Times family (eng_30d4d13c, eng_8a4769ba): bylines the gold marked as titles = text; COMMENT pointer box, OFFER,
  TIMESRADIO ad = text; puzzle box with its own title ("Quintagram No 2031") = L3; untitled-column short stories
  (no briefs label) = L3 each.
- USA TODAY (eng_e4dd9616): skybox teasers ("... In News") text; colophon box with field titles (Customer service,
  Contact us, Corrections and clarifications, Postal information) = text; PAID ADVERTISEMENT pages (advertorial with
  headline + subheads) = all text. In-story subheads = L4, also inside jumped continuations.
- A second bold line under a photo-story headline (separated by the photo) = deck (text). OPEN: borderline.
- USA TODAY jumps are headed by a one-word jump head ("Beryl", "Safety rules", "Page" for the Susan Page column):
  continuation node with that text. Small front-page box "Biden backers sound off ... 6A" = teaser (text).
- Washington Post (eng_105ddbb0): front-page teaser strip with section names (METRO, SPORTS, TRAVEL ...) = text.
  A2 has no section tag: its stories stay L3 under the masthead. Small CORRECTIONS box = L3 (like "For the record").
  A section banner ("POLITICS & THE NATION") holds over later pages that print no tag; a topic label on top of a page
  ("ELECTION 2024") under that section = rubric (text). OPEN: could be read as a section tag (L2).
- Jumps with their own new headline ("BIDEN FROM A1" + new headline) = continuation node with that headline text.
- Explainer packages: headline L3, item heads (numbered "1. ...", or each ruling) L4; case-name lines are text.
- Boston Globe (eng_1153fb03): same as the reference. DAILY BRIEFING items under the label = L4. Kickers in caps above
  a headline (BITTERSWEET MOMENT, CLOSE CONNECTION) = text; rubrics inside the text (NEWS ANALYSIS, POLITICAL
  NOTEBOOK) = text. Front-page photo teaser with "C1" pointer = text. Running tag "The Region" on the last page = L2.
- Multi-section USA TODAY (eng_ad370234): section-front banners MONEY / SPORTS = L2 (the "USA TODAY | date | SECTION B"
  line is text); "MORE COVERAGE" pointer box = text; briefs column MONEYLINE = L3 with items L4; "50 States" page =
  L3 briefs feature under the running section (MONEY), "STATE City:" run-ins text, "HIGHLIGHT: INDIANA" L4.
- Q&A interviews: bold questions are run-in text (no nodes).
- Boston Sunday Globe (eng_b5e01773): "THE WORLD TODAY" briefs label L3, brief headlines L4, country label above each
  brief = kicker (text, not merged; node text = headline only). INDEX box text. An ad page between two pages with the
  same running tag does not restart the section (no new node).
- Front-page series banner over two stories ("Hope in America", eng_c21bfc8c) = package: banner L3, stories L4.
  Their jumps ("Hope", "Maine") are L3 continuation nodes continuing the L4 story (the jump role is story-level
  there). OPEN: whether a jump of a package story should be L3 or L4.
- Recurring explainer rubric "Just Curious / You've got questions?..." = rubric (text); pull-quote attribution text.
- Opinion page tag "OPINION" printed in the page header = L2 (section change).
- 人民日报 (chn_11ae9c36): masthead 人民日报 L1, pinyin RENMIN RIBAO text. A main headline printed over two boxes
  (中共中央国务院举行春节团拜会 / 习近平发表讲话 in the same big type) = one L3 with merge; 引题 above and 副题 below
  (smaller lines, "——" subtitles) are decks (text); when the subtitle shares the box, node text = main title only.
  Speech title on front page = L3 (date, speaker text). Red bullet summary lines = text. Page tag 要闻 on p2 = L2.
  Segment heads 【镜头一：...】 = L4 and the repeated 【回访】 under each = L5.
- 经济日报 (chn_43c21ee3): column rubrics (经济论坛, 金视角) and red 专栏 labels (学习贯彻..., 四中全会精神在基层) =
  text; 引题 ending in "——" = deck; 导读 index box = text. When the gold box holds 引题/专栏 + main title, node text =
  main title only. OPEN: red 专栏 labels could be read as section-like banners.
- Chinese dailies: every page's header tag (要闻, 综合, 国际, 时评, 地方, 企业, ...) = L2 on that page (each page is a
  new section). Front page (版面 1) has no tag: stories L3 under the masthead.
- Jump without a printed headline ("（上接第一版）" followed by the story's subheads): continuation node on the jump
  line, text as printed "（上接第一版）", continues the front-page story; its subheads become its L4 children.
  OPEN: text for such a node (jump line vs. original headline).
- A cartoon box (漫画) with a title and a short comment = L3 (box with its own title).
- 农民日报 (chn_adf00537): headline repeated above a jump on p2 = continuation node with `continues`. A jump that is
  only a body paragraph starting "（上接第一版）" with no subheads = no node (nothing to hang).
- A regulation/decision printed in full inside a news story: the document's own title = L4 under the story, its
  chapters (第一章) = L5.
- 光明日报 (chn_eb87830d): advertorials framed with "·广告·" (university profiles with headline + subheads) = all
  text. A special-supplement page banner (红船初心专刊) = L2. Figure-only rubric banners (光明论坛, 光明时评, 连续报道)
  are not elements to label. Numbered part markers printed alone as heading lines ("（一）") = L4 nodes.
  OPEN: advertorials might be wanted as stories since they read like articles.

## Level mapping used across the category
- L1 masthead; L2 section (printed banner, running section tag in English papers when the section changes, the page
  tag of every Chinese page, special-supplement banners); L3 story / briefs label / package banner / box with own
  title; L4 in-story subheads, briefs items, package stories; L5 one level further (e.g. 【回访】 under 【镜头】).
- Gold had front-page stories at L2 directly under the masthead; all moved to L3 (implicit front-page section).

## Open questions
1. Front-page "IN THE NEWS" style boxes whose items all end in page pointers: treated as teaser boxes (text). If they
   should be a briefs column (label L3, items L4), eng_30d4d13c and eng_8a4769ba change.
2. Deck-like second line under a photo-story headline (eng_e4dd9616 #34, eng_ed12985c #46): treated as decks.
3. Washington Post "ELECTION 2024" page label under POLITICS & THE NATION: rubric (text) or a section tag (L2)?
4. Jump of a package story (eng_c21bfc8c "Hope", "Maine"): placed at L3 (story role on the jump page), not L4.
5. Chinese jumps with no printed headline: node on the "（上接第一版）" line (or the paragraph that opens with it)
   with that text, so the jumped subheads have a parent. Alternative: copy the original headline as node text.
   Jumps with no subheads after them got no node.
6. 引题/副题 and 专栏 rubrics (经济论坛, 金视角, 学习贯彻...) treated as text, node text = main title only, even when
   the 引题 carries the news subject (e.g. "李强签署国务院令" / "公布修订后的...办法").
7. Advertorials framed "·广告·" (光明日报 university profiles) and USA TODAY PAID ADVERTISEMENT pages: all text.
8. Running-tag-only sections in English papers: an ad page between two pages with the same tag does not restart the
   section; Chinese pages with the same tag on consecutive pages (综合, 综合) get one node.
9. Magazine cover lines (The Times Magazine, Guardian Weekly): all teasers (text), including the main cover story line.

## Label rule pass
- Generic segment label repeated under every in-story segment (【回访】 after each 【镜头N】 head, 人民日报 p2): text
  (removed 11 nodes, not_headings "label rule").
- Briefs column label (DAILY BRIEFING) printed in several sections: kept as story-level heading (briefs-column note),
  not a generic label.
- Salutations opening a speech or reader letter (同志们，朋友们：; 编辑部：) and lead-in sentences before a list ("The
  least renter-friendly cities are:"): text, left as is.
