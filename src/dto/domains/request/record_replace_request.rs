use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct RecordReplaceRequest {
    pub domain: ParamValue,
    pub records: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
}

impl RecordReplaceRequest {
    pub fn new(
        domain: impl Into<ParamValue>,
        records: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            domain: domain.into(),
            records: records.into(),
            x_shopper_id,
        }
    }
}
