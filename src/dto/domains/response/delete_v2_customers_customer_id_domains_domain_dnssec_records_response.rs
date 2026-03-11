use serde_json::Value;

#[derive(Clone, Debug)]
pub struct DeleteV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse {
    pub raw: Value,
}

impl DeleteV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse {
    pub fn from_value(raw: Value) -> Self {
        Self { raw }
    }
}
