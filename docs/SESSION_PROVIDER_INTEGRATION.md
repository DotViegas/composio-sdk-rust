# Session + Provider Integration

## 📋 Overview

This document explains the integration between the Session system and the Provider system in the Composio Rust SDK. This integration enables seamless conversion of Composio's universal tool format to framework-specific formats (OpenAI, Anthropic, etc.).

## 🎯 What Was Implemented

### New Method: `Session::get_provider_tools()`

```rust
pub async fn get_provider_tools<P>(
    &self,
    provider: &P,
) -> Result<P::ToolCollection, ComposioError>
where
    P: crate::providers::Provider,
```

This method:
1. Fetches meta tools from the Composio API (universal `ToolSchema` format)
2. Converts them using the provided `Provider` implementation
3. Returns tools in the framework-specific format

## 🚀 Usage Examples

### Example 1: OpenAI Format

```rust
use composio_sdk::{ComposioClient, providers::OpenAIProvider};

let client = ComposioClient::builder()
    .api_key("your_api_key")
    .build()?;

let session = client
    .create_session("user_123")
    .toolkits(vec!["github"])
    .send()
    .await?;

// Get tools in OpenAI format
let provider = OpenAIProvider::new();
let openai_tools = session.get_provider_tools(&provider).await?;
// Type: Vec<ChatCompletionToolParam>

// Use with OpenAI API
// let response = openai_client.chat().completions().create(
//     ChatCompletionRequest {
//         model: "gpt-4",
//         messages: vec![...],
//         tools: Some(openai_tools),
//         ...
//     }
// ).await?;
```

### Example 2: OpenAI with Strict Validation

```rust
use composio_sdk::{ComposioClient, providers::OpenAIProvider};

let provider = OpenAIProvider::new().with_strict(true);
let openai_tools = session.get_provider_tools(&provider).await?;
// All tools will have strict: true
```

### Example 3: Anthropic Format

```rust
use composio_sdk::{ComposioClient, providers::AnthropicProvider};

let client = ComposioClient::builder()
    .api_key("your_api_key")
    .build()?;

let session = client
    .create_session("user_123")
    .send()
    .await?;

// Get tools in Anthropic format
let provider = AnthropicProvider::new();
let anthropic_tools = session.get_provider_tools(&provider).await?;
// Type: Vec<AnthropicTool>

// Use with Anthropic API
// let response = anthropic_client.messages().create(
//     MessageRequest {
//         model: "claude-3-5-sonnet-20241022",
//         messages: vec![...],
//         tools: Some(anthropic_tools),
//         ...
//     }
// ).await?;
```

### Example 4: Universal Format (No Provider)

```rust
// If you need the universal format (ToolSchema), use get_meta_tools()
let universal_tools = session.get_meta_tools().await?;
// Type: Vec<ToolSchema>

// This is useful for:
// - Custom provider implementations
// - Inspecting tool schemas
// - Building your own tool format
```

## 🔄 Comparison with Python SDK

### Python Implementation

```python
# In Python SDK
class Session:
    def tools(self) -> TToolCollection:
        """Get meta tools wrapped for the provider"""
        return self._tool_router.get_meta_tools(
            session_id=self.session_id,
            provider=self.provider
        )

# Usage
session = composio.create(user_id="user_123")
tools = session.tools()  # Automatically uses session's provider
```

### Rust Implementation

```rust
// In Rust SDK
impl Session {
    pub async fn get_provider_tools<P>(
        &self,
        provider: &P,
    ) -> Result<P::ToolCollection, ComposioError>
    where
        P: crate::providers::Provider,
    {
        let schemas = self.get_meta_tools().await?;
        let tools = provider.wrap_tools(schemas);
        Ok(tools)
    }
}

// Usage
let session = client.create_session("user_123").send().await?;
let provider = OpenAIProvider::new();
let tools = session.get_provider_tools(&provider).await?;
```

### Key Differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Provider Storage** | Stored in Session | Passed as parameter |
| **Type Safety** | Runtime (generic type) | Compile-time (generic method) |
| **Flexibility** | One provider per session | Any provider per call |
| **Ergonomics** | `session.tools()` | `session.get_provider_tools(&provider)` |

### Why the Difference?

**Rust Approach Benefits:**
1. **More Flexible**: Can use different providers with the same session
2. **Type Safe**: Compiler ensures provider compatibility
3. **No State**: Session doesn't need to store provider
4. **Explicit**: Clear which provider is being used

**Python Approach Benefits:**
1. **More Ergonomic**: Shorter syntax
2. **Consistent**: Provider set once, used everywhere

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Session                              │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │ get_meta_tools()                                    │    │
│  │   ↓                                                 │    │
│  │ Fetch from API → Vec<ToolSchema> (Universal)       │    │
│  └────────────────────────────────────────────────────┘    │
│                           ↓                                  │
│  ┌────────────────────────────────────────────────────┐    │
│  │ get_provider_tools(&provider)                       │    │
│  │   ↓                                                 │    │
│  │ 1. Call get_meta_tools()                            │    │
│  │ 2. provider.wrap_tools(schemas)                     │    │
│  │ 3. Return P::ToolCollection                         │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                           ↓
        ┌──────────────────┴──────────────────┐
        ↓                                      ↓
┌───────────────────┐              ┌───────────────────┐
│  OpenAIProvider   │              │ AnthropicProvider │
│                   │              │                   │
│ wrap_tools()      │              │ wrap_tools()      │
│   ↓               │              │   ↓               │
│ Vec<ChatCompletion│              │ Vec<AnthropicTool>│
│ ToolParam>        │              │                   │
└───────────────────┘              └───────────────────┘
```

## 🎯 Design Decisions

### 1. Generic Method vs Generic Struct

**Chosen: Generic Method**

```rust
// ✅ Chosen approach
impl Session {
    pub async fn get_provider_tools<P>(&self, provider: &P) -> Result<P::ToolCollection>
}

// ❌ Alternative (not chosen)
struct Session<P: Provider> {
    provider: P,
}
```

**Rationale:**
- More flexible (can use multiple providers with same session)
- Simpler API (no need to specify provider at session creation)
- Matches Rust idioms (explicit over implicit)
- Easier to add new providers without changing Session

### 2. Provider as Parameter vs Stored in Session

**Chosen: Provider as Parameter**

**Rationale:**
- No lifetime issues
- No need to clone provider
- Clear which provider is being used
- Can switch providers easily

### 3. Separate Methods vs Single Method

**Chosen: Two Methods**
- `get_meta_tools()` - Returns universal format
- `get_provider_tools()` - Returns provider-specific format

**Rationale:**
- Clear separation of concerns
- Users can choose universal format if needed
- Easier to test
- More flexible for custom implementations

## 🧪 Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::{OpenAIProvider, AnthropicProvider};

    #[tokio::test]
    async fn test_get_provider_tools_openai() {
        // Test OpenAI provider integration
    }

    #[tokio::test]
    async fn test_get_provider_tools_anthropic() {
        // Test Anthropic provider integration
    }

    #[tokio::test]
    async fn test_get_provider_tools_with_strict() {
        // Test OpenAI strict validation
    }
}
```

### Integration Tests

See `examples/session_provider_integration.rs` for a complete working example.

## 📝 Documentation

### Method Documentation

The `get_provider_tools()` method includes:
- Comprehensive doc comments
- Type parameter documentation
- Multiple usage examples (OpenAI, Anthropic)
- Error documentation
- Related method references

### Examples

1. `examples/session_provider_integration.rs` - Complete demo
2. `examples/provider_system_demo.rs` - Provider basics
3. Doc comments in `src/session.rs` - Inline examples

## ✅ Checklist

- [x] Implement `Session::get_provider_tools()`
- [x] Add comprehensive documentation
- [x] Create working example
- [x] Test compilation
- [x] Verify provider tests pass
- [x] Document design decisions
- [x] Compare with Python implementation

## 🚀 Next Steps (Future Enhancements)

### 1. Helper Methods (Medium Priority)

```rust
// Optional trait for convenience methods
pub trait ProviderHelpers: Provider {
    type Response;
    type ToolCall;
    
    fn execute_tool_call(
        &self,
        user_id: &str,
        tool_call: &Self::ToolCall,
    ) -> Result<ToolExecutionResponse, ComposioError>;
    
    fn handle_tool_calls(
        &self,
        user_id: &str,
        response: &Self::Response,
    ) -> Result<Vec<ToolExecutionResponse>, ComposioError>;
}
```

### 2. File Helper Integration (Medium Priority)

```rust
pub struct FileHelper {
    client: Arc<ComposioClient>,
    outdir: Option<PathBuf>,
}

impl FileHelper {
    pub fn process_file_uploadable_schema(&self, schema: &mut Value);
    pub async fn upload_file(&self, path: &Path) -> Result<String>;
    pub async fn download_file(&self, url: &str) -> Result<PathBuf>;
}
```

### 3. Modifiers Integration (Low Priority)

```rust
impl Session {
    pub async fn get_provider_tools_with_modifiers<P>(
        &self,
        provider: &P,
        modifiers: &Modifiers,
    ) -> Result<P::ToolCollection, ComposioError>;
}
```

## 📚 Related Documentation

- [Provider System Explained](./PROVIDER_SYSTEM_EXPLAINED.md)
- [Provider System Visual Guide](./PROVIDER_SYSTEM_VISUAL.md)
- [Provider System Rust Implementation](./PROVIDER_SYSTEM_RUST_IMPLEMENTATION.md)
- [Python vs Rust Provider Comparison](./PYTHON_VS_RUST_PROVIDER_COMPARISON.md)

## 🎉 Conclusion

The Session + Provider integration is now **complete and functional**. Users can:

1. ✅ Create sessions with toolkit configuration
2. ✅ Get meta tools in universal format
3. ✅ Convert tools to OpenAI format
4. ✅ Convert tools to Anthropic format
5. ✅ Use tools with AI frameworks
6. ✅ Switch providers easily

This provides **feature parity** with the Python SDK's core provider functionality, with the added benefits of Rust's type safety and compile-time guarantees.
