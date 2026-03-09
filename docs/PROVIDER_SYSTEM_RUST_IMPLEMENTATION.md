# Provider System - Implementação em Rust

## 🎯 Objetivo

Implementar um sistema de providers em Rust que permita suportar múltiplos frameworks de IA (OpenAI, Anthropic, etc.) com type safety e zero-cost abstractions.

---

## 🏗️ Arquitetura Proposta

### 1. Trait Base: `Provider`

```rust
use serde::{Serialize, Deserialize};
use crate::models::response::ToolSchema;

/// Provider trait for converting Composio tools to framework-specific formats
///
/// This trait enables the SDK to work with different AI frameworks by providing
/// a common interface for tool conversion.
///
/// # Type Parameters
///
/// * `Tool` - The framework-specific tool type (e.g., `ChatCompletionToolParam` for OpenAI)
/// * `ToolCollection` - The collection type returned by `wrap_tools`
///
/// # Example
///
/// ```rust
/// use composio_sdk::Provider;
///
/// struct OpenAIProvider;
///
/// impl Provider for OpenAIProvider {
///     type Tool = ChatCompletionToolParam;
///     type ToolCollection = Vec<ChatCompletionToolParam>;
///     
///     fn name(&self) -> &str {
///         "openai"
///     }
///     
///     fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
///         // Convert ToolSchema to OpenAI format
///         todo!()
///     }
///     
///     fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection {
///         tools.into_iter().map(|t| self.wrap_tool(&t)).collect()
///     }
/// }
/// ```
pub trait Provider: Send + Sync + 'static {
    /// The individual tool type for this provider
    type Tool: Serialize + for<'de> Deserialize<'de> + Send + Sync;
    
    /// The collection type returned by wrap_tools
    type ToolCollection: IntoIterator<Item = Self::Tool> + Send + Sync;
    
    /// Get the provider name
    fn name(&self) -> &str;
    
    /// Convert a single Composio tool to the provider's format
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool;
    
    /// Convert multiple Composio tools to the provider's format
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection;
}
```

---

### 2. Provider OpenAI

```rust
use serde::{Deserialize, Serialize};
use crate::Provider;
use crate::models::response::ToolSchema;

/// OpenAI tool format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionToolParam {
    pub r#type: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// OpenAI provider for Chat Completions API
///
/// Converts Composio tools to OpenAI's ChatCompletionToolParam format.
///
/// # Example
///
/// ```rust
/// use composio_sdk::{ComposioClient, OpenAIProvider};
///
/// let client = ComposioClient::with_provider(OpenAIProvider::new())
///     .api_key("your_key")
///     .build()?;
///
/// let session = client.create_session("user_123")
///     .toolkits(vec!["github"])
///     .send()
///     .await?;
///
/// // Get tools in OpenAI format
/// let tools = session.get_tools().await?;
/// // tools: Vec<ChatCompletionToolParam>
/// ```
#[derive(Debug, Clone, Default)]
pub struct OpenAIProvider {
    /// Whether to use strict schema validation
    pub strict: bool,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider
    pub fn new() -> Self {
        Self { strict: false }
    }
    
    /// Enable strict schema validation
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
}

impl Provider for OpenAIProvider {
    type Tool = ChatCompletionToolParam;
    type ToolCollection = Vec<ChatCompletionToolParam>;
    
    fn name(&self) -> &str {
        "openai"
    }
    
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
        ChatCompletionToolParam {
            r#type: "function".to_string(),
            function: FunctionDefinition {
                name: tool.slug.clone(),
                description: tool.description.clone(),
                parameters: tool.input_parameters.clone(),
                strict: if self.strict { Some(true) } else { None },
            },
        }
    }
    
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection {
        tools.iter().map(|t| self.wrap_tool(t)).collect()
    }
}
```

---

### 3. Provider Anthropic

```rust
use serde::{Deserialize, Serialize};
use crate::Provider;
use crate::models::response::ToolSchema;

/// Anthropic tool format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Anthropic provider for Claude API
///
/// Converts Composio tools to Anthropic's tool format.
#[derive(Debug, Clone, Default)]
pub struct AnthropicProvider;

impl AnthropicProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Provider for AnthropicProvider {
    type Tool = AnthropicTool;
    type ToolCollection = Vec<AnthropicTool>;
    
    fn name(&self) -> &str {
        "anthropic"
    }
    
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
        AnthropicTool {
            name: tool.slug.clone(),
            description: tool.description.clone(),
            input_schema: tool.input_parameters.clone(),
        }
    }
    
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection {
        tools.iter().map(|t| self.wrap_tool(t)).collect()
    }
}
```

---

### 4. Cliente Genérico

```rust
use std::marker::PhantomData;
use crate::Provider;

/// Composio client with provider support
///
/// The client is generic over a provider type, allowing it to work with
/// different AI frameworks while maintaining type safety.
///
/// # Type Parameters
///
/// * `P` - The provider type (defaults to `OpenAIProvider`)
///
/// # Example
///
/// ```rust
/// use composio_sdk::{ComposioClient, OpenAIProvider, AnthropicProvider};
///
/// // With OpenAI (default)
/// let client = ComposioClient::builder()
///     .api_key("key")
///     .build()?;
///
/// // With explicit provider
/// let client = ComposioClient::with_provider(OpenAIProvider::new())
///     .api_key("key")
///     .build()?;
///
/// // With Anthropic
/// let client = ComposioClient::with_provider(AnthropicProvider::new())
///     .api_key("key")
///     .build()?;
/// ```
pub struct ComposioClient<P: Provider = OpenAIProvider> {
    http_client: reqwest::Client,
    config: ComposioConfig,
    provider: P,
}

impl ComposioClient<OpenAIProvider> {
    /// Create a new client builder with default OpenAI provider
    pub fn builder() -> ComposioClientBuilder<OpenAIProvider> {
        ComposioClientBuilder::new(OpenAIProvider::new())
    }
}

impl<P: Provider> ComposioClient<P> {
    /// Create a new client builder with a specific provider
    pub fn with_provider(provider: P) -> ComposioClientBuilder<P> {
        ComposioClientBuilder::new(provider)
    }
    
    /// Get a reference to the provider
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

/// Builder for ComposioClient with provider support
pub struct ComposioClientBuilder<P: Provider> {
    provider: P,
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    // ... other fields
}

impl<P: Provider> ComposioClientBuilder<P> {
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            api_key: None,
            base_url: None,
            timeout: None,
        }
    }
    
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    pub fn build(self) -> Result<ComposioClient<P>, ComposioError> {
        let api_key = self.api_key
            .or_else(|| std::env::var("COMPOSIO_API_KEY").ok())
            .ok_or_else(|| ComposioError::ConfigError(
                "API key not provided".to_string()
            ))?;
        
        let config = ComposioConfig::new(api_key);
        let http_client = reqwest::Client::new();
        
        Ok(ComposioClient {
            http_client,
            config,
            provider: self.provider,
        })
    }
}
```

---

### 5. Session com Provider

```rust
use crate::Provider;

/// Session with provider support
///
/// The session uses the client's provider to convert tools to the
/// appropriate format for the AI framework being used.
pub struct Session<P: Provider> {
    client: Arc<ComposioClient<P>>,
    session_id: String,
    mcp_url: String,
    tools: Vec<String>,
}

impl<P: Provider> Session<P> {
    /// Get tools in the provider's format
    ///
    /// Returns tools formatted for the specific AI framework.
    ///
    /// # Example
    ///
    /// ```rust
    /// // With OpenAI provider
    /// let client = ComposioClient::builder().api_key("key").build()?;
    /// let session = client.create_session("user_123").send().await?;
    /// let tools = session.get_tools().await?;
    /// // tools: Vec<ChatCompletionToolParam>
    ///
    /// // With Anthropic provider
    /// let client = ComposioClient::with_provider(AnthropicProvider::new())
    ///     .api_key("key")
    ///     .build()?;
    /// let session = client.create_session("user_123").send().await?;
    /// let tools = session.get_tools().await?;
    /// // tools: Vec<AnthropicTool>
    /// ```
    pub async fn get_tools(&self) -> Result<P::ToolCollection, ComposioError> {
        // 1. Get meta tools schemas from API
        let schemas = self.get_meta_tools().await?;
        
        // 2. Convert to provider format
        let tools = self.client.provider().wrap_tools(schemas);
        
        Ok(tools)
    }
    
    /// Get tools for a specific toolkit
    pub async fn get_toolkit_tools(
        &self,
        toolkit: impl Into<String>,
    ) -> Result<P::ToolCollection, ComposioError> {
        // Implementation similar to get_tools
        todo!()
    }
}
```

---

## 🎯 Uso Prático

### Exemplo 1: OpenAI Chat Completions

```rust
use composio_sdk::{ComposioClient, OpenAIProvider};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client with OpenAI provider
    let client = ComposioClient::builder()
        .api_key("your_key")
        .build()?;
    
    // 2. Create session
    let session = client
        .create_session("user_123")
        .toolkits(vec!["github"])
        .send()
        .await?;
    
    // 3. Get tools in OpenAI format
    let tools = session.get_tools().await?;
    // tools: Vec<ChatCompletionToolParam>
    
    // 4. Use with OpenAI SDK
    let response = openai_client
        .chat()
        .completions()
        .create(json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "Create an issue"}],
            "tools": tools  // ← OpenAI format
        }))
        .await?;
    
    // 5. Execute tool calls manually
    if let Some(tool_calls) = response.choices[0].message.tool_calls {
        for tool_call in tool_calls {
            let result = session.execute_tool(
                &tool_call.function.name,
                serde_json::from_str(&tool_call.function.arguments)?
            ).await?;
            
            println!("Result: {:?}", result);
        }
    }
    
    Ok(())
}
```

### Exemplo 2: Anthropic Claude

```rust
use composio_sdk::{ComposioClient, AnthropicProvider};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client with Anthropic provider
    let client = ComposioClient::with_provider(AnthropicProvider::new())
        .api_key("your_key")
        .build()?;
    
    // 2. Create session
    let session = client
        .create_session("user_123")
        .toolkits(vec!["github"])
        .send()
        .await?;
    
    // 3. Get tools in Anthropic format
    let tools = session.get_tools().await?;
    // tools: Vec<AnthropicTool>
    
    // 4. Use with Anthropic SDK
    let response = anthropic_client
        .messages()
        .create(json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Create an issue"}],
            "tools": tools  // ← Anthropic format
        }))
        .await?;
    
    // 5. Execute tool calls
    // Similar to OpenAI example
    
    Ok(())
}
```

### Exemplo 3: Múltiplos Providers

```rust
use composio_sdk::{ComposioClient, OpenAIProvider, AnthropicProvider};

async fn process_with_openai(user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder().api_key("key").build()?;
    let session = client.create_session(user_id).send().await?;
    let tools = session.get_tools().await?;
    // tools: Vec<ChatCompletionToolParam>
    Ok(())
}

async fn process_with_anthropic(user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::with_provider(AnthropicProvider::new())
        .api_key("key")
        .build()?;
    let session = client.create_session(user_id).send().await?;
    let tools = session.get_tools().await?;
    // tools: Vec<AnthropicTool>
    Ok(())
}
```

---

## 🚀 Vantagens da Implementação Rust

### 1. **Zero-Cost Abstractions**
```rust
// Trait é resolvido em compile-time
// Sem overhead de runtime!
impl Provider for OpenAIProvider {
    fn wrap_tool(&self, tool: &ToolSchema) -> ChatCompletionToolParam {
        // Código inline, sem virtual dispatch
    }
}
```

### 2. **Type Safety**
```rust
// Tipos inferidos automaticamente
let client = ComposioClient::with_provider(OpenAIProvider::new());
let tools = session.get_tools().await?;
// ↑ Compilador sabe que tools: Vec<ChatCompletionToolParam>

// Erro em compile-time se tentar usar tipo errado
let anthropic_tools: Vec<AnthropicTool> = tools; // ❌ Erro!
```

### 3. **Memory Safety**
```rust
// Sem race conditions, sem memory leaks
// Ownership garante segurança
let provider = OpenAIProvider::new();
let client = ComposioClient::with_provider(provider);
// provider movido, não pode ser usado novamente
```

### 4. **Performance**
```rust
// Conversão inline, sem alocações desnecessárias
fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Vec<ChatCompletionToolParam> {
    tools.into_iter()  // ← Consome vetor original
        .map(|t| self.wrap_tool(&t))  // ← Inline
        .collect()  // ← Aloca uma vez
}
```

---

## 📊 Comparação: Python vs Rust

| Aspecto | Python | Rust |
|---------|--------|------|
| **Type Safety** | Runtime (typing hints) | Compile-time (traits) |
| **Performance** | Interpretado | Compilado (zero-cost) |
| **Memory** | GC overhead | Stack allocation |
| **Abstractions** | Virtual dispatch | Monomorphization |
| **Errors** | Runtime exceptions | Compile-time errors |
| **Concurrency** | GIL limitations | Fearless concurrency |

---

## 🎯 Próximos Passos

1. ✅ Definir trait `Provider`
2. ✅ Implementar `OpenAIProvider`
3. ✅ Implementar `AnthropicProvider`
4. ⏳ Integrar com `ComposioClient`
5. ⏳ Integrar com `Session`
6. ⏳ Adicionar testes
7. ⏳ Documentar padrões
8. ⏳ Adicionar mais providers (Google, etc.)

Pronto para implementar! 🚀
