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
use godaddy_rust::dto::ans::request::SearchAnsNameRequest;

let request = SearchAnsNameRequest {
    agent_display_name: Some("example.com".into()),
    version: Some("example.com".into()),
    agent_host: Some("example.com".into()),
    protocol: Some("example.com".into()),
    limit: Some(1_i64.into()),
    offset: Some(1_i64.into()),
};
let response = client.ans().search_ans_name(request).await?;
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
use godaddy_rust::dto::ans::request::RegisterAgentRequest;

let request = RegisterAgentRequest {
    body: serde_json::json!({"domain": "example.com"}).into(),
};
let response = client.ans().register_agent(request).await?;
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
use godaddy_rust::dto::ans::request::ResolveAnsNameRequest;

let request = ResolveAnsNameRequest {
    body: serde_json::json!({"domain": "example.com"}).into(),
};
let response = client.ans().resolve_ans_name(request).await?;
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
use godaddy_rust::dto::ans::request::GetAgentRequest;

let request = GetAgentRequest {
    agent_id: "123456789".into(),
};
let response = client.ans().get_agent(request).await?;
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
use godaddy_rust::dto::ans::request::ValidateRegistrationRequest;

let request = ValidateRegistrationRequest {
    "123456789".into(),
};
let response = client.ans().validate_registration(request).await?;
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
use godaddy_rust::dto::ans::request::VerifyDnsRecordsRequest;

let request = VerifyDnsRecordsRequest {
    agent_id: "123456789".into(),
};
let response = client.ans().verify_dns_records(request).await?;
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
use godaddy_rust::dto::ans::request::VerifyDnsRecordsRequest;

let request = VerifyDnsRecordsRequest {
    agent_id: "123456789".into(),
};
let response = client.ans().verify_dns_records(request).await?;
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
use godaddy_rust::dto::ans::request::GetAgentIdentityCertificateByAgentIdRequest;

let request = GetAgentIdentityCertificateByAgentIdRequest {
    agent_id: "123456789".into(),
};
let response = client.ans().get_agent_identity_certificate_by_agent_id(request).await?;
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
use godaddy_rust::dto::ans::request::SubmitAgentIdentityCsrByAgentIdRequest;

let request = SubmitAgentIdentityCsrByAgentIdRequest {
    agent_id: "123456789".into(),
    body: serde_json::json!({"domain": "example.com"}).into(),
};
let response = client.ans().submit_agent_identity_csr_by_agent_id(request).await?;
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
use godaddy_rust::dto::ans::request::GetAgentServerCertificateByAgentIdRequest;

let request = GetAgentServerCertificateByAgentIdRequest {
    agent_id: "123456789".into(),
};
let response = client.ans().get_agent_server_certificate_by_agent_id(request).await?;
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
use godaddy_rust::dto::ans::request::SubmitAgentServerCsrByAgentIdRequest;

let request = SubmitAgentServerCsrByAgentIdRequest {
    agent_id: "123456789".into(),
    body: serde_json::json!({"domain": "example.com"}).into(),
};
let response = client.ans().submit_agent_server_csr_by_agent_id(request).await?;
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
use godaddy_rust::dto::ans::request::GetAgentCsrStatusByAgentIdRequest;

let request = GetAgentCsrStatusByAgentIdRequest {
    agent_id: "123456789".into(),
    csr_id: "123456789".into(),
};
let response = client.ans().get_agent_csr_status_by_agent_id(request).await?;
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
use godaddy_rust::dto::ans::request::GetAgentEventsRequest;

let request = GetAgentEventsRequest {
    x_request_id: Some("header-value".into()),
    provider_id: Some("example.com".into()),
    last_log_id: Some("example.com".into()),
    limit: Some(1_i64.into()),
};
let response = client.ans().get_agent_events(request).await?;
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




