use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateEmailHistoryRequest {
    pub certificate_id: ParamValue,
}

impl CertificateEmailHistoryRequest {
    pub fn new(certificate_id: impl Into<ParamValue>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
        }
    }
}
