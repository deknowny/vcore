use pyo3::prelude::*;
use serde_json;
use std::collections::HashMap;



enum ResponseState {
    Success,
    Error
}


pub struct SerdeValueProxy<'a> {
    serde_value: &'a serde_json::Value
}

impl IntoPy<PyObject> for SerdeValueProxy<'_> {
    fn into_py(self, py: Python) -> PyObject {
        match self.serde_value {
            serde_json::Value::Bool(val) => val.to_object(py),
            serde_json::Value::String(val) => val.to_object(py),
            serde_json::Value::Null => Option::<isize>::None.to_object(py),
            serde_json::Value::Number(val) => {
                if val.is_f64() { val.as_f64().to_object(py) }
                else { val.as_i64().to_object(py) }
            },
            serde_json::Value::Array(val) => {
                let mut result_vec = vec![];
                for elem in val {
                    let wrapped_elem = SerdeValueProxy { serde_value: &elem };
                    result_vec.push(wrapped_elem.into_py(py));
                }
                result_vec.to_object(py)
            },
            serde_json::Value::Object(val) => {
                let mut result_hashmap = HashMap::new();
                for (key, elem) in val {
                    let wrapped_elem = SerdeValueProxy { serde_value: &elem };
                    result_hashmap.insert(key, wrapped_elem.into_py(py));
                }
                result_hashmap.to_object(py)
            }
        }
    }
}


#[pyclass(module = "vcore_ext.response")]
pub struct APIResponse {
    content: serde_json::Value,
    state: ResponseState
}

impl APIResponse {
    pub fn new(content: serde_json::Value) -> Self {
        // println!("{}", content);
        let state = match content.get("response") {
            Some(_) => ResponseState::Success,
            None => ResponseState::Error
        };
        APIResponse {
            content,
            state
        }
    }
}

#[pymethods]
impl APIResponse {
    pub fn get(&self, fields_chain: &PyAny) -> PyResult<SerdeValueProxy> {
        let first_access_key = match self.state {
            ResponseState::Success => "response",
            ResponseState::Error => "error"
        };
        let mut current_value = &self.content[first_access_key];
        let chain_is_str = fields_chain.extract::<&str>();
        let chain: Vec<&PyAny>;
        match chain_is_str {
            Ok(_) => { chain = vec![fields_chain]; },
            Err(_) => { chain = fields_chain.extract::<Vec<&PyAny>>()?; }
        }
        for step in chain {
            let convertion_to_str = step.extract::<&str>();
            match convertion_to_str {
                PyResult::Ok(key) => current_value = &current_value[key],
                _ => {
                    let convertion_to_int: usize = step.extract()?;
                    current_value = &current_value[convertion_to_int];
                }
            }
        }
        let proxied_serde = SerdeValueProxy {
            serde_value: current_value
        };
        Ok(proxied_serde)
    }
}


#[pymodule]
fn response(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<APIResponse>()?;
    Ok(())
}
