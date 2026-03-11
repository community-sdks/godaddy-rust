use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateGetEntitlementRequest {
    pub entitlement_id: ParamValue,
    pub latest: Option<ParamValue>,
}

impl CertificateGetEntitlementRequest {
    pub fn new(entitlement_id: impl Into<ParamValue>, latest: Option<ParamValue>) -> Self {
        Self {
            entitlement_id: entitlement_id.into(),
            latest,
        }
    }
}
