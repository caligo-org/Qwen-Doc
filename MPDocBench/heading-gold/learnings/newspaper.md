# newspaper learnings (v3)

## Level mapping
- L1 masthead (pinyin / English name of the masthead = text; running masthead logo in page headers = repeat of L1).
- L2 section: Chinese page tag (要闻, 综合...) on every non-front page; English running section tag when the section
  changes; printed section-front banners; special-supplement banners.
- L3 story headline, briefs-column label, box with its own title; front-page stories are L3 directly under L1.
- L4 in-story subheads, briefs items; L5 labels with their own block under an L4 segment.

## Decisions
- 引题/副题 (smaller lines above/below a Chinese headline, often ending "——") = decks (text). When the deck shares the
  box with the main title, node text = main title only (text override). Main headline printed over two big lines
  (separate boxes) = one union.
- Column rubrics (本报评论员, 经济论坛, 金视角), red 专栏 labels above a headline, 导读 index box = text.
- v3 change: generic repeated labels with their own block (【回访】 under each 【镜头N】 segment) = headings, one
  level below the segment (L5).
- A jump that is only a body paragraph "（上接第一版）..." with no headline or subheads = no node.
- Jump with no printed headline but with subheads after it ("（上接第一版）"/"（上接1版）" line or paragraph start):
  continuation node on that element, text "（上接第一版）" as printed, `continues` the front-page story, L3 under the
  page section; the subheads are its L4 children. OPEN: node text (jump line vs. copying the original headline).
- Headline reprinted over a jump (农民日报 p2) = continuation node with `continues`.
- Same page tag on consecutive pages (综合 p3 and p4) = repeat of the first tag node, not a new section.
- A regulation/decision printed in full inside a story: its title L4 under the story, its chapters (第一章) L5;
  numbered paragraphs "一、…。text" that run into the paragraph are run-in (text).
- Numbered part markers printed alone on their own line with a block below ("（一）") = L4 nodes (no title to union
  with). OPEN.
- Advertorials framed "·广告·" (university profiles with headline + subheads) = all text (ads). OPEN: they pass
  containment as articles.
- Cartoon box with its own title + comment text = L3 box with its own title.
- Special-supplement page banner (红船初心专刊) with no page tag = L2.
- Photo story title with its own caption text = L3. Place line + headline in one title box (安徽省…县：大马力…) kept
  as one heading text; a place 引题 that is visually a deck (smaller, separate line) = deck.
- Garbled masthead OCR (交的日報) fixed to the printed masthead (光明日報).
- OPEN: 引题 that carries the news subject (李强签署国务院令 / 公布修订后的…办法; 《求是》杂志发表…重要文章 /
  因地制宜发展新质生产力) treated as decks; node text = main title only.

## English papers
- Front-page teaser strips (section names METRO/SPORTS..., skybox teasers "... In News", "SHOOTING FOR MORE ... C1"),
  index boxes (TODAY'S EDITION, INDEX), weather boxes, pointer boxes (COMMENT quote + page, MORE COVERAGE), promos,
  subscription offers, colophon/contact boxes and their field labels, ads (incl. full-page ads and advertorial
  "PAID ADVERTISEMENT" pages) = text.
- The Times "IN THE NEWS" box: every item ends in a page pointer = teaser box (text). OPEN: could be a briefs column.
- Split masthead (THE | crest | TIMES) = one L1 union. Magazine supplement: THE TIMES ∪ MAGAZINE = L1 union; cover
  lines (teasers) text; contents page without a contents title: entries text; a boxed product feature with its own
  title (FAB FIVE: ...) = L3.
- Running masthead in page headers ("THE BOSTON GLOBE" alone) = repeat of L1 (suggest_repeats); folio lines that add the
  date ("THE WASHINGTON POST · SUNDAY...") left as text. Masthead name restated as colophon / editorial-board box title
  = repeat of L1.
- Running section tag: new L2 node when the section changes; `continues` when that section appeared before (Boston Globe
  "The Nation" after "The World"); repeat while it stays open. Running tag over a full-page ad after another section =
  no node (no content).
- Jumps: "CONTINUED FROM PAGE 1 <short head>", one-word jump heads ("Shooting", "Troon"), reprinted or new headlines
  ("X FROM A1" + new headline) = continuation L3 node with the printed head text, `continues` the front-page story.
- Briefs column label (DAILY BRIEFING, MONEYLINE, THE WORLD TODAY) = L3, brief headlines L4. Corrections box
  (CORRECTIONS, For the record) = L3. Puzzle box with own title = L3. Untitled column of short stories = each L3.
- Explainer packages: headline L3; numbered items / each ruling head L4 (also unlabelled siblings in the same style).
- v3 change: interview / FAQ questions printed on their own line with an answer block = L4 headings (USA TODAY Q&A
  "Question: ...", "Can you say more ...?"; jump FAQ questions).
- Kickers in caps above a headline, rubrics inside the text (NEWS ANALYSIS, POLITICAL NOTEBOOK), topic labels on a
  page top under an open section (ELECTION 2024) = text. OPEN: ELECTION 2024 as a section tag.
- Front-page photo teasers ending in a page pointer (BLOOMIN' GROUP ... B1, SHOOTING FOR MORE ... C1) = text.
- Package banner over two front-page stories = L3, stories L4. A jump that reprints the package banner = L3
  continuation of the banner, with the sub-story jump as L4 continuation (Boston Globe). A one-word jump of a package
  story without the banner (USA TODAY "Hope", "Maine") = L3 continuation of the L4 story. OPEN.
- Opinion pages: printed Opinion banner = L2; EDITORIAL rubric and columnist names above a column = text; letters
  column label (INBOX) = L3 with letter headlines L4; masthead restated over the editorial-board box = repeat of L1.
- Guardian Weekly map briefs: numbered location labels ("1 EUROPE") = kickers (text); brief headlines L3 under the
  "Global report" L2 banner. Contents page with no contents title: masthead repeat; page numbers and section entries
  text.

## OPEN cases
1. The Times "IN THE NEWS" box (all items end in page pointers): teaser box (text) vs. briefs column (L3 + L4).
2. Second bold line under a photo-story headline (USA TODAY e4dd9616 #34, ed12985c #46): deck (text).
3. Washington Post "ELECTION 2024" / Boston Globe "CAMPAIGN 2024" page-top topic labels: rubric (text) vs. section tag.
4. Jump of a package story without the banner: L3 (story role on the jump page) vs. L4.
5. Chinese jumps with no printed headline: node on the "（上接第一版）" line with that text vs. copying the original headline.
6. 引题 carrying the news subject (李强签署国务院令; 《求是》杂志发表习近平总书记重要文章): deck (text), node = main title.
7. Advertorials framed "·广告·" and USA TODAY PAID ADVERTISEMENT pages: all text although they pass containment.
8. Numbered part markers printed alone ("（一）" in 光明日报): L4 headings with no title.
9. Running section tag over a full-page ad after another section (USA TODAY 8A NEWS): no node (no content).
10. Magazine and weekly cover lines incl. the main cover story line: teasers (text).

## Breadcrumb review round 2
Checked all 22 newspaper trees (~790 breadcrumb lines read); breadcrumbs.py raised no FLAG on any of them, so no
ACCEPTED entries. review/unseen_pages.txt lists no newspaper pages.
- Fix (containment / level, 1): chn_11ae9c36 #20 "在二〇二三年春节团拜会上的讲话" L3 -> L4 under #12 (中共中央国务院举行春节团拜会
  习近平发表讲话). The speech box is printed inside that story, and the story body (#47-#60) follows the box in
  reading order. As an L3 sibling, the story body fell under the speech node, outside its own story.
- Fix (consistency, 1): eng_105ddbb0 #120, the masthead restated as the contact-box title: not_headings -> repeat of
  L1, the same as in the other papers.
- Checked and kept: the A2 page with no section tag keeps its stories under L1 (105ddbb0). The running tag over p6
  (The Nation) keeps "Putin visits Chechnya" under The Nation as printed (1153fb03). The "EDITORIAL" rubric over the
  reprinted Declaration stays text, and its content sits under Opinion L2 (daf7a3e4 p10). The Guardian Weekly
  "Eyewitness" rubric is text, with "The rat pack" as an L3 photo story.
- Known OPEN items show in breadcrumbs but were not changed: jump nodes titled "（上接第一版）"/"（上接1版）" (OPEN 5;
  ec796be7 has two on p2), 引题-less main titles such as "公布修订后的《对外使用国徽图案的办法》" (OPEN 6), title-less
  "（一）" nodes (OPEN 8), and one-word jump heads that are a columnist name ("Page", ed12985c #329; it continues
  #14).
- Limitation (OPEN 11): the gold reading order cannot express a boxed sidebar printed before the rest of its story's
  body. The trailing story body always falls under the last heading in reading order. Nesting the box under its
  story keeps the breadcrumb inside that story.

## Breadcrumb review round 3
Independent second review of all 22 newspaper trees (~830 breadcrumb lines read). A per-node content dump in reading
order (scratchpad) was used to check containment. breadcrumbs.py raised no FLAG before or after the fixes, so there
are no ACCEPTED entries. review/unseen_pages.txt lists no newspaper pages. check.py gives no ERROR on any tree.
Fixes by class:
- Jump without a printed headline takes the original's title (NOTATIONS "Every path is a breadcrumb"; resolves OPEN 5;
  the `why` records what is printed), 23 nodes. Chinese "（上接第一版）"/"（上接1版）" nodes (6): 4f3edefe #128,
  ccf4163f #137, e542162c #105, eb87830d #89, ec796be7 #122 and #146. USA TODAY jump words (17): ad370234 #66 #106
  #183 #612 #839 #857 #883, c21bfc8c #72 #136 #239 #269, e4dd9616 #87 #172, ed12985c #95 #118 #158 #329 ("Page", the
  columnist's name). Jump heads that are short headlines ("Shoplifters go unpunished", "Edwards pleads guilty") keep
  their printed text.
- Containment, new continuation node on a jump paragraph "（上接第一版）…" with no headline (4). The round-1 rule "jump
  paragraph with no headline or subheads = no node" put the jumped text under the preceding story:
  11ae9c36 #199 (continues #5), adf00537 #128 (continues #4, was considered), e73ccfbf #101 (continues #59),
  ec796be7 #88 (continues #45). Each is L3 under the page section, with the original's title as its text.
- Bare part markers printed alone with no title -> text (resolves OPEN 8), 3: eb87830d #132 #139 #147 "（一）（二）
  （三）". The story "让老兵的故事传遍神州大地" is now a leaf.
- 引题 that names who acts -> part of the node (resolves OPEN 6), 14 nodes. The rule is: a 引题 above the main title
  that names the actor (a person, organization or company whose action the main title reports, or a place line
  ending in "：") goes into the node. When it is a separate box, it is a union. When it shares the box, it is in the
  node text. Separate-box unions: 4f3edefe #94∪#95 (李强签署国务院令), #121∪#122 (陈希…强调), #471∪#472 (天津航空…),
  adf00537 #51∪#52 (刘国中强调), eb87830d #157∪#158 (河南三门峡：). Same box: 4f3edefe #504 (青岛榕信…),
  e542162c #6 (《求是》杂志发表…), #31 (多家银行…), #414 (宁波工业园区——), #424 (江西崇仁…), e73ccfbf #59
  (李克强考察…时强调), eb87830d #277 #322 #340 (place lines). Theme 引题 with no actor ("商品与服务并重…——",
  "提前研判困难…——", "实施已逾两月——", "结束出席…访问" above "习近平回到北京") and ——副题 below the main title stay
  decks.
- Advertorials with their own headline and subheads -> nodes (resolves OPEN 7). As text, their content fell under the
  preceding story and failed containment. The headline is L3 under the page section, as owner ∪ title when an
  institution banner heads it, and the subheads are L4. There are 3 advertorials (3 L3 + 11 L4): eb87830d #216∪#217
  (鞍山师范学院音乐学院, p3 under 评论 as printed), #297∪#298 (湖南农业大学, p4), e4dd9616 #228 (PAID ADVERTISEMENT
  page, p4).
- Kept from round 2: the boxed speech nested under its story (11ae9c36 #20 L4, OPEN 11). A one-word jump of a package
  story stays L3 (OPEN 4, c21bfc8c #72 #136), now with the package story's title.
OPEN (round 3):
11. Gold reading order, which a tree cannot change. 11ae9c36 p2: the headline #94 comes after its intro (#65-#68) and
    after 镜头一–三 in reading order, so that intro falls under the p1 speech node. Page tags and some banners (for
    example daf7a3e4 #542 Opinion) come after the page's content. The tree parents are correct, but content is
    attributed in reading order.
12. Display ads that are not article-shaped (for example the full-page Fisher Investments ad on ad370234 p8 with "RULE
    THE ROAD TO RETIREMENT") stay text. Their content falls under the last story before them, which is a containment
    exception for ads.
13. ——副题 below the main title that names the actor (adf00537 #14 ——农业农村部扎实推进…) stays a deck. Only a 引题
    above the title was folded in.

## Round 4 (full path review)
Every page of all 22 newspaper trees was rendered with the tree drawn and read (119 pages); every node's breadcrumb was
checked against its page (724 nodes before the fixes). Per-node verdicts are in review/paths/newspaper.tsv. check.py
gives no ERROR on any tree. Fixes (2, both by "containment closes subtrees"):
- eng_ad370234 p12 #732 "HIGHLIGHT:INDIANA" L4 under "50 States" -> text. It is a kicker over one photo brief, and the
  50 run-in state briefs that follow in reading order fell into its subtree. Its brief now sits under "50 States" with
  the other state briefs.
- eng_b6de1de8 p2 #9 "The rat pack" L3 under L1 -> text. Its block is a two-line photo caption, and the colophon box
  (p2) and the whole contents page (p3, no contents title) fell into its subtree. The caption now sits under the
  masthead. This replaces the round-2 note that kept it as an L3 photo story.
Checked and kept: front-page teaser boxes (The Times IN THE NEWS, Washington Post teaser strip, Globe front-page teasers)
and full-page display ads still fall under the preceding story in reading order (OPEN 1, 12); package banners with L4
stories (Globe "Healey's message...", USA TODAY "Hope in America") and their jumps; "50 States" stays L3.
