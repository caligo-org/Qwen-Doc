//! The Pillow 10.4 operations CDM uses: open a PNG (modes as Pillow maps PNG colour
//! types), `convert("L")` / `convert("RGB")`, `crop` (zero fill outside), save an RGB
//! PNG, `paste`, and `ImageDraw.rectangle(outline=..., width=...)`.

use std::fs::File;
use std::io::BufWriter;

use crate::py::{PyErr, PyResult};

/// Pixel data of an opened PNG, in the Pillow mode it would have.
pub enum Decoded {
    /// "1": values 0 / 255
    One(Vec<u8>),
    L(Vec<u8>),
    /// "I;16"
    I16(Vec<u16>),
    LA(Vec<[u8; 2]>),
    Rgb(Vec<[u8; 3]>),
    Rgba(Vec<[u8; 4]>),
    /// "P": indices and the 256-entry palette (unset entries are a grey ramp, as Pillow)
    P(Vec<u8>, Vec<[u8; 3]>),
}

pub struct Opened {
    pub w: usize,
    pub h: usize,
    pub px: Decoded,
}

#[derive(Clone)]
pub struct RgbImage {
    pub w: usize,
    pub h: usize,
    pub data: Vec<u8>,
}

/// `(r * 19595 + g * 38470 + b * 7471 + 0x8000) >> 16` (Pillow's L24)
#[inline]
pub fn l24(r: u8, g: u8, b: u8) -> u8 {
    ((r as u32 * 19595 + g as u32 * 38470 + b as u32 * 7471 + 0x8000) >> 16) as u8
}

fn unpack_bits(row: &[u8], bits: u8, w: usize) -> Vec<u8> {
    let per = 8 / bits as usize;
    let mask = (1u16 << bits) as u8 - 1;
    (0..w)
        .map(|x| {
            let byte = row[x / per];
            let shift = 8 - bits as usize * (x % per + 1);
            (byte >> shift) & mask
        })
        .collect()
}

pub fn open_png(path: &str) -> PyResult<Opened> {
    let f = File::open(path).map_err(|e| PyErr::new(format!("FileNotFoundError: {path}: {e}")))?;
    let mut dec = png::Decoder::new(std::io::BufReader::new(f));
    dec.set_transformations(png::Transformations::IDENTITY);
    dec.ignore_checksums(false);
    let mut reader = dec.read_info().map_err(|e| PyErr::new(format!("PIL error: {e}")))?;
    let info = reader.info();
    let (w, h) = (info.width as usize, info.height as usize);
    let ct = info.color_type;
    let bd = info.bit_depth as u8;
    let palette: Option<Vec<u8>> = info.palette.as_ref().map(|p| p.to_vec());
    let mut buf = vec![0u8; reader.output_buffer_size().ok_or_else(|| PyErr::new("PIL: image too large"))?];
    let fi = reader.next_frame(&mut buf).map_err(|e| PyErr::new(format!("OSError: {e}")))?;
    let line = fi.line_size;
    let row = |y: usize| &buf[y * line..(y + 1) * line];
    let n = w * h;
    let px = match (ct, bd) {
        (png::ColorType::Grayscale, 1) => {
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                v.extend(unpack_bits(row(y), 1, w).into_iter().map(|b| if b != 0 { 255 } else { 0 }));
            }
            Decoded::One(v)
        }
        (png::ColorType::Grayscale, 2 | 4) => {
            let scale = if bd == 2 { 85 } else { 17 };
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                v.extend(unpack_bits(row(y), bd, w).into_iter().map(|b| b * scale));
            }
            Decoded::L(v)
        }
        (png::ColorType::Grayscale, 8) => {
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                v.extend_from_slice(&row(y)[..w]);
            }
            Decoded::L(v)
        }
        (png::ColorType::Grayscale, 16) => {
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                let r = row(y);
                v.extend((0..w).map(|x| u16::from_be_bytes([r[2 * x], r[2 * x + 1]])));
            }
            Decoded::I16(v)
        }
        (png::ColorType::GrayscaleAlpha, 8 | 16) => {
            let s = if bd == 8 { 1 } else { 2 };
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                let r = row(y);
                v.extend((0..w).map(|x| [r[2 * s * x], r[2 * s * x + s]]));
            }
            Decoded::LA(v)
        }
        (png::ColorType::Rgb, 8 | 16) => {
            let s = if bd == 8 { 1 } else { 2 };
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                let r = row(y);
                v.extend((0..w).map(|x| [r[3 * s * x], r[3 * s * x + s], r[3 * s * x + 2 * s]]));
            }
            Decoded::Rgb(v)
        }
        (png::ColorType::Rgba, 8 | 16) => {
            let s = if bd == 8 { 1 } else { 2 };
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                let r = row(y);
                v.extend(
                    (0..w).map(|x| [r[4 * s * x], r[4 * s * x + s], r[4 * s * x + 2 * s], r[4 * s * x + 3 * s]]),
                );
            }
            Decoded::Rgba(v)
        }
        (png::ColorType::Indexed, 1 | 2 | 4 | 8) => {
            let mut v = Vec::with_capacity(n);
            for y in 0..h {
                if bd == 8 {
                    v.extend_from_slice(&row(y)[..w]);
                } else {
                    v.extend(unpack_bits(row(y), bd, w));
                }
            }
            let mut pal: Vec<[u8; 3]> = (0..256).map(|i| [i as u8; 3]).collect();
            if let Some(p) = palette {
                for (i, c) in p.chunks_exact(3).take(256).enumerate() {
                    pal[i] = [c[0], c[1], c[2]];
                }
            }
            Decoded::P(v, pal)
        }
        _ => return Err(PyErr::new(format!("PIL: unsupported PNG {ct:?}/{bd}"))),
    };
    Ok(Opened { w, h, px })
}

impl Opened {
    /// `convert("L")`
    pub fn to_l(&self) -> Vec<u8> {
        match &self.px {
            Decoded::One(v) | Decoded::L(v) => v.clone(),
            Decoded::I16(v) => v.iter().map(|&x| if x >= 255 { 255 } else { x as u8 }).collect(),
            Decoded::LA(v) => v.iter().map(|p| p[0]).collect(),
            Decoded::Rgb(v) => v.iter().map(|p| l24(p[0], p[1], p[2])).collect(),
            Decoded::Rgba(v) => v.iter().map(|p| l24(p[0], p[1], p[2])).collect(),
            Decoded::P(v, pal) => {
                let lut: Vec<u8> = pal.iter().map(|c| l24(c[0], c[1], c[2])).collect();
                v.iter().map(|&i| lut[i as usize]).collect()
            }
        }
    }

    /// `convert("RGB")`
    pub fn to_rgb(&self) -> RgbImage {
        let mut data = Vec::with_capacity(self.w * self.h * 3);
        match &self.px {
            Decoded::One(v) | Decoded::L(v) => v.iter().for_each(|&x| data.extend_from_slice(&[x, x, x])),
            Decoded::I16(v) => v.iter().for_each(|&x| {
                let x = if x >= 255 { 255 } else { x as u8 };
                data.extend_from_slice(&[x, x, x])
            }),
            Decoded::LA(v) => v.iter().for_each(|p| data.extend_from_slice(&[p[0], p[0], p[0]])),
            Decoded::Rgb(v) => v.iter().for_each(|p| data.extend_from_slice(p)),
            Decoded::Rgba(v) => v.iter().for_each(|p| data.extend_from_slice(&p[..3])),
            Decoded::P(v, pal) => v.iter().for_each(|&i| data.extend_from_slice(&pal[i as usize])),
        }
        RgbImage { w: self.w, h: self.h, data }
    }
}

impl RgbImage {
    pub fn new(w: usize, h: usize, c: [u8; 3]) -> RgbImage {
        let mut data = Vec::with_capacity(w * h * 3);
        for _ in 0..w * h {
            data.extend_from_slice(&c);
        }
        RgbImage { w, h, data }
    }

    /// `Image.crop((x0, y0, x1, y1))`; area outside the source is 0.
    pub fn crop(&self, x0: i64, y0: i64, x1: i64, y1: i64) -> RgbImage {
        let w = (x1 - x0).max(0) as usize;
        let h = (y1 - y0).max(0) as usize;
        let mut out = RgbImage { w, h, data: vec![0u8; w * h * 3] };
        out.paste(self, -x0, -y0);
        out
    }

    /// `Image.paste(im, (x, y))` for same-mode images (clipped).
    pub fn paste(&mut self, src: &RgbImage, x: i64, y: i64) {
        let sx0 = (-x).max(0);
        let sy0 = (-y).max(0);
        let dx0 = x.max(0);
        let dy0 = y.max(0);
        let cw = (src.w as i64 - sx0).min(self.w as i64 - dx0);
        let ch = (src.h as i64 - sy0).min(self.h as i64 - dy0);
        if cw <= 0 || ch <= 0 {
            return;
        }
        let (cw, ch) = (cw as usize, ch as usize);
        for r in 0..ch {
            let s = ((sy0 as usize + r) * src.w + sx0 as usize) * 3;
            let d = ((dy0 as usize + r) * self.w + dx0 as usize) * 3;
            self.data[d..d + cw * 3].copy_from_slice(&src.data[s..s + cw * 3]);
        }
    }

    #[inline]
    fn point(&mut self, x: i64, y: i64, c: [u8; 3]) {
        if x >= 0 && x < self.w as i64 && y >= 0 && y < self.h as i64 {
            let o = (y as usize * self.w + x as usize) * 3;
            self.data[o..o + 3].copy_from_slice(&c);
        }
    }

    fn hline(&mut self, mut x0: i64, y0: i64, mut x1: i64, c: [u8; 3]) {
        if y0 >= 0 && y0 < self.h as i64 {
            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
            }
            if x0 < 0 {
                x0 = 0;
            } else if x0 >= self.w as i64 {
                return;
            }
            if x1 < 0 {
                return;
            } else if x1 >= self.w as i64 {
                x1 = self.w as i64 - 1;
            }
            for x in x0..=x1 {
                self.point(x, y0, c);
            }
        }
    }

    /// Pillow's `line32` (Bresenham); only vertical lines are drawn by `rectangle`.
    fn line(&mut self, mut x0: i64, mut y0: i64, x1: i64, y1: i64, c: [u8; 3]) {
        let mut dx = x1 - x0;
        let xs = if dx < 0 {
            dx = -dx;
            -1
        } else {
            1
        };
        let mut dy = y1 - y0;
        let ys = if dy < 0 {
            dy = -dy;
            -1
        } else {
            1
        };
        let n = dx.max(dy);
        if dx == 0 {
            for _ in 0..dy {
                self.point(x0, y0, c);
                y0 += ys;
            }
        } else if dy == 0 {
            for _ in 0..dx {
                self.point(x0, y0, c);
                x0 += xs;
            }
        } else if dx > dy {
            let mut e = 2 * dy - dx;
            for _ in 0..n {
                self.point(x0, y0, c);
                if e >= 0 {
                    y0 += ys;
                    e -= 2 * dx;
                }
                e += 2 * dy;
                x0 += xs;
            }
        } else {
            let mut e = 2 * dx - dy;
            for _ in 0..n {
                self.point(x0, y0, c);
                if e >= 0 {
                    x0 += xs;
                    e -= 2 * dy;
                }
                e += 2 * dx;
                y0 += ys;
            }
        }
    }

    /// `ImageDraw.rectangle([x0, y0, x1, y1], fill=None, outline=c, width=width)`
    pub fn rectangle_outline(&mut self, x0: i64, mut y0: i64, x1: i64, mut y1: i64, c: [u8; 3], width: i64) -> PyResult<()> {
        if x1 < x0 {
            return Err(PyErr::new("ValueError: x1 must be greater than or equal to x0"));
        }
        if y1 < y0 {
            return Err(PyErr::new("ValueError: y1 must be greater than or equal to y0"));
        }
        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
        }
        let width = if width == 0 { 1 } else { width };
        for i in 0..width {
            self.hline(x0, y0 + i, x1, c);
            self.hline(x0, y1 - i, x1, c);
            self.line(x1 - i, y0 + width, x1 - i, y1 - width + 1, c);
            self.line(x0 + i, y0 + width, x0 + i, y1 - width + 1, c);
        }
        Ok(())
    }

    /// Pixels of `convert("L").point(lambda x: 255 if x == 255 else 0, '1').convert("RGB")`
    pub fn binarize(&self) -> RgbImage {
        let mut data = Vec::with_capacity(self.data.len());
        for p in self.data.chunks_exact(3) {
            let v = if l24(p[0], p[1], p[2]) == 255 { 255 } else { 0 };
            data.extend_from_slice(&[v, v, v]);
        }
        RgbImage { w: self.w, h: self.h, data }
    }

    /// `Image.save(path)` as an 8-bit RGB PNG (same pixels as Pillow; the byte stream differs).
    pub fn save_png(&self, path: &str) -> PyResult<()> {
        let f = File::create(path).map_err(|e| PyErr::new(format!("OSError: {path}: {e}")))?;
        let mut enc = png::Encoder::new(BufWriter::new(f), self.w as u32, self.h as u32);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);
        let mut wr = enc.write_header().map_err(|e| PyErr::new(format!("OSError: {e}")))?;
        wr.write_image_data(&self.data).map_err(|e| PyErr::new(format!("OSError: {e}")))?;
        wr.finish().map_err(|e| PyErr::new(format!("OSError: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Outlines drawn by Pillow 10.4 on small images (incl. clipping and degenerate boxes).
    #[test]
    fn rectangle_matches_pillow() {
        let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!("../tests/data/rect_ref.json")).unwrap();
        for c in cases {
            let a = c.as_array().unwrap();
            let g = |i: usize| a[i].as_i64().unwrap();
            let (w, h) = (g(0) as usize, g(1) as usize);
            let mut im = RgbImage::new(w, h, [0, 0, 0]);
            im.rectangle_outline(g(2), g(3), g(4), g(5), [255, 0, 0], g(6)).unwrap();
            let got: Vec<i64> =
                im.data.chunks_exact(3).enumerate().filter(|(_, p)| p[0] != 0).map(|(i, _)| i as i64).collect();
            let want: Vec<i64> = a[7].as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect();
            assert_eq!(got, want, "{c}");
        }
    }
}
