use pyo3::prelude::*;

/// A simple function to greet from Rust.
#[pyfunction]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// The Python module definition.
#[pymodule]
fn pyrspack(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}

