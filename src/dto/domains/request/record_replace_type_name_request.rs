use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RecordReplaceTypeNameRequest {
    pub domain: ParamValue,
    pub type_: ParamValue,
    pub name: ParamValue,
    pub records: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl RecordReplaceTypeNameRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        name: impl Into<ParamValue>,
        records: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            type_: type_.into(),
            name: name.into(),
            records: records.into(),
            x_shopper_id,
        }
    }
}
