use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateActionRetrieveRequest {
    pub certificate_id: ParamValue,
}

impl CertificateActionRetrieveRequest {
    pub fn new(certificate_id: impl Into<ParamValue>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
        }
    }
}
