# CountriesService

Country and market metadata endpoints used across purchase flows.

## Accessor

```rust
let service = client.countries();
```

## Endpoints

### get_countries

Calls `GET /v1/countries`.

```rust
let response = client.countries().get_countries("sample".into()).await?;
```

```json
{}
```

### get_country

Calls `GET /v1/countries/{countryKey}`.

```rust
let response = client.countries().get_country("sample".into(), "sample".into()).await?;
```

```json
{}
```
