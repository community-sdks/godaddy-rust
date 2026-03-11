use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateCallbackGetRequest {
    pub certificate_id: ParamValue,
}

impl CertificateCallbackGetRequest {
    pub fn new(certificate_id: impl Into<ParamValue>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
        }
    }
}
