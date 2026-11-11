//! Formula CDM: wired to the Rust `cdm` crate (in progress).

use crate::pyval::V;

pub struct CdmConfig {
    pub workers: usize,
    pub marathon: bool,
}

pub fn cdm_metric(_cfg: &CdmConfig, _samples: &[V], _prefix: &str, _output_root: &str) -> (Vec<V>, V) {
    panic!("CDM is not wired in yet");
}
