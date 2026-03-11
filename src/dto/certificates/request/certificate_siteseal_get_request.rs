use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CertificateSitesealGetRequest {
    pub certificate_id: ParamValue,
    pub theme: Option<ParamValue>,
    pub locale: Option<ParamValue>,
}

impl CertificateSitesealGetRequest {
    pub fn new(
        certificate_id: impl Into<ParamValue>,
        theme: Option<ParamValue>,
        locale: Option<ParamValue>,
    ) -> Self {
        Self {
            certificate_id: certificate_id.into(),
            theme,
            locale,
        }
    }
}
