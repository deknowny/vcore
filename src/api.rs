use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_asyncio;
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
    fn new(py: Python, token: String) -> PyResult<Self> {
        pyo3_asyncio::try_init(py)?;
        pyo3_asyncio::tokio::init_multi_thread_once();
        Ok(
            APIExecutor {
                token,
                client: reqwest::Client::new()
            }
        )
    }
    #[args(params = "**")]
    fn call(&self, py: Python, method_name: String, params: Option<&PyDict>) -> PyResult<PyObject> {
        let client = self.client.clone();
        let token = self.token.clone();
        let mut used_params = HashMap::from([
            ("access_token".to_owned(), token.clone()),
            ("v".to_owned(), "5.131".to_owned())
        ]);

        if let Some(custom_params) = params {
            for pair in custom_params.items() {
                let (key, value): (String, String) = pair.extract()?;
                used_params.insert(key, value);
            }
        };
        pyo3_asyncio::tokio::into_coroutine(py, async move {
            let response = client.post(
                format!("https://api.vk.com/method/{}", method_name),
            )
            .form(&used_params)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();

            Ok(Python::with_gil(|py| APIResponse::new(response).into_py(py)))
        })
    }

}



#[pymodule]
fn api(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<APIExecutor>()?;
    Ok(())
}
