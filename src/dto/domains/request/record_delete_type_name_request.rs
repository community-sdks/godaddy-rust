use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RecordDeleteTypeNameRequest {
    pub domain: ParamValue,
    pub type_: ParamValue,
    pub name: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl RecordDeleteTypeNameRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        name: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            type_: type_.into(),
            name: name.into(),
            x_shopper_id,
        }
    }
}
