use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ResolveAnsNameResponse {
    pub raw: Value,
}

impl ResolveAnsNameResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
