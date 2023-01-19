// std imports
use std::path::PathBuf;

// 3rd party modules
use anyhow::Result;
use pyo3::prelude::*;
use rusteomics_proteomics_io::mgf::writer::Writer as BaseWriter;

// internal imports
use crate::mgf::spectrum::Spectrum;


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
    /// * `mgf_file_path` - Path to MGF file
    /// 
    #[new]
    pub fn new(mgf_file_path: PathBuf) -> PyResult<Self> {
        match BaseWriter::new(&mgf_file_path) {
            Ok(base_writer) => Ok(Self{base_writer}),
            Err(err) => Err(err.into())
        }
    }

    pub fn write_spectrum(&mut self, spectrum: &Spectrum) -> Result<usize> {
        match self.base_writer.write_spectrum(spectrum.into()) {
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
