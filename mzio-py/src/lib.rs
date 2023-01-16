use pyo3::prelude::*;

pub mod fasta;

/// Python bindings for rusteomics_proteomics_io
#[pymodule]
fn rusteomics_proteomics_io_py(py: Python, m: &PyModule) -> PyResult<()> {
    register_fasta_module(py, m)?;
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
