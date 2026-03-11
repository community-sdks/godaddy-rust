# ANS Service

Client accessor: `client.ans()`

## Method Index

- [`search`](#search): `SearchResponse`
- [`register`](#register): `RegisterResponse`
- [`resolve`](#resolve): `ResolveResponse`
- [`get`](#get): `GetResponse`
- [`revoke`](#revoke): `RevokeResponse`
- [`verify_acme`](#verify_acme): `VerifyAcmeResponse`
- [`verify_dns`](#verify_dns): `VerifyDnsResponse`
- [`get_identity_certificates`](#get_identity_certificates): `GetIdentityCertificatesResponse`
- [`submit_identity_csr`](#submit_identity_csr): `SubmitIdentityCsrResponse`
- [`get_server_certificates`](#get_server_certificates): `GetServerCertificatesResponse`
- [`submit_server_csr`](#submit_server_csr): `SubmitServerCsrResponse`
- [`get_csr_status`](#get_csr_status): `GetCsrStatusResponse`
- [`events`](#events): `EventsResponse`

## Methods

### search

Returns: `SearchResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::SearchRequest;

let request = SearchRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().search(request).await?;
```

Response JSON example:

```json
{
  "agents": [
    {
      "agentId": "agt_001",
      "displayName": "Checkout Agent",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ],
  "totalCount": 1,
  "returnedCount": 1,
  "limit": 10,
  "offset": 0,
  "hasMore": false
}
```

### register

Returns: `RegisterResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::RegisterRequest;

let request = RegisterRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().register(request).await?;
```

Response JSON example:

```json
{
  "agentId": "agt_001",
  "status": "ACTIVE",
  "displayName": "Checkout Agent",
  "endpoints": [
    {
      "url": "https://agent.example.com",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ]
}
```

### resolve

Returns: `ResolveResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::ResolveRequest;

let request = ResolveRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().resolve(request).await?;
```

Response JSON example:

```json
{
  "agentId": "agt_001",
  "status": "ACTIVE",
  "displayName": "Checkout Agent",
  "endpoints": [
    {
      "url": "https://agent.example.com",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ]
}
```

### get

Returns: `GetResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::GetRequest;

let request = GetRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().get(request).await?;
```

Response JSON example:

```json
{
  "agentId": "agt_001",
  "status": "ACTIVE",
  "displayName": "Checkout Agent",
  "endpoints": [
    {
      "url": "https://agent.example.com",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ]
}
```

### revoke

Returns: `RevokeResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::RevokeRequest;

let request = RevokeRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().revoke(request).await?;
```

Response JSON example:

```json
{
  "agentId": "agt_001",
  "status": "ACTIVE",
  "displayName": "Checkout Agent",
  "endpoints": [
    {
      "url": "https://agent.example.com",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ]
}
```

### verify_acme

Returns: `VerifyAcmeResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::VerifyAcmeRequest;

let request = VerifyAcmeRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().verify_acme(request).await?;
```

Response JSON example:

```json
{
  "agentId": "agt_001",
  "status": "ACTIVE",
  "displayName": "Checkout Agent",
  "endpoints": [
    {
      "url": "https://agent.example.com",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ]
}
```

### verify_dns

Returns: `VerifyDnsResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::VerifyDnsRequest;

let request = VerifyDnsRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().verify_dns(request).await?;
```

Response JSON example:

```json
{
  "agentId": "agt_001",
  "status": "ACTIVE",
  "displayName": "Checkout Agent",
  "endpoints": [
    {
      "url": "https://agent.example.com",
      "protocol": "MCP",
      "status": "ACTIVE"
    }
  ]
}
```

### get_identity_certificates

Returns: `GetIdentityCertificatesResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::GetIdentityCertificatesRequest;

let request = GetIdentityCertificatesRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().get_identity_certificates(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123",
      "status": "ISSUED",
      "expiresAt": "2027-03-11T00:00:00Z"
    }
  ]
}
```

### submit_identity_csr

Returns: `SubmitIdentityCsrResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::SubmitIdentityCsrRequest;

let request = SubmitIdentityCsrRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().submit_identity_csr(request).await?;
```

Response JSON example:

```json
{
  "csrId": "csr_123"
}
```

### get_server_certificates

Returns: `GetServerCertificatesResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::GetServerCertificatesRequest;

let request = GetServerCertificatesRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().get_server_certificates(request).await?;
```

Response JSON example:

```json
{
  "certificates": [
    {
      "certificateId": "crt_123",
      "status": "ISSUED",
      "expiresAt": "2027-03-11T00:00:00Z"
    }
  ]
}
```

### submit_server_csr

Returns: `SubmitServerCsrResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::SubmitServerCsrRequest;

let request = SubmitServerCsrRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().submit_server_csr(request).await?;
```

Response JSON example:

```json
{
  "csrId": "csr_123"
}
```

### get_csr_status

Returns: `GetCsrStatusResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::GetCsrStatusRequest;

let request = GetCsrStatusRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().get_csr_status(request).await?;
```

Response JSON example:

```json
{
  "csrId": "csr_123",
  "status": "PENDING",
  "updatedAt": "2026-03-11T12:00:00Z"
}
```

### events

Returns: `EventsResponse`

Request code:

```rust
use godaddy_rust::dto::ans::request::EventsRequest;

let request = EventsRequest::new(
    // Fill endpoint fields here
);
let response = client.ans().events(request).await?;
```

Response JSON example:

```json
{
  "items": [
    {
      "eventId": "evt_1",
      "type": "AGENT_UPDATED",
      "createdAt": "2026-03-11T12:00:00Z"
    }
  ]
}
```

## Exceptions

Service-specific exceptions are exposed under `godaddy_rust::error` for ans endpoints.
