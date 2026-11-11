//! Bit-exact Rust ports of the HTML table helpers of the MPDocBench scorer:
//!
//! * [`normalized_html_table`] — `utils/data_preprocess.py::normalized_html_table`
//! * [`normalized_html_table_relation`] — the variant in `utils/merge_tables.py`
//!   (used by `utils/relation_utils.py::_norm_table`)
//! * [`merge_tables`] — `utils/merge_tables.py::merge_tables`
//! * [`td_strings`] — the unmatched-cell extraction of `utils/match.py::match_gt2pred_simple`
//! * [`html_unescape`] — Python's `html.unescape`
//!
//! They sit on faithful ports of CPython 3.11.18 `html.parser` ([`parser`]) and
//! the parts of BeautifulSoup 4.11.1 reached by that code ([`dom`]).

pub mod dom;
mod generated;
pub mod parser;
pub mod pystr;
mod tables;
mod unescape;

pub use tables::{
    merge_tables, normalized_html_table, set_python_caller_depths, normalized_html_table_relation, td_strings, try_normalized_html_table,
    try_normalized_html_table_relation, try_td_strings,
};
pub use unescape::html_unescape;

/// A Python exception raised by the original code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// `"ExcType: message"` of the exception Python would raise.
    Python(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Python(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for Error {}

/// `str(BeautifulSoup(html, 'html.parser'))` (exposed for differential tests).
pub fn soup_str(html: &str) -> Result<String, Error> {
    let (mut soup, root) = dom::Soup::parse(html);
    soup.base_frames = 3; // as called by gen/dump_cases.py
    soup.decode(root)
}
