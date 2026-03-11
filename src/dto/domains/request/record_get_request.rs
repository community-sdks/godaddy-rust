use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RecordGetRequest {
    pub domain: ParamValue,
    pub type_: ParamValue,
    pub name: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
    pub offset: Option<ParamValue>,
    pub limit: Option<ParamValue>,
}

impl RecordGetRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        name: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            type_: type_.into(),
            name: name.into(),
            x_shopper_id,
            offset,
            limit,
        }
    }
}
