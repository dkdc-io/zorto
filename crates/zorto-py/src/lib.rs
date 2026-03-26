use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyfunction]
fn run_cli(argv: Vec<String>) -> PyResult<()> {
    zorto::run(argv.iter().map(|s| s.as_str()))
        .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))
}

#[pymodule]
mod core {
    use super::*;

    #[pymodule_init]
    fn module_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(run_cli, m)?)?;
        Ok(())
    }
}
