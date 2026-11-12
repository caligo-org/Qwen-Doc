# book learnings

## Decisions
- Cover: L1 = the book title only. Author names (even when printed biggest, e.g. ALAFAIR BURKE), series lines
  ("EAST COAST DEVILS BOOK ONE"), taglines, blurbs/quotes, genre lines ("a novel") are text.
- A cover repeated on the next page, half-title and title page (title + author again) are text (title repeated).
- Publisher ebook inserts ("Thank you for downloading this ... ebook", "CLICK HERE TO SIGN UP") are ads: text.
- Copyright page, dedication: text. Contents title: L2 heading; TOC entries text.
- Node text = the heading as printed; where the OCR text is wrong (cover script fonts), the node text is written as
  printed and the note says so (book_eng_ea532ac6).

- Front-matter page titles are L2, same level as chapters: Contents, Dedication (when printed as a title),
  "Also by"/"By the same author", Playlist/Theme Song pages. Group labels inside an "Also by" list (series names
  printed as headings) are L3.
- Back-cover tagline over the blurb (also printed on the cover) is text.
- Chapter label + chapter title (e.g. "Chapter" / "ONE") is one node; chapters are L2 under the book title.
- Cover lines naming the institution/department/publisher (lecture notes, lab manuals) are text; the manual/notes
  title is L1.
- A book excerpt with no printed title (title only as running head) gets its stories/chapters as top-level L1 nodes
  (book_eng_600ba37c); titled exercise boxes (Before Reading / After Reading) one level below.
- A chapter number printed alone as the chapter's only title ("1", "2" at the top of the chapter's first page, even
  when labelled header) is the chapter heading (book_eng_10fbf0e9). A dedication line itself ("TO LEON WERTH") is text.
- Textbooks: "Chapter 1" + chapter title merge (L2); numbered sections below. Titled sidebar boxes one level below
  the section they sit in.
- A numbered section title run into its first paragraph (same box, e.g. "1.1 Data Communication: When we ...") is
  still the section heading when it has numbered children; node text is the title part only.
- Numbered list items ("(2) Their transmission technology") and a heading line repeated on the next line are text.
- Subtitle printed in the same box as the book title is text (node text = title only); an author line in the same
  box as a section title likewise.
- Non-fiction self-help/textbook: "In This Chapter, You'll Learn:" objective boxes, "Did You Know?" callouts, pull
  quotes, list-intro sentences ending in a colon, definition lead-ins ("Foundation, as defined by ..., is:") are text.
- Brand/publisher logo text on a cover (tutorialspoint) is text; the subject title is L1 and the front-matter
  sections (About the Tutorial, Audience, ...) are L2 like chapters.
- A section title repeated at the top of every page (tutorial layouts) is a continuation node at the section's
  level with continues -> the first occurrence; subsections on that page hang under the continuation. A subsection
  title repeated with or without "(continued)" likewise (book_eng_82b4201c).
- Organisation/mission boxes on the inside cover, speaker labels in transcripts are text. Bold labels naming a quoted
  source excerpt ("From the ... website (date):") are headings one level below the section when printed as own lines.
- Label-only summary boxes (KEY POINTS, KEY POINT) are text, like learning-objective boxes.
- Chinese exam-prep books: callout labels ([萌主点拨], 【黄文涛参考答案】, 【相关重点法条】) are text; restated question
  lines before answers and plain numbered list lines ((1) 没有继承人。) are text. A 序言 preface heading is a sibling of
  the 一、二、 sections that follow it, not their parent.
- Chinese textbook chapters (第三章 / 牙列与咬合): chapter label + title merge into the L1 node; 提要 (chapter summary
  box title) and 课后思考题 are L2 like 第x节; 一、 L3; （一） L4. Grey run-in labels ("1. 乳牙殆特征 完整的…") are
  text; an own-line numbered sub-head is a heading even when its siblings are run-in.
- Exam-prep law books: recurring block titles that head real content under each 考点 (命题点拨, 关键词记忆, 小案例训练)
  are headings one level below the 考点; 【本章主要法律规定】 / 本章主要内容提示 are chapter-level sections (same level as
  第x节).

## Open questions
- POV narrator labels under a chapter title (romance novels: "GRACE", "Lenora") are treated as text (labels, like a
  kicker); gold had them as L3. Arguable: they do tell the reader whose voice they are in.
- book_eng_dc66edfa (lecture notes): own-line italic labels ("Simplex:", "Numbers:", "Type of Connection") taken as
  headings one level below n.n.n; own-line bold labels (Star Topology:, Local Area Networks:) placed by the section
  they sit in (L6 under 1.2.3.1, L5 under 1.2.4), which breaks "same look = same level". Advantages:/Disadvantages:
  labels kept as text.
- book_eng_a59ff1e5: two underlined p5 subheads ("Locating House on Property", "Locating Outlying Landscape
  Features") have no element of their own (OCR folded them into neighbouring boxes and dropped the text); they cannot be
  labelled. Same pattern may recur in other docs.
- book_eng_77552588: mixed-case bold heads sit one level below uppercase heads, but in sections without an uppercase
  head (1.1, 1.5) they are placed directly below the section (L3) rather than skipping a level; same-look rule broken
  in favour of no-skip. Same choice as in book_eng_dc66edfa.
- book_chn_0875d4ae: own-line lines "4岁以前乳牙殆特征" / "4~6岁期间乳牙殆特征是：" sit inside run-in item 1. and are kept as
  headings below （二）; "2. 面部协调关系" (own line) kept while its sibling "1. 鼻翼耳屏线" is run-in text. Mixed outline.
- book_eng_e343e570 (landscape guide): scenario titles kept at L2 (own page style) although logically they belong
  under "DESIGN SCENARIOS"; generic box labels CHALLENGES:/GOAL: and figure labels SOLUTION 1/2 treated as text (like
  KEY POINTS), leaving scenarios without children. Arguable: CHALLENGES/GOAL title real content boxes. The program-office
  contact box title is text; the other inside-cover banners (WHAT ARE ..., SERVICES, ACKNOWLEDGEMENTS) kept as L2.

## User decisions (2026-11-12)
- book_eng_dc66edfa: CONFIRMED. Item-label headings (bold/italic own-line labels naming the items a section lists) sit
  one level below the section they are in, even when that breaks "same look = same level" across sections
  (Star Topology: L6 under 1.2.3.1; Local Area Networks: L5 under 1.2.4). Repeating generic labels
  (Advantages:/Disadvantages:) are text. Rule added to NOTATIONS.md level rule 2.

## Label rule pass
- Generic block labels that repeat in every recipe/step/chapter are text even when printed in the heading style:
  Ingredients / Directions / Recipe Notes (book_eng_3ef07034), Before Reading: / After Reading: boxes
  (book_eng_600ba37c), Purpose / Getting Started under each Step (book_eng_a59ff1e5), 命题点拨 under each 考点
  (book_chn_38c7e1dc). Removed from the trees and listed in not_headings ("label rule").
- Repeating own-line labels already text stay text: INTERMEDIATE: / ADVANCED: (book_eng_c5211a9e), Plant
  Characteristics to Look For: / Design Solutions: (book_eng_e343e570), Advantages: / Disadvantages:, 相关法条：.
- Lead-in sentences before a list stay text ("An effective communicator:", "A player should not:", "Mental Toughness
  is defined as:", 职务发明创造分为两类：, 具体包括：); dialogue lines ending in a colon are text.
- Kept as headings: numbered outline heads ending in a colon (1.1 Data Communication:, 案例一：), item labels naming
  specific content (Numbers:, Simplex:, Star Topology: - user-confirmed), source-excerpt labels ("From the Head Start
  Bureau website (...):"), 4~6岁期间乳牙殆特征是： (paired with its colon-less sibling head), and repeated real section
  titles (Introduction as front matter and as a subsection: false hit).
- book_eng_dc66edfa: "Metropolitan Area Network:" printed again right under "Metropolitan Area Network (MAN):" left as
  text (a restatement, not a new section).
- OPEN: book_chn_38c7e1dc 关键词记忆 / 小案例训练 also repeat under every 考点 but were not candidates and stay headings
  (关键词记忆 has the 一、二、 children). The earlier note above (命题点拨, 关键词记忆, 小案例训练 as headings) is superseded
  for 命题点拨; the other two need a decision.
