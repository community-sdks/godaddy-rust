use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateReissueRequest {
    pub certificate_id: ParamValue,
    pub reissue_create: ParamValue,
}

impl CertificateReissueRequest {
    pub fn new(
        certificate_id: impl Into<ParamValue>,
        reissue_create: impl Into<ParamValue>,
    ) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            reissue_create: reissue_create.into(),
        }
    }
}
