use serde_json::Value;

#[derive(Clone, Debug)]
pub struct VerifyDnsRecordsResponse {
    pub raw: Value,
}

impl VerifyDnsRecordsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
