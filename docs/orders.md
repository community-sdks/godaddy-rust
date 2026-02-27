# OrdersService

Order lookup endpoints for commerce and fulfillment status.

## Accessor

```rust
let service = client.orders();
```

## Endpoints

### list

Calls `GET /v1/orders`.

```rust
let response = client.orders().list("header-value".into(), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into()), Some("sample".into()), Some("header-value".into())).await?;
```

```json
{}
```

### get

Calls `GET /v1/orders/{orderId}`.

```rust
let response = client.orders().get("sample".into(), "header-value".into(), Some("header-value".into()), Some("header-value".into())).await?;
```

```json
{}
```
