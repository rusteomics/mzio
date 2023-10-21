// std imports
use std::collections::HashMap;
use std::fmt;

// 3rd party imports
use crate::fasta::headers::Header;

/// A parsed UniProt header.
/// Keeps the database, accession, entry name, protein name and keyword attributes.
/// 
pub struct UniProt {
    database: String,
    accession: String,
    entry_name: String,
    protein_name: String,
    keyword_attributes: HashMap<String, String>
}

impl UniProt {
    /// Returns the database of the UniProt header.
    /// 
    pub fn get_database(&self) -> &str {
        &self.database
    }

    /// Returns the accession of the UniProt header.
    /// 
    pub fn get_accession(&self) -> &str {
        &self.accession
    }

    /// Returns the entry name of the UniProt header.
    /// 
    pub fn get_entry_name(&self) -> &str {
        &self.entry_name
    }

    /// Returns the protein name of the UniProt header.
    /// 
    pub fn get_protein_name(&self) -> &str {
        &self.protein_name
    }

    /// Returns the keyword attributes of the UniProt header, like gene (GN), organism (OS), etc.
    /// 
    pub fn get_keyword_attributes(&self) -> &HashMap<String, String> {
        &self.keyword_attributes
    }

    /// Processes and adds a keyword attribute to the HashMap
    /// # Arguments
    ///
    /// * `raw_attr` - Raw attributes, e.g. `key=value with spaces`
    /// * `keyword_attributes` - Additional keyword attributes
    /// 
    fn prep_and_add_attribute_to_keyword_attributes(raw_attr: &str, keyword_attributes: &mut HashMap<String, String>) {
        let attr_split = raw_attr.split("=").collect::<Vec<&str>>();
        if let Some(key) = attr_split.get(0) {
            if let Some(value) = attr_split.get(1) {
                keyword_attributes.insert(
                    key.to_string(),
                    value.to_string()
                );
            }
        }
    }

    /// Creates a UniProt header of the given header.
    /// 
    /// # Arguments
    ///
    /// * `header` - A FASTA header
    /// 
    fn internal_new(header: &str) -> Self {
        // Split by '|' and extract database and accession 
        let mut header_split = header.split("|").collect::<Vec<&str>>();
        let mut database: String = header_split.remove(0).to_string();
        database = database.as_str()[1..].to_string(); // remove '>'
        let accession: String = header_split.remove(0).to_string();

        // Split by ' '
        header_split = header_split.remove(0).split(" ").collect::<Vec<&str>>();

        // Extract entry name 
        let entry_name: String = header_split.remove(0).to_string();
        // Add chunks to protein name until first string with '=' occurs (begin of keyword attributes) 
        let mut protein_name: String = header_split.remove(0).to_string();
        loop {
            if let Some(chunk) = header_split.get(0) {
                if !chunk.contains("=") {
                    protein_name.push_str(" ");
                    protein_name.push_str(header_split.remove(0));
                } else {
                    break
                }
            }
        }
        // Extract keyword attributes
        let mut keyword_attributes: HashMap<String, String> = HashMap::new();
        if header_split.len() > 0 {
            let mut current_attr: String = String::new();
            while header_split.len() > 0 {
                if let Some(chunk) = header_split.get(0) {
                    // Every time a chunk does not start a new attribute (chunk does not contains '=')
                    // add the chunk to the current attribute, otherwise process the current attribute
                    // and begin a new one
                    if !chunk.contains("=") {
                        current_attr.push_str(" ");
                        current_attr.push_str(header_split.remove(0));
                    } else {
                        Self::prep_and_add_attribute_to_keyword_attributes(
                            &current_attr, 
                            &mut keyword_attributes
                        );
                        if header_split.len() > 0 {
                            current_attr = header_split.remove(0).to_string();
                        }
                    }
                }
            }
            // Process the remaining attribute
            Self::prep_and_add_attribute_to_keyword_attributes(
                &current_attr, 
                &mut keyword_attributes
            );
        }
        Self {
            database,   // database
            accession,   // accession
            entry_name,
            protein_name,
            keyword_attributes
        }
    }

    /// Converts the keyword_attributes to a String, format
    /// `key=value key=value`
    /// 
    /// **Attention!** In test mode it will sort the attributes to make sure it the same output.
    ///
    fn keyword_attributes_to_string(&self) -> String {
        if self.get_keyword_attributes().len() > 0 {
            #[allow(unused_mut)] // needs to be mutable in test mode for sorting
            let mut keyword_arguments: Vec<String> = self.get_keyword_attributes().into_iter()
                .map(|(key, value)| format!("{}={}", key, value)).collect();
            // Only sort in test mode to have predictable output (HashMaps are not ordered).
            #[cfg(test)]
            {
                keyword_arguments.sort();
            }            
            return keyword_arguments.join(" ").to_owned();
        }
        return String::new();
    }
}

impl Header for UniProt {
    fn new(header: &str) -> Self {
        Self::internal_new(header)
    }
}

impl fmt::Display for UniProt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">{}|{}|{} {} {}", self.get_database(), self.get_accession(), self.get_entry_name(), self.get_protein_name(), self.keyword_attributes_to_string())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const TEST_HEADER: &'static str = ">sp|P27748|ACOX_CUPNH Acetoin catabolism protein X OS=Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337) OX=381666 GN=acoX PE=4 SV=2";
    const EXPECTED_DATABASE: &'static str = "sp";
    const EXPECTED_ACCESSION: &'static str = "P27748";
    const EXPECTED_ENTRY_NAME: &'static str = "ACOX_CUPNH";
    const EXPECTED_PROTEIN_NAME: &'static str = "Acetoin catabolism protein X";
    const EXPECTED_KEYWORD_ATTRIBUTES: [(&'static str, &'static str,); 5] = [
        ("OS", "Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337)"),
        ("OX", "381666"),
        ("GN", "acoX"),
        ("PE", "4"),
        ("SV", "2")
    ];
    const EXPECTED_HEADER: &'static str = ">sp|P27748|ACOX_CUPNH Acetoin catabolism protein X GN=acoX OS=Cupriavidus necator (strain ATCC 17699 / H16 / DSM 428 / Stanier 337) OX=381666 PE=4 SV=2";

    #[test]
    fn test_creation() {
        let uniprot = UniProt::new(TEST_HEADER);
        assert_eq!(uniprot.get_accession(), EXPECTED_ACCESSION);
        assert_eq!(uniprot.get_database(), EXPECTED_DATABASE);
        assert_eq!(uniprot.get_entry_name(), EXPECTED_ENTRY_NAME);
        assert_eq!(uniprot.get_protein_name(), EXPECTED_PROTEIN_NAME);
        for (key, value) in EXPECTED_KEYWORD_ATTRIBUTES.iter() {
            let key_value = uniprot.get_keyword_attributes().get_key_value(*key);
            assert_eq!(key_value, Some((&(*key).to_owned(), &(*value).to_owned())));
        }
    }

    #[test]
    fn test_display() {
        let uniprot = UniProt::new(TEST_HEADER);
        assert_eq!(uniprot.to_string(), EXPECTED_HEADER.to_owned());
    }
}