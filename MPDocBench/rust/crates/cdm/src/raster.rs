//! PDF -> RGBA pixels at 200 dpi in Rust (hayro), in place of
//! `magick -density 200 -quality 100 X.pdf -alpha set X.png` (Ghostscript's `pngalpha`
//! device with 4 text / graphics alpha bits).
//!
//! What the CDM code reads from that PNG is the RGB of each pixel (Pillow drops alpha):
//! `pngalpha` leaves unpainted pixels (255,255,255,0) and gives every pixel an object
//! touches the object's own colour (coverage only goes to alpha). Anti-aliased hayro
//! output is premultiplied, which loses the exact colour at low coverage, so pixels are
//! rasterised without anti-aliasing instead: a pixel takes the colour of the last
//! object covering more than `COVERAGE_THRESHOLD`/255 of it, which reproduces
//! "any visible coverage" of the 4-bit alpha device. Against Ghostscript 10.08 on 1126
//! token boxes of 38 Tectonic renders the box corners differ by 0.27 px on average
//! (glyph outline rasterisation differs; the threshold, hinting and sub-pixel offsets
//! were tried and change little), which moves a minority of CDM scores slightly.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::kurbo::Affine;
use hayro::vello_cpu::color::palette::css::TRANSPARENT;
use hayro::vello_cpu::{Pixmap, RasterizerSettings, RenderContext, Resources, TargetInit};
use hayro::{RenderCache, RenderSettings};

use crate::pil::{Decoded, Opened};
use crate::py::{PyErr, PyResult};

/// Minimum coverage (of 255) for a pixel to take an object's colour.
pub const COVERAGE_THRESHOLD: u8 = 12;

const DPI: f64 = 200.0;

/// Rasterises the single page of `pdf`. Err where the `magick` call would leave no
/// `X.png` (an unreadable PDF, or several pages, which magick writes as `X-0.png`, ...).
pub fn rasterize(pdf: &[u8], threshold: u8) -> PyResult<Opened> {
    let pdf = parse(pdf)?;
    let pages = pdf.pages();
    if pages.len() != 1 {
        return Err(PyErr::new(format!("FileNotFoundError: PDF has {} pages", pages.len())));
    }
    rasterize_page(&pages[0], threshold)
}

/// Parses a PDF for [`rasterize_page`].
pub fn parse(pdf: &[u8]) -> PyResult<Pdf> {
    Pdf::new(pdf.to_vec()).map_err(|e| PyErr::new(format!("FileNotFoundError: PDF not rasterised: {e:?}")))
}

/// Rasterises one page (as [`rasterize`] does the single page of a PDF).
pub fn rasterize_page(page: &hayro::hayro_syntax::page::Page<'_>, threshold: u8) -> PyResult<Opened> {
    let mb = page.media_box();
    let s = DPI / 72.0;
    let w = ((mb.x1 - mb.x0) * s + 0.5).floor();
    let h = ((mb.y1 - mb.y0) * s + 0.5).floor();
    if !(1.0..=u16::MAX as f64).contains(&w) || !(1.0..=u16::MAX as f64).contains(&h) {
        return Err(PyErr::new("FileNotFoundError: page size out of range"));
    }
    let (w, h) = (w as u16, h as u16);
    // PDF user space -> device pixels, y down, origin at the top left of the media box
    let tr = Affine::new([s, 0.0, 0.0, -s, -mb.x0 * s, h as f64 + mb.y0 * s]);
    let mut ctx = RenderContext::new(w, h);
    let cache = RenderCache::new();
    let rs = RenderSettings { hinting: false, aliasing_threshold: Some(threshold), ..Default::default() };
    hayro::render_into(page, &cache, &InterpreterSettings::default(), &rs, &mut ctx, tr);
    ctx.flush();
    let mut pm = Pixmap::new(w, h);
    ctx.render_with(
        &mut pm,
        &mut Resources::default(),
        RasterizerSettings { target_init: TargetInit::Clear(TRANSPARENT), ..Default::default() },
    );
    let px = pm
        .data()
        .iter()
        .map(|p| {
            if p.a == 0 {
                [255, 255, 255, 0]
            } else if p.a == 255 {
                [p.r, p.g, p.b, 255]
            } else {
                // not expected without anti-aliasing; un-premultiply
                let un = |c: u8| ((c as u32 * 255 + p.a as u32 / 2) / p.a as u32).min(255) as u8;
                [un(p.r), un(p.g), un(p.b), p.a]
            }
        })
        .collect();
    Ok(Opened { w: w as usize, h: h as usize, px: Decoded::Rgba(px) })
}
