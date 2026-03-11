use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateDownloadEntitlementRequest {
    pub entitlement_id: ParamValue,
}

impl CertificateDownloadEntitlementRequest {
    pub fn new(entitlement_id: impl Into<ParamValue>) -> Self {
        Self {
            entitlement_id: entitlement_id.into(),
        }
    }
}
