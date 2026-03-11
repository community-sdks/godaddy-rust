# Domains Service

Client accessor: `client.domains()`

## Method Index

- [`list`](#list): `ListResponse`
- [`get_agreement`](#get_agreement): `GetAgreementResponse`
- [`available`](#available): `AvailableResponse`
- [`available_bulk`](#available_bulk): `AvailableBulkResponse`
- [`contacts_validate`](#contacts_validate): `ContactsValidateResponse`
- [`purchase`](#purchase): `PurchaseResponse`
- [`schema`](#schema): `SchemaResponse`
- [`validate`](#validate): `ValidateResponse`
- [`suggest`](#suggest): `SuggestResponse`
- [`tlds`](#tlds): `TldsResponse`
- [`cancel`](#cancel): `CancelResponse`
- [`get`](#get): `GetResponse`
- [`update`](#update): `UpdateResponse`
- [`update_contacts`](#update_contacts): `UpdateContactsResponse`
- [`cancel_privacy`](#cancel_privacy): `CancelPrivacyResponse`
- [`purchase_privacy`](#purchase_privacy): `PurchasePrivacyResponse`
- [`record_add`](#record_add): `RecordAddResponse`
- [`record_replace`](#record_replace): `RecordReplaceResponse`
- [`record_get`](#record_get): `RecordGetResponse`
- [`record_replace_type_name`](#record_replace_type_name): `RecordReplaceTypeNameResponse`
- [`record_delete_type_name`](#record_delete_type_name): `RecordDeleteTypeNameResponse`
- [`record_replace_type`](#record_replace_type): `RecordReplaceTypeResponse`
- [`renew`](#renew): `RenewResponse`
- [`transfer_in`](#transfer_in): `TransferInResponse`
- [`verify_email`](#verify_email): `VerifyEmailResponse`
- [`get_customer_domain`](#get_customer_domain): `GetCustomerDomainResponse`
- [`cancel_customer_domain_change_of_registrant`](#cancel_customer_domain_change_of_registrant): `CancelCustomerDomainChangeOfRegistrantResponse`
- [`get_customer_domain_change_of_registrant`](#get_customer_domain_change_of_registrant): `GetCustomerDomainChangeOfRegistrantResponse`
- [`add_customer_domain_dnssec_records`](#add_customer_domain_dnssec_records): `AddCustomerDomainDnssecRecordsResponse`
- [`remove_customer_domain_dnssec_records`](#remove_customer_domain_dnssec_records): `RemoveCustomerDomainDnssecRecordsResponse`
- [`replace_customer_domain_name_servers`](#replace_customer_domain_name_servers): `ReplaceCustomerDomainNameServersResponse`
- [`get_customer_domain_privacy_forwarding`](#get_customer_domain_privacy_forwarding): `GetCustomerDomainPrivacyForwardingResponse`
- [`update_customer_domain_privacy_forwarding`](#update_customer_domain_privacy_forwarding): `UpdateCustomerDomainPrivacyForwardingResponse`
- [`redeem_customer_domain`](#redeem_customer_domain): `RedeemCustomerDomainResponse`
- [`renew_customer_domain`](#renew_customer_domain): `RenewCustomerDomainResponse`
- [`transfer_customer_domain`](#transfer_customer_domain): `TransferCustomerDomainResponse`
- [`get_customer_domain_transfer_status`](#get_customer_domain_transfer_status): `GetCustomerDomainTransferStatusResponse`
- [`validate_customer_domain_transfer`](#validate_customer_domain_transfer): `ValidateCustomerDomainTransferResponse`
- [`accept_customer_domain_transfer_in`](#accept_customer_domain_transfer_in): `AcceptCustomerDomainTransferInResponse`
- [`cancel_customer_domain_transfer_in`](#cancel_customer_domain_transfer_in): `CancelCustomerDomainTransferInResponse`
- [`restart_customer_domain_transfer_in`](#restart_customer_domain_transfer_in): `RestartCustomerDomainTransferInResponse`
- [`retry_customer_domain_transfer_in`](#retry_customer_domain_transfer_in): `RetryCustomerDomainTransferInResponse`
- [`initiate_customer_domain_transfer_out`](#initiate_customer_domain_transfer_out): `InitiateCustomerDomainTransferOutResponse`
- [`accept_customer_domain_transfer_out`](#accept_customer_domain_transfer_out): `AcceptCustomerDomainTransferOutResponse`
- [`reject_customer_domain_transfer_out`](#reject_customer_domain_transfer_out): `RejectCustomerDomainTransferOutResponse`
- [`delete_domain_forwarding`](#delete_domain_forwarding): `DeleteDomainForwardingResponse`
- [`get_domain_forwarding`](#get_domain_forwarding): `GetDomainForwardingResponse`
- [`update_domain_forwarding`](#update_domain_forwarding): `UpdateDomainForwardingResponse`
- [`create_domain_forwarding`](#create_domain_forwarding): `CreateDomainForwardingResponse`
- [`list_customer_domain_actions`](#list_customer_domain_actions): `ListCustomerDomainActionsResponse`
- [`cancel_customer_domain_action`](#cancel_customer_domain_action): `CancelCustomerDomainActionResponse`
- [`get_customer_domain_action`](#get_customer_domain_action): `GetCustomerDomainActionResponse`
- [`get_customer_domain_notifications`](#get_customer_domain_notifications): `GetCustomerDomainNotificationsResponse`
- [`get_customer_domain_notification_opt_ins`](#get_customer_domain_notification_opt_ins): `GetCustomerDomainNotificationOptInsResponse`
- [`update_customer_domain_notification_opt_ins`](#update_customer_domain_notification_opt_ins): `UpdateCustomerDomainNotificationOptInsResponse`
- [`get_customer_domain_notification_schema`](#get_customer_domain_notification_schema): `GetCustomerDomainNotificationSchemaResponse`
- [`acknowledge_customer_domain_notification`](#acknowledge_customer_domain_notification): `AcknowledgeCustomerDomainNotificationResponse`
- [`register_customer_domain`](#register_customer_domain): `RegisterCustomerDomainResponse`
- [`get_customer_domain_register_schema`](#get_customer_domain_register_schema): `GetCustomerDomainRegisterSchemaResponse`
- [`validate_customer_domain_register`](#validate_customer_domain_register): `ValidateCustomerDomainRegisterResponse`
- [`list_domain_maintenances`](#list_domain_maintenances): `ListDomainMaintenancesResponse`
- [`get_domain_maintenance`](#get_domain_maintenance): `GetDomainMaintenanceResponse`
- [`get_domain_usage`](#get_domain_usage): `GetDomainUsageResponse`
- [`update_customer_domain_contacts`](#update_customer_domain_contacts): `UpdateCustomerDomainContactsResponse`
- [`regenerate_customer_domain_auth_code`](#regenerate_customer_domain_auth_code): `RegenerateCustomerDomainAuthCodeResponse`

## Methods

### list

Returns: `ListResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ListRequest;

let request = ListRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().list(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### get_agreement

Returns: `GetAgreementResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetAgreementRequest;

let request = GetAgreementRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_agreement(request).await?;
```

Response JSON example:

```json
{
  "agreements": [
    {
      "agreementKey": "DNRA",
      "title": "Domain Name Registration Agreement",
      "url": "https://www.godaddy.com/legal/agreements/domain-registration"
    }
  ]
}
```

### available

Returns: `AvailableResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::AvailableRequest;

let request = AvailableRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().available(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "available": true,
  "price": 1999,
  "currency": "USD",
  "definitive": true,
  "period": 1
}
```

### available_bulk

Returns: `AvailableBulkResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::AvailableBulkRequest;

let request = AvailableBulkRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().available_bulk(request).await?;
```

Response JSON example:

```json
{
  "domains": [
    {
      "domain": "example.com",
      "available": true,
      "price": 1999,
      "currency": "USD"
    }
  ]
}
```

### contacts_validate

Returns: `ContactsValidateResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ContactsValidateRequest;

let request = ContactsValidateRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().contacts_validate(request).await?;
```

Response JSON example:

```json
{
  "valid": false,
  "errors": [
    {
      "path": "domain",
      "message": "Domain is invalid"
    }
  ]
}
```

### purchase

Returns: `PurchaseResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::PurchaseRequest;

let request = PurchaseRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().purchase(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### schema

Returns: `SchemaResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::SchemaRequest;

let request = SchemaRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().schema(request).await?;
```

Response JSON example:

```json
{
  "fields": [
    {
      "path": "consent.agreementKeys",
      "type": "array",
      "required": true
    }
  ]
}
```

### validate

Returns: `ValidateResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ValidateRequest;

let request = ValidateRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().validate(request).await?;
```

Response JSON example:

```json
{
  "valid": false,
  "errors": [
    {
      "path": "domain",
      "message": "Domain is invalid"
    }
  ]
}
```

### suggest

Returns: `SuggestResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::SuggestRequest;

let request = SuggestRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().suggest(request).await?;
```

Response JSON example:

```json
[
  "example.com",
  "example.net"
]
```

### tlds

Returns: `TldsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::TldsRequest;

let request = TldsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().tlds(request).await?;
```

Response JSON example:

```json
{
  "tld": "com",
  "type": "GENERIC",
  "isIdn": false
}
```

### cancel

Returns: `CancelResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::CancelRequest;

let request = CancelRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().cancel(request).await?;
```

Response JSON example:

```json
{
  "success": true
}
```

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetRequest;

let request = GetRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### update

Returns: `UpdateResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::UpdateRequest;

let request = UpdateRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().update(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### update_contacts

Returns: `UpdateContactsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::UpdateContactsRequest;

let request = UpdateContactsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().update_contacts(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### cancel_privacy

Returns: `CancelPrivacyResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::CancelPrivacyRequest;

let request = CancelPrivacyRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().cancel_privacy(request).await?;
```

Response JSON example:

```json
{
  "success": true
}
```

### purchase_privacy

Returns: `PurchasePrivacyResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::PurchasePrivacyRequest;

let request = PurchasePrivacyRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().purchase_privacy(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### record_add

Returns: `RecordAddResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RecordAddRequest;

let request = RecordAddRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().record_add(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### record_replace

Returns: `RecordReplaceResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RecordReplaceRequest;

let request = RecordReplaceRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().record_replace(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### record_get

Returns: `RecordGetResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RecordGetRequest;

let request = RecordGetRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().record_get(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### record_replace_type_name

Returns: `RecordReplaceTypeNameResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RecordReplaceTypeNameRequest;

let request = RecordReplaceTypeNameRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().record_replace_type_name(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### record_delete_type_name

Returns: `RecordDeleteTypeNameResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RecordDeleteTypeNameRequest;

let request = RecordDeleteTypeNameRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().record_delete_type_name(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### record_replace_type

Returns: `RecordReplaceTypeResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RecordReplaceTypeRequest;

let request = RecordReplaceTypeRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().record_replace_type(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### renew

Returns: `RenewResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RenewRequest;

let request = RenewRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().renew(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### transfer_in

Returns: `TransferInResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::TransferInRequest;

let request = TransferInRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().transfer_in(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### verify_email

Returns: `VerifyEmailResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::VerifyEmailRequest;

let request = VerifyEmailRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().verify_email(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### get_customer_domain

Returns: `GetCustomerDomainResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainRequest;

let request = GetCustomerDomainRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### cancel_customer_domain_change_of_registrant

Returns: `CancelCustomerDomainChangeOfRegistrantResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::CancelCustomerDomainChangeOfRegistrantRequest;

let request = CancelCustomerDomainChangeOfRegistrantRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().cancel_customer_domain_change_of_registrant(request).await?;
```

Response JSON example:

```json
{
  "success": true
}
```

### get_customer_domain_change_of_registrant

Returns: `GetCustomerDomainChangeOfRegistrantResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainChangeOfRegistrantRequest;

let request = GetCustomerDomainChangeOfRegistrantRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_change_of_registrant(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### add_customer_domain_dnssec_records

Returns: `AddCustomerDomainDnssecRecordsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::AddCustomerDomainDnssecRecordsRequest;

let request = AddCustomerDomainDnssecRecordsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().add_customer_domain_dnssec_records(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### remove_customer_domain_dnssec_records

Returns: `RemoveCustomerDomainDnssecRecordsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RemoveCustomerDomainDnssecRecordsRequest;

let request = RemoveCustomerDomainDnssecRecordsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().remove_customer_domain_dnssec_records(request).await?;
```

Response JSON example:

```json
{
  "type": "A",
  "name": "@",
  "data": "203.0.113.10",
  "ttl": 600
}
```

### replace_customer_domain_name_servers

Returns: `ReplaceCustomerDomainNameServersResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ReplaceCustomerDomainNameServersRequest;

let request = ReplaceCustomerDomainNameServersRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().replace_customer_domain_name_servers(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### get_customer_domain_privacy_forwarding

Returns: `GetCustomerDomainPrivacyForwardingResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainPrivacyForwardingRequest;

let request = GetCustomerDomainPrivacyForwardingRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_privacy_forwarding(request).await?;
```

Response JSON example:

```json
{
  "fqdn": "example.com",
  "type": "PERMANENT",
  "to": "https://www.example.com"
}
```

### update_customer_domain_privacy_forwarding

Returns: `UpdateCustomerDomainPrivacyForwardingResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::UpdateCustomerDomainPrivacyForwardingRequest;

let request = UpdateCustomerDomainPrivacyForwardingRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().update_customer_domain_privacy_forwarding(request).await?;
```

Response JSON example:

```json
{
  "fqdn": "example.com",
  "type": "PERMANENT",
  "to": "https://www.example.com"
}
```

### redeem_customer_domain

Returns: `RedeemCustomerDomainResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RedeemCustomerDomainRequest;

let request = RedeemCustomerDomainRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().redeem_customer_domain(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### renew_customer_domain

Returns: `RenewCustomerDomainResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RenewCustomerDomainRequest;

let request = RenewCustomerDomainRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().renew_customer_domain(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### transfer_customer_domain

Returns: `TransferCustomerDomainResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::TransferCustomerDomainRequest;

let request = TransferCustomerDomainRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().transfer_customer_domain(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### get_customer_domain_transfer_status

Returns: `GetCustomerDomainTransferStatusResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainTransferStatusRequest;

let request = GetCustomerDomainTransferStatusRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_transfer_status(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### validate_customer_domain_transfer

Returns: `ValidateCustomerDomainTransferResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ValidateCustomerDomainTransferRequest;

let request = ValidateCustomerDomainTransferRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().validate_customer_domain_transfer(request).await?;
```

Response JSON example:

```json
{
  "valid": false,
  "errors": [
    {
      "path": "domain",
      "message": "Domain is invalid"
    }
  ]
}
```

### accept_customer_domain_transfer_in

Returns: `AcceptCustomerDomainTransferInResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::AcceptCustomerDomainTransferInRequest;

let request = AcceptCustomerDomainTransferInRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().accept_customer_domain_transfer_in(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### cancel_customer_domain_transfer_in

Returns: `CancelCustomerDomainTransferInResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::CancelCustomerDomainTransferInRequest;

let request = CancelCustomerDomainTransferInRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().cancel_customer_domain_transfer_in(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### restart_customer_domain_transfer_in

Returns: `RestartCustomerDomainTransferInResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RestartCustomerDomainTransferInRequest;

let request = RestartCustomerDomainTransferInRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().restart_customer_domain_transfer_in(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### retry_customer_domain_transfer_in

Returns: `RetryCustomerDomainTransferInResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RetryCustomerDomainTransferInRequest;

let request = RetryCustomerDomainTransferInRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().retry_customer_domain_transfer_in(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### initiate_customer_domain_transfer_out

Returns: `InitiateCustomerDomainTransferOutResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::InitiateCustomerDomainTransferOutRequest;

let request = InitiateCustomerDomainTransferOutRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().initiate_customer_domain_transfer_out(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### accept_customer_domain_transfer_out

Returns: `AcceptCustomerDomainTransferOutResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::AcceptCustomerDomainTransferOutRequest;

let request = AcceptCustomerDomainTransferOutRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().accept_customer_domain_transfer_out(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### reject_customer_domain_transfer_out

Returns: `RejectCustomerDomainTransferOutResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RejectCustomerDomainTransferOutRequest;

let request = RejectCustomerDomainTransferOutRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().reject_customer_domain_transfer_out(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "transferStatus": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### delete_domain_forwarding

Returns: `DeleteDomainForwardingResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::DeleteDomainForwardingRequest;

let request = DeleteDomainForwardingRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().delete_domain_forwarding(request).await?;
```

Response JSON example:

```json
{
  "fqdn": "example.com",
  "type": "PERMANENT",
  "to": "https://www.example.com"
}
```

### get_domain_forwarding

Returns: `GetDomainForwardingResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetDomainForwardingRequest;

let request = GetDomainForwardingRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_domain_forwarding(request).await?;
```

Response JSON example:

```json
{
  "fqdn": "example.com",
  "type": "PERMANENT",
  "to": "https://www.example.com"
}
```

### update_domain_forwarding

Returns: `UpdateDomainForwardingResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::UpdateDomainForwardingRequest;

let request = UpdateDomainForwardingRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().update_domain_forwarding(request).await?;
```

Response JSON example:

```json
{
  "fqdn": "example.com",
  "type": "PERMANENT",
  "to": "https://www.example.com"
}
```

### create_domain_forwarding

Returns: `CreateDomainForwardingResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::CreateDomainForwardingRequest;

let request = CreateDomainForwardingRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().create_domain_forwarding(request).await?;
```

Response JSON example:

```json
{
  "fqdn": "example.com",
  "type": "PERMANENT",
  "to": "https://www.example.com"
}
```

### list_customer_domain_actions

Returns: `ListCustomerDomainActionsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ListCustomerDomainActionsRequest;

let request = ListCustomerDomainActionsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().list_customer_domain_actions(request).await?;
```

Response JSON example:

```json
{
  "actionType": "CHANGE_OF_REGISTRANT",
  "status": "PENDING",
  "createdAt": "2026-03-11T12:00:00Z",
  "domain": "example.com"
}
```

### cancel_customer_domain_action

Returns: `CancelCustomerDomainActionResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::CancelCustomerDomainActionRequest;

let request = CancelCustomerDomainActionRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().cancel_customer_domain_action(request).await?;
```

Response JSON example:

```json
{
  "success": true
}
```

### get_customer_domain_action

Returns: `GetCustomerDomainActionResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainActionRequest;

let request = GetCustomerDomainActionRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_action(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

### get_customer_domain_notifications

Returns: `GetCustomerDomainNotificationsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainNotificationsRequest;

let request = GetCustomerDomainNotificationsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_notifications(request).await?;
```

Response JSON example:

```json
{
  "type": "EXPIRATION",
  "optedIn": true
}
```

### get_customer_domain_notification_opt_ins

Returns: `GetCustomerDomainNotificationOptInsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainNotificationOptInsRequest;

let request = GetCustomerDomainNotificationOptInsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_notification_opt_ins(request).await?;
```

Response JSON example:

```json
{
  "type": "EXPIRATION",
  "optedIn": true
}
```

### update_customer_domain_notification_opt_ins

Returns: `UpdateCustomerDomainNotificationOptInsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::UpdateCustomerDomainNotificationOptInsRequest;

let request = UpdateCustomerDomainNotificationOptInsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().update_customer_domain_notification_opt_ins(request).await?;
```

Response JSON example:

```json
{
  "type": "EXPIRATION",
  "optedIn": true
}
```

### get_customer_domain_notification_schema

Returns: `GetCustomerDomainNotificationSchemaResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainNotificationSchemaRequest;

let request = GetCustomerDomainNotificationSchemaRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_notification_schema(request).await?;
```

Response JSON example:

```json
{
  "fields": [
    {
      "path": "type",
      "type": "string",
      "required": true
    }
  ]
}
```

### acknowledge_customer_domain_notification

Returns: `AcknowledgeCustomerDomainNotificationResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::AcknowledgeCustomerDomainNotificationRequest;

let request = AcknowledgeCustomerDomainNotificationRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().acknowledge_customer_domain_notification(request).await?;
```

Response JSON example:

```json
{
  "notificationId": "notif_1",
  "acknowledged": true,
  "acknowledgedAt": "2026-03-11T12:00:00Z"
}
```

### register_customer_domain

Returns: `RegisterCustomerDomainResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RegisterCustomerDomainRequest;

let request = RegisterCustomerDomainRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().register_customer_domain(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### get_customer_domain_register_schema

Returns: `GetCustomerDomainRegisterSchemaResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetCustomerDomainRegisterSchemaRequest;

let request = GetCustomerDomainRegisterSchemaRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_customer_domain_register_schema(request).await?;
```

Response JSON example:

```json
{
  "fields": [
    {
      "path": "consent.agreementKeys",
      "type": "array",
      "required": true
    }
  ]
}
```

### validate_customer_domain_register

Returns: `ValidateCustomerDomainRegisterResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ValidateCustomerDomainRegisterRequest;

let request = ValidateCustomerDomainRegisterRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().validate_customer_domain_register(request).await?;
```

Response JSON example:

```json
{
  "valid": false,
  "errors": [
    {
      "path": "domain",
      "message": "Domain is invalid"
    }
  ]
}
```

### list_domain_maintenances

Returns: `ListDomainMaintenancesResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::ListDomainMaintenancesRequest;

let request = ListDomainMaintenancesRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().list_domain_maintenances(request).await?;
```

Response JSON example:

```json
{
  "maintenanceId": "mnt_1",
  "status": "SCHEDULED",
  "startsAt": "2026-03-20T00:00:00Z",
  "endsAt": "2026-03-20T02:00:00Z"
}
```

### get_domain_maintenance

Returns: `GetDomainMaintenanceResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetDomainMaintenanceRequest;

let request = GetDomainMaintenanceRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_domain_maintenance(request).await?;
```

Response JSON example:

```json
{
  "maintenanceId": "mnt_1",
  "status": "SCHEDULED",
  "startsAt": "2026-03-20T00:00:00Z",
  "endsAt": "2026-03-20T02:00:00Z"
}
```

### get_domain_usage

Returns: `GetDomainUsageResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::GetDomainUsageRequest;

let request = GetDomainUsageRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().get_domain_usage(request).await?;
```

Response JSON example:

```json
{
  "month": "202603",
  "domainsUnderManagement": 120,
  "domainAdds": 10,
  "domainTransfersIn": 3
}
```

### update_customer_domain_contacts

Returns: `UpdateCustomerDomainContactsResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::UpdateCustomerDomainContactsRequest;

let request = UpdateCustomerDomainContactsRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().update_customer_domain_contacts(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "status": "PENDING",
  "submittedAt": "2026-03-11T12:00:00Z"
}
```

### regenerate_customer_domain_auth_code

Returns: `RegenerateCustomerDomainAuthCodeResponse`

Request code:

```rust
use godaddy_rust::dto::domains::request::RegenerateCustomerDomainAuthCodeRequest;

let request = RegenerateCustomerDomainAuthCodeRequest::new(
    // Fill endpoint fields here
);
let response = client.domains().regenerate_customer_domain_auth_code(request).await?;
```

Response JSON example:

```json
{
  "domain": "example.com",
  "status": "ACTIVE",
  "expires": "2027-03-11T00:00:00Z",
  "authCode": "AUTHCODE"
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for domains endpoints.
