//! Exact Rust ports of MPDocBench's tree-edit-distance metrics:
//!
//! * [`teds`]: `metrics/table_metric.py::TEDS(structure_only).evaluate(pred, true)`
//!   (HTML parsed with libxml2's HTML parser the way lxml 5.3.0 does it);
//! * [`head_teds`]: `metrics/head_metric.py::HeadTEDS().evaluate(pred, gold)`;
//! * [`tree_edit_distance`]: the exact ordered tree edit distance both use (APTED's result).

mod compat;
mod head;
pub mod html;
pub mod lev;
pub mod pystr;
mod table;
mod ted;

pub use head::{head_teds, markdown_text, HeadId, HeadNode};
pub use table::{debug_parse, teds, DebugNode};
pub use ted::{tree_edit_distance, unit_tree_edit_distance};

/// A condition under which the Python code raises; callers score it as 0.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// lxml raises while parsing (empty document, `str` with an XML encoding declaration).
    Parse(&'static str),
    /// `int()` of a `colspan`/`rowspan` attribute that is not an integer literal.
    InvalidInt(&'static str),
    /// `ZeroDivisionError`
    ZeroDivision,
    /// `KeyError`: a relation names a node that is not in the heading nodes.
    MissingNode,
    /// `TypeError`: a heading node reached from the root has no string text.
    NonStringText,
    /// `ValueError` unpacking `node.split(' ', 1)` for a heading like `#\tTitle`.
    HeadingSplit,
    /// `RecursionError`: the annotated heading relations have a cycle reachable from the root.
    Cycle,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
