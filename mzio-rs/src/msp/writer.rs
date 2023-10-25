use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::Result;

// internal imports 
use crate::msp::MspPsm;

/// Writer for MSP files
/// Use flush() to ensure the buffer is written completely.
pub struct MspWriter<W: Write> {
    psm_count: usize,
    internal_writer: BufWriter<W>
}

impl<W: Write> MspWriter<W> {
    /// Creates a new MspWriter
    /// 
    /// # Arguments
    ///
    /// * `buf_writer` - Buffer Writer
    /// 
    pub fn new(buf_writer: BufWriter<W>) -> Self {
        Self {
            psm_count: 0,
            internal_writer: buf_writer
        }
    }

    /// Creates a new MspWriter from a byte slice reference
    ///
    /// # Arguments
    ///
    /// * `buffer` - Buffer used to write the bytes
    ///
    /*pub fn from_buffer(buffer: &[u8]) -> MspWriter<Cursor<&[u8]>> {
        let cursor = Cursor::new(buffer);
        let buf_writer = BufWriter::new(cursor);
        MspWriter::new(buf_writer)
    }*/

    /*pub fn from_buffer<'a>(buffer: &'a mut Vec<u8>) -> MspWriter<Cursor<&'a mut Vec<u8>>> {
        let cursor = Cursor::new(buffer);
        let buf_writer = BufWriter::new(cursor);
        MspWriter::new(buf_writer)
    }*/

    /*pub fn from_buffer(buffer: &mut Vec<u8>) -> MspWriter {
        let buf_writer = BufWriter::new(buffer);
        MspWriter::new(buf_writer)
    }*/


    /// Writes a PSM into the file.
    ///
    /// # Arguments
    ///
    /// * `psm` - MSP PSM entry
    ///
    pub fn write_psm(&mut self, psm: &MspPsm) -> Result<usize> {
        let psm_header = &psm.header;

        let mut written_bytes: usize = 0;

        if self.psm_count > 0 {
            // Add an empty line between consecutive PSMs
            written_bytes += self._write_str("\n")?;
        }

        written_bytes += self._write_string(format!("Name: {}\n", psm_header.get_name()))?;
        written_bytes += self._write_string(format!("MW: {}\n", psm_header.get_mw()))?;
        written_bytes += self._write_string(format!("Comment: {}\n", psm_header.get_comment()))?;
        written_bytes += self._write_string(format!("Num peaks: {}\n", psm_header.get_num_peaks()))?;

        for ((&mz, &intensity), &ref annotation) in psm.get_mz_list().iter().zip(psm.get_intensity_list()).zip(psm.get_annotation_list()) {
            written_bytes += self._write_string(format!("{mz}\t{intensity}\t\"{annotation}\"\n"))?;
        }

        self.psm_count += 1;

        Ok(written_bytes)
    }

    #[inline(always)]
    fn _write_str(&mut self, str: &str) -> Result<usize> {
        Ok(self.internal_writer.write(str.as_bytes())?)
    }

    #[inline(always)]
    fn _write_string(&mut self, string: String) -> Result<usize> {
        Ok(self.internal_writer.write(string.as_bytes())?)
    }

    /// Writes multiple spectra to file.
    ///
    /// # Arguments
    ///
    /// * `psms` - Iterator of PSMs
    ///
    pub fn write_all<'b, I>(&mut self, psms: I) -> Result<usize>
    where
        I: Iterator<Item = &'b MspPsm>,
    {
        let mut written_bytes: usize = 0;
        for psm in psms {
            written_bytes += self.write_psm(psm)?;
        }

        Ok(written_bytes)
    }

    /// Flushes the buffer
    ///
    pub fn flush(&mut self) -> Result<()> {
        self.internal_writer.flush()?;
        Ok(())
    }

}

pub trait MspFileWriter {
    fn from_path(msp_file_path: &Path) -> Result<MspWriter<File>>;
}

impl MspFileWriter for MspWriter<File> {
    fn from_path(msp_file_path: &Path) -> Result<MspWriter<File>> {
        let file = File::create(msp_file_path)?;
        let buf_writer = BufWriter::new(file);
        Ok(MspWriter::new(buf_writer))
    }
}

pub trait MspBufferWriter {
    fn from_buffer(buffer: &mut Vec<u8>) -> MspWriter<&mut Vec<u8>>;
}

impl MspBufferWriter for MspWriter<&mut Vec<u8>> {
    fn from_buffer(buffer: &mut Vec<u8>) -> MspWriter<&mut Vec<u8>> {
        let buf_writer = BufWriter::new(buffer);
        MspWriter::new(buf_writer)
    }
}


/*
pub trait MspFileWriter {
    /// Creates a new MspWriter from a file path
    ///
    /// # Arguments
    ///
    /// * `msp_file_path` - Path to MSP file
    ///
    fn from_path(msp_file_path: &Path) -> Result<MspWriter<File>>;
}

impl MspFileWriter for MspWriter<File> {

    /// Creates a new MspWriter from a file path
    ///
    /// # Arguments
    ///
    /// * `msp_file_path` - Path to MSP file
    ///
    fn from_path(msp_file_path: &Path) -> Result<MspWriter<File>> {
        Ok(MspWriter::new(BufWriter::new(File::create(msp_file_path)?)))
    }
}*/