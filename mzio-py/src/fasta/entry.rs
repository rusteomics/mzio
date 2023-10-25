// std imports
use std::collections::HashMap;

// 3rd party imports
use anyhow::Result;
use pyo3::prelude::*;
use mzio::fasta::entry::Entry as BaseEntry;

/// Wrapper for the rust implementation entry
/// 
#[pyclass]
pub struct Entry {
    base_entry: BaseEntry
}

#[pymethods]
impl Entry {
    /// Python constructor
    /// 
    /// # Arguments
    /// 
    /// * `database` - The FASTA database
    /// * `accession` - Entry accession
    /// * `entry_name` - Entry name
    /// * `protein_name` - Protein name
    /// * `keyword_attributes` - Additional keyword attributes, e.g. OX=381666
    /// * `sequence` - Amino acid sequence
    /// 
    #[new]
    fn new(database: String, accession: String, entry_name: String, protein_name: String,
        keyword_attributes: HashMap<String, String>, sequence: String) -> Self {
        Self {
            base_entry: BaseEntry::new(
                database,
                accession,
                entry_name,
                protein_name,
                keyword_attributes,
                sequence,
                None
            )
        }
    }

    /// Returns the database type
    ///
    #[getter]
    pub fn database(&self) -> Result<&String> {
        Ok(self.base_entry.get_database())
    }

    /// Returns the accession
    ///
    #[getter]
    pub fn accession(&self) -> Result<&String> {
        Ok(self.base_entry.get_accession())
    }

    /// Entry name
    ///
    #[getter]
    pub fn entry_name(&self) -> Result<&String> {
        Ok(self.base_entry.get_entry_name())
    }

    /// Returns the protein name
    ///
    #[getter]
    pub fn protein_name(&self) -> Result<&String> {
        Ok(self.base_entry.get_protein_name())
    }

    /// Returns additional keyword attributes, e.g
    /// * OX = 381666
    /// * GN = acoX
    ///
    // !!! TODO Reference to HasMap is no convertible to PyResult by default.
    #[getter]
    pub fn keyword_attributes(&self) -> Result<HashMap<String, String>> {
        // TODO: avoid clone?
        Ok(self.base_entry.get_keyword_attributes().clone())
    }

    /// Returns the amino acid sequence
    ///
    #[getter]
    pub fn sequence(&self) -> Result<&String> {
        Ok(self.base_entry.get_sequence())
    }

    /// Returns the plain header (before parsing)
    ///
    #[getter]
    pub fn plain_header(&self) -> Result<&String> {
        Ok(self.base_entry.get_sequence())
    }
}


impl From<BaseEntry> for Entry {
    /// Convert entry from the Rust implementation to the python wrapper.
    /// 
    /// # Arguments
    /// 
    /// * `base_entry` - Entry from rust implementation
    fn from(base_entry: BaseEntry) -> Self {
        Self {
            base_entry
        }
    }
}

impl<'a> Into<&'a BaseEntry> for &'a Entry {
    fn into(self) -> &'a BaseEntry {
        &self.base_entry
    }
}
