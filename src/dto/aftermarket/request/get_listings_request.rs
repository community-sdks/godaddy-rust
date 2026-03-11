use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetListingsRequest {
    pub customer_id: ParamValue,
    pub domains: Option<ParamValue>,
    pub listing_status: Option<ParamValue>,
    pub transfer_before: Option<ParamValue>,
    pub transfer_after: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub offset: Option<ParamValue>,
}

impl GetListingsRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        domains: Option<ParamValue>,
        listing_status: Option<ParamValue>,
        transfer_before: Option<ParamValue>,
        transfer_after: Option<ParamValue>,
        limit: Option<ParamValue>,
        offset: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            domains,
            listing_status,
            transfer_before,
            transfer_after,
            limit,
            offset,
        }
    }
}
