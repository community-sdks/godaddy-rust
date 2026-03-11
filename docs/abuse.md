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

let request = GetTicketsRequest::new(
    // Fill endpoint fields here
);
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

let request = CreateTicketRequest::new(
    // Fill endpoint fields here
);
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

let request = GetTicketInfoRequest::new(
    // Fill endpoint fields here
);
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

let request = GetTicketsV2Request::new(
    // Fill endpoint fields here
);
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

let request = CreateTicketV2Request::new(
    // Fill endpoint fields here
);
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

let request = GetTicketInfoV2Request::new(
    // Fill endpoint fields here
);
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
