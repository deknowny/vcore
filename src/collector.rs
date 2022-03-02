use pyo3::prelude::*;



#[pyclass]
struct HandlersCollector {
    handlers: Vec<PyObject>
}

#[pymethods]
impl HandlersCollector {
    #[new]
    fn new() -> Self {
        HandlersCollector {
            handlers: vec![]
        }
    }

    fn add(&mut self, handler: PyObject) {
        Python::with_gil(|_| {
            self.handlers.push(handler)
        })
    }
}

#[pymodule]
fn collector(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<HandlersCollector>()?;
    Ok(())
}