use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateValidateRequest {
    pub certificate_create: ParamValue,
    pub x_market_id: Option<ParamValue>,
}

impl CertificateValidateRequest {
    pub fn new(certificate_create: impl Into<ParamValue>, x_market_id: Option<ParamValue>) -> Self {
        Self {
            certificate_create: certificate_create.into(),
            x_market_id,
        }
    }
}
