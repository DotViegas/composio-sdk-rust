# Composio Rust SDK

A minimal, type-safe Rust SDK for the [Composio](https://composio.dev) Tool Router REST API.

[![Crates.io](https://img.shields.io/crates/v/composio-sdk.svg)](https://crates.io/crates/composio-sdk)
[![Documentation](https://docs.rs/composio-sdk/badge.svg)](https://docs.rs/composio-sdk)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

- 🦀 **Type-Safe**: Compile-time validation with Rust's type system
- ⚡ **Async/Await**: Built on tokio for high-performance async operations
- 🔄 **Automatic Retries**: Exponential backoff for transient failures
- 📦 **Minimal Footprint**: ~2 MB memory overhead
- 🛡️ **Error Handling**: Comprehensive error types with actionable messages
- 🔧 **Session Management**: User-scoped sessions for data isolation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
composio-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use composio_sdk::{ComposioClient, ComposioError};

#[tokio::main]
async fn main() -> Result<(), ComposioError> {
    // Initialize the client
    let client = ComposioClient::builder()
        .api_key(std::env::var("COMPOSIO_API_KEY")?)
        .build()?;

    // Create a session for a user
    let session = client
        .create_session("user_123")
        .toolkits(vec!["github", "gmail"])
        .manage_connections(true)
        .send()
        .await?;

    println!("Session ID: {}", session.session_id());
    println!("MCP URL: {}", session.mcp_url());

    // Execute a tool
    let result = session
        .execute_tool(
            "GITHUB_CREATE_ISSUE",
            serde_json::json!({
                "owner": "composio",
                "repo": "composio",
                "title": "Test issue",
                "body": "Created via Rust SDK"
            })
        )
        .await?;

    println!("Result: {:?}", result.data);

    Ok(())
}
```

## Core Concepts

### Sessions

Sessions provide user-scoped access to tools and manage authentication:

```rust
// Create a session with specific toolkits
let session = client
    .create_session("user_123")
    .toolkits(vec!["github", "gmail", "slack"])
    .send()
    .await?;

// Disable specific toolkits
let session = client
    .create_session("user_456")
    .disable_toolkits(vec!["exa", "firecrawl"])
    .send()
    .await?;
```

### Tool Execution

Execute tools within a session:

```rust
// Execute a regular tool
let result = session
    .execute_tool("GMAIL_SEND_EMAIL", serde_json::json!({
        "to": "user@example.com",
        "subject": "Hello from Rust",
        "body": "This email was sent using Composio Rust SDK"
    }))
    .await?;

// Execute a meta tool
let search_result = session
    .execute_meta_tool(
        MetaToolSlug::ComposioSearchTools,
        serde_json::json!({
            "query": "create a GitHub issue"
        })
    )
    .await?;
```

### Error Handling

The SDK provides comprehensive error handling:

```rust
use composio_sdk::ComposioError;

match session.execute_tool("INVALID_TOOL", serde_json::json!({})).await {
    Ok(result) => println!("Success: {:?}", result),
    Err(ComposioError::ApiError { status, message, suggested_fix, .. }) => {
        eprintln!("API error ({}): {}", status, message);
        if let Some(fix) = suggested_fix {
            eprintln!("Suggested fix: {}", fix);
        }
    }
    Err(ComposioError::NetworkError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Configuration

Customize client behavior:

```rust
use std::time::Duration;

let client = ComposioClient::builder()
    .api_key("your-api-key")
    .base_url("https://backend.composio.dev/api/v3")
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .initial_retry_delay(Duration::from_secs(1))
    .max_retry_delay(Duration::from_secs(10))
    .build()?;
```

## Authentication

### In-Chat Authentication (Default)

The agent automatically prompts users with Connect Links during conversation:

```rust
let session = client
    .create_session("user_123")
    .toolkits(vec!["github"])
    .manage_connections(true)  // Default: true
    .send()
    .await?;

// Agent will automatically handle authentication when needed
```

### Manual Authentication

Pre-authenticate users before they start chatting:

```rust
// Create auth link
let link = session
    .create_auth_link("github", Some("https://yourapp.com/callback"))
    .await?;

println!("Redirect user to: {}", link.redirect_url);

// Check connection status
let toolkits = session.list_toolkits().send().await?;
for toolkit in toolkits.items {
    if toolkit.slug == "github" {
        if let Some(account) = toolkit.connected_account {
            println!("GitHub connected: {}", account.status);
        }
    }
}
```

## Toolkit Management

List and filter available toolkits:

```rust
// List all toolkits
let toolkits = session.list_toolkits().send().await?;

// Filter by connection status
let connected = session
    .list_toolkits()
    .is_connected(true)
    .send()
    .await?;

// Search toolkits
let results = session
    .list_toolkits()
    .search("email")
    .send()
    .await?;
```

## Meta Tools

The SDK supports all 5 meta tools:

```rust
use composio_sdk::MetaToolSlug;

// Search for tools
let search = session.execute_meta_tool(
    MetaToolSlug::ComposioSearchTools,
    serde_json::json!({ "query": "send email" })
).await?;

// Execute multiple tools in parallel
let multi = session.execute_meta_tool(
    MetaToolSlug::ComposioMultiExecuteTool,
    serde_json::json!({
        "tools": [
            { "tool_slug": "GITHUB_GET_REPOS", "arguments": { "owner": "composio" } },
            { "tool_slug": "GITHUB_GET_ISSUES", "arguments": { "owner": "composio", "repo": "composio" } }
        ]
    })
).await?;

// Manage connections
let manage = session.execute_meta_tool(
    MetaToolSlug::ComposioManageConnections,
    serde_json::json!({ "action": "list" })
).await?;

// Remote workbench (Python sandbox)
let workbench = session.execute_meta_tool(
    MetaToolSlug::ComposioRemoteWorkbench,
    serde_json::json!({ "code": "import pandas as pd\ndf = pd.DataFrame({'a': [1, 2, 3]})\nprint(df)" })
).await?;

// Remote bash
let bash = session.execute_meta_tool(
    MetaToolSlug::ComposioRemoteBashTool,
    serde_json::json!({ "command": "ls -la" })
).await?;
```

## Examples

See the [`examples/`](examples/) directory for complete working examples:

- [`basic_usage.rs`](examples/basic_usage.rs) - Session creation and tool execution
- [`meta_tools.rs`](examples/meta_tools.rs) - Using meta tools
- [`error_handling.rs`](examples/error_handling.rs) - Error handling patterns
- [`authentication.rs`](examples/authentication.rs) - Authentication flows
- [`toolkit_listing.rs`](examples/toolkit_listing.rs) - Listing and filtering toolkits

Run an example:

```bash
cargo run --example basic_usage
```

## Requirements

- Rust 1.70 or later
- Tokio runtime
- Composio API key ([get one here](https://app.composio.dev))

## Memory Footprint

The SDK is designed for minimal memory usage:

- **Library size**: 2.45 MB (release build)
- **Runtime overhead**: 112 bytes (client) + 296 bytes (session builder)
- **Initialization time**: ~200 µs (client creation)
- **Text section**: 28 KiB (SDK code only, excluding dependencies)

### Performance Characteristics

- Zero-copy deserialization where possible
- Efficient Arc-based sharing for client references
- Minimal stack allocation
- Suitable for resource-constrained environments

### Optimization Options

For production builds, add to your `Cargo.toml`:

```toml
[profile.release]
lto = true             # Link-time optimization (15-20% size reduction)
codegen-units = 1      # Better optimization
strip = true           # Strip debug symbols
```

See [MEMORY_FOOTPRINT_REPORT.md](MEMORY_FOOTPRINT_REPORT.md) for detailed analysis.

## API Coverage

This SDK focuses on the Tool Router API:

- ✅ Session creation and retrieval
- ✅ Tool execution (regular and meta tools)
- ✅ Toolkit listing and filtering
- ✅ Authentication link creation
- ✅ Meta tools schema retrieval
- ✅ Automatic retry with exponential backoff
- ✅ Comprehensive error handling

## Roadmap

Future enhancements:

- [ ] Direct tool execution (non-session)
- [ ] Triggers support
- [ ] Connected accounts management
- [ ] Auth configs management
- [ ] File upload/download
- [ ] Workbench mount operations

## Documentation

- [API Documentation](https://docs.rs/composio-sdk)
- [Composio Platform Docs](https://docs.composio.dev)
- [Examples](examples/)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Support

- [Discord Community](https://discord.gg/composio)
- [GitHub Issues](https://github.com/composio/composio-rust-sdk/issues)
- [Documentation](https://docs.composio.dev)

## Acknowledgments

This SDK is built for [ZeroClaw](https://github.com/zeroclaw), a lightweight Rust AI assistant, and follows the design patterns from the official [Composio Python SDK](https://github.com/ComposioHQ/composio).
