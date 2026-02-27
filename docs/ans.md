# AnsService

Authoritative DNS record and nameserver management endpoints.

## Accessor

```rust
let service = client.ans();
```

## Endpoints

### search_ans_name

Calls `GET /v1/agents`.

```rust
let response = client.ans().search_ans_name(Some("sample".into()), Some("sample".into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into()), Some(1_i64.into())).await?;
```

```json
{}
```

### register_agent

Calls `POST /v1/agents/register`.

```rust
let response = client.ans().register_agent(json!({"sample": true}).into()).await?;
```

```json
{}
```

### resolve_ans_name

Calls `POST /v1/agents/resolution`.

```rust
let response = client.ans().resolve_ans_name(json!({"sample": true}).into()).await?;
```

```json
{}
```

### get_agent

Calls `GET /v1/agents/{agentId}`.

```rust
let response = client.ans().get_agent("sample".into()).await?;
```

```json
{}
```

### validate_registration

Calls `POST /v1/agents/{agentId}/verify-acme`.

```rust
let response = client.ans().validate_registration("sample".into()).await?;
```

```json
{}
```

### verify_dns_records

Calls `POST /v1/agents/{agentId}/verify-dns`.

```rust
let response = client.ans().verify_dns_records("sample".into()).await?;
```

```json
{}
```

### get_agent_identity_certificate_by_agent_id

Calls `GET /v1/agents/{agentId}/certificates/identity`.

```rust
let response = client.ans().get_agent_identity_certificate_by_agent_id("sample".into()).await?;
```

```json
{}
```

### submit_agent_identity_csr_by_agent_id

Calls `POST /v1/agents/{agentId}/certificates/identity`.

```rust
let response = client.ans().submit_agent_identity_csr_by_agent_id("sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### get_agent_server_certificate_by_agent_id

Calls `GET /v1/agents/{agentId}/certificates/server`.

```rust
let response = client.ans().get_agent_server_certificate_by_agent_id("sample".into()).await?;
```

```json
{}
```

### submit_agent_server_csr_by_agent_id

Calls `POST /v1/agents/{agentId}/certificates/server`.

```rust
let response = client.ans().submit_agent_server_csr_by_agent_id("sample".into(), json!({"sample": true}).into()).await?;
```

```json
{}
```

### get_agent_csr_status_by_agent_id

Calls `GET /v1/agents/{agentId}/csrs/{csrId}/status`.

```rust
let response = client.ans().get_agent_csr_status_by_agent_id("sample".into(), "sample".into()).await?;
```

```json
{}
```

### get_agent_events

Calls `GET /v1/agents/events`.

```rust
let response = client.ans().get_agent_events(Some("header-value".into()), Some("sample".into()), Some("sample".into()), Some(1_i64.into())).await?;
```

```json
{}
```
