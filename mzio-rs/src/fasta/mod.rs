pub mod entry;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    use std::path::Path;
    use crate::fasta::writer::Writer;

    const FASTA_FILE_PATH_STR: &'static str = "../test_files/fasta/partial_mouse.fasta";
    const EXPECTED_NUM_PROTEINS: usize = 10;
    const TEMP_FASTA_PATH_STR: &'static str = "../test_files/fasta/partial_mouse.fasta.tmp";

    #[test]
    /// Reads a FASTA file, parses the proteins,
    /// write them back into a temporary file and compares it with the original one.
    fn test_reading_and_writing() {
        let fasta_file_path = Path::new(FASTA_FILE_PATH_STR);
        let tmp_fasta_file_path = Path::new(TEMP_FASTA_PATH_STR);

        assert!(fasta_file_path.exists(), "FASTA file not found at path: {}", FASTA_FILE_PATH_STR);

        let reader = reader::Reader::new(
            fasta_file_path,
            1024,
            true
        ).unwrap();

        let mut entries = Vec::with_capacity(1000);

        for entry in reader {

            let entry_as_string = Writer::stringify_entry(&entry, true, None);

            assert_eq!(
                [entry.get_plain_header().to_owned().unwrap(), entry.get_sequence().to_owned()].join("\n"),
                entry_as_string,
                "the formatted entries are not matching"
            );

            entries.push(entry);
        }
        assert_eq!(entries.len(), EXPECTED_NUM_PROTEINS);

        let mut writer = writer::Writer::new_with_default_seq_formatting(
            tmp_fasta_file_path,
            true
        ).unwrap();

        writer.write_all(entries.iter()).unwrap();
        writer.flush().unwrap();

        let tmp_fasta_content  = fs::read_to_string(tmp_fasta_file_path).unwrap().trim().to_string();
        fs::remove_file(tmp_fasta_file_path).unwrap();

        assert_eq!(
            fs::read_to_string(fasta_file_path).unwrap().trim().replace("\r\n", "\n"),
            tmp_fasta_content.as_str()
        );
    }
}
