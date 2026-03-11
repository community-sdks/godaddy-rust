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

let request = GetMetricsRequest::new(
    // Fill endpoint fields here
);
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

let request = GetMetricsByDomainRequest::new(
    // Fill endpoint fields here
);
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
