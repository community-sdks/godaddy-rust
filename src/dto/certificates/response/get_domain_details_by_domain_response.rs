use serde_json::Value;

#[derive(Clone, Debug)]
pub struct GetDomainDetailsByDomainResponse {
    pub raw: Value,
}

impl GetDomainDetailsByDomainResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
