use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateResendEmailRequest {
    pub certificate_id: ParamValue,
    pub email_id: ParamValue,
}

impl CertificateResendEmailRequest {
    pub fn new(certificate_id: impl Into<ParamValue>, email_id: impl Into<ParamValue>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            email_id: email_id.into(),
        }
    }
}
