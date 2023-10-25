
// std imports
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::{Result, bail, ensure};
use fallible_iterator::{FallibleIterator, IntoFallibleIterator};

// internal imports
use crate::msp::psm::MspPsm;

/// Reader for MSP
pub struct MspReader {
    msp_file_path: String,
    internal_reader: BufReader<File>
}

impl MspReader {
    /// Creates a new MspReader
    /// 
    /// # Arguments
    ///
    /// * `msp_file_path` - Path to MSP file
    /// 
    pub fn new(msp_file_path: &Path, buffer_size: usize) -> Result<Self> {
        Ok(Self {
            msp_file_path: msp_file_path.to_string_lossy().to_string(),
            internal_reader: BufReader::with_capacity(buffer_size, File::open(msp_file_path)?)
        })
    }
}

impl FallibleIterator for MspReader {
    type Item = MspPsm;
    type Error = anyhow::Error;

    fn next(&mut self) -> Result<Option<Self::Item>> {

        let mut found_psm_header = false;
        let mut eof_reached = false;
        let mut name = String::new();
        let mut comment = String::new();
        let mut mw: f64 = 0.0;
        let mut num_peaks: usize = 0;
        let mut mz_list: Vec<f64> = Vec::new();
        let mut intensity_list: Vec<f32> = Vec::new();
        let mut annotation_list: Vec<String> = Vec::new();

        loop {
            let mut line = String::new();

            let num_bytes = self.internal_reader.read_line(&mut line)?;

            if num_bytes == 0 {
                if eof_reached {
                    return Ok(None);
                }
                eof_reached = true;
            }

            if eof_reached || line.is_empty() || line.chars().all(|c| c.is_ascii_whitespace()) {
                if !found_psm_header {
                    return Ok(None);
                } else {
                    ensure!(
                    num_peaks ==  mz_list.len(),
                    "different number of expected peaks ({}) and parsed peaks ({})", num_peaks, mz_list.len()
                );

                // return last read PSM
                return Ok(Some(MspPsm::new(
                        name,
                        mw,
                        comment,
                        num_peaks,
                        mz_list,
                        intensity_list,
                        annotation_list
                    )));
                }
            }

            // Remove trailing \n
            line = line.as_mut_str().trim().to_string();

            let first_char = line.chars().next().unwrap();

            if first_char.is_ascii_digit() {
                let mut line_parts = line.split("\t");

                match line_parts.next() {
                    Some(mz_str) => mz_list.push(fast_float::parse(mz_str)?),
                    None => bail!("m/z value is missing")
                }

                match line_parts.next() {
                    Some(intensity_str) => intensity_list.push(fast_float::parse(intensity_str)?),
                    None => bail!("intensity value is missing")
                }

                match line_parts.next() {
                    Some(annotation_str) => {
                        let mut annotation_chars =  annotation_str.chars();

                        // Remove double quotes
                        if annotation_chars.next().unwrap_or('\0') != '"' || annotation_chars.next_back().unwrap_or('\0') != '"' {
                            bail!("invalid format of annotation string ({}), annotation should be surrounded by double quotes", annotation_str);
                        }

                        annotation_list.push(annotation_chars.collect::<String>())
                    }
                    None => bail!("annotation value is missing")
                }

            } else {

                let mut header_parts = line.split(": ");
                let header_name_opt = header_parts.next();
                let header_value_opt = header_parts.next();

                if header_name_opt.is_none() || header_value_opt.is_none() {
                    bail!("unexpected line in .msp file '{}', neither a header or peak line: {}", self.msp_file_path, line);
                }

                match header_name_opt.unwrap() {
                    "Name" => name = header_value_opt.unwrap().to_string(),
                    "MW" => mw = fast_float::parse(header_value_opt.unwrap())?,
                    "Comment" => comment = header_value_opt.unwrap().to_string(),
                    "Num peaks" => num_peaks = header_value_opt.unwrap().parse().unwrap_or(0),
                    _ => { },
                }

                found_psm_header = true;

            } // ends else first_char.is_ascii_digit()

        } // ends loop
    }
}