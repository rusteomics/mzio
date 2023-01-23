/// Contains FASTA I/O.
pub mod entry;
pub mod headers;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod test {
    use super::*;

    use std::fmt::Display;
    use std::fs;
    use std::path::Path;

    use crate::fasta::headers::{
        Header,
        plain::Plain,
        uniprot::UniProt
    };


    const FASTA_FILE_PATH_STR: &'static str = "./test_files/fasta/partial_mouse.fasta";
    const EXPECTED_NUM_PROTEINS: usize = 10;
    const TEMP_FASTA_PATH_STR: &'static str = "./test_files/fasta/partial_mouse.fasta.tmp";


    /// Test reading and writing of a FASTA without parsing the header
    ///
    #[test]
    fn test_plain_header() {
        test_reading_and_writing::<Plain>("plain");
    }

    /// Test reading and writing header while parsing the headers UniProt style
    /// 
    #[test] 
    fn test_uniprot_header() {
        test_reading_and_writing::<UniProt>("uniprot");
    }

    /// Reads a FASTA file, parses the proteins,
    /// write them back into a temporary file and compares it with the original one.
    /// 
    /// # Arguments
    /// `tmp_file_suffix` - Suffix for the temporary file
    ///
    fn test_reading_and_writing<T>(tmp_file_suffix: &'static str) where T: Header + Display {
        let fasta_file_path = Path::new(FASTA_FILE_PATH_STR);
        let tmp_file_path_str = format!("{}.{}", TEMP_FASTA_PATH_STR, tmp_file_suffix);
        let tmp_fasta_file_path = Path::new(&tmp_file_path_str);

        let reader = reader::Reader::new(
            fasta_file_path,
            1024
        ).unwrap();

        let entries: Vec<entry::Entry<T>> = reader.into_iter().collect();
        assert_eq!(entries.len(), EXPECTED_NUM_PROTEINS);

        let mut writer = writer::Writer::new(
            tmp_fasta_file_path
        ).unwrap();

        writer.write_all(entries.iter()).unwrap();
        writer.flush().unwrap();

        let tmp_fasta_content  = fs::read_to_string(tmp_fasta_file_path).unwrap().trim().to_string();
        fs::remove_file(tmp_fasta_file_path).unwrap();

        assert_eq!(
            fs::read_to_string(fasta_file_path).unwrap().trim(),
            tmp_fasta_content.as_str()
        );
    }
}
