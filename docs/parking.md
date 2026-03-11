# Parking Service

Client accessor: `client.parking()`

## Method Index

- [`get_metrics`](#get_metrics): `GetMetricsResponse`
- [`get_metrics_by_domain`](#get_metrics_by_domain): `GetMetricsByDomainResponse`

## Methods

### get_metrics

Returns: `GetMetricsResponse`

Request code:

```rust
use godaddy_rust::dto::parking::request::GetMetricsRequest;

let request = GetMetricsRequest {
    customer_id: "123456789".into(),
    period_start_ptz: Some(1_i64.into()),
    period_end_ptz: Some(1_i64.into()),
    limit: Some(1_i64.into()),
    offset: Some(1_i64.into()),
    x_request_id: Some("header-value".into()),
};
let response = client.parking().get_metrics(request).await?;
```

Response JSON example:

```json
{
  "currencyId": "USD",
  "metrics": [
    {
      "periodPtz": "2026-03-11",
      "visitCount": 120,
      "adClickCount": 14,
      "revenue": 3450
    }
  ],
  "pagination": {
    "total": 1,
    "next": null
  }
}
```

### get_metrics_by_domain

Returns: `GetMetricsByDomainResponse`

Request code:

```rust
use godaddy_rust::dto::parking::request::GetMetricsByDomainRequest;

let request = GetMetricsByDomainRequest {
    customer_id: "123456789".into(),
    start_date: "example.com".into(),
    end_date: "example.com".into(),
    domains: Some(vec!["example.com"].into()),
    domain_like: Some("example.com".into()),
    portfolio_id: Some("example.com".into()),
    limit: Some(1_i64.into()),
    offset: Some(1_i64.into()),
    x_request_id: Some("header-value".into()),
};
let response = client.parking().get_metrics_by_domain(request).await?;
```

Response JSON example:

```json
{
  "currencyId": "USD",
  "startDate": "2026-03-01",
  "endDate": "2026-03-11",
  "metrics": [
    {
      "domain": "example.com",
      "visitCount": 120,
      "pageViewCount": 200,
      "revenue": 3450
    }
  ],
  "pagination": {
    "total": 1,
    "next": null
  }
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for parking endpoints.




