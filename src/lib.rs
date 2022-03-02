use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod api;
mod response;
mod collector;
use api::*;
use response::*;
use collector::*;


#[pymodule]
fn vcore_ext(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pymodule!(api))?;
    module.add_wrapped(wrap_pymodule!(response))?;
    module.add_wrapped(wrap_pymodule!(collector))?;
    Ok(())
}
