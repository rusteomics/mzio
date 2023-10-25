
// 3rd party imports
use serde::{Serialize, Deserialize};

// internal imports
use mzcore::ms::spectrum::SpectrumData;
use mzcore::ms::utils::mz_to_mass;

/// Spectrum representation for MGF files.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MgfSpectrumHeader {
    pub title: String,
    pub precursor_mz: f64,
    pub precursor_charge: Option<i8>,
    pub precursor_mass: Option<f64>, // Not in the header (for post-processing convenience)
    pub retention_time: Option<f64>,
}

impl MgfSpectrumHeader {
    /// Creates a new spectrum
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
    pub fn new(
        title: String,
        precursor_mz: f64,
        precursor_charge: Option<i8>,
        retention_time: Option<f64>
    ) -> Self {
        Self {
            title,
            precursor_mz,
            precursor_charge,
            precursor_mass: None,
            retention_time,
        }
    }

    /// Returns the spectrum title
    /// 
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Returns the precursor mass
    /// 
    pub fn get_precursor_mz(&self) -> f64 {
        self.precursor_mz
    }

    /// Returns precursor charge
    /// 
    pub fn get_precursor_charge(&self) -> Option<i8> {
        self.precursor_charge
    }

    /// Returns precursor mass
    ///
    pub fn get_precursor_mass(&self) -> Option<f64> {
        self.precursor_mass
    }

    /// Sets precursor mass
    ///
    pub fn set_precursor_mass(&mut self, precursor_mass: f64) -> &MgfSpectrumHeader {
        self.precursor_mass = Some(precursor_mass);
        self
    }

    /// Calculates and sets precursor mass
    ///
    pub fn calc_precursor_mass(&mut self) -> &MgfSpectrumHeader {
        self.precursor_mass = self.precursor_charge.map(|z| mz_to_mass(self.precursor_mz, z as i32));
        self
    }

    /// Returns retention time
    /// 
    pub fn get_retention_time(&self) -> Option<f64>  {
        self.retention_time
    }

}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MgfSpectrum {
    pub header: MgfSpectrumHeader,
    pub data: SpectrumData,
}

impl MgfSpectrum {
    /// Creates a new spectrum
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
    pub fn new(
        title: String,
        precursor_mz: f64,
        precursor_charge: Option<i8>,
        retention_time: Option<f64>,
        mz_list: Vec<f64>,
        intensity_list: Vec<f32>
    ) -> Self {

        let mgf_header =  MgfSpectrumHeader::new(title, precursor_mz, precursor_charge, retention_time);

        let data = SpectrumData {
            mz_list,
            intensity_list,
        };

        Self {
            header: mgf_header,
            data: data,
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

}


