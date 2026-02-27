# DomainsService

Domain availability, purchase, management, transfer, and DNS endpoints.

## Accessor

```rust
let service = client.domains();
```

## Endpoints

### list

Calls `GET /v1/domains`.

```rust
let response = client.domains().list(Some("header-value".into()), Some(vec!["sample"].into()), Some(vec!["sample"].into()), Some(1_i64.into()), Some("sample".into()), Some(vec!["sample"].into()), Some("sample".into())).await?;
```

```json
{}
```

### get_agreement

Calls `GET /v1/domains/agreements`.

```rust
let response = client.domains().get_agreement(vec!["sample"].into(), true.into(), Some("header-value".into()), Some(true.into())).await?;
```

```json
{}
```

### available

Calls `GET /v1/domains/available`.

```rust
let response = client.domains().available("sample".into(), Some("sample".into()), Some(true.into())).await?;
```

```json
{}
```

### available_bulk

Calls `POST /v1/domains/available`.

```rust
let response = client.domains().available_bulk(vec!["sample"].into(), Some("sample".into())).await?;
```

```json
{}
```

### contacts_validate

Calls `POST /v1/domains/contacts/validate`.

```rust
let response = client.domains().contacts_validate(json!({"sample": true}).into(), Some("header-value".into()), Some("sample".into())).await?;
```

```json
{}
```

### purchase

Calls `POST /v1/domains/purchase`.

```rust
let response = client.domains().purchase(json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### schema

Calls `GET /v1/domains/purchase/schema/{tld}`.

```rust
let response = client.domains().schema("sample".into()).await?;
```

```json
{}
```

### validate

Calls `POST /v1/domains/purchase/validate`.

```rust
let response = client.domains().validate(json!({"sample": true}).into()).await?;
```

```json
{}
```

### suggest

Calls `GET /v1/domains/suggest`.

```rust
let response = client.domains().suggest(Some("header-value".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some(vec!["sample"].into()), Some(vec!["sample"].into()), Some(1_i64.into()), Some(1_i64.into()), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### tlds

Calls `GET /v1/domains/tlds`.

```rust
let response = client.domains().tlds().await?;
```

```json
{}
```

### cancel

Calls `DELETE /v1/domains/{domain}`.

```rust
let response = client.domains().cancel("sample".into()).await?;
```

```json
{}
```

### get

Calls `GET /v1/domains/{domain}`.

```rust
let response = client.domains().get("sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### update

Calls `PATCH /v1/domains/{domain}`.

```rust
let response = client.domains().update("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### update_contacts

Calls `PATCH /v1/domains/{domain}/contacts`.

```rust
let response = client.domains().update_contacts("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### cancel_privacy

Calls `DELETE /v1/domains/{domain}/privacy`.

```rust
let response = client.domains().cancel_privacy("sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### purchase_privacy

Calls `POST /v1/domains/{domain}/privacy/purchase`.

```rust
let response = client.domains().purchase_privacy("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### record_add

Calls `PATCH /v1/domains/{domain}/records`.

```rust
let response = client.domains().record_add("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### record_replace

Calls `PUT /v1/domains/{domain}/records`.

```rust
let response = client.domains().record_replace("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### record_get

Calls `GET /v1/domains/{domain}/records/{type}/{name}`.

```rust
let response = client.domains().record_get("sample".into(), "sample".into(), "sample".into(), Some("header-value".into()), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### record_replace_type_name

Calls `PUT /v1/domains/{domain}/records/{type}/{name}`.

```rust
let response = client.domains().record_replace_type_name("sample".into(), "sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### record_delete_type_name

Calls `DELETE /v1/domains/{domain}/records/{type}/{name}`.

```rust
let response = client.domains().record_delete_type_name("sample".into(), "sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### record_replace_type

Calls `PUT /v1/domains/{domain}/records/{type}`.

```rust
let response = client.domains().record_replace_type("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### renew

Calls `POST /v1/domains/{domain}/renew`.

```rust
let response = client.domains().renew("sample".into(), Some("header-value".into()), Some(json!({"sample": true}).into())).await?;
```

```json
{}
```

### transfer_in

Calls `POST /v1/domains/{domain}/transfer`.

```rust
let response = client.domains().transfer_in("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### verify_email

Calls `POST /v1/domains/{domain}/verifyRegistrantEmail`.

```rust
let response = client.domains().verify_email("sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_domain

Calls `GET /v2/customers/{customerId}/domains/{domain}`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_domain("sample".into(), "sample".into(), Some("header-value".into()), Some(vec!["sample"].into())).await?;
```

```json
{}
```

### delete_v2_customers_customer_id_domains_domain_change_of_registrant

Calls `DELETE /v2/customers/{customerId}/domains/{domain}/changeOfRegistrant`.

```rust
let response = client.domains().delete_v2_customers_customer_id_domains_domain_change_of_registrant("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_domain_change_of_registrant

Calls `GET /v2/customers/{customerId}/domains/{domain}/changeOfRegistrant`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_domain_change_of_registrant("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### patch_v2_customers_customer_id_domains_domain_dnssec_records

Calls `PATCH /v2/customers/{customerId}/domains/{domain}/dnssecRecords`.

```rust
let response = client.domains().patch_v2_customers_customer_id_domains_domain_dnssec_records("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### delete_v2_customers_customer_id_domains_domain_dnssec_records

Calls `DELETE /v2/customers/{customerId}/domains/{domain}/dnssecRecords`.

```rust
let response = client.domains().delete_v2_customers_customer_id_domains_domain_dnssec_records("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### put_v2_customers_customer_id_domains_domain_name_servers

Calls `PUT /v2/customers/{customerId}/domains/{domain}/nameServers`.

```rust
let response = client.domains().put_v2_customers_customer_id_domains_domain_name_servers("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_domain_privacy_forwarding

Calls `GET /v2/customers/{customerId}/domains/{domain}/privacy/forwarding`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_domain_privacy_forwarding("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### patch_v2_customers_customer_id_domains_domain_privacy_forwarding

Calls `PATCH /v2/customers/{customerId}/domains/{domain}/privacy/forwarding`.

```rust
let response = client.domains().patch_v2_customers_customer_id_domains_domain_privacy_forwarding("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_redeem

Calls `POST /v2/customers/{customerId}/domains/{domain}/redeem`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_redeem("sample".into(), "sample".into(), Some("header-value".into()), Some(json!({"sample": true}).into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_renew

Calls `POST /v2/customers/{customerId}/domains/{domain}/renew`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_renew("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer

Calls `POST /v2/customers/{customerId}/domains/{domain}/transfer`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_domain_transfer

Calls `GET /v2/customers/{customerId}/domains/{domain}/transfer`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_domain_transfer("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_validate

Calls `POST /v2/customers/{customerId}/domains/{domain}/transfer/validate`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_validate("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_in_accept

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferInAccept`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_in_accept("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_in_cancel

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferInCancel`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_in_cancel("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_in_restart

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferInRestart`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_in_restart("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_in_retry

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferInRetry`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_in_retry("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_out

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferOut`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_out("sample".into(), "sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_out_accept

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferOutAccept`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_out_accept("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_transfer_out_reject

Calls `POST /v2/customers/{customerId}/domains/{domain}/transferOutReject`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_transfer_out_reject("sample".into(), "sample".into(), Some("header-value".into()), Some("sample".into())).await?;
```

```json
{}
```

### domains_forwards_delete

Calls `DELETE /v2/customers/{customerId}/domains/forwards/{fqdn}`.

```rust
let response = client.domains().domains_forwards_delete("sample".into(), "sample".into()).await?;
```

```json
{}
```

### domains_forwards_get

Calls `GET /v2/customers/{customerId}/domains/forwards/{fqdn}`.

```rust
let response = client.domains().domains_forwards_get("sample".into(), "sample".into(), Some(true.into())).await?;
```

```json
{}
```

### domains_forwards_put

Calls `PUT /v2/customers/{customerId}/domains/forwards/{fqdn}`.

```rust
let response = client.domains().domains_forwards_put("sample".into(), "sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### domains_forwards_post

Calls `POST /v2/customers/{customerId}/domains/forwards/{fqdn}`.

```rust
let response = client.domains().domains_forwards_post("sample".into(), "sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_domain_actions

Calls `GET /v2/customers/{customerId}/domains/{domain}/actions`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_domain_actions("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### delete_v2_customers_customer_id_domains_domain_actions_type

Calls `DELETE /v2/customers/{customerId}/domains/{domain}/actions/{type}`.

```rust
let response = client.domains().delete_v2_customers_customer_id_domains_domain_actions_type("sample".into(), "sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_domain_actions_type

Calls `GET /v2/customers/{customerId}/domains/{domain}/actions/{type}`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_domain_actions_type("sample".into(), "sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_notifications

Calls `GET /v2/customers/{customerId}/domains/notifications`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_notifications("sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_notifications_opt_in

Calls `GET /v2/customers/{customerId}/domains/notifications/optIn`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_notifications_opt_in("sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### put_v2_customers_customer_id_domains_notifications_opt_in

Calls `PUT /v2/customers/{customerId}/domains/notifications/optIn`.

```rust
let response = client.domains().put_v2_customers_customer_id_domains_notifications_opt_in("sample".into(), vec!["sample"].into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_notifications_schemas_type

Calls `GET /v2/customers/{customerId}/domains/notifications/schemas/{type}`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_notifications_schemas_type("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge

Calls `POST /v2/customers/{customerId}/domains/notifications/{notificationId}/acknowledge`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_register

Calls `POST /v2/customers/{customerId}/domains/register`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_register("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_customers_customer_id_domains_register_schema_tld

Calls `GET /v2/customers/{customerId}/domains/register/schema/{tld}`.

```rust
let response = client.domains().get_v2_customers_customer_id_domains_register_schema_tld("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_register_validate

Calls `POST /v2/customers/{customerId}/domains/register/validate`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_register_validate("sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_domains_maintenances

Calls `GET /v2/domains/maintenances`.

```rust
let response = client.domains().get_v2_domains_maintenances(Some("header-value".into()), Some(vec!["sample"].into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into())).await?;
```

```json
{}
```

### get_v2_domains_maintenances_maintenance_id

Calls `GET /v2/domains/maintenances/{maintenanceId}`.

```rust
let response = client.domains().get_v2_domains_maintenances_maintenance_id("sample".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get_v2_domains_usage_yyyymm

Calls `GET /v2/domains/usage/{yyyymm}`.

```rust
let response = client.domains().get_v2_domains_usage_yyyymm("sample".into(), Some("header-value".into()), Some(vec!["sample"].into())).await?;
```

```json
{}
```

### patch_v2_customers_customer_id_domains_domain_contacts

Calls `PATCH /v2/customers/{customerId}/domains/{domain}/contacts`.

```rust
let response = client.domains().patch_v2_customers_customer_id_domains_domain_contacts("sample".into(), "sample".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### post_v2_customers_customer_id_domains_domain_regenerate_auth_code

Calls `POST /v2/customers/{customerId}/domains/{domain}/regenerateAuthCode`.

```rust
let response = client.domains().post_v2_customers_customer_id_domains_domain_regenerate_auth_code("sample".into(), "sample".into(), Some("header-value".into())).await?;
```

```json
{}
```
