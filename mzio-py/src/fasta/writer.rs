// std imports
use std::path::PathBuf;

// 3rd party modules
use pyo3::prelude::*;
use rusteomics_proteomics_io::fasta::writer::Writer as BaseWriter;

// internal imports
use crate::fasta::error::Error;
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
    pub fn new(fasta_file_path: PathBuf) -> PyResult<Self> {
        match BaseWriter::new(&fasta_file_path) {
            Ok(base_writer) => Ok(Self{base_writer}),
            Err(err) => Err(Error::from(err).into())
        }
    }

    pub fn write_entry(&mut self, entry: &Entry, sort_keyword_attributes: bool) -> Result<usize, PyErr> {
        match self.base_writer.write_entry(entry.into(), sort_keyword_attributes) {
            Ok(written_bytes) => Ok(written_bytes),
            Err(err) => Err(Error::from(err).into())
        }
    }


    pub fn flush(&mut self) -> Result<(), Error> {
        match self.base_writer.flush() {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::from(err).into())
        }
    }
}