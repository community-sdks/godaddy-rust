use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub x_app_key: ParamValue,
    pub x_shopper_id: Option<ParamValue>,
    pub x_market_id: Option<ParamValue>,
    pub product_group_keys: Option<ParamValue>,
    pub includes: Option<ParamValue>,
    pub offset: Option<ParamValue>,
    pub limit: Option<ParamValue>,
    pub sort: Option<ParamValue>,
}

impl ListRequest {
    pub fn new(
        x_app_key: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        x_market_id: Option<ParamValue>,
        product_group_keys: Option<ParamValue>,
        includes: Option<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
        sort: Option<ParamValue>,
    ) -> Self {
        Self {
            x_app_key: x_app_key.into(),
            x_shopper_id,
            x_market_id,
            product_group_keys,
            includes,
            offset,
            limit,
            sort,
        }
    }
}
