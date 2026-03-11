use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateRevokeRequest {
    pub certificate_id: ParamValue,
    pub certificate_revoke: ParamValue,
}

impl CertificateRevokeRequest {
    pub fn new(
        certificate_id: impl Into<ParamValue>,
        certificate_revoke: impl Into<ParamValue>,
    ) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            certificate_revoke: certificate_revoke.into(),
        }
    }
}
