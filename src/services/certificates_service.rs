use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct CertificatesService {
    inner: AbstractService,
}

impl CertificatesService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn certificate_create(
        &self,
        certificate_create: impl Into<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_create = certificate_create.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates",
                Vec::new(),
                Vec::new(),
                vec![("X-Market-Id", x_market_id)],
                Some(certificate_create),
            )
            .await
    }

    pub async fn certificate_validate(
        &self,
        certificate_create: impl Into<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_create = certificate_create.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/validate",
                Vec::new(),
                Vec::new(),
                vec![("X-Market-Id", x_market_id)],
                Some(certificate_create),
            )
            .await
    }

    pub async fn certificate_get(&self, certificate_id: impl Into<ParamValue>) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v1/certificates/{certificateId}",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_action_retrieve(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v1/certificates/{certificateId}/actions",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_resend_email(
        &self,
        certificate_id: impl Into<ParamValue>,
        email_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let email_id = email_id.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/email/{emailId}/resend",
                vec![
                    ("certificateId", Some(certificate_id)),
                    ("emailId", Some(email_id)),
                ],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_alternate_email_address(
        &self,
        certificate_id: impl Into<ParamValue>,
        email_address: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let email_address = email_address.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/email/resend/{emailAddress}",
                vec![
                    ("certificateId", Some(certificate_id)),
                    ("emailAddress", Some(email_address)),
                ],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_resend_email_address(
        &self,
        certificate_id: impl Into<ParamValue>,
        email_id: impl Into<ParamValue>,
        email_address: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let email_id = email_id.into();
        let email_address = email_address.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/email/{emailId}/resend/{emailAddress}",
                vec![
                    ("certificateId", Some(certificate_id)),
                    ("emailId", Some(email_id)),
                    ("emailAddress", Some(email_address)),
                ],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_email_history(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v1/certificates/{certificateId}/email/history",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_callback_delete(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "DELETE",
                "/v1/certificates/{certificateId}/callback",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_callback_get(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v1/certificates/{certificateId}/callback",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_callback_replace(
        &self,
        certificate_id: impl Into<ParamValue>,
        callback_url: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let callback_url = callback_url.into();
        self.inner
            .call(
                "PUT",
                "/v1/certificates/{certificateId}/callback",
                vec![("certificateId", Some(certificate_id))],
                vec![("callbackUrl", Some(callback_url))],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_cancel(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/cancel",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_download(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v1/certificates/{certificateId}/download",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_reissue(
        &self,
        certificate_id: impl Into<ParamValue>,
        reissue_create: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let reissue_create = reissue_create.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/reissue",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                Some(reissue_create),
            )
            .await
    }

    pub async fn certificate_renew(
        &self,
        certificate_id: impl Into<ParamValue>,
        renew_create: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let renew_create = renew_create.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/renew",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                Some(renew_create),
            )
            .await
    }

    pub async fn certificate_revoke(
        &self,
        certificate_id: impl Into<ParamValue>,
        certificate_revoke: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        let certificate_revoke = certificate_revoke.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/revoke",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                Some(certificate_revoke),
            )
            .await
    }

    pub async fn certificate_siteseal_get(
        &self,
        certificate_id: impl Into<ParamValue>,
        theme: Option<ParamValue>,
        locale: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v1/certificates/{certificateId}/siteSeal",
                vec![("certificateId", Some(certificate_id))],
                vec![("theme", theme), ("locale", locale)],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_verifydomaincontrol(
        &self,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "POST",
                "/v1/certificates/{certificateId}/verifyDomainControl",
                vec![("certificateId", Some(certificate_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_get_entitlement(
        &self,
        entitlement_id: impl Into<ParamValue>,
        latest: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let entitlement_id = entitlement_id.into();
        self.inner
            .call(
                "GET",
                "/v2/certificates",
                Vec::new(),
                vec![("entitlementId", Some(entitlement_id)), ("latest", latest)],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn certificate_create_v2(
        &self,
        subscription_certificate_create: impl Into<ParamValue>,
        x_market_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let subscription_certificate_create = subscription_certificate_create.into();
        self.inner
            .call(
                "POST",
                "/v2/certificates",
                Vec::new(),
                Vec::new(),
                vec![("X-Market-Id", x_market_id)],
                Some(subscription_certificate_create),
            )
            .await
    }

    pub async fn certificate_download_entitlement(
        &self,
        entitlement_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let entitlement_id = entitlement_id.into();
        self.inner
            .call(
                "GET",
                "/v2/certificates/download",
                Vec::new(),
                vec![("entitlementId", Some(entitlement_id))],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_customer_certificates_by_customer_id(
        &self,
        customer_id: impl Into<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        self.inner
            .call(
                "GET",
                "/v2/customers/{customerId}/certificates",
                vec![("customerId", Some(customer_id))],
                vec![("offset", offset), ("limit", limit)],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_certificate_detail_by_cert_identifier(
        &self,
        customer_id: impl Into<ParamValue>,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v2/customers/{customerId}/certificates/{certificateId}",
                vec![
                    ("customerId", Some(customer_id)),
                    ("certificateId", Some(certificate_id)),
                ],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_domain_information_by_certificate_id(
        &self,
        customer_id: impl Into<ParamValue>,
        certificate_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let certificate_id = certificate_id.into();
        self.inner
            .call(
                "GET",
                "/v2/customers/{customerId}/certificates/{certificateId}/domainVerifications",
                vec![
                    ("customerId", Some(customer_id)),
                    ("certificateId", Some(certificate_id)),
                ],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_domain_details_by_domain(
        &self,
        customer_id: impl Into<ParamValue>,
        certificate_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let certificate_id = certificate_id.into();
        let domain = domain.into();
        self.inner
            .call(
                "GET",
                "/v2/customers/{customerId}/certificates/{certificateId}/domainVerifications/{domain}",
                vec![
            ("customerId", Some(customer_id)),
            ("certificateId", Some(certificate_id)),
            ("domain", Some(domain)),
        ],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn get_acme_external_account_binding(
        &self,
        customer_id: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        self.inner
            .call(
                "GET",
                "/v2/customers/{customerId}/certificates/acme/externalAccountBinding",
                vec![("customerId", Some(customer_id))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn retrieve_ssl_by_domain_reseller(
        &self,
        page_size: Option<ParamValue>,
        page: Option<ParamValue>,
        domain: Option<ParamValue>,
        status: Option<ParamValue>,
        type_: Option<ParamValue>,
        validation: Option<ParamValue>,
    ) -> ApiResult<Value> {
        self.inner
            .call(
                "GET",
                "/v2/certificates/subscriptions/search",
                Vec::new(),
                vec![
                    ("pageSize", page_size),
                    ("page", page),
                    ("domain", domain),
                    ("status", status),
                    ("type", type_),
                    ("validation", validation),
                ],
                Vec::new(),
                None,
            )
            .await
    }

    pub async fn retrieve_ssl_by_domain_subscription_reseller(
        &self,
        guid: impl Into<ParamValue>,
        page_size: Option<ParamValue>,
        page: Option<ParamValue>,
        domain: Option<ParamValue>,
        status: Option<ParamValue>,
        type_: Option<ParamValue>,
        validation: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let guid = guid.into();
        self.inner
            .call(
                "GET",
                "/v2/certificates/subscription/{guid}",
                vec![("guid", Some(guid))],
                vec![
                    ("pageSize", page_size),
                    ("page", page),
                    ("domain", domain),
                    ("status", status),
                    ("type", type_),
                    ("validation", validation),
                ],
                Vec::new(),
                None,
            )
            .await
    }
}
