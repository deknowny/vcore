use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3_asyncio;
use reqwest;

use crate::response::APIResponse;



#[pyclass(module = "vcore_ext.api")]
struct APIExecutor {
    #[pyo3(get)]
    token: String,

    client: reqwest::Client
}


// Rust only methods
// idk lol how do it prettier, PRs are welcome
impl APIExecutor {
    fn cast_param_value(&self, value: &PyAny) -> String {
        match value.extract::<String>() {
            Ok(val) => val,
            Err(_) => match value.extract::<isize>() {
                Ok(val) => val.to_string(),
                Err(_) => match value.extract::<bool>() {
                    Ok(val) => val.to_string(),
                    Err(_) => match value.extract::<Vec<&PyAny>>() {
                        Ok(val) => {
                            let mut params_array = vec![];
                            for elem in val {
                                params_array.push(self.cast_param_value(elem));
                            }
                            params_array.join(",")
                        },
                        Err(_) => panic!("Invalid argument type {}", value),
                    },
                }
            }
        }
    }

    fn extract_params(&self, used_params: &mut HashMap<String, String>, py_params: Option<&PyDict>) {
        if let Some(custom_params) = py_params {
            for pair in custom_params.items() {
                // First and second unwrap never gonna fall
                let (key, value): (String, &PyAny) = pair.extract().unwrap();
                let value_exists: Option<&PyAny> = value.extract().unwrap();
                if value_exists.is_some() {
                    let suuported_value_view = self.cast_param_value(value);
                    used_params.insert(key, suuported_value_view);
                }
            }
        };
    }
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
        self.extract_params(&mut used_params, params);

        // println!("{:?}", used_params);
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
