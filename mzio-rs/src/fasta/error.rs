use std::fmt;
use std::io;

/// Represents errors which can occure during FASTA handling.
#[derive(Debug)]
pub enum Error {
    IoError(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::IoError(ref message) => write!(f, "{}", message)
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        return Error::IoError(err)
    }
}