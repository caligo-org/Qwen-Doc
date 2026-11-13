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
- Containment closes subtrees: if content that is not about a heading would fall into its subtree (because no later
  heading closes it), that heading is text, even if it is an own-line label with a block (the label rule applies only
  when its block really ends before unrelated content).
- Exception (user decision): own-line label headings stay headings when a later sibling of the same list is run in
  (book_eng_dc66edfa "Star Topology:", "Bus Topology:"); the run-in sibling's content sits under the previous label.
- Trailing untitled content (user decision 2026-11-13): a heading with a real block of its own stays a heading when
  untitled content about something else follows its block and no printed heading closes it (Contents before an
  untitled novel opening, References before author bios, a slide label before the next slide's figures, a story before
  a teaser box or a full ad). That content sits under it: a small containment error. A heading whose only block is a
  caption or one line is text. (Whether this also restores labels made text by containment in rounds 2-3 is open.)
- Substance beats containment (user decision 2026-11-13): a one-line bullet with nothing below it is text, even when as
  a node it would close the section above it.
- Exception (user decision): a numbered list of headings stays headings when a closing or summary paragraph follows
  its last item with no heading in between. The paragraph sits under the last item: a small containment error is
  better than dropping the list's headings.
- A restated title of the whole document inside it (e.g. 审计报告 restated above the auditor's letter of an
  "X 审计报告" bundle) is a repeat of L1, not a child of it: its sections become children of L1.
- When the owner of the document is printed only in a box that also holds other fields (a cable header with the
  SUBJECT line, a 10-K cover field), the node uses that box and its text is only the heading part.

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
(user decision: "MSCI" ∪ "2024 Annual Report", "DENON" ∪ product ∪ manual type); a logo with no text box stays out.
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
- manual: brand ∪ product ∪ manual type is the title (the brand logo's text joins when it has a text box); warning bands are text.
- newspaper / magazine: masthead L1; section banner L2 (and the running section tag when the section changes from the
  previous content page, with `continues` when that section appeared before); story headline L3; in-story subhead L4.
  The front page is a section with no printed banner. Kickers, decks, ads, teasers, skyboxes, pointer boxes, weather
  and index boxes, colophons, columnist names are text. A briefs column label (Daily Briefing, INBOX) is a story-level
  heading, the briefs one level below.
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
