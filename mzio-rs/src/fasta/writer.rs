// std imports
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::Result;

// internal imports
use crate::fasta::entry::Entry;
use crate::fasta::headers::Header;


/// Max amino acids per sequence line.
const MAX_AMINO_ACIDS_PER_LINE: usize = 60;

/// Writer for common FASTA files as distributed by UniProt (https://uniprot.org)
/// Use flush() to make sure the buffer is written completely.
pub struct Writer<T> where T: Header {
    internal_writer: BufWriter<File>,
    _header_phantom: std::marker::PhantomData<T>
}

impl<'a, T> Writer<T> where T: Header + 'a{
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// 
    pub fn new(fasta_file_path: &Path) -> Result<Self> {
        let fasta_file: File = File::create(fasta_file_path)?;
        Ok(Self {
            internal_writer: BufWriter::new(fasta_file),
            _header_phantom: std::marker::PhantomData
        })
    }

    /// Splits sequence into chunk of MAX_AMINO_ACIDS_PER_LINE.
    /// 
    /// # Arguments
    ///
    /// * `sequence` - Amino acid sequence
    /// 
    fn format_sequence(sequence: &str) -> String {
        return sequence.chars()
            .collect::<Vec<char>>()
            .chunks(MAX_AMINO_ACIDS_PER_LINE)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
    }

    /// Writes entry into the FASTA file.
    /// 
    /// # Arguments
    ///
    /// * `entry` - FASTA entry
    /// 
    pub fn write_entry(&mut self, entry: &Entry<T>) -> Result<usize> {
        let mut written_bytes: usize = 0;
        written_bytes += self.internal_writer.write(format!(
            "{}\n{}\n", entry.get_header(),
            Self::format_sequence(entry.get_sequence())
        ).as_bytes())?;
        return Ok(written_bytes);
    }

    /// Writes multiple FASTA entry to file.
    /// 
    /// # Arguments
    ///
    /// * `entires` - Iterator of FASTA entries
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    /// 
    pub fn write_all<'b, I>(&mut self, entries: I) -> Result<usize>
    where
        I: Iterator<Item = &'a Entry<T>>,
    {
        let mut written_bytes: usize = 0;
        for entry in entries {
            written_bytes += self.write_entry(entry)?;
        }
        return Ok(written_bytes);
    }

    /// Flushes the buffer
    /// 
    pub fn flush(&mut self) -> Result<()> {
        self.internal_writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::fasta::headers::{
        plain::Plain
    };

    const TEST_SEQUENCE: &'static str = "MGHAAGASAQIAPVVGIIANPISARDIRRVIANANSLQLADRVNIVLRLLAALASCGVER\
        VLMMPDREGLRVMLARHLARRQGPDSGLPAVDYLDMPVTARVDDTLRAARCMADAGVAAI\
        IVLGGDGTHRAVVRECGAVPIAGLSTGTNNAYPEMREPTIIGLATGLYATGRIPPAQALA\
        SNKRLDIVIRDGNGGFRRDIALVDAVISHEHFIGARALWKTDTLAAVYVSFADPEAIGLS\
        SIAGLLEPVGRREEGGLAIELAAPGEGEFDLCAPIAPGLMCTVPVAGWQRLEHGRPHRVR\
        QRSGIVALDGERELAFGPDDEVTVTLHDHAFRSIDVAACMRHAGRHHLMRSLPQPAAVG";
    
    const EXPECTED_SEQUENCE: &'static str = "MGHAAGASAQIAPVVGIIANPISARDIRRVIANANSLQLADRVNIVLRLLAALASCGVER
VLMMPDREGLRVMLARHLARRQGPDSGLPAVDYLDMPVTARVDDTLRAARCMADAGVAAI
IVLGGDGTHRAVVRECGAVPIAGLSTGTNNAYPEMREPTIIGLATGLYATGRIPPAQALA
SNKRLDIVIRDGNGGFRRDIALVDAVISHEHFIGARALWKTDTLAAVYVSFADPEAIGLS
SIAGLLEPVGRREEGGLAIELAAPGEGEFDLCAPIAPGLMCTVPVAGWQRLEHGRPHRVR
QRSGIVALDGERELAFGPDDEVTVTLHDHAFRSIDVAACMRHAGRHHLMRSLPQPAAVG";

    #[test]
    /// Tests the creation of a FASTA entry from a header and a sequence.
    /// Same for ech header header.
    ///
    fn test_seqeunce_formatting() {
        let formatted_sequence  = Writer::<Plain>::format_sequence(TEST_SEQUENCE);
        assert_eq!(formatted_sequence, EXPECTED_SEQUENCE)
    }
}
