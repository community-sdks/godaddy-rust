use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateRenewRequest {
    pub certificate_id: ParamValue,
    pub renew_create: ParamValue,
}

impl CertificateRenewRequest {
    pub fn new(certificate_id: impl Into<ParamValue>, renew_create: impl Into<ParamValue>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            renew_create: renew_create.into(),
        }
    }
}
