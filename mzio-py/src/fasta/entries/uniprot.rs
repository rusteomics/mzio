// std imports
use std::collections::HashMap;

// 3rd party imports
use pyo3::prelude::*;
use mzio::fasta::entry::Entry as BaseEntry;
use mzio::fasta::headers::{
    Header,
    uniprot::UniProt as BaseUniProt
};


/// Wrapper for the rust implementation entry
/// 
#[pyclass]
pub struct UniProt {
    base_entry: BaseEntry<BaseUniProt>
}

#[pymethods]
impl UniProt {
    /// Python constructor
    /// 
    /// # Arguments
    /// 
    #[new]
    fn new(header: String, sequence: String) -> Self {
        Self {
            base_entry: BaseEntry::new(
                BaseUniProt::new(&header),
                sequence
            )
        }
    }

    /// Returns the database type
    ///
    #[getter]
    pub fn database(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_header().get_database())
    }

    /// Returns the accession
    ///
    #[getter]
    pub fn accession(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_header().get_accession())
    }

    /// UniProt name
    ///
    #[getter]
    pub fn entry_name(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_header().get_entry_name())
    }

    /// Returns the protein name
    ///
    #[getter]
    pub fn protein_name(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_header().get_protein_name())
    }

    /// Returns additional keyword attributes, e.g
    /// * OX = 381666
    /// * GN = acoX
    ///
    // !!! TODO Reference to HasMap is no convertible to PyResult by default.
    #[getter]
    pub fn keyword_attributes(&self) -> PyResult<HashMap<String, String>> {
        // TODO: avoid clone?
        Ok(self.base_entry.get_header().get_keyword_attributes().clone())
    }

    /// Returns the amino acid sequence
    ///
    #[getter]
    pub fn sequence(&self) -> PyResult<&str> {
        Ok(&self.base_entry.get_sequence())
    }
}


impl From<BaseEntry<BaseUniProt>> for UniProt {
    /// Convert entry from the Rust implementation to the python wrapper.
    /// 
    /// # Arguments
    /// 
    /// * `base_entry` - UniProt from rust implementation
    fn from(base_entry: BaseEntry<BaseUniProt>) -> Self {
        Self {
            base_entry
        }
    }
}

impl<'a> Into<&'a BaseEntry<BaseUniProt>> for &'a UniProt {
    fn into(self) -> &'a BaseEntry<BaseUniProt> {
        &self.base_entry
    }
}
