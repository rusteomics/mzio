
// std imports
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::{Result, anyhow};

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

    /// Creates a new spectrum from string values.
    /// 
    /// # Arguments
    ///
    /// * `title` - Specturm title
    /// * `precursor_mz` -  Precursor mass
    /// * `precursor_charge` -  Precursor charge
    /// * `retention_time` -  Retention time
    /// * `mz_list` - M/Z list
    /// * `intensity_list` -  Intensity list
    /// 
    fn create_spectrum(title: String, precursor_mz: String, precursor_charge: Option<String>,
        retention_time: Option<String>, mz_list: Vec<String>, intensity_list: Vec<String>) -> Result<Spectrum> {
            Ok(Spectrum::new(
                title,
                fast_float::parse(&precursor_mz)?,
                if let Some(p_charge) = precursor_charge { Some(p_charge.parse::<i8>()?) } else { None },
                if let Some(r_time) = retention_time { Some(fast_float::parse(r_time)?) } else { None },
                mz_list.into_iter().map(|mz| fast_float::parse(mz)).collect::<Result<Vec<_>, _>>()?,
                intensity_list.into_iter().map(|intens| fast_float::parse(intens)).collect::<Result<Vec<_>, _>>()?
            ))
        }
}

impl Iterator for Reader {
    type Item = Result<Spectrum>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut in_spectrum: bool = false;
        let mut title = String::new();
        let mut precursor_mz: String = String::new();
        let mut precursor_charge: Option<String> = None;
        let mut retention_time: Option<String> = None;
        let mut mz_list: Vec<String> = Vec::new();
        let mut intensity_list: Vec<String> = Vec::new();

        loop {
            let mut line = String::new();
            if let Ok(num_bytes) = self.internal_reader.read_line(&mut line) {
                if num_bytes == 0 {
                    if in_spectrum {
                        return Some(Err(anyhow!("reach EOF before END IONS (incomplete spectrum)".to_string())));
                    }
                    return None;
                }
                line = line.as_mut_str().trim().to_string();
                if line.is_empty() {
                    continue
                }
                
                if line.chars().nth(0).unwrap().is_numeric() {
                    let mut split = line.split_ascii_whitespace();

                    match split.next() {
                        Some(mz) => mz_list.push(mz.to_owned()),
                        None => return Some(Err(anyhow!("mz value is missing")))
                    };

                    match split.next() {
                        Some(intens) => intensity_list.push(intens.to_owned()),
                        None => return Some(Err(anyhow!("intensity value is missing")))
                    };
                    
                } else if line.starts_with("TITLE=") {
                    title = line[6..].to_owned();
                } else if line.starts_with("PEPMASS=") {
                    precursor_mz = line[8..].to_owned();
                } else if line.starts_with("RTINSECONDS=") {
                    retention_time = Some(line[12..].to_owned());
                } else if line.starts_with("CHARGE=") {
                    precursor_charge = Some(line[7..].to_owned());
                } else if line == "BEGIN IONS" {
                    in_spectrum = true;
                } else if line == "END IONS" {
                    return Some(Self::create_spectrum(
                        title,
                        precursor_mz,
                        precursor_charge,
                        retention_time,
                        mz_list,
                        intensity_list
                    ));
                }
                
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::iter::zip;

    use super::*;

    const TEST_TITLE: &'static str = "SOME_GENERIC_4LPH4NUM3RIC_TILE";
    const TEST_PRECURSOR_MZ: &'static str = "824.836730957031";
    const TEST_PRECURSOR_CHARGE: &'static str = "2";
    const TEST_RETENTION_TIME: &'static str = "212.9232";
    const TEST_MZ_LIST: [&'static str; 16] = [
        "118.936477661133",
        "122.26781463623",
        "138.923324584961",
        "188.516448974609",
        "268.502807617188",
        "269.640869140625",
        "291.39013671875",
        "301.587707519531",
        "326.118194580078",
        "340.996948242188",
        "355.069671630859",
        "357.8935546875",
        "708.145690917969",
        "731.38818359375",
        "1201.05639648438",
        "1364.15832519531"
    ];
    const TEST_INTENSITY_LIST: [&'static str; 16] = [
        "429.616",
        "354.588",
        "369.316",
        "367.936",
        "408.742",
        "1.0035e+006",          // Scientific annotation ais allowed in MGFs, lets test it
        "405.47",
        "425.719",
        "429.315",
        "657.018",
        "2271.57",
        "477.306",
        "479.876",
        "348.131",
        "385.268",
        "385.311"
    ];

    const EXPECTED_TITLE: &'static str = "SOME_GENERIC_4LPH4NUM3RIC_TILE";
    const EXPECTED_PRECURSOR_MZ: f64 = 824.836730957031;
    const EXPECTED_PRECURSOR_CHARGE: i8 = 2;
    const EXPECTED_RETENTION_TIME: f64 = 212.9232;
    const EXPECTED_MZ_LIST: [f64; 16] = [
        118.936477661133,
        122.26781463623,
        138.923324584961,
        188.516448974609,
        268.502807617188,
        269.640869140625,
        291.39013671875,
        301.587707519531,
        326.118194580078,
        340.996948242188,
        355.069671630859,
        357.8935546875,
        708.145690917969,
        731.38818359375,
        1201.05639648438,
        1364.15832519531
    ];
    const EXPECTED_INTENSITY_LIST: [f32; 16] = [
        429.616,
        354.588,
        369.316,
        367.936,
        408.742,
        1003500.0,
        405.47,
        425.719,
        429.315,
        657.018,
        2271.57,
        477.306,
        479.876,
        348.131,
        385.268,
        385.311
    ];
    

    #[test]
    /// Tests the creation of a spectrum from string values
    ///
    fn test_entry_creation() {
        let spectrum = Reader::create_spectrum(
            TEST_TITLE.to_owned(), 
            TEST_PRECURSOR_MZ.to_owned(),
            Some(TEST_PRECURSOR_CHARGE.to_owned()),
            Some(TEST_RETENTION_TIME.to_owned()),
            TEST_MZ_LIST.into_iter().map(|mz| mz.to_owned()).collect(),
            TEST_INTENSITY_LIST.into_iter().map(|intens| intens.to_owned()).collect(),
        ).unwrap();


        assert_eq!(
            spectrum.get_title(),
            EXPECTED_TITLE.to_owned()
        );

        assert_eq!(
            spectrum.get_precursor_mz(),
            EXPECTED_PRECURSOR_MZ
        );

        assert_eq!(
            *spectrum.get_precursor_charge(),
            Some(EXPECTED_PRECURSOR_CHARGE)
        );

        assert_eq!(
            *spectrum.get_retention_time(),
            Some(EXPECTED_RETENTION_TIME)
        );

        assert_eq!(
            spectrum.get_mz_list().len(),
            EXPECTED_MZ_LIST.len()
        );

        for (mz, expected_mz) in zip(spectrum.get_mz_list(), EXPECTED_MZ_LIST) {
            assert_eq!(
                *mz,
                expected_mz
            );
        }

        assert_eq!(
            spectrum.get_intensity_list().len(),
            EXPECTED_INTENSITY_LIST.len()
        );

        for (intens, expected_intens) in zip(spectrum.get_intensity_list(), EXPECTED_INTENSITY_LIST) {
            assert_eq!(
                *intens,
                expected_intens
            );
        }
    }
}
