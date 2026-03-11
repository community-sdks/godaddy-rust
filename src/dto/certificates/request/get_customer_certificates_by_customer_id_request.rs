use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetCustomerCertificatesByCustomerIdRequest {
    pub customer_id: ParamValue,
    pub offset: Option<ParamValue>,
    pub limit: Option<ParamValue>,
}

impl GetCustomerCertificatesByCustomerIdRequest {
    pub fn new(
        customer_id: impl Into<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> Self {
        Self {
            customer_id: customer_id.into(),
            offset,
            limit,
        }
    }
}
