use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateResendEmailAddressRequest {
    pub certificate_id: ParamValue,
    pub email_id: ParamValue,
    pub email_address: ParamValue,
}

impl CertificateResendEmailAddressRequest {
    pub fn new(
        certificate_id: impl Into<ParamValue>,
        email_id: impl Into<ParamValue>,
        email_address: impl Into<ParamValue>,
    ) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            email_id: email_id.into(),
            email_address: email_address.into(),
        }
    }
}
