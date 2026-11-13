# nationalStandard learnings (v3)

All 17 docs are Chinese GB standards on the SAC template; one mapping covers the category.

## Decisions (recurring)
- L1 = Chinese title ∪ English title on the cover (union). "中华人民共和国国家标准" (gold L1 in most docs) is the
  standard-family label: not_heading. GB number, 代替 line, ICS/CCS, GB logo, 发布/实施 dates, issuing bodies: text.
- Title restated above clause 1 on the first body page: repeat of L1 (document title, its section is open).
- 目次 / 前言 / 引言 / 附录 X / 参考文献 = L2, the level of "1 范围" (front/back matter at the top-clause level).
- Clause n → L2, n.n → L3, n.n.n → L4 (numbering decides). Annex A.1 → L3, A.1.1 → L4.
- Term entries: number box ("3.1") ∪ term box ("商品煤 commercial coal") = one node, one level below the clause
  holding it (user decision: kept as headings).
- Permitted synonym line under a term (动力煤 under 3.2 动力用煤): text, part of the entry body.
- Untitled numbered clauses ("7.4.1 下列情况时，应进行型式检验：", "8.1.2 标识应采用…"): number + sentence, a
  paragraph / lead-in, no title: text (not_heading where gold had them as titles).
- Table captions incl. "表 1（续）", notes, TOC entries, running heads "GB xxxx—yyyy", page numbers: text.
- Chinese + English title printed in one box (7b396eb8, 9fd6a86f): the L1 node is that one box; the restated
  Chinese-only title above clause 1 is still a repeat of it.
- Number-only clause box followed by a title box (all term entries here): one union, level from the numbering
  (3.1 → L3, 3.1.1 → L4 under a titled term group "3.1 设备和设备的类别", 96c25428).
- A numbered clause whose text is a requirement sentence is text even without end punctuation
  ("5.1.3 有效跟高不应大于 25 mm", "5.2.1 可拆卸小附件不应完全容入小零件试验器" cf448e58,
  "7.4.2 尺寸测量精度精确到 0.02 mm" 7b396eb8); a short noun-phrase clause title is a heading even when it
  contains 应 ("7.3 应力松弛试验").
- Annex heading "附录 A (规范性) <title>" printed over 2–3 lines in one box: one L2 node; A.n → L3, A.n.n → L4.
- Excerpt documents (cover + 目次 + 前言 only: 1a47ad75, 2bed867d, 689650a7, 9fd6a86f, 65073101, 6e5f6a70):
  L1 plus the front-matter L2 nodes that are printed; nothing else.
- Preface change lists ("与…相比，主要技术变化如下：" + a)… items), 历次版本 lines, "不适用于：" lead-ins,
  "式中：", 注：, [来源：…] lines: text. No heading was found among non-title elements in this category.

## OPEN
- Term entries as headings (user decision keeps them): a definition entry is short (one sentence + source line),
  so it is borderline under the substance test.
- Synonym line under a term (00457a35 #56, 040d3ee5 #54, e4f3909e #106 "原位清洗"): treated as text in the entry
  body; it could instead be a second part of the term union.
- 目次 counted as an L2 heading (its block is the TOC listing); TOC entries themselves are text.

## Breadcrumb review round 2
- 17/17 docs checked; 320 breadcrumb lines read (`tools/breadcrumbs.py`); 0 automatic FLAGs; check.py clean on all.
- Every path reads title › clause n › n.n (› n.n.n › n.n.n.n), front/back matter (目次/前言/引言/参考文献) and annexes
  at the clause-1 level, term entries one level below their clause or titled term group. No fix needed.
- Containment scan: every non-node element starting with a clause number (n.n…, A.n…) or a front/back-matter word
  was listed across the category (scratchpad scan). All are requirement / procedure sentences or TOC listings
  (text), so no titled clause is missing and no heading's subtree holds content it does not describe.
- No pages of this category are in review/unseen_pages.txt.
- Fixes: none. Flags accepted: none.

## Breadcrumb review round 3
- Independent review: 17/17 docs checked, 320 breadcrumb lines read; 0 automatic FLAGs; check.py clean on all.
- "Every path is a breadcrumb" applied to every doc:
  - No bare markers: every term number ("3.1", "3.1.1") is a union with its term box; every clause node prints a
    title.
  - Restated title above clause 1 (all full docs): it is a repeat of L1, and clauses are children of L1.
  - Containment: every non-node element in the category was scanned in reading order (scratchpad scan). Untitled
    numbered clauses (7.4.1/7.4.2/8.1.1/8.1.2 in 00457a35, 6.1–6.3 in 040d3ee5, 4.1.x in 733bf6f5, 4.5.1.x in
    5ad1e78c) are requirement sentences inside the subtree of their titled parent and are about it. No content
    falls under a heading that does not describe it. Pages with no node (preface/TOC continuations, table/figure
    pages: 040d3ee5 p5, 1a47ad75 p3, 2cbf14a0 p6, 5ad1e78c p4, 65073101 p6, 7b396eb8 p12, 96c25428 p3-4,
    cf448e58 p7) belong to the open section.
  - Excerpt docs end on 目次/前言: their content is that front matter (no containment leak).
- Pages viewed: e4f3909e p5-6, 7b396eb8 p11-12 (review/*_tree.jpg). No pages of this category are in
  review/unseen_pages.txt.
- Fixes: none. Flags accepted: none.
- Round-1 OPEN cases settled by NOTATIONS v3 (none of them makes a path inconsistent):
  - Term entries as headings: user decision keeps them. CLOSED.
  - Synonym line under a term (00457a35 #56, 040d3ee5 #54, e4f3909e #106): text in the entry body. The entry path
    "… › 3 术语和定义 › 3.n <term>" covers it. CLOSED.
  - 目次 as L2: the contents title is a heading ("Usually text": the contents title itself is a heading). CLOSED.
- OPEN: none.

## Round 4 (full path review, every node on its page)
- 17/17 docs, every page rendered with `show.py --tree` and read (100 pages); 303 nodes, one line each in
  `review/paths/nationalStandard.tsv`: 303 OK, 0 FIXED, 0 ADDED, 0 REMOVED. check.py: no ERROR on any doc.
- Every node text matches its printed boxes (term entries: number box ∪ term box; L1: Chinese ∪ English title,
  one box in 7b396eb8 / 9fd6a86f). Every path reads title › clause n › n.n (› n.n.n › n.n.n.n), front/back matter
  and annexes at the clause-1 level.
- Missing-heading scan (all non-node elements per page): only untitled numbered requirement sentences
  (e.g. 7.4.1/7.4.2 00457a35, 7.5.1/7.5.2 040d3ee5, 4.3.11.1/4.3.11.2 5ad1e78c, 5.1–5.7 and 6.1–6.4 733bf6f5,
  9.1–9.4 7b396eb8, 5.1.x 4234f2c5/cf448e58/f3f3a014), synonym lines, notes, table/figure captions, TOC
  listings, preface change lists. None is a heading; no fix needed.
- OPEN: none.
