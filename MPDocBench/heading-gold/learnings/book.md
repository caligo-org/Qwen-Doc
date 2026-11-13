# book learnings (v3)

## Decisions
- Title (L1) of a novel / trade book = the book title on the cover (with a genre word in the same box). The author
  name is text, like paper authors (the book title names the document by itself; the author is the creator, not an
  owner + generic document type). Series lines, bestseller lines, taglines, review quotes, publisher logos: text.
- Title page / second cover image / title restated on the copyright page ("Angry God / L.J. Shen") = repeat of L1.
  Author name alone on the title page = text.
- Publisher ebook inserts ("Thank you for downloading...", "CLICK HERE TO SIGN UP") = ads: text.
- Front matter page titles = L2 like chapters: Contents, Dedication (when printed as a title), By the same author /
  Also by, Playlist, Theme Song. Series names over their lists inside an "Also by" page = L3 (label + own block).
- Copyright-page cataloguing lines (Library of Congress / Archives Canada CIP label), dedication lines: text.
- Back-cover tagline over the blurb (a slogan also printed on the cover): text.
- Chapter label + number/title = one heading (one box or a union), L2.
- POV narrator labels under a chapter title ("GRACE", "Lenora") = L3 headings: own-line label with its own block
  (v3 label rule), naming whose narration follows. Age/date line under it ("Lenora, 12; Vaughn, 13") is text.
- Node text: where OCR garbled or dropped a script-font word on a cover, text is written as printed.

## OPEN
- Author as part of the title union: decided text (paper analogy). If the user means "owner" = author for books,
  every book cover L1 would become author ∪ title.
- Course / lab manuals issued by an institution: L1 = institution ∪ manual title when printed together on the cover
  (owner + document type; b5a23ac1). Publisher brand logos on tutorial covers (tutorialspoint) are text, the subject
  title alone is L1 (0f8b3268). An acronym printed on the cover as part of the title (BMPs, also in the running
  footer) joins the union (04edb0ff).
- Running heads / footers naming the document = repeats of L1; running section tags (top headers, side tabs) =
  repeats of the open section. A side tab printed on the section's first page before the heading in reading order
  cannot be a repeat (section not open yet): left in `considered`.
- Titled sidebar / box titles (dated callout box with its own paragraph, 'Weak and Strong AI' box, a box heading a
  list) = one level below the section they sit in (21695dfc).
- Content beats look: a smaller bold head under a larger subsection whose block is about that subsection's topic is
  one level below it (0f8b3268 'What a Basketball Player Should not Do?' under 'Rules of the Game').
- A chapter number printed alone at the head of the chapter (header box "1", "2") is the chapter heading when the
  chapter has no title (10fbf0e9). A dedication heading line ("TO LEON WERTH") over a dedication paragraph = L2
  front matter; the restated dedication line closing that block = text.
- Book excerpt with no printed title (book title only in running heads): the stories are L1 nodes; the book-title
  running heads go to `considered` (nothing to repeat); story running heads = repeats of the open story (600ba37c).
- Titled exercise boxes that repeat in every story (Before Reading: / After Reading: + their questions) are headings
  one level below the story (v3 label rule; v2 had them as text).
- A section title restated at the top of every page while the section is still open (tutorial layouts) = repeat
  (v2 had continuation nodes); a subsection title restated on the next page, with or without "(continued)", = repeat
  (82b4201c). Tool / item labels with their own paragraph under a subsection = one level below it.
- KEY POINTS / KEY POINT summary boxes with their own bullet block = headings (v3 label rule; v2 had text), one level
  below the section whose content they summarize; when the box is printed beside a new section heading, its content
  decides the section (77552588 #33 under 1.2). Node order within a page may follow content, not box id.
- Chapter excerpt of a larger book: L1 = chapter title; even running heads with the book title -> `considered`; odd
  running heads with the chapter title = repeats (77552588).
- Mixed-case bold heads one level below uppercase heads; in sections with no uppercase head they sit directly under
  the numbered section (no skip) (77552588, same as v2).
- Pull-quote rule boxes (one-sentence callouts between rules), verse quotations: text.
- Sections whose body lists items that then get their own headings in the same style (Value of Landscaping ->
  Aesthetic/Economic/...; Steps in Developing a Landscape Plan -> Step 1..6): the item headings sit one level below
  the listing section (content beats look, a59ff1e5). Checklist titles with their numbered list = headings.
- OPEN (carried from v2): a59ff1e5 p5 'Locating House on Property', 'Locating Outlying Landscape Features' and p19
  'Plant Palette' are headings with no box of their own (OCR folded them into paragraphs); cannot be labelled.
- Issuer printed as an imprint block at the bottom of a handbook cover (separate from the title block) = text; issuer
  printed as part of the title block = union (b5a23ac1 vs ab95f5db). OPEN: the line between the two is look-based.
- Inside-cover organisation boxes (org name + MISSION STATEMENT + paragraph) = L2 front matter box titles with the
  MISSION STATEMENT label one level below (v2 had text) (ab95f5db).
- Transcript speaker labels on their own line with the speaker's answer block below ("(Family One - husband)") =
  headings one level below the transcript heading, like POV labels (v2 had text). Source-excerpt labels ("From the
  ... website (date):") one level below their section; a quoted document's title inside the excerpt one below that.
- Titled strategy boxes ("Strategy: X" + Involves/Reason lines) = box titles one level below their section.
- OPEN: c5211a9e 'INTERMEDIATE:' / 'ADVANCED:' labels each over a single sequence line: kept text (substance: the
  block is one line, like a list-item label), although the v3 label rule could make them headings.
- Owner on a cover: a heading-style block (department/college printed large) joins the L1 union with the title
  (dc66edfa: Computer Networks Lecture Notes ∪ department/college); a small imprint line or logo stays text.
- Advantages: / Disadvantages: own-line labels with their list = headings (v3 label rule; supersedes the v2 user
  decision that had them as text); placed one level below the section they sit in. A label restated on the very next
  line ("Metropolitan Area Network (MAN):" then "Metropolitan Area Network:") = repeat.
- Run-in labels inside a paragraph box (Mesh:, Ring Topology, Text:) have no box of their own: text.
- e343e570: scenario titles sit one level below 'DESIGN SCENARIOS' (the section that introduces them; content beats
  the banner look; resolves the v2 OPEN). CHALLENGES:/GOAL: box labels and SOLUTION n labels = headings below the
  scenario (v3 label rule); 'Plant Characteristics to Look For:' / 'Design Solutions:' + list one level below GOAL
  (inside its box). Inside-cover banners = L2 front matter; the program-office contact box title = text.
- Self-help books (f2e8d789): list lead-in sentences ending in a colon, pull quotes and "Did You Know?" quote bands =
  text; "In This Chapter, You'll Learn:" objectives box = box title heading (v3 label rule, like KEY POINTS);
  numbered pillar badges + definition labels with their bullet block = item-heading unions one level below the
  section that lists them. OPEN: whether objective boxes that merely restate the chapter TOC have "substance".
- Chinese textbook chapter excerpts: L1 = 第x章 ∪ chapter title; 提要 box and 第x节 = L2; 一、 L3; （一） L4. Running
  heads "第九章 title" = repeats of L1. Numbered run-in items ("1. 耳前淋巴结 位于...") = text.
- OPEN (v2 carried): 0875d4ae own-line '4岁以前乳牙殆特征' / '4~6岁期间乳牙殆特征是：' sit inside run-in item 1. and are kept as headings below （二）; '2. 面部协调关系' own line kept as heading while sibling '1. 鼻翼耳屏线' is run-in.
- Exam-prep law books (38c7e1dc): no cover, L1 = the part (第一部分 公司法); 专题 L2; 考点 L3; 命题点拨 / 关键词记忆 /
  小案例训练 (小案例分析) = L4 label headings (v3 label rule, supersedes v2 text for 命题点拨); 一、 L5; （一）/own-line
  numbered heads L6. 【...】 callout labels and case questions with their answers (问/答) = text. Own-line （二）/（三）
  heads are headings even when sibling （一） is run-in.
- Case-training sets (a0187360): case headings L2; ***参考答案与重点法条*** / ***本题涉及的重点考点*** L3; each question restated
  in the answer section with its own answer block = L4 heading (question with its own answer block, like FAQ;
  supersedes v2 text). The question list under the case facts stays text (exam questions, no answer there).
- OPEN: 【黄文涛参考答案】/【相关重点法条】/【法考点睛】 bracket labels each heading a block: kept text as callout labels
  (like 注意/Tip); the v3 repeating-label rule could make them headings.
- OPEN: 38c7e1dc 小案例训练 numbered case questions followed by '答：' answers kept text (the question is itself a long case paragraph, not a label; some answers are footnotes); differs from a0187360 restated questions.
- dd57800f (law exam-prep): 序言 section is a sibling of the 一、 sections (v2 rule kept). Own-line [萌主点拨] box
  labels heading their own table/diagram = headings one level below; run-in [萌主点拨] at a paragraph start = text.
  OPEN: '(1) 没有继承人。' style own-line numbered items each with a following paragraph kept text (v2), arguably
  item headings under v3.
- dbd95bcd: 【本章主要法律规定】 with its statute list is a chapter-level section (same level as 第x节) in every chapter, including ch.1 where the gold had it under 第二节. Running heads are publisher ads (text).

## Breadcrumb review round 2
- Checked all 33 book docs (every breadcrumb line read, ~1,000 paths); no book pages in review/unseen_pages.txt.
- Fix class: containment, run-in sibling items (3 docs, 9 nodes -> text). An own-line item label followed by
  run-in sibling items (same list, label inside the paragraph box) gets the run-in items in its subtree. When no own
  box exists for the run-in items, the whole list is text under the section:
  69dd792e #354/#365/#378 ('1.', '2.', '6.' with run-in 3.-5. and 7.-9.); 0875d4ae #247/#257 (own-line labels inside
  run-in item 1., followed by run-in items 2./3.; this resolves the v2 OPEN); dc66edfa #81/#84/#88/#95 (Mesh: and Ring
  Topology are run-in, so Advantages:/Disadvantages: read as advantages of physical topology and Ring sat under Bus
  Topology). Run-in siblings that come before an own-line item do not break containment (0875d4ae '2. 面部协调关系',
  69dd792e '4. 神经分布和支配', 38c7e1dc （二）/（三）: kept).
- Fix class: title union (1): 3ef07034 L1 = 'Cooking Around the World' ∪ the program line printed inside the same
  title band (issuer in the title block); agency logos at the foot of the cover stay text.
- Fix class: node text as printed (5 docs, 18 nodes): 9c2fed1c 'SAVING the CEO'; e343e570 'Landscaping™' (OCR '?');
  0875d4ae '𬌗' (OCR '?'); 38c7e1dc / dd57800f underline markup removed.
- Kept: sidebar / band boxes (21695dfc 1956 box, 77552588 KEY POINTS, 3ef07034 Nutrition Facts band) bound their own
  content; main-column text that follows them in reading order is not in their subtree.
- Kept: document's own numbering slips (82b4201c letters c., d. reused under II; 0f8b3268 'Basketball - Tournaments'
  printed as a subsection of 6. Variants with the same look).
- ACCEPTED book_eng_600ba37c #29 ROOTS: excerpt with no printed book title; the four stories are the top level (round-1 decision).
- ACCEPTED book_chn_dd57800f #5 FRONT: '序言：物权变动的两大基础公式' is the opening section of 第一节 (sibling of 一、), not front matter.
- ACCEPTED (no flag) bare-number nodes: 10fbf0e9 chapters '1'..'8' (no chapter titles printed), a0187360 '案例一：'
  case labels (no case titles printed).
- OPEN: the run-in rule above turns some bold own-line labels (dc66edfa 'Star Topology:') into text only because a
  later sibling is run-in; the alternative is to keep them and accept a broken containment.

## Breadcrumb review round 3
- Independent second review of all 33 book docs: every breadcrumb line read (860 paths before fixes, 827 after); no
  book pages in review/unseen_pages.txt. Pages viewed for every decision below.
- Fix class: bare marker with no title -> text, with the containment cascade (2 docs, 31 nodes). NOTATIONS "Every path
  is a breadcrumb": a bare marker is a union with the title it labels, or text. Supersedes the round-2 ACCEPTED bare
  numbers.
  - 10fbf0e9: chapter numbers I-VIII (no chapter titles printed) -> text; then 'TO LEON WERTH' would hold the whole
    book in its subtree (no later heading closes it) -> text. Tree = L1 only.
  - a0187360: 案例一：/案例二：/案例三 (no case titles; the TOC prints only the labels) -> text. Cascade: each case's
    本题涉及的重点考点 and its restated questions and 参考答案与重点法条 of cases 1-2 would hold the next case -> text;
    目录 would hold the cases -> text. Only the last case's 参考答案与重点法条 (L2) and its 7 restated questions (L3)
    pass containment (they run to the end of the excerpt). Markdown bold markup removed from the label text.
- Fix class: bare chapter label ∪ narrator line (2 docs): 9e4de927 'Chapter ONE' ∪ 'Lenora', d4818a54 'CHAPTER 1' ∪
  'GRACE'. The chapter opener prints the POV name under the label as the chapter's only title line; the excerpt ends
  inside the chapter. Supersedes the round-1 POV-label L3 node for these two (the age line stays text).
- Fix class: wrong parent by content (2): 0f8b3268 'Basketball - Tournaments' (not a variant) -> L2 between chapters 6
  and 7 (supersedes round-2 "kept: numbering slip"); 77552588 'Hospital library' (not a round) -> L3 under 1.4
  EDUCATION.
- Fix class: node text = heading part only (2 docs, 8 nodes): f2e8d789 'Introduction' (byline in the same box);
  e343e570 SCENARIO A-G titles without the "Two/Three design options (...)" deck printed in the same box.
- Fix class: node text as printed (2): 69dd792e / 0875d4ae '提要' (OCR '【提要' / '-[提要]-' from the dashed frame).
- Checked, kept (containment): framed sidebar boxes bind only their own content; the text parents of the main column
  come from the MPDocBench relations (main text under the section, not under the box), and the heading order puts each
  box where the main flow reaches a heading. 21695dfc '1956 - when the term AI was first coined.' and 'Weak and Strong
  AI', 77552588 all KEY POINTS boxes (each one placed where its content belongs: #33 under 1.2, #117 after Listening,
  #120 before Meetings).
- ACCEPTED 3ef07034 Recipe Notes / Directions (#103, #204 etc.): the Nutrition Facts band below them is a framed
  page-bottom band repeated on every recipe page (with the "For more recipes" line); its MPDocBench parent is the last
  label of the recipe. Making the labels text would break every recipe; the band is page furniture of the recipe.
- dc66edfa verified on pp. 8-12: 'Mesh:' and 'Ring Topology' are run-in (inside paragraph boxes); 'Advantages:' /
  'Disadvantages:' are mesh's, so as headings the path would read "Physical Topology > Advantages"; 'Bus Topology:'
  would hold the run-in ring section; 'Star Topology:' would then hold the bus content. All text (round 2 kept). The
  round-2 OPEN on 'Star Topology:' is resolved by this cascade.
- Kept: 82b4201c c.-f. under II (the TOC gives II pages 5-11, the interface pages are inside it); 600ba37c ROOTS and
  dd57800f FRONT as in round 2.
- ACCEPTED book_eng_600ba37c #29 ROOTS: excerpt with no printed book title; the four stories are the top level.
- ACCEPTED book_chn_dd57800f #5 FRONT: '序言：物权变动的两大基础公式' opens 第一节 (sibling of 一、), not front matter.
- OPEN: the bare-marker rule removes all structure from 10fbf0e9 (L1 only) and almost all from a0187360 (only the last
  case's answers remain, at a level that no longer names the case). The alternative (keep an untitled chapter/case
  label as a heading because it delimits a real unit) was the round-2 decision; the user should confirm which.

## User decisions (2026-11-13)
- Bare marker that opens a real unit with no printed title is a heading on its own (NOTATIONS "Every path is a
  breadcrumb"). Supersedes the round-3 "bare marker -> text" fix and its containment cascade.
  - 10fbf0e9: chapter numbers '1'..'8' (header boxes #8 #27 #71 #106 #127 #155 #171 #209, Arabic numerals) = L2 under
    L1 again. 'TO LEON WERTH' #4 = L2 front matter again (chapter 1 now closes the dedication); the restated
    dedication line #6 stays text. Tree: L1 + dedication + 8 chapters.
  - a0187360: 目录 #7 L2; 案例一：#17 / 案例二：#120 / 案例三 #196 L2; each case's 参考答案与重点法条 (#38 #131 #213) L3
    under its case, so the breadcrumb names the case ("… › 案例二： › 参考答案与重点法条"); restated questions (8 + 6 + 7)
    L4 under it; 本题涉及的重点考点 (#110 #182) L3 under its case (closed by the next case). All pass containment once
    the case labels are headings. Markdown bold markup stays out of the label text.
- Own-line label headings stay headings when a later sibling is run in (NOTATIONS exception); the run-in sibling's
  content sits under the previous label. Supersedes the round-2/3 cascade for dc66edfa.
  - dc66edfa: 'Star Topology:' #88 and 'Bus Topology:' #95 = L6 under 1.2.3.1 Physical Topology (Ring Topology, run-in
    in #104, falls under Bus Topology). 'Advantages:' / 'Disadvantages:' (#81 #84, the run-in Mesh item's) stay text.
    The other label headings (Numbers:…Video:, Simplex:…Full-Duplex:, Performance:…Security:, Type of Connection and
    its items, Local Area Networks: L5 under 1.2.4, MAN, WAN) were already headings; their `why` now names the decision.
- The round-3 OPEN on bare markers is closed by this decision.

## Round 4 (full path review, 2026-11-13)
- All 33 book docs, all 449 pages viewed with the tree drawn; 864 node lines in review/paths/book.tsv: 856 OK, 4 FIXED, 4 ADDED, 0 REMOVED.
- 0875d4ae ADDED #145 '4. 后牙覆𬌗、覆盖关系分类（图3-20）' L5 under （二）唇（颊）舌向的关系: own-line numbered item in its own box with its (1)-(n) block, closed by （三）; items 1.-3. are run in. Same treatment as #311 '2. 面部协调关系' and 69dd792e #126.
- OPEN 0875d4ae #247 '4岁以前乳牙𬌗特征' / #257 '4~6岁期间乳牙𬌗特征是：' kept text: own-line labels nested inside run-in item '1. 乳牙𬌗特征'; the run-in '2. 混牙𬌗特征' that follows belongs to the outer list, so the user's run-in-sibling exception (same list) does not cover them and their subtree would absorb item 2.
- 69dd792e ADDED #354 '1. 唾液腺导管', #365 '2. 舌神经和舌下神经', #378 '6. 舌动脉' as L4 under 一、重要的解剖结构与外科手术 (own-line numbered items with their (1)-(n) blocks; 3.-5. and 7.-9. are run in and fall under the previous own-line item). Applies the 2026-11-13 user decision (run-in sibling exception) that superseded the round-2 containment fix; round 2 had made these text. SEQ flag 2. -> 6. is expected.
- dd57800f FIXED node text as printed: #64 '（2）预告登记★★', #90 '（3）动产交付★★' (OCR half-width opening bracket; the page prints the full-width form, like sibling #33).
- 04edb0ff #1 L1 text as printed: 'BMPs' (OCR 'BMPS'); union unchanged.
- 9d008b33 (round 4): OPEN: novel excerpt whose body (pp6-8) prints no chapter heading; the body falls under 'Contents', and the copyright page and dedication fall under 'By the same author'. Containment would make both front-matter titles text; kept as headings (front matter titles, NOTATIONS "Usually headings") pending a user decision.
- 9e4de927 (round 4): OPEN: the untitled epigraph page (p6) falls under 'Table of Contents'; same question as 9d008b33 (front-matter title followed by untitled front matter). Kept.
- a59ff1e5 (round 4, all 19 pages seen): no change. OPEN (no box, cannot be labelled): besides p5 'Locating House on Property' / 'Locating Outlying Landscape Features' and p19 'Plant Palette', p18 'Pulling it all together' is an own-line head with no box; its two paragraphs (#261 #262) fall under 'Accenting the main entrance'.
- e343e570 #147: node text 'SCENARIO C: ALONG SIDEWALKs' -> 'SCENARIO C: ALONG SIDEWALKS' (printed all caps; OCR case slip). FIXED.
- OPEN e343e570 p17 (SCENARIO F): the 'Design Solutions:' label is printed inside bullet box #257 (no box of its own), so the three solution bullets fall under 'Plant Characteristics to Look For:' (#254). Cannot be labelled; same class as the a59ff1e5 unboxed heads.

## Round 5 (breaks)
- All 33 book docs, all 449 pages viewed with `show.py --tree`; 44 breaks in 21 docs, 2 nodes restored, 12 docs with neither. review/breaks/book.tsv has one line per break / restored node / NONE doc.
- Front matter followed by untitled front matter or body (resolves the round-4 OPEN on 9d008b33 / 9e4de927): a break closes the front-matter title (9d008b33 'By the same author' before the copyright page, 'Contents' before the novel text; 9e4de927 'Table of Contents' before the epigraph; 4ba599d0 'ALSO BY' before the title page; a0187360 目录 before the author bio / 重要说明).
- Label box ends, main text resumes: 600ba37c Before Reading boxes (4), 21695dfc '1956' sidebar, 04edb0ff 'Practical Recommendations' box, 77552588 KEY POINT(S) boxes (3), f2e8d789 objectives box, f0c264a5 提要 frame (chapter intro outside the frame), dd57800f [萌主点拨] box (inheritance diagram after it), 38c7e1dc 【指导案例】 after （三）.
- Closing paragraph about a whole list: 0f8b3268 (rules), b5a23ac1 (safety guidelines), c5211a9e (principles 1.-4.), f2e8d789 (3 pillars), a59ff1e5 checklist lead-ins (2).
- 3ef07034: the framed Nutrition Facts band of every recipe is closed off from the last label by a break (12 recipes); supersedes the round-3 ACCEPTED note (band as furniture under the last label).
- Printed heads with no box of their own (round-4 OPENs): a59ff1e5 'Locating House on Property' (p5), 'Pulling it all together' (p18), 'Plant Palette' (p19); e343e570 SCENARIO F 'Design Solutions:' (closes GOAL). The heads themselves still cannot be nodes.
- Run-in siblings after an own-line label (NOTATIONS update): dc66edfa break before run-in 'Ring Topology' (closes 'Bus Topology:'); 69dd792e breaks before run-in 3. (closes '2. 舌神经和舌下神经') and 7. (closes '6. 舌动脉').
- RESTORED 0875d4ae #247 '4岁以前乳牙𬌗特征' and #257 '4~6岁期间乳牙𬌗特征是：' as L5 under （二）不同发育阶段𬌗特征 (were text for containment only); break before the run-in '2. 混牙𬌗特征' (#267) closes #257. Resolves the round-4 OPEN.
- 77552588: the node list keeps content order (KEY POINTS boxes placed where their content belongs), so breaks were placed against that order (#17 needs none: main text on p2 already sits under 'You, your staff...').
- Kept without a break (user decision on blocks inside a section): floating figures between checklist items (a59ff1e5 Fig. 11/12), pull quotes, untitled shaded boxes inside a section (04edb0ff p11).
