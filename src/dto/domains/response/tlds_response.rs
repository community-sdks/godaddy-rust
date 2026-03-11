use serde_json::Value;

#[derive(Clone, Debug)]
pub struct TldsResponse {
    pub raw: Value,
}

impl TldsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
