# Certificates Service

Client accessor: `client.certificates()`

## Method Index

- [`create`](#create): `CreateResponse`
- [`validate`](#validate): `ValidateResponse`
- [`get`](#get): `GetResponse`
- [`list_actions`](#list_actions): `ListActionsResponse`
- [`resend_email`](#resend_email): `ResendEmailResponse`
- [`add_alternate_email_address`](#add_alternate_email_address): `AddAlternateEmailAddressResponse`
- [`resend_email_to_address`](#resend_email_to_address): `ResendEmailToAddressResponse`
- [`get_email_history`](#get_email_history): `GetEmailHistoryResponse`
- [`delete_callback`](#delete_callback): `DeleteCallbackResponse`
- [`get_callback`](#get_callback): `GetCallbackResponse`
- [`replace_callback`](#replace_callback): `ReplaceCallbackResponse`
- [`cancel`](#cancel): `CancelResponse`
- [`download`](#download): `DownloadResponse`
- [`reissue`](#reissue): `ReissueResponse`
- [`renew`](#renew): `RenewResponse`
- [`revoke`](#revoke): `RevokeResponse`
- [`get_site_seal`](#get_site_seal): `GetSiteSealResponse`
- [`verify_domain_control`](#verify_domain_control): `VerifyDomainControlResponse`
- [`get_by_entitlement`](#get_by_entitlement): `GetByEntitlementResponse`
- [`create_for_entitlement`](#create_for_entitlement): `CreateForEntitlementResponse`
- [`download_by_entitlement`](#download_by_entitlement): `DownloadByEntitlementResponse`
- [`list_customer_certificates`](#list_customer_certificates): `ListCustomerCertificatesResponse`
- [`get_customer_certificate`](#get_customer_certificate): `GetCustomerCertificateResponse`
- [`list_domain_verifications`](#list_domain_verifications): `ListDomainVerificationsResponse`
- [`get_domain_verification_details`](#get_domain_verification_details): `GetDomainVerificationDetailsResponse`
- [`get_acme_external_account_binding`](#get_acme_external_account_binding): `GetAcmeExternalAccountBindingResponse`
- [`search_subscriptions_by_domain`](#search_subscriptions_by_domain): `SearchSubscriptionsByDomainResponse`
- [`list_subscription_certificates`](#list_subscription_certificates): `ListSubscriptionCertificatesResponse`

## Methods

### create

Returns: `CreateResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateCreateRequest;

let request = CertificateCreateRequest {
    certificate_create: "example.com".into(),
    x_market_id: Some("header-value".into()),
};
let response = client.certificates().certificate_create(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### validate

Returns: `ValidateResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateValidateRequest;

let request = CertificateValidateRequest {
    certificate_create: "example.com".into(),
    x_market_id: Some("header-value".into()),
};
let response = client.certificates().certificate_validate(request).await?;
```

Response JSON example:

```json
{
  "valid": false,
  "issues": [
    {
      "path": "dnsNames[0]",
      "message": "SAN entry is invalid"
    }
  ]
}
```

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateGetRequest;

let request = CertificateGetRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_get(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### list_actions

Returns: `ListActionsResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateActionRetrieveRequest;

let request = CertificateActionRetrieveRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_action_retrieve(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "ISSUED",
  "commonName": "example.com",
  "expires": "2027-03-11T00:00:00Z"
}
```

### resend_email

Returns: `ResendEmailResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateResendEmailRequest;

let request = CertificateResendEmailRequest {
    certificate_id: "123456789".into(),
    email_id: "123456789".into(),
};
let response = client.certificates().certificate_resend_email(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### add_alternate_email_address

Returns: `AddAlternateEmailAddressResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateAlternateEmailAddressRequest;

let request = CertificateAlternateEmailAddressRequest {
    certificate_id: "123456789".into(),
    email_address: "example.com".into(),
};
let response = client.certificates().certificate_alternate_email_address(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### resend_email_to_address

Returns: `ResendEmailToAddressResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateResendEmailAddressRequest;

let request = CertificateResendEmailAddressRequest {
    certificate_id: "123456789".into(),
    email_id: "123456789".into(),
    email_address: "example.com".into(),
};
let response = client.certificates().certificate_resend_email_address(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### get_email_history

Returns: `GetEmailHistoryResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateEmailHistoryRequest;

let request = CertificateEmailHistoryRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_email_history(request).await?;
```

Response JSON example:

```json
{
  "history": [
    {
      "emailId": "mail_1",
      "emailAddress": "admin@example.com",
      "status": "SENT",
      "createdAt": "2026-03-11T12:00:00Z"
    }
  ]
}
```

### delete_callback

Returns: `DeleteCallbackResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateCallbackDeleteRequest;

let request = CertificateCallbackDeleteRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_callback_delete(request).await?;
```

Response JSON example:

```json
{
  "callbackUrl": "https://example.com/callback",
  "enabled": true
}
```

### get_callback

Returns: `GetCallbackResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateCallbackGetRequest;

let request = CertificateCallbackGetRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_callback_get(request).await?;
```

Response JSON example:

```json
{
  "callbackUrl": "https://example.com/callback",
  "enabled": true
}
```

### replace_callback

Returns: `ReplaceCallbackResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateCallbackReplaceRequest;

let request = CertificateCallbackReplaceRequest {
    certificate_id: "123456789".into(),
    callback_url: "example.com".into(),
};
let response = client.certificates().certificate_callback_replace(request).await?;
```

Response JSON example:

```json
{
  "callbackUrl": "https://example.com/callback",
  "enabled": true
}
```

### cancel

Returns: `CancelResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateCancelRequest;

let request = CertificateCancelRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_cancel(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### download

Returns: `DownloadResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateDownloadRequest;

let request = CertificateDownloadRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_download(request).await?;
```

Response JSON example:

```json
{
  "certificate": "-----BEGIN CERTIFICATE-----...",
  "privateKey": "-----BEGIN PRIVATE KEY-----...",
  "caBundle": "-----BEGIN CERTIFICATE-----..."
}
```

### reissue

Returns: `ReissueResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateReissueRequest;

let request = CertificateReissueRequest {
    certificate_id: "123456789".into(),
    reissue_create: "example.com".into(),
};
let response = client.certificates().certificate_reissue(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### renew

Returns: `RenewResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateRenewRequest;

let request = CertificateRenewRequest {
    certificate_id: "123456789".into(),
    renew_create: "example.com".into(),
};
let response = client.certificates().certificate_renew(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### revoke

Returns: `RevokeResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateRevokeRequest;

let request = CertificateRevokeRequest {
    certificate_id: "123456789".into(),
    certificate_revoke: "example.com".into(),
};
let response = client.certificates().certificate_revoke(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### get_site_seal

Returns: `GetSiteSealResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateGetRequest;

let request = CertificateGetRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_get(request).await?;
```

Response JSON example:

```json
{
  "html": "<div>Site Seal</div>"
}
```

### verify_domain_control

Returns: `VerifyDomainControlResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateVerifydomaincontrolRequest;

let request = CertificateVerifydomaincontrolRequest {
    certificate_id: "123456789".into(),
};
let response = client.certificates().certificate_verifydomaincontrol(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### get_by_entitlement

Returns: `GetByEntitlementResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateGetEntitlementRequest;

let request = CertificateGetEntitlementRequest {
    entitlement_id: "123456789".into(),
    latest: Some("example.com".into()),
};
let response = client.certificates().certificate_get_entitlement(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123456",
      "commonName": "example.com",
      "status": "ISSUED",
      "expires": "2027-03-11T00:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

### create_for_entitlement

Returns: `CreateForEntitlementResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateCreateRequest;

let request = CertificateCreateRequest {
    certificate_create: "example.com".into(),
    x_market_id: Some("header-value".into()),
};
let response = client.certificates().certificate_create(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### download_by_entitlement

Returns: `DownloadByEntitlementResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::CertificateDownloadEntitlementRequest;

let request = CertificateDownloadEntitlementRequest {
    entitlement_id: "123456789".into(),
};
let response = client.certificates().certificate_download_entitlement(request).await?;
```

Response JSON example:

```json
{
  "certificate": "-----BEGIN CERTIFICATE-----...",
  "privateKey": "-----BEGIN PRIVATE KEY-----...",
  "caBundle": "-----BEGIN CERTIFICATE-----..."
}
```

### list_customer_certificates

Returns: `ListCustomerCertificatesResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::GetCustomerCertificatesByCustomerIdRequest;

let request = GetCustomerCertificatesByCustomerIdRequest {
    customer_id: "123456789".into(),
    offset: Some(1_i64.into()),
    limit: Some(1_i64.into()),
};
let response = client.certificates().get_customer_certificates_by_customer_id(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123456",
      "commonName": "example.com",
      "status": "ISSUED",
      "expires": "2027-03-11T00:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

### get_customer_certificate

Returns: `GetCustomerCertificateResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::GetCustomerCertificatesByCustomerIdRequest;

let request = GetCustomerCertificatesByCustomerIdRequest {
    customer_id: "123456789".into(),
    offset: Some(1_i64.into()),
    limit: Some(1_i64.into()),
};
let response = client.certificates().get_customer_certificates_by_customer_id(request).await?;
```

Response JSON example:

```json
{
  "certificateId": "crt_123456",
  "status": "PENDING_ISSUANCE"
}
```

### list_domain_verifications

Returns: `ListDomainVerificationsResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::GetDomainInformationByCertificateIdRequest;

let request = GetDomainInformationByCertificateIdRequest {
    customer_id: "123456789".into(),
    certificate_id: "123456789".into(),
};
let response = client.certificates().get_domain_information_by_certificate_id(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123456",
      "commonName": "example.com",
      "status": "ISSUED",
      "expires": "2027-03-11T00:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

### get_domain_verification_details

Returns: `GetDomainVerificationDetailsResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::GetDomainDetailsByDomainRequest;

let request = GetDomainDetailsByDomainRequest {
    customer_id: "123456789".into(),
    certificate_id: "123456789".into(),
    domain: "example.com".into(),
};
let response = client.certificates().get_domain_details_by_domain(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "method": "DNS",
  "status": "PENDING",
  "token": "_acme-challenge",
  "value": "token-value"
}
```

### get_acme_external_account_binding

Returns: `GetAcmeExternalAccountBindingResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::GetAcmeExternalAccountBindingRequest;

let request = GetAcmeExternalAccountBindingRequest {
    customer_id: "123456789".into(),
};
let response = client.certificates().get_acme_external_account_binding(request).await?;
```

Response JSON example:

```json
{
  "kid": "kid_123",
  "hmacKey": "hmac_abc"
}
```

### search_subscriptions_by_domain

Returns: `SearchSubscriptionsByDomainResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::RetrieveSslByDomainResellerRequest;

let request = RetrieveSslByDomainResellerRequest {
    Some(1_i64.into()),
    Some(1_i64.into()),
    domain: Some("example.com".into()),
    status: Some("example.com".into()),
    type_: Some("example.com".into()),
    validation: Some("example.com".into()),
};
let response = client.certificates().retrieve_ssl_by_domain_reseller(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123456",
      "commonName": "example.com",
      "status": "ISSUED",
      "expires": "2027-03-11T00:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

### list_subscription_certificates

Returns: `ListSubscriptionCertificatesResponse`

Request code:

```rust
use godaddy_rust::dto::certificates::request::RetrieveSslByDomainSubscriptionResellerRequest;

let request = RetrieveSslByDomainSubscriptionResellerRequest {
    "123456789".into(),
    Some(1_i64.into()),
    Some(1_i64.into()),
    Some("example.com".into()),
    Some("example.com".into()),
    Some("example.com".into()),
    Some("example.com".into()),
};
let response = client.certificates().retrieve_ssl_by_domain_subscription_reseller(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123456",
      "commonName": "example.com",
      "status": "ISSUED",
      "expires": "2027-03-11T00:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for certificates endpoints.




