# textBook learnings

- L1 = subject title on the cover (物理, 数学 ...). Course/volume lines (必修, 第三册, 选择性必修), series line
  (普通高中教科书), publisher, editor-in-chief, approval seal: text. Title repeated on the title page: not_heading.
- Front-matter titles (preface, 目录/Contents) and chapters (第N章 + title) share the top level under L1 (L2).
- Chinese preface letter opening with a salutation printed as a heading line (亲爱的同学：): kept as the preface's
  L2 heading (it is the only title of that front-matter piece). OPEN: could be judged a salutation = text.
- "本书栏目介绍" pages: the sample-feature callouts (实验探究, 多学一点 ...) are labels -> text.
- L1 = subject title on the cover (物理, 数学 ...). Course/volume lines (必修, 第三册, 选择性必修), series line
  (普通高中教科书), publisher, editor-in-chief, approval seal: text. Title repeated on the title page: not_heading.
- Front-matter titles (preface, 目录/Contents) and chapters (第N章 + title) share the top level under L1 (L2).
- Chinese preface letter opening with a salutation printed as a heading line (亲爱的同学：): kept as the preface's
  L2 heading (it is the only title of that front-matter piece). OPEN: could be judged a salutation = text.
- "本书栏目介绍" pages: the sample-feature callouts (实验探究, 多学一点 ...) are labels -> text.
- Series line printed above the title (义务教育教科书, 普通高中教科书): text. Book type printed as part of the cover
  title (教师用书 under 体育与健康): merged into the L1 title. Grade/volume line (四年级 全一册) text.
- Teacher's books: recurring part headings inside each 节 (教学目标, 知识要点, 教学内容, 教学策略, 教学评价), printed as
  their own heading lines (badges or banner): headings one level below the 节, all at the same level even when one is
  boxed differently.
- Pinyin transliteration of the cover title (SHU XUE): text (translation of the title).
- CIP / copyright-page block title (图书在版编目(CIP)数据): colophon -> text.
- Callout boxes labelled only by their kind (思考, 归纳, 探究, 观察, ?-boxes, margin notes): label -> text.
- Exercise banners (练习, 习题N.N, Exercises) printed as their own heading line/banner after a section's teaching
  text: heading one level below that section (the exercises sit under it). 例1/解/分析 are run-in labels -> text.
- 人教 统编 high-school books: 单元 (unit) > 课 (lesson, number badge + 第N课 + title) > 框 (bar heading) > 目
  (coloured heading). Units sit at L2 beside 目录.
- Recurring activity boxes titled only by kind (探究与分享, 相关链接, 名词点击, 阅读与思考 ...): label -> text; terms
  inside a term box -> text. Diagram/infographic banners (三种建国方案 over a 3-column chart) -> figure title, text.
- Picture-book lessons (primary 美术 etc.) print the lesson title only in the top band (第N课 badge + title as header
  boxes). The band on the lesson's FIRST page = the lesson heading (badge and title merged); the same band on later
  pages = running head (text). OPEN: relies on header boxes being headings.
- Pinyin ruby above characters (mù lù / 目录) is dropped from node text.
- Primary-book box labels (小窍门, 学习建议, 生活中的设计, 走进博物馆, 老师的话, 儿童美术馆, 访问艺术家): text.
- Credits line on the colophon page (义务教育教科书·美术 over the editor list): text.
- Module/volume name under the subject title (绘画, 政治与法治, 必修3): text, title stays the subject alone (consistent
  across docs). OPEN: a module name like 《绘画》 is arguably part of the book title.
- 第N课 label + lesson title in separate boxes: one node with merge.
- Recurring lesson-part headings printed on their own line (问题导入, 学习目标 ...) that open a lesson: headings at the
  level of the lesson's first numbered sections (一、), not lower, to avoid a level skip. OPEN (could be label).
- Lesson number badge printed apart from the lesson title (big "1" beside 土壤里有什么): merged into the title node,
  text "1 <title>" (badge first, as read).
- Activity-kind labels with an icon (科学实践, 探究, 活动) followed by an activity title line: the kind label is text,
  the activity title (探究土壤的成分) is the heading one level below the lesson.
- English twin of a Chinese front-matter title (CONTENTS on the second 目录 page): text (translation of a title).
- Gold often nests same-style 目 headings under the first one (北京人/山顶洞人 under 我国境内的早期人类): same look =
  same level, made siblings.
- 语文 unit opener: 活动任务单 banner = heading under the unit; 任务一/二/三 printed as own boxed lines = headings under
  it (their names are run-in bold leads of the following paragraph, so the number line stands alone). OPEN.
- Back cover: series banner and series book list are text. 内容提要 (summary on the copyright page): front-matter
  heading at L2; the CIP block on that page stays text.
- Topic-opening panels of small icon labels over bullet lists (问题导引 | 学习目标 | 核心术语 side by side): labels
  -> text. Contrast: teacher-book parts that run full-width in sequence as the section body are headings.
- Box = kind label + title (探究 | "资源赌约"谁会赢; 阅读 | 稀土资源): kind label text, title heading one level below
  the section the box sits in.
- Node text: a title printed on two lines in one box is joined with ONE space even in Chinese (高等数学 习题全解指南),
  as the brief says; letter-spaced titles keep the gold spacing (目 录 -> as gold text).
- 学习目标 printed as a small label over a short objective list at the start of a 节: kind label -> text (same as
  the 问题导引/学习目标 panels). OPEN vs teacher-book parts.
- Fix OCR damage in node text only where the gold text duplicates/garbles the printed line (第一章 章 -> 第一章).
- English open textbooks: cover-image note ("On the cover:"), copyright/license page labels ("You are free:",
  "Under the following conditions:"): text. Subtitle restated inside the acknowledgements: text.
- Code-listing captions ("Octave Script 1: Example"): caption -> text. Revision-history entries (year + "SECOND
  EDITION" small caps + bullets): list-entry labels -> text. OPEN.
- A front-matter page titled "Copyright" printed as a page heading (same style as Acknowledgements): kept as L2
  front-matter heading; labels inside a license text stay text. OPEN.
- English open textbooks (Pressbooks): gold flattens two heading weights; bold semibold heads = sections, lighter
  regular-weight heads of the same size = subsections under the preceding bold head (look decides).
- Margin labels "Definition 0.1", "Example" beside definition/example blocks: run-in labels -> text.
- Repository cover sheets (ScholarWorks: logo, "Recommended Citation", "Authors"): text; L1 = the first printed
  title; the book's own title page later is a repeat. Project/author-group line under a title: text.

## Label rule pass
- Salutation opening a front-matter letter (亲爱的同学： / 亲爱的伙伴： / 同学们：) under a printed letter title (致同学,
  致同学们, 编者的话): text, left as is.
- Salutation that is the letter's only title, printed large in title position (textBook_chn_028c0453 亲爱的同学：):
  kept as the L2 front-matter heading (real section title ending in a colon). OPEN.
- License-page lead-ins ("You are free to:", "Under the following terms:"): text, left as is.
