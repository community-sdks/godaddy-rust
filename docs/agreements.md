# Agreements Service

Client accessor: `client.agreements()`

## Method Index

- [`get`](#get): `GetResponse`

## Methods

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::agreements::request::GetRequest;

let request = GetRequest {
    keys: vec!["example.com"].into(),
    x_private_label_id: Some("header-value".into()),
    x_market_id: Some("header-value".into()),
};
let response = client.agreements().get(request).await?;
```

Response JSON example:

```json
{
  "agreementKey": "DNRA",
  "title": "Domain Name Registration Agreement",
  "url": "https://www.godaddy.com/legal/agreements/domain-registration"
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for agreements endpoints.




