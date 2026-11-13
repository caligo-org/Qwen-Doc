# Magazine learnings (v3, from scratch)

## Level scheme
- L1 = masthead on the cover (printed masthead text where gold OCR is garbled, or where the box also holds the issue
  date/badges/price: node text = masthead only). Masthead printed in two boxes (Total + TVguide) = one union.
- Special issues / bookazines: L1 = owner ∪ document type: brand masthead ∪ issue/guide title
  (Fine WoodWorking ∪ Tools & Shops; BUILD ROCK HARD ABS … ∪ Men's Fitness GUIDE; FROM THE MAKERS OF TRAIL MAGAZINE ∪
  WINTER GEAR GUIDE — the colophon restates it as TRAIL MAGAZINE WINTER GEAR GUIDE).
- L2 = section: section banner or running section tag when the section changes from the previous content page;
  contents title = L2; front matter (editor's letter, introduction, Contributors) = L2.
- L3 = story headline; L4 = in-story subhead. Briefs column label (Compass 360) = story level, items one below.
- Text: ads (whole ad pages incl. product headlines and order-form labels; event posters inside images), cover lines,
  kickers/rubrics above headlines (VIDEO, OUT AND ABOUT, BUY THE BOOK, ENTERTAINMENT), TOC entries, run-in
  【...】 brief labels, colophon labels and boxes, contact/social boxes, editor sign-offs, captions.

## Repeats
- Masthead / title restated over a colophon column, credits page, contents spread or part opener = repeat of L1.
- Running headers and running footers that name the magazine (± issue date) = repeats of L1 (FUSION, Model Boats •
  November 2025, BEIJING REVIEW DECEMBER 4, 2025, 知\音\海\外\版 vertical header). Footers that are only a URL
  (gardensillustrated.com, BASEBALLAMERICA.COM) or a sponsor line = text.
- Same section tag on a later page of the open section = repeat of the section node (Signals, Leading Off, bare
  THIS WEEK). Social handles under icons (FOURFOURTWO) = considered text.

## Decisions
- Story whose section is not printed, after a printed section/contents L2: story takes the L2 slot (subheads L3).
  An L3 would become a child of the previous section (wrong breadcrumb). When only the masthead is open and no
  sibling L2 exists (儿童文学 film feature, column only a logo figure) the story is L3 (level skip).
- Section tag first printed AFTER the opener of the SAME story (名人明星 on p9 of a story opening on p8): story holds
  the L2 slot; the later tags = considered text. Section tag first printed on a NEW story's page after an untagged
  editorial (中华人民共和国成立75周年系列征文, 本刊视点): tag = L2 section, the editorial before it = L2 story.
- Facing-page running tag of the same section with different wording (·笑口常开 轻松一刻· after ·笑话·) = considered.
- Compound tags: bare tag first (THIS WEEK) = L2, each compound tag (THIS WEEK SOCIETY/ECONOMY/WORLD/PEOPLE &
  POINTS) = L3 subsection; photo stories and briefs inside are siblings (L4); data box (NUMBERS) = L4.
  Where the bare part name is never printed alone (INTRODUCTION WORKOUT STRUCTURE / INTRODUCTION ANATOMY in one box)
  each compound tag = L2 when the sub-tag changes.
- Contents: printed contents title = L2; contents group headings in their own boxes (Garden makeovers, Best buys …)
  = L3 under it; group labels merged into TOC text blocks stay text. A contents spread with no contents title:
  its group headings (features, in every issue) = L2. Info boxes with own titles in the contents sidebar (ON THE
  COVER, COVER IMAGE, PRODUCTS ON THE COVER) = L3; subscription teasers there = text.
- Contents box that also holds the logo and issue line (知音 目录 2024年…): node text = 目录 only. A contents page
  headed only by the issue date + front section name (NOVEMBER/DECEMBER Leading Off) = contents title L2.
- Front-matter banner holding the editor's note and contents (Your Total TV WEEK) = L2; From the Editor, Inside = L3.
- Front-matter page boxes with own titles on the same page as the contents (TRAIL AWARDS, YOUR TESTERS) = L2.
- Person names heading their own bio paragraph (contributors, testers, contributing editors) = headings one level
  below their block title (own-line label with its own block). Dancer names under photos = captions (text).
- Story headline laid out as a figure element (长久的婚姻，都是势均力敌) is still a node. Gold OCR misreads in
  headlines: write the printed text when clearly legible (冲天辩 → 冲天辫, FUSIOH → FUSION).
- Headline split over boxes (伊拉克老王： + 跟着味蕾…; KOREA OPP | ORTUNITIES) = one union; a word split across
  boxes is joined without space. Number badge ∪ title (1 + TYPE OF INSULATION; WORKOUT 1 + ABS AND LEGS) = union.
- Ranked entries with write-ups (1. DRAKE BALDWIN …) = one level below the feature; sidebar in the feature (THE
  REST OF OUR PICKS) = same level as its entries (one below the feature). Cartoon column title (Trap Toon) = story.
- Joke pages: section tag ·笑话· L2, each joke title with its joke L3.
- Colophon-column notice boxes (本刊稿件授权声明, 诚招…, 账户信息, 本刊声明) and the new-media promo box
  (知音新媒体矩阵) = text.
- Photo-portfolio magazine: page titles (photographer's name on her intro page, CAPTURE THE MOMENT) = L2.

## OPEN
1. Untagged story after a printed L2 (section slot L2 vs an L3 that would sit under the wrong section); the tree
   cannot express "unprinted section after a printed one".
2. Untagged editorial opener that topically belongs to the series/section whose tag is first printed on the next
   story (中国速度 before 中华人民共和国成立75周年系列征文; 长久的婚姻… before 本刊视点): kept as a sibling L2.
3. 本刊稿件授权声明 / 本刊声明 have their own statement blocks (could pass as disclaimer-section titles); kept as
   colophon text.
4. Compound tags with the part name never printed alone (INTRODUCTION + sub-tag): two sibling L2 nodes; the shared
   part is not expressed.
5. Contents spread without a Contents word: group headings as L2 vs the restated title as contents heading.
6. Owner lines that read as provenance (FROM THE MAKERS OF TRAIL MAGAZINE) joined into the L1 union.
7. Rubric banners with an item below (OUT AND ABOUT, BUY THE BOOK, PRIZE DRAW ANNOUNCEMENT, VIDEO) treated as
   kickers, not labels with their own block.

## Breadcrumb review round 2
Checked all 25 docs, 258 breadcrumb lines; no automatic FLAG in any doc (before or after). All 67 unseen magazine pages
viewed (review/*_tree.jpg).

Fixes by class:
- Step not narrower / section-tag-after-opener (2): 知音 feature pages whose contents entry lists the opener as THE
  feature of the section (本刊视点 › 长久的婚姻… p8; 中华人民共和国成立75周年系列征文 › 中国速度 p4, next entry p10/p17).
  The tag printed first on the following page belongs to the same feature: tag -> considered, the case story below
  becomes L3 under the feature (d7b9b688 #136, b3948362 #60). This resolves OPEN 2 for these two docs: the contents
  page decides that the opener and the next story are one feature.
- Title union (2): 知音 海外版 masthead whose edition logo 海外版 is a separate figure box (3b0b9920, 90f6827e) -> L1 =
  知音 ∪ 海外版, same L1 as 5997ac40 where one box holds both.
- Node text not a description / redundant prefix (4): 812 CONTENTS -> CONTENTS (issue number in the contents box,
  f13451d6); running section tags printed as 海外版 | 大千世界 / 海外版本刊特稿 -> 大千世界 / 本刊特稿 (edition prefix repeats
  the L1; contents lists the bare section name).
- Recorded (no tree change): Harper's BAZAAR p6 MEGHAN, Duchess of Sussex + photo/styling credits = cover credit box,
  considered text inside THE ART ISSUE.

Flags accepted: none (no flags raised).

OPEN:
8. Contents page titled by issue + front section name (Baseball America NOVEMBER/DECEMBER Leading Off) holds ON THE
   COVER under it, then the Leading Off tag on p4 opens a second L2: kept per the round-1 decision; the alternative
   (one Leading Off L2 opened by the contents page) would put the colophon and Numbers Game under the contents title.
9. SEAL of approval (Model Boats p10) is treated as a section tag above Brown-Water Navy Revival; it could also be a
   punning kicker of the story.

## Breadcrumb review round 3
Independent second review. Checked all 25 docs, 229 breadcrumb lines read; no automatic FLAG in any doc before or after.
All 67 unseen magazine pages re-rendered (review/*_tree.jpg) and viewed; nothing missing (ads, covers, contents
pages, photo/portrait pages with captions only).

Fixes by class:
- Same section opened twice (1): Baseball America (7d53ba06). The contents page is the opening page of the front
  section Leading Off (its box prints the issue date + Leading Off); the p4 tag Leading Off is a repeat of it, not a
  second L2. #49 -> repeat of #16; Numbers Game, From Phenom To The Farm, Trap Toon now under #16; node text
  "Leading Off" only (issue date NOVEMBER/DECEMBER in the same box is text, like a masthead box). ON THE COVER stays
  L3 in that section. Resolves OPEN 8.
- Slogan/pun as a section step (1): Model Boats (22066f34) "SEAL of approval" is a pun on the one story (SEAL support
  craft), not a section the contents knows -> considered text; Brown-Water Navy Revival takes the L2 story slot
  (unprinted section after the printed Signals), its subheads L3. Resolves OPEN 9.
- Containment inside a box (1): Men's Fitness (fe444622) TEMPO EXPLAINED was L5 under SUPERSET STRUCTURE; tempo is
  not part of the superset structure -> L4 sibling under SESSION BREAKDOWN. Node text SUPerset -> SUPERSET (OCR).
- Checked, no change: every untagged story kept at L2 after a printed section (张朝阳…, 陈道明…, 『冲天辫』老妈…,
  INSULATED JACKETS buying guide, In Good Faith, KOREA OPPORTUNITIES, Brown-Water Navy Revival): each path reads
  masthead › story › subhead; no story sits under a section it is not in. Resolves OPEN 1 (decision: an untagged
  story after a printed section is an L2 sibling of that section; the tree does not express the unprinted section).
- Compound running tags (INTRODUCTION WORKOUT STRUCTURE / INTRODUCTION ANATOMY): each reads as one step
  "Introduction: <sub-tag>", narrower than the guide title, and its subtree is about it; kept as two L2 nodes.
  Resolves OPEN 4 for paths (the shared part name is printed only in the same box, so no INTRODUCTION node can exist).
- Men's Fitness cover union (BUILD ROCK HARD ABS IN JUST 4 WEEKS ∪ Men's Fitness GUIDE): owner ∪ document type as the
  rule asks, in reading order; kept.

Flags accepted: none (no flags raised).

OPEN (no path is inconsistent; these are labelling choices only): 2 (resolved for the two 知音 docs in round 2),
3, 5, 6, 7 as above.

## Round 4 (full path review)
Every node of all 25 magazine docs checked on its page (all 165 pages rendered with the tree into
review/paths/magazine_tmp/ and viewed). 229 nodes: 223 OK, 6 FIXED, 0 ADDED, 0 REMOVED. Lines in
review/paths/magazine.tsv. No ERROR from check.py.

Fixes (node text only; no level or structure change):
- Stray decoration in node text (1): 故事会 (f13451d6) #87 笑话· -> 笑话 (the dots of ·笑话· are decoration; the
  contents lists 笑话 13 则).
- Logo without word gap (1): Beijing Review (2b291ce7) L1 BEIJINGREVIEW -> BEIJING REVIEW (running heads and colophon
  print BEIJING REVIEW; same class as FUSIOH -> FUSION).
- Compound tag restating its parent (4): Beijing Review THIS WEEK SOCIETY / ECONOMY / WORLD / PEOPLE & POINTS under
  the THIS WEEK node #55 -> SOCIETY / ECONOMY / WORLD / PEOPLE & POINTS. The path read "THIS WEEK › THIS WEEK SOCIETY";
  the THIS WEEK part restates the parent node, like the edition prefix 海外版 restated the L1 in round 2. Where the
  part name has no node of its own (Men's Fitness INTRODUCTION WORKOUT STRUCTURE / INTRODUCTION ANATOMY) the full
  compound text stays, as decided in round 3.

OPEN:
10. FUSION (757ea556): the p3 editor's letter has no printed heading of its own (only the masthead restated over the
    colophon column, a repeat of L1), so its text falls under CONTENTS (p2) until REVIEW (p4). Kept; the restated
    masthead as the editor's-page heading would contradict the repeat rule.
