use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateAlternateEmailAddressRequest {
    pub certificate_id: ParamValue,
    pub email_address: ParamValue,
}

impl CertificateAlternateEmailAddressRequest {
    pub fn new(
        certificate_id: impl Into<ParamValue>,
        email_address: impl Into<ParamValue>,
    ) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            email_address: email_address.into(),
        }
    }
}
