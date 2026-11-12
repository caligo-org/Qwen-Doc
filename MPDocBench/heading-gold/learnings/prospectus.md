# prospectus learnings

## Decisions (carry over)
- CN IPO prospectus cover (招股说明书): L1 = the document title line (e.g. 首次公开发行股票并在科创板上市招股说明书). Issuer name,
  English name, address, 科创板投资风险提示 box title, sponsor (保荐人/主承销商) block: text (not_headings).
- CN front matter (本次发行概况, 重要声明/声明, 重大事项提示, 目录, 释义) and body chapters (第X节) = L2; 一、 = L3; （一） = L4; 1、 = L5.
- Sub-headings that the layout tagged as table_caption / text_block but are printed in the heading style of their
  siblings (e.g. a bold （一） risk title) are headings; use their id.
- Text of a heading wrapped over several lines: CJK-CJK line joins without a space (the printed line is one string);
  other joins with one space.
- Running heads (issuer name + 招股说明书) are text.

## Open questions

## Decisions added while working (EN fund prospectuses)
- EN fund prospectus cover: L1 = the document-title line ("Prospectus ...", "Sales Prospectus with Management
  Regulations", "PROSPECTUS"); the fund/issuer name printed as a separate, often bigger box is text (same rule as the
  CN issuer name). When the title and the fund name are one box ("Prospectus SEB SICAV 2"), the whole box is L1.
- A repeated title page (title restated, issuer block with legal form and registered office, "OFFER of separate
  classes of shares ...") is text.
- Front-matter sections (Important information, Glossary of terms, Definitions, Table of contents, Organisation,
  Warning) are at the level of the body's top units (Parts / top banners) = L2.
- Directory pages (ADMINISTRATION INFORMATION / MANAGEMENT AND ADMINISTRATION): the section title is a heading; the
  labels of each entry (CHAIRMAN:, DIRECTORS:, Custodian, Management Company, Paying Agents, Contact Details) are text
  (lists of names/addresses), even when styled like headings.
- Run-in bold labels in per-fund fact blocks (Asset Class, Investment Objectives, Risk Considerations) are text.
- Gold sometimes promotes a numbered sibling to the parent level (36d76993: "2. Involved parties" L2 next to
  "1. General information" L3): numbering decides, both under the Part.

## Open questions (EN)
- Fund cover with fund name and "Prospectus" in separate boxes: the fund name is the more prominent line. I made the
  "Prospectus ..." line L1 and the fund name text (consistent with the CN issuer rule); merging both into one L1 is the
  alternative (01db2dde, 0ec528dd, 241e405e, 4da61756).
- Directory-entry labels styled exactly like section headings (4da61756 p11 Management Company, Depositary Bank, ...):
  made text by the "lists of names/offices" rule; the look alone would make them L4.
- cf5aeb95 #129: a bold numbered 4、 sentence (a rule, not a topic) printed like its 1、-3、 heading siblings: kept as
  heading by "same look = same level".
- Bold labels sitting directly above a table ("Overview of Subfunds: ...", "Shares of the classes »PA« and »PT«") are
  table titles: text (bab1780c, 4da61756).
- Bold lead-ins ending in a colon that introduce a bullet list ("Investors are reminded that:", "Man Sub-Funds:",
  "Renewable Energy:") are text.
- Bulleted bold subheads ("• Limits"): heading, list marker dropped from the node text (d2e37b2e).
- When the gold nests a later section under a directory label (cc0c7019 LEGAL STRUCTURE under CONTACTS), the section
  goes back to its own level under the Book/Part.
- Pages that hold only a TOC continuation or a definitions/term table were checked via listing.py (one text_block or
  table, no title candidates) rather than by image.

## Open questions (more)
- 60d1e00b ch.4: bold body-size subheads (Investment Objective ... Reference Currency) share the face of the sub-fund
  names (L4 under larger fund-group headings) but come before the first group; made L3 (no level skip) rather than L4
  (same look). Same-look rule and no-skip rule conflict here.
- Fund-prospectus cover where "PROSPECTUS" is a small line under a much larger fund name (edf49303, bab1780c): L1 is
  the small PROSPECTUS line by the issuer-name rule; reviewer may prefer the fund name.

## User decisions (2026-11-12)
- Cover: L1 = the document title only, issuer/fund name text (prospectus_chn_30f024e2 decision). 36d76993:
  "Prospectus SEB SICAV 2" -> "Prospectus". Resolves the fund-cover open question (fund name = text, not merged).

## Label rule pass

- Signature field "董事长签字：" and list lead-in "措施：" (introduces the numbered measures) -> stay text.
- d2e37b2e: own-line category labels in the investment-policy list (Renewable Energy: / Energy Efficiency: /
  Resource Efficiency:) name the bullets under them -> new L3 headings under INVESTMENT OBJECTIVE AND POLICY (L2).
  The general policy paragraphs after the list now fall under "Resource Efficiency:" by reading order; the sub-fund
  line "QUEST MANAGEMENT – QUEST CLEANTECH FUND (denominated in EUR):" (not a candidate) was left as text.
