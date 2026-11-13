# examPaper learnings (v3)

## Decisions (carry over)
- L1 = exam title. Subject/time/score lines, school names, 绝密★启用前, name/seat fields are text.
- Parts (第I卷/第II卷, Section A) = L2; question groups (一、选择题…) one level below the part, or L2 when no parts.
  The group node keeps its whole printed line incl. the scoring instruction.
- Individual questions are text (user decision), also "16.（本小题满分12分）" printed alone on a line.
- Front labels with their own block below ("考试注意：" + numbered instructions, "参考公式：" + formulas): headings at
  the level of the body's top sections (L2), by the v3 generic-label rule (own line + own block). The instruction
  lines themselves are text. (0d1c391f)
- Title printed as exam name + subject paper on two lines/boxes (…选考科目考试 / 地理试题): one L1 union. (439ef81c)
- Parts named 选择题部分 / 非选择题部分 are L2 parts like 第I卷/第II卷. (439ef81c)
- Sub-groups （一）必考题 / （二）选考题 one below their group; elective modules ([物理—选修3-3]) one below 选考题. (12e76eb4)
- Group line split over a page break = one union (4d42e18f #83+#85). A module numbered like a group
  ("四、[物理--选修3-4]") stays at group level even when it is semantically an elective under 三、选考题. (4d42e18f)
- 材料一 / 材料二 inside a question: text (they belong to the question, which is text; no outline level). (707b861f)
- Question stems like "16. 读图文资料，回答下列问题。" are text (considered). (6b10fb9e, 707b861f)
- IMO-style papers (logo + "Language … day N" page-header boxes, date line, run-in "第N题."/"N." problems, time/score
  footers): no printed title, so the tree is empty; day headers are running heads (inside the page-header area),
  problems are text. (85419d1b, 99ec4b2c, b2d34828, d80f262e, f672afcf)
- A numbered composite question title ("16. 实验题（I、II、III三题共14分）") is still an individual question: text. (ff633fd0)
- Elective module labels ([选修4-5…]) one level below the 选考题 sub-group, even when the first module label is printed
  inside the sub-group's own box. (ab4971e2)
- Solution sheets without a document title: same-styled section titles ("Solution", "Grading guidelines") are all L1;
  running page headers ("Problem 1 – Solution") are text. (eng_08960d5c)
- v3 generic-label rule: an own-line "Marking breakdown" with its own marks block IS a heading (one below the
  problem's solution title). A "Marking breakdown:" at the very top that belongs to a problem not on these pages:
  L2 with parent root (role level; its parent is not printed). (eng_0b2fd078) OPEN: L1 vs L2 for that orphan label.
- Method labels "1st method"/"2nd method" inside part a, when part b is a run-in paragraph: text (a method heading
  would contain part b; containment). (eng_0b2fd078)
- Single-task olympiad statements (IOI): task name L1, Implementation details/Input format/… L2; the letterhead
  (olympiad, dates, Day N Tasks) and task tag headers are text. (eng_11dba3b2)
- Own-line italic "Theoretical part:" / "Experimental part:" each with its marks list: headings one below Solution
  (v3 label rule). Acknowledgement and Literature: back matter, L1 beside Solution. (eng_2122d135)
- Problem sets made only of bold run-in labels (A1., Answer., Solution 1., Lemma.): empty tree. (eng_23110a64)
- University exam covers: "浙江大学 2018–2019 学年春夏学期" ∪ "《Course》课程期末考试试卷" = one L1 union (owner/term +
  document type); course number, exam form/date, slogans, name fields, score table are text. (eng_2a2d3def, eng_2a4c8db3)
- Bold own-line "Questions:" separating a problem's narrative from its numbered tasks: L2 under the problem title
  (v3 generic label rule; supersedes the v2 label pass). (eng_58d7c49f)
- A paper title printed again at the top of the second day's page: repeat of L1; "Day I"/"Day II" in the body under
  the title: L2 parts. (eng_658ab010)
- Single-problem documents: the problem title is L1; numbered sub-tasks (3.1, 3.2 run-in) are text; IPhO page
  headers (competition/date/Question N) are text. (eng_4f919f1c)
- Solution/answer labels ("Solution:", "Numerical results:") in a set whose problems are run-in paragraphs
  ("Problem 2. Water of mass…"): text, because the following run-in problem statement would fall under the label
  (containment). Only when the next problem has a heading of its own (or the label's block runs to the end) does
  the v3 label rule make them headings. (eng_6c2631df vs eng_0b2fd078) OPEN: borderline; the v3 label rule alone
  would make them headings.
- Day parts with date/time limit in the same box ("First Day — July 17, 1991 Time Limit…"): L2. (eng_75d067cd)
- Sub-task numbers alone on a line in a solution ("1a.", "2b.", "2e. Graphics"): text (individual sub-questions; a
  number alone has no title to form a union with). Task 1/Task 2/Task 3 lines: L2 under the solution title. (eng_dd12c9cc)
- IOI tasks: "Example 1" inside "Examples": L3. (eng_f46d7359)
- Contest problem pack with appended policy (MCM): policy title L2 under the problem title, its sections L3,
  "Report on Use of AI" L4 (user decision, sample shown inside Citation and Referencing Directions); colon-ended bold
  section titles ("Files provided:", "References:") are headings; "Glossary" and "Glossary of key terms/concepts:"
  are sibling L2s. (eng_930559d3)

## OPEN
- Front-instruction labels (考试注意 / 考生注意 / 参考公式) made L2 headings by the v3 label rule; the exam note
  "instructions are text" could be read as covering the label too. (0d1c391f, 439ef81c, ff633fd0)
- eng_0b2fd078: orphan "Marking breakdown:" at page top (belongs to a problem not on these pages) set to L2 with
  parent root; L1 is the alternative.
- Solution labels vs run-in problems (eng_6c2631df): text by containment; the v3 label rule alone says heading.
- IMO-style papers: "Day: 1" / "day 1" inside the page-header block treated as running heads, so the trees are
  empty; they could be read as top-level parts (several L1, like untitled notes).
- 4d42e18f: "四、[物理--选修3-4]" kept at group level by its numbering although it is an elective module of 三、选考题.
- eng_2122d135: Acknowledgement (centred) and Literature (left) differ in look; both L1 as back matter.

## Breadcrumb review round 2
Scope: 38 docs, 125 breadcrumb paths read (10 docs have empty trees: IMO-style papers, the run-in problem set and eng_6c2631df), all
54 examPaper pages in review/unseen_pages.txt viewed, plus the pages needed to check missing groups (0fced805,
6b10fb9e, ab4971e2 p5, 3e46326d p3, 707b861f p4). No tree changes: every path reads title › part › group (› sub-group
› module), or title › labelled block; no check ERROR.
- ACCEPTED examPaper_eng_08960d5c #17 ROOTS: untitled solution sheet; Solution and Grading guidelines are same-styled
  top sections. The page header "Problem 1 – Solution" (p1–2) / "Problem 3 – Solution" (p3) is a running head on every
  page (and its p3 number does not match the grading block, which grades the six parts of problem 1): kept text.
- ACCEPTED examPaper_eng_0b2fd078 #12 ROOTS: orphan "Marking breakdown:" of a question not on these pages precedes
  "Solution of question 3." (see OPEN, round 1).
- ACCEPTED examPaper_eng_2122d135 #36 ROOTS: no printed title; Acknowledgement and Literature are back matter beside
  Solution.
- Checked, no fix: ab4971e2 "三、解答题 › （二）选考题" with no （一） sibling: "（一）必考题：共60分。" is printed inside
  the 三、 group box (#78), so it cannot be its own node; the path still reads correctly.
- Checked, no fix: 0fced805 and 6b10fb9e print only one group line / no group line (all other lines are questions);
  12e76eb4 and 254e4dc5 start at 二、 because they are the physics part of a combined paper.
- Checked, no fix: d1508a80 L1 "Question 2." is the only printed title of a single-problem solution; it names the
  document (solution of question 2), so it stays.
- Unseen pages (ab4971e2 1–4, ff633fd0 2–10, 23110a64 2–14, 2a2d3def, 2a4c8db3, dd12c9cc, 11dba3b2, 66b20269, IMO
  papers dbf0e536/498d26ee/fc1f7054, 2122d135, 0b2fd078, 08960d5c): only questions, answers, options, tables,
  figures and run-in labels; no heading missed, no wrong box.
- OPEN: 08960d5c: the running head "Problem 1 – Solution" is the only line that says whose solution the sheet is;
  making it the L1 (Solution/Grading guidelines as L2) would give a full breadcrumb but contradicts the running-head
  rule and the misnumbered p3 head ("Problem 3").

## Breadcrumb review round 3
Scope: 38 docs, all breadcrumb paths read (before: 125 paths; after: 141 paths, 2 docs still empty: eng_23110a64,
eng_6c2631df), 54 examPaper pages of review/unseen_pages.txt viewed again plus pages needed for each case
(08960d5c p1-3, 0b2fd078 p1-3, 2122d135 p1-3, IMO papers, 58d7c49f, 4f919f1c, 11dba3b2, 66b20269, f46d7359, 4d42e18f p6-7).
Fix classes:
- Title = owner ∪ document where the owner is printed only in a header box (new "every path" rule; text = heading
  part only) - 4 docs: eng_58d7c49f (IPhO2002 header ∪ I. Ground-Penetrating Radar; p2 header = repeat),
  eng_11dba3b2 (letterhead "International Olympiad in Informatics 2017" ∪ Nowruz), eng_66b20269 (header "IOI 2021" ∪
  Dungeons Game), eng_f46d7359 (header "IOI 2021" ∪ Distributing Candies; gold OCR text of the header box is empty).
  Supersedes the round-1 "letterhead / task tag headers are text" note for these.
- Only line saying whose document = L1, restated title = repeat - 1 doc: eng_08960d5c: page header "Problem 1 –
  Solution" (#1) is L1; body "Solution" (#3) restates it: repeat of L1; "Grading guidelines" L2 under it; p2/p3 headers
  repeats (p3 prints "Problem 3", a misnumbered running head: the page grades problem 1's six parts). Resolves OPEN.
- Section-only running header = that section's heading - 8 docs (IMO-style papers 85419d1b, 99ec4b2c, b2d34828,
  d80f262e, f672afcf, dbf0e536, 498d26ee, fc1f7054): the "…, day N" / "Day: N" header is printed only on that day's
  page, so it is the day section's heading; no title text is printed (owner only in the logo), so the days are L1.
  Text = the day part only (language is another field of the box). Resolves the OPEN "IMO day headers" case.
- Orphan label level - 1 doc: eng_0b2fd078 #1 "Marking breakdown:" L2 (unprinted parent) -> L1, so its path has no
  missing step. Resolves OPEN.
ACCEPTED (flags):
- ACCEPTED examPaper_eng_0b2fd078 #12 ROOTS: the sheet starts with the marking breakdown of a problem not on these
  pages, then "Solution of question 3."; no document title printed.
- ACCEPTED examPaper_eng_2122d135 #36 ROOTS: no printed title; Solution, Acknowledgement and Literature are the
  sheet's top sections (back matter beside the body; the look difference is centring only). Fig. 11 at p3 top falls
  under "Experimental part:": it shows the measured characteristic, so it is about the experimental part (containment
  holds). Resolves OPEN.
- ACCEPTED examPaper_chn_85419d1b #13, chn_99ec4b2c #18, chn_b2d34828 #15, chn_d80f262e #15, chn_f672afcf #11,
  eng_dbf0e536 #15, eng_498d26ee #15, eng_fc1f7054 #12 ROOTS: day 1 / day 2 sections of an untitled IMO paper (above).
Checked, no fix:
- 4d42e18f: "三、选考题…[物理--选修3-3]" (#83+#85) contains only the 3-3 questions and "四、[物理--选修3-4]" follows as
  its sibling: both paths read consistently. OPEN closed.
- Front labels 考试注意/考生注意/参考公式 (0d1c391f, 439ef81c, ff633fd0): own line + own block that ends at the next
  heading: headings by the label rule; containment holds. OPEN closed.
- eng_6c2631df: "Solution:" labels stay text; the new section says containment beats the label rule (the run-in
  "Problem 2." would fall under it). OPEN closed. eng_23110a64 (run-in labels only) stays empty.
- eng_4f919f1c: owner printed only in the IPhO logo (no text box) and "Theoretical Competition: 12 July 2011" is a
  document type + date, not an owner: L1 stays the problem title.
- eng_d1508a80 "Question 2." is the only printed title and labels the whole solution document: kept as L1.
OPEN: none of this category concerns a path.

## Round 4 (full path review)
Scope: all 38 examPaper docs, every page viewed with the tree drawn (180 pages), 141 nodes + 2 empty trees
(eng_23110a64, eng_6c2631df) written to review/paths/examPaper.tsv. No tree changes: 141 OK, 0 FIXED/ADDED/REMOVED;
check.py shows no ERROR on any doc.
- Checked, no fix: eng_930559d3 italic "Example 1: row 5" / "Example 2: rows 8 – 12" / "Example 3: row 51" are table
  captions (kept text, as in `considered`), so "Examples to Help Understand the Data Set" has no children; this differs
  in form from the IOI "Example 1" own-line heading (eng_f46d7359 L3).
- Checked, no fix: 4d42e18f "四、[物理--选修3-4]" is printed at the foot of the last page; its questions are not in the
  document (truncated pack), so the node has no content on these pages.
- Checked, no fix: no group line missed in 0fced805, 6b10fb9e, 254e4dc5 (no module labels printed for 13-16),
  3e46326d (第Ⅱ卷 has only question titles 22-25).
