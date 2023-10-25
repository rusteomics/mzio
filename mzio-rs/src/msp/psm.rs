
// 3rd party imports
use anyhow::*;
use serde::{Serialize, Deserialize};

// internal imports
use mzcore::ms::spectrum::SpectrumData;
use mzcore::ms::utils::mass_to_mz;

/// Non-parsed PSM representation for MSP files.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MspPsm {
    pub header: MspPsmHeader,
    pub data: SpectrumData,
    pub annotation_list: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MspPsmHeader {
    pub name: String,
    pub mw: f64,
    pub comment: String,
    pub num_peaks: usize,
}

/// Parsed PSM representation for MSP files.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MspParsedPsm {
    pub comment: String,
    pub sequence: String,
    pub mass: f64,
    pub mz: f64,
    pub charge: i8,
    pub peaks: Vec<MspPeak>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MspPeak {
    pub peak_index: usize,
    pub peak_mz: f64,
    pub peak_intensity: f32,
    pub unknown: Option<MspPeakUnknownData>,
    pub annotations: Vec<MspPeakAnnotation>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MspPeakUnknownData {
    pub numerator: usize, // FIXME: what is this?
    pub denominator: usize, // FIXME: what is this?
    pub value: f32, // FIXME: what is this?
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MspPeakAnnotation {
    Fragment(MspFragAnnotation),
    Sequence(MspSeqAnnotation)
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct MspFragAnnotation {
    pub ion_type: char,
    pub frag_index: u16,  // index of m/z value in the fragmentation table (starts at 0)
    pub neutral_loss: Option<f64>,
    pub charge: Option<i8>,
    pub mz_error: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MspSeqAnnotation {
    pub sequence: String,
    pub mz_error: Option<f64>,
}

impl MspPsmHeader {
    /// Creates a new MspPsmHeader
    /// 
    /// # Arguments
    ///
    /// * `name` - PSM name
    /// * `mw` - Precursor mass
    /// * `comment` - PSM description
    /// * `num_peaks` - Number of annotated peaks
    ///
    pub fn new(
        name: String,
        mw: f64,
        comment: String,
        num_peaks: usize,
    ) -> Self {
        Self {
            name,
            mw,
            comment,
            num_peaks,
        }
    }

    /// Returns the spectrum title
    /// 
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the precursor mass
    /// 
    pub fn get_mw(&self) -> f64 {
        self.mw
    }

    /// Returns precursor charge
    /// 
    pub fn get_comment(&self) -> &String {
        &self.comment
    }

    /// Returns precursor mass
    ///
    pub fn get_num_peaks(&self) -> usize {
        self.num_peaks
    }

}

impl MspPsm {
    /// Creates a new MspPsm
    ///
    /// # Arguments
    ///
    /// * `name` - PSM name
    /// * `mw` - Precursor mass
    /// * `comment` - PSM description
    /// * `num_peaks` - Number of annotated peaks
    /// * `mz_list` - M/Z list
    /// * `intensity_list` -  Intensity list
    ///
    pub fn new(
        name: String,
        mw: f64,
        comment: String,
        num_peaks: usize,
        mz_list: Vec<f64>,
        intensity_list: Vec<f32>,
        annotation_list: Vec<String>
    ) -> Self {

        let msp_header = MspPsmHeader::new(name, mw, comment, num_peaks);

        let data = SpectrumData {
            mz_list,
            intensity_list,
        };

        Self {
            header: msp_header,
            data: data,
            annotation_list
        }
    }

    /// Returns M/Z list
    ///
    pub fn get_mz_list(&self) -> &Vec<f64> {
        &self.data.mz_list
    }

    /// Returns intensity list
    ///
    pub fn get_intensity_list(&self) -> &Vec<f32> {
        &self.data.intensity_list
    }

    /// Returns annotation_ list
    ///
    pub fn get_annotation_list(&self) -> &Vec<String> {
        &self.annotation_list
    }

    /// Parses then returns an MspParsedPsm
    ///
    pub(crate) fn to_parsed_psm(&self) -> Result<MspParsedPsm> {
        let mut name_parts = self.header.name.split("/");
        let seq_opt = name_parts.next();
        let charge_str_opt = name_parts.next();

        if seq_opt.is_none() || charge_str_opt.is_none() {
            bail!("can't parse MSP PSM name because no '/' character found");
        }

        let charge = charge_str_opt.unwrap().parse::<i8>()?;

        let mut peaks = Vec::with_capacity(self.header.num_peaks);
        let mut peak_idx = 0;
        let tuple_iter = self.get_mz_list().iter().zip(self.get_intensity_list()).zip(self.get_annotation_list());
        for ((&mz, &intensity), &ref annotation) in tuple_iter {

            let cloned_annotation = annotation.clone();
            let mut annotation_parts = cloned_annotation.split_ascii_whitespace();
            let annotation_first_part_opt = annotation_parts.next();
            let annotation_second_part_opt = annotation_parts.next();
            let annotation_third_part_opt = annotation_parts.next();

            /*if annotation_first_part_opt.is_none() || annotation_second_part_opt.is_none() || annotation_third_part_opt.is_none() {
                bail!("can't parse supplementary peak data ({}) because two whitespace characters are expected", annotation);
            }*/

            // FIXME: we should also parse ?i and ?*
            let peak_annotations = if annotation.starts_with('?') {
                Vec::new()
            } else {
                let matched_annotations_str_iter = annotation_first_part_opt.unwrap().split(',');
                let mut matched_peak_annotations = Vec::new();

                for matched_annotation_str in matched_annotations_str_iter {

                    let mut matched_annotation_parts = matched_annotation_str.split('/');
                    let matched_annotation_part1_opt = matched_annotation_parts.next();
                    let matched_annotation_part2_opt = matched_annotation_parts.next();

                    let mut mz_error_opt = None;
                    let matched_annotation_before_slash = if matched_annotation_part1_opt.is_some() && matched_annotation_part2_opt.is_some() {
                        mz_error_opt = Some(fast_float::parse(matched_annotation_part2_opt.unwrap())?);
                        matched_annotation_part1_opt.unwrap()
                    } else {
                        matched_annotation_str
                    };

                    let contains_digit = matched_annotation_before_slash.chars().any(|c| c.is_ascii_digit());

                    if !contains_digit {
                        matched_peak_annotations.push(MspPeakAnnotation::Sequence(MspSeqAnnotation {
                            sequence: matched_annotation_before_slash.to_string(),
                            mz_error: mz_error_opt,
                        }))
                    } else {
                        let contains_dash = matched_annotation_before_slash.contains('-');
                        let contains_circumflex  = matched_annotation_before_slash.contains('^');

                        let mut frag_chars = matched_annotation_before_slash.chars();
                        let ion_type = frag_chars.next().ok_or(anyhow!("can't parse ion type"))?;
                        let frag_index_chars = frag_chars.by_ref().take_while(char::is_ascii_digit);
                        let frag_index_str = frag_index_chars.collect::<String>();

                        let mut nl_opt = None;
                        let mut charge_opt = None;

                        if contains_dash {
                            let nl_str = frag_chars.by_ref().take_while(char::is_ascii_digit).collect::<String>();
                            nl_opt = Some(nl_str.parse()?);

                            if contains_circumflex {
                                let charge_str = frag_chars.by_ref().take_while(char::is_ascii_digit).collect::<String>();
                                charge_opt = Some(charge_str.parse()?)
                            }
                        } else if contains_circumflex {
                            let charge_str = frag_chars.by_ref().take_while(char::is_ascii_digit).collect::<String>();
                            charge_opt = Some(charge_str.parse()?)
                        } else {
                            ensure!(frag_chars.next().is_none(), "can't parse peak annotation '{}'", matched_annotation_before_slash);
                        }

                        matched_peak_annotations.push(MspPeakAnnotation::Fragment(MspFragAnnotation {
                            ion_type: ion_type,
                            frag_index: frag_index_str.parse()?,
                            neutral_loss: nl_opt,
                            charge: charge_opt,
                            mz_error: mz_error_opt,
                        }));
                    }
                }

                matched_peak_annotations
            };

            let msp_peak = self._create_msp_peak(
                peak_idx,
                mz,
                intensity,
                peak_annotations,
                annotation_second_part_opt,
                annotation_third_part_opt
            )?;

            peaks.push(msp_peak);

            peak_idx += 1
        }

        Ok(MspParsedPsm {
            comment: self.header.comment.clone(),
            sequence: seq_opt.unwrap().to_string(),
            mz: mass_to_mz(self.header.mw, charge as i32),
            charge: charge,
            mass: self.header.mw,
            peaks: peaks
        })
    }

    fn _create_msp_peak(
        &self,
        peak_idx: usize,
        mz: f64,
        intensity: f32,
        peak_annotations: Vec<MspPeakAnnotation>,
        annotation_part2: Option<&str>,
        annotation_part3: Option<&str>
    ) -> Result<MspPeak> {

        let unknown_data_opt = if annotation_part2.is_none() || annotation_part3.is_none() {
            //bail!("can't parse unmatched peak data ({}) because no '/' character found", annotation);
            None
        } else {
            let mut unmatched_peak_info_parts = annotation_part2.unwrap().split('/');
            let numerator_opt = unmatched_peak_info_parts.next();
            let denominator_opt = unmatched_peak_info_parts.next();

            Some(MspPeakUnknownData {
                numerator: numerator_opt.unwrap().parse::<usize>()?,
                denominator: denominator_opt.unwrap().parse::<usize>()?,
                value: fast_float::parse(annotation_part3.unwrap())?
            })
        };

        let msp_peak = MspPeak {
            peak_index: peak_idx,
            peak_mz: mz,
            peak_intensity: intensity,
            unknown: unknown_data_opt,
            annotations: peak_annotations
        };

        Ok(msp_peak)
    }

}

impl MspParsedPsm {
    /// Creates a new MspParsedPsm
    ///
    /// # Arguments
    ///
    /// * `comment` - PSM description
    /// * `sequence` - PSM sequence
    /// * `mass` - Precursor mass
    /// * `mz` - Precursor mass
    /// * `peaks` - Annotated peaks
    ///
    pub fn new(
        comment: String,
        sequence: String,
        mass: f64,
        mz: f64,
        charge: i8,
        peaks: Vec<MspPeak>
    ) -> Self {
        Self {
            comment,
            sequence,
            mz,
            charge,
            mass,
            peaks
        }
    }

    /// Converts this MspParsedPsm to an MspPsm
    ///
    pub fn to_psm(&self) -> Result<MspPsm> {

        let annotation_list = self.peaks.iter().map(|p| {
            let mut annotation_parts = Vec::with_capacity(3);

            if p.annotations.is_empty() {
                annotation_parts.push("?".to_string());
            } else {
                let matched_peaks_as_str = p.annotations.iter().map(|pa| {
                    let mut matched_peaks_buff = Vec::new();
                    match pa {
                        // Example: "y4-17^2/-0.02"
                        MspPeakAnnotation::Fragment(frag_annot) => {
                            matched_peaks_buff.push(format!("{}{}", frag_annot.ion_type, frag_annot.frag_index));
                            if frag_annot.neutral_loss.is_some() {
                                matched_peaks_buff.push(format!("-{}", frag_annot.neutral_loss.unwrap()));
                            }
                            if frag_annot.charge.is_some() {
                                matched_peaks_buff.push(format!("^{}", frag_annot.charge.unwrap()));
                            }
                            if frag_annot.mz_error.is_some() {
                                matched_peaks_buff.push(format!("/{}", frag_annot.mz_error.unwrap()));
                            }
                        },
                        // Example: "IKC/0.12"
                        MspPeakAnnotation::Sequence(seq_annot) => {
                            matched_peaks_buff.push(format!("{}", seq_annot.sequence));
                            if seq_annot.mz_error.is_some() {
                                matched_peaks_buff.push(format!("/{}", seq_annot.mz_error.unwrap()));
                            }
                        },
                    }

                    matched_peaks_buff.join("")

                }).collect::<Vec<String>>();

                annotation_parts.push(matched_peaks_as_str.join(","));
            }

            if p.unknown.is_some() {
                let data = p.unknown.unwrap();
                annotation_parts.push(format!("{}/{} {}", data.numerator, data.denominator, data.value));
            }

            annotation_parts.join(" ")
        }).collect();
        Ok(MspPsm::new(
            [self.sequence.clone(),self.charge.to_string()].join( "/"),
            self.mass,
            self.comment.clone(),
            self.peaks.len(),
            self.peaks.iter().map(|p| p.peak_mz).collect(),
            self.peaks.iter().map(|p| p.peak_intensity).collect(),
            annotation_list
      ))
    }

}
