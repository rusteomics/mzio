// std imports
use std::fmt::Display;

// internal imports
use crate::fasta::headers::Header;

/// Keeps all information of a FASTA entry
/// 
pub struct Entry<T> where T: Header {
    header: T,
    sequence: String
}

impl<T> Entry<T> where T: Header + Display {
    /// Creates a new FASTA entry
    /// # Arguments
    ///
    /// * `header` - Header
    /// * `sequence` - Amino acid sequence
    /// 
    pub fn new(header: T, sequence: String) -> Self {
        Self {
            header,
            sequence
        }
    }

    /// Returns the amino acid sequence of the FASTA entry
    /// 
    pub fn get_sequence(&self) -> &str {
        &self.sequence
    }

    /// Returns the header of the FASTA entry
    /// 
    pub fn get_header(&self) -> &T {
        &self.header
    }
}