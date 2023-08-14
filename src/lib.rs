use pyo3::prelude::*;

/// Hello world function.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello, world!".to_string())
}

/// avrora package.
#[pymodule]
fn _avrora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
