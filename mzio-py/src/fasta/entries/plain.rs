// 3rd party imports
use pyo3::prelude::*;
use mzio::fasta::entry::Entry as BaseEntry;
use mzio::fasta::headers::{
    Header,
    plain::Plain as BasePlain
};

/// Wrapper for the rust implementation entry
/// 
#[pyclass]
pub struct Plain {
    base_entry: BaseEntry<BasePlain>
}

#[pymethods]
impl Plain {
    /// Python constructor
    /// 
    /// # Arguments
    /// 
    #[new]
    fn new(header: String, sequence: String) -> Self {
        Self {
            base_entry: BaseEntry::new(
                BasePlain::new(&header),
                sequence
            )
        }
    }

    /// Returns the header
    ///
    #[getter]
    pub fn header(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_header().get_header())
    }

    /// Returns the amino acid sequence
    ///
    #[getter]
    pub fn sequence(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_sequence())
    }
}


impl From<BaseEntry<BasePlain>> for Plain {
    /// Convert entry from the Rust implementation to the python wrapper.
    /// 
    /// # Arguments
    /// 
    /// * `base_entry` - Plain from rust implementation
    fn from(base_entry: BaseEntry<BasePlain>) -> Self {
        Self {
            base_entry
        }
    }
}

impl<'a> Into<&'a BaseEntry<BasePlain>> for &'a Plain {
    fn into(self) -> &'a BaseEntry<BasePlain> {
        &self.base_entry
    }
}
