// std imports
use std::path::PathBuf;

// 3rd party modules
use anyhow::Result;
use pyo3::prelude::*;
use mzio::fasta::headers::uniprot::UniProt as BaseUniProt;
use mzio::fasta::writer::Writer as BaseWriter;

// internal imports
use crate::fasta::entries::uniprot::UniProt;


/// A FASTA writer which writes the header in UniProt format.
/// 
#[pyclass]
pub struct UniProtWriter {
    base_writer: BaseWriter<BaseUniProt>
}

#[pymethods]
impl UniProtWriter {
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// 
    #[new]
    pub fn new(fasta_file_path: PathBuf) -> PyResult<Self> {
        Ok(Self { base_writer: BaseWriter::<BaseUniProt>::new(&fasta_file_path)? })
    }

    pub fn write_entry(&mut self, entry: &UniProt) -> Result<usize> {
        match self.base_writer.write_entry(entry.into()) {
            Ok(written_bytes) => Ok(written_bytes),
            Err(err) => Err(err)
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        match self.base_writer.flush() {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}