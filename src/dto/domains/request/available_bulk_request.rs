use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct AvailableBulkRequest {
    pub domains: ParamValue,
    pub check_type: Option<ParamValue>,
}

impl AvailableBulkRequest {
    pub fn new(domains: impl Into<ParamValue>, check_type: Option<ParamValue>) -> Self {
        Self {
            domains: domains.into(),
            check_type,
        }
    }
}
