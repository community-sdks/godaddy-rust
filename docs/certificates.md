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
use godaddy_rust::dto::certificates::request::CreateRequest;

let request = CreateRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().create(request).await?;
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
use godaddy_rust::dto::certificates::request::ValidateRequest;

let request = ValidateRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().validate(request).await?;
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
use godaddy_rust::dto::certificates::request::GetRequest;

let request = GetRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get(request).await?;
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
use godaddy_rust::dto::certificates::request::ListActionsRequest;

let request = ListActionsRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().list_actions(request).await?;
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
use godaddy_rust::dto::certificates::request::ResendEmailRequest;

let request = ResendEmailRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().resend_email(request).await?;
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
use godaddy_rust::dto::certificates::request::AddAlternateEmailAddressRequest;

let request = AddAlternateEmailAddressRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().add_alternate_email_address(request).await?;
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
use godaddy_rust::dto::certificates::request::ResendEmailToAddressRequest;

let request = ResendEmailToAddressRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().resend_email_to_address(request).await?;
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
use godaddy_rust::dto::certificates::request::GetEmailHistoryRequest;

let request = GetEmailHistoryRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get_email_history(request).await?;
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
use godaddy_rust::dto::certificates::request::DeleteCallbackRequest;

let request = DeleteCallbackRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().delete_callback(request).await?;
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
use godaddy_rust::dto::certificates::request::GetCallbackRequest;

let request = GetCallbackRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get_callback(request).await?;
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
use godaddy_rust::dto::certificates::request::ReplaceCallbackRequest;

let request = ReplaceCallbackRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().replace_callback(request).await?;
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
use godaddy_rust::dto::certificates::request::CancelRequest;

let request = CancelRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().cancel(request).await?;
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
use godaddy_rust::dto::certificates::request::DownloadRequest;

let request = DownloadRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().download(request).await?;
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
use godaddy_rust::dto::certificates::request::ReissueRequest;

let request = ReissueRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().reissue(request).await?;
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
use godaddy_rust::dto::certificates::request::RenewRequest;

let request = RenewRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().renew(request).await?;
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
use godaddy_rust::dto::certificates::request::RevokeRequest;

let request = RevokeRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().revoke(request).await?;
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
use godaddy_rust::dto::certificates::request::GetSiteSealRequest;

let request = GetSiteSealRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get_site_seal(request).await?;
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
use godaddy_rust::dto::certificates::request::VerifyDomainControlRequest;

let request = VerifyDomainControlRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().verify_domain_control(request).await?;
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
use godaddy_rust::dto::certificates::request::GetByEntitlementRequest;

let request = GetByEntitlementRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get_by_entitlement(request).await?;
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
use godaddy_rust::dto::certificates::request::CreateForEntitlementRequest;

let request = CreateForEntitlementRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().create_for_entitlement(request).await?;
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
use godaddy_rust::dto::certificates::request::DownloadByEntitlementRequest;

let request = DownloadByEntitlementRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().download_by_entitlement(request).await?;
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
use godaddy_rust::dto::certificates::request::ListCustomerCertificatesRequest;

let request = ListCustomerCertificatesRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().list_customer_certificates(request).await?;
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
use godaddy_rust::dto::certificates::request::GetCustomerCertificateRequest;

let request = GetCustomerCertificateRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get_customer_certificate(request).await?;
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
use godaddy_rust::dto::certificates::request::ListDomainVerificationsRequest;

let request = ListDomainVerificationsRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().list_domain_verifications(request).await?;
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
use godaddy_rust::dto::certificates::request::GetDomainVerificationDetailsRequest;

let request = GetDomainVerificationDetailsRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().get_domain_verification_details(request).await?;
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

let request = GetAcmeExternalAccountBindingRequest::new(
    // Fill endpoint fields here
);
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
use godaddy_rust::dto::certificates::request::SearchSubscriptionsByDomainRequest;

let request = SearchSubscriptionsByDomainRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().search_subscriptions_by_domain(request).await?;
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
use godaddy_rust::dto::certificates::request::ListSubscriptionCertificatesRequest;

let request = ListSubscriptionCertificatesRequest::new(
    // Fill endpoint fields here
);
let response = client.certificates().list_subscription_certificates(request).await?;
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
