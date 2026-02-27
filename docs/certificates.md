# CertificatesService

SSL certificate purchase, validation, lifecycle, and revocation endpoints.

## Accessor

```rust
let service = client.certificates();
```

## Endpoints

### certificate_create

Calls `POST /v1/certificates`.

```rust
let response = client.certificates().certificate_create(json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### certificate_validate

Calls `POST /v1/certificates/validate`.

```rust
let response = client.certificates().certificate_validate(json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### certificate_get

Calls `GET /v1/certificates/{certificateId}`.

```rust
let response = client.certificates().certificate_get("sample".into()).await?;
```

```json
{}
```

### certificate_action_retrieve

Calls `GET /v1/certificates/{certificateId}/actions`.

```rust
let response = client.certificates().certificate_action_retrieve("sample".into()).await?;
```

```json
{}
```

### certificate_resend_email

Calls `POST /v1/certificates/{certificateId}/email/{emailId}/resend`.

```rust
let response = client.certificates().certificate_resend_email("sample".into(), "sample".into()).await?;
```

```json
{}
```

### certificate_alternate_email_address

Calls `POST /v1/certificates/{certificateId}/email/resend/{emailAddress}`.

```rust
let response = client.certificates().certificate_alternate_email_address("sample".into(), vec!["sample"].into()).await?;
```

```json
{}
```

### certificate_resend_email_address

Calls `POST /v1/certificates/{certificateId}/email/{emailId}/resend/{emailAddress}`.

```rust
let response = client.certificates().certificate_resend_email_address("sample".into(), "sample".into(), vec!["sample"].into()).await?;
```

```json
{}
```

### certificate_email_history

Calls `GET /v1/certificates/{certificateId}/email/history`.

```rust
let response = client.certificates().certificate_email_history("sample".into()).await?;
```

```json
{}
```

### certificate_callback_delete

Calls `DELETE /v1/certificates/{certificateId}/callback`.

```rust
let response = client.certificates().certificate_callback_delete("sample".into()).await?;
```

```json
{}
```

### certificate_callback_get

Calls `GET /v1/certificates/{certificateId}/callback`.

```rust
let response = client.certificates().certificate_callback_get("sample".into()).await?;
```

```json
{}
```

### certificate_callback_replace

Calls `PUT /v1/certificates/{certificateId}/callback`.

```rust
let response = client.certificates().certificate_callback_replace("sample".into(), "sample".into()).await?;
```

```json
{}
```

### certificate_cancel

Calls `POST /v1/certificates/{certificateId}/cancel`.

```rust
let response = client.certificates().certificate_cancel("sample".into()).await?;
```

```json
{}
```

### certificate_download

Calls `GET /v1/certificates/{certificateId}/download`.

```rust
let response = client.certificates().certificate_download("sample".into()).await?;
```

```json
{}
```

### certificate_reissue

Calls `POST /v1/certificates/{certificateId}/reissue`.

```rust
let response = client.certificates().certificate_reissue("sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### certificate_renew

Calls `POST /v1/certificates/{certificateId}/renew`.

```rust
let response = client.certificates().certificate_renew("sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### certificate_revoke

Calls `POST /v1/certificates/{certificateId}/revoke`.

```rust
let response = client.certificates().certificate_revoke("sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### certificate_siteseal_get

Calls `GET /v1/certificates/{certificateId}/siteSeal`.

```rust
let response = client.certificates().certificate_siteseal_get("sample".into(), Some("sample".into()), Some("sample".into())).await?;
```

```json
{}
```

### certificate_verifydomaincontrol

Calls `POST /v1/certificates/{certificateId}/verifyDomainControl`.

```rust
let response = client.certificates().certificate_verifydomaincontrol("sample".into()).await?;
```

```json
{}
```

### certificate_get_entitlement

Calls `GET /v2/certificates`.

```rust
let response = client.certificates().certificate_get_entitlement("sample".into(), Some(true.into())).await?;
```

```json
{}
```

### certificate_create_v2

Calls `POST /v2/certificates`.

```rust
let response = client.certificates().certificate_create_v2(json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```

### certificate_download_entitlement

Calls `GET /v2/certificates/download`.

```rust
let response = client.certificates().certificate_download_entitlement("sample".into()).await?;
```

```json
{}
```

### get_customer_certificates_by_customer_id

Calls `GET /v2/customers/{customerId}/certificates`.

```rust
let response = client.certificates().get_customer_certificates_by_customer_id("sample".into(), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### get_certificate_detail_by_cert_identifier

Calls `GET /v2/customers/{customerId}/certificates/{certificateId}`.

```rust
let response = client.certificates().get_certificate_detail_by_cert_identifier("sample".into(), "sample".into()).await?;
```

```json
{}
```

### get_domain_information_by_certificate_id

Calls `GET /v2/customers/{customerId}/certificates/{certificateId}/domainVerifications`.

```rust
let response = client.certificates().get_domain_information_by_certificate_id("sample".into(), "sample".into()).await?;
```

```json
{}
```

### get_domain_details_by_domain

Calls `GET /v2/customers/{customerId}/certificates/{certificateId}/domainVerifications/{domain}`.

```rust
let response = client.certificates().get_domain_details_by_domain("sample".into(), "sample".into(), "sample".into()).await?;
```

```json
{}
```

### get_acme_external_account_binding

Calls `GET /v2/customers/{customerId}/certificates/acme/externalAccountBinding`.

```rust
let response = client.certificates().get_acme_external_account_binding("sample".into()).await?;
```

```json
{}
```

### retrieve_ssl_by_domain_reseller

Calls `GET /v2/certificates/subscriptions/search`.

```rust
let response = client.certificates().retrieve_ssl_by_domain_reseller(Some(1_i64.into()), Some(1_i64.into()), Some("sample".into()), Some(vec!["sample"].into()), Some("sample".into()), Some("sample".into())).await?;
```

```json
{}
```

### retrieve_ssl_by_domain_subscription_reseller

Calls `GET /v2/certificates/subscription/{guid}`.

```rust
let response = client.certificates().retrieve_ssl_by_domain_subscription_reseller("sample".into(), Some(1_i64.into()), Some(1_i64.into()), Some("sample".into()), Some(vec!["sample"].into()), Some("sample".into()), Some("sample".into())).await?;
```

```json
{}
```
