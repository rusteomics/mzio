// std imports
use std::ops::{Deref, DerefMut};

// 3rd party imports
use pyo3::PyErr;
use pyo3::exceptions::PyIOError;
use rusteomics_proteomics_io::fasta::error::Error as FastaError;

/// Wrapper for Error from the rusteomics-proteomics-io-rs fasta-module
/// to implement an automatic type cast to PyErr
pub struct Error (
    FastaError
);

impl Deref for Error {
    type Target = FastaError;

    /// Implements derefferencing the error from the wrapper.
    ///
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for Error {
    /// Implements mutable derefferencing the error from the wrapper.
    ///
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FastaError> for Error {
    /// Creates and Wrapper from the original error.
    /// 
    /// # Arguments
    ///
    /// * `err` - Original error
    fn from(err: FastaError) -> Self {
        Self(err)
    }
}

impl From<Error> for PyErr {
    /// Converts error wrapper to PyErr
    /// 
    /// # Arguments
    ///
    /// * `err` - Error wrapper
    fn from(err: crate::fasta::error::Error) -> Self {
        match *err {
            FastaError::IoError(ref err) => PyIOError::new_err(format!("{err}"))
        }
    }
}
