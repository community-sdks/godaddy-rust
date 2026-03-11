use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct AddExpiryListingsRequest {
    pub expiry_listings: ParamValue,
}

impl AddExpiryListingsRequest {
    pub fn new(expiry_listings: impl Into<ParamValue>) -> Self {
        Self {
            expiry_listings: expiry_listings.into(),
        }
    }
}
