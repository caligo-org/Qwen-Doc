# Magazine learnings

## Level scheme (all magazine docs)
- L1 = masthead on the cover (once). Masthead repeated on colophon/contents page = text.
- L2 = section: a running section tag (header like "海外版|大千世界", vertical 一滴墨香) or a section banner, when the
  section changes from the previous content page. Contents title (目录/CONTENTS) = L2.
- L3 = article headline; L4 = in-article subhead (or L3 where the article is L2, see below).
- Cover lines/teasers, ads (whole ad page incl. its headlines), colophon boxes, logos, TOC entries and TOC section
  labels, run-in 【...】 brief labels = text.
- Node text = as printed: where gold OCR text is wrong (知言 for printed 知音) the node text is the printed text;
  vertical one-char-per-line tags are joined without spaces.

## Decisions
- Article after a printed section whose own section is NOT printed (TOC names a different section): put it directly
  under the masthead at L2 (its subheads L3), since the tree cannot express "unprinted section" after a printed one
  and parenting it under the previous section would be a wrong breadcrumb. (magazine_chn_3b0b9920 p8)
- Briefs page with its own big banner (百科&生活) and listed in the TOC as its own section: L2 section; the
  【label】 lead-ins of the briefs are run-in labels (text).
- Notice boxes in the colophon column (本刊稿件授权声明, 诚招…, 账户信息): text (colophon/contact boxes).

## Open questions
- magazine_chn_3b0b9920 p8: L2 article (section unprinted) vs L3 under the previous section; chose L2.

## 知音·海外版 family (magazine_chn_3b0b9920, _5997ac40, _90f6827e): fixed page pattern
- p1 cover: masthead 知音 L1 (gold OCR text often garbage: write 知音); vertical BOSOM FRIEND header and cover lines = text.
- p2 subscription ad (header 广告): all its titles = text.
- p3 poem page: vertical header 一滴墨香 = L2 section tag, poem title L3.
- p4 BOSOM FRIEND masthead repeat over colophon + TOC column = text. p5 目录 CONTENTS = L2; 知音海外版 logo and the
  three colophon-column boxes (本刊稿件授权声明 / 诚招… / 账户信息) = text.
- p6 header 海外版|大千世界 = L2 section, news items L3; p7 百科&生活 banner = L2 (briefs, run-in labels).
- Later sections: header 海外版|<section> = L2 when the section changes; repeated on following pages = text.
- Article opening on a full-bleed photo page without a section tag (tag only on the next page, after the headline):
  article L2 under the masthead, later tags of the same section = its running heads (text). (_90f6827e p8-10)
- Headline split over two vertical boxes (伊拉克老王： + 跟着味蕾…) = one node with merge.
- Articles with no printed section and nothing in between: L3 directly under the masthead (level skip), e.g. the
  儿童文学 film feature (_6d102faf) whose column label is only a logo figure.

## 知音 (main edition) family (magazine_chn_5b74dff1, _b3948362, _c1e72d14)
- p1 masthead 知音 L1; p2 colophon + 目录 box (logo + 目录 + issue line in one box): node text 目录, L2.
- p3 promo box 知音新媒体矩阵 and 本刊声明 = text.
- Articles carry no section tag (section only in the TOC); since 目录 L2 precedes them they are L2 under the
  masthead with subheads L3 (open question, same as above).
- A series running tag (中华人民共和国成立75周年系列征文) first printed on the article page = L2 section; the editorial
  opener on the previous page without tag = L2 article (_b3948362).

## 故事会 (magazine_chn_f13451d6)
- Inside-cover contest notice and book ads = text. Column tag before the contents (开卷故事, p4) = L2 with its
  story L3. Contents header "812 CONTENTS" = L2 (text CONTENTS). Colophon masthead/labels (发稿编辑…) = text.
- Joke pages: section tag ·笑话· L2; each joke title L3; the facing-page tag (·笑口常开 轻松一刻·) is the same
  section's running tag = text.
- Editorial headline laid out as a figure element (长久的婚姻，都是势均力敌, _d7b9b688) is still a node.

## English magazines
- Full-page and in-page advertisements: every gold title in them = text (Chanel, Howes Models, Billing Boats, …).
- Colophon block (masthead repeat, EDITORIAL/ADVERTISING/SUBSCRIPTIONS labels) = text; "contents" = L2.
- Editor's letter (WELCOME TO …) = L2 directly under the masthead (no section printed).
- Model Boats (_22066f34): top-left tab tags (Signals, SEAL of approval) = L2 sections (same place and style); a
  news column (Compass 360) = L3 briefs label, its items L4; blue rubric banners above item headlines (OUT AND ABOUT,
  BUY THE BOOK, PRIZE DRAW ANNOUNCEMENT) = text (rubrics), so all items stay L4 (same look).
- Open question: "SEAL of approval" may be an article-specific kicker rather than a section; chose L2 by same-look.
- Beijing Review (_2b291ce7): section banners (EDITOR'S DESK, COVER STORY) and the THIS WEEK tag = L2; the
  THIS WEEK <SUB> tags (SOCIETY, ECONOMY, WORLD, PEOPLE & POINTS) = L3 subsections; bare THIS WEEK on facing pages =
  running head (text). Photo stories (big photo heads) and news briefs on a subsection page are siblings at L4
  (gold had the briefs as children of the photo story). Data box (NUMBERS) title = L4. Country labels on photos =
  caption labels (text). Colophon masthead and WRITE TO US box = text.
- Cover lines (even large ones like URBAN UPGRADE) and in-magazine house ads = text.
- Special-issue guide (_571b7df5): L1 = guide title (WINTER GEAR GUIDE); intro, Contents, and front-matter boxes
  with own titles (TRAIL AWARDS, YOUR TESTERS) = L2; tester names = list of names (text); category opener
  (INSULATED JACKETS buying guide) = L2; numbered points: number badge merged with its title, L3.
- Masthead box that also holds the issue banner/date (AWARDS 2025, THE ART ISSUE, issue 220): node text is the
  masthead only. Garbled gold OCR on photo covers: write the printed masthead.
- Baseball America (_7d53ba06): contents page titled by the issue (NOVEMBER/DECEMBER + Leading Off in one box) = L2
  contents; section tag Leading Off = L2 (repeated on a later page of the same section = text); column titles
  (Numbers Game, From Phenom To The Farm) L3; feature headline L4; ranked entries (1. DRAKE BALDWIN …) L5; a
  sidebar inside the feature (THE REST OF OUR PICKS) L5.
- Fusion (_757ea556): editor's letter has no heading (MISSION START! is a sign-off); REVIEW label L2; opinion column
  title (In Good Faith) L2.
- Fine Woodworking Tools & Shops (_86eb2981): L1 = the issue title Tools & Shops (Fine WoodWorking brand logo in a
  header = text); cover-line labels SHOPS/PROJECTS/TOOLS = text; contents spread with no "Contents" word: its
  group labels (features, in every issue) = L2 and the repeated title = text; department banner (Fine WoodWorking
  UNLIMITED) L2, its boxes (Online extras, Additional perks) L3, items L4; VIDEO kicker labels above items = text.
- Open question: contents spreads without a "Contents" heading — group labels as L2 (chosen) vs. making the
  repeated title the contents heading.
- Contributors pages (_a217be3f): "Contributors" = L2; contributor names heading each short bio = list of names
  (text); a second titled block (CONTRIBUTING EDITORS) = L3.
- Info boxes with own titles in the contents sidebar (ON THE COVER, COVER IMAGE, EVENTS AND OFFERS, PRODUCTS ON
  THE COVER) = L3 under Contents; subscription teasers there (Subscribe & save) = text (house ad).
- Contents with a printed contents title: its group labels (Garden makeovers, Best buys, …) = L3 under it.
- Sign-offs of editor's letters (Yours merrily, The … Team; MISSION START!) = text. Order-form labels in ads = text.
- Photo-portfolio magazine (_b2553d3f): page titles (photographer's name on her intro page, CAPTURE THE MOMENT) =
  L2; dancer names under photos = captions (text).
- Masthead printed in two boxes (Total + TVguide) = one node with merge. Headline split across a spread
  (KOREA OPP | ORTUNITIES) = one node with merge, joined without space where a word is split.
- Front-matter page banner holding the editor's note and contents (Your Total TV WEEK) = L2, From the Editor and
  Inside = L3.
- Bookazine guide (_fe444622): L1 = guide title on the cover; title repeated on the credits page and as a part
  opener = text; running tag "INTRODUCTION <SUB>" = L2 each time the sub-tag changes (open question: the two
  INTRODUCTION tags are siblings, the shared part is not expressed); WORKOUT n label + title = merge.

## Open questions (consolidated)
1. Article after a printed section/contents L2 whose own section is not printed: made L2 under the masthead (subheads
   L3) instead of L3 under the previous section (知音 family, Fusion In Good Faith, editor's letters).
2. Section tag first printed on the page after the article's opener (知音海外版 名人明星 p9; 中国力量 series tag;
   本刊视点): opener L2, later tag = running head or L2 section for the following article.
3. Masthead nodes: text set to the printed masthead where gold OCR is garbled or the box also holds date/issue
   banners (知音, TOWN&COUNTRY, Harper's BAZAAR, FUSION, hi-fi+, Modern Gardens …). Is overriding gold text OK?
4. Model Boats "SEAL of approval" tab: section (chosen) or article kicker.
5. Contents spreads without a "Contents" word: group labels L2 (Fine Woodworking) vs L3 under a contents node.
6. Compound running tags (INTRODUCTION + WORKOUT STRUCTURE / ANATOMY; THIS WEEK + SOCIETY): one L2 per sub-tag change
   in the guide, but L2 THIS WEEK + L3 subsections in Beijing Review where the bare section tag is printed first.
7. Contributor/tester names heading short bios: treated as lists of names (text).
8. Colophon-column notice boxes (本刊稿件授权声明, 账户信息, 本刊声明) treated as colophon text, not
   disclaimer-section headings.

## Label rule pass
- 知音 colophon promo box (知音新媒体矩阵): channel names with colons (知音公众号: / 知音微刊： / 知音真实故事： / 知音网课：)
  each beside a QR code = house-ad entries, text (all 4 issues, 16 candidates, no change).
