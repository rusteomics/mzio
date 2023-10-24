/// Module for dealing with MGF files

pub mod reader;
pub mod spectrum;
pub mod writer;
pub mod prelude;

pub use prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    use std::iter::zip;
    use std::path::Path;

    //use fallible_iterator::IntoFallibleIterator;
    //use fallible_iterator::FallibleIterator;

    const MGF_FILE_PATH_STR: &'static str = "../test_files/mgf/Velos005137.mgf";
    const EXPECTED_NUM_SPECTRA: usize = 100;
    const TEMP_MGF_PATH_STR: &'static str = "../test_files/mgf/Velos005137.mgf.tmp";

    #[test]
    /// Reads a MGF file, parses the spectra,
    /// write them back into a temporary file and compares it with the original one.
    fn test_reading_and_writing() {
        let mgf_file_path = Path::new(MGF_FILE_PATH_STR);
        let tmp_mgf_file_path = Path::new(TEMP_MGF_PATH_STR);

        let mgf_reader = MgfReader::new(
            mgf_file_path,
            1024
        ).unwrap();

        let entries: Vec<MgfSpectrum> = mgf_reader.into_fallible_iter().collect().unwrap();
        assert_eq!(entries.len(), EXPECTED_NUM_SPECTRA);

        let mut mgf_writer = MgfWriter::new(
            tmp_mgf_file_path
        ).unwrap();

        mgf_writer.write_all(entries.iter()).unwrap();
        mgf_writer.flush().unwrap();

        let tmp_mgf_content  = fs::read_to_string(tmp_mgf_file_path).unwrap().trim().to_string();
        fs::remove_file(tmp_mgf_file_path).unwrap();

        let test_mgf_content = fs::read_to_string(mgf_file_path).unwrap();

        // Compare files line by line without new line characters, 
        // to make sure different line endings doesn't falsify the test.
        for (test_line, tmp_line) in zip(test_mgf_content.lines(), tmp_mgf_content.lines()) {
            assert_eq!(
                test_line,
                tmp_line
            )
        }
    }
}
