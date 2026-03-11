# Abuse Service

Client accessor: `client.abuse()`

## Method Index

- [`get_tickets`](#get_tickets): `GetTicketsResponse`
- [`create_ticket`](#create_ticket): `CreateTicketResponse`
- [`get_ticket_info`](#get_ticket_info): `GetTicketInfoResponse`
- [`get_tickets_v2`](#get_tickets_v2): `GetTicketsV2Response`
- [`create_ticket_v2`](#create_ticket_v2): `CreateTicketV2Response`
- [`get_ticket_info_v2`](#get_ticket_info_v2): `GetTicketInfoV2Response`

## Methods

### get_tickets

Returns: `GetTicketsResponse`

Request code:

```rust
use godaddy_rust::dto::abuse::request::GetTicketsRequest;

let request = GetTicketsRequest {
    type_: Some("example.com".into()),
    closed: Some("example.com".into()),
    source_domain_or_ip: Some("example.com".into()),
    target: Some("example.com".into()),
    created_start: Some("example.com".into()),
    created_end: Some("example.com".into()),
    limit: Some(1_i64.into()),
    offset: Some(1_i64.into()),
};
let response = client.abuse().get_tickets(request).await?;
```

Response JSON example:

```json
{
  "ticketIds": [
    "TCK-100001"
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

### create_ticket

Returns: `CreateTicketResponse`

Request code:

```rust
use godaddy_rust::dto::abuse::request::CreateTicketRequest;

let request = CreateTicketRequest {
    body: serde_json::json!({"domain": "example.com"}).into(),
};
let response = client.abuse().create_ticket(request).await?;
```

Response JSON example:

```json
{
  "ticketId": "TCK-100001"
}
```

### get_ticket_info

Returns: `GetTicketInfoResponse`

Request code:

```rust
use godaddy_rust::dto::abuse::request::GetTicketInfoRequest;

let request = GetTicketInfoRequest {
    ticket_id: "123456789".into(),
};
let response = client.abuse().get_ticket_info(request).await?;
```

Response JSON example:

```json
{
  "ticketId": "TCK-100001",
  "type": "PHISHING",
  "source": "203.0.113.10",
  "target": "example.com",
  "closed": false,
  "notes": [
    {
      "message": "Initial report",
      "createdAt": "2026-03-11T12:00:00Z"
    }
  ]
}
```

### get_tickets_v2

Returns: `GetTicketsV2Response`

Request code:

```rust
use godaddy_rust::dto::abuse::request::GetTicketsV2Request;

let request = GetTicketsV2Request {
    type_: Some("example.com".into()),
    closed: Some("example.com".into()),
    source_domain_or_ip: Some("example.com".into()),
    target: Some("example.com".into()),
    created_start: Some("example.com".into()),
    created_end: Some("example.com".into()),
    limit: Some(1_i64.into()),
    offset: Some(1_i64.into()),
};
let response = client.abuse().get_tickets_v2(request).await?;
```

Response JSON example:

```json
{
  "ticketIds": [
    "TCK-100001"
  ],
  "pagination": {
    "total": 1,
    "start": 0,
    "limit": 25
  }
}
```

### create_ticket_v2

Returns: `CreateTicketV2Response`

Request code:

```rust
use godaddy_rust::dto::abuse::request::CreateTicketV2Request;

let request = CreateTicketV2Request {
    body: serde_json::json!({"domain": "example.com"}).into(),
};
let response = client.abuse().create_ticket_v2(request).await?;
```

Response JSON example:

```json
{
  "ticketId": "TCK-100001"
}
```

### get_ticket_info_v2

Returns: `GetTicketInfoV2Response`

Request code:

```rust
use godaddy_rust::dto::abuse::request::GetTicketInfoV2Request;

let request = GetTicketInfoV2Request {
    ticket_id: "123456789".into(),
};
let response = client.abuse().get_ticket_info_v2(request).await?;
```

Response JSON example:

```json
{
  "ticketId": "TCK-100001",
  "type": "PHISHING",
  "source": "203.0.113.10",
  "target": "example.com",
  "closed": false,
  "notes": [
    {
      "message": "Initial report",
      "createdAt": "2026-03-11T12:00:00Z"
    }
  ]
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for abuse endpoints.




