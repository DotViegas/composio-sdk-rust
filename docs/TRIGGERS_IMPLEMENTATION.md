# Triggers Implementation

This document describes the implementation of triggers functionality in the Composio Rust SDK, following the official Python SDK patterns.

## Overview

Triggers are event listeners that notify your application when specific events occur in connected services. The Rust SDK provides complete trigger management capabilities including:

- Listing and discovering trigger types
- Creating trigger instances
- Managing trigger lifecycle (enable/disable/delete)
- Webhook signature verification
- Support for multiple webhook payload versions (V1, V2, V3)

## Architecture

### Data Structures

All trigger-related data structures are defined in `src/models/triggers.rs`:

#### Core Types

- **`WebhookVersion`**: Enum for webhook payload versions (V1, V2, V3)
- **`TriggerEvent`**: Normalized trigger event structure
- **`TriggerMetadata`**: Event metadata including trigger and account information
- **`TriggerConnectedAccount`**: Connected account details in trigger context
- **`TriggerType`**: Template defining what event to listen for
- **`TriggerInstance`**: Active listener for a specific user and connected account

#### Request/Response Types

- **`TriggerTypeListParams`**: Parameters for listing trigger types
- **`TriggerTypeListResponse`**: Paginated list of trigger types
- **`TriggerInstanceListParams`**: Parameters for listing active triggers
- **`TriggerInstanceListResponse`**: Paginated list of trigger instances
- **`TriggerCreateParams`**: Parameters for creating a trigger instance
- **`TriggerCreateResponse`**: Response from creating a trigger
- **`WebhookVerifyParams`**: Parameters for webhook verification
- **`VerifyWebhookResult`**: Result of webhook verification

### Client Methods

All trigger methods are implemented in `src/client.rs` as part of the `ComposioClient`:

#### 1. `list_trigger_types()`

Lists available trigger types with filtering and pagination.

```rust
pub async fn list_trigger_types(
    &self,
    params: TriggerTypeListParams,
) -> Result<TriggerTypeListResponse, ComposioError>
```

**Features:**
- Filter by toolkit slugs
- Pagination support with cursor
- Toolkit version specification

**Example:**
```rust
let params = TriggerTypeListParams {
    toolkit_slugs: Some(vec!["github".to_string()]),
    limit: Some(10),
    ..Default::default()
};

let response = client.list_trigger_types(params).await?;
for trigger_type in response.items {
    println!("{}: {}", trigger_type.slug, trigger_type.name);
}
```

#### 2. `get_trigger_type()`

Retrieves detailed information about a specific trigger type.

```rust
pub async fn get_trigger_type(
    &self,
    slug: impl Into<String>,
) -> Result<TriggerType, ComposioError>
```

**Returns:**
- Trigger configuration schema
- Payload schema
- Instructions for setup
- Trigger type (webhook or poll)

**Example:**
```rust
let trigger = client.get_trigger_type("GITHUB_COMMIT_EVENT").await?;
println!("Type: {}", trigger.trigger_type);
println!("Config schema: {}", serde_json::to_string_pretty(&trigger.config)?);
```

#### 3. `list_active_triggers()`

Lists active trigger instances with comprehensive filtering.

```rust
pub async fn list_active_triggers(
    &self,
    params: TriggerInstanceListParams,
) -> Result<TriggerInstanceListResponse, ComposioError>
```

**Filtering options:**
- Trigger IDs
- Trigger names
- Auth config IDs
- Connected account IDs
- Show disabled triggers
- Pagination

**Example:**
```rust
let params = TriggerInstanceListParams {
    trigger_names: Some(vec!["GITHUB_COMMIT_EVENT".to_string()]),
    show_disabled: Some(false),
    limit: Some(20),
    ..Default::default()
};

let response = client.list_active_triggers(params).await?;
```

#### 4. `create_trigger()`

Creates a new trigger instance for a user.

```rust
pub async fn create_trigger(
    &self,
    params: TriggerCreateParams,
) -> Result<TriggerCreateResponse, ComposioError>
```

**Features:**
- Auto-find connected account by user_id
- Or specify connected_account_id directly
- Custom trigger configuration
- Toolkit version support

**Example:**
```rust
let mut config = HashMap::new();
config.insert("repo".to_string(), serde_json::json!("composio"));
config.insert("owner".to_string(), serde_json::json!("composio"));

let params = TriggerCreateParams {
    slug: "GITHUB_COMMIT_EVENT".to_string(),
    user_id: Some("user_123".to_string()),
    connected_account_id: None,
    trigger_config: Some(config),
    toolkit_versions: None,
};

let response = client.create_trigger(params).await?;
println!("Created trigger: {}", response.id);
```

#### 5. `delete_trigger()`

Permanently deletes a trigger instance.

```rust
pub async fn delete_trigger(
    &self,
    trigger_id: impl Into<String>,
) -> Result<(), ComposioError>
```

**Note:** This operation cannot be undone.

**Example:**
```rust
client.delete_trigger("ti_abc123").await?;
```

#### 6. `enable_trigger()`

Enables a previously disabled trigger instance.

```rust
pub async fn enable_trigger(
    &self,
    trigger_id: impl Into<String>,
) -> Result<(), ComposioError>
```

**Example:**
```rust
client.enable_trigger("ti_abc123").await?;
```

#### 7. `disable_trigger()`

Temporarily disables a trigger instance without deleting it.

```rust
pub async fn disable_trigger(
    &self,
    trigger_id: impl Into<String>,
) -> Result<(), ComposioError>
```

**Example:**
```rust
client.disable_trigger("ti_abc123").await?;
```

#### 8. `verify_webhook()`

Verifies an incoming webhook payload and signature.

```rust
pub fn verify_webhook(
    &self,
    params: WebhookVerifyParams,
) -> Result<VerifyWebhookResult, ComposioError>
```

**Verification process:**
1. Validates timestamp is within tolerance window
2. Verifies HMAC-SHA256 signature
3. Parses payload and detects version (V1, V2, or V3)
4. Normalizes payload to standard `TriggerEvent` format

**Example:**
```rust
// In your webhook handler (e.g., Actix-web, Axum, etc.)
let params = WebhookVerifyParams {
    id: request.headers().get("webhook-id").unwrap().to_str()?.to_string(),
    payload: body_string,
    signature: request.headers().get("webhook-signature").unwrap().to_str()?.to_string(),
    timestamp: request.headers().get("webhook-timestamp").unwrap().to_str()?.to_string(),
    secret: env::var("COMPOSIO_WEBHOOK_SECRET")?,
    tolerance: Some(300), // 5 minutes
};

let result = client.verify_webhook(params)?;
println!("Received {} event", result.payload.trigger_slug);
```

## Webhook Verification Details

### Signature Algorithm

Composio uses HMAC-SHA256 for webhook signatures:

1. **Signing string format:** `{webhook_id}.{webhook_timestamp}.{payload}`
2. **HMAC key:** Webhook secret from Composio dashboard
3. **Encoding:** Base64
4. **Header format:** `v1,{base64_signature}`

### Webhook Payload Versions

#### V1 (Legacy)
```json
{
  "trigger_name": "GITHUB_COMMIT_EVENT",
  "connection_id": "conn_123",
  "trigger_id": "ti_456",
  "payload": { ... },
  "log_id": "log_789"
}
```

#### V2 (Legacy)
```json
{
  "type": "github_commit",
  "timestamp": "2024-01-01T00:00:00Z",
  "log_id": "log_789",
  "data": {
    "connection_id": "conn_123",
    "trigger_id": "ti_456",
    "user_id": "user_123",
    ...
  }
}
```

#### V3 (Current - Recommended)
```json
{
  "id": "evt_123",
  "type": "composio.trigger.message",
  "timestamp": "2024-01-01T00:00:00Z",
  "metadata": {
    "trigger_id": "ti_456",
    "trigger_slug": "GITHUB_COMMIT_EVENT",
    "user_id": "user_123",
    "connected_account_id": "ca_789",
    "auth_config_id": "ac_101",
    "log_id": "log_202"
  },
  "data": { ... }
}
```

**V3 Benefits:**
- Cleaner separation of metadata and event data
- Support for non-trigger events (e.g., `composio.connected_account.expired`)
- Consistent structure across all event types
- Better type safety

### Payload Normalization

All webhook versions are normalized to a standard `TriggerEvent` structure:

```rust
pub struct TriggerEvent {
    pub id: String,
    pub uuid: String,
    pub user_id: String,
    pub toolkit_slug: String,
    pub trigger_slug: String,
    pub metadata: TriggerMetadata,
    pub payload: Option<serde_json::Value>,
    pub original_payload: Option<serde_json::Value>,
}
```

This ensures consistent handling regardless of webhook version.

## Implementation Notes

### Differences from Python SDK

1. **Synchronous webhook verification**: The `verify_webhook()` method is synchronous (not async) since it only performs local cryptographic operations.

2. **No subscription support**: The Python SDK's `subscribe()` method (WebSocket-based) is not implemented in the Rust SDK. Use webhooks instead for production applications.

3. **Error handling**: Uses Rust's `Result` type instead of exceptions.

4. **Type safety**: All parameters and responses are strongly typed with Rust structs.

### Security Considerations

1. **Constant-time comparison**: Signature verification uses timing-safe comparison to prevent timing attacks.

2. **Timestamp validation**: Configurable tolerance window (default: 300 seconds) prevents replay attacks.

3. **Secret management**: Webhook secrets should be stored securely (environment variables, secret managers).

4. **HTTPS only**: Webhook URLs must use HTTPS in production.

## Testing

Run the examples:

```bash
# Set your API key
export COMPOSIO_API_KEY="your_api_key_here"

# Run triggers example
cargo run --example triggers_usage
```

## API Endpoints Used

- `GET /api/v3/triggers_types` - List trigger types
- `GET /api/v3/triggers_types/{slug}` - Get trigger type
- `GET /api/v3/trigger_instances/active` - List active triggers
- `POST /api/v3/trigger_instances/{slug}/upsert` - Create/update trigger
- `DELETE /api/v3/trigger_instances/manage/{id}` - Delete trigger
- `PATCH /api/v3/trigger_instances/manage/{id}` - Enable/disable trigger

## Dependencies

The webhook verification functionality requires:

- `hmac = "0.12"` - HMAC implementation
- `sha2 = "0.10"` - SHA-256 hashing
- `base64 = "0.21"` - Base64 encoding/decoding

## Future Enhancements

Potential improvements for future versions:

1. **WebSocket subscription support**: Implement real-time trigger subscription using WebSockets (similar to Python SDK's `subscribe()` method).

2. **Trigger event filtering**: Add client-side filtering for trigger events based on custom criteria.

3. **Batch operations**: Support for creating/managing multiple triggers in a single request.

4. **Trigger templates**: Pre-configured trigger setups for common use cases.

5. **Event replay**: Ability to replay missed events from a specific timestamp.

## Related Documentation

- [Python SDK Triggers](https://github.com/ComposioHQ/composio/blob/main/python/composio/core/models/triggers.py)
- [Composio Triggers API](https://docs.composio.dev/api-reference/triggers)
- [Webhook Verification Guide](https://docs.composio.dev/guides/verifying-webhooks)
