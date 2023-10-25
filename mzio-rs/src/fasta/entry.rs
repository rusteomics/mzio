use std::collections::HashMap;

/// Keeps all information of FASTA entry
pub struct Entry {
    database: String,
    accession: String,
    entry_name: String,
    protein_name: String,
    keyword_attributes: HashMap<String, String>,
    sequence: String,
    plain_header: Option<String>
}

impl Entry {
    /// Creates a new FASTA entry
    /// # Arguments
    ///
    /// * `database` - The FASTA database
    /// * `accession` - Entry accession
    /// * `entry_name` - Entry name
    /// * `protein_name` - Protein name
    /// * `keyword_attributes` - Additional keyword attributes, e.g. OX=381666
    /// * `sequence` - Amino acid sequence
    /// 
    pub fn new(database: String, accession: String, entry_name: String, protein_name: String,
        keyword_attributes: HashMap<String, String>, sequence: String, plain_header: Option<String>) -> Self {
            Self {
                database,
                accession,
                entry_name,
                protein_name,
                keyword_attributes,
                sequence,
                plain_header,
            }
        }

        /// Returns the database type
        ///
        pub fn get_database(&self) -> &String {
            &self.database
        }

        /// Returns the accession
        ///
        pub fn get_accession(&self) -> &String {
            &self.accession
        }

        /// Entry name
        ///
        pub fn get_entry_name(&self) -> &String {
            &self.entry_name
        }

        /// Returns the protein name
        ///
        pub fn get_protein_name(&self) -> &String {
            &self.protein_name
        }

        /// Returns additional keyword attributes, e.g
        /// * OX = 381666
        /// * GN = acoX
        ///
        pub fn get_keyword_attributes(&self) -> &HashMap<String, String> {
            &self.keyword_attributes
        }

        /// Returns the amino acid sequence
        /// 
        pub fn get_sequence(&self) -> &String {
            &self.sequence
        }

        /// Returns the plain header (before parsing)
        ///
        pub fn get_plain_header(&self) -> &Option<String> {
            &self.plain_header
        }
}