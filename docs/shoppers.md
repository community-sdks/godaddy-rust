# ShoppersService

Shopper profile, account, and delegated access endpoints.

## Accessor

```rust
let service = client.shoppers();
```

## Endpoints

### create_subaccount

Calls `POST /v1/shoppers/subaccount`.

```rust
let response = client.shoppers().create_subaccount(json!({"sample": true}).into()).await?;
```

```json
{}
```

### get

Calls `GET /v1/shoppers/{shopperId}`.

```rust
let response = client.shoppers().get(json!({"sample": true}).into(), Some(vec!["sample"].into())).await?;
```

```json
{}
```

### update

Calls `POST /v1/shoppers/{shopperId}`.

```rust
let response = client.shoppers().update(json!({"sample": true}).into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### delete

Calls `DELETE /v1/shoppers/{shopperId}`.

```rust
let response = client.shoppers().delete(json!({"sample": true}).into(), "sample".into()).await?;
```

```json
{}
```

### get_status

Calls `GET /v1/shoppers/{shopperId}/status`.

```rust
let response = client.shoppers().get_status(json!({"sample": true}).into(), "sample".into()).await?;
```

```json
{}
```

### change_password

Calls `PUT /v1/shoppers/{shopperId}/factors/password`.

```rust
let response = client.shoppers().change_password(json!({"sample": true}).into(), json!({"sample": true}).into()).await?;
```

```json
{}
```
