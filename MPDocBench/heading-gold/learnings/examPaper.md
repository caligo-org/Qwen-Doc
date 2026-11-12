# examPaper learnings

## Decisions (carry over)
- L1 = exam title (one node). Subject/time/score lines, school names, "绝密★启用前", name/seat fields are text.
- Parts (第Ⅰ卷/第Ⅱ卷, Paper 1/Section A) = L2; question groups (一、选择题 …) = one level below the part (L3), or L2 when
  the paper prints no parts. A question-group line keeps its whole printed line as node text, including the scoring
  instruction ("本大题共10小题，每小题5分…") when it is in the same box.
- Questions are headings only when the number is printed as its own line above the question body
  (e.g. "16.（本小题满分12分）" alone on a line): one level below the group. Inline-numbered questions are text.
- Long-answer questions: the question is a heading when its stem stands alone as a short line that carries no problem
  content of its own -- number + score + a read-and-answer instruction ("26. 阅读材料，完成下列问题。",
  "12.（36分）阅读图文资料…完成下列各题", "27. 读长江经济带略图，完成下列问题。(10分)") -- and the materials and
  sub-questions (1)(2)… follow below. Level: one below the group (or below L1 when no group is printed). A stem that is
  itself the problem statement ("9. 某同学利用图（a）所示装置…") is text, as are multiple-choice/fill-in questions.
  Applied to examPaper_chn_7603db95 (gold had it), _439ef81c, _6b10fb9e, _707b861f (gold missed them).
- 材料一 / 材料二 labels before quoted material are run-in labels: text.
- A group line split over a page break is one node with `merge` (examPaper_chn_4d42e18f #83+#85).
- A title printed on two lines in two boxes (exam name / subject "地理试题") is one L1 with merge.
- Front instructions (考试注意 / 注意事项 / Instructions) are text, and so is the "参考公式：" label in the front
  instruction block (examPaper_chn_0d1c391f).
- IMO-style papers (logo + "Language: … Day: N" header boxes, date line, bold run-in "第N题."/"Problem N." labels):
  no printed title, so the tree is empty. Day headers are running heads, dates and time/score footers are text
  (examPaper_chn_85419d1b, _99ec4b2c, _b2d34828, _d80f262e).
- Elective modules ([物理—选修3-3], [选修4-5…]) are one level below the 选考题 group; when the first module label is
  printed inside the group's own box, the later module still goes one below the group (examPaper_chn_ab4971e2 #104).
  A module line numbered like a group ("四、[物理--选修3-4]") stays at group level.
- Solution sheets without a document title: same-styled section titles ("Solution", "Grading guidelines",
  "Acknowledgement", "Literature") are all L1 (examPaper_eng_08960d5c, _2122d135); back matter is never a child of
  "Solution". Bold run-in problem labels (A1., B2., Problem 3, Answer., Solution 1., Lemma.) and italic labels
  (Marking breakdown, 1st method, Theoretical part:) are text; a set made only of them gets an empty tree
  (examPaper_eng_23110a64).
- A title reprinted at the top of the second day/part page is text (not_heading); "Day I"/"Day II" lines printed in
  the body under the title are part headings, L2 (examPaper_eng_658ab010). Contrast: "Day: 1" inside the page-header
  box of IMO papers stays a running head.
- A bold stand-alone "Questions:" line that separates a problem's narrative from its numbered tasks is L2 under the
  problem title (examPaper_eng_58d7c49f, gold had it).
- Appended documents inside an exam pack (e.g. a contest's AI-use policy after the problem statement) are L2 under
  the exam title, their own sections L3 (examPaper_eng_930559d3).
- Task part numbers alone on a line ("1a.", "2b.") are labels: text (examPaper_eng_dd12c9cc).

## Open questions
- examPaper_eng_930559d3: "Report on Use of AI" (centred bold, inside Citation and Referencing Directions) set to L4;
  by look alone it could be L3 beside the other policy sections.
- examPaper_eng_2122d135: "Acknowledgement" (centred) and "Literature" (left) differ in look; both set to L1 as back
  matter of an untitled solution sheet.
- IMO papers: "Day: 1"/"Day: 2" could be read as the two parts (several L1 nodes, like untitled handwritten
  notes); treated as running heads, so these trees are empty.
- Long-answer question stems as headings (rule above): gold is inconsistent (marked in 7603db95, not in 6b10fb9e,
  707b861f, 439ef81c). The line between "generic stem" and "problem statement" is a judgement call.
- "参考公式：" (reference formulas) block label in the front matter: treated as text with the instructions; it could be
  argued as a front-matter section title.

## User decisions (2026-11-12)
- examPaper_eng_930559d3: CONFIRMED L4. "Report on Use of AI" is the sample section shown in the examples of
  "Citation and Referencing Directions" (L3), so it is that section's child even though it shares the L3 style.
- CONFIRMED by user: individual questions are never headings (16./（17）/26. stems, "本小题满分", "读图文资料，回答下列问题").
  Headings in exams = title, parts, question groups. Applied to 0d1c391f, 3e46326d, 439ef81c, 6b10fb9e, 707b861f,
  7603db95, 9627d6c2, ff633fd0. Single-problem documents (eng_4f919f1c, eng_d1508a80) keep the problem title as L1.

## Label rule pass
- Numbered question-group heads ending in a colon ("（一）必考题：", "（二）选考题：") are outline headings: kept.
- Generic own-line labels are text: "Questions:" in an IPhO problem (eng_58d7c49f, removed, supersedes the earlier
  L2 decision), "Solution:", "Numerical solution:", "Numerical results:", "Marking breakdown:", "Theoretical part:" /
  "Experimental part:" in a marking scheme.
- Lead-ins ("Consider the following call:", "The game proceeds as follows:", "where:", "Thus:", "回答下列问题：",
  "Teams are required to:"), form fields (姓名/准考证号, 考生姓名/学号) and question stems (第5题…) stay text.
- Bold section titles of an appended pack that end in a colon ("Files provided:", "Glossary of key terms/concepts:",
  "References:", eng_930559d3) are real section titles: kept.
