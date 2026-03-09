# Webhook Events Implementation

## Overview

This document describes the Rust implementation of the `webhook_events.py` module from the Composio Python SDK. The implementation provides strongly-typed webhook event handling for Composio's webhook system.

## Files Created

### 1. `src/models/webhook_events.rs`

Main implementation file containing all webhook event types and utilities.

**Key Components:**

- **Enums:**
  - `WebhookEventType`: Known webhook event types (ConnectionExpired, TriggerMessage)
  - `ConnectionStatus`: Connection status values (Initializing, Initiated, Active, Failed, Expired, Inactive)

- **Structs:**
  - `ConnectedAccountToolkit`: Toolkit information in webhook payloads
  - `ConnectedAccountAuthConfig`: Auth config details
  - `ConnectionStateVal`: Connection state value (varies by auth scheme)
  - `ConnectionState`: Connection state discriminated by auth scheme
  - `SingleConnectedAccountDetailedResponse`: Complete connected account data
  - `WebhookConnectionMetadata`: Webhook metadata (project_id, org_id)
  - `ConnectionExpiredEvent`: Connection expired webhook event payload

- **Type Aliases:**
  - `WebhookEvent`: Union type for all webhook events (currently only ConnectionExpired)

- **Functions:**
  - `is_connection_expired_event()`: Type guard to check if payload is a connection expired event

### 2. `examples/webhook_events_usage.rs`

Comprehensive example demonstrating webhook event handling.

**Features:**
- Connection expired event handling
- Generic webhook event router
- Event type matching
- Practical action recommendations

## Comparison with Python Implementation

### Similarities

1. **Structure**: Maintains the same data structure hierarchy as Python
2. **Field Names**: Uses identical field names (snake_case) for API compatibility
3. **Type Safety**: Provides strong typing for all webhook payloads
4. **Documentation**: Comprehensive documentation with examples

### Differences

1. **Type System**:
   - Python uses `TypedDict` and `typing_extensions.TypeGuard`
   - Rust uses `struct` with `serde` for serialization/deserialization
   - Rust provides compile-time type checking vs Python's runtime checking

2. **Optional Fields**:
   - Python uses `total=False` in TypedDict
   - Rust uses `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`

3. **Enums**:
   - Python uses `str` subclass with `Enum`
   - Rust uses native enums with `#[serde(rename_all)]` attributes

4. **Type Guards**:
   - Python: `def is_connection_expired_event(...) -> TypeGuard[ConnectionExpiredEvent]`
   - Rust: `fn is_connection_expired_event(...) -> bool`

5. **Union Types**:
   - Python: `WebhookEvent = t.Union[ConnectionExpiredEvent]`
   - Rust: `enum WebhookEvent { ConnectionExpired(ConnectionExpiredEvent) }`

## Usage Examples

### Basic Event Detection

```rust
use composio_sdk::models::{is_connection_expired_event, ConnectionExpiredEvent};

fn handle_webhook(payload: serde_json::Value) {
    if is_connection_expired_event(&payload) {
        if let Ok(event) = serde_json::from_value::<ConnectionExpiredEvent>(payload) {
            println!("Connection {} expired for user {}", 
                event.data.id, event.data.user_id);
        }
    }
}
```

### Generic Event Router

```rust
use composio_sdk::models::ConnectionExpiredEvent;

fn route_webhook_event(payload: serde_json::Value) {
    let event_type = payload
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("unknown");

    match event_type {
        "composio.connected_account.expired" => {
            if let Ok(event) = serde_json::from_value::<ConnectionExpiredEvent>(payload) {
                handle_connection_expired(event);
            }
        }
        "composio.trigger.message" => {
            // Handle trigger message
        }
        _ => {
            eprintln!("Unknown event type: {}", event_type);
        }
    }
}

fn handle_connection_expired(event: ConnectionExpiredEvent) {
    // Send re-authentication notification to user
    println!("User {} needs to re-authenticate {}", 
        event.data.user_id, event.data.toolkit.slug);
}
```

### Event Type Serialization

```rust
use composio_sdk::models::WebhookEventType;

let event_type = WebhookEventType::ConnectionExpired;
let json = serde_json::to_string(&event_type).unwrap();
// json == "\"composio.connected_account.expired\""
```

## Integration with Existing Code

The webhook events module integrates seamlessly with the existing Composio SDK:

1. **No Conflicts**: Uses unique type names to avoid conflicts with `connected_accounts` module
2. **Consistent Patterns**: Follows the same patterns as other model modules
3. **Exported Types**: All types are properly exported through `src/models/mod.rs`

### Export Strategy

To avoid naming conflicts:
- `ConnectionStatus` and `ConnectionState` from `connected_accounts` module are kept as primary exports
- `webhook_events::ConnectionStatus` is not re-exported (same enum)
- `webhook_events::ConnectionState` is not re-exported (different struct for webhook payloads)
- Other webhook-specific types are exported with clear documentation

## Testing

The implementation includes comprehensive tests:

1. **Serialization Tests**: Verify enum serialization matches expected format
2. **Deserialization Tests**: Verify complex event payloads can be parsed
3. **Type Guard Tests**: Verify event type detection works correctly
4. **Field Tests**: Verify all auth scheme fields are properly handled

Run tests:
```bash
cargo test --lib webhook_events
```

## Future Enhancements

Potential additions to match Python SDK evolution:

1. **Additional Event Types**: Add more webhook event types as they become available
2. **Webhook Verification**: Integrate with existing webhook verification in `triggers.rs`
3. **Event Handlers**: Trait-based event handler system for extensibility
4. **Async Support**: Async event processing utilities

## References

- Python Implementation: `temp/composio/core/models/webhook_events.py`
- Triggers Module: `src/models/triggers.rs`
- Connected Accounts Module: `src/models/connected_accounts.rs`
- API Documentation: File 21 in Composio Documentation Guide
