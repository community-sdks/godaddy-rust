use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PatchV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse {
    pub raw: Value,
}

impl PatchV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
