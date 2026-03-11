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

let request = ListRequest {
    x_app_key: "header-value".into(),
    period_start: Some(1_i64.into()),
    period_end: Some(1_i64.into()),
    domain: Some("example.com".into()),
    product_group_id: Some("example.com".into()),
    payment_profile_id: Some("example.com".into()),
    parent_order_id: Some("example.com".into()),
    offset: Some(1_i64.into()),
    limit: Some(1_i64.into()),
    sort: Some("example.com".into()),
    x_shopper_id: Some("header-value".into()),
};
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

let request = GetRequest {
    order_id: "123456789".into(),
    x_app_key: "header-value".into(),
    x_shopper_id: Some("header-value".into()),
    x_market_id: Some("header-value".into()),
};
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




