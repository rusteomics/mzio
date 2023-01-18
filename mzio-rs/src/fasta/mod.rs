pub mod entry;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    use std::path::Path;

    const FASTA_FILE_PATH_STR: &'static str = "./test_files/fasta/partial_mouse.fasta";
    const EXPECTED_NUM_PROTEINS: usize = 10;
    const TEMP_FASTA_PATH_STR: &'static str = "./test_files/fasta/partial_mouse.fasta.tmp";

    #[test]
    /// Reads a FASTA file, parses the proteins,
    /// write them back into a temporary file and compares it with the original one.
    fn test_reading_and_writing() {
        let fasta_file_path = Path::new(FASTA_FILE_PATH_STR);
        let tmp_fasta_file_path = Path::new(TEMP_FASTA_PATH_STR);

        let reader = reader::Reader::new(
            fasta_file_path
        ).unwrap();

        let entries: Vec<entry::Entry> = reader.into_iter().collect();
        assert_eq!(entries.len(), EXPECTED_NUM_PROTEINS);

        let mut writer = writer::Writer::new(
            tmp_fasta_file_path
        ).unwrap();

        writer.write_all(entries.iter(), true).unwrap();
        writer.flush().unwrap();

        let tmp_fasta_content  = fs::read_to_string(tmp_fasta_file_path).unwrap().trim().to_string();
        fs::remove_file(tmp_fasta_file_path).unwrap();

        assert_eq!(
            fs::read_to_string(fasta_file_path).unwrap().trim(),
            tmp_fasta_content.as_str()
        );
    }
}
