use pyo3::prelude::*;

#[pyfunction]
fn version_major() -> PyResult<i32> {
    Ok(weaveheap_capi::weaveheap_version_major())
}

#[pymodule]
fn weaveheap_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version_major, m)?)?;
    Ok(())
}
