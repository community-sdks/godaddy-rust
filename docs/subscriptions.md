# Subscriptions Service

Client accessor: `client.subscriptions()`

## Method Index

- [`list`](#list): `ListResponse`
- [`product_groups`](#product_groups): `ProductGroupsResponse`
- [`cancel`](#cancel): `CancelResponse`
- [`get`](#get): `GetResponse`
- [`update`](#update): `UpdateResponse`

## Methods

### list

Returns: `ListResponse`

Request code:

```rust
use godaddy_rust::dto::subscriptions::request::ListRequest;

let request = ListRequest::new(
    // Fill endpoint fields here
);
let response = client.subscriptions().list(request).await?;
```

Response JSON example:

```json
{
  "subscriptions": [
    {
      "subscriptionId": "sub_123456",
      "status": "ACTIVE",
      "renewAuto": true
    }
  ],
  "pagination": {
    "total": 1,
    "next": null
  }
}
```

### product_groups

Returns: `ProductGroupsResponse`

Request code:

```rust
use godaddy_rust::dto::subscriptions::request::ProductGroupsRequest;

let request = ProductGroupsRequest::new(
    // Fill endpoint fields here
);
let response = client.subscriptions().product_groups(request).await?;
```

Response JSON example:

```json
{
  "productGroups": [
    {
      "productGroupKey": "domains",
      "subscriptionCount": 1
    }
  ]
}
```

### cancel

Returns: `CancelResponse`

Request code:

```rust
use godaddy_rust::dto::subscriptions::request::CancelRequest;

let request = CancelRequest::new(
    // Fill endpoint fields here
);
let response = client.subscriptions().cancel(request).await?;
```

Response JSON example:

```json
{
  "subscriptionId": "sub_123456",
  "status": "CANCELLED"
}
```

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::subscriptions::request::GetRequest;

let request = GetRequest::new(
    // Fill endpoint fields here
);
let response = client.subscriptions().get(request).await?;
```

Response JSON example:

```json
{
  "subscriptionId": "sub_123456",
  "status": "ACTIVE",
  "label": "My Subscription",
  "renewAuto": true
}
```

### update

Returns: `UpdateResponse`

Request code:

```rust
use godaddy_rust::dto::subscriptions::request::UpdateRequest;

let request = UpdateRequest::new(
    // Fill endpoint fields here
);
let response = client.subscriptions().update(request).await?;
```

Response JSON example:

```json
{
  "subscriptionId": "sub_123456",
  "status": "ACTIVE"
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for subscriptions endpoints.
