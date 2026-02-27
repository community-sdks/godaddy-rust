# ParkingService

Domain parking optimization and template management endpoints.

## Accessor

```rust
let service = client.parking();
```

## Endpoints

### get_metrics

Calls `GET /v1/customers/{customerId}/parking/metrics`.

```rust
let response = client.parking().get_metrics("sample".into(), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into()), Some("header-value".into())).await?;
```

```json
{}
```

### get_metrics_by_domain

Calls `GET /v1/customers/{customerId}/parking/metricsByDomain`.

```rust
let response = client.parking().get_metrics_by_domain("sample".into(), "sample".into(), "sample".into(), Some(vec!["sample"].into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into()), Some("header-value".into())).await?;
```

```json
{}
```
