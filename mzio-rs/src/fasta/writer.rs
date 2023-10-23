use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

use crate::fasta::entry::Entry;
use anyhow::Result;

/// DEFAULT max amino acids per sequence line.
const DEFAULT_MAX_AMINO_ACIDS_PER_LINE: usize = 60;

/// Writer for common FASTA files as distributed by UniProt (https://uniprot.org)
/// Use flush() to mak ensure the buffer is written completely.
pub struct Writer {
    /// Max amino acids per sequence line.
    max_amino_acids_per_line: Option<usize>,
    sort_keyword_attributes: bool,
    internal_writer: BufWriter<File>
}

impl Writer {
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    /// * `max_amino_acids_per_line` - If Some(), will format the sequence line to not exceed the given length.
    /// 
    pub fn new(fasta_file_path: &Path, sort_keyword_attributes: bool, max_amino_acids_per_line: Option<usize>) -> Result<Self> {
        let fasta_file: File = File::create(fasta_file_path)?;

        Ok(Self {
            max_amino_acids_per_line,
            sort_keyword_attributes,
            internal_writer: BufWriter::new(fasta_file),
        })
    }

    /// Creates a new Writer
    ///
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    ///
    pub fn new_with_default_seq_formatting(fasta_file_path: &Path, sort_keyword_attributes: bool) -> Result<Self> {
        let fasta_file: File = File::create(fasta_file_path)?;

        Ok(Self {
            max_amino_acids_per_line: Some(DEFAULT_MAX_AMINO_ACIDS_PER_LINE),
            sort_keyword_attributes,
            internal_writer: BufWriter::new(fasta_file)
        })
    }

    /// Creates a new Writer
    ///
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// * `sort_keyword_attributes` - If true the keyword attributes will be sorted (for testing and readability reasons)
    ///
    pub fn new_without_seq_formatting(fasta_file_path: &Path, sort_keyword_attributes: bool) -> Result<Self> {
        let fasta_file: File = File::create(fasta_file_path)?;

        Ok(Self {
            max_amino_acids_per_line: None,
            sort_keyword_attributes,
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
    /// * `max_amino_acids_per_line` - Maximum length of each line containing sequence amino acids.
    /// 
    fn format_sequence(sequence: &str, max_amino_acids_per_line: usize) -> String {
        return sequence.chars()
            .collect::<Vec<char>>()
            .chunks(max_amino_acids_per_line)
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
    /// * `max_amino_acids_per_line` - If Some(), will format the sequence line to not exceed the given length.
    ///
    pub fn stringify_entry(entry: &Entry, sort_keyword_attributes: bool, max_amino_acids_per_line: Option<usize>) -> String {

        let header_as_string = Self::create_header(entry, sort_keyword_attributes);

        let seq_as_string = if max_amino_acids_per_line.is_some() {
            Self::format_sequence(entry.get_sequence(), max_amino_acids_per_line.unwrap())
        } else {
            entry.get_sequence().to_string()
        };

        [header_as_string, seq_as_string].join("\n")
    }

    /// Writes entry into the FASTA file.
    /// 
    /// # Arguments
    ///
    /// * `entry` - FASTA entry
    ///
    pub fn write_entry(&mut self, entry: &Entry) -> Result<usize> {
        let mut entry_as_string = Self::stringify_entry(entry, self.sort_keyword_attributes, self.max_amino_acids_per_line);
        entry_as_string.push_str("\n");

        let written_bytes = self.internal_writer.write(entry_as_string.as_bytes())?;

        return Ok(written_bytes);
    }

    /// Writes multiple FASTA entry to file.
    /// 
    /// # Arguments
    ///
    /// * `entried` - Iterator of FASTA entries
    ///
    pub fn write_all<'b, I>(&mut self, entries: I) -> Result<usize>
    where
        I: Iterator<Item = &'b Entry>,
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
    fn test_sequence_formatting() {
        let formatted_sequence  = Writer::format_sequence(TEST_SEQUENCE, DEFAULT_MAX_AMINO_ACIDS_PER_LINE);
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
            TEST_SEQUENCE.to_string(),
            None
        );
        let header = Writer::create_header(&entry, true);
        assert_eq!(header, EXPECTED_HEADER);
    }
}
