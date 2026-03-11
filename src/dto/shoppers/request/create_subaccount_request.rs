use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct CreateSubaccountRequest {
    pub subaccount: ParamValue,
}

impl CreateSubaccountRequest {
    pub fn new(subaccount: impl Into<ParamValue>) -> Self {
        Self {
            subaccount: subaccount.into(),
        }
    }
}
