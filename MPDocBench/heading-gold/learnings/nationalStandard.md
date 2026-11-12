# nationalStandard learnings

All 17 docs are Chinese GB standards with the same SAC template, so one mapping covers the category.

## Decisions (recurring)
- Cover: "中华人民共和国国家标准" (gold L1 in 16/17 docs) is the standard-family label, not the title: not_heading.
  GB number, 代替 line, ICS/CCS codes, GB logo, 发布/实施 dates and issuing bodies are text.
- L1 = the Chinese title on the cover; the English title box merges into it (`merge`). When both languages are
  printed in one box (7b396eb8, 9fd6a86f) the node is that one box.
- A part title printed as two lines in one box ("工业自动化产品安全要求 第10部分：…") is one L1 line.
- Title restated above clause 1 on the first body page: not_heading (repeated title).
- Level map: 目次 / 前言 / 引言 / 附录 X / 参考文献 = L2, the same level as "1 范围". Clause n → L2, n.n → L3,
  n.n.n → L4, n.n.n.n → L5. Annex subclauses A.1 → L3, A.4.1 → L4 (numbering decides).
- Annex heading box "附录 A （规范性） <title>" is one heading, text kept as printed in one line.
- Term entries: number box ("3.1") + term box ("商品煤 commercial coal") = one node, one level below the clause that
  holds it (L3 under "3 术语和定义"; L4 when terms sit under a titled term group "3.1 设备和设备的类别", 96c25428).
  Text = "3.1 商品煤 commercial coal".
- A permitted synonym printed on its own line under the term (e.g. "动力煤" under "3.2 动力用煤") is part of the
  entry body: not_heading.
- Untitled numbered clauses ("7.4.1 下列情况时，应进行型式检验：", "8.1.2 标识应采用…") are paragraphs, not
  headings: number + sentence, no title. not_heading. (Gold mostly already had these as text; 6 were gold titles.)
- Table/figure captions ("表 1 …", "表 1（续）", "图 1 …"), notes (注：), TOC entries, running heads "GB xxxx—yyyy",
  page numbers: text. No missed headings were found among non-title elements in this category.

## Open questions
- Term entries as headings: the brief says "number + term = one heading". Kept, but terms are borderline under the
  breadcrumb test (a definition entry is short and rarely "deep"); the user may prefer them as text.
- Synonym line under a term (00457a35 #56, 040d3ee5 #54): treated as text; could instead merge into the term node.
- 目次 (contents) placed at L2 like 前言; the contents title counts as a heading per NOTATIONS.

## Label rule pass
- Lead-ins to exception lists ("本文件不适用以下产品：", "不适用于：", "不适用以下情况：", "以上要求不适用于：") and
  the generic "式中：" stay text. No tree changes.
