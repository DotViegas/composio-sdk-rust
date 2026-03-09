# Quick Start: Using Providers with Sessions

## 🚀 5-Minute Guide

This guide shows you how to use the Provider System with Sessions in the Composio Rust SDK.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
composio-sdk = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Basic Usage

### Step 1: Create a Client

```rust
use composio_sdk::ComposioClient;

let client = ComposioClient::builder()
    .api_key(std::env::var("COMPOSIO_API_KEY")?)
    .build()?;
```

### Step 2: Create a Session

```rust
let session = client
    .create_session("user_123")
    .toolkits(vec!["github", "gmail"])
    .send()
    .await?;
```

### Step 3: Get Tools in Your Framework's Format

#### For OpenAI

```rust
use composio_sdk::providers::OpenAIProvider;

let provider = OpenAIProvider::new();
let tools = session.get_provider_tools(&provider).await?;

// Use with OpenAI API
// openai_client.chat().completions().create(
//     ChatCompletionRequest {
//         model: "gpt-4",
//         tools: Some(tools),
//         ...
//     }
// )
```

#### For Anthropic

```rust
use composio_sdk::providers::AnthropicProvider;

let provider = AnthropicProvider::new();
let tools = session.get_provider_tools(&provider).await?;

// Use with Anthropic API
// anthropic_client.messages().create(
//     MessageRequest {
//         model: "claude-3-5-sonnet-20241022",
//         tools: Some(tools),
//         ...
//     }
// )
```

## Complete Example

```rust
use composio_sdk::{ComposioClient, providers::OpenAIProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client
    let client = ComposioClient::builder()
        .api_key(std::env::var("COMPOSIO_API_KEY")?)
        .build()?;

    // 2. Create session
    let session = client
        .create_session("user_123")
        .toolkits(vec!["github"])
        .send()
        .await?;

    // 3. Get tools for OpenAI
    let provider = OpenAIProvider::new();
    let tools = session.get_provider_tools(&provider).await?;

    println!("Got {} tools in OpenAI format", tools.len());

    // 4. Use with your AI framework
    // ... your OpenAI API calls here ...

    Ok(())
}
```

## Advanced Features

### OpenAI with Strict Validation

```rust
let provider = OpenAIProvider::new().with_strict(true);
let tools = session.get_provider_tools(&provider).await?;
```

### Universal Format (No Provider)

```rust
// Get tools in Composio's universal format
let tools = session.get_meta_tools().await?;
// Type: Vec<ToolSchema>
```

### Multiple Providers with Same Session

```rust
// Get tools in different formats from the same session
let openai_tools = session.get_provider_tools(&OpenAIProvider::new()).await?;
let anthropic_tools = session.get_provider_tools(&AnthropicProvider::new()).await?;
```

## Available Providers

| Provider | Format | Use Case |
|----------|--------|----------|
| `OpenAIProvider` | `ChatCompletionToolParam` | OpenAI Chat Completions API |
| `AnthropicProvider` | `AnthropicTool` | Anthropic Messages API |

## Common Patterns

### Pattern 1: Session with Specific Toolkits

```rust
let session = client
    .create_session("user_123")
    .toolkits(vec!["github", "gmail", "slack"])
    .send()
    .await?;
```

### Pattern 2: Session with Custom Auth

```rust
let session = client
    .create_session("user_123")
    .auth_config("github", "ac_custom_oauth")
    .send()
    .await?;
```

### Pattern 3: Session with Tag Filtering

```rust
use composio_sdk::models::enums::TagType;

let session = client
    .create_session("user_123")
    .tags(
        Some(vec![TagType::ReadOnlyHint]),  // Only read-only tools
        Some(vec![TagType::DestructiveHint]) // Exclude destructive tools
    )
    .send()
    .await?;
```

## Error Handling

```rust
match session.get_provider_tools(&provider).await {
    Ok(tools) => {
        println!("Got {} tools", tools.len());
        // Use tools...
    }
    Err(e) => {
        eprintln!("Error getting tools: {}", e);
        // Handle error...
    }
}
```

## Type Safety

The provider system is fully type-safe:

```rust
// Compile-time type checking
let openai_tools: Vec<ChatCompletionToolParam> = 
    session.get_provider_tools(&OpenAIProvider::new()).await?;

let anthropic_tools: Vec<AnthropicTool> = 
    session.get_provider_tools(&AnthropicProvider::new()).await?;
```

## Next Steps

- See `examples/session_provider_integration.rs` for a complete working example
- Read [Session Provider Integration](./SESSION_PROVIDER_INTEGRATION.md) for detailed documentation
- Check [Provider System Explained](./PROVIDER_SYSTEM_EXPLAINED.md) for architecture details

## Need Help?

- Check the [examples](../examples/) directory
- Read the [full documentation](./SESSION_PROVIDER_INTEGRATION.md)
- Open an issue on GitHub

## Summary

```rust
// 1. Create client
let client = ComposioClient::builder().api_key("...").build()?;

// 2. Create session
let session = client.create_session("user_123").send().await?;

// 3. Get tools for your framework
let tools = session.get_provider_tools(&OpenAIProvider::new()).await?;

// 4. Use with your AI framework
// ... your API calls here ...
```

That's it! You're ready to use Composio tools with your AI framework. 🚀
