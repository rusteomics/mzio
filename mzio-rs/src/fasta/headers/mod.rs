/// Contains headers for parsing different FASTA headers of different formats.

// std imports
use std::fmt;

// publish modules
pub mod plain;
pub mod uniprot;

/// Trait for defining a header.
/// 
/// Display trait should rebuild the header in it's original form.
/// 
pub trait Header: fmt::Display {
    /// Create header from FASTA header
    /// 
    /// # Arguments
    /// * `header` - FASTA header
    fn new(header: &str) -> Self;
}
