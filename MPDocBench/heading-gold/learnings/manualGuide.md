# manualGuide learnings (v3, from scratch)

## Recurring decisions
- Bilingual heading = one union (zh line + en translation line), never a child level. Bilingual title the same.
- Coloured own-line lines ending in a colon that head a separate bullet block and subdivide a section ("短期使用…原因可能包括：")
  are label headings one level below the section they sit in (content beats look), even if styled like the sections.
  (chn_1a73fd62)
- Bold black body-style statements/lead-ins ("他們還是可以接受其他的治療。", "和您的醫生…討論：") are text: not the
  heading style of the document, run-in key statements / list lead-ins. (chn_1a73fd62)
- Publisher logo/header, 備註/Note boxes: text.
- Excerpt with no printed document title (chn_51cbdc09, starts at 序言 under company/department/doc-class running heads):
  chapters are the top level (several L1), 目录 at L1 beside them. Running heads naming the company are text, not a title.
- Running heads that name the document (model + manual type + version, chn_57d8d5ed) are repeats of the L1, one repeat
  entry per page. Running heads that are only the company name, a slogan or a doc-class tag are text.
- Own-line noun-phrase labels ending in a colon with their own bullet block ("EM9161主要特点：", "关于CN1中相关信号的进一步说明：")
  are label headings one level below their section (v3 label rule). Sentences that lead into a table/list ("下表是…",
  "对DIO操作，CN2各管脚的定义如下：", imperative "和您的醫生…討論：") stay text.
  OPEN: chn_57d8d5ed #8 "EM9161主要特点：" sits in unheaded front matter; after its bullets come intro/contact paragraphs
  that are not about features but fall in its subtree.
- Numbered outline headings boxed as figure / table_caption ("2、外形尺寸", "3.1 … 信号定义") are nodes (TOC confirms).
- Numbered paragraphs printed as body text (1.1 本文档…) are text. 修改记录 and 目录 are front-matter headings at the
  level of the chapters.
- Camera manual cover (chn_59b90f53): brand logo (FUJIFILM) text; product line+model ∪ 用户手册 = L1. Front-matter
  regulatory sections (■ RoHS table, 标志的含义, packaging marks) are L2 beside 目录/关于本手册 although printed in the ■
  style of L3 subheads (front matter at top-section level). OPEN: same-look rule vs front-matter rule.
- A titled box on the contents pages (可使用的存储卡) = L3 under 目录 (box one level below the section it sits in).
- Running section tags in the page header (目录, 关于本手册) = repeats of the open section.
- ■注 / 注意 / 重要 callout labels inside a section: text.
- AV-receiver quick guide (chn_5b94cfef): L1 = product type ∪ model ∪ manual type (开始使用), brand logo text. Black bands
  L2, grey bands L3, □ / underlined subheads L4; front-matter safety sections L2. Step badge + step title = one union
  heading; v3: step badge + short instruction sentence with its own figure block ("1" + "将主设备电源插头插入电源插座。")
  is also a step heading (procedure step with its own block). Rounded box titles = one level below their chapter.
- Side thumb tabs listing all chapters (准备/安装向导/播放) on every page are a tab index: text, not repeats.
- 目录 printed inside the first body chapter page (under the 使用必读 band) = L3 inside that chapter (placing it at L2
  would wrongly parent the chapter's sections under 目录). OPEN.
- A running head on the cover page printed above the title cannot be a repeat (precedes the L1 in reading order): text.
- OPEN (chn_9818e751 p13): 通行级别选项： is an own-line label with a list, but its sibling 防潜回： is run-in in the same
  style; both text so the 防潜回 block is not filed under 通行级别选项.
- Numbered chapter 八、注意事项 is an outline heading, not a callout (chn_894ac6ec).
- EIZO manual (chn_bf4b003e): cover 用户手册 ∪ model ∪ product type = L1. Chapter band L2, "1-1." L3, grey-rule subhead
  L4, ● item one level below its parent: v3 levels do not skip, so a ● printed directly under "1-3." is L4 while a ●
  under a grey-rule subhead is L5 (same look, different level; differs from v2). Running footers naming the open
  chapter / contents = repeats.
- Datasheet (chn_db260b52): company logo above the title is text; "AC781x 数据手册" = L1; running heads naming it are
  repeats. v3 change: bullet group labels ("• 操作特性") each with its own ➤ sub-list are label headings one level below
  the chapter (own-line label + own block), unlike v2.
- Laptop chapter excerpt (chn_f3fc9a26, no document title): chapter is L1. Numbered hardware groups with bullets under
  them are L3 under 1.2; "11. 84 键 Windows 键盘" has no block of its own before the next item: text (substance; v2
  added it).
- Product brochures (eng_3135929d): L1 = product model on the cover (brand logo and slogan text; no document type is
  printed, so the union is the model alone). Headlines L2, feature titles in a box L3, step captions / picture labels
  text. A spec page title printed in the header slot that names only that page is that section's heading (L2).
  OPEN: slogan vs model as L1 for brochures.
- Building-product sheet (eng_6202ad06): L1 = INSTALLATION INSTRUCTIONS ∪ product line. The product line restated at the
  top of the installation pages / page header is a repeat of the title (v3: title restated = repeat), so installation
  sections, warranty and care parts are all L2. Bold all-caps opening sentences of clauses and run-in labels (GENERAL:)
  are text.
- Garmin installation instructions (eng_62437ad7): blue heads L2, bold black L3, bold italic L4. WARNING / CAUTION /
  NOTICE bands are callouts (not_headings). v3 change: table labels that repeat under each spec section (Transmit /
  Receive / Transmit and Receive) are label headings one level below their section (own block = the table).
- AEG booklet (eng_714e4656): v3 change: the button/control sections (Programme selector dial, ON/OFF button, ...) are
  printed like "Control panel" but the TOC indents them under it and they describe its parts: L4 under Control panel
  (content beats look). a) Start / b) Pause L5. Small "Contents" labels on continuation pages = repeats. "Important"
  welcome-page box label = callout.
- Spec sheet (eng_8cb5bbc3): L1 = product name (the copy under the photo; the identical header band before it in
  reading order is text, later header / side-band copies are repeats). All grey-band spec heads L2; a section resumed
  on a later page after other sections ("Optional Accessories" p3) is a continuation. Form fields (APPROVAL:, Item No.) text.
- KRONE (eng_8d2c3220): L1 = Originele handleiding (box also holds the document number) ∪ product/models box.
  "EG-conformiteitsverklaring" (header box) = L2; "Voorwoord" exists only as the running header of p3 = that section's
  heading (v3 rule: a section-only running header is the section's heading). Manufacturer name, salutations, form
  fields, "Let op, belangrijk:" are text.
- LEGO set booklet (eng_a2fab9df): set name L1; spread headlines L2; spaced kicker and the big pull quote text. OCR "?"
  for accented letters fixed in node text.
- Quick installation guide (eng_a3556095): brand logo text; L1 = Quick Installation Guide ∪ product/model. Per-language
  copies of the same section are sibling sections (not continuations). Note / Advertencia labels = callouts.
  Consistency rule used throughout: a brand logo printed apart from the title line is text (not part of the union);
  OPEN whether owner logos should join the L1 union ("whose document").
- Epson quick guide (eng_a7839445): title line holds product + document type; model code band and logo text. Problem
  heads printed inside the Troubleshooting table have no element id: left out (OPEN: no box to annotate).
- Generac (eng_c53257de): ADVERTENCIA bands = callouts; L1 = 'Manual del usuario' + product line (one box); running
  footer naming the manual = repeats; bold lead-in sentence above the data form = text.
- Garmin product sheet (eng_c8f5395d): same brochure rule (model L1, cover slogan text, page-2 header model = repeat);
  ▶ heads L2; feature boxes L3 with their bold one-line subtitle as a deck (text).
- Garmin owner's manual (eng_e19dcc01, French): cover box GARMIN + Oregon Série 700 (brand printed inside the title
  box) ∪ Manuel d'utilisation = L1. Chapter large black L2 (Table des matières L2 beside them), blue bold L3, black bold
  L4, bold italic L5 (matches the TOC indents). Running footers naming the chapter/contents = repeats. Signal-word
  bands = callouts.
- Endress+Hauser (eng_e665d244): ISO safety messages: signal word and the bold hazard name are both callout text.
  L1 = Einbauanleitung / Installation Instructions ∪ Kit CSF48. Running section tags printed above the chapter heading
  on the same page come before it in reading order: text; later ones are repeats. German then English part: all
  chapters L2 in reading order.

## Breadcrumb review round 2
28 docs, 454 breadcrumb paths read; no unseen pages listed for this category.
Fix classes:
- Wrong parent by content (2 docs, 6 nodes): eng_62437ad7 connector views / Software Update / Tools Needed were filed
  under "Important Safety Information" (bold black L3 after the blue safety head) though they are not about safety ->
  L2; the Chinese substance declaration (物質宣言) was under Specifications -> L2 regulatory back matter.
  chn_5b94cfef #197 将电源切换到待机状态后 (standby block at the foot of the 开始使用 panel) was under step 2 -> L4 beside
  the steps under 开始使用.
- Containment, label -> text (1): chn_57d8d5ed #8 "EM9161主要特点：" (round-1 OPEN resolved): the scope, website and
  contact paragraphs after its bullets are not features and no heading closes it -> considered (list lead-in).
- Node text cleanup (4 docs, 9 nodes): line breaks in L1 union text (eng_2ea63cc3, eng_714e4656, eng_8d2c3220) and
  OCR "\(\bullet\)" markup in chn_bf4b003e node texts.
ACCEPTED manualGuide_chn_51cbdc09 #16 ROOTS: excerpt with no printed document title; chapters and 目录 are the top level.
ACCEPTED manualGuide_chn_5b94cfef #41 FRONT: 目录 is printed inside the 使用必读 band page; as L2 it would swallow the
  chapter's sections (round-1 decision).
ACCEPTED manualGuide_chn_894ac6ec #61 SEQ: the document prints "2、异常及处理方法：" after "2、控制器面板按键说明：" (checked p5).
ACCEPTED manualGuide_chn_f3fc9a26 #11 #15 #18 #20 #22 #24 #27 #31 #33 #35 #40 DEEP: "1. CPU" … "12. 扩展接口" are the
  numbered hardware items listed inside 1.2, a list numbering, not section numbers.
ACCEPTED manualGuide_chn_f3fc9a26 #40 SEQ: "11. 84 键 Windows 键盘" has no block of its own (text by substance).
Kept although the step is not narrower: chn_59b90f53 目录 › 可使用的存储卡 (titled box on a contents page; box rule).
OPEN: eng_43edf789 L1 is "New Product Announcement" alone; the owner (Garmin) is only a logo and "Marketing Memo" a
  header label; product (quatix) only in the photo.

## Breadcrumb review round 3
28 docs, 452 breadcrumb paths read (independent second review); no unseen pages listed for this category.
Fix classes:
- Step not narrower -> top-section level (1): chn_59b90f53 #25 可使用的存储卡 (memory-card note box on the last
  contents page) L3 under 目录 -> L2 front-matter box (round-2 "kept" case resolved; its subtree is its own sentence).
- Neither placement is a breadcrumb -> text (2): chn_5b94cfef #41 目录 (contents box inside the 使用必读 page): L3 under
  使用必读 is no breadcrumb (the contents lists 使用必读 itself), L2 would swallow 附件/关于本手册/操作注意事项
  (containment) -> considered; FRONT flag gone. eng_6202ad06 #169 CONTACT US on the care page (contact box) -> text.
- Containment closed by a sibling item (1): chn_f3fc9a26 #39 "11. 84 键 Windows 键盘": as text it fell into
  "10. 外置设备" (a built-in keyboard is not an external device) -> L3 node like items 1.-12.; SEQ flag gone.
- Title = owner + document type (1): eng_43edf789 L1 = GARMIN logo box (#2, the only print of the owner; quatix only in
  the photo) ∪ New Product Announcement (round-2 OPEN resolved).
- Node text cleanup (4): OCR "?" -> ® / ™ (chn_bf4b003e FORIS®, eng_c8f5395d nüvi®, eng_6202ad06 SHAKE™); missing space
  in eng_8cb5bbc3 L1; eng_8d2c3220 L1 drops the document number printed in the same box (heading part only).
Brand logos and title unions (known case): manual rule = product ∪ manual type (∪ brand only when printed as part of the
  title). Kept: a brand logo printed apart from a title that already names the product (FUJIFILM, DENON, EIZO, TruWood,
  Garmin on nüvi, Electrolux) stays text; the product is the "whose" of a manual. The logo joins
  the union only when nothing else names whose document it is (eng_43edf789).
ACCEPTED manualGuide_chn_51cbdc09 #16 ROOTS: excerpt with no printed document title; chapters and 目录 are the top level.
ACCEPTED manualGuide_chn_894ac6ec #61 SEQ: the document prints "2、异常及处理方法：" after "2、控制器面板按键说明：" (p5).
ACCEPTED manualGuide_chn_f3fc9a26 #11 #15 #18 #20 #22 #24 #27 #31 #33 #35 #39 #40 DEEP: "1. CPU" ... "12. 扩展接口" are
  the numbered hardware items listed inside 1.2 (list numbering, not section numbers); each path reads
  第一章 系统简介 › 1.2产品硬件配置 › <item>.
Checked and kept: eng_e19dcc01 Présentation de l'appareil › Informations sur la batterie / Mise en place de piles AA
  (the TOC on p3 indents them under it); eng_e665d244 "3 Identification" (printed number).
OPEN: none concerning paths. Still open from round 1 (not path cases): brochure L1 slogan vs model (eng_3135929d,
  eng_c8f5395d).

## User decisions (2026-11-13)
- Owner printed only as a logo: the logo's text joins the L1 union when the layout has a box holding that text
  ("DENON" ∪ product ∪ manual type); a logo with no text box stays out. Replaces the round-3 "brand logo printed apart
  from the title stays text" rule. Boxes in reading order; a mixed logo box (logo + tagline) contributes only the name.
  Changed (old -> new L1; logo box removed from not_headings/considered; node id re-pointed where the logo comes first):
  - chn_1a73fd62: + header #3 COALITION FOR COMPASSIONATE CARE OF CALIFORNIA (publisher logo, after the title).
  - chn_59b90f53: + FUJIFILM #1 (id #2 -> #1). The title already says 富士; joined because the brand is printed only
    as the logo wordmark.
  - chn_5b94cfef: + DENON #1 (id #2 -> #1).
  - chn_db260b52: + AutoChips #1 (id #2 -> #1).
  - eng_3135929d: + brother #2 ("at your side" tagline dropped from the text): "DCP-T500W brother".
  - eng_6202ad06: + TruWood® SIDING header #1 (id #2 -> #1).
  - eng_62437ad7: + GARMIN® #1 (id #3 -> #1).
  - eng_714e4656: + AEG Electrolux footer logo #7 (last in reading order; the "AEG" figure #5 is the drawing's panel).
  - eng_8cb5bbc3: + Electrolux header #2 (id #6 -> #2).
  - eng_a3556095: + TRENDNET #1 (id #2 -> #1).
  - eng_a7839445: + EPSON header #2 (tagline dropped; id #3 -> #2).
  - eng_c53257de: + GENERAC header #1 (id #2 -> #1).
  - eng_e665d244: + Endress+Hauser footer #9 (slogan dropped; last in reading order).
  Unchanged, logo without a text box: chn_57d8d5ed, chn_561cd575 (AiLink header image), chn_9818e751, chn_bf4b003e
  (EIZO footer image), eng_8d2c3220 (KRONE header image; figure #5 "KIONE" is the machine photo), eng_a2fab9df (LEGO),
  eng_c8f5395d (Garmin header image). Already named: eng_2ea63cc3 (ASUS), eng_43edf789 (GARMIN), eng_e19dcc01
  (GARMIN). No printed title: chn_51cbdc09, chn_f3fc9a26. chn_b4d94d54 prints its owner as a plain text line, not a
  logo (out of this decision's scope).

## Round 4 (full path review, 2026-11-13)
28 docs, 171 pages rendered with the tree and read, 453 nodes checked (review/paths/manualGuide.tsv): 449 OK, 1 ADDED,
3 FIXED, 0 REMOVED.
- Containment, missing front-matter heading (1): chn_5b94cfef #30 「根据电子信息产品污染控制管理办法的有毒·有害物质或元素的
  标识表」 (bracketed table title on p II) was considered text, so the substance table and 10-year mark notes fell under
  安装注意事项 -> L2 front-matter node (same treatment as chn_59b90f53 #28).
- Node text = printed heading (3, eng_e19dcc01): #11 "Table des matières", #102 "Verrouillage de l'écran tactile" (OCR
  "I'ecran"), #153 "Connect IQObjets" (printed so in heading and TOC; tree had "IQObjects").
OPEN: eng_a3556095 #16 / #40: the untitled per-language LED-status notes + tables printed below each Declaration of
  Conformity panel fall under "Declaration:" / "Déclaration:"; no printed heading closes the panel. Kept.
Checked and kept: eng_3135929d the refill-angle paragraph #33 precedes its headline #34 in reading order (column
  layout); eng_e19dcc01 #57 "Informations sur la batterie" repeated under "Mise en place de piles AA" (printed there,
  bold italic, with its warning bands).
