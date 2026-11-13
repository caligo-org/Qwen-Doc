# textBook learnings (v3)

- Title union (L1) = subject title ∪ the course/volume lines printed with it as part of the book's name (物理 ∪ 必修
  ∪ 第三册): they say which book this is. Series line (普通高中教科书, 义务教育教科书), publisher, editors, approval
  seal: text. Title page restating the title (with its volume lines): repeat of L1 (all restated boxes).
- Salutation opening a preface letter (亲爱的同学：), even when printed in title position: text (does not describe
  the letter). The letter sits directly under L1.
- Front matter titles (目录, 前言) and chapters (第N章 + title) at L2. Chapter-opening question lines: text.
- Teacher's book (教师用书): book type is part of the title union (体育与健康 ∪ 教师用书 ∪ 四年级 全一册).
- 第N章 badge + chapter title in separate boxes: one union node. Running head naming the open chapter: repeat.
  Running head naming the book (体育与健康教师用书 四年级): repeat of L1. CIP/colophon restatement of the title: text.
- Teacher-book part labels inside each 节 (教学目标, 知识要点, 教学内容, 教学策略, 教学评价), each with its own block:
  headings one level below the 节, all the same level even when one is boxed differently.
- Pinyin transliteration of the cover title (SHU XUE): text. Edition badge (A版): text. CIP block title
  (图书在版编目(CIP)数据) and the colophon restating the title: colophon text, not repeats.
- BOX RULE (v3 generic-label rule): a box/banner label with its own significant block (material, list, exercises,
  pictures) is a heading one level below the section it sits in, even a kind label repeated in every section
  (练习, 探究与分享, 名词点击, 学习建议, 生活中的设计, 走进博物馆, 儿童美术馆, 访问艺术家). A label whose box holds only a
  one-line prompt/note (思考, 归纳, 老师的话) is a callout: text. Tip boxes (小窍门): text by the category note even
  with pictures. OPEN: main text resuming after the box (例3 after 练习) is absorbed by the box node in the tree.
- Running foot/head naming the open chapter (第四章 数列): repeat of the chapter node.
- Module name under the subject (政治与法治, 必修3): part of the title union (it names which book).
- 人教 统编 books: 单元 L2 > 课 L3 (badge + 第N课 + title) > 框 (bar heading) L4 > 目 (coloured heading) L5.
  Activity/term boxes (探究与分享, 名词点击): headings by the box rule; the terms line inside: text. Diagram banner over a chart
  (三种建国方案): figure title, text. Running foot naming the open unit / lesson: repeat.
- Picture-book lessons: the top band (第N课 badge + lesson title, header boxes) on the lesson's first page = the
  lesson heading (L2, union); the same band on later pages = repeat. Pinyin ruby dropped from node text.
- Colophon credits line (义务教育教科书·美术 over the editor list): text. Cover pinyin (MEI SHU): text.
- Lesson-part labels opening a lesson (问题导入, 学习目标 printed as own-line heads with their own block): headings at
  the level of the lesson's numbered sections (一、), siblings, not one level lower (no level skip; content: they are
  parts of the lesson like 一、).
- Activity-kind label with icon (科学实践, 探究, 活动) printed above an activity title: kind label text; the activity
  title is the heading one level below the lesson. Lesson number badge apart from the title: union "1 <title>".
- Prompt boxes (思考与讨论, 思考, 探究) holding one or two short questions: callout, text. Contrast 练习 with a list of
  exercises: heading by the box rule.
- English twin of the contents title on the continuing contents page (CONTENTS after 目录): repeat of 目录.
- Study-guide books (辅导与题解): L1 = the cover title; the "companion to <original textbook>" line, authors, series
  line (经典教材辅导用书), back-cover series list: text. 内容提要 on the copyright page: L2 front matter; CIP block: text.
- Topic-opening panel labels (问题导引 | 学习目标 | 核心术语 side by side, each over its own short bullet list): headings
  by the v3 label rule, at the level of the topic's sections (siblings of 自然资源的类型), NOT text as in v2. OPEN:
  side-by-side panels are arguably one panel of callouts. The same labels on a "how to use this book" page = text.
- Box = kind label + title (探究 | "资源赌约"谁会赢; 阅读 | 稀土资源): kind label text, title heading one level below
  the section the box sits in (the open section at that point).
- Activity labels: a label over only a question (讨论, 思考) = callout text; a label over a task with its own
  material/table (分析 + table, 立即行动 + table, 运用你的经验 + picture) = heading one level below the section.
  OPEN: the line between "one prompt" and "own block" is judgment.
- English open textbooks: L1 = cover title (author, date: text; fix OCR casing). "On the cover:" over its own
  paragraph = front-matter label heading L2 (v3 label rule; v2 had text). OPEN. License lead-ins ("You are free:",
  "Under the following conditions:", "With the understanding that:") = lead-in sentences to a list: text.
- Code-listing captions ("Octave Script 1: Example"): caption, text. Revision-history entries (year in the margin +
  "SECOND EDITION" label over its bullet list): union year ∪ label, heading one level below Revision history (v3
  label rule; v2 had text). Running heads PREFACE / CONTENTS: repeats.
- Pressbooks books: bold semibold heads = sections; lighter regular-weight heads of the same size = subsections under
  the preceding bold head (look decides; content agrees: they name items the section lists). Running foot naming the
  chapter: repeat. Chapter-end "Footnotes" heading: same level as the sections.
- Margin labels beside definition/example blocks ("Definition 0.1", "Example"): run-in labels, text.
- Repository cover sheets (ScholarWorks): L1 = the first printed book title; a second copy of the sheet and the book's
  own title page are repeats; "Recommended Citation" / "Authors" labels: cover-sheet metadata, text (OPEN under the
  v3 label rule); project/author-group line under a title: text; author name on a cover: byline text even when gold
  marks it. Running foot naming the book: repeat of L1.
- Title text: fix OCR only where the printed title is visibly garbled or lost a graphic word (TRIGONOMeTRY,
  FUNDAMENTALS [of] MATRIX ALGEBRA, MULTIVARIABLECALCULUS).

## Breadcrumb review round 2

37 docs, 234 breadcrumb lines read; flagged/doubtful paths and all 14 textBook unseen pages viewed.
- FIX level by look (1): textBook_eng_a237ffc8 #100 "Road Map" L3 -> L4 under "Other Boring Stuff": printed in the
  L4 size of Copyright/Acknowledgements/Changelog, not the larger L3 preface heads.
- FIX node text (1): textBook_chn_06561845 #23 "致 同 学" -> "致同学" (badge spacing is not part of the title).
- ACCEPTED textBook_eng_a237ffc8 #75 FRONT: the preface really prints Acknowledgements (with Copyright, About the cover,
  Changelog) as a subhead of its "Other Boring Stuff" section.
- ACCEPTED textBook_eng_5074afd3 #48 SEQ: false positive; siblings are revision-history years 2017, 2019, 2020 (FIRST,
  SECOND, THIRD EDITION), printed in sequence.
- ACCEPTED textBook_eng_7ee5bbca #31 #42 #54 #66 SEQ: false positive of the number parser; the book prints Section 1.1,
  1.2, 1.3, 1.4, 1.5 in sequence.
- Kept: 活动任务单 › 任务一/二/三 (textBook_chn_912349ba): own-line task badges, each over its own block; the task name
  (新闻阅读) is a run-in at the start of that block and cannot be unioned. OPEN: "任务一" alone is close to a bare
  number; if bare item numbers must always become text, these three become text and the blocks sit under 活动任务单.
- Unseen pages viewed: 06561845 p5 (blank), 215881e5 p9 (acknowledgements continued), 6f1daec7 p8 (blank) p9
  (contents), 81dcb97d p3-4 (blank) p5 (title page = repeat of L1), a237ffc8 p3-4 p9-11 (preface sections, contents
  list), 9587c15d p14-15 (nomenclature groups printed inside one text box: no separate boxes, nothing to add). No
  missed headings.

## Breadcrumb review round 3

Independent second review: 37 docs, all 266 breadcrumb lines read, plus the content flow under every node (every
element in reading order up to the next heading) to test containment of every box/label node. Unseen pages viewed
again: 06561845 p5 (blank), 215881e5 p9, 6f1daec7 p8-9, 81dcb97d p3-5, a237ffc8 p3-4 p9-11, 9587c15d p14-15:
nothing missing.
- FIX containment, box followed by resumed main text -> text (11): 12b0d2b4 #92 练习 (例3 + 分析 follow);
  1f0d94bf #40 #61 #93 #105 探究与分享 and #50 名词点击 (main text of the 目 resumes after each box);
  87e64920 #36 相关史事 (问题思考 + the section's site map follow); bd56ef2c #80 "资源赌约"谁会赢 (p9 definition text
  follows); fdc97197 #85 分析 (the separate 思考 prompt follows; 讨论/分析/思考 now sit under the case title 京张铁路).
  Same box kind stays a heading where its block really ends at a heading or the end of the excerpt
  (1f0d94bf #30 #115 探究与分享; 27d8ad59 走进博物馆, whose frame spans p6-7; fdc97197 立即行动; 8b86cd5f boxes).
- FIX containment, lesson/topic opener labels -> text (5): 47c3d791 #28 问题导入 (only the grey question is its
  block; the lesson's main text follows before 一、); bd56ef2c #67 #71 #75 问题导引 | 学习目标 | 核心术语 (the 探究
  box and the topic's main text would fall under 核心术语, and the panels before it under the next: callouts of the
  topic opener). Resolves the round-1 OPEN on side-by-side panels. fdc97197 学习目标 kept (its block ends at 一、).
- FIX bare item number -> text (3): 912349ba #40 #42 #44 任务一/二/三: bare marker, the task name is a run-in inside
  the paragraph box and cannot be unioned; the task paragraphs are items of 活动任务单. Resolves round-2 OPEN.
- FIX containment, front-matter label -> text (1): 0ee658d7 #3 "On the cover:" (title-page repeat, author, date
  and the license page would fall under it). Resolves the round-1 OPEN.
- ACCEPTED textBook_eng_a237ffc8 #75 FRONT: the preface really prints Acknowledgements as a subhead of its "Other
  Boring Stuff" section (same size as Copyright, About the cover, Changelog).
- Rule learned: the BOX RULE applies per instance. A box label is a heading only when its block ends at a heading
  (or the excerpt ends); a box sitting mid-section with the main text resuming after it is text, even when the
  same kind label is a heading elsewhere in the book.
- OPEN: none for paths. Remaining judgment: 06561845 lists the title-page pinyin SHU XUE as part of the L1
  repeat while the cover pinyin is text (no path effect).

## Round 4 (full path review)

All 37 docs, all 211 nodes, every page rendered with the tree and viewed (286 pages). Paths:
review/paths/textBook.tsv (207 OK, 4 FIXED, 0 ADDED, 0 REMOVED). No missing headings found; earlier-round
decisions (box rule per instance, opener labels, bare task markers, a237ffc8 Acknowledgements under Other Boring
Stuff) re-checked on the page and kept.
- FIX node text, lines of one heading joined without a space (3): 6c84840b #44 "第一节 地球的宇宙环境";
  bd56ef2c #2 "地理 选择性必修3 资源、环境与国家安全"; bd56ef2c #60 "第1单元 自然资源与人类活动".
- FIX node text, cover ornament dashes dropped (1): fdc97197 #4 "通用技术 选择性必修3 工程设计基础" (the "——" and
  "—" around 选择性必修3 are decoration, not title words).
- Rule applied: node text joins the printed lines of a heading with one space and drops decorative marks (dashes,
  badge spacing), so the breadcrumb reads like the printed title.
- OPEN: none.
