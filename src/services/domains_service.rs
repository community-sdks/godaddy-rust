use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::error::ApiResult;

use super::abstract_service::AbstractService;

#[derive(Clone)]
pub struct DomainsService {
    inner: AbstractService,
}

impl DomainsService {
    pub const BASE_URL: &'static str = "https://api.ote-godaddy.com";

    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            inner: AbstractService::new(client, Self::BASE_URL),
        }
    }

    pub async fn list(
        &self,
        request: crate::dto::domains::request::ListRequest,
    ) -> ApiResult<crate::dto::domains::response::ListResponse> {
        let x_shopper_id = request.x_shopper_id;
        let statuses = request.statuses;
        let status_groups = request.status_groups;
        let limit = request.limit;
        let marker = request.marker;
        let includes = request.includes;
        let modified_date = request.modified_date;
        self.inner
            .call(
                "GET",
                "/v1/domains",
                Vec::new(),
                vec![
                    ("statuses", statuses),
                    ("statusGroups", status_groups),
                    ("limit", limit),
                    ("marker", marker),
                    ("includes", includes),
                    ("modifiedDate", modified_date),
                ],
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::ListResponse::from_value)
    }

    pub async fn get_agreement(
        &self,
        request: crate::dto::domains::request::GetAgreementRequest,
    ) -> ApiResult<crate::dto::domains::response::GetAgreementResponse> {
        let tlds = request.tlds;
        let privacy = request.privacy;
        let x_market_id = request.x_market_id;
        let for_transfer = request.for_transfer;
        self.inner
            .call(
                "GET",
                "/v1/domains/agreements",
                Vec::new(),
                vec![
                    ("tlds", Some(tlds)),
                    ("privacy", Some(privacy)),
                    ("forTransfer", for_transfer),
                ],
                vec![("X-Market-Id", x_market_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::GetAgreementResponse::from_value)
    }

    pub async fn available(
        &self,
        request: crate::dto::domains::request::AvailableRequest,
    ) -> ApiResult<crate::dto::domains::response::AvailableResponse> {
        let domain = request.domain;
        let check_type = request.check_type;
        let for_transfer = request.for_transfer;
        self.inner
            .call(
                "GET",
                "/v1/domains/available",
                Vec::new(),
                vec![
                    ("domain", Some(domain)),
                    ("checkType", check_type),
                    ("forTransfer", for_transfer),
                ],
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::domains::response::AvailableResponse::from_value)
    }

    pub async fn available_bulk(
        &self,
        request: crate::dto::domains::request::AvailableBulkRequest,
    ) -> ApiResult<crate::dto::domains::response::AvailableBulkResponse> {
        let domains = request.domains;
        let check_type = request.check_type;
        self.inner
            .call(
                "POST",
                "/v1/domains/available",
                Vec::new(),
                vec![("checkType", check_type)],
                Vec::new(),
                Some(domains),
            )
            .await
            .map(crate::dto::domains::response::AvailableBulkResponse::from_value)
    }

    pub async fn contacts_validate(
        &self,
        request: crate::dto::domains::request::ContactsValidateRequest,
    ) -> ApiResult<crate::dto::domains::response::ContactsValidateResponse> {
        let body = request.body;
        let x_private_label_id = request.x_private_label_id;
        let market_id = request.market_id;
        self.inner
            .call(
                "POST",
                "/v1/domains/contacts/validate",
                Vec::new(),
                vec![("marketId", market_id)],
                vec![("X-Private-Label-Id", x_private_label_id)],
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::ContactsValidateResponse::from_value)
    }

    pub async fn purchase(
        &self,
        request: crate::dto::domains::request::PurchaseRequest,
    ) -> ApiResult<crate::dto::domains::response::PurchaseResponse> {
        let body = request.body;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "POST",
                "/v1/domains/purchase",
                Vec::new(),
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::PurchaseResponse::from_value)
    }

    pub async fn schema(
        &self,
        request: crate::dto::domains::request::SchemaRequest,
    ) -> ApiResult<crate::dto::domains::response::SchemaResponse> {
        let tld = request.tld;
        self.inner
            .call(
                "GET",
                "/v1/domains/purchase/schema/{tld}",
                vec![("tld", Some(tld))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::domains::response::SchemaResponse::from_value)
    }

    pub async fn validate(
        &self,
        request: crate::dto::domains::request::ValidateRequest,
    ) -> ApiResult<crate::dto::domains::response::ValidateResponse> {
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v1/domains/purchase/validate",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::ValidateResponse::from_value)
    }

    pub async fn suggest(
        &self,
        request: crate::dto::domains::request::SuggestRequest,
    ) -> ApiResult<crate::dto::domains::response::SuggestResponse> {
        let x_shopper_id = request.x_shopper_id;
        let query = request.query;
        let country = request.country;
        let city = request.city;
        let sources = request.sources;
        let tlds = request.tlds;
        let length_max = request.length_max;
        let length_min = request.length_min;
        let limit = request.limit;
        let wait_ms = request.wait_ms;
        self.inner
            .call(
                "GET",
                "/v1/domains/suggest",
                Vec::new(),
                vec![
                    ("query", query),
                    ("country", country),
                    ("city", city),
                    ("sources", sources),
                    ("tlds", tlds),
                    ("lengthMax", length_max),
                    ("lengthMin", length_min),
                    ("limit", limit),
                    ("waitMs", wait_ms),
                ],
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::SuggestResponse::from_value)
    }

    pub async fn tlds(
        &self,
        _request: crate::dto::domains::request::TldsRequest,
    ) -> ApiResult<crate::dto::domains::response::TldsResponse> {
        self.inner
            .call(
                "GET",
                "/v1/domains/tlds",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::domains::response::TldsResponse::from_value)
    }

    pub async fn cancel(
        &self,
        request: crate::dto::domains::request::CancelRequest,
    ) -> ApiResult<crate::dto::domains::response::CancelResponse> {
        let domain = request.domain;
        self.inner
            .call(
                "DELETE",
                "/v1/domains/{domain}",
                vec![("domain", Some(domain))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::domains::response::CancelResponse::from_value)
    }

    pub async fn get(
        &self,
        request: crate::dto::domains::request::GetRequest,
    ) -> ApiResult<crate::dto::domains::response::GetResponse> {
        let domain = request.domain;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "GET",
                "/v1/domains/{domain}",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::GetResponse::from_value)
    }

    pub async fn update(
        &self,
        request: crate::dto::domains::request::UpdateRequest,
    ) -> ApiResult<crate::dto::domains::response::UpdateResponse> {
        let domain = request.domain;
        let body = request.body;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PATCH",
                "/v1/domains/{domain}",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::UpdateResponse::from_value)
    }

    pub async fn update_contacts(
        &self,
        request: crate::dto::domains::request::UpdateContactsRequest,
    ) -> ApiResult<crate::dto::domains::response::UpdateContactsResponse> {
        let domain = request.domain;
        let contacts = request.contacts;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PATCH",
                "/v1/domains/{domain}/contacts",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(contacts),
            )
            .await
            .map(crate::dto::domains::response::UpdateContactsResponse::from_value)
    }

    pub async fn cancel_privacy(
        &self,
        request: crate::dto::domains::request::CancelPrivacyRequest,
    ) -> ApiResult<crate::dto::domains::response::CancelPrivacyResponse> {
        let domain = request.domain;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "DELETE",
                "/v1/domains/{domain}/privacy",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::CancelPrivacyResponse::from_value)
    }

    pub async fn purchase_privacy(
        &self,
        request: crate::dto::domains::request::PurchasePrivacyRequest,
    ) -> ApiResult<crate::dto::domains::response::PurchasePrivacyResponse> {
        let domain = request.domain;
        let body = request.body;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "POST",
                "/v1/domains/{domain}/privacy/purchase",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::PurchasePrivacyResponse::from_value)
    }

    pub async fn record_add(
        &self,
        request: crate::dto::domains::request::RecordAddRequest,
    ) -> ApiResult<crate::dto::domains::response::RecordAddResponse> {
        let domain = request.domain;
        let records = request.records;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PATCH",
                "/v1/domains/{domain}/records",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(records),
            )
            .await
            .map(crate::dto::domains::response::RecordAddResponse::from_value)
    }

    pub async fn record_replace(
        &self,
        request: crate::dto::domains::request::RecordReplaceRequest,
    ) -> ApiResult<crate::dto::domains::response::RecordReplaceResponse> {
        let domain = request.domain;
        let records = request.records;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PUT",
                "/v1/domains/{domain}/records",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(records),
            )
            .await
            .map(crate::dto::domains::response::RecordReplaceResponse::from_value)
    }

    pub async fn record_get(
        &self,
        request: crate::dto::domains::request::RecordGetRequest,
    ) -> ApiResult<crate::dto::domains::response::RecordGetResponse> {
        let domain = request.domain;
        let type_ = request.type_;
        let name = request.name;
        let x_shopper_id = request.x_shopper_id;
        let offset = request.offset;
        let limit = request.limit;
        self.inner
            .call(
                "GET",
                "/v1/domains/{domain}/records/{type}/{name}",
                vec![
                    ("domain", Some(domain)),
                    ("type", Some(type_)),
                    ("name", Some(name)),
                ],
                vec![("offset", offset), ("limit", limit)],
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::RecordGetResponse::from_value)
    }

    pub async fn record_replace_type_name(
        &self,
        request: crate::dto::domains::request::RecordReplaceTypeNameRequest,
    ) -> ApiResult<crate::dto::domains::response::RecordReplaceTypeNameResponse> {
        let domain = request.domain;
        let type_ = request.type_;
        let name = request.name;
        let records = request.records;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PUT",
                "/v1/domains/{domain}/records/{type}/{name}",
                vec![
                    ("domain", Some(domain)),
                    ("type", Some(type_)),
                    ("name", Some(name)),
                ],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(records),
            )
            .await
            .map(crate::dto::domains::response::RecordReplaceTypeNameResponse::from_value)
    }

    pub async fn record_delete_type_name(
        &self,
        request: crate::dto::domains::request::RecordDeleteTypeNameRequest,
    ) -> ApiResult<crate::dto::domains::response::RecordDeleteTypeNameResponse> {
        let domain = request.domain;
        let type_ = request.type_;
        let name = request.name;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "DELETE",
                "/v1/domains/{domain}/records/{type}/{name}",
                vec![
                    ("domain", Some(domain)),
                    ("type", Some(type_)),
                    ("name", Some(name)),
                ],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::RecordDeleteTypeNameResponse::from_value)
    }

    pub async fn record_replace_type(
        &self,
        request: crate::dto::domains::request::RecordReplaceTypeRequest,
    ) -> ApiResult<crate::dto::domains::response::RecordReplaceTypeResponse> {
        let domain = request.domain;
        let type_ = request.type_;
        let records = request.records;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "PUT",
                "/v1/domains/{domain}/records/{type}",
                vec![("domain", Some(domain)), ("type", Some(type_))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(records),
            )
            .await
            .map(crate::dto::domains::response::RecordReplaceTypeResponse::from_value)
    }

    pub async fn renew(
        &self,
        request: crate::dto::domains::request::RenewRequest,
    ) -> ApiResult<crate::dto::domains::response::RenewResponse> {
        let domain = request.domain;
        let x_shopper_id = request.x_shopper_id;
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v1/domains/{domain}/renew",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                body,
            )
            .await
            .map(crate::dto::domains::response::RenewResponse::from_value)
    }

    pub async fn transfer_in(
        &self,
        request: crate::dto::domains::request::TransferInRequest,
    ) -> ApiResult<crate::dto::domains::response::TransferInResponse> {
        let domain = request.domain;
        let body = request.body;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "POST",
                "/v1/domains/{domain}/transfer",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::TransferInResponse::from_value)
    }

    pub async fn verify_email(
        &self,
        request: crate::dto::domains::request::VerifyEmailRequest,
    ) -> ApiResult<crate::dto::domains::response::VerifyEmailResponse> {
        let domain = request.domain;
        let x_shopper_id = request.x_shopper_id;
        self.inner
            .call(
                "POST",
                "/v1/domains/{domain}/verifyRegistrantEmail",
                vec![("domain", Some(domain))],
                Vec::new(),
                vec![("X-Shopper-Id", x_shopper_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::VerifyEmailResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_domain(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainResponse>
    {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        let includes = request.includes;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/{domain}",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        vec![("includes", includes)],
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainResponse::from_value)
    }

    pub async fn delete_v2_customers_customer_id_domains_domain_change_of_registrant(
        &self,
        request: crate::dto::domains::request::DeleteV2CustomersCustomerIdDomainsDomainChangeOfRegistrantRequest,
    ) -> ApiResult<crate::dto::domains::response::DeleteV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "DELETE",
        "/v2/customers/{customerId}/domains/{domain}/changeOfRegistrant",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::DeleteV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_domain_change_of_registrant(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainChangeOfRegistrantRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/{domain}/changeOfRegistrant",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainChangeOfRegistrantResponse::from_value)
    }

    pub async fn patch_v2_customers_customer_id_domains_domain_dnssec_records(
        &self,
        request: crate::dto::domains::request::PatchV2CustomersCustomerIdDomainsDomainDnssecRecordsRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PatchV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "PATCH",
        "/v2/customers/{customerId}/domains/{domain}/dnssecRecords",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PatchV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse::from_value)
    }

    pub async fn delete_v2_customers_customer_id_domains_domain_dnssec_records(
        &self,
        request: crate::dto::domains::request::DeleteV2CustomersCustomerIdDomainsDomainDnssecRecordsRequest,
    ) -> ApiResult<crate::dto::domains::response::DeleteV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "DELETE",
        "/v2/customers/{customerId}/domains/{domain}/dnssecRecords",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::DeleteV2CustomersCustomerIdDomainsDomainDnssecRecordsResponse::from_value)
    }

    pub async fn put_v2_customers_customer_id_domains_domain_name_servers(
        &self,
        request: crate::dto::domains::request::PutV2CustomersCustomerIdDomainsDomainNameServersRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PutV2CustomersCustomerIdDomainsDomainNameServersResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "PUT",
        "/v2/customers/{customerId}/domains/{domain}/nameServers",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PutV2CustomersCustomerIdDomainsDomainNameServersResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_domain_privacy_forwarding(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainPrivacyForwardingRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/{domain}/privacy/forwarding",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse::from_value)
    }

    pub async fn patch_v2_customers_customer_id_domains_domain_privacy_forwarding(
        &self,
        request: crate::dto::domains::request::PatchV2CustomersCustomerIdDomainsDomainPrivacyForwardingRequest,
    ) -> ApiResult<crate::dto::domains::response::PatchV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "PATCH",
        "/v2/customers/{customerId}/domains/{domain}/privacy/forwarding",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PatchV2CustomersCustomerIdDomainsDomainPrivacyForwardingResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_redeem(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainRedeemRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainRedeemResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        let body = request.body;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/redeem",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        body,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainRedeemResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_renew(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainRenewRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainRenewResponse>
    {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/renew",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainRenewResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transfer",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_domain_transfer(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainTransferRequest,
    ) -> ApiResult<
        crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainTransferResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/{domain}/transfer",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainTransferResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_validate(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferValidateRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferValidateResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transfer/validate",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferValidateResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_accept(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInAcceptRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInAcceptResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferInAccept",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInAcceptResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_cancel(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInCancelRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInCancelResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferInCancel",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInCancelResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_restart(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInRestartRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInRestartResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferInRestart",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInRestartResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_retry(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInRetryRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInRetryResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferInRetry",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferInRetryResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_out(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferOutRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferOutResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let registrar = request.registrar;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferOut",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        vec![("registrar", Some(registrar))],
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferOutResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_out_accept(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferOutAcceptRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferOutAcceptResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferOutAccept",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferOutAcceptResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_out_reject(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferOutRejectRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferOutRejectResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        let reason = request.reason;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/transferOutReject",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        vec![("reason", reason)],
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainTransferOutRejectResponse::from_value)
    }

    pub async fn domains_forwards_delete(
        &self,
        request: crate::dto::domains::request::DomainsForwardsDeleteRequest,
    ) -> ApiResult<crate::dto::domains::response::DomainsForwardsDeleteResponse> {
        let customer_id = request.customer_id;
        let fqdn = request.fqdn;
        self.inner
            .call(
                "DELETE",
                "/v2/customers/{customerId}/domains/forwards/{fqdn}",
                vec![("customerId", Some(customer_id)), ("fqdn", Some(fqdn))],
                Vec::new(),
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::domains::response::DomainsForwardsDeleteResponse::from_value)
    }

    pub async fn domains_forwards_get(
        &self,
        request: crate::dto::domains::request::DomainsForwardsGetRequest,
    ) -> ApiResult<crate::dto::domains::response::DomainsForwardsGetResponse> {
        let customer_id = request.customer_id;
        let fqdn = request.fqdn;
        let include_subs = request.include_subs;
        self.inner
            .call(
                "GET",
                "/v2/customers/{customerId}/domains/forwards/{fqdn}",
                vec![("customerId", Some(customer_id)), ("fqdn", Some(fqdn))],
                vec![("includeSubs", include_subs)],
                Vec::new(),
                None,
            )
            .await
            .map(crate::dto::domains::response::DomainsForwardsGetResponse::from_value)
    }

    pub async fn domains_forwards_put(
        &self,
        request: crate::dto::domains::request::DomainsForwardsPutRequest,
    ) -> ApiResult<crate::dto::domains::response::DomainsForwardsPutResponse> {
        let customer_id = request.customer_id;
        let fqdn = request.fqdn;
        let body = request.body;
        self.inner
            .call(
                "PUT",
                "/v2/customers/{customerId}/domains/forwards/{fqdn}",
                vec![("customerId", Some(customer_id)), ("fqdn", Some(fqdn))],
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::DomainsForwardsPutResponse::from_value)
    }

    pub async fn domains_forwards_post(
        &self,
        request: crate::dto::domains::request::DomainsForwardsPostRequest,
    ) -> ApiResult<crate::dto::domains::response::DomainsForwardsPostResponse> {
        let customer_id = request.customer_id;
        let fqdn = request.fqdn;
        let body = request.body;
        self.inner
            .call(
                "POST",
                "/v2/customers/{customerId}/domains/forwards/{fqdn}",
                vec![("customerId", Some(customer_id)), ("fqdn", Some(fqdn))],
                Vec::new(),
                Vec::new(),
                Some(body),
            )
            .await
            .map(crate::dto::domains::response::DomainsForwardsPostResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_domain_actions(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainActionsRequest,
    ) -> ApiResult<
        crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainActionsResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/{domain}/actions",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainActionsResponse::from_value)
    }

    pub async fn delete_v2_customers_customer_id_domains_domain_actions_type(
        &self,
        request: crate::dto::domains::request::DeleteV2CustomersCustomerIdDomainsDomainActionsTypeRequest,
    ) -> ApiResult<
        crate::dto::domains::response::DeleteV2CustomersCustomerIdDomainsDomainActionsTypeResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let type_ = request.type_;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "DELETE",
        "/v2/customers/{customerId}/domains/{domain}/actions/{type}",
        vec![
        ("customerId", Some(customer_id)),
        ("domain", Some(domain)),
        ("type", Some(type_)),
        ],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::DeleteV2CustomersCustomerIdDomainsDomainActionsTypeResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_domain_actions_type(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainActionsTypeRequest,
    ) -> ApiResult<
        crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainActionsTypeResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let type_ = request.type_;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/{domain}/actions/{type}",
        vec![
        ("customerId", Some(customer_id)),
        ("domain", Some(domain)),
        ("type", Some(type_)),
        ],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsDomainActionsTypeResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_notifications(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsNotificationsRequest,
    ) -> ApiResult<
        crate::dto::domains::response::GetV2CustomersCustomerIdDomainsNotificationsResponse,
    > {
        let customer_id = request.customer_id;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/notifications",
        vec![("customerId", Some(customer_id))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsNotificationsResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_notifications_opt_in(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsNotificationsOptInRequest,
    ) -> ApiResult<
        crate::dto::domains::response::GetV2CustomersCustomerIdDomainsNotificationsOptInResponse,
    > {
        let customer_id = request.customer_id;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/notifications/optIn",
        vec![("customerId", Some(customer_id))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsNotificationsOptInResponse::from_value)
    }

    pub async fn put_v2_customers_customer_id_domains_notifications_opt_in(
        &self,
        request: crate::dto::domains::request::PutV2CustomersCustomerIdDomainsNotificationsOptInRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PutV2CustomersCustomerIdDomainsNotificationsOptInResponse,
    > {
        let customer_id = request.customer_id;
        let types = request.types;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "PUT",
        "/v2/customers/{customerId}/domains/notifications/optIn",
        vec![("customerId", Some(customer_id))],
        vec![("types", Some(types))],
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PutV2CustomersCustomerIdDomainsNotificationsOptInResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_notifications_schemas_type(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeResponse>{
        let customer_id = request.customer_id;
        let type_ = request.type_;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/notifications/schemas/{type}",
        vec![("customerId", Some(customer_id)), ("type", Some(type_))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeResponse>{
        let customer_id = request.customer_id;
        let notification_id = request.notification_id;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/notifications/{notificationId}/acknowledge",
        vec![
        ("customerId", Some(customer_id)),
        ("notificationId", Some(notification_id)),
        ],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_register(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsRegisterRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsRegisterResponse>
    {
        let customer_id = request.customer_id;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/register",
        vec![("customerId", Some(customer_id))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsRegisterResponse::from_value)
    }

    pub async fn get_v2_customers_customer_id_domains_register_schema_tld(
        &self,
        request: crate::dto::domains::request::GetV2CustomersCustomerIdDomainsRegisterSchemaTldRequest,
    ) -> ApiResult<
        crate::dto::domains::response::GetV2CustomersCustomerIdDomainsRegisterSchemaTldResponse,
    > {
        let customer_id = request.customer_id;
        let tld = request.tld;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/customers/{customerId}/domains/register/schema/{tld}",
        vec![("customerId", Some(customer_id)), ("tld", Some(tld))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2CustomersCustomerIdDomainsRegisterSchemaTldResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_register_validate(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsRegisterValidateRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PostV2CustomersCustomerIdDomainsRegisterValidateResponse,
    > {
        let customer_id = request.customer_id;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/register/validate",
        vec![("customerId", Some(customer_id))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsRegisterValidateResponse::from_value)
    }

    pub async fn get_v2_domains_maintenances(
        &self,
        request: crate::dto::domains::request::GetV2DomainsMaintenancesRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2DomainsMaintenancesResponse> {
        let x_request_id = request.x_request_id;
        let status = request.status;
        let modified_at_after = request.modified_at_after;
        let starts_at_after = request.starts_at_after;
        let limit = request.limit;
        self.inner
            .call(
                "GET",
                "/v2/domains/maintenances",
                Vec::new(),
                vec![
                    ("status", status),
                    ("modifiedAtAfter", modified_at_after),
                    ("startsAtAfter", starts_at_after),
                    ("limit", limit),
                ],
                vec![("X-Request-Id", x_request_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::GetV2DomainsMaintenancesResponse::from_value)
    }

    pub async fn get_v2_domains_maintenances_maintenance_id(
        &self,
        request: crate::dto::domains::request::GetV2DomainsMaintenancesMaintenanceIdRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2DomainsMaintenancesMaintenanceIdResponse>
    {
        let maintenance_id = request.maintenance_id;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "GET",
        "/v2/domains/maintenances/{maintenanceId}",
        vec![("maintenanceId", Some(maintenance_id))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::GetV2DomainsMaintenancesMaintenanceIdResponse::from_value)
    }

    pub async fn get_v2_domains_usage_yyyymm(
        &self,
        request: crate::dto::domains::request::GetV2DomainsUsageYyyymmRequest,
    ) -> ApiResult<crate::dto::domains::response::GetV2DomainsUsageYyyymmResponse> {
        let yyyymm = request.yyyymm;
        let x_request_id = request.x_request_id;
        let includes = request.includes;
        self.inner
            .call(
                "GET",
                "/v2/domains/usage/{yyyymm}",
                vec![("yyyymm", Some(yyyymm))],
                vec![("includes", includes)],
                vec![("X-Request-Id", x_request_id)],
                None,
            )
            .await
            .map(crate::dto::domains::response::GetV2DomainsUsageYyyymmResponse::from_value)
    }

    pub async fn patch_v2_customers_customer_id_domains_domain_contacts(
        &self,
        request: crate::dto::domains::request::PatchV2CustomersCustomerIdDomainsDomainContactsRequest,
    ) -> ApiResult<
        crate::dto::domains::response::PatchV2CustomersCustomerIdDomainsDomainContactsResponse,
    > {
        let customer_id = request.customer_id;
        let domain = request.domain;
        let body = request.body;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "PATCH",
        "/v2/customers/{customerId}/domains/{domain}/contacts",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        Some(body),
        )
        .await
            .map(crate::dto::domains::response::PatchV2CustomersCustomerIdDomainsDomainContactsResponse::from_value)
    }

    pub async fn post_v2_customers_customer_id_domains_domain_regenerate_auth_code(
        &self,
        request: crate::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainRegenerateAuthCodeRequest,
    ) -> ApiResult<crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainRegenerateAuthCodeResponse>{
        let customer_id = request.customer_id;
        let domain = request.domain;
        let x_request_id = request.x_request_id;
        self.inner
        .call(
        "POST",
        "/v2/customers/{customerId}/domains/{domain}/regenerateAuthCode",
        vec![("customerId", Some(customer_id)), ("domain", Some(domain))],
        Vec::new(),
        vec![("X-Request-Id", x_request_id)],
        None,
        )
        .await
            .map(crate::dto::domains::response::PostV2CustomersCustomerIdDomainsDomainRegenerateAuthCodeResponse::from_value)
    }
}
