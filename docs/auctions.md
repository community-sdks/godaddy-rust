# Auctions Service

Client accessor: `client.auctions()`

## Method Index

- [`place_bids`](#place_bids): `PlaceBidsResponse`

## Methods

### place_bids

Returns: `PlaceBidsResponse`

Request code:

```rust
use godaddy_rust::dto::auctions::request::PlaceBidsRequest;

let request = PlaceBidsRequest::new(
    // Fill endpoint fields here
);
let response = client.auctions().place_bids(request).await?;
```

Response JSON example:

```json
{
  "listingId": 200000,
  "bidId": "bid_001",
  "bidAmountUsd": 1500,
  "status": "ACTIVE",
  "isHighestBidder": true
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for auctions endpoints.
