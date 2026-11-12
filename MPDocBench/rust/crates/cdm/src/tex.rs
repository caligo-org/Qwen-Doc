//! LaTeX -> PDF with Tectonic's XeTeX engine linked in as a library (replaces
//! `xelatex -interaction=nonstopmode`).
//!
//! Tectonic reads its TeX Live support files from a bundle that it downloads on demand
//! and caches on disk; after the files a render needs are cached, renders work offline.
//! Cache location: `CDM_TECTONIC_CACHE` (bundle files under `bundles/`, generated format
//! files under `formats/`), else Tectonic's own default (`TECTONIC_CACHE_DIR`, else the
//! per-user cache directory). `CDM_TECTONIC_BUNDLE` overrides the bundle URL or path;
//! `CDM_TECTONIC_ONLY_CACHED=1` never touches the network.
//!
//! Like `xelatex -interaction=nonstopmode`, TeX errors do not stop the run (Tectonic's
//! `continue_on_errors`) and TeX runs once (no reruns). Tectonic's XeTeX engine keeps
//! global C state: the library serialises engine runs inside one process with a lock,
//! so concurrent renders queue.

use std::path::PathBuf;
use std::time::SystemTime;

use tectonic::driver::{OutputFormat, PassSetting, ProcessingSessionBuilder};
use tectonic::status::NoopStatusBackend;
use tectonic::unstable_opts::UnstableOptions;

fn cache_root() -> Option<PathBuf> {
    match std::env::var_os("CDM_TECTONIC_CACHE") {
        Some(p) if !p.is_empty() => Some(PathBuf::from(p)),
        _ => None,
    }
}

/// The bundle URL (or local path) renders use.
pub fn bundle_location() -> String {
    match std::env::var("CDM_TECTONIC_BUNDLE") {
        Ok(s) if !s.is_empty() => s,
        _ => tectonic_bundles::get_fallback_bundle_url(tectonic::FORMAT_SERIAL),
    }
}

/// Compiles `tex` (the content of `<name>.tex`) and returns the PDF bytes, or None where
/// xelatex would leave no PDF behind. Err only for setup failures (bundle unavailable).
pub fn compile(tex: &str) -> Result<Option<Vec<u8>>, String> {
    compile_with_log(tex, false).map(|(pdf, _)| pdf)
}

/// The PDF (None: no PDF) and the TeX log (when requested).
pub type PdfAndLog = (Option<Vec<u8>>, Option<Vec<u8>>);

/// [`compile`], also returning the TeX log when `log` is set.
pub fn compile_with_log(tex: &str, log: bool) -> Result<PdfAndLog, String> {
    let root = cache_root();
    let only_cached = std::env::var("CDM_TECTONIC_ONLY_CACHED").is_ok_and(|v| v == "1");
    let bundle = tectonic_bundles::detect_bundle(bundle_location(), only_cached, root.as_ref().map(|r| r.join("bundles")))
        .map_err(|e| format!("tectonic bundle: {e:#}"))?
        .ok_or_else(|| "tectonic bundle: unrecognised location".to_string())?;
    let formats = match &root {
        Some(r) => r.join("formats"),
        None => tectonic::config::PersistentConfig::default().format_cache_path().map_err(|e| format!("tectonic cache: {e:#}"))?,
    };
    let mut status = NoopStatusBackend::default();
    let mut sb = ProcessingSessionBuilder::default();
    let un = UnstableOptions { continue_on_errors: true, ..Default::default() };
    sb.bundle(bundle)
        .primary_input_buffer(tex.as_bytes())
        .tex_input_name("texput.tex")
        .format_name("latex")
        .format_cache_path(formats)
        .keep_logs(log)
        .keep_intermediates(false)
        .print_stdout(false)
        .output_format(OutputFormat::Pdf)
        .pass(PassSetting::Default)
        .reruns(0)
        .unstables(un)
        .build_date(SystemTime::UNIX_EPOCH)
        .do_not_write_output_files();
    let mut sess = sb.create(&mut status).map_err(|e| format!("tectonic session: {e:#}"))?;
    // An Err here is a fatal TeX error (or an engine failure): use whatever PDF the run
    // left behind, as the Python code checks only for the file.
    let _ = sess.run(&mut status);
    let files = sess.into_file_data();
    let pdf = files.get("texput.pdf").map(|f| f.data.clone()).filter(|d| !d.is_empty());
    Ok((pdf, files.get("texput.log").map(|f| f.data.clone())))
}
