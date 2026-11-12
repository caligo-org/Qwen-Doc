//! Port of `metrics/cdm/modules/latex2bbox_color.py`: tokenize + colour every token,
//! render (Tectonic, or `xelatex`), rasterise (hayro, or `magick`), crop, and recover one
//! box per token colour.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::latex_processor::{normalize_latex, token_add_color_rgb};
use crate::pil;
use crate::py::{self, path_join, PyErr, PyResult};
use crate::tokenize::{tokenize_latex, Tokenizer};
use crate::{raster, tex, Backend, Raster, Tex};

const FORMULAR_TEMPLATE_HEAD: &str = r"
\documentclass[12pt]{article}
\usepackage[landscape]{geometry}
\usepackage{geometry}
\geometry{a<PaperSize>paper,scale=0.98}
\pagestyle{empty}
\usepackage{amsmath}
\usepackage{upgreek}
\usepackage{amssymb}
\usepackage{xcolor}
\usepackage{xeCJK}
\setCJKmainfont{Source Han Sans SC}
\setCJKsansfont{Source Han Sans SC}
\setCJKmonofont{Source Han Sans SC}
\xeCJKsetup{CJKmath=true}
\begin{document}
\makeatletter
\renewcommand*{\@textcolor}[3]{%
  \protect\leavevmode
  \begingroup
    \color#1{#2}#3%
  \endgroup
}
\makeatother
\begin{displaymath}
";
const FORMULAR_TEMPLATE_TAIL: &str = r"
\end{displaymath}
\end{document}
";

/// `CDM.gen_color_list(num=5800)`
pub fn gen_color_list(num: usize, gap: usize) -> Vec<[u8; 3]> {
    let num = num + 1;
    let single_num = 255 / gap + 1;
    let max_num = single_num.pow(3);
    let num = num.min(max_num);
    (1..num)
        .map(|idx| {
            let r = idx / (single_num * single_num);
            let gb = idx % (single_num * single_num);
            let g = gb / single_num;
            let b = gb % single_num;
            [(r * gap) as u8, (g * gap) as u8, (b * gap) as u8]
        })
        .collect()
}

/// `run_cmd(cmd, timeout_sec=30, temp_dir)`: `/bin/sh -c cmd`, killed after 30 s.
fn run_cmd(cmd: &str, temp_dir: Option<&str>) -> PyResult<()> {
    let mut c = Command::new("/bin/sh");
    c.arg("-c").arg(cmd);
    if let Some(t) = temp_dir {
        for k in ["TMPDIR", "TMP", "TEMP", "MAGICK_TMPDIR", "TEXMFCACHE", "TEXMFVAR"] {
            c.env(k, t);
        }
    }
    let mut child = c.spawn().map_err(|e| PyErr::new(format!("OSError: {e}")))?;
    let start = Instant::now();
    let mut sleep = Duration::from_millis(2);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return Ok(()),
            Ok(None) => {}
            Err(e) => return Err(PyErr::new(format!("OSError: {e}"))),
        }
        if start.elapsed() >= Duration::from_secs(30) {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(());
        }
        std::thread::sleep(sleep);
        sleep = (sleep * 2).min(Duration::from_millis(20));
    }
}

/// The `try:` block of `latex2bbox_color`: returns None where Python logs and returns.
fn preprocess(latex: &str, basename: &str, temp_dir: &str, total_color_list: &[[u8; 3]], tok: Tokenizer) -> PyResult<(String, Vec<String>, usize)> {
    let p = prepare(latex, basename, temp_dir, total_color_list, tok)?;
    let head = FORMULAR_TEMPLATE_HEAD.replace("<PaperSize>", &p.paper_size.to_string());
    let final_latex = format!("{head}{}{FORMULAR_TEMPLATE_TAIL}", p.rgb_latex);
    Ok((final_latex, p.token_list, p.n_colors))
}

/// A formula ready to typeset: its coloured LaTeX, paper size and tokens.
pub(crate) struct Prepared {
    pub rgb_latex: String,
    pub paper_size: u32,
    pub token_list: Vec<String>,
    pub n_colors: usize,
}

/// The template head up to (excluding) `\begin{displaymath}`, and the template tail
/// after `\end{displaymath}`, for batched documents.
pub(crate) fn template_head(paper_size: u32) -> String {
    let h = FORMULAR_TEMPLATE_HEAD.replace("<PaperSize>", &paper_size.to_string());
    h.strip_suffix("\\begin{displaymath}\n").unwrap().to_string()
}

/// `\begin{displaymath}` + formula + `\end{displaymath}` exactly as in the template.
pub(crate) fn template_body(rgb_latex: &str) -> String {
    format!("\\begin{{displaymath}}\n{rgb_latex}\n\\end{{displaymath}}\n")
}

pub(crate) fn prepare(latex: &str, basename: &str, temp_dir: &str, total_color_list: &[[u8; 3]], tok: Tokenizer) -> PyResult<Prepared> {
    let latex = latex.replace('\n', " ");
    let latex = latex.replace("\\%", "<PERCENTAGETOKEN>");
    let (ret, new_latex) = tokenize_latex(&latex, &path_join(temp_dir, &format!("{basename}.txt")), tok)?;
    let new_latex = if !ret || new_latex.is_empty() { latex.clone() } else { new_latex };
    let new_latex = new_latex.replace("< P E R C E N T A G E T O K E N >", "\\%");
    let (rgb_latex, token_list, n_colors) = color_tokens(&new_latex, total_color_list)?;
    let paper_size = if token_list.len() > 1300 {
        3
    } else if token_list.len() > 600 {
        4
    } else {
        5
    };
    Ok(Prepared { rgb_latex, paper_size, token_list, n_colors })
}

pub(crate) fn normalize_and_color(latex: &str) -> PyResult<(String, Vec<String>)> {
    let colors = gen_color_list(5800, 15);
    let (s, t, _) = color_tokens(latex, &colors)?;
    Ok((s, t))
}

fn color_tokens(new_latex: &str, total_color_list: &[[u8; 3]]) -> PyResult<(String, Vec<String>, usize)> {
    let latex = normalize_latex(new_latex)?;
    let mut token_list: Vec<String> = Vec::new();
    let mut l_split: Vec<String> = py::strip(&latex).split(' ').map(|x| x.to_string()).collect();
    let n_colors = l_split.len().min(total_color_list.len());
    let mut idx: i64 = 0;
    while idx < l_split.len() as i64 {
        idx = token_add_color_rgb(&mut l_split, idx, &mut token_list, false, 0)?;
    }
    let rgb_latex = replace_color_placeholders(&l_split.join(" "), &total_color_list[..n_colors]);
    Ok((rgb_latex, token_list, n_colors))
}

/// `for idx, color in enumerate(color_list): s = s.replace(f"<color_{idx}>", "R,G,B")`
/// (one pass is equivalent: the replacements contain commas, so they never form a new
/// placeholder, and placeholders cannot overlap).
fn replace_color_placeholders(s: &str, colors: &[[u8; 3]]) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find("<color_") {
        out.push_str(&rest[..p]);
        let after = &rest[p + 7..];
        let ndig = after.bytes().take_while(|b| b.is_ascii_digit()).count();
        let digits = &after[..ndig];
        let canonical = ndig > 0 && (ndig == 1 || !digits.starts_with('0'));
        if canonical && after[ndig..].starts_with('>') {
            if let Ok(i) = digits.parse::<usize>() {
                if i < colors.len() {
                    let c = colors[i];
                    out.push_str(&format!("{},{},{}", c[0], c[1], c[2]));
                    rest = &after[ndig + 1..];
                    continue;
                }
            }
        }
        out.push_str("<color_");
        rest = after;
    }
    out.push_str(rest);
    out
}

fn bbox_line(token: &str, b: &Option<[i64; 4]>) -> String {
    let mut s = String::from("{\"bbox\": ");
    match b {
        Some(b) => s.push_str(&format!("[{}, {}, {}, {}]", b[0], b[1], b[2], b[3])),
        None => s.push_str("[]"),
    }
    s.push_str(", \"token\": ");
    py::json_str(token, &mut s);
    s.push_str("}\n");
    s
}

/// `latex2bbox_color((latex, basename, output_path, temp_dir, total_color_list))`.
/// Ok(()) also covers the early returns; Err is an exception escaping the function.
pub fn latex2bbox_color(
    latex: &str,
    basename: &str,
    output_path: &str,
    temp_dir: &str,
    total_color_list: &[[u8; 3]],
    backend: &Backend,
) -> PyResult<()> {
    let basename = basename.replace(".jpg", "");
    let output_bbox_path = path_join(&path_join(output_path, "bbox"), &format!("{basename}.jsonl"));
    let output_vis_path = path_join(&path_join(output_path, "vis"), &format!("{basename}.png"));
    let output_base_path = path_join(&path_join(output_path, "vis"), &format!("{basename}_base.png"));
    if Path::new(&output_bbox_path).exists() && Path::new(&output_vis_path).exists() && Path::new(&output_base_path).exists() {
        return Ok(());
    }

    let (final_latex, token_list, n_colors) = match preprocess(latex, &basename, temp_dir, total_color_list, backend.tokenizer) {
        Ok(x) => x,
        Err(_) => return Ok(()), // logging.info(...); return
    };
    let color_list = &total_color_list[..n_colors];

    let pre_name = format!("{}_{}", output_path.replace(['/', '.'], "_"), basename);
    let tex_filename = path_join(temp_dir, &format!("{pre_name}.tex"));
    let tex_source = final_latex.replace(r"\mathcolor", r"\color");
    let pdf_filename = format!("{}.pdf", &tex_filename[..tex_filename.len() - 4]);
    // the PDF of a Tectonic render stays in memory
    let pdf: Option<Vec<u8>> = match backend.tex {
        Tex::Tectonic => match tex::compile(&tex_source).map_err(PyErr::new)? {
            None => return Ok(()),
            Some(p) => Some(p),
        },
        Tex::Xelatex => {
            fs::write(&tex_filename, &tex_source).map_err(|e| PyErr::new(format!("OSError: {tex_filename}: {e}")))?;
            run_cmd(
                &format!("xelatex -interaction=nonstopmode -output-directory={temp_dir} \"{tex_filename}\" >/dev/null"),
                Some(temp_dir),
            )?;
            if !Path::new(&pdf_filename).exists() {
                return Ok(());
            }
            None
        }
    };
    let opened = match backend.raster {
        Raster::Native => {
            let bytes = match pdf {
                Some(p) => p,
                None => fs::read(&pdf_filename).map_err(|e| PyErr::new(format!("OSError: {pdf_filename}: {e}")))?,
            };
            raster::rasterize(&bytes, backend.coverage_threshold)?
        }
        Raster::Magick => {
            if let Some(p) = &pdf {
                fs::write(&pdf_filename, p).map_err(|e| PyErr::new(format!("OSError: {pdf_filename}: {e}")))?;
            }
            run_cmd(&format!("magick -density 200 -quality 100 {pdf_filename} -alpha set {output_base_path}"), None)?;
            pil::open_png(&output_base_path)?
        }
    };

    write_boxes(&opened, &token_list, color_list, &output_bbox_path, &output_vis_path, &output_base_path)
}

/// The output paths of `latex2bbox_color` for one formula: (bbox jsonl, vis, base).
pub(crate) fn output_paths(output_path: &str, basename: &str) -> (String, String, String) {
    let basename = basename.replace(".jpg", "");
    (
        path_join(&path_join(output_path, "bbox"), &format!("{basename}.jsonl")),
        path_join(&path_join(output_path, "vis"), &format!("{basename}.png")),
        path_join(&path_join(output_path, "vis"), &format!("{basename}_base.png")),
    )
}

/// From the page pixels on: `crop_image`, `extrac_bbox_from_color_image`, the bbox
/// jsonl and the two PNGs.
pub(crate) fn write_boxes(
    opened: &pil::Opened,
    token_list: &[String],
    color_list: &[[u8; 3]],
    output_bbox_path: &str,
    output_vis_path: &str,
    output_base_path: &str,
) -> PyResult<()> {
    // crop_image(output_base_path): its file is re-read unchanged by the next step,
    // so the cropped image is kept in memory.
    let gray = opened.to_l();
    let w = opened.w as i64;
    let (mut x_min, mut y_min, mut x_max, mut y_max) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for (i, &v) in gray.iter().enumerate() {
        if v != 255 {
            let x = i as i64 % w;
            let y = i as i64 / w;
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
    }
    if x_min == i64::MAX {
        (x_min, y_min, x_max, y_max) = (0, 0, 10, 10);
    }
    let pad = 8;
    let img = opened.to_rgb().crop(x_min - pad, y_min - pad, x_max + pad, y_max + pad);

    // extrac_bbox_from_color_image
    let mut index: HashMap<[u8; 3], usize> = HashMap::with_capacity(color_list.len());
    for (i, c) in color_list.iter().enumerate() {
        index.entry(*c).or_insert(i);
    }
    let mut boxes: Vec<Option<[i64; 4]>> = vec![None; color_list.len()];
    let cw = img.w as i64;
    for (i, p) in img.data.chunks_exact(3).enumerate() {
        if let Some(&k) = index.get(&[p[0], p[1], p[2]]) {
            let x = i as i64 % cw;
            let y = i as i64 / cw;
            boxes[k] = Some(match boxes[k] {
                None => [x, y, x, y],
                Some(b) => [b[0].min(x), b[1].min(y), b[2].max(x), b[3].max(y)],
            });
        }
    }
    let bbox_list: Vec<Option<[i64; 4]>> =
        boxes.into_iter().map(|b| b.map(|b| [b[0] - 1, b[1] - 1, b[2] + 1, b[3] + 1])).collect();
    let base = img.binarize();
    base.save_png(output_base_path)?;

    // bbox jsonl + vis (rectangles; the token text labels are not drawn)
    let mut vis = base;
    let mut out = String::new();
    for (token, b) in token_list.iter().zip(bbox_list.iter()) {
        out.push_str(&bbox_line(token, b));
        if let Some(b) = b {
            vis.rectangle_outline(b[0], b[1], b[2], b[3], [0, 250, 0], 1)?;
        }
    }
    fs::write(output_bbox_path, out).map_err(|e| PyErr::new(format!("OSError: {e}")))?;
    vis.save_png(output_vis_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        let c = gen_color_list(5800, 15);
        assert_eq!(c.len(), 5800);
        assert_eq!(c[0], [0, 0, 15]);
        assert_eq!(c[17], [0, 15, 0]);
        assert_eq!(c[323], [15, 0, 0]);
    }

    #[test]
    fn placeholders() {
        let c = gen_color_list(20, 15);
        assert_eq!(replace_color_placeholders("a<color_0>b<color_10><color_07><color_99>", &c), "a0,0,15b0,0,165<color_07><color_99>");
    }
}

/// Test hook: the document `latex2bbox_color` compiles for `latex` (native tokenizer)
/// and the number of token colours in it.
pub(crate) fn tex_document(latex: &str) -> PyResult<(String, usize)> {
    let colors = gen_color_list(5800, 15);
    let (doc, _, n) = preprocess(latex, "x", ".", &colors, Tokenizer::Native)?;
    Ok((doc.replace(r"\mathcolor", r"\color"), n))
}
