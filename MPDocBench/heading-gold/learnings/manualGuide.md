# manualGuide learnings

## Decisions that recur
- Bilingual headings (Chinese line + English translation line, or both on one line): ONE heading; the translation box is
  merged (rule 6), never a child level. Same for a bilingual document title.
- Bold black list lead-ins ("和您的醫生和醫療團隊討論：") printed in body style, unlike the coloured section headings:
  text (run-in label). (chn_1a73fd62)
- Single-sheet guides with one tier of coloured section headings: L1 title, L2 sections.

## Open questions
- Excerpts that start mid-manual with no printed title (chn_51cbdc09 starts at 序言): chapter titles are the top level,
  several L1 nodes (as the handwritten rule). Running heads (company/department/document-class) are text.
- Numbered paragraphs printed as body text (1.1 本文档..., 2.1 ...) are text; only numbered lines printed as heading
  lines are headings. Revision-history title (修改记录) and 目录 are front-matter headings at L2. (chn_561cd575)
- Section headings boxed as figure / table_caption (e.g. "2、外形尺寸" as figure, "3.1 ... 信号定义" as table caption)
  are added as nodes when they are numbered outline headings (check the TOC). (chn_57d8d5ed)
- Body-size lines ending in a colon that introduce a bullet list ("EM9161主要特点：", "关于CN1中相关信号的进一步说明：")
  are text (list lead-ins), even when the gold marked them.
- Camera-manual cover (FUJIFILM / 富士数码相机 FINEPIX Z200fd / 用户手册): brand logo is text; product line + manual type
  merge into one L1. Front-matter pages (TOC, notice box, RoHS/标志 sections) are L2 even when printed in the ■ style that
  marks L3 subheads in the body, because front matter sits at the level of the top sections. (chn_59b90f53)
- ■-styled "注" (note) lines are callouts: text.
- Strip OCR artifacts of bullets ("| 关于...") from node text; fill in printed symbols the OCR dropped (十字键（▲▼◀▶）).
- AV-receiver quick guide (chn_5b94cfef): cover = brand logo (text) + product type + model + manual type (开始使用);
  product + model + manual type merge into L1. Black full-width bands = L2 chapters, grey bands = L3, □ lines = L4.
  Step number badge + big step title = one heading (merge); small bold step instruction sentences with badges
  ("按ENTER。") are text even though gold marked them. Rounded box titles inside a chapter = one level below it.
- OPEN: a □ heading (L4 look elsewhere) that sits directly in a chapter with no grey band above it (□连接扬声器) was put
  at L3 to avoid parenting it under an unrelated box; same-look rule broken. Same for the front-matter ■/□ safety and
  RoHS sections set at L2.
- Callout labels 注意： / 警告： / 注 at the top of a warning box: text.
- Cover with product name and a separate big "使用说明书"/"用户指南" line: merge into one L1 (chn_894ac6ec). Numbered
  section "八、注意事项" is a section heading (numbered outline), not a callout label.
- EIZO monitor manual (chn_bf4b003e): chapter band L2, "1-1." L3, grey-rule subhead L4, ● items L5, numbered/plain
  minor heads L6. A ● item keeps L5 even where no L4 subhead is printed above it (level skip, rule 5) — same look =
  same level wins over a dense outline. Cover "用户手册" + product logo/model + product type merge into L1.
- Grey callout box "重要" and labels 注意/注 inside a section: text.
- Strip the printed bullet glyph (●, ·, □, ■) from node text.
- Datasheet cover (AutoChips logo + "AC781x 数据手册"): company logo is text, document title L1.
- Bullet groups in a nested feature list ("• 操作特性" with ➤ sub-items, two columns) are list items, not headings
  (chn_db260b52). OPEN: they do group content and could be read as minor subheads; decided text because they are
  printed as list bullets in body size.
- Laptop manual chapter (chn_f3fc9a26): numbered hardware groups "1. CPU" ... "12. 扩展接口" with bullets under them are
  L3 under "1.2"; an item the gold missed (#39 "11. 84 键 Windows 键盘", one line, no bullets) is added by numbering.
  OPEN: #39 is printed lighter than its siblings.
- ASUS motherboard cover (eng_2ea63cc3): model (PRIME H510M-K, a text box) + product type on the spine ("Motherboard")
  merge into L1; the ASUS logo is text. The bold "Copyright © ..." sentence at the top of the legal page is text.
- Product brochures (eng_3135929d, Brother DCP-T500W): L1 = the product model on the cover; the big cover slogan
  ("Smarter Design. Bigger Savings.") is text (slogan). Blue section headlines L2; feature titles inside a box
  (Print Speed / Wireless / Mobile Printing) L3; picture labels ("Other Printers", "Potential spillage") and step
  captions are text. "<model> Technical Specifications" boxed as header is a real L2 section. OPEN: slogan vs model as
  L1 for brochures.
- Memo-style announcements (eng_43edf789): memo-type label ('Marketing Memo') and To/From block are text; the
  announcement headline is L1. Fix OCR'd trademark glyphs in node text (quatix™).
- Building-product installation sheet (eng_6202ad06): L1 = manual type + product line from the cover. The product name
  restated at the top of the installation pages kept as L2 (same look as the warranty and care part heads, which are
  L2); OPEN: rule "title repeated later = text" vs keeping the three parts parallel. Bold all-caps opening sentences
  of warranty clauses are text. Run-in labels (GENERAL:, INSPECTION:) are text.
- Garmin installation instructions (eng_62437ad7, eng_e19dcc01 family?): blue heads L2, bold black heads L3, bold
  italic heads L4 (e.g. "Connecting to Power"), small bold table heads (Transmit/Receive) one below their section.
  Coloured callout bands WARNING / CAUTION / NOTICE are always text, whatever level the gold gave them.
- AEG washer-dryer booklet (eng_714e4656): large bold = L2, medium bold = L3, small bold "a) Start" = L4. Cover models +
  product type + "Instruction Booklet" merge to L1. "Important" box label and the small "Contents" labels on TOC
  continuation pages are text. OPEN: the TOC indents the button sections (ON/OFF button, ...) under "Control panel",
  but they are printed in the same style as "Control panel"; kept at the same level (look decides when unnumbered).
- Equipment spec sheets (eng_8cb5bbc3, Electrolux): L1 = product name; grey-band section heads all L2 (also the
  page-2 spec bands Electric / Water: / Installation: ...). A section resumed on a later page after other sections
  ("Optional Accessories" p3) is a continuation node (continues the first one).
- KRONE Dutch manual (eng_8d2c3220): L1 = "Originele handleiding" + product/model box; the document number on the
  title line and the logo are text. "EG-conformiteitsverklaring" (boxed as header) is the declaration's heading (L2);
  the manufacturer name inside it is text. Salutations ("Zeer geachte klant !") and "Let op, belangrijk:" are text.
  OPEN: "Voorwoord" exists only as the running section tag of p3; added as L2 because nothing else names that section
  (analogous to the newspaper section-change exception).
- LEGO set booklet front matter (eng_a2fab9df): set name L1; spread headlines L2; the spaced kicker above a headline
  ("LA TÊTE DANS LES ÉTOILES") and the big pull quote with attribution are text. Fix OCR "?" for accented letters.
- Quick installation guide (eng_a3556095, TRENDnet): brand logo text; L1 = "Quick Installation Guide" + product/model
  lines. Per-language repeats of the same section (Declaration of Conformity / Déclaration de conformité, Limited
  Warranty in 8 languages) are separate sibling sections, not continuations. "Note" / "Advertencia" labels are text.
- Epson quick guide (eng_a7839445): title line already holds product + manual type; model code band (ELPCB02) and
  logo are text. Problem headings printed inside a table cell (Troubleshooting) have no element id; left out.
- Garmin product sheet (eng_c8f5395d): same brochure rule as eng_3135929d (model L1, cover slogan text); ▶ section
  heads L2; feature boxes (nüRoute™, PhotoReal™) L3 with their bold one-line subtitle as a deck (text). The Generac
  cover (eng_c53257de): ADVERTENCIA bands text, L1 = 'Manual del usuario' + product line (already one box).
- Garmin owner's manual (eng_e19dcc01, French): chapter (large black) L2, blue bold L3, black bold L4, bold italic L5;
  matches the chapter/section nesting of its TOC. Cover: product name + "Manuel d'utilisation" = L1, logo text.
- ISO-style safety messages (eng_e665d244, Endress+Hauser): signal word (GEFAHR/WARNUNG/VORSICHT/HINWEIS) and the
  bold hazard name under it ("Herabstürzende Lasten") are both part of the callout: text. Cover "Einbauanleitung /
  Installation Instructions" + product "Kit CSF48" merge into L1. Bilingual manuals with a German part then an English
  part restarting the numbering (1 Identifizierung ... 3 Identification): all chapters L2, in reading order.

## Label rule pass
- Numbered section titles ending in a colon (一、概述：, 1、控制器显示说明：, chn_894ac6ec) and grey-band spec section
  titles (Water:, Installation:, Capacity:, Key Information:, eng_8cb5bbc3) are outline headings: kept.
- Lead-in sentences in heading style that introduce a list ("短期使用人工呼吸器的原因可能包括： Reasons for short-term
  ventilator use may include:", chn_1a73fd62) -> text; both language boxes go to not_headings.
- Table labels that repeat under each spec section (Transmit / Receive under NMEA 2000 and NMEA 0183, eng_62437ad7) ->
  text, like Advantages/Disadvantages; "Transmit and Receive" (#196, same class) removed with them.
- Specific own-line labels in a declaration or spec block (Product Information:, Declaration:, eng_a3556095;
  笔记本电脑重量尺寸：, chn_f3fc9a26) are headings: kept.
- A section title that recurs in two places of the real outline (Informations sur la batterie, eng_e19dcc01) is not a
  generic label: kept.
- Text left as text: list/table lead-ins (用于…的电缆如下：, 适用于以下产品：, This button has two functions:, Vous allez
  aimer :, Im Lieferumfang sind enthalten:), step instructions ("2 Complete an action:", "4 Sélectionnez une option :"),
  form fields (APPROVAL:, Signature:, Bouwjaar:, Machinenr:), callouts (Let op, belangrijk:).
- OPEN: 通行级别选项： (chn_9818e751 p13) is own-line and specific, but its sibling 防潜回： is run-in in the same body
  style; left as text.
