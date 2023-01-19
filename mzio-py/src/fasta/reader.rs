// std imports
use std::path::PathBuf;

// 3rd party modules
use pyo3::prelude::*;
use anyhow::Result;
use rusteomics_proteomics_io::fasta::reader::Reader as BaseReader;

// internal imports
use crate::fasta::entry::Entry;

#[pyclass]
pub struct Reader {
    base_reader: BaseReader
}

#[pymethods]
impl Reader {
    #[new]
    fn new(fasta_file_path: PathBuf, buffer_size: usize) -> Result<Self> {
        match BaseReader::new(&fasta_file_path, buffer_size) {
            Ok(base_reader) => Ok(Self{base_reader}),
            Err(err) => Err(err)
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Entry> {
        match slf.base_reader.next() {
            Some(base_entry) => Some(Entry::from(base_entry)),
            None => None
        }
    }
}
