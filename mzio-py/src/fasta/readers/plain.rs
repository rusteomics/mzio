// std imports
use std::path::PathBuf;

// 3rd party modules
use pyo3::prelude::*;
use anyhow::{Result};
use mzio::fasta::headers::plain::Plain as BasePlain;
use mzio::fasta::reader::Reader as BaseReader;

// internal imports
use crate::fasta::entries::plain::Plain;


/// A FASTA reader which returns the header in plain text.
/// 
#[pyclass]
pub struct PlainReader  {
    base_reader: BaseReader<BasePlain>
}

#[pymethods]
impl PlainReader {
    #[new]
    fn new(fasta_file_path: PathBuf, buffer_size: usize) -> Result<Self> {
        Ok(Self { 
            base_reader: BaseReader::<BasePlain>::new(&fasta_file_path, buffer_size)? 
        })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Plain> {
        match slf.base_reader.next() {
            Some(base_entry) => Some(Plain::from(base_entry)),
            None => None
        }
    }
}
