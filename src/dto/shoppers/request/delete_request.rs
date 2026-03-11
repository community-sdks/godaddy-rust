use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    pub shopper_id: ParamValue,
    pub audit_client_ip: ParamValue,
}

impl DeleteRequest {
    pub fn new(shopper_id: impl Into<ParamValue>, audit_client_ip: impl Into<ParamValue>) -> Self {
        Self {
            shopper_id: shopper_id.into(),
            audit_client_ip: audit_client_ip.into(),
        }
    }
}
