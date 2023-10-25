
/// Module for dealing with MSP files

pub mod psm;
pub mod reader;
pub mod prelude;
pub mod writer;

pub use prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    use std::iter::zip;
    use std::path::Path;

    const MSP_FILE_PATH_STR: &'static str = "../test_files/msp/example.msp";

    #[test]
    /// Read an MSP file, load the PSMs,
    /// write them back into a temporary file and compares it with the original one.
    fn test_reading_and_writing() {
        let msp_file_path = Path::new(MSP_FILE_PATH_STR);

        let msp_reader = MspReader::new(
            msp_file_path,
            1024
        ).unwrap();

        let entries: Vec<MspPsm> = msp_reader.into_fallible_iter().collect().unwrap();

        _write_and_compare_entries(entries)
    }

    #[test]
    /// Read an MSP file, load the PSMs, parse them
    /// write them back into a temporary file and compares it with the original one.
    fn test_parsing_roundtrip() {
        let msp_file_path = Path::new(MSP_FILE_PATH_STR);

        let msp_reader = MspReader::new(
            msp_file_path,
            1024
        ).unwrap();

        let entries = msp_reader.into_fallible_iter().map(|read_psm| {
            let parsed_psm = read_psm.to_parsed_psm().unwrap();
            let psm = parsed_psm.to_psm().unwrap();
            Ok(psm)
        }).collect::<Vec<MspPsm>>().unwrap();

        _write_and_compare_entries(entries)
    }

    fn _write_and_compare_entries(entries: Vec<MspPsm>) {

        let msp_file_path = Path::new(MSP_FILE_PATH_STR);

        let test_msp_content = fs::read_to_string(msp_file_path).unwrap();

        let mut buffer: Vec<u8> = Vec::new();

        // Perform operations that require mutable access to the buffer
        {
            let mut msp_writer = MspWriter::from_buffer(&mut buffer);
            msp_writer.write_all(entries.iter()).unwrap();
            msp_writer.flush().unwrap();
        }

        let tmp_msp_content = String::from_utf8_lossy(&*buffer);

        assert_eq!(
            test_msp_content.lines().count(),
            tmp_msp_content.lines().count(),
            "different number of lines between the test and generated MSP contents"
        );

        // Compare files line by line without new line characters,
        // to make sure different line endings doesn't falsify the test.
        for (test_line, tmp_line) in zip(test_msp_content.lines(), tmp_msp_content.lines()) {
            assert_eq!(
                test_line,
                tmp_line
            )
        }
    }
}
