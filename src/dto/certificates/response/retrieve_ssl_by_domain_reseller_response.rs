use serde_json::Value;

#[derive(Clone, Debug)]
pub struct RetrieveSslByDomainResellerResponse {
    pub raw: Value,
}

impl RetrieveSslByDomainResellerResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
