use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crate::fasta::entry::Entry;
use anyhow::Result;

/// Reader for common FASTA files as distributed by UniProt (https://uniprot.org)
pub struct Reader {
    internal_reader: BufReader<File>,
    is_eof: bool,
    header: String,
    sequence: String,
}

impl Reader {
    /// Creates a new Reader
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    ///
    pub fn new(fasta_file_path: &Path, buffer_size: usize) -> Result<Self> {
        let fasta_file: File = File::open(fasta_file_path)?;
        Ok(Self {
            internal_reader: BufReader::with_capacity(buffer_size, fasta_file),
            is_eof: false,
            header: String::new(),
            sequence: String::new(),
        })
    }

    /// Processes and adds a keyword attribute to the `HashMap`
    /// # Arguments
    ///
    /// * `raw_attr` - Raw attributes, e.g. `key=value with spaces`
    /// * `keyword_attributes` - Additional keyword attributes
    ///
    fn prep_and_add_attribute_to_keyword_attributes(
        raw_attr: &str,
        keyword_attributes: &mut HashMap<String, String>,
    ) {
        let attr_split = raw_attr.split('=').collect::<Vec<&str>>();
        if let Some(key) = attr_split.first() {
            if let Some(value) = attr_split.get(1) {
                keyword_attributes.insert((*key).to_string(), (*value).to_string());
            }
        }
    }

    /// Creates a new Entry from the given header and sequence.
    ///
    /// # Arguments
    ///
    /// * `header` - A FASTA header
    /// * `sequence` - Amino acid sequence
    ///
    pub fn create_entry(header: &str, sequence: &str) -> Option<Entry> {
        // Split by '|' and extract database and accession
        let mut header_split = header.split('|').collect::<Vec<&str>>();
        let mut database: String = header_split.remove(0).to_string();
        database = database.as_str()[1..].to_string(); // remove '>'
        let accession: String = header_split.remove(0).to_string();

        // Split by ' '
        header_split = header_split.remove(0).split(' ').collect::<Vec<&str>>();

        // Extract entry name
        let entry_name: String = header_split.remove(0).to_string();
        // Add chunks to protein name until first string with '=' occurs (begin of keyword attributes)
        let mut protein_name: String = header_split.remove(0).to_string();
        loop {
            if let Some(chunk) = header_split.first() {
                if chunk.contains('=') {
                    break;
                }
                protein_name.push(' ');
                protein_name.push_str(header_split.remove(0));
            }
        }
        // Extract keyword attributes
        let mut keyword_attributes: HashMap<String, String> = HashMap::new();
        if !header_split.is_empty() {
            let mut current_attr: String = String::new();
            while !header_split.is_empty() {
                if let Some(chunk) = header_split.first() {
                    // Every time a chunk does not start a new attribute (chunk does not contains '=')
                    // add the chunk to the current attribute, otherwise process the current attribute
                    // and begin a new one
                    if chunk.contains('=') {
                        Self::prep_and_add_attribute_to_keyword_attributes(
                            &current_attr,
                            &mut keyword_attributes,
                        );
                        if !header_split.is_empty() {
                            current_attr = header_split.remove(0).to_string();
                        }
                    } else {
                        current_attr.push(' ');
                        current_attr.push_str(header_split.remove(0));
                    }
                }
            }
            // Process the remaining attribute
            Self::prep_and_add_attribute_to_keyword_attributes(
                &current_attr,
                &mut keyword_attributes,
            );
        }
        Some(Entry::new(
            database,  // database
            accession, // accession
            entry_name,
            protein_name,
            keyword_attributes,
            sequence.replace('\n', ""),
        ))
    }
}

impl Iterator for Reader {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof {
            return None;
        }
        self.sequence = String::new(); // Reset sequence, new header is still present from last iteration.
        loop {
            let mut line = String::new();
            if let Ok(num_bytes) = self.internal_reader.read_line(&mut line) {
                if num_bytes == 0 {
                    self.is_eof = true;
                    return Self::create_entry(&self.header, &self.sequence);
                }
                line = line.as_mut_str().trim().to_string();
                if !line.starts_with('>') && num_bytes > 0 {
                    self.sequence.push_str(&line);
                } else if !self.header.is_empty() {
                    let entry = Self::create_entry(&self.header, &self.sequence);
                    self.header = line; // safe newly read header
                    return entry;
                } else {
                    self.header = line;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_HEADER: &str = ">sp|P27748|ACOX_CUPNH Acetoin catabolism protein X OS=Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337) OX=381666 GN=acoX PE=4 SV=2";
    const TEST_SEQUENCE: &str = "MGHAAGASAQIAPVVGIIANPISARDIRRVIANANSLQLADRVNIVLRLLAALASCGVER
VLMMPDREGLRVMLARHLARRQGPDSGLPAVDYLDMPVTARVDDTLRAARCMADAGVAAI
IVLGGDGTHRAVVRECGAVPIAGLSTGTNNAYPEMREPTIIGLATGLYATGRIPPAQALA
SNKRLDIVIRDGNGGFRRDIALVDAVISHEHFIGARALWKTDTLAAVYVSFADPEAIGLS
SIAGLLEPVGRREEGGLAIELAAPGEGEFDLCAPIAPGLMCTVPVAGWQRLEHGRPHRVR
QRSGIVALDGERELAFGPDDEVTVTLHDHAFRSIDVAACMRHAGRHHLMRSLPQPAAVG";
    const EXPECTED_DATABASE: &str = "sp";
    const EXPECTED_ACCESSION: &str = "P27748";
    const EXPECTED_ENTRY_NAME: &str = "ACOX_CUPNH";
    const EXPECTED_PROTEIN_NAME: &str = "Acetoin catabolism protein X";
    const EXPECTED_KEYWORD_ATTRIBUTES: [(&str, &str); 5] = [
        (
            "OS",
            "Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337)",
        ),
        ("OX", "381666"),
        ("GN", "acoX"),
        ("PE", "4"),
        ("SV", "2"),
    ];
    const EXPECTED_SEQUENCE: &str = "MGHAAGASAQIAPVVGIIANPISARDIRRVIANANSLQLADRVNIVLRLLAALASCGVER\
        VLMMPDREGLRVMLARHLARRQGPDSGLPAVDYLDMPVTARVDDTLRAARCMADAGVAAI\
        IVLGGDGTHRAVVRECGAVPIAGLSTGTNNAYPEMREPTIIGLATGLYATGRIPPAQALA\
        SNKRLDIVIRDGNGGFRRDIALVDAVISHEHFIGARALWKTDTLAAVYVSFADPEAIGLS\
        SIAGLLEPVGRREEGGLAIELAAPGEGEFDLCAPIAPGLMCTVPVAGWQRLEHGRPHRVR\
        QRSGIVALDGERELAFGPDDEVTVTLHDHAFRSIDVAACMRHAGRHHLMRSLPQPAAVG";

    #[test]
    /// Tests the creation of a FASTA entry from a header and a sequence.
    ///
    fn test_entry_creation() {
        let entry = Reader::create_entry(TEST_HEADER, TEST_SEQUENCE).unwrap();
        assert_eq!(entry.get_database(), EXPECTED_DATABASE);
        assert_eq!(entry.get_accession(), EXPECTED_ACCESSION);
        assert_eq!(entry.get_entry_name(), EXPECTED_ENTRY_NAME);
        assert_eq!(entry.get_protein_name(), EXPECTED_PROTEIN_NAME);
        assert_eq!(entry.get_sequence(), EXPECTED_SEQUENCE);

        for key_value in EXPECTED_KEYWORD_ATTRIBUTES {
            assert!(entry.get_keyword_attributes().contains_key(key_value.0));
            assert_eq!(
                entry.get_keyword_attributes().get(key_value.0).unwrap(),
                key_value.1
            );
        }
    }
}
