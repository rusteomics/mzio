// std imports
use std::path::PathBuf;

// 3rd party imports
use pyo3::prelude::*;
use anyhow::Result;
use fallible_iterator::FallibleIterator;
use mzio::mgf::reader::MgfReader as BaseMgfReader;

// internal imports
use crate::mgf::spectrum::MgfSpectrum;

#[pyclass]
pub struct MgfReader {
    base_reader: BaseMgfReader
}

#[pymethods]
impl MgfReader {
    #[new]
    #[pyo3(signature = (mgf_file_path, buffer_size=4096))]
    fn new(mgf_file_path: PathBuf, buffer_size: usize) -> Result<Self> {
        let base_reader = BaseMgfReader::new(&mgf_file_path, buffer_size)?;
        Ok(Self{base_reader})
        /*match BaseReader::new(&mgf_file_path, buffer_size) {
            Ok(base_reader) => Ok(Self{base_reader}),
            Err(err) => Err(err)
        }*/
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<MgfSpectrum> {
        match slf.base_reader.next().ok()? {
            Some(base_spectrum) => Some(MgfSpectrum::from(base_spectrum)),
            None => None
        }
    }
}
