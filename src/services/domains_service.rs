use std::sync::Arc;

use serde_json::Value;

use crate::api_client::{ApiClient, ParamValue};
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
        x_shopper_id: Option<ParamValue>,
        statuses: Option<ParamValue>,
        status_groups: Option<ParamValue>,
        limit: Option<ParamValue>,
        marker: Option<ParamValue>,
        includes: Option<ParamValue>,
        modified_date: Option<ParamValue>,
    ) -> ApiResult<Value> {
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
    }

    pub async fn get_agreement(
        &self,
        tlds: impl Into<ParamValue>,
        privacy: impl Into<ParamValue>,
        x_market_id: Option<ParamValue>,
        for_transfer: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let tlds = tlds.into();
        let privacy = privacy.into();
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
    }

    pub async fn available(
        &self,
        domain: impl Into<ParamValue>,
        check_type: Option<ParamValue>,
        for_transfer: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
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
    }

    pub async fn available_bulk(
        &self,
        domains: impl Into<ParamValue>,
        check_type: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domains = domains.into();
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
    }

    pub async fn contacts_validate(
        &self,
        body: impl Into<ParamValue>,
        x_private_label_id: Option<ParamValue>,
        market_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let body = body.into();
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
    }

    pub async fn purchase(
        &self,
        body: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let body = body.into();
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
    }

    pub async fn schema(&self, tld: impl Into<ParamValue>) -> ApiResult<Value> {
        let tld = tld.into();
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
    }

    pub async fn validate(&self, body: impl Into<ParamValue>) -> ApiResult<Value> {
        let body = body.into();
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
    }

    pub async fn suggest(
        &self,
        x_shopper_id: Option<ParamValue>,
        query: Option<ParamValue>,
        country: Option<ParamValue>,
        city: Option<ParamValue>,
        sources: Option<ParamValue>,
        tlds: Option<ParamValue>,
        length_max: Option<ParamValue>,
        length_min: Option<ParamValue>,
        limit: Option<ParamValue>,
        wait_ms: Option<ParamValue>,
    ) -> ApiResult<Value> {
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
    }

    pub async fn tlds(&self) -> ApiResult<Value> {
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
    }

    pub async fn cancel(&self, domain: impl Into<ParamValue>) -> ApiResult<Value> {
        let domain = domain.into();
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
    }

    pub async fn get(
        &self,
        domain: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
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
    }

    pub async fn update(
        &self,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn update_contacts(
        &self,
        domain: impl Into<ParamValue>,
        contacts: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let contacts = contacts.into();
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
    }

    pub async fn cancel_privacy(
        &self,
        domain: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
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
    }

    pub async fn purchase_privacy(
        &self,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn record_add(
        &self,
        domain: impl Into<ParamValue>,
        records: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let records = records.into();
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
    }

    pub async fn record_replace(
        &self,
        domain: impl Into<ParamValue>,
        records: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let records = records.into();
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
    }

    pub async fn record_get(
        &self,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        name: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        offset: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let type_ = type_.into();
        let name = name.into();
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
    }

    pub async fn record_replace_type_name(
        &self,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        name: impl Into<ParamValue>,
        records: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let type_ = type_.into();
        let name = name.into();
        let records = records.into();
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
    }

    pub async fn record_delete_type_name(
        &self,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        name: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let type_ = type_.into();
        let name = name.into();
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
    }

    pub async fn record_replace_type(
        &self,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        records: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let type_ = type_.into();
        let records = records.into();
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
    }

    pub async fn renew(
        &self,
        domain: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
        body: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
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
    }

    pub async fn transfer_in(
        &self,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn verify_email(
        &self,
        domain: impl Into<ParamValue>,
        x_shopper_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let domain = domain.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_domain(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
        includes: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn delete_v2_customers_customer_id_domains_domain_change_of_registrant(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_domain_change_of_registrant(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn patch_v2_customers_customer_id_domains_domain_dnssec_records(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn delete_v2_customers_customer_id_domains_domain_dnssec_records(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn put_v2_customers_customer_id_domains_domain_name_servers(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_domain_privacy_forwarding(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn patch_v2_customers_customer_id_domains_domain_privacy_forwarding(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_redeem(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
        body: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_renew(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_domain_transfer(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_validate(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_accept(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_cancel(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_restart(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_in_retry(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_out(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        registrar: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let registrar = registrar.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_out_accept(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_transfer_out_reject(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
        reason: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn domains_forwards_delete(
        &self,
        customer_id: impl Into<ParamValue>,
        fqdn: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let fqdn = fqdn.into();
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
    }

    pub async fn domains_forwards_get(
        &self,
        customer_id: impl Into<ParamValue>,
        fqdn: impl Into<ParamValue>,
        include_subs: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let fqdn = fqdn.into();
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
    }

    pub async fn domains_forwards_put(
        &self,
        customer_id: impl Into<ParamValue>,
        fqdn: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let fqdn = fqdn.into();
        let body = body.into();
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
    }

    pub async fn domains_forwards_post(
        &self,
        customer_id: impl Into<ParamValue>,
        fqdn: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let fqdn = fqdn.into();
        let body = body.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_domain_actions(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }

    pub async fn delete_v2_customers_customer_id_domains_domain_actions_type(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let type_ = type_.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_domain_actions_type(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let type_ = type_.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_notifications(
        &self,
        customer_id: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_notifications_opt_in(
        &self,
        customer_id: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
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
    }

    pub async fn put_v2_customers_customer_id_domains_notifications_opt_in(
        &self,
        customer_id: impl Into<ParamValue>,
        types: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let types = types.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_notifications_schemas_type(
        &self,
        customer_id: impl Into<ParamValue>,
        type_: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let type_ = type_.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge(
        &self,
        customer_id: impl Into<ParamValue>,
        notification_id: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let notification_id = notification_id.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_register(
        &self,
        customer_id: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let body = body.into();
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
    }

    pub async fn get_v2_customers_customer_id_domains_register_schema_tld(
        &self,
        customer_id: impl Into<ParamValue>,
        tld: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let tld = tld.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_register_validate(
        &self,
        customer_id: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let body = body.into();
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
    }

    pub async fn get_v2_domains_maintenances(
        &self,
        x_request_id: Option<ParamValue>,
        status: Option<ParamValue>,
        modified_at_after: Option<ParamValue>,
        starts_at_after: Option<ParamValue>,
        limit: Option<ParamValue>,
    ) -> ApiResult<Value> {
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
    }

    pub async fn get_v2_domains_maintenances_maintenance_id(
        &self,
        maintenance_id: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let maintenance_id = maintenance_id.into();
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
    }

    pub async fn get_v2_domains_usage_yyyymm(
        &self,
        yyyymm: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
        includes: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let yyyymm = yyyymm.into();
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
    }

    pub async fn patch_v2_customers_customer_id_domains_domain_contacts(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        body: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
        let body = body.into();
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
    }

    pub async fn post_v2_customers_customer_id_domains_domain_regenerate_auth_code(
        &self,
        customer_id: impl Into<ParamValue>,
        domain: impl Into<ParamValue>,
        x_request_id: Option<ParamValue>,
    ) -> ApiResult<Value> {
        let customer_id = customer_id.into();
        let domain = domain.into();
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
    }
}
