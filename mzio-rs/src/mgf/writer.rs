use std::fs::File;
use std::iter::zip;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::Result;

// internal imports 
use crate::mgf::spectrum::MgfSpectrum;

/// Writer for MGF files
/// Use flush() to make ensure the buffer is written completely.
pub struct MgfWriter {
    internal_writer: BufWriter<File>
}

impl MgfWriter {
    /// Creates a new Writer
    /// 
    /// # Arguments
    ///
    /// * `mgf_file_path` - Path to MGF file
    /// 
    pub fn new(mgf_file_path: &Path) -> Result<Self> {
        let mgf_file: File = File::create(mgf_file_path)?;
        Ok(Self {
            internal_writer: BufWriter::new(mgf_file)
        })
    }

    /// Writes a spectrum into the file.
    /// 
    /// # Arguments
    ///
    /// * `spectrum` - Spectrum
    /// 
    pub fn write_spectrum(&mut self, spectrum: &MgfSpectrum) -> Result<usize> {
        let spec_header = &spectrum.header;

        let mut written_bytes: usize = 0;
        written_bytes += self.internal_writer.write("BEGIN IONS\n".as_bytes())?;
        written_bytes += self.internal_writer.write(format!("TITLE={}\n", spec_header.get_title()).as_bytes())?;
        written_bytes += self.internal_writer.write(format!("PEPMASS={}", spec_header.get_precursor_mz()).as_bytes())?;
        if let Some(retention_time) = spec_header.get_retention_time() {
            written_bytes += self.internal_writer.write(format!("\nRTINSECONDS={}", retention_time).as_bytes())?;
        }
        if let Some(charge) = spec_header.get_precursor_charge() {
            let charge_sign = if charge < 0 { '-'} else { '+' };
            written_bytes += self.internal_writer.write(format!("\nCHARGE={}{}", charge, charge_sign).as_bytes())?;
        }
        for (mz, intensity) in zip(spectrum.get_mz_list(), spectrum.get_intensity_list()) {
            written_bytes += self.internal_writer.write(format!("\n{mz} {intensity}").as_bytes())?;
        }
        written_bytes += self.internal_writer.write("\nEND IONS\n".as_bytes())?;
        return Ok(written_bytes);
    }

    /// Writes multiple spectra to file.
    /// 
    /// # Arguments
    ///
    /// * `spectra` - Iterator of spectra
    /// 
    pub fn write_all<'b, I>(&mut self, spectra: I) -> Result<usize>
    where
        I: Iterator<Item = &'b MgfSpectrum>,
    {
        let mut written_bytes: usize = 0;
        for spectrum in spectra {
            written_bytes += self.write_spectrum(spectrum)?;
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