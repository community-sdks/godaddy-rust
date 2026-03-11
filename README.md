# GoDaddy Rust SDK

A community-maintained Rust SDK for the provided GoDaddy OpenAPI specs.

## Installation

Add the crate to your project:

```toml
[dependencies]
community-sdks-godaddy = { path = "../community-sdks-godaddy" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

## Getting Started

```rust
use godaddy_rust::{Client, Config};
use godaddy_rust::dto::domains::request::TldsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    config.api_key = Some(String::from("your-api-key"));
    config.api_secret = Some(String::from("your-api-secret"));

    let client = Client::new(config);
    let response = client.domains().tlds(TldsRequest::new()).await?;

    println!("{}", response.raw);
    Ok(())
}
```


## Services

- [AbuseService](./docs/abuse.md): Abuse reporting and ticket lookup endpoints for phishing, malware, and related investigations.
- [AftermarketService](./docs/aftermarket.md): Aftermarket listing and sales endpoints for secondary-market domain workflows.
- [AgreementsService](./docs/agreements.md): Agreement retrieval endpoints for legal terms and consent workflows.
- [AnsService](./docs/ans.md): Authoritative DNS record and nameserver management endpoints.
- [AuctionsService](./docs/auctions.md): Auction listing discovery endpoints for GoDaddy Auctions inventory.
- [CertificatesService](./docs/certificates.md): SSL certificate purchase, validation, lifecycle, and revocation endpoints.
- [CountriesService](./docs/countries.md): Country and market metadata endpoints used across purchase flows.
- [DomainsService](./docs/domains.md): Domain availability, purchase, management, transfer, and DNS endpoints.
- [OrdersService](./docs/orders.md): Order lookup endpoints for commerce and fulfillment status.
- [ParkingService](./docs/parking.md): Domain parking optimization and template management endpoints.
- [ShoppersService](./docs/shoppers.md): Shopper profile, account, and delegated access endpoints.
- [SubscriptionsService](./docs/subscriptions.md): Subscription listing and management endpoints for recurring products.
