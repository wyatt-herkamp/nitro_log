use log::kv::{Source, ToKey};

pub mod default_structure_dump;

#[derive(Debug, Clone)]
pub enum Variable {
    PathVariable(String, Vec<String>),
    SinglePartVariable(String),
}

impl Variable {
    /// Locates the Value
    pub(crate) fn get_value<'kvs>(&'kvs self, source: &'kvs dyn Source) -> String {
        match self {
            Variable::PathVariable(key, path) => {
                let option = source.get(key.to_key());
                if let Some(value) = option {
                    let value = serde_json::to_value(value);
                    match value.as_ref() {
                        Ok(ok) => {
                            let mut value = ok;
                            for inner_key in path {
                                if let Some(inner_key) = value.get(inner_key) {
                                    value = inner_key
                                } else {
                                    return undefined();
                                }
                            }
                            value.as_str().map(|v| v.to_owned()).unwrap_or_else(undefined)
                        }
                        Err(error) => {
                            format!("(Unable to parse via serde_json: {})", error)
                        }
                    }
                } else {
                    undefined()
                }
            }
            Variable::SinglePartVariable(variable) => {
                source.get(variable.to_key()).map(|v| v.to_string()).unwrap_or_else(undefined)
            }
        }
    }
}

fn undefined() -> String {
    "{undefined}".to_owned()
}