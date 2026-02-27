# AuctionsService

Auction listing discovery endpoints for GoDaddy Auctions inventory.

## Accessor

```rust
let service = client.auctions();
```

## Endpoints

### place_bids

Calls `POST /v1/customers/{customerId}/aftermarket/listings/bids`.

```rust
let response = client.auctions().place_bids("sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```
