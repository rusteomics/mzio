// std imports
use std::path::PathBuf;

// 3rd party modules
use pyo3::prelude::*;
use anyhow::{Result};
use mzio::fasta::headers::uniprot::UniProt as BaseUniProt;
use mzio::fasta::reader::Reader as BaseReader;

// internal imports
use crate::fasta::entries::uniprot::UniProt;


/// A FASTA reader which parses the UniProt formatted header format.
/// 
#[pyclass]
pub struct UniProtReader  {
    base_reader: BaseReader<BaseUniProt>
}

#[pymethods]
impl UniProtReader {
    #[new]
    fn new(fasta_file_path: PathBuf, buffer_size: usize) -> Result<Self> {
        Ok(Self { 
            base_reader: BaseReader::<BaseUniProt>::new(&fasta_file_path, buffer_size)? 
        })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<UniProt> {
        match slf.base_reader.next() {
            Some(base_entry) => Some(UniProt::from(base_entry)),
            None => None
        }
    }
}
