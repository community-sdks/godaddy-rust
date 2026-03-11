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

let request = CreateSubaccountRequest::new(
    // Fill endpoint fields here
);
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

let request = GetRequest::new(
    // Fill endpoint fields here
);
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

let request = UpdateRequest::new(
    // Fill endpoint fields here
);
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

let request = DeleteRequest::new(
    // Fill endpoint fields here
);
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

let request = GetStatusRequest::new(
    // Fill endpoint fields here
);
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

let request = ChangePasswordRequest::new(
    // Fill endpoint fields here
);
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
