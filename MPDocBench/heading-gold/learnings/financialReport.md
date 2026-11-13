# financialReport learnings (v3)

## Decisions
- Cover: owner name + report title (separate boxes) = one L1 union (v3 overrides the v2 "title only" decision).
  Cover date, logo = text. Running header "company + report title" = repeat of L1.
- A-share half-year excerpt: 第八节 财务报告 = L2, 一、审计报告 / 二、财务报表 = L3, 1、合并资产负债表 = L4.
- Excerpt without cover/title (starts at 目录): L1 role not printed -> top sections L2 with parent root
  (目录, 审计报告). The TOC group 二、财务报表 is not printed in the body -> statement page title (合并资产负债表)
  = L2 beside 审计报告 (no invented nodes).
- Audit letter: 一、 = L3 under 审计报告; key audit matter （一） = L4; "1. 事项描述 / 2. 审计应对" repeated labels
  with own blocks = L5 headings. （一）…（六） items in the auditor-responsibility paragraph flow = text.
- Report number (天健审〔…〕号), addressee line (…全体股东：), auditor letterhead, signatures, seals = text.
- Running head printed on the first page above the title it repeats: cannot be a repeat (section not open yet)
  -> considered/text.
- Owner printed only in the running head (16e75a27: body title "2025年半年度财务会计报告" alone): L1 = the p1 running
  head box (owner part) ∪ the body title (round 3; was the body title alone); later running heads = repeats of it.
- Audit-report cover (owner ∪ 审计报告, separate boxes) = L1 union. The 审计报告 restated at the head of the auditor's
  letter = repeat of L1 (title restated, v3), not an L2 (v2 had L2). Letter sections 一、 = L2, （一） = L3, 1. = L4.
  Audit firm name / English name / report number on the cover = text.
- Each statement page title (合并资产负债表, 母公司利润表 …, also rotated titles on landscape pages) = its own heading
  at the level beside 审计报告 when the 二、财务报表 group is not printed in the body.
- 财务报表附注 with the company name printed on the line above (a sub-title block, 4b6d76a1): union owner + 财务报表附注
  at section level (L2 without a document title). OPEN: could be read as owner = text, notes title alone.
- Notes: 一、 = one below 财务报表附注; （一）编制基础 text_blocks with their own paragraph = headings one level below.
- Audit bundle WITH statements (58dd29fc, 87bc5908, ad31878f, d65056e3, dd929555): the 审计报告 restated above the
  letter = repeat of the owner ∪ 审计报告 L1 (round 3, restated-title rule; was L2), letter sections 一、 = L2 beside
  the statements (L2) and 目录. Same as the letter-only excerpt (486c6f77): one outcome for one printed pattern.
- Cover with 审计报告 above the company name (58dd29fc) = union in reading order (type, owner).
- "（以下无正文）", "（此页无正文…签字盖章页）" = text.
- Statement "(续)" title on the following page with nothing in between = repeat of the statement (v2 made it a node).
- Running head split over two boxes (company / report title, either wrapped or left+right) = one repeat with both boxes.
- Cover qualifier in its own box (（未经审计）) = text; inside the title box it stays in the title text.
- Glossy HK cover (d1c80a8f): report title ∪ owner (Prudential plc under it) = L1; stock code = text. Small label
  我們的宗旨 = L2 (names the purpose/mission block); big slogan 我們是保誠。 = not_heading (callout).
  OPEN: the label's block is short (slogan + one mission sentence); the TOC without a title then falls inside it.
- Running footer "owner + report title" = repeat of L1.
- HK interim (d603bfdc): L1 = Chinese company name ∪ 中期財務報告 box (period line inside the box stays). Brand logo
  text and English company name = not_headings (cover text); incorporation / stock code / year = text.
  Statement table_captions = L2. "(續)" caption on the next page = repeat. "附註：" own line + its own block below a
  statement = heading one level below the statement (v3 label rule; v2 had it as run-in text).
- A-share half-year statements-only reports (16e75a27, 8c038830, eef30e08): L1 title; 一、审计报告 / 二、财务报表 = L2;
  1、…表 = L3. Signature field lines (法定代表人：…) = text.

## OPEN
- (resolved round 3) 财务报表附注 with the company name above it (4b6d76a1): union kept.
- (resolved round 3) Restated 审计报告 above the auditor letter: always a repeat of L1.
- Unprinted TOC groups (二、财务报表 / 审计报告附送): no invented nodes, statements sit beside 审计报告.

## Breadcrumb review round 2
Docs 15, all breadcrumb lines read (187 before the fix, 186 after). Unseen pages of this category: none listed.
- FIX containment/substance (1): d1c80a8f #5 我們的宗旨 L2 -> not_heading. Its block is the slogan + one mission
  sentence, and the untitled TOC (#9/#10) after it is not about it, with no later heading to close it. Resolves the
  earlier OPEN. Tree is now L1 only.
- ACCEPTED financialReport_chn_07ebf5d6 #8 ROOTS: excerpt starts at 目录, and no title is printed anywhere. 目录 /
  审计报告 / 合并资产负债表 are top sections with the L1 role not printed (NOTATIONS levels rule 5).
- ACCEPTED financialReport_chn_4b6d76a1 #8 ROOTS: same pattern (目录, 审计报告, 8 statements, 财务报表附注 at root,
  no title printed).
- Checked and kept: d603bfdc 簡明綜合現金流量表 › 附註： (label with its own cash-equivalents note block, about the
  statement); 02be59d4 / 486c6f77 / 16e75a27 page images match their trees.
- OPEN 16e75a27: the owner + title are printed only in the p1 running head above the body title. L1 stays the body
  title alone (round-1 decision), so the breadcrumb lacks "whose". The alternative is to make the running head (#1)
  the L1 and the body title (#2) a repeat.
- OPEN (kept from round 1): restated 审计报告 as L2 under an owner ∪ 审计报告 L1 (58dd29fc, 87bc5908, ad31878f,
  d65056e3, dd929555) reads "X 审计报告 › 审计报告". The step is narrower in role (the letter vs the bundle) but not
  in wording.

## Breadcrumb review round 3
Docs 15, all breadcrumb lines read (186 before, 181 after). Unseen pages of this category: none listed.
- FIX title union, owner only in the running head (1): 16e75a27 L1 = #1 (p1 running head, owner part) ∪ #2
  (2025年半年度财务会计报告) -> "山西太钢不锈钢股份有限公司 2025年半年度财务会计报告"; #1 removed from considered;
  children and the 11 running-head repeats now point to #1. Resolves round-2 OPEN.
- FIX restated title = repeat of L1 (5): 58dd29fc #14, 87bc5908 #16, ad31878f #11, d65056e3 #17, dd929555 #12
  审计报告 L2 -> repeat of L1; letter sections 一、…六、 L3 -> L2 under L1, （一） L4 -> L3, 1./2. labels L5 -> L4.
  Paths no longer read "X 审计报告 › 审计报告". Resolves the round-1/2 OPEN (one outcome, same as 486c6f77).
- KEPT 4b6d76a1 #232 浙江富特科技股份有限公司 ∪ 财务报表附注 (L2 at root): the notes are printed with their own
  centred title block naming owner + document type (owner ∪ type rule); no document title exists to carry "whose".
  Resolves the OPEN.
- ACCEPTED financialReport_chn_07ebf5d6 #8 ROOTS: excerpt starts at 目录; no title, owner only in the letter's
  addressee line (…全体股东：), which is not a title box. Top sections with the L1 role not printed (levels rule 5).
  This 审计报告 is the bundle's TOC section 一、审计报告, not a restated title (there is no L1 to repeat).
- ACCEPTED financialReport_chn_4b6d76a1 #8 ROOTS: same pattern (目录, 审计报告, 8 statements, notes at root).
- OPEN: none concerning a path.

## Round 4 (full path review)
Docs 15, pages 130 (all rendered with the tree and looked at), nodes 181: OK 178, FIXED 3, ADDED 0, REMOVED 0.
No missing headings found; every statement title, letter section, key audit matter and 1./2. label sits on its page
as the tree says. Earlier decisions (ROOTS for 07ebf5d6 / 4b6d76a1, restated 审计报告 = repeat, p1 running heads
above the title = considered) confirmed on the pages.
- FIX node text (3): the L1 text carried the layout box's line breaks, so the breadcrumb printed over several lines.
  Joined into one line (union text rule): d1c80a8f #2 "二零二五年上半年度財務報告 Prudential plc" (one wrapped title,
  joined without spaces), d603bfdc #4 "中國天元醫療集團有限公司 中期財務報告 截至二零二五年六月三十日止六個月",
  dd929555 #1 "深圳市卫光生物制品股份有限公司 审计报告". Boxes, levels and parents unchanged.
- OPEN: none.
