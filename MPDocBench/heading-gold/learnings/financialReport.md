# financialReport learnings

## Decisions
- Cover: company name printed on its own line above the report title -> text; L1 = report title line
  ("2025年半年度财务报告"). A printed qualifier inside the title box ("（未经审计）") stays in the text; a qualifier
  in its own box under the title is text.
- Excerpt starting at 第八节 财务报告 (A-share half-year report, financial section only): 第八节 = L2, 一、 = L3,
  1、 = L4. When the excerpt has no 第X节 line, 一、 = L2.
- Running headers (company + report title) are text.
- Glossy front pages (HK/UK style): the slogan under a small section label is text; the label ("我們的宗旨") is the
  L2 heading. TOC block without a printed contents title: no heading.

## Open questions
- d1c80a8f p2: 我們的宗旨 (small label) vs 我們是保誠。 (large slogan) - chose the label as the heading.
- Company name + report title printed in ONE title block (same size, e.g. "四川新金路集团股份有限公司 2025年半年度财务报告",
  "深圳市…股份有限公司 二〇二四年度 审计报告") -> L1 keeps the company name.
- A-share half-year template without 第X节: 一、审计报告 / 二、财务报表 = L2, 1、合并资产负债表 … = L3.
- Audit report (审计报告) excerpt: 目录 and 审计报告 = L2; 一、审计意见 = L3; （一） = L4; 1、 = L5.
- HK interim report (statements only): L1 = 中期財務報告 (+ period line in the same box); brand logo text, English and
  Chinese company names, incorporation and stock code lines are text. Statement titles printed as table_caption at the
  top of each page (綜合損益表 …) are L2 headings; "(續)" pages are continuation nodes. "附註：" under a statement is a
  run-in label -> text.
- Audit-report bundles (pages numbered 6-1-x, cover "XX公司 / 审计报告 / 文号 / 会计师事务所"): the cover title is the
  bundle title (L1) and the 审计报告 restated at the head of the auditor's letter is L2 (a section of the bundle, the
  statements follow it; ad31878f's TOC lists 审计报告 beside the statements). Kept consistent in 486c6f77 where the
  excerpt ends before the statements. 文号, addressee line (全体股东：), firm name, letterhead, signatures are text.
## Open questions (cont.)
- 486c6f77: the restated 审计报告 on p2 could instead be read as a title repeat (text) because the excerpt contains only
  the letter; kept as L2 for consistency with ad31878f.
- Excerpt without a cover/title page (starts at 目录): title role not printed -> 目录, 审计报告 and statement page
  titles are L2 with parent root; audit letter sections keep L3/L4/L5. The TOC's 二、财务报表 group is not printed in
  the body, so a statement page title (合并资产负债表) sits at L2 beside 审计报告.
- Audit bundles: statement page titles (合并资产负债表 …) = L2 beside 审计报告 (the TOC groups them under 二、财务报表 /
  审计报告附送, which is not printed in the body). "（续）" statement pages = L2 continuation nodes (gold had them as
  children at L3). Rotated statement titles on landscape pages are still headings.
- 财务报表附注 with the company name on the line above: the notes title is L2, the company name is text; its 一、 = L3,
  （一） = L4 (gold missed （一）/（二） text_blocks in 4b6d76a1).
- Cover where company name and 审计报告 share one detected box (dd929555) -> L1 keeps both; where they are separate boxes
  (486c6f77, 58dd29fc) the company line is text. Same visual pattern, decided by the box.
## Open questions (cont.)
- dd929555 vs 486c6f77: company name + 审计报告 on the cover is visually the same pattern but the box split differs;
  L1 text follows the box. A uniform rule (always drop the company name) may be preferable.
- Statement-only audit bundle pages: should the unprinted 二、财务报表 group make statements L3 under an implied node?
  Chose L2 (no invented nodes).

## User decisions (2026-11-12)
- L1 = the document title only; the company name is text even in the same title block (6 trees changed). Resolves
  the open question on company names in audit-report covers.

## Label rule pass

- Signature-block field labels (法定代表人：, 主管会计工作负责人：, 会计机构负责人：, 中国注册会计师：) under the
  auditor's letter and statement pages repeat on every page and name a signer, not content -> stay text (35 items).
- Kind A 审计报告 (cover title L1 + restated 审计报告 at the head of the auditor's letter L2) is a false hit: the
  audit-bundle decision above stands -> kept (486c6f77, 58dd29fc, dd929555).
