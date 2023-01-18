/// Spectrum representation for MGF files.
pub struct Spectrum {
    title: String,
    precursor_mz: f64,
    precursor_charge: Option<i8>,
    retention_time: Option<f64>,
    mz_list: Vec<f64>,
    intensity_list: Vec<f32>
}

impl Spectrum {
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
    pub fn new(title: String, precursor_mz: f64, precursor_charge: Option<i8>,
        retention_time: Option<f64>, mz_list: Vec<f64>, intensity_list: Vec<f32>) -> Self {
        Self {
            title,
            precursor_mz,
            precursor_charge,
            retention_time,
            mz_list,
            intensity_list,
        }
    }

    /// Returns the spectrum title
    /// 
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Returns the precursor mass
    /// 
    pub fn get_precursor_mz(&self) -> f64 {
        self.precursor_mz
    }

    /// Returns precursor charge
    /// 
    pub fn get_precursor_charge(&self) -> &Option<i8> {
        &self.precursor_charge
    }

    /// Returns retention time
    /// 
    pub fn get_retention_time(&self) -> &Option<f64>  {
        &self.retention_time
    }

    /// Returns M/Z list
    /// 
    pub fn get_mz_list(&self) -> &Vec<f64> {
        &self.mz_list
    }

    /// Returns intensity list
    /// 
    pub fn get_intensity_list(&self) -> &Vec<f32> {
        &self.intensity_list
    }

}