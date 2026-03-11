use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct AvailableRequest {
    pub domain: ParamValue,
    pub check_type: Option<ParamValue>,
    pub for_transfer: Option<ParamValue>,
}

impl AvailableRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        check_type: Option<ParamValue>,
        for_transfer: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            check_type,
            for_transfer,
        }
    }
}
