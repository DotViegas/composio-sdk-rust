# 🚀 Guia Rápido: Implementação MCP no Rust SDK

## 📋 Checklist Rápido

### ✅ Pré-requisitos

- [ ] Rust 1.70+ instalado
- [ ] Projeto composio-sdk clonado
- [ ] Análise completa revisada
- [ ] Branch `feature/mcp-client` criada

### 🎯 Objetivo

Implementar cliente MCP completo no SDK Rust, alcançando 100% de paridade com Python SDK.

---

## 📂 Arquivos a Criar/Modificar

### Criar Novos Arquivos

```bash
# Cliente MCP
touch src/mcp_client.rs

# Testes
touch tests/mcp_structure_test.rs
touch tests/mcp_operations_test.rs
touch tests/mcp_workflows_test.rs
```

### Modificar Arquivos Existentes

```bash
# Adicionar método mcp()
src/client.rs

# Exportar McpClient
src/lib.rs

# Atualizar exemplo
examples/mcp_usage.rs
```

---

## 🏗️ Implementação Passo a Passo

### Passo 1: Criar `src/mcp_client.rs`

```rust
//! MCP client for managing Model Context Protocol servers

use crate::client::ComposioClient;
use crate::error::ComposioError;
use crate::models::mcp::*;
use std::collections::HashMap;
use serde_json::Value;

/// MCP client for managing Model Context Protocol servers
pub struct McpClient<'a> {
    client: &'a ComposioClient,
}

impl<'a> McpClient<'a> {
    /// Create a new MCP client
    pub(crate) fn new(client: &'a ComposioClient) -> Self {
        Self { client }
    }

    /// Create a new MCP server
    pub fn create(&self, name: impl Into<String>) -> McpCreateBuilder<'a> {
        McpCreateBuilder::new(self, name.into())
    }

    /// List MCP servers
    pub async fn list(&self, params: MCPListParams) -> Result<MCPListResponse, ComposioError> {
        let url = format!("{}/api/v3/mcp/servers", self.client.base_url());
        
        let response = self.client
            .http_client()
            .get(&url)
            .query(&params)
            .send()
            .await?;

        self.client.handle_response(response).await
    }

    /// Get MCP server by ID
    pub async fn get(&self, id: &str) -> Result<MCPItem, ComposioError> {
        let url = format!("{}/api/v3/mcp/{}", self.client.base_url(), id);
        
        let response = self.client
            .http_client()
            .get(&url)
            .send()
            .await?;

        self.client.handle_response(response).await
    }

    /// Update MCP server
    pub async fn update(
        &self,
        id: &str,
        params: MCPUpdateParams,
    ) -> Result<MCPUpdateResponse, ComposioError> {
        let url = format!("{}/api/v3/mcp/{}", self.client.base_url(), id);
        
        let response = self.client
            .http_client()
            .patch(&url)
            .json(&params)
            .send()
            .await?;

        self.client.handle_response(response).await
    }

    /// Delete MCP server
    pub async fn delete(&self, id: &str) -> Result<MCPDeleteResponse, ComposioError> {
        let url = format!("{}/api/v3/mcp/{}", self.client.base_url(), id);
        
        let response = self.client
            .http_client()
            .delete(&url)
            .send()
            .await?;

        self.client.handle_response(response).await
    }

    /// Generate MCP server instance for a user
    pub async fn generate(
        &self,
        user_id: &str,
        server_id: &str,
        options: Option<HashMap<String, Value>>,
    ) -> Result<MCPServerInstance, ComposioError> {
        let url = format!("{}/api/v3/mcp/servers/generate", self.client.base_url());
        
        let mut body = serde_json::json!({
            "mcp_server_id": server_id,
            "user_ids": [user_id],
        });

        if let Some(opts) = options {
            if let Some(obj) = body.as_object_mut() {
                for (key, value) in opts {
                    obj.insert(key, value);
                }
            }
        }

        let response = self.client
            .http_client()
            .post(&url)
            .json(&body)
            .send()
            .await?;

        self.client.handle_response(response).await
    }
}

/// Builder for creating MCP servers
pub struct McpCreateBuilder<'a> {
    client: &'a McpClient<'a>,
    name: String,
    toolkits: Vec<String>,
    auth_config_ids: Option<Vec<String>>,
    allowed_tools: Option<Vec<String>>,
    manually_manage_connections: Option<bool>,
}

impl<'a> McpCreateBuilder<'a> {
    fn new(client: &'a McpClient<'a>, name: String) -> Self {
        Self {
            client,
            name,
            toolkits: Vec::new(),
            auth_config_ids: None,
            allowed_tools: None,
            manually_manage_connections: None,
        }
    }

    /// Set toolkits
    pub fn toolkits(mut self, toolkits: Vec<impl Into<String>>) -> Self {
        self.toolkits = toolkits.into_iter().map(|t| t.into()).collect();
        self
    }

    /// Set auth config IDs
    pub fn auth_configs(mut self, ids: Vec<impl Into<String>>) -> Self {
        self.auth_config_ids = Some(ids.into_iter().map(|i| i.into()).collect());
        self
    }

    /// Set allowed tools
    pub fn allowed_tools(mut self, tools: Vec<impl Into<String>>) -> Self {
        self.allowed_tools = Some(tools.into_iter().map(|t| t.into()).collect());
        self
    }

    /// Set manually manage connections
    pub fn manually_manage_connections(mut self, value: bool) -> Self {
        self.manually_manage_connections = Some(value);
        self
    }

    /// Send the request
    pub async fn send(self) -> Result<MCPCreateResponse, ComposioError> {
        let params = MCPCreateParams {
            name: self.name,
            toolkits: self.toolkits,
            auth_config_ids: self.auth_config_ids,
            custom_tools: self.allowed_tools,
            managed_auth_via_composio: self.manually_manage_connections,
        };

        let url = format!("{}/api/v3/mcp/servers", self.client.client.base_url());
        
        let response = self.client.client
            .http_client()
            .post(&url)
            .json(&params)
            .send()
            .await?;

        self.client.client.handle_response(response).await
    }
}
```


### Passo 2: Modificar `src/client.rs`

Adicione o método `mcp()` ao `ComposioClient`:

```rust
// No início do arquivo, adicione o import
use crate::mcp_client::McpClient;

// Na implementação de ComposioClient, adicione:
impl ComposioClient {
    // ... métodos existentes ...

    /// Get MCP client for managing Model Context Protocol servers
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use composio_sdk::Composio;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Composio::builder().api_key("key").build()?;
    /// 
    /// let server = client.mcp()
    ///     .create("my-server")
    ///     .toolkits(vec!["github"])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mcp(&self) -> McpClient {
        McpClient::new(self)
    }
}
```

### Passo 3: Modificar `src/lib.rs`

Adicione os exports necessários:

```rust
// Adicione o módulo
pub mod mcp_client;

// Adicione aos exports públicos
pub use mcp_client::{McpClient, McpCreateBuilder};
```

### Passo 4: Criar Teste Básico

Crie `tests/mcp_structure_test.rs`:

```rust
//! Tests for MCP client structure and availability

use composio_sdk::client::ComposioClient;

#[test]
fn test_mcp_namespace_exists() {
    let client = ComposioClient::builder()
        .api_key("test_key")
        .build()
        .unwrap();
    
    // Verifica que o método mcp() existe
    let _mcp = client.mcp();
}

#[test]
fn test_mcp_client_has_required_methods() {
    let client = ComposioClient::builder()
        .api_key("test_key")
        .build()
        .unwrap();
    
    let mcp = client.mcp();
    
    // Verifica que os métodos existem (não executa)
    // Isso garante que a API pública está completa
    let _ = mcp.create("test");
    // list, get, update, delete, generate são async, 
    // então não podemos testar aqui sem executar
}
```

### Passo 5: Compilar e Testar

```bash
# Compilar
cargo build

# Rodar testes
cargo test mcp_structure

# Verificar warnings
cargo clippy

# Formatar código
cargo fmt
```

---

## 🧪 Testes Completos

### Criar `tests/mcp_operations_test.rs`

```rust
//! Integration tests for MCP operations

use composio_sdk::client::ComposioClient;
use composio_sdk::models::mcp::*;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_create_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v3/mcp/servers"))
        .and(header("x-api-key", "test_key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "mcp_abc123",
            "name": "test-server",
            "allowed_tools": ["GITHUB_CREATE_ISSUE"],
            "auth_config_ids": [],
            "toolkits": ["github"],
            "mcp_url": "https://mcp.composio.dev/mcp_abc123",
            "created_at": "2024-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = ComposioClient::builder()
        .api_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let server = client.mcp()
        .create("test-server")
        .toolkits(vec!["github"])
        .allowed_tools(vec!["GITHUB_CREATE_ISSUE"])
        .send()
        .await
        .unwrap();

    assert_eq!(server.id, "mcp_abc123");
    assert_eq!(server.name, "test-server");
}

#[tokio::test]
async fn test_mcp_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/mcp/servers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [],
            "current_page": 1,
            "total_pages": 1
        })))
        .mount(&mock_server)
        .await;

    let client = ComposioClient::builder()
        .api_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let response = client.mcp()
        .list(Default::default())
        .await
        .unwrap();

    assert_eq!(response.current_page, 1);
}

// Adicione mais testes conforme necessário...
```

---

## 📝 Exemplo de Uso

Atualize `examples/mcp_usage.rs`:

```rust
//! Example demonstrating MCP client usage

use composio_sdk::Composio;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize client
    let client = Composio::builder()
        .api_key(std::env::var("COMPOSIO_API_KEY")?)
        .build()?;

    println!("🚀 MCP Client Example\n");

    // 1. Create MCP server
    println!("1. Creating MCP server...");
    let server = client.mcp()
        .create("rust-example-server")
        .toolkits(vec!["github", "slack"])
        .allowed_tools(vec![
            "GITHUB_CREATE_ISSUE",
            "SLACK_SEND_MESSAGE"
        ])
        .manually_manage_connections(false)
        .send()
        .await?;

    println!("   ✅ Created: {} ({})", server.name, server.id);

    // 2. List servers
    println!("\n2. Listing MCP servers...");
    let servers = client.mcp()
        .list(Default::default())
        .await?;

    println!("   ✅ Found {} servers", servers.items.len());

    // 3. Get server details
    println!("\n3. Getting server details...");
    let details = client.mcp()
        .get(&server.id)
        .await?;

    println!("   ✅ Server: {}", details.name);
    println!("   Toolkits: {:?}", details.toolkits);

    // 4. Generate instance for user
    println!("\n4. Generating instance for user...");
    let instance = client.mcp()
        .generate("user_123", &server.id, None)
        .await?;

    println!("   ✅ Instance URL: {}", instance.url);
    println!("   User ID: {}", instance.user_id);

    // 5. Delete server
    println!("\n5. Cleaning up...");
    let delete_response = client.mcp()
        .delete(&server.id)
        .await?;

    println!("   ✅ Deleted: {}", delete_response.deleted);

    println!("\n🎉 Example completed successfully!");

    Ok(())
}
```

---

## ✅ Validação

### Checklist de Validação

- [ ] Código compila sem erros
- [ ] Código compila sem warnings
- [ ] Testes passam: `cargo test`
- [ ] Clippy não reporta issues: `cargo clippy`
- [ ] Código formatado: `cargo fmt --check`
- [ ] Exemplo funciona: `cargo run --example mcp_usage`
- [ ] Documentação gerada: `cargo doc --open`

### Comandos de Validação

```bash
# Compilação
cargo build --release

# Testes
cargo test --all

# Linting
cargo clippy -- -D warnings

# Formatação
cargo fmt --all -- --check

# Documentação
cargo doc --no-deps --open

# Exemplo
COMPOSIO_API_KEY=your_key cargo run --example mcp_usage
```

---

## 🐛 Troubleshooting

### Erro: "método mcp não encontrado"

**Solução**: Verifique que `mcp_client` está exportado em `src/lib.rs`

```rust
pub mod mcp_client;
pub use mcp_client::McpClient;
```

### Erro: "lifetime mismatch"

**Solução**: Verifique que `McpClient` tem lifetime `'a`:

```rust
pub struct McpClient<'a> {
    client: &'a ComposioClient,
}
```

### Erro de compilação em testes

**Solução**: Adicione dependências de teste em `Cargo.toml`:

```toml
[dev-dependencies]
wiremock = "0.5"
tokio-test = "0.4"
```

---

## 📚 Próximos Passos

Após completar a implementação básica:

1. ✅ Implementar testes de erro
2. ✅ Adicionar testes de workflow completo
3. ✅ Documentar todos os métodos públicos
4. ✅ Criar guia de migração Python → Rust
5. ✅ Atualizar CHANGELOG.md
6. ✅ Preparar release notes

---

## 🎯 Meta Final

**Objetivo**: SDK Rust com 100% de paridade MCP com Python

**Critérios de Sucesso**:
- ✅ Todos os métodos MCP implementados
- ✅ Cobertura de testes > 90%
- ✅ Documentação completa
- ✅ Exemplos funcionais
- ✅ Zero warnings do compilador

**Tempo Estimado**: 2 semanas (9 dias úteis)

