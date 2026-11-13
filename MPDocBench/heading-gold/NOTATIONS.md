# Heading notation (v3)

This is the synth2 rulebook (`~/dev/caligo/heading-research/synth2/HEADINGS.md`) applied to MPDocBench, plus the
user's decisions listed at the end. Where the two differ, the user decisions here win.

## The test: containment, breadcrumb, substance

1. **Containment.** A line is a heading only if it describes everything in its subtree: the content that follows it
   until the next heading of the same or a higher level is *about* what the heading says.
2. **Breadcrumb.** For every item in the document, the chain of its parents must read as a consistent path, each step a
   narrower description of the one above (金科地产 2019年年度报告 › 第三节 公司业务概要 › 一、主要业务).
3. **Substance.** A heading must have a significant, standalone block of content attached to it: clearly separated,
   and on average much more content than heading. A callout text by itself is never a heading; a bullet item with
   nothing below it until the next item is never a heading. A question CAN be a heading when it has its own separate
   answer block (interviews, FAQs).

Everything else is text, however big, bold or coloured it is printed.

## Every path is a breadcrumb (user goal)

- Every node's text describes its part of the document. A continuation whose page prints no headline (only
  "（上接第一版）" or a jump word) takes the original's title as its text (the `why` records what is printed). A bare
  marker ("（一）", "3.1", "Part 2") is a union with the title it labels. A bare marker that opens a real unit with no
  printed title (chapters "1"…"8", "案例一：") is a heading on its own (user decision): "The Little Prince › 2" is a
  breadcrumb. It is text only when it opens no unit.
- Containment: content that is not about a heading must not sit in its subtree. Where no later heading closes the
  section, a break closes it (see "Breaks"); the heading stays.
- Own-line label headings stay headings when a later sibling of the same list is run in (user decision,
  book_eng_dc66edfa "Star Topology:", "Bus Topology:"); a break before the run-in sibling closes the previous label,
  so the run-in sibling's content sits under the list's parent.
- Trailing untitled content (user decision 2026-11-13): a heading with a real block of its own stays a heading when
  untitled content about something else follows its block; a break closes it there (see "Breaks"). A heading whose
  only block is a caption or one line is text.
- Substance beats containment (user decision 2026-11-13): a one-line bullet with nothing below it is text, even when as
  a node it would close the section above it.
- A numbered or labelled list of headings followed by a closing or summary paragraph about the whole list: the list
  stays headings and a break before the paragraph closes the last item (the paragraph belongs to the list's parent).
- A restated title of the whole document inside it (e.g. 审计报告 restated above the auditor's letter of an
  "X 审计报告" bundle) is a repeat of L1, not a child of it: its sections become children of L1.
- When the owner of the document is printed only in a box that also holds other fields (a cable header with the
  SUBJECT line, a 10-K cover field), the node uses that box and its text is only the heading part.

## Breaks

A **break** closes a heading's section where the page ends it without printing a new heading: the content after the
break belongs to the parent of the closed heading (user decision 2026-11-13). In the tree: `breaks:
[{"before": <first box after the break>, "closes": <node id>, "why"}]`. Closing a node also closes every section open
inside it. A break changes only where content sits, never the heading tree or any path, and it is not used for
scoring or training yet.

- Put a break wherever untitled content that is not about the open heading follows that heading's block: a slide
  label's block ends and the slide's figures follow; Contents or References ends and untitled pages follow; a story
  ends and a teaser box, an ad or a colophon follows; a label list ends and a closing paragraph about the whole list
  follows; an own-line label's block ends and the section's main text resumes (探究与分享 boxes, 学习目标 panels).
- Because a break fixes containment, a label with its own block is a heading even when the content after its block
  is not about it: the break closes it. Containment no longer turns such a heading into text. Text stays text for its
  other reasons (substance: a caption, a one-line bullet; a lead-in; a chart title; a run-in label).
- A block set inside a section whose own content continues after it (a quote box or pull quote in a story, a
  sidebar field between bullets, a note between clues) gets no break: a break is needed only where the hierarchy is
  affected, i.e. where the content after it no longer belongs to the open section (user decision 2026-11-13).
- No break is needed where a heading closes the section anyway (a heading of the same or a higher level).
- A printed heading with no layout box of its own cannot be a node; put a break before its content so that content
  does not sit under the previous heading.

## Section starts

Where the layout's reading order puts part of a heading's own content before the heading's box (a headline set below
or beside its story's first column, a label printed under its box), the node gets `starts`: the first box of its
section. The heading tree is unchanged; like breaks, `starts` only says where content sits. A content box belongs to
the node whose section start (page, box id) is the greatest one at or before it, minus what breaks close, plus
`places`. Section starts must be distinct but need not follow the node order, so a box that the reading order puts
before its own section's heading (a KEY POINTS box printed above the section it summarises) is listed in the tree
where it belongs.

Where the layout's reading order puts a content box of one section after a later heading (a story's photo read
after the next headline, a grid whose blocks interleave), `places: [{"box", "in", "why"}]` assigns that box to the
node it belongs to.

## Unions

A heading is a **union** of one or more printed boxes: the boxes that together print one heading. Typical unions:
number + title ("3.1" + "纺织产品 textile products"), chapter label + chapter title ("第一章" + "健康基础知识"),
step badge + step title, "PART 01." + part title, a title split over boxes or lines, a title + its translation printed
with it, and on covers the **owner + the document type** (see "The title"). The boxes of a union need not be adjacent:
lines between them that are not part of the heading stay text. In the tree, `boxes` lists the union's boxes in reading
order; the node's text is their text joined with one space.

## The title (L1)

L1 is the union of the lines that together name the document, once: **whose document + what document**. Deep in the
document the breadcrumb must say "section 5 of the Naruida annual report", not "of the annual report" and not "of
Naruida". So on a report cover the company name and the report title are one union wherever they sit
(广东纳睿雷达科技股份有限公司 ∪ 首次公开发行股票并在科创板上市招股说明书; 金科地产集团股份有限公司 ∪ 2019年年度报告;
registrant name ∪ FORM 10-K; The New Ireland Fund ∪ Annual Report October 31, 2022). Everything else on the cover is
text: the owner's English name, address, dates, dates, edition and document numbers, regulator names,
series lines, slogans, ribbons, risk banners, sponsor / underwriter blocks. When the owner is printed only as a logo, the logo's text joins the union if the layout has a box with that text
(user decision: "MSCI" ∪ "2024 Annual Report", "DENON" ∪ product ∪ manual type). When the owner is printed on the cover
only as a logo and the layout has a box for that logo, even an image box with no text (header_image / figure), the logo
box joins the union and the node text includes the owner's name as printed on the logo, in the main language (川财证券,
西南证券, brother) (user decision 2026-11-13). This applies only where the L1 text does not already name the owner, and
only to "whose document + what document" titles: annual and financial reports, prospectuses, research reports, manuals,
government documents, and slide decks that are an organisation's report. It does not apply to books, textbooks, papers,
exams, magazines, newspapers, national standards or handwritten notes (book authors, publishers and journals are not
owners; mastheads are already L1). The main printed title box stays the first box of the union (the node id); the logo
box comes after it. A sidebar metadata field that repeats the owner (川财研究 | 宏观策略部) is text, not part of the title.
A document with no L1 stays without one.
Book authors are not the owner: a book's L1 is its title (user decision). Periodicals: the masthead is L1.
A title repeated later (half-title, title page, title restated above clause 1, running head) is a repeat.

## Levels and parents

1. Same look = same level among the document's outline headings; more prominent = higher level.
2. Numbering settles what the look leaves open (1 above 1.1, 第一章 above 第一节, 一、 above （一）).
3. Where look and content disagree, the content decides: a label heading that names the items a section lists sits one
   level below the section it is in, so the same style can appear at different levels.
4. A heading's parent is the nearest earlier heading of a higher level: every parent comes earlier in reading order.
5. Levels do not skip inside the document's own outline; they skip only where a role is not printed (a newspaper front
   page has no section banner, so its stories are L3 directly under the L1 masthead).
6. Every level of the body outline counts, numbered or not, including levels printed like body text and set apart only
   by spacing.

## A heading printed again

- **Repeat** (its section is still open): running head naming the current section or the document, "(continued)" on
  the next page, 续. Not a node: listed in `repeats` with `of`.
- **Continuation** (other sections came in between): a story jumped to a later page, a section resumed after another
  section. A new node at the level of its role there, with `continues` pointing to the original. Never a child of it.
- A running header printed only on one section's pages is that section's heading; if the body also prints a title,
  the body title is one level below it (600 Group: "CFO Statement" L2, "Chief Financial Officer Statement" L3).

## Usually text

- Cover lines other than the title union (see "The title").
- Running heads and running section tags that are repeats (newspapers: a section tag starts a section node when the
  section changes; see newspapers), page numbers.
- Callouts (WARNING, NOTE, 注意, Tip), kickers and rubrics above a headline, run-in labels starting a paragraph (bold
  lead-ins, "Definition 2.1", 第一条 at a paragraph start), lead-in sentences that introduce a list or table.
- Decks, subtitles, standfirsts, bylines, pull quotes, captions, chart titles, axis titles, figure labels.
- TOC entries (the contents title itself is a heading), index entries, lists of names, form fields, ads, colophons,
  contact boxes.
- A number alone ("3.1", "PART 01", "第三章"): it is part of the union with the title it labels.
- Individual exam questions (no answer block on the page).

## Usually headings

- Front and back matter titles at the level of the body's top sections (contents, preface, foreword, introduction,
  abstract, definitions/释义, references, appendices, disclaimer section titles).
- Every level of the body outline.
- A box with its own title (sidebar, case study) when the title says what the box is about: one level below the section
  it sits in.
- An own-line label with its own separate block below it, **even when the same label repeats from section to section**
  ("Ingredients" / "Directions" in each recipe, "Advantages:" + its list, "Datapoints/trends we're focused on" in each
  region box, 学习建议 with its block): one level below the section it is in.
- Interview / FAQ questions with their own answer block.

## Per-category notes

- annual / financial / filing / prospectus: L1 = owner ∪ document type (see "The title"). Statement titles and notes:
  a title that describes the block below it is a heading. Board / executive name lists are text.
- research report: L1 = the report's headline (∪ the company / industry it covers when printed as part of the title);
  rating, analysts, related reports, office addresses are text; disclaimer section titles are back matter.
- paper / academic: L1 = paper title (∪ its translation printed with it); authors, affiliations, journal header,
  column labels are text; Abstract is a heading; Keywords is a run-in label. Headings of another article sharing the
  pages are text.
- book / textbook: chapter label ∪ title; boxes that are only labels with no block (Tip, 小窍门) are text.
- exam: exam title L1; parts (第Ⅰ卷, 选择题部分) and question groups (一、选择题…) are headings; individual questions and
  instructions are text. A document that is one problem printed on its own has that problem's title as L1.
- slides: deck title L1; section divider slides L2; slide titles L3; titled blocks inside a slide L4; a "(cont.)" slide
  title is a repeat when nothing came in between, a continuation otherwise.
- standard: L1 = the standard's title ∪ its English title; the standard number, ICS codes, "中华人民共和国国家标准",
  issuing bodies are text; term entries: number ∪ term = one heading (user decision: kept as headings).
- government / legal: law, notice or agreement title L1; chapter / article levels below; issuing body, document number,
  dates, signatures are text. Newsletters follow the newspaper notes.
- manual: brand ∪ product ∪ manual type is the title (the brand logo joins when the layout has a box for it, see "The title"); warning bands are text.
- newspaper / magazine: masthead L1; section banner L2 (and the running section tag when the section changes from the
  previous content page, with `continues` when that section appeared before); story headline L3; in-story subhead L4.
  The front page is a section with no printed banner. Kickers, decks, ads, teasers, skyboxes, pointer boxes, weather
  and index boxes, colophons, columnist names are text. A briefs column label (Daily Briefing, INBOX) is a story-level
  heading, the briefs one level below. A story whose section is printed nowhere on its pages skips to L3 under the
  masthead only before the first L2 section (a front page); after an L2 section (contents, an earlier section) it
  takes the section slot, L2, because an L3 after an L2 would sit under that section in Markdown levels. A running
  section tag printed on a story's pages is an L2 node above the story, with `starts` at the story's opener when the
  tag is printed later than the opener.
- handwritten: when the notes have no title, the top groupings are the top level (several L1 nodes).

## User decisions (all apply)

- Containment / breadcrumb / substance test (synth2 HEADINGS.md).
- Unions: a heading is a union of boxes; on covers the L1 is owner ∪ document type.
- Generic own-line labels with their own block are headings even when they repeat (replaces the v2 label rule).
- Interview questions with their own answer are headings.
- National-standard term entries stay headings.
- Exam questions are text.
- Repeats vs continuations as above; a section-only running header is that section's heading.
- Content beats look for levels (label headings one level below their section).
