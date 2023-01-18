
// std imports
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::{Result, bail};
use fallible_iterator::FallibleIterator;

// internal imports
use crate::mgf::spectrum::Spectrum;

/// Reader for MGF
pub struct Reader {
    internal_reader: BufReader<File>
}

impl Reader {
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

impl FallibleIterator for Reader {
    type Item = Spectrum;
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

                if line.chars().nth(0).unwrap().is_numeric() {
                    let mut split = line.split_ascii_whitespace();

                    match split.next() {
                        Some(mz) => mz_list.push(fast_float::parse(mz)?),
                        None => bail!("mz value is missing")
                    };

                    match split.next() {
                        Some(intens) => intensity_list.push(fast_float::parse(intens)?),
                        None => bail!("intensity value is missing")
                    };

                } else if line.starts_with("TITLE=") {
                    title = line[6..].to_owned();
                } else if line.starts_with("PEPMASS=") {
                    precursor_mz = fast_float::parse(&line[8..])?;
                } else if line.starts_with("RTINSECONDS=") {
                    retention_time = Some(fast_float::parse(&line[12..])?);
                } else if line.starts_with("CHARGE=") {
                    precursor_charge = Some(line[7..].parse()?);
                } else if line == "BEGIN IONS" {
                    in_spectrum = true;
                } else if line == "END IONS" {
                    return Ok(Some(Spectrum::new(
                        title,
                        precursor_mz,
                        precursor_charge,
                        retention_time,
                        mz_list,
                        intensity_list
                    )));
                }
            }
        }
    }
}
