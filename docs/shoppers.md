# Shoppers Service

Client accessor: `client.shoppers()`

## Method Index

- [`create_subaccount`](#create_subaccount): `CreateSubaccountResponse`
- [`get`](#get): `GetResponse`
- [`update`](#update): `UpdateResponse`
- [`delete`](#delete): `DeleteResponse`
- [`get_status`](#get_status): `GetStatusResponse`
- [`change_password`](#change_password): `ChangePasswordResponse`

## Methods

### create_subaccount

Returns: `CreateSubaccountResponse`

Request code:

```rust
use godaddy_rust::dto::shoppers::request::CreateSubaccountRequest;

let request = CreateSubaccountRequest {
    subaccount: "example.com".into(),
};
let response = client.shoppers().create_subaccount(request).await?;
```

Response JSON example:

```json
{
  "shopperId": "987654321",
  "customerId": "123456789"
}
```

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::shoppers::request::GetRequest;

let request = GetRequest {
    shopper_id: "123456789".into(),
    includes: Some(vec!["example.com"].into()),
};
let response = client.shoppers().get(request).await?;
```

Response JSON example:

```json
{
  "shopperId": "987654321",
  "nameFirst": "Jane",
  "nameLast": "Doe",
  "email": "admin@example.com",
  "marketId": "en-US",
  "customerId": "123456789"
}
```

### update

Returns: `UpdateResponse`

Request code:

```rust
use godaddy_rust::dto::shoppers::request::UpdateRequest;

let request = UpdateRequest {
    shopper_id: "123456789".into(),
    shopper: "example.com".into(),
};
let response = client.shoppers().update(request).await?;
```

Response JSON example:

```json
{
  "shopperId": "987654321",
  "customerId": "123456789"
}
```

### delete

Returns: `DeleteResponse`

Request code:

```rust
use godaddy_rust::dto::shoppers::request::DeleteRequest;

let request = DeleteRequest {
    shopper_id: "123456789".into(),
    audit_client_ip: "example.com".into(),
};
let response = client.shoppers().delete(request).await?;
```

Response JSON example:

```json
{
  "deleted": true
}
```

### get_status

Returns: `GetStatusResponse`

Request code:

```rust
use godaddy_rust::dto::shoppers::request::GetStatusRequest;

let request = GetStatusRequest {
    shopper_id: "123456789".into(),
    audit_client_ip: "example.com".into(),
};
let response = client.shoppers().get_status(request).await?;
```

Response JSON example:

```json
{
  "billingState": "ACTIVE"
}
```

### change_password

Returns: `ChangePasswordResponse`

Request code:

```rust
use godaddy_rust::dto::shoppers::request::ChangePasswordRequest;

let request = ChangePasswordRequest {
    shopper_id: "123456789".into(),
    secret: "example.com".into(),
};
let response = client.shoppers().change_password(request).await?;
```

Response JSON example:

```json
{
  "shopperId": "987654321",
  "customerId": "123456789"
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for shoppers endpoints.




