use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SearchAnsNameResponse {
    pub raw: Value,
}

impl SearchAnsNameResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
