// std imports
use std::path::PathBuf;

// 3rd party modules
use pyo3::prelude::*;
use anyhow::Result;
use rusteomics_proteomics_io::mgf::reader::Reader as BaseReader;

use fallible_iterator::FallibleIterator;

// internal imports
use crate::mgf::spectrum::Spectrum;

#[pyclass]
pub struct Reader {
    base_reader: BaseReader
}

#[pymethods]
impl Reader {
    #[new]
    #[args(buffer_size=4096)]
    fn new(mgf_file_path: PathBuf, buffer_size: usize) -> Result<Self> {
        match BaseReader::new(&mgf_file_path, buffer_size) {
            Ok(base_reader) => Ok(Self{base_reader}),
            Err(err) => Err(err)
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Spectrum> {
        match slf.base_reader.next().ok()? {
            Some(base_spectrum) => Some(Spectrum::from(base_spectrum)),
            None => None
        }
    }
}
