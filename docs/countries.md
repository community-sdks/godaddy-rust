# Countries Service

Client accessor: `client.countries()`

## Method Index

- [`get_countries`](#get_countries): `GetCountriesResponse`
- [`get_country`](#get_country): `GetCountryResponse`

## Methods

### get_countries

Returns: `GetCountriesResponse`

Request code:

```rust
use godaddy_rust::dto::countries::request::GetCountriesRequest;

let request = GetCountriesRequest {
    market_id: "123456789".into(),
};
let response = client.countries().get_countries(request).await?;
```

Response JSON example:

```json
{
  "countryKey": "US",
  "label": "United States",
  "callingCode": "1"
}
```

### get_country

Returns: `GetCountryResponse`

Request code:

```rust
use godaddy_rust::dto::countries::request::GetCountryRequest;

let request = GetCountryRequest {
    country_key: "example.com".into(),
    market_id: "123456789".into(),
};
let response = client.countries().get_country(request).await?;
```

Response JSON example:

```json
{
  "countryKey": "US",
  "label": "United States",
  "states": [
    {
      "stateKey": "AZ",
      "label": "Arizona"
    }
  ]
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for countries endpoints.




