# handwritten learnings

## Decisions
- Multi-sheet notes where each sheet prints its own big title: each distinct sheet title is L1 (several L1 nodes); a
  title that is only repeated at the top of later sheets is a running title -> not_heading (01fba5b3).
- A topic heading repeated at the top of the next sheet while its numbered list continues (注意事项 1-11, then 12-21)
  is a continuation node with `continues` = the first occurrence (01fba5b3).
- Labels drawn inside a figure box (DAY1..DAY5 in an itinerary diagram) have no element of their own: no node.

- Lesson notes with two lessons, each with its own title (教案 + 第二课时): each lesson title is L1 (048c9a89).
- Worked exercises / problem statements ("2. 如图...", "(3) 如图...") are body text, even when gold marked them as
  titles; a topic line such as "(5) 架桥选址问题" or "差的最大值经典习题(1)" printed as its own heading line is a
  heading (048c9a89).
- Printed drill workbook filled by hand (1ae487be): round banners (第二轮 …) L1, day markers (第N天) L2, 类型/考法 L3,
  unnumbered sub-type lines L4. The first round's banner is not printed, so its days are L2 under root (role-skip).
  Same look = same level across rounds (gold had 第N天 at L1 in round 1 and L2 in round 2).
- A separate notebook sheet appended with its own title is another L1.
- Exam/practice problem statements (printed or handwritten, "1. 设…", "(Ⅰ) 证明…") are content, not headings, even
  when one problem fills a sheet; a doc of only such sheets has no nodes (65cdc9d3).
- Forms where the whole page is one table element: no nodes (375d58b7).
- Course notes (50d71ae2): chapter 第八章 L1; section 7.3 L2 (its chapter is not printed -> under root); Chinese-numbered
  titled items (四. 常用傅氏变换对, 一. 概念) L3 in both chapters (same look; chapter 8 prints no 8.x level); own-line
  numbered property titles (2. 位移性质：) L4. Run-in labels sharing a line with a formula (1. 定义：…, 1. 线性性质: F[…])
  and numbered formula-only cases (一. ①当 f(t)=δ(t)) are text.
- Lab reports (6a1cedac, 92bf52bc): the 实验项目/实验名称 line is the L1 title; 一、二… L2; own-line numbered titles
  (1. 内容, 4. 离心技术, 4. 思考题) L3 and (1)/(2) L4; run-in "1. 原理：…" items and numbered one-line statements are text.
- Essays (92741860, 9ad83bc1): essay title L1; centred part subtitles L2 (gold had them as extra L1).
- Several printed worksheets/lessons in one doc, each with its own title, give several L1 (d1ec88bb).
- Answered exam paper without title (94af97ba): question groups 一、二、三 L1; a group label inside a group that holds
  several items (21. 计算题：…) L2; question statements and sub-questions text.
- Study notes without title (a6f48a98, edc60213): the top numbered topics / bracketed section labels ([方法突破]) are L1;
  a restarted Chinese-numbered topic (二.价格 after 四.纸币) is a new L1, not a child. When the photographed sheets
  skip the page that holds a section heading, its topics are L2 under root.
- Callout labels (启示：, [例], [中心思想], 近义词, 注：) and principle/theorem labels ending in a colon or before a
  formula (Newton公式, 最低能量原理（焓变判据）：) are text.
- Notebook No./Date headers, platform watermarks (知乎 @…, @…), student names are text.

## Open questions
- 1ae487be: content types (类型一…) run across day boundaries; a sub-type (盐与盐的反应) that resumes under a new
  day sits at L4 directly under the L2 day (level skip). Alternative: drop the day markers as headings.
- 50d71ae2: own-line "2. 位移性质：" is a node but its sibling "1. 线性性质: F[…]" is run-in with a formula, so it is text.
- 65cdc9d3 / 048c9a89: problem statements that fill a whole sheet (printed, bold) were gold titles; dropped as content.
  If the user wants one heading per problem, they return as L1 (65cdc9d3) or L3 (048c9a89).
- a6f48a98: "2. 纸币的职能：纸币能代替…" (title + statement on one line) kept as L2 because its series (1., 4.) are
  own-line headings; the (1)-(4) list under 货币 (mostly run-in) is text.
- 94af97ba: "21. 计算题：（每题5分…）" kept as L2 while questions 22-24 are text.
- edc60213: the photos are out of order (notebook pages 5, ?, 3), so topic 4 precedes topics 2-3 in reading order.

## Label rule pass
- Numbered own-line topics ending in a colon (四. 常用傅氏变换对：, 1. 含义：, 2. 集合问题与数形结合思想：) are numbered
  outline headings, not labels: kept.
- Own-line labels naming a specific principle with its content under them (最低能量原理（焓变判据）：, Gibbs 函数(变)判据：,
  eac2d8c2) are now headings one level below their section (L3). This replaces the earlier "principle labels are text".
  Unnumbered own-line 拉普拉斯逆变换： (50d71ae2) stays a heading for the same reason.
- Lead-ins before a formula (对于一般的化学反应：, 任意状态下：) and margin notes in another ink (连续函数：) are text.
- An own-line label whose siblings in the same series are run-in labels (机场： among 首站：/火车站：/景区交通：, 01fba5b3)
  is text: it only wrapped its content to the next line, same look as the run-in siblings.
