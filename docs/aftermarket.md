# Aftermarket Service

Client accessor: `client.aftermarket()`

## Method Index

- [`get_listings`](#get_listings): `GetListingsResponse`
- [`delete_listings`](#delete_listings): `DeleteListingsResponse`
- [`add_expiry_listings`](#add_expiry_listings): `AddExpiryListingsResponse`

## Methods

### get_listings

Returns: `GetListingsResponse`

Request code:

```rust
use godaddy_rust::dto::aftermarket::request::GetListingsRequest;

let request = GetListingsRequest::new(
    // Fill endpoint fields here
);
let response = client.aftermarket().get_listings(request).await?;
```

Response JSON example:

```json
{
  "listings": [
    {
      "fqdn": "example.com",
      "listingId": 1001,
      "listingStatus": "ACTIVE",
      "price": 2499,
      "currency": "USD"
    }
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 20
  }
}
```

### delete_listings

Returns: `DeleteListingsResponse`

Request code:

```rust
use godaddy_rust::dto::aftermarket::request::DeleteListingsRequest;

let request = DeleteListingsRequest::new(
    // Fill endpoint fields here
);
let response = client.aftermarket().delete_listings(request).await?;
```

Response JSON example:

```json
{
  "listingActionId": 900122
}
```

### add_expiry_listings

Returns: `AddExpiryListingsResponse`

Request code:

```rust
use godaddy_rust::dto::aftermarket::request::AddExpiryListingsRequest;

let request = AddExpiryListingsRequest::new(
    // Fill endpoint fields here
);
let response = client.aftermarket().add_expiry_listings(request).await?;
```

Response JSON example:

```json
{
  "listingActionId": 900122
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for aftermarket endpoints.
