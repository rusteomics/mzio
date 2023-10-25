
// std imports
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::{Result, bail};
use fallible_iterator::FallibleIterator;

// internal imports
use crate::mgf::spectrum::MgfSpectrum;

/// Reader for MGF
pub struct MgfReader {
    internal_reader: BufReader<File>
}

impl MgfReader {
    /// Creates a new Reader
    /// 
    /// # Arguments
    ///
    /// * `mgf_file_path` - Path to MGF file
    /// 
    pub fn new(mgf_file_path: & Path, buffer_size: usize) -> Result<Self> {
        let mgf_file: File = File::open(mgf_file_path)?;
        Ok(Self {
            internal_reader: BufReader::with_capacity(buffer_size, mgf_file),
        })
    }
}

impl FallibleIterator for MgfReader {
    type Item = MgfSpectrum;
    type Error = anyhow::Error;

    fn next(&mut self) -> Result<Option<Self::Item>> {

        let mut in_spectrum: bool = false;
        let mut title = String::new();
        let mut precursor_mz: f64 = 0.0;
        let mut precursor_charge: Option<i8> = None;
        let mut retention_time: Option<f64> = None;
        let mut mz_list: Vec<f64> = Vec::new();
        let mut intensity_list: Vec<f32> = Vec::new();

        loop {
            let mut line = String::new();
            if let Ok(num_bytes) = self.internal_reader.read_line(&mut line) {
                if num_bytes == 0 {
                    if in_spectrum {
                        bail!("reach EOF before END IONS (incomplete spectrum)".to_string());
                    }
                    return Ok(None);
                }

                line = line.as_mut_str().trim().to_string();
                if line.is_empty() {
                    continue
                }

                let first_char = line.chars().next().unwrap();

                if first_char == 'B' && line.starts_with("BEGIN IONS") {
                    in_spectrum = true;
                } else if first_char == 'E' && line.starts_with("END IONS") {

                    //in_spectrum = false;

                    return Ok(Some(MgfSpectrum::new(
                        title,
                        precursor_mz,
                        precursor_charge,
                        retention_time,
                        mz_list,
                        intensity_list
                    )));
                } else if in_spectrum {
                    // if line contains a peak
                    if first_char.is_numeric() {
                        let mut split = line.split_ascii_whitespace();

                        match split.next() {
                            Some(mz) => mz_list.push(fast_float::parse(mz)?),
                            None => bail!("m/z value is missing")
                        };

                        match split.next() {
                            Some(intens) => intensity_list.push(fast_float::parse(intens)?),
                            None => bail!("intensity value is missing")
                        };

                    } else {
                        if line.starts_with("TITLE=") {
                            title = line[6..].to_owned();
                        } else if line.starts_with("PEPMASS=") {
                            let prec_mz_str_opt = &line[8..].split_ascii_whitespace().next();
                            precursor_mz = fast_float::parse(prec_mz_str_opt.unwrap_or_else(|| "0.0"))?;
                        } else if line.starts_with("RTINSECONDS=") {
                            retention_time = Some(fast_float::parse(&line[12..])?);
                        } else if line.starts_with("CHARGE=") {
                            // Locate the value and its sign within the string
                            let value = &line[7..];
                            let mut chars = value.chars();
                            let charge_sign_idx = chars.position(|c| !c.is_numeric()).unwrap_or(value.len());

                            // Parse the charge value
                            let charge_str = &value[0..charge_sign_idx];
                            let mut charge = charge_str.parse()?;

                            // Parse the charge sign and update the value accordingly
                            let sign = chars.nth(charge_sign_idx).unwrap_or('+');
                            if sign == '-' {
                                charge *= -1;
                            }

                            precursor_charge = Some(charge);
                        }
                    }
                } // ends else if in_spectrum
            } // ends read_line
        } // ends loop
    }
}