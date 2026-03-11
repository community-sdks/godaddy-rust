# Orders Service

Client accessor: `client.orders()`

## Method Index

- [`list`](#list): `ListResponse`
- [`get`](#get): `GetResponse`

## Methods

### list

Returns: `ListResponse`

Request code:

```rust
use godaddy_rust::dto::orders::request::ListRequest;

let request = ListRequest::new(
    // Fill endpoint fields here
);
let response = client.orders().list(request).await?;
```

Response JSON example:

```json
{
  "orders": [
    {
      "orderId": "1234567890",
      "currency": "USD",
      "createdAt": "2026-03-11T12:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "next": null
  }
}
```

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::orders::request::GetRequest;

let request = GetRequest::new(
    // Fill endpoint fields here
);
let response = client.orders().get(request).await?;
```

Response JSON example:

```json
{
  "orderId": "1234567890",
  "currency": "USD",
  "createdAt": "2026-03-11T12:00:00Z",
  "status": "PENDING",
  "pricing": {
    "total": "14.99"
  },
  "items": [
    {
      "itemId": "line-1",
      "label": "example.com",
      "status": "PENDING"
    }
  ]
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for orders endpoints.
