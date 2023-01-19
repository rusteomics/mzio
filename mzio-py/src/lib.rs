use pyo3::prelude::*;

pub mod fasta;
pub mod mgf;

/// Python bindings for rusteomics_proteomics_io
#[pymodule]
fn rusteomics_proteomics_io_py(py: Python, m: &PyModule) -> PyResult<()> {
    register_fasta_module(py, m)?;
    register_mgf_module(py, m)?;
    Ok(())
}

/// Registers a submodule called `fasta`
/// 
/// # Arguments
/// 
/// `py` - Python
/// `parent_module` - Parent module of the fasta module
fn register_fasta_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "fasta")?;
    child_module.add_class::<fasta::entry::Entry>()?;
    child_module.add_class::<fasta::reader::Reader>()?;
    child_module.add_class::<fasta::writer::Writer>()?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}

/// Registers a submodule called `mgf`
/// 
/// # Arguments
/// 
/// `py` - Python
/// `parent_module` - Parent module of the mgf module
fn register_mgf_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "mgf")?;
    child_module.add_class::<mgf::spectrum::Spectrum>()?;
    // child_module.add_class::<mgf::reader::Reader>()?;
    // child_module.add_class::<mgf::writer::Writer>()?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
