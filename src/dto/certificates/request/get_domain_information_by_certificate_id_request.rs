use crate::api_client::ParamValue;

#[derive(Clone, Debug)]
pub struct GetDomainInformationByCertificateIdRequest {
    pub customer_id: ParamValue,
    pub certificate_id: ParamValue,
}

impl GetDomainInformationByCertificateIdRequest {
    pub fn new(customer_id: impl Into<ParamValue>, certificate_id: impl Into<ParamValue>) -> Self {
        Self {
            customer_id: customer_id.into(),
            certificate_id: certificate_id.into(),
        }
    }
}
