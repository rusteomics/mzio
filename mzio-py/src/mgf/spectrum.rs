// 3rd party imports
use pyo3::prelude::*;
use rusteomics_proteomics_io::mgf::spectrum::Spectrum as BaseSpectrum;


/// Wrapper for the rust implementation spectrum
/// 
#[pyclass]
pub struct Spectrum {
    base_spectrum: BaseSpectrum
}

#[pymethods]
impl Spectrum {
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
    fn new(title: String, precursor_mz: f64, precursor_charge: Option<i8>,
        retention_time: Option<f64>, mz_list: Vec<f64>, intensity_list: Vec<f32>) -> Self {
        Self {
            base_spectrum: BaseSpectrum::new(
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
    pub fn title(&self) -> PyResult<&str> {
        Ok(&self.base_spectrum.get_title())
    }

    /// Returns the precursor mz
    ///
    #[getter]
    pub fn precursor_mz(&self) -> PyResult<f64> {
        Ok(self.base_spectrum.get_precursor_mz())
    }

    /// Returns the precursor charge
    ///
    #[getter]
    pub fn precursor_charge(&self) -> PyResult<Option<i8>> {
        Ok(*self.base_spectrum.get_precursor_charge())
    }

    /// Returns the retention time
    ///
    #[getter]
    pub fn retention_time(&self) -> PyResult<Option<f64>> {
        Ok(*self.base_spectrum.get_retention_time())
    }

    /// Returns the mzs
    ///
    #[getter]
    pub fn mzs(&self) -> PyResult<Vec<f64>> {
        Ok(self.base_spectrum.get_mz_list().to_vec())
    }

    /// Returns the intensities
    ///
    #[getter]
    pub fn intensities(&self) -> PyResult<Vec<f32>> {
        Ok(self.base_spectrum.get_intensity_list().to_vec())
    }
}


impl From<BaseSpectrum> for Spectrum {
    /// Convert entry from the Rust implementation to the python wrapper.
    /// 
    /// # Arguments
    /// 
    /// * `base_spectrum` - Spectrum from rust implementation
    fn from(base_spectrum: BaseSpectrum) -> Self {
        Self {
            base_spectrum
        }
    }
}

impl<'a> Into<&'a BaseSpectrum> for &'a Spectrum {
    fn into(self) -> &'a BaseSpectrum {
        &self.base_spectrum
    }
}
