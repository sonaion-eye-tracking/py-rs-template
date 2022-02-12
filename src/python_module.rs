use pyo3::prelude::*;

#[pyfunction]
fn py_double(py: Python, x: PyObject) -> PyResult<i32> {
    let x = x.extract::<i32>(py)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("'x' couldn't be casted to i32: {}", e)))?;
    Ok(crate::double(x))
}

#[pymodule]
fn screen_recorder_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_double, m)?)?;
    Ok(())
}