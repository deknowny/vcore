use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use reqwest;

use crate::response::APIResponse;


#[pyclass(module = "vcore_ext.api")]
struct APIExecutor {
    #[pyo3(get)]
    token: String,

    client: reqwest::Client
}

#[pymethods]
impl APIExecutor {
    #[new]
    fn new(token: String) -> Self {
        APIExecutor {
            token,
            client: reqwest::Client::new()
        }
    }
    #[args(params = "**")]
    fn call(&self, method_name: &str, params: Option<&PyDict>) -> PyResult<APIResponse> {
        let used_params = HashMap::from([
            ("access_token", self.token.as_str()),
            ("v", "5.131")
        ]);
        if let Some(custom_params) = params {
            for pair in custom_params.items() {
                let (key, value): (&str, &str) = pair.extract()?;
                used_params.insert(key, value);
            }
        };
        let response = self.client.post(
            format!("https://api.vk.com/method/{}", method_name),
        )
        .form(&used_params)
        .send();
        // TODO
    }

}



#[pymodule]
fn api(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<APIExecutor>()?;
    Ok(())
}
