# AgreementsService

Agreement retrieval endpoints for legal terms and consent workflows.

## Accessor

```rust
let service = client.agreements();
```

## Endpoints

### get

Calls `GET /v1/agreements`.

```rust
let response = client.agreements().get(vec!["sample"].into(), Some("header-value".into()), Some("header-value".into())).await?;
```

```json
{}
```
