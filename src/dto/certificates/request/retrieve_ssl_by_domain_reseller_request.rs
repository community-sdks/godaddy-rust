use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RetrieveSslByDomainResellerRequest {
    pub page_size: Option<ParamValue>,
    pub page: Option<ParamValue>,
    pub domain: Option<ParamValue>,
    pub status: Option<ParamValue>,
    pub type_: Option<ParamValue>,
    pub validation: Option<ParamValue>,
}

impl RetrieveSslByDomainResellerRequest {
    pub fn new(
        page_size: Option<ParamValue>,
        page: Option<ParamValue>,
        domain: Option<ParamValue>,
        status: Option<ParamValue>,
        type_: Option<ParamValue>,
        validation: Option<ParamValue>,
    ) -> Self {
        Self {
            page_size,
            page,
            domain,
            status,
            type_,
            validation,
        }
    }
}
