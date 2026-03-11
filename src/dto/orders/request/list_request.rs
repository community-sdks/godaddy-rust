use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub x_app_key: ParamValue,
    pub period_start: Option<ParamValue>,
    pub period_end: Option<ParamValue>,
    pub domain: Option<ParamValue>,
    pub product_group_id: Option<ParamValue>,
    pub payment_profile_id: Option<ParamValue>,
    pub parent_order_id: Option<ParamValue>,
    pub offset: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub sort: Option<ParamValue>,
    pub x_shopper_id: Option<ParamValue>,
}

impl ListRequest {
    pub fn new(
        x_app_key: impl Into<ParamValue>,
        period_start: Option<ParamValue>,
        period_end: Option<ParamValue>,
        domain: Option<ParamValue>,
        product_group_id: Option<ParamValue>,
        payment_profile_id: Option<ParamValue>,
        parent_order_id: Option<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
        sort: Option<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> Self {
        Self {
            x_app_key: x_app_key.into(),
            period_start,
            period_end,
            domain,
            product_group_id,
            payment_profile_id,
            parent_order_id,
            offset,
            limit,
            sort,
            x_shopper_id,
        }
    }
}
