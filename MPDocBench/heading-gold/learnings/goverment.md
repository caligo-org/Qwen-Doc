# goverment learnings (v3, from scratch)

## Decisions
- Regulation without chapters (chn_48775ee4): L1 = regulation title alone (no separate owner line; the issuing body
  is inside the title or in the adoption note). Adoption/promulgation note in parentheses = text. 第N条 bold run-in at
  paragraph start = text (run-in label); (一)… items = text.
- Chinese State Council regulations (all 12 chn docs): L1 = title; L2 = 第N章 label + title (one box; spaced
  "总  则" kept with whitespace collapsed to one space); L3 = 第N节 when present (chn_536e78f8). Articles 第N条 =
  run-in text; (一)… items = text. No running heads in any of them; suggest_repeats found nothing.
- Declassified US cables (eng_04fdb344 and the other cable docs): no printed title; SUBJECT: is a field inside the
  PAGE/E.O./TAGS block = text. Classification markings, RELEASED IN PART stamps, review-authority blocks = text.
- Newsletter (eng_10e7766e): newspaper notes apply. Masthead L1 (owner already in the masthead, owner block in the
  sidebar = text). No section banner -> stories L3 under L1; titled sidebar boxes that are their own articles (own
  source line) = story level L3; subheads and tip labels with their own block L4. "Inside this issue:" = index box
  title -> text (not_headings). "(Continued)" sidebar head after the main story intervened = continuation node.
- OPEN eng_10e7766e p4: main story resumes after the p3 tips sidebar with no printed resumption head; its first
  subhead ("Video interviews") is made the L3 continuation node (continues the story) so the following subhead keeps
  the right breadcrumb. Alternative: L4 under the sidebar (wrong breadcrumb).
- OPEN eng_10e7766e p5: the tail of the main story (#75-76) falls after the p4 "(Continued)" tips node in reading
  order; no printed head to resume the story, so it sits under the tips node.
- OPEN: "Inside this issue:" text by newspaper index-box rule vs the general rule "contents title is a heading".
- Cables with underlined/dashed section heads (eng_1e210e8c, _5a7b6d51, _c8c79eab, _d4c4a9f2): no title line, so
  the section heads are the top level (several L1, as handwritten notes without a title). "SUMMARY"/"COMMENT" heads
  count. Underlined classification marking (SENSITIVE BUT UNCLASSIFIED, eng_d4c4a9f2) = not_heading. Cable page-id
  lines (BANGKOK 00001158 002 OF 003) = text, not repeats. Participant lists by country (eng_aa75e353) = text.
- Diplomatic notes (eng_1fb9cb74, eng_a323eb0a): note numbers (No. 543), letterheads, DIPLOMATIC NOTE stamps = text.
- Memo summary sheet (eng_186128cd): title L1; own-line colon labels over their own bullet groups = L2 (label rule
  v3); "The proposal includes the following key elements:" = lead-in text; memo fields (Submitted by/To/On) = text.
- Budget-justification excerpt (eng_553a7f3f): no document title; account page titles = several L1; "Highlights:"
  own line over its bullet list = L2 (v3 label rule; v2 had made it text).
- Report cover (eng_b95bd887): act name ∪ "Report to Congress" printed apart = one L1 union; title page restating
  both (act name box + "Report to Congress / June 2008" box) = one repeat of the L1.
- State Dept directory (eng_446784ca): owner + type in one box = L1; report section L2; countries L3; running footer
  "Key Officers of Foreign Posts" = repeat of the section node (suggest_repeats missed it: wording differs from the
  head, "Foreign Posts" vs "Foreign Service Posts"). Bold city lead-ins of post entries = run-in text.
- Diplomatic List (eng_d44e80eb): cover title L1 alone (publisher/series line on p2 is text, not owner of the
  title); Preface / National Holidays L2; quoted convention articles (Article 29/31) L3.
- OPEN eng_a323eb0a: TRANSLATION label over the translated reply note kept as the only node (L1); the US note has
  no title at all. Could be text (it is the translating office's label).
- (resolved round 2) eng_d44e80eb Article 29/31 and eng_553a7f3f "Highlights:" made text (containment).

## Breadcrumb review round 2

26 docs checked (12 chn, 14 eng), every breadcrumb line read; no goverment pages in review/unseen_pages.txt.
Chinese regulations: all paths consistent (title › 第N章 › 第N节), no missed chapter/section labels.

Fixes by class:
- Containment, label made text (2 docs, 3 labels): eng_d44e80eb "Article 29"/"Article 31" (quoted convention
  provisions inside the Preface; the Preface's own family-member/NOTE/availability paragraphs followed Article 31
  with no closing head) -> not_headings, Preface keeps the whole page; eng_553a7f3f "Highlights:" (general account
  paragraphs #32-35 followed the bullets with no closing head) -> not_headings.
- Wrong parent / step not narrower (1): eng_10e7766e "MORE ON WEB RECRUITING" was L4 under the "Video interviews"
  continuation node (a link list is not about video interviews) -> L3 related-links box under the masthead.

ACCEPTED goverment_eng_1e210e8c #19 ROOTS: cable without a title line (SUBJECT is inside the E.O./TAGS box); the
  underlined section heads are the top level.
ACCEPTED goverment_eng_5a7b6d51 #14 ROOTS: same, cable without a title line.
ACCEPTED goverment_eng_c8c79eab #10 ROOTS: same, cable without a title line.
ACCEPTED goverment_eng_d4c4a9f2 #14 ROOTS: same, cable without a title line.
ACCEPTED goverment_eng_553a7f3f #5 ROOTS: budget-justification excerpt with no document title; one L1 per account page.

OPEN:
- eng_10e7766e p5: the tail of the "MORE ON WEB RECRUITING" link list (#75-76) falls after "Say Thanks" in reading
  order and sits under the tips continuation; no printed head to resume the box (replaces the earlier wording
  "tail of the main story").
- Cables: the breadcrumb lacks "whose + what document" because the SUBJECT line shares one box with E.O./TAGS;
  a box-level union cannot isolate it. Kept as text.

## Breadcrumb review round 3

26 docs checked (12 chn, 14 eng); every breadcrumb line read (chn: title › 第N章 › 第N节, all consistent, no change).
No goverment pages in review/unseen_pages.txt.

Fixes by class:
- Title from a shared header box (6 docs): cables 04fdb344 #14, 1e210e8c #6, 5a7b6d51 #6, c8c79eab #5, d4c4a9f2 #5,
  aa75e353 #5: L1 = the E.O./TAGS/SUBJECT box, text = only the SUBJECT text (new rule "the node uses that box and its
  text is only the heading part"). Their underlined/dashed section heads moved L1 -> L2 under it (4 docs, 21 nodes);
  04fdb344 and aa75e353 had no node before and now have the L1. Replaces the round-1 decision "SUBJECT = text" and
  the round-2 OPEN case; the four ROOTS acceptances of round 2 are withdrawn (no flag left).
- Continuation with no printed head (eng_10e7766e, 2 nodes): p4 Recruiting story resumes at paragraph #54 after the
  tips sidebar -> continuation node on #54, text = original headline "RECRUITING 3.0: …", continues #9; "Video
  interviews" #56 back to an L4 subhead under it (was itself the L3 continuation node, breadcrumb skipped the story).
  p5 link-list tail #75-76 -> continuation node on #75, text "MORE ON WEB RECRUITING", continues #59 (was under
  JOB HUNTING TIPS (Continued) › Say Thanks). "MORE ON WEB RECRUITING" #59 kept L3 (titled related-links box).
- Title = owner ∪ document type (eng_a323eb0a, 1): TRANSLATION #27 ∪ ministry letterhead #28 = L1 of the translated
  reply note (text without the address line); #28 removed from not_headings. The US note pp1-5 prints no title
  (No. 543 = note number, DIPLOMATIC NOTE = stamp) and keeps no node; same for eng_1fb9cb74 (two untitled notes).

ACCEPTED goverment_eng_553a7f3f #5 ROOTS: budget-justification excerpt; no document title is printed on any page
  (checked p1-5), each account page title is a top-level section.

Resolved round-1/2 OPEN cases: eng_10e7766e p4 and p5 (continuation nodes above); cables without title line (SUBJECT
L1); eng_a323eb0a TRANSLATION (union with the letterhead); "Inside this issue:" stays text (newsletters follow the
newspaper notes: index box title = text).

OPEN: none.

## Round 4 (full path review)

26 docs, every page rendered with the tree and looked at (12 chn, 14 eng); 120 nodes, all OK, no fix.
Chinese regulations: no missed 第N章/第N节 label on any page; 第N条 run-ins and (一) items stay text.
Cables: SUBJECT-box L1 + dashed/underlined section heads read as consistent breadcrumbs; run-in "(U) Summary." and
numbered paragraphs stay text. eng_d4c4a9f2 "JOINT MINISTER'S STATEMENT" also holds ¶9-11 (POC paragraph) because the
cable prints no closing head before COMMENT: kept (the cable's own outline). eng_1fb9cb74 has no node (two untitled
notes). No OPEN question.

## Round 5 (breaks)

26 docs, every page rendered with `show.py --tree` and viewed (12 chn, 14 eng). 8 breaks, 3 nodes restored; check.py
clean on all 26.

- Chinese regulations (12): NONE. Title › 第N章 › 第N节 with articles only; every article is about its chapter and the
  last article (施行) ends the document. No untitled foreign content.
- Cable signatures (class, 3 breaks): a sender's signature in its own box (1e210e8c BRIDGEWATER #52, d4c4a9f2 JOHN #31,
  c8c79eab Jackson #33 with the classified-website pointer #32) signs the whole cable -> break closing the last
  section. Signatures inside a paragraph box (04fdb344 CLAPP, 5a7b6d51 BOYCE, aa75e353 RICE) cannot be broken out.
- Cable paragraphs not about the open section (2): c8c79eab ¶9 mayors-and-coalitions table (#21, all cities) after the
  Oujda section -> break closes #16; d4c4a9f2 ¶11 POC/documents paragraph (#25) after JOINT MINISTER'S STATEMENT ->
  break closes #21 (round 4 had kept it there for lack of breaks).
- Newsletter eng_10e7766e: p6 Dale Carnegie quote box #93-95 after the LISTEN UP story -> break closes #77.
- Restored (containment entries from round 2): eng_553a7f3f "Highlights:" #23 L2 under the CSH account, break before
  #32; eng_d44e80eb "Article 29" #8 and "Article 31" #10 L3 under Preface, break before #15 (Preface's own text).
- NONE with nothing to close: docs with only an L1 (04fdb344, aa75e353, a323eb0a, b95bd887), no nodes (1fb9cb74),
  or content that stays on topic (186128cd, 446784ca, 5a7b6d51).

OPEN: eng_10e7766e p2 Churchill quote box #24-26 interrupts the "Digital dirt" subsection (story resumes at #27); a
break would also move #27-29 out of the section. NOTATIONS has no rule for a box that interrupts a story mid-flow.

## Round 6 (starts)
Checked all pages by script; only flagged pages were viewed with `show.py --tree`, not every page. The script looked for
content boxes printed below a heading in its column, or right of it, that come earlier in reading order. It also looked
for nodes whose only content is a figure or caption. Table: review/starts/goverment.tsv.
- No body heading has its content before it in reading order. In these layouts, the box ids follow the columns.
- No photo + caption items: every node whose only block is a figure is a page or section title over a chart or diagram
  (for example researchReport_eng_b3735ab5 "JGB yields", annualReport_chn_588028f9 核心竞争力分析). These stay headings.
- Cover content printed beside or below the L1 title that comes before L1 in reading order (research-report and
  paper sidebars, a newsletter sidebar, a document number) sat outside every section. L1 now `starts` at its first
  box. Content printed above the title (logos, ICS codes, cable header fields) and seals stamped over the owner's name
  are left as they are.
- Round 6 visual pass: every page of every goverment document viewed with show.py --tree; the script scan's NONE results and the existing breaks/starts hold; no change.
