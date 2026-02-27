# SubscriptionsService

Subscription listing and management endpoints for recurring products.

## Accessor

```rust
let service = client.subscriptions();
```

## Endpoints

### list

Calls `GET /v1/subscriptions`.

```rust
let response = client.subscriptions().list("header-value".into(), Some("header-value".into()), Some("header-value".into()), Some(vec!["sample"].into()), Some(vec!["sample"].into()), Some(1_i64.into()), Some(1_i64.into()), Some("sample".into())).await?;
```

```json
{}
```

### product_groups

Calls `GET /v1/subscriptions/productGroups`.

```rust
let response = client.subscriptions().product_groups("header-value".into(), Some("header-value".into())).await?;
```

```json
{}
```

### cancel

Calls `DELETE /v1/subscriptions/{subscriptionId}`.

```rust
let response = client.subscriptions().cancel(json!({"sample": true}).into(), "header-value".into(), Some("header-value".into())).await?;
```

```json
{}
```

### get

Calls `GET /v1/subscriptions/{subscriptionId}`.

```rust
let response = client.subscriptions().get(json!({"sample": true}).into(), "header-value".into(), Some("header-value".into())).await?;
```

```json
{}
```

### update

Calls `PATCH /v1/subscriptions/{subscriptionId}`.

```rust
let response = client.subscriptions().update(json!({"sample": true}).into(), "header-value".into(), json!({"sample": true}).into(), Some("header-value".into())).await?;
```

```json
{}
```
