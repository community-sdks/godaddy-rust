use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateCreateV2Request {
    pub subscription_certificate_create: ParamValue,
    pub x_market_id: Option<ParamValue>,
}

impl CertificateCreateV2Request {
    pub fn new(
        subscription_certificate_create: impl Into<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> Self {
        Self {
            subscription_certificate_create: subscription_certificate_create.into(),
            x_market_id,
        }
    }
}
