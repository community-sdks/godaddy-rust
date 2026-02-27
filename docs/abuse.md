# AbuseService

Abuse reporting and ticket lookup endpoints for phishing, malware, and related investigations.

## Accessor

```rust
let service = client.abuse();
```

## Endpoints

### get_tickets

Calls `GET /v1/abuse/tickets`.

```rust
let response = client.abuse().get_tickets(Some("sample".into()), Some(true.into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### create_ticket

Calls `POST /v1/abuse/tickets`.

```rust
let response = client.abuse().create_ticket(json!({"sample": true}).into()).await?;
```

```json
{}
```

### get_ticket_info

Calls `GET /v1/abuse/tickets/{ticketId}`.

```rust
let response = client.abuse().get_ticket_info("sample".into()).await?;
```

```json
{}
```

### get_tickets_v2

Calls `GET /v2/abuse/tickets`.

```rust
let response = client.abuse().get_tickets_v2(Some("sample".into()), Some(true.into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### create_ticket_v2

Calls `POST /v2/abuse/tickets`.

```rust
let response = client.abuse().create_ticket_v2(json!({"sample": true}).into()).await?;
```

```json
{}
```

### get_ticket_info_v2

Calls `GET /v2/abuse/tickets/{ticketId}`.

```rust
let response = client.abuse().get_ticket_info_v2("sample".into()).await?;
```

```json
{}
```
