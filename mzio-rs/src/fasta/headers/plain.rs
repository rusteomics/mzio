// std imports
use std::fmt;

// local imports
use crate::fasta::headers::Header;

/// A plain header, which is just a string.
/// 
pub struct Plain {
    header: String
}

impl Plain {
    pub fn get_header(&self) -> &str {
        &self.header
    }
}

impl Header for Plain {
    fn new(header: &str) -> Self {
        Self { 
            header: header.to_owned()
        }
    }
}

impl fmt::Display for Plain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.header)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_HEADER: &'static str = ">sp|P27748|ACOX_CUPNH Acetoin catabolism protein X OS=Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337) OX=381666 GN=acoX PE=4 SV=2";

    #[test]
    fn test_display() {
        let plain = Plain::new(TEST_HEADER);
        assert_eq!(plain.to_string(), TEST_HEADER.to_owned());
    }
}