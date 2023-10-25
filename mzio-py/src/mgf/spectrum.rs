// 3rd party imports
use pyo3::prelude::*;
use mzio::mgf::spectrum::MgfSpectrum as BaseMgfSpectrum;


/// Wrapper for the rust implementation spectrum
/// 
#[pyclass]
#[derive(Clone, Debug)]
pub struct MgfSpectrum {
    base_spectrum: BaseMgfSpectrum
}

#[pymethods]
impl MgfSpectrum {
    /// Python constructor
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
    #[new]
    fn new(
        mz_list: Vec<f64>,
        intensity_list: Vec<f32>,
        title: String,
        precursor_mz: f64,
        precursor_charge: Option<i8>,
        retention_time: Option<f64>
    ) -> Self {
        Self {
            base_spectrum: BaseMgfSpectrum::new(
                title,
                precursor_mz,
                precursor_charge,
                retention_time,
                mz_list,
                intensity_list,
            )
        }
    }

    /// Returns the spectrum title
    ///
    #[getter]
    pub fn title(&self) -> String {
        self.base_spectrum.header.get_title().to_owned()
    }

    /// Returns the precursor mz
    ///
    #[getter]
    pub fn precursor_mz(&self) -> f64 {
        self.base_spectrum.header.get_precursor_mz()
    }

    /// Returns the precursor charge
    ///
    #[getter]
    pub fn precursor_charge(&self) -> Option<i8> {
        self.base_spectrum.header.get_precursor_charge()
    }

    /// Returns the retention time
    ///
    #[getter]
    pub fn retention_time(&self) -> Option<f64> {
        self.base_spectrum.header.get_retention_time()
    }

    /// Returns the mzs
    ///
    #[getter]
    pub fn mzs(&self) -> Vec<f64> {
        self.base_spectrum.get_mz_list().to_owned()
    }

    /// Returns the intensities
    ///
    #[getter]
    pub fn intensities(&self) -> Vec<f32> {
        self.base_spectrum.get_intensity_list().to_owned()
    }
}


impl From<BaseMgfSpectrum> for MgfSpectrum {
    /// Convert entry from the Rust implementation to the python wrapper.
    /// 
    /// # Arguments
    /// 
    /// * `base_spectrum` - Spectrum from rust implementation
    fn from(base_spectrum: BaseMgfSpectrum) -> Self {
        Self {
            base_spectrum
        }
    }
}

impl<'a> Into<&'a BaseMgfSpectrum> for &'a MgfSpectrum {
    fn into(self) -> &'a BaseMgfSpectrum {
        &self.base_spectrum
    }
}
