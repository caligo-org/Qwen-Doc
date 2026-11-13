# prospectus learnings (v3)

## Decisions (carry over)
- CN cover: L1 = issuer name ∪ document-type line (首次公开发行股票并在科创板上市招股说明书). English issuer name, address,
  seals/logos, 科创板投资风险提示 box, sponsor/underwriter blocks (保荐机构（主承销商）, 联席主承销商): text.
- CN front matter (重要声明, 本次发行概况, 目录, 释义, 重大事项提示) and body chapters (第X节) = L2.
- CN running heads: the header pair "issuer name" + "招股说明书" on each page = one repeat of the L1 union per page.
- TOC entries are text; the 目录 title is L2.
- 科创板投资风险提示 box on the cover, even when its title is a separate box (30f024e2 #1, fd769c1b #1): cover risk banner = text.
- Running head printed on the cover page above the title (dac1e5e9 #1/#2): cannot be a repeat (title not yet open) -> considered/text.
- CN numbering ladder inside a front-matter section: 重大事项提示 L2 > 一、 L3 > （一） L4 > 1、 L5. Sub-heads tagged
  text_block/table_caption but printed bold on their own line like siblings are headings (use their id).
- Numbered 1、2、3、 commitment paragraphs (run-in, body weight) are text; bold own-line 1、 labels followed by paragraphs are headings.
- cf5aeb95 #129: bold "4、..." rule sentence with no block of its own (next line is （四）) -> text by substance, although
  its 1、-3、 siblings are headings (v2 had kept it).
- Signature field 董事长签字： -> considered (text).

## EN fund prospectuses
- Cover: L1 = fund/company name (owner) ∪ "Prospectus ..." line(s), boxes in reading order even when apart
  (01db2dde RICHELIEU BOND ∪ Prospectus September 2015; 0ec528dd Kapitalfonds L.K. ∪ Prospectus including fund
  management regulations; 241e405e PROSPECTUS ∪ FRANKLIN TEMPLETON INVESTMENT FUNDS). Dates, legal form, management
  company, sponsor logos: text.
- Running head / running foot naming the fund (Richelieu Bond; Franklin Templeton Investment Funds) = repeat of L1.
  Website footers (franklintempleton.lu): text.
- Repeated title page (company name + legal form + registered office in one box) = repeat of L1 (241e405e #10).
- "OFFER" printed as the first word of the offer sentence (241e405e #11): text (run-in).
- Front matter (WARNING when it is pages of notices, IMPORTANT INFORMATION, CONTENTS, DEFINITIONS, ORGANISATION,
  ADMINISTRATION INFORMATION) = L2, same level as PART I / top body banners; their own-line subheads L3.
- Directory pages: entry labels on their own line (CHAIRMAN:, DIRECTORS:, CUSTODIAN:, Contact Details:) with their
  block of names/addresses = L3 headings by the own-line-label rule. OPEN: NOTATIONS also lists "contact boxes" and
  "lists of names" as text; I read that as the names, not the labels.
- Directory laid out as a two-column table (01db2dde ORGANISATION): labels are table cells -> text.
- Per-fund fact blocks: run-in labels (Asset Class, Investment Objectives, Risk Considerations) = text; the fund name
  heading = one level below the section.
- Part banner repeated as a running head on later pages (4da61756 SALES PROSPECTUS) = repeat of the part node.
  Sections the TOC lists inside that part (FUND OVERVIEW, MANAGEMENT AND ADMINISTRATION) stay under it at the
  chapter level, even though they come after the numbered chapters.
- Running foot printing the document type (SALES PROSPECTUS WITH MANAGEMENT REGULATIONS) = repeat of L1.
- Bold underlined own-line labels printed like sibling subheads whose block is a table (4da61756 "Shares of the
  classes »PA« and »PT«") = headings (own block), not table captions. OPEN: v2 made them text as table titles.
- A one-box cover title holding both type and fund ("Prospectus SEB SICAV 2") = the whole box is L1 (union in one box).
- Own-line bold labels ending in a colon with their own bullet list (51f57153 "Man Sub-Funds:") = headings one level
  below the section (own-line label with block). v2 had them as text lead-ins. A lead-in sentence that introduces a
  list ("The following sub-funds have been established to date:") stays text.
- Entities listed under a subsection and printed in the same face as it (51f57153 the three investment managers under
  "Investment Managers") = one level below it (content beats look).
- 60d1e00b ch.4: general subheads (Investment Objective ... Reference Currency) share the face of the sub-fund names
  but are not part of any fund group -> L3 under the chapter; fund-group headings (larger) L3; sub-fund names L4
  under their group (content decides; resolves the v2 open case).
- Lead-in sentences ending in a colon that introduce a bullet list ("Investors are reminded that:") stay text, even
  when bold on their own line; noun labels with their own list ("Man Sub-Funds:") are headings.
- d2e37b2e: plain-weight category labels inside the investment-policy list (Renewable Energy: / Energy Efficiency: /
  Resource Efficiency:) -> text: the general policy paragraphs after the list would fall under the last label
  (containment fails). Reverses v2.
- Bulleted bold subheads ("• Limits") = headings, list marker dropped from the node text.
- Cover legal-form lines tagged title (Société d'Investissement à Capital / Variable à Compartiments Multiples) =
  not_headings; the document-type line (Sales Prospectus March 2015) joins the owner in the L1 union.
- bab1780c #22 bold underlined table title ("Overview of Subfunds: ...") whose table is followed by more
  introduction text unrelated to it -> text (containment fails), unlike 4da61756 where the next heading follows.
- Arrow/bullet-marked own-line subheads in front matter (edf49303 "► United States of America") = L3 under the L2
  front-matter section.

## OPEN
- Directory entry labels (CHAIRMAN:, Custodian Bank, Auditors ...) made headings by the own-line-label rule; NOTATIONS
  also lists contact boxes / name lists as text (241e405e, 4da61756, 51f57153, cc0c7019, d2e37b2e).
- Table-titling subheads printed like sibling subheads (4da61756 #128/#130) kept as headings; table titles followed by
  unrelated text (bab1780c #22) made text. The line between "table caption" and "label with its own block" is look-based.
- Running heads printed on the cover above the title (dac1e5e9 #1/#2, d2e37b2e #1) cannot be repeats by check.py
  (title not yet open): put in considered.
- Running heads naming only the owner (Richelieu Bond, EAST CAPITAL (LUX)) or only the document type (SALES
  PROSPECTUS WITH MANAGEMENT REGULATIONS) treated as repeats of the L1 union (they restate half of it).

## Breadcrumb review round 2
- 27 docs checked, 375 breadcrumb lines read; check.py: no ERROR in any doc; breadcrumbs.py: no FLAG in any doc.
- Unseen pages for prospectus in review/unseen_pages.txt: none.
- Every title-category box of every prospectus doc is accounted for (node, repeat, not_headings or considered).
- Pages re-viewed for doubtful paths: 241e405e p3-4, 60d1e00b p4-7, 51f57153 p6-7, 613dbac9 p2-3, cc0c7019 p4-5,
  d2e37b2e p11-13,16. All paths hold; no fixes this round.
- Kept, read and judged correct:
  - cc0c7019 #42 "NB" L3: printed in the face of its sibling subsections (INTRODUCTION, REQUEST INFORMATION) with a
    rule and four paragraphs of notices below; a section by look and substance, not a callout.
  - d2e37b2e #153 "USE OF DERIVATIVE INSTRUMENTS" (no number) and #197 "2. EFFICIENT PORTFOLIO MANAGEMENT TECHNIQUES"
    both L3 under TECHNIQUES AND INSTRUMENTS: the document prints the "1." only implicitly; same face, same level.
  - 613dbac9 L1 text keeps "December 2015" because the date shares one box with PROSPECTUS (boxes cannot be split).
- OPEN: none new.

## Breadcrumb review round 3
- 27 docs checked (13 CN, 14 EN), all 375 breadcrumb lines read; check.py: no ERROR; breadcrumbs.py: no FLAG in any doc,
  so no ACCEPTED flags. Unseen pages for prospectus in review/unseen_pages.txt: none.
- Pages viewed: 241e405e p8-11, 51f57153 p4-5, cc0c7019 p6-7, d2e37b2e p1,5-7, 4da61756 p11, 613dbac9 p1-2,
  cd9d4d87 p1, dac1e5e9 p1; cover title boxes of all 14 EN docs listed.
- Fix class "L1 text = heading part only" (NOTATIONS: owner/type printed in a box that also holds other fields -> node
  uses the box, text is only the heading part), 4 docs:
  - 613dbac9 L1 "EAST CAPITAL (LUX) PROSPECTUS" (box #3 also prints "December 2015"; was kept in round 2).
  - 01db2dde L1 "RICHELIEU BOND Prospectus" (box #2 also prints "September 2015").
  - d2e37b2e L1 "QUEST MANAGEMENT, SICAV Sales Prospectus" (box #6 also prints "March 2015").
  - cd9d4d87 L1 "OYSTER PROSPECTUS" (box #2 also prints the legal form "An Open-ended Mutual Investment Fund (SICAV)
    Luxembourg", cover text by the decisions above).
- RESOLVED (OPEN since round 1) directory entry labels (241e405e CHAIRMAN: ... Contact Details:, 51f57153 Board of
  Directors › Chairman / Custodian Bank ..., cc0c7019 CONTACTS › The Company ... Auditors, d2e37b2e ADMINISTRATION ›
  REGISTERED OFFICE ... LEGAL ADVISORS, 4da61756 MANAGEMENT AND ADMINISTRATION › Management Company ... Distributors):
  kept as headings. Every path reads as a breadcrumb (owner prospectus › administration section › role), each label
  is an own-line label whose block is exactly that role's entry, and containment holds: each directory ends where
  the next front-matter / body section starts (241e405e Contact Details: -> FUND INFORMATION p11; 51f57153 Legal
  Advisers -> The Fund p6; cc0c7019 Auditors -> LEGAL STRUCTURE p8; d2e37b2e LEGAL ADVISORS -> INTRODUCTION p8;
  4da61756 Distributors = last block of the document). "Lists of names / contact boxes" = the name/address blocks,
  which stay text. Run-in labels inside a block (241e405e "in Austria:", 4da61756 "Paying Agent in Luxembourg:") stay text.
- RESOLVED cover running heads above the title (dac1e5e9 #1/#2, d2e37b2e #1): stay considered (text). They are page
  furniture printed before the title opens, not nodes, so no path is affected; the same heads on later pages are repeats.
- Table-titling subheads (4da61756 #128/#130 vs bab1780c #22): paths re-read, both hold (4da61756 labels close at
  the next numbered chapter; bab1780c #22 stays text by containment). No change.
- OPEN: none.

## Round 4 (full path review, every node on its page)
- 27 docs (13 CN, 14 EN), 223 pages viewed, 342 nodes judged: 338 OK, 4 FIXED, 0 ADDED, 4 REMOVED. check.py: no ERROR in any doc.
  Lines in review/paths/prospectus.tsv.
- Node text = printed text (OCR LaTeX removed):
  - cf5aeb95 #74, #75: "\(5\%\)" -> "5%".
  - 4da61756 #128, #130: "\(\gg\) PA ..." -> "Shares of the classes »PA« and »PT«" / "»IA« and »IT«".
- Containment: an own-line label that is the last label before a major section takes in the unrelated notices after
  its block -> text (NOTATIONS "containment closes subtrees"):
  - 01db2dde #29 "Handling of data" (WARNING): KIID/subscription paragraphs #37-#38 (p5) would fall under it.
  - 241e405e #40 "Board of Directors' Powers": share offer, Fund segregation, translations and availability notices
    #47-#51 would fall under it; its paragraphs now sit under "The Company".
  - 613dbac9 #22 "Data Protection" (business-day definition and Directors' responsibility on p4 would fall under it),
    then #14 "United States" (with #22 text, the data-protection paragraphs would fall under it). All notes now sit under
    NOTE TO THE READERS.
- OPEN: 51f57153 #102 "GLG Sub-Funds:" is the last of three label headings (Man / Man AHL / GLG Sub-Funds:) and is
  followed by a paragraph about the investment policy of all sub-funds. NOTATIONS keeps a numbered list's headings when
  a closing paragraph follows; this list is not numbered. Kept by analogy.
- OPEN: edf49303 #23 "Hong Kong": paragraph #26 introduces the next section (Bank Holding Company Act) but is printed
  before its heading, so it falls under Hong Kong. Making #23 text would cascade to all the sibling country labels. Kept.

## Round 5 (breaks)
- 27 docs (13 CN, 14 EN), every page rendered with show.py --tree and viewed. 10 breaks added, 8 nodes restored, 17 docs
  with neither. check.py: no ERROR in any doc. Lines in review/breaks/prospectus.tsv.
- CN breaks: 4aa05674 before #67 closes #57 (rounding note 注：本招股说明书数值... ends the 释义 chapter, not about 二、专业术语);
  5da38fd1 before #45 closes #36 (董事长签字 signs the whole 致投资者的声明); cf5aeb95 before #129 closes #116 (one-line rule
  4、, text by substance, is about the whole （三）实施方案, not about 3、). CN covers, declarations, overview tables and TOCs need no break.
- EN restored (round 1/4 containment text, now headings with a break): 01db2dde #29 Handling of data (break before #37;
  also break before #43 closes TABLE OF CONTENTS: the note after the TOC belongs to L1); 241e405e #40 Board of Directors'
  Powers (break before #47); 613dbac9 #14 United States + #22 Data Protection (break before #29); bab1780c #22 Overview of
  Subfunds table title (break before #27; treated like 4da61756 #128/#130, its only stated reason was containment);
  d2e37b2e #115 QUEST MANAGEMENT – QUEST CLEANTECH FUND L3 + #119/#127/#133 category labels L4 (break before #139, the
  general paragraphs go to the sub-fund).
- Round 4 OPENs resolved by breaks: 51f57153 before #104 closes #102 (closing paragraph after the Man / Man AHL / GLG
  label list); edf49303 before #26 closes #23 (the BHCA introduction is not about Hong Kong).
- Kept as text (other reasons): 241e405e #11 OFFER (run-in), cc0c7019 #34 lead-in, cover lines, running heads.
- OPEN: none.
