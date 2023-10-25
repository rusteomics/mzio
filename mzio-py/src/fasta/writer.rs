// std imports
use std::path::PathBuf;

// 3rd party modules
use anyhow::Result;
use pyo3::prelude::*;
use mzio::fasta::writer::Writer as BaseWriter;

// internal imports
use crate::fasta::entry::Entry;


#[pyclass]
pub struct Writer {
    base_writer: BaseWriter
}

#[pymethods]
impl Writer {
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// 
    #[new]
    pub fn new(fasta_file_path: PathBuf, sort_keyword_attributes: bool) -> PyResult<Self> {
        match BaseWriter::new_with_default_seq_formatting(&fasta_file_path, sort_keyword_attributes) {
            Ok(base_writer) => Ok(Self{base_writer}),
            Err(err) => Err(err.into())
        }
    }

    pub fn write_entry(&mut self, entry: &Entry) -> Result<usize> {
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
