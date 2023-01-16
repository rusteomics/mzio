use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

use crate::fasta::entry::Entry;
use crate::fasta::error::Error;

/// Max amino acids per sequence line.
const MAX_AMINO_ACIDS_PER_LINE: usize = 60;

/// Writer for common FASTA files as distributed by UniProt (https://uniprot.org)
/// Use flush() to mak ensure the buffer is written completely.
pub struct Writer {
    internal_writer: BufWriter<File>
}

impl Writer {
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// 
    pub fn new(fasta_file_path: &Path) -> Result<Self, Error> {
        let fasta_file: File = File::create(fasta_file_path)?;
        Ok(Self {
            internal_writer: BufWriter::new(fasta_file)
        })
    }

    /// Creates a fasta header from the given entry.
    /// 
    /// # Arguments
    ///
    /// * `entry` - FASTA entry
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    /// 
    fn create_header(entry: &Entry, sort_keyword_attributes: bool) -> String {
        let mut header = ">".to_string();
        header.push_str(entry.get_database());
        header.push_str("|");
        header.push_str(entry.get_accession());
        header.push_str("|");
        header.push_str(entry.get_entry_name());
        header.push_str(" ");
        header.push_str(entry.get_protein_name());
        if entry.get_keyword_attributes().len() > 0 {
            header.push_str(" ");
            let mut keyword_arguments: Vec<String> = entry.get_keyword_attributes().into_iter()
                .map(|(key, value)| format!("{}={}", key, value)).collect();
            if sort_keyword_attributes {
                keyword_arguments.sort();

            }
            header.push_str(&keyword_arguments.join(" "));
        }
        return header;
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
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    /// 
    pub fn write_entry(&mut self, entry: &Entry, sort_keyword_attributes: bool) -> Result<usize, Error> {
        let mut written_bytes: usize = 0;
        written_bytes += self.internal_writer.write(Self::create_header(entry, sort_keyword_attributes).as_bytes())?;
        written_bytes += self.internal_writer.write(b"\n")?;
        written_bytes += self.internal_writer.write(Self::format_sequence(entry.get_sequence()).as_bytes())?;
        written_bytes += self.internal_writer.write(b"\n")?;
        return Ok(written_bytes);
    }

    /// Writes multiple FASTA entry to file.
    /// 
    /// # Arguments
    ///
    /// * `entires` - Iterator of FASTA entries
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    /// 
    pub fn write_all<'b, I>(&mut self, entries: I, sort_keyword_attributes: bool) -> Result<usize, Error>
    where
        I: Iterator<Item = &'b Entry>,
    {
        let mut written_bytes: usize = 0;
        for entry in entries {
            written_bytes += self.write_entry(entry, sort_keyword_attributes)?;
        }
        return Ok(written_bytes);
    }

    /// Flushes the buffer
    /// 
    pub fn flush(&mut self) -> Result<(), Error> {
        self.internal_writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    const TEST_SEQUENCE: &'static str = "MGHAAGASAQIAPVVGIIANPISARDIRRVIANANSLQLADRVNIVLRLLAALASCGVER\
        VLMMPDREGLRVMLARHLARRQGPDSGLPAVDYLDMPVTARVDDTLRAARCMADAGVAAI\
        IVLGGDGTHRAVVRECGAVPIAGLSTGTNNAYPEMREPTIIGLATGLYATGRIPPAQALA\
        SNKRLDIVIRDGNGGFRRDIALVDAVISHEHFIGARALWKTDTLAAVYVSFADPEAIGLS\
        SIAGLLEPVGRREEGGLAIELAAPGEGEFDLCAPIAPGLMCTVPVAGWQRLEHGRPHRVR\
        QRSGIVALDGERELAFGPDDEVTVTLHDHAFRSIDVAACMRHAGRHHLMRSLPQPAAVG";
    const TEST_DATABASE: &'static str = "sp";
    const TEST_ACCESSION: &'static str = "P27748";
    const TEST_ENTRY_NAME: &'static str = "ACOX_CUPNH";
    const TEST_PROTEIN_NAME: &'static str = "Acetoin catabolism protein X";
    const TEST_KEYWORD_ATTRIBUTES: [(&'static str, &'static str,); 5] = [
        ("OS", "Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337)"),
        ("OX", "381666"),
        ("GN", "acoX"),
        ("PE", "4"),
        ("SV", "2")
    ];
    const EXPECTED_HEADER: &'static str = ">sp|P27748|ACOX_CUPNH Acetoin catabolism protein X GN=acoX OS=Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337) OX=381666 PE=4 SV=2";
    const EXPECTED_SEQUENCE: &'static str = "MGHAAGASAQIAPVVGIIANPISARDIRRVIANANSLQLADRVNIVLRLLAALASCGVER
VLMMPDREGLRVMLARHLARRQGPDSGLPAVDYLDMPVTARVDDTLRAARCMADAGVAAI
IVLGGDGTHRAVVRECGAVPIAGLSTGTNNAYPEMREPTIIGLATGLYATGRIPPAQALA
SNKRLDIVIRDGNGGFRRDIALVDAVISHEHFIGARALWKTDTLAAVYVSFADPEAIGLS
SIAGLLEPVGRREEGGLAIELAAPGEGEFDLCAPIAPGLMCTVPVAGWQRLEHGRPHRVR
QRSGIVALDGERELAFGPDDEVTVTLHDHAFRSIDVAACMRHAGRHHLMRSLPQPAAVG";

    #[test]
    /// Tests the creation of a FASTA entry from a header and a sequence.
    ///
    fn test_seqeunce_formatting() {
        let formatted_sequence  = Writer::format_sequence(TEST_SEQUENCE);
        assert_eq!(formatted_sequence, EXPECTED_SEQUENCE)
    }

    #[test]
    /// Reads a FASTA file, parses the proteins and counts protein
    fn test_header_creation() {
        let entry = Entry::new(
            TEST_DATABASE.to_string(),
            TEST_ACCESSION.to_string(),
            TEST_ENTRY_NAME.to_string(),
            TEST_PROTEIN_NAME.to_string(),
            TEST_KEYWORD_ATTRIBUTES.into_iter().map(|elem| (elem.0.to_string(), elem.1.to_string())).collect::<HashMap<String, String>>(),
            TEST_SEQUENCE.to_string()
        );
        let header = Writer::create_header(&entry, true);
        assert_eq!(header, EXPECTED_HEADER);
    }
}