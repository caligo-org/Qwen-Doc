//! Compares the native rasteriser with Ghostscript's `pngalpha` device (what `magick
//! -density 200 ... -alpha set` runs) on the token boxes CDM extracts:
//!
//!     raster_calib PAIRS.json WORKDIR [THRESHOLDS, e.g. 4,8,12,16]
//!
//! Each gt / pred formula of PAIRS is compiled once with Tectonic; the PDF is rasterised
//! with `gs` (on PATH) and natively at each coverage threshold. Reported per threshold:
//! the share of token colours whose box (relative to the crop origin, as CDM uses it)
//! equals Ghostscript's, and the mean absolute box-coordinate difference.

use std::collections::HashMap;
use std::process::Command;

fn boxes(w: usize, px: &[[u8; 4]], colors: &HashMap<[u8; 3], usize>) -> (HashMap<usize, [i64; 4]>, (i64, i64)) {
    let (mut x0, mut y0) = (i64::MAX, i64::MAX);
    let mut m: HashMap<usize, [i64; 4]> = HashMap::new();
    for (i, p) in px.iter().enumerate() {
        let c = [p[0], p[1], p[2]];
        if c == [255, 255, 255] {
            continue;
        }
        let (x, y) = ((i % w) as i64, (i / w) as i64);
        x0 = x0.min(x);
        y0 = y0.min(y);
        if let Some(&k) = colors.get(&c) {
            let e = m.entry(k).or_insert([x, y, x, y]);
            e[0] = e[0].min(x);
            e[1] = e[1].min(y);
            e[2] = e[2].max(x);
            e[3] = e[3].max(y);
        }
    }
    (m, (x0, y0))
}

fn read_png(p: &str) -> (usize, Vec<[u8; 4]>) {
    let mut dec = png::Decoder::new(std::io::BufReader::new(std::fs::File::open(p).unwrap()));
    dec.set_transformations(png::Transformations::IDENTITY);
    let mut r = dec.read_info().unwrap();
    let mut buf = vec![0; r.output_buffer_size().unwrap()];
    let fi = r.next_frame(&mut buf).unwrap();
    assert_eq!(fi.color_type, png::ColorType::Rgba);
    (fi.width as usize, buf.chunks(4).map(|c| [c[0], c[1], c[2], c[3]]).collect())
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let pairs: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let work = std::path::PathBuf::from(&a[2]);
    std::fs::create_dir_all(&work).unwrap();
    let thrs: Vec<u8> = a.get(3).map(|s| s.split(',').map(|t| t.parse().unwrap()).collect()).unwrap_or(vec![4, 8, 12, 16, 24]);
    let palette: HashMap<[u8; 3], usize> = cdm::__colors().into_iter().enumerate().map(|(i, c)| (c, i)).collect();
    // per threshold: (tokens, equal, sum abs diff, missing/extra)
    let mut stats: Vec<(usize, usize, i64, usize)> = vec![(0, 0, 0, 0); thrs.len()];
    let mut signed: Vec<[i64; 4]> = vec![[0; 4]; thrs.len()];
    let mut docs = 0;
    for (pi, p) in pairs.iter().enumerate() {
        let (gt, pred) = cdm::clean(p["gt"].as_str().unwrap(), p["pred"].as_str().unwrap());
        for (k, latex) in [gt, pred].iter().enumerate() {
            if latex.trim().is_empty() {
                continue;
            }
            let Ok((doc, n)) = cdm::__tex_document(latex) else { continue };
            let Ok(Some(pdf)) = cdm::__compile(&doc) else { continue };
            let colors: HashMap<[u8; 3], usize> = palette.iter().filter(|(_, &i)| i < n).map(|(c, i)| (*c, *i)).collect();
            let pdf_path = work.join(format!("{pi}_{k}.pdf"));
            let png_path = work.join(format!("{pi}_{k}.png"));
            std::fs::write(&pdf_path, &pdf).unwrap();
            let st = Command::new("gs")
                .args(["-sstdout=%stderr", "-dQUIET", "-dSAFER", "-dBATCH", "-dNOPAUSE", "-dNOPROMPT", "-dMaxBitmap=500000000"])
                .args(["-dAlignToPixels=0", "-dGridFitTT=2", "-sDEVICE=pngalpha", "-dTextAlphaBits=4", "-dGraphicsAlphaBits=4", "-r200"])
                .arg(format!("-sOutputFile={}", png_path.display()))
                .arg(&pdf_path)
                .status()
                .unwrap();
            if !st.success() || !png_path.exists() {
                continue;
            }
            docs += 1;
            let (gw, gpx) = read_png(png_path.to_str().unwrap());
            let (gb, go) = boxes(gw, &gpx, &colors);
            for (ti, &t) in thrs.iter().enumerate() {
                let (w, _h, px) = cdm::__rasterize(&pdf, t).unwrap();
                let (hb, ho) = boxes(w, &px, &colors);
                let s = &mut stats[ti];
                for (c, b) in &gb {
                    s.0 += 1;
                    match hb.get(c) {
                        Some(h) => {
                            let rel = |b: &[i64; 4], o: (i64, i64)| [b[0] - o.0, b[1] - o.1, b[2] - o.0, b[3] - o.1];
                            let (x, y) = if std::env::var("CDM_CALIB_ABS").is_ok() { (*b, *h) } else { (rel(b, go), rel(h, ho)) };
                            if x == y {
                                s.1 += 1;
                            }
                            s.2 += (0..4).map(|j| (x[j] - y[j]).abs()).sum::<i64>();
                            for j in 0..4 {
                                signed[ti][j] += y[j] - x[j];
                            }
                        }
                        None => s.3 += 1,
                    }
                }
                s.3 += hb.keys().filter(|c| !gb.contains_key(c)).count();
            }
        }
    }
    println!("documents {docs}");
    for (ti, t) in thrs.iter().enumerate() {
        let s = stats[ti];
        println!(
            "threshold {t:3}: tokens {}, box equal {:.4}, mean |coord diff| {:.4}, tokens present in one only {}",
            s.0,
            s.1 as f64 / s.0 as f64,
            s.2 as f64 / (4 * s.0) as f64,
            s.3
        );
        println!("    mean signed native-gs [x0 y0 x1 y1]: {:?}", signed[ti].iter().map(|v| format!("{:.3}", *v as f64 / s.0 as f64)).collect::<Vec<_>>());
    }
}
