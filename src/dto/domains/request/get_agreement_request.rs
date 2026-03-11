use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetAgreementRequest {
    pub tlds: ParamValue,
    pub privacy: ParamValue,
    pub x_market_id: Option<ParamValue>,
    pub for_transfer: Option<ParamValue>,
}

impl GetAgreementRequest {
    pub fn new(
        tlds: impl Into<ParamValue>,
        privacy: impl Into<ParamValue>,
        x_market_id: Option<ParamValue>,
        for_transfer: Option<ParamValue>,
    ) -> Self {
        Self {
            tlds: tlds.into(),
            privacy: privacy.into(),
            x_market_id,
            for_transfer,
        }
    }
}
