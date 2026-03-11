use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateCallbackReplaceRequest {
    pub certificate_id: ParamValue,
    pub callback_url: ParamValue,
}

impl CertificateCallbackReplaceRequest {
    pub fn new(certificate_id: impl Into<ParamValue>, callback_url: impl Into<ParamValue>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            callback_url: callback_url.into(),
        }
    }
}
