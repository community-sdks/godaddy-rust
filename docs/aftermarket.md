# AftermarketService

Aftermarket listing and sales endpoints for secondary-market domain workflows.

## Accessor

```rust
let service = client.aftermarket();
```

## Endpoints

### get_listings

Calls `GET /v1/customers/{customerId}/auctions/listings`.

```rust
let response = client.aftermarket().get_listings("sample".into(), Some(vec!["sample"].into()), Some(vec!["sample"].into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### delete_listings

Calls `DELETE /v1/aftermarket/listings`.

```rust
let response = client.aftermarket().delete_listings(vec!["sample"].into()).await?;
```

```json
{}
```

### add_expiry_listings

Calls `POST /v1/aftermarket/listings/expiry`.

```rust
let response = client.aftermarket().add_expiry_listings(vec!["sample"].into()).await?;
```

```json
{}
```
