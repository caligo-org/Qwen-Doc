# goverment learnings

## Decisions
- Regulations/laws without chapters (第N条 run-in at paragraph start, bold label + body in one box): only the title is a
  heading; articles are run-in labels = text. Adoption/promulgation note in parentheses under the title = text.
  (goverment_chn_48775ee4)

- Chinese State Council regulations (国务院令 family, 11 docs): L1 title; L2 第N章 (centered, spaced 总  则 kept as
  one-space join); L3 第N节; 第N条 run-in = text; (一)… items = text. Gold was already correct in all of them.
- Declassified cable (eng_04fdb344): no heading at all (empty tree). SUBJECT: is a field inside the header block.
- Newsletter without sections (eng_10e7766e): masthead L1; stories and titled sidebar boxes L3 directly under the
  masthead (level skip, as magazine learnings); subheads / tip titles L4. "Inside this issue:" box title = text
  (index box). Sidebar "(Continued)" = continuation node of the sidebar.

- US diplomatic cables (eng_1e210e8c, _5a7b6d51, _c8c79eab, _d4c4a9f2, _aa75e353, _04fdb344): no title line;
  SUBJECT: is a field inside the E.O./TAGS block, so it is not a node. Underlined/dashed section heads are the top
  level (several L1). Underlined classification markings (SENSITIVE BUT UNCLASSIFIED) = text.
- Exchanges of diplomatic notes (eng_1fb9cb74, eng_a323eb0a): note numbers (No. 543), letterheads, DIPLOMATIC NOTE
  stamps = text.
- Budget-justification excerpt (eng_553a7f3f): account page titles = several L1; bold "Highlights:" own line = L2.
- Report cover (eng_b95bd887): act name + "Report to Congress" printed apart on the cover = one L1 with merge; title
  page repeat = text.
- State Dept directory/list publications (eng_446784ca, eng_d44e80eb): cover title L1, front sections / report
  section L2, country names or quoted convention articles L3. Bold city lead-ins of post entries = run-in text.

## Open questions
- eng_a323eb0a: TRANSLATION label above the translated reply note kept as the only node (L1); the US note itself has
  no title. Could also be text (it is a label of the translation office).
- eng_186128cd: "Relationship to HCFC phaseout:" / "Relationship with the UNFCCC ...:" are body-style lines ending
  in a colon over bullet groups; made L2 (they name the topic of the group). Could be lead-in sentences (text).
- eng_553a7f3f: "Highlights:" kept L2 (bold own line); it is close to a run-in label.
- eng_10e7766e p5: the tail of the main story's list (#75-76) falls after the p4 sidebar "(Continued)" node in
  reading order, so its breadcrumb is the sidebar; no printed head exists to resume the story there.
- eng_10e7766e p4: the main story resumes after a p3 sidebar box (reading order puts the sidebar in between). No
  printed resumption head, so its first subhead on p4 ("Video interviews") is an L3 node with continues = the story.
  Alternative would be L4 under the sidebar (wrong breadcrumb).
- eng_10e7766e: "Inside this issue:" treated as index box (text); could be read as a contents title (heading).

## Label rule pass
- eng_553a7f3f "Highlights:" is a generic label: made text (supersedes the earlier L2).
- eng_186128cd "Relationship to HCFC phaseout:" / "Relationship with the UNFCCC ...:" name the specific topic of
  their bullet groups: kept L2 (settles the open question).
