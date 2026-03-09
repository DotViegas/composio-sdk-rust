# Tool Router Implementation - Python to Rust Translation

## Overview

This document describes the translation of the Python `tool_router.py` module to Rust. The implementation adds missing types and features to match the Python SDK's Tool Router functionality.

## What Was Implemented

### 1. Response Types (src/models/response.rs)

Added the following types to represent Tool Router session responses:

#### `ToolRouterMcpServerType`
```rust
pub enum ToolRouterMcpServerType {
    Http,
    Sse,
}
```
- Represents the type of MCP server (HTTP or Server-Sent Events)
- Equivalent to Python's `ToolRouterMCPServerType`

#### `ToolRouterMcpServerConfig`
```rust
pub struct ToolRouterMcpServerConfig {
    pub server_type: ToolRouterMcpServerType,
    pub url: String,
    pub headers: Option<HashMap<String, Option<String>>>,
}
```
- Contains MCP server connection details
- Equivalent to Python's `ToolRouterMCPServerConfig`

#### `ToolRouterSessionExperimental`
```rust
pub struct ToolRouterSessionExperimental {
    pub assistive_prompt: Option<String>,
}
```
- Holds experimental features data from session response
- Equivalent to Python's `ToolRouterSessionExperimental`

#### Toolkit Connection Types

**`ToolkitConnectionAuthConfig`**
```rust
pub struct ToolkitConnectionAuthConfig {
    pub id: String,
    pub mode: String,
    pub is_composio_managed: bool,
}
```
- Auth config information for a toolkit connection
- Equivalent to Python's `ToolkitConnectionAuthConfig`

**`ToolkitConnectedAccount`**
```rust
pub struct ToolkitConnectedAccount {
    pub id: String,
    pub status: String,
}
```
- Connected account information for a toolkit
- Equivalent to Python's `ToolkitConnectedAccount`

**`ToolkitConnection`**
```rust
pub struct ToolkitConnection {
    pub is_active: bool,
    pub auth_config: Option<ToolkitConnectionAuthConfig>,
    pub connected_account: Option<ToolkitConnectedAccount>,
}
```
- Connection information for a toolkit
- Equivalent to Python's `ToolkitConnection`

**`ToolkitConnectionState`**
```rust
pub struct ToolkitConnectionState {
    pub slug: String,
    pub name: String,
    pub is_no_auth: bool,
    pub connection: Option<ToolkitConnection>,
    pub logo: Option<String>,
}
```
- Complete connection state of a toolkit in a session
- Equivalent to Python's `ToolkitConnectionState`

**`ToolkitConnectionsDetails`**
```rust
pub struct ToolkitConnectionsDetails {
    pub items: Vec<ToolkitConnectionState>,
    pub total_pages: u32,
    pub next_cursor: Option<String>,
}
```
- Paginated list of toolkit connection states
- Equivalent to Python's `ToolkitConnectionsDetails`

### 2. Request Types (src/models/request.rs)

Added experimental configuration types:

#### `AssistivePromptConfig`
```rust
pub struct AssistivePromptConfig {
    pub user_timezone: Option<String>,
}
```
- Configuration for timezone-aware assistive prompts
- Equivalent to Python's `ToolRouterAssistivePromptConfig`

#### `ExperimentalConfig`
```rust
pub struct ExperimentalConfig {
    pub assistive_prompt: Option<AssistivePromptConfig>,
}
```
- Container for experimental features
- Equivalent to Python's `ToolRouterExperimentalConfig`

#### Updated `SessionConfig`
Added `experimental` field to `SessionConfig`:
```rust
pub struct SessionConfig {
    // ... existing fields ...
    pub experimental: Option<ExperimentalConfig>,
    // ...
}
```

### 3. Session Builder Enhancement (src/session.rs)

Added new builder method:

#### `experimental()`
```rust
pub fn experimental(mut self, user_timezone: Option<String>) -> Self
```
- Configures experimental features for the session
- Sets timezone for assistive prompt generation
- Example:
  ```rust
  let session = client
      .create_session("user_123")
      .experimental(Some("America/New_York".to_string()))
      .send()
      .await?;
  ```

### 4. Module Exports (src/models/mod.rs)

Added exports for all new types:
- `AssistivePromptConfig`
- `ExperimentalConfig`
- `ToolRouterMcpServerType`
- `ToolRouterMcpServerConfig`
- `ToolRouterSessionExperimental`
- `ToolkitConnectionAuthConfig`
- `ToolkitConnectedAccount`
- `ToolkitConnection`
- `ToolkitConnectionState`
- `ToolkitConnectionsDetails`

## Comparison: Python vs Rust

### Python Implementation

The Python `tool_router.py` provides:
1. **Type Definitions**: TypedDict classes for configuration
2. **ToolRouter Class**: Manages session creation and retrieval
3. **Helper Functions**: Internal methods for creating closures
4. **Generic Support**: Uses TypeVar for provider-specific types

### Rust Implementation

The Rust implementation provides:
1. **Type Definitions**: Strongly-typed structs with serde support
2. **Session & SessionBuilder**: Fluent API for session creation
3. **Direct Methods**: Methods directly on Session struct
4. **Compile-time Safety**: Type checking at compile time

### Key Differences

| Feature | Python | Rust |
|---------|--------|------|
| Type System | Runtime (TypedDict) | Compile-time (structs) |
| Generics | TypeVar with Protocol | Generic parameters on Session |
| Builder Pattern | Method chaining on ToolRouter | SessionBuilder with fluent API |
| Error Handling | Exceptions | Result<T, E> |
| Async | async/await | async/await with Tokio |

## What Already Existed

The following were already implemented in Rust before this work:

1. **Session Management** (`src/session.rs`):
   - `Session` struct
   - `SessionBuilder` with fluent API
   - `execute_tool()` method
   - `execute_meta_tool()` method
   - `list_toolkits()` method
   - `get_meta_tools()` method
   - `create_auth_link()` method

2. **Basic Configuration Types** (`src/models/request.rs`):
   - `SessionConfig`
   - `ToolkitFilter`
   - `ToolsConfig` and `ToolFilter`
   - `TagsConfig`
   - `ManageConnectionsConfig`
   - `WorkbenchConfig`

3. **Response Types** (`src/models/response.rs`):
   - `SessionResponse`
   - `McpInfo`
   - `ToolSchema`
   - `ToolExecutionResponse`
   - `ToolkitListResponse`
   - `LinkResponse`

## Usage Examples

### Creating a Session with Experimental Features

```rust
use composio_sdk::ComposioClient;

let client = ComposioClient::builder()
    .api_key("your-api-key")
    .build()?;

let session = client
    .create_session("user_123")
    .toolkits(vec!["github", "gmail"])
    .experimental(Some("America/New_York".to_string()))
    .send()
    .await?;
```

### Checking Toolkit Connection Status

```rust
let toolkits = session.list_toolkits().send().await?;

for toolkit in toolkits.items {
    if let Some(connection) = toolkit.connection {
        if connection.is_active {
            println!("{} is connected", toolkit.name);
        }
    }
}
```

### Using Workbench Configuration

```rust
let session = client
    .create_session("user_123")
    .workbench(Some(true), Some(1000))
    .send()
    .await?;
```

### Tag-based Tool Filtering

```rust
use composio_sdk::models::enums::TagType;

let session = client
    .create_session("user_123")
    .tags(
        Some(vec![TagType::ReadOnlyHint]),
        Some(vec![TagType::DestructiveHint])
    )
    .send()
    .await?;
```

## Testing

A comprehensive example demonstrating all new features is available:
- `examples/tool_router_experimental.rs`

Run it with:
```bash
COMPOSIO_API_KEY=your_key cargo run --example tool_router_experimental
```

## Architecture Notes

### Design Decisions

1. **Struct-based Types**: Used structs instead of enums for configuration to match API structure
2. **Optional Fields**: Used `Option<T>` extensively for optional configuration
3. **Builder Pattern**: Extended existing `SessionBuilder` instead of creating new `ToolRouter` class
4. **Serde Integration**: All types support serialization/deserialization
5. **Documentation**: Added comprehensive doc comments with examples

### Future Enhancements

Potential improvements for future versions:

1. **Generic Session**: Add generic parameters to Session for provider-specific tool types
2. **Callback Functions**: Implement closure-based helpers like Python's `_create_tools_fn`
3. **Advanced Filtering**: Add more sophisticated tool filtering options
4. **Validation**: Add compile-time validation for configuration combinations
5. **Async Traits**: Consider using async traits for extensibility

## Conclusion

The Rust implementation now has feature parity with the Python `tool_router.py` module. All types, configurations, and experimental features are available with the added benefits of:

- Compile-time type safety
- Zero-cost abstractions
- Memory safety without garbage collection
- Excellent performance
- Clear error handling with Result types

The implementation maintains the same API structure and behavior as the Python version while leveraging Rust's strengths.
