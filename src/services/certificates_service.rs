use std::sync::Arc;

use crate::api_client::ApiClient;
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
        request: crate::dto::certificates::request::CertificateCreateRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateCreateResponse> {
        let certificate_create = request.certificate_create;
        let x_market_id = request.x_market_id;
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
            .map(crate::dto::certificates::response::CertificateCreateResponse::from_value)
    }

    pub async fn certificate_validate(
        &self,
        request: crate::dto::certificates::request::CertificateValidateRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateValidateResponse> {
        let certificate_create = request.certificate_create;
        let x_market_id = request.x_market_id;
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
            .map(crate::dto::certificates::response::CertificateValidateResponse::from_value)
    }

    pub async fn certificate_get(
        &self,
        request: crate::dto::certificates::request::CertificateGetRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateGetResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateGetResponse::from_value)
    }

    pub async fn certificate_action_retrieve(
        &self,
        request: crate::dto::certificates::request::CertificateActionRetrieveRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateActionRetrieveResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateActionRetrieveResponse::from_value)
    }

    pub async fn certificate_resend_email(
        &self,
        request: crate::dto::certificates::request::CertificateResendEmailRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateResendEmailResponse> {
        let certificate_id = request.certificate_id;
        let email_id = request.email_id;
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
            .map(crate::dto::certificates::response::CertificateResendEmailResponse::from_value)
    }

    pub async fn certificate_alternate_email_address(
        &self,
        request: crate::dto::certificates::request::CertificateAlternateEmailAddressRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateAlternateEmailAddressResponse>
    {
        let certificate_id = request.certificate_id;
        let email_address = request.email_address;
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
            .map(crate::dto::certificates::response::CertificateAlternateEmailAddressResponse::from_value)
    }

    pub async fn certificate_resend_email_address(
        &self,
        request: crate::dto::certificates::request::CertificateResendEmailAddressRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateResendEmailAddressResponse> {
        let certificate_id = request.certificate_id;
        let email_id = request.email_id;
        let email_address = request.email_address;
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
            .map(crate::dto::certificates::response::CertificateResendEmailAddressResponse::from_value)
    }

    pub async fn certificate_email_history(
        &self,
        request: crate::dto::certificates::request::CertificateEmailHistoryRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateEmailHistoryResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateEmailHistoryResponse::from_value)
    }

    pub async fn certificate_callback_delete(
        &self,
        request: crate::dto::certificates::request::CertificateCallbackDeleteRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateCallbackDeleteResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateCallbackDeleteResponse::from_value)
    }

    pub async fn certificate_callback_get(
        &self,
        request: crate::dto::certificates::request::CertificateCallbackGetRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateCallbackGetResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateCallbackGetResponse::from_value)
    }

    pub async fn certificate_callback_replace(
        &self,
        request: crate::dto::certificates::request::CertificateCallbackReplaceRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateCallbackReplaceResponse> {
        let certificate_id = request.certificate_id;
        let callback_url = request.callback_url;
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
            .map(crate::dto::certificates::response::CertificateCallbackReplaceResponse::from_value)
    }

    pub async fn certificate_cancel(
        &self,
        request: crate::dto::certificates::request::CertificateCancelRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateCancelResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateCancelResponse::from_value)
    }

    pub async fn certificate_download(
        &self,
        request: crate::dto::certificates::request::CertificateDownloadRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateDownloadResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateDownloadResponse::from_value)
    }

    pub async fn certificate_reissue(
        &self,
        request: crate::dto::certificates::request::CertificateReissueRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateReissueResponse> {
        let certificate_id = request.certificate_id;
        let reissue_create = request.reissue_create;
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
            .map(crate::dto::certificates::response::CertificateReissueResponse::from_value)
    }

    pub async fn certificate_renew(
        &self,
        request: crate::dto::certificates::request::CertificateRenewRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateRenewResponse> {
        let certificate_id = request.certificate_id;
        let renew_create = request.renew_create;
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
            .map(crate::dto::certificates::response::CertificateRenewResponse::from_value)
    }

    pub async fn certificate_revoke(
        &self,
        request: crate::dto::certificates::request::CertificateRevokeRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateRevokeResponse> {
        let certificate_id = request.certificate_id;
        let certificate_revoke = request.certificate_revoke;
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
            .map(crate::dto::certificates::response::CertificateRevokeResponse::from_value)
    }

    pub async fn certificate_siteseal_get(
        &self,
        request: crate::dto::certificates::request::CertificateSitesealGetRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateSitesealGetResponse> {
        let certificate_id = request.certificate_id;
        let theme = request.theme;
        let locale = request.locale;
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
            .map(crate::dto::certificates::response::CertificateSitesealGetResponse::from_value)
    }

    pub async fn certificate_verifydomaincontrol(
        &self,
        request: crate::dto::certificates::request::CertificateVerifydomaincontrolRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateVerifydomaincontrolResponse> {
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::CertificateVerifydomaincontrolResponse::from_value)
    }

    pub async fn certificate_get_entitlement(
        &self,
        request: crate::dto::certificates::request::CertificateGetEntitlementRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateGetEntitlementResponse> {
        let entitlement_id = request.entitlement_id;
        let latest = request.latest;
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
            .map(crate::dto::certificates::response::CertificateGetEntitlementResponse::from_value)
    }

    pub async fn certificate_create_v2(
        &self,
        request: crate::dto::certificates::request::CertificateCreateV2Request,
    ) -> ApiResult<crate::dto::certificates::response::CertificateCreateV2Response> {
        let subscription_certificate_create = request.subscription_certificate_create;
        let x_market_id = request.x_market_id;
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
            .map(crate::dto::certificates::response::CertificateCreateV2Response::from_value)
    }

    pub async fn certificate_download_entitlement(
        &self,
        request: crate::dto::certificates::request::CertificateDownloadEntitlementRequest,
    ) -> ApiResult<crate::dto::certificates::response::CertificateDownloadEntitlementResponse> {
        let entitlement_id = request.entitlement_id;
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
            .map(crate::dto::certificates::response::CertificateDownloadEntitlementResponse::from_value)
    }

    pub async fn get_customer_certificates_by_customer_id(
        &self,
        request: crate::dto::certificates::request::GetCustomerCertificatesByCustomerIdRequest,
    ) -> ApiResult<crate::dto::certificates::response::GetCustomerCertificatesByCustomerIdResponse>
    {
        let customer_id = request.customer_id;
        let offset = request.offset;
        let limit = request.limit;
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
            .map(crate::dto::certificates::response::GetCustomerCertificatesByCustomerIdResponse::from_value)
    }

    pub async fn get_certificate_detail_by_cert_identifier(
        &self,
        request: crate::dto::certificates::request::GetCertificateDetailByCertIdentifierRequest,
    ) -> ApiResult<crate::dto::certificates::response::GetCertificateDetailByCertIdentifierResponse>
    {
        let customer_id = request.customer_id;
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::GetCertificateDetailByCertIdentifierResponse::from_value)
    }

    pub async fn get_domain_information_by_certificate_id(
        &self,
        request: crate::dto::certificates::request::GetDomainInformationByCertificateIdRequest,
    ) -> ApiResult<crate::dto::certificates::response::GetDomainInformationByCertificateIdResponse>
    {
        let customer_id = request.customer_id;
        let certificate_id = request.certificate_id;
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
            .map(crate::dto::certificates::response::GetDomainInformationByCertificateIdResponse::from_value)
    }

    pub async fn get_domain_details_by_domain(
        &self,
        request: crate::dto::certificates::request::GetDomainDetailsByDomainRequest,
    ) -> ApiResult<crate::dto::certificates::response::GetDomainDetailsByDomainResponse> {
        let customer_id = request.customer_id;
        let certificate_id = request.certificate_id;
        let domain = request.domain;
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
            .map(crate::dto::certificates::response::GetDomainDetailsByDomainResponse::from_value)
    }

    pub async fn get_acme_external_account_binding(
        &self,
        request: crate::dto::certificates::request::GetAcmeExternalAccountBindingRequest,
    ) -> ApiResult<crate::dto::certificates::response::GetAcmeExternalAccountBindingResponse> {
        let customer_id = request.customer_id;
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
            .map(crate::dto::certificates::response::GetAcmeExternalAccountBindingResponse::from_value)
    }

    pub async fn retrieve_ssl_by_domain_reseller(
        &self,
        request: crate::dto::certificates::request::RetrieveSslByDomainResellerRequest,
    ) -> ApiResult<crate::dto::certificates::response::RetrieveSslByDomainResellerResponse> {
        let page_size = request.page_size;
        let page = request.page;
        let domain = request.domain;
        let status = request.status;
        let type_ = request.type_;
        let validation = request.validation;
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
            .map(
                crate::dto::certificates::response::RetrieveSslByDomainResellerResponse::from_value,
            )
    }

    pub async fn retrieve_ssl_by_domain_subscription_reseller(
        &self,
        request: crate::dto::certificates::request::RetrieveSslByDomainSubscriptionResellerRequest,
    ) -> ApiResult<
        crate::dto::certificates::response::RetrieveSslByDomainSubscriptionResellerResponse,
    > {
        let guid = request.guid;
        let page_size = request.page_size;
        let page = request.page;
        let domain = request.domain;
        let status = request.status;
        let type_ = request.type_;
        let validation = request.validation;
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
            .map(crate::dto::certificates::response::RetrieveSslByDomainSubscriptionResellerResponse::from_value)
    }
}
