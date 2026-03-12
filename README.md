# Composio Rust SDK

Type-safe, async Rust SDK for Composio’s Tool Router and API v3.

## Overview

Composio helps AI apps and agents connect to external services through a unified API.  
This SDK gives Rust developers a strongly-typed client for sessions, tools, auth flows, MCP servers, files, and trigger-related APIs.

## Features

- **Tool execution**: list/retrieve/execute/proxy tools and generate tool inputs.
- **Tool Router sessions**: create per-user sessions and execute tools in session context.
- **MCP servers**: create, list, retrieve, update, delete, and generate MCP server URLs.
- **Connected accounts**: list/retrieve/link/refresh/update-status/delete connected accounts.
- **Auth configs**: list/create/retrieve/update/delete auth configs and update status.
- **Files**: list files, request upload URLs, and use upload/download helper abstractions.
- **CLI sessions**: create and retrieve CLI authentication sessions.
- **Migration helper**: convert legacy UUIDs to NanoIDs.
- **Project config**: retrieve and update project-level configuration.
- **Triggers**: list/retrieve trigger types, retrieve trigger enums, and work with trigger instances.

## Installation

```bash
cargo add composio-sdk
```

Or add manually:

```toml
[dependencies]
composio-sdk = "0.2"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

## Quick Start

Minimal end-to-end flow:

```rust
use composio_sdk::client::ComposioClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(std::env::var("COMPOSIO_API_KEY")?)
        .build()?;

    // 1) Create a user-scoped Tool Router session
    let session = client
        .create_session("user_123")
        .toolkits(vec!["github"])
        .send()
        .await?;

    // 2) Execute a tool via that session
    let result = session
        .execute_tool(
            "GITHUB_CREATE_ISSUE",
            serde_json::json!({
                "owner": "acme",
                "repo": "demo",
                "title": "Issue from Rust SDK"
            }),
        )
        .await?;

    println!("success: {}", result.successful);
    Ok(())
}
```

## Authentication

Use your Composio API key (sent as `x-api-key`):

```rust
let client = composio_sdk::client::ComposioClient::builder()
    .api_key(std::env::var("COMPOSIO_API_KEY")?)
    .build()?;
```

## Core Concepts

### Tools
Use `list_tools`, `get_tool`, `execute_tool`, `generate_tool_inputs`, `proxy_tool`, and `retrieve_tool_enum`.

### Tool Router
Create sessions with `create_session(...).send()`, then execute tools and meta tools per user context.

### MCP Servers
Use `create_mcp_server`, `list_mcp_servers`, `get_mcp_server`, `update_mcp_server`, `delete_mcp_server`, `generate_mcp_server`, and related methods.

### Connected Accounts
Use `list_connected_accounts`, `get_connected_account`, `create_connected_account_link`, `refresh_connected_account`, `update_connected_account_status`, and `delete_connected_account`.

### Auth Configs
Use `list_auth_configs`, `create_auth_config`, `get_auth_config`, `update_auth_config`, `delete_auth_config`, and `update_auth_config_status`.

### Files
Use endpoint wrappers (`list_files`, `create_file_upload_request`) or helper abstractions (`FileUploadable`, `FileDownloadable`, `FileHelper`).

### CLI Sessions
Use `create_cli_session` and `get_cli_session` for CLI auth workflows.

### Migration
Use `get_migration_nanoid` for UUID → NanoID migration lookups.

### Project Config
Use `get_project_config` and `update_project_config` for project-level settings.

## Examples

### Execute a tool (client-level)

```rust
use composio_sdk::models::tools::ToolExecuteParams;
use std::collections::HashMap;

let mut arguments = HashMap::new();
arguments.insert("owner".to_string(), serde_json::json!("acme"));
arguments.insert("repo".to_string(), serde_json::json!("demo"));
arguments.insert("title".to_string(), serde_json::json!("Issue from SDK"));

let response = client
    .execute_tool(ToolExecuteParams {
        slug: "GITHUB_CREATE_ISSUE".to_string(),
        arguments,
        user_id: Some("user_123".to_string()),
        ..Default::default()
    })
    .await?;
```

### Create an MCP server

```rust
use composio_sdk::models::mcp::MCPCreateParams;

let mcp = client
    .create_mcp_server(MCPCreateParams {
        name: "my-mcp".to_string(),
        auth_config_ids: vec!["ac_123".to_string()],
        allowed_tools: None,
        no_auth_apps: None,
        toolkits: None,
        custom_tools: None,
        managed_auth_via_composio: Some(false),
    })
    .await?;
```

### List connected accounts

```rust
use composio_sdk::models::connected_accounts::ConnectedAccountListParams;

let accounts = client
    .list_connected_accounts(ConnectedAccountListParams {
        user_ids: Some(vec!["user_123".to_string()]),
        ..Default::default()
    })
    .await?;
```

### Upload a file (helper flow)

```rust
use composio_sdk::models::files::FileUploadable;
use std::path::Path;

let uploaded = FileUploadable::from_path(
    &client,
    Path::new("./invoice.pdf"),
    "GMAIL_SEND_EMAIL",
    "gmail",
)
.await?;

println!("s3 key: {}", uploaded.s3key);
```

### Create a CLI session

```rust
let cli = client.create_cli_session().await?;
println!("session code: {}", cli.code);
```

## Architecture

- `src/client.rs`: API methods and HTTP orchestration.
- `src/models/`: typed request/response models by domain.
- `src/session.rs`: Tool Router session abstraction.
- `src/meta_tools/`: Rust-native meta tool helpers.

## Error Handling

Most APIs return `Result<T, ComposioError>`.

```rust
match client.create_cli_session().await {
    Ok(session) => println!("code={}", session.code),
    Err(err) => eprintln!("request failed: {err}"),
}
```

## Contributing

1. Keep changes focused by domain.
2. Add/adjust serialization + deserialization tests for model changes.
3. Run:
   - `cargo fmt --all`
   - `cargo test`
   - `cargo check`
4. Keep parity work aligned with `COMPOSIO_CLIENT_AUDIT_REPORT.md`.

## Roadmap

Major parity gaps have been closed (MCP, CLI, migration, project config, connected-account lifecycle, auth-config lifecycle, direct link endpoint, files list/upload-request, tool/trigger enums, toolkit retrieve version params).

Remaining work is mainly contract-shape refinement in partially covered domains tracked in `COMPOSIO_CLIENT_AUDIT_REPORT.md`.

## License

Licensed under either:

- MIT
- Apache-2.0

at your option.
