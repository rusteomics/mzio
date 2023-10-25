// std imports
use std::path::PathBuf;

// 3rd party modules
use anyhow::Result;
use pyo3::prelude::*;
use mzio::mgf::writer::MgfWriter as BaseMgfWriter;

// internal imports
use crate::mgf::spectrum::MgfSpectrum;


#[pyclass]
pub struct MgfWriter {
    base_writer: BaseMgfWriter
}

#[pymethods]
impl MgfWriter {
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `mgf_file_path` - Path to MGF file
    /// 
    #[new]
    pub fn new(mgf_file_path: PathBuf) -> Result<Self> {
        let base_writer = BaseMgfWriter::new(&mgf_file_path)?;
        Ok(Self {base_writer})
    }

    pub fn write_spectrum(&mut self, spectrum: &MgfSpectrum) -> Result<usize> {
        self.base_writer.write_spectrum(spectrum.into())
        /*match self.base_writer.write_spectrum(spectrum.into()) {
            Ok(written_bytes) => Ok(written_bytes),
            Err(err) => Err(err)
        }*/
    }


    pub fn flush(&mut self) -> Result<()> {
        self.base_writer.flush()
        /*match self.base_writer.flush() {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }*/
    }
}
