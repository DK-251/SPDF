//! PyO3 bindings exposing SPDF core to Python (FastAPI backend).
//! Placeholder — full implementation in Week 3 (Day 11).

use pyo3::prelude::*;

#[pyfunction]
fn version() -> &'static str {
    "0.1.0"
}

#[pymodule]
fn spdf_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
