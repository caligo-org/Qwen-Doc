# Heading notation (v2)

## The test

A line is a heading if, when you are deep inside the document, its entry in the breadcrumb tells you something you
need to know about where you are. Everything else is text, however big or bold it is printed.

Example (prospectus_chn_30f024e2, page 1): the cover prints a risk-warning box, the company logo, the issuer's legal
name, its English name, its address, the document title and the sponsor's name and address. Four levels deep
(重大事项提示 › 二、特别风险因素 › （二）销售区域集中) the only cover line you need is the title
"首次公开发行股票并在科创板上市招股说明书" (the IPO prospectus). So the title is L1 and every other cover line is text.

## Levels

1. L1 is the document's title, once. (Periodicals: the masthead.)
2. Same look = same level: headings printed in the same style (size, weight, colour, banner, indentation) are at the
   same level throughout the document. More prominent = higher level. This holds among headings of the same kind
   (the document's outline headings). Label headings that name the items a section lists ("Star Topology:",
   "Local Area Networks:", "Simplex:") sit one level below the section they are in, whatever that section's level,
   so the same label style can appear at different levels (book_eng_dc66edfa: L6 under 1.2.3.1, L5 under 1.2.4).
3. Numbering, when there is any, settles what the look leaves open (1 above 1.1, 第一章 above 第一节, 一、 above （一）).
   Without numbering, the look decides.
4. A heading's parent is the nearest earlier heading of a higher level. So every parent comes earlier in reading order.
5. Levels do not skip inside a document's own outline. They skip only where a role is not printed: a newspaper's front
   page has no section banner, so its stories are L3 directly under the L1 masthead.
6. A heading printed in two boxes or lines (chapter label + chapter title, term number + term, step number + step
   title, a title split over two boxes) is ONE heading: one node whose `merge` lists the other boxes, text joined.

## A heading printed again: two cases

A. **Repeat** (nothing in between): the same heading printed again while its section is still open: the next page
   with "(continued)" / "(cont.)" / 续, a running head naming the current section or the document. Not a node:
   listed in `repeats` with `of: <the heading it repeats>`. The section stays one node; its content keeps flowing
   under it.
B. **Continuation** (something in between): the section or story comes back after other sections or stories:
   a story jumped to a later page ("Continued from Page 1A", 上接第一版), a newspaper section resumed after another
   section. A new node where it reappears, at the level of its role there, with `continues: <the original>`.
   Never a child of the original.

check.py enforces the difference: a repeat must point to a heading that is still open at that point, a
continuation to one whose section was closed by something in between.

## Usually text (apply the test; these almost never pass it)

- Cover lines other than the title: issuer/company/author/publisher names, logos and brand text, translations of the
  title, addresses, dates, edition and volume lines, document numbers, series lines, ribbons and slogans, risk
  banners and regulatory warning boxes, sponsor/underwriter blocks.
- A title repeated later (half-title, title page, the title restated above clause 1).
- Running heads, running section tags (except a newspaper section change, see the newspaper notes), page numbers.
- Labels: callouts (WARNING, CAUTION, NOTE, 注意, 提示, Tip, Did you know?), kickers and rubrics above a headline,
  run-in labels starting a paragraph (bold lead-ins, "Definition 2.1", "Example", "Proof", 第一条 at a paragraph start).
- Decks, subtitles and standfirsts under a headline, bylines, pull quotes, captions, chart and axis titles.
- TOC entries (the contents title itself is a heading), index entries, lists of names (board members, analysts,
  authors, offices), form fields, ads, colophons, subscription and contact boxes, related-reports lists.
- A number alone ("3.1", "PART 01", "Chapter 1", "第三章", a step badge): merge it with the title it labels.

## Labels (user decision)

- Heading: an own-line label that names the specific content under it ("Star Topology:", "Local Area Networks:",
  "Simplex:"). It sits one level below the section it is in.
- Text: a generic label that repeats from section to section ("Advantages:", "Disadvantages:", 注意, 学习建议,
  "Keywords", "Example"); a lead-in sentence that introduces a list or table ("Investors are reminded that:",
  "保险业务收入分险种如下："); anything run into a paragraph (a bold opening phrase followed by text on the same line).
- Numbered outline headings are not labels: "1. 事项描述 / 2. 审计应对" repeated under each matter stay headings.

## Usually headings

- The document title; front and back matter titles at the level of the body's top sections (contents, preface,
  foreword, introduction, abstract, declaration, definitions/释义, references, appendices, disclaimers' section titles).
- Every level of the body outline (chapters, sections, numbered or styled).
- A box with its own title (sidebar, case study) when the title says what the box is about: one level below the
  section it sits in.
- Newspapers/magazines: masthead L1; section banner L2 (and the running section tag when the section changes from the
  previous content page, with `continues` if that section appeared before); story headline L3; in-story subhead L4.

## Per-category notes

Add what you learn in learnings/<category>.md. The notes below are starting points, not rules over the test.

- newspaper / magazine: reference tree trees/newspaper_eng_daf7a3e4.json. Ads, teasers, skyboxes, pointer boxes
  ("More coverage … 3A"), weather and index boxes, colophons are text. Columnist names are text; the column's headline
  is the story. A briefs column label ("Daily Briefing", INBOX) is a story-level heading, briefs one level below.
- manualGuide: the title is product + manual type; brand and model lines on the cover merge into it or are text.
  Warning bands are text. Step titles are headings only when printed as heading lines.
- nationalStandard: L1 = the standard's title (the English title merges into it). "中华人民共和国国家标准", numbers,
  ICS codes, issuing bodies are text. Term entries: number + term = one heading.
- prospectus / annualReport / financialReport: L1 = the document title only ("2019年年度报告", "Annual Report & Accounts
  2022", "FORM 10-K", the 招股说明书 line). The issuer/company/fund name is text, even when printed in the same title
  block (user decision). Person names, cover ribbons, risk banners are text.
- researchReport: L1 = the report's headline. Rating, industry label, analysts, related reports, office addresses are
  text. Disclaimer section titles are back-matter headings.
- paper: L1 = paper title. Authors, affiliations, journal header, column labels are text; Abstract is a heading,
  "Keywords" a run-in label.
- book / textBook: chapter label + title merge. Learning boxes that are only labels (学习建议, 小窍门) are text.
- examPaper: exam title L1; parts (第Ⅰ卷, 选择题部分) and question groups (一、选择题…) are headings. Individual
  questions are text, whatever their stem says (user decision). A document that is one problem printed on its own has
  that problem's title as L1. Instructions (注意事项) are text.
- pptSlides: deck title L1; section divider slides L2; slide titles L3; block titles inside a slide L4; "(cont.)" slide
  titles are continuation nodes.
- goverment: law/notice title L1; 第一章 levels; issuing body, document number, dates, signatures are text.
  Newsletters follow the newspaper notes.
- handwritten: when the notes have no title, the top groupings (第1天, 第八章) are the top level (several L1 nodes).
