# Análise Comparativa: Testes MCP Python vs Rust

## 📋 Resumo Executivo

Esta análise compara os testes de integração MCP do SDK Python (`conftest.py` e `test_mcp.py`) com a implementação atual do SDK Rust, classificando funcionalidades em três categorias: **Existe**, **Diferente** e **Não Tem**.

---

## 🔍 Análise do `conftest.py`

### Funcionalidades Identificadas

| Funcionalidade | Status | Descrição |
|----------------|--------|-----------|
| **Fixtures de configuração** | ❌ Não Tem | Sistema de fixtures pytest para compartilhar recursos entre testes |
| **Validação de API Key** | ✅ Existe | Rust usa variáveis de ambiente, mas sem validação automática no conftest |
| **Cliente compartilhado** | ⚠️ Diferente | Python usa fixture de sessão; Rust cria cliente por teste |
| **Cleanup automático** | ❌ Não Tem | Python tem `mcp_server_cleanup` fixture para limpar recursos criados |
| **Dados de teste reutilizáveis** | ⚠️ Diferente | Python usa fixtures; Rust usa funções helper ou constantes |
| **Setup de ambiente** | ⚠️ Diferente | Python usa `setup_environment` fixture; Rust usa `#[tokio::test]` |

### Relevância de Implementação


#### 🔴 Alta Prioridade

1. **Cleanup Automático de Recursos**
   - **Por quê**: Evita poluição de dados de teste na API real
   - **Impacto**: Testes mais limpos e confiáveis
   - **Implementação Rust**: Usar `Drop` trait ou fixtures com `defer`

2. **Validação de API Key**
   - **Por quê**: Falha rápida se configuração está incorreta
   - **Impacto**: Melhor experiência do desenvolvedor
   - **Implementação Rust**: Macro ou função helper no início dos testes

#### 🟡 Média Prioridade

3. **Cliente Compartilhado**
   - **Por quê**: Reduz overhead de criação de clientes
   - **Impacto**: Testes mais rápidos
   - **Implementação Rust**: `lazy_static` ou `once_cell` para singleton

4. **Dados de Teste Reutilizáveis**
   - **Por quê**: Consistência entre testes
   - **Impacto**: Manutenção mais fácil
   - **Implementação Rust**: Módulo `test_fixtures` com funções helper

---

## 🔍 Análise do `test_mcp.py`

### Estrutura de Classes de Teste


| Classe Python | Funcionalidades | Status Rust | Observações |
|---------------|-----------------|-------------|-------------|
| `TestMCPStructure` | Verifica namespace e métodos MCP | ❌ Não Tem | Rust tem tipos, mas sem testes de estrutura |
| `TestMCPOperations` | CRUD completo de MCP | ⚠️ Parcial | Rust tem modelos, mas sem implementação de cliente |
| `TestMCPErrorHandling` | Tratamento de erros | ✅ Existe | Rust tem testes de erro em `session_creation_test.rs` |
| `TestMCPNoAuthToolkits` | Toolkits sem autenticação | ❌ Não Tem | Caso de uso importante não coberto |
| `TestMCPRealWorldScenarios` | Workflows completos | ⚠️ Parcial | Rust tem alguns, mas não específicos para MCP |

### Funcionalidades Detalhadas

#### 1. TestMCPStructure

| Teste | Status | Relevância | Implementação Rust |
|-------|--------|------------|-------------------|
| `test_mcp_namespace_exists` | ❌ Não Tem | 🔴 Alta | Verificar que `client.mcp()` existe |
| `test_mcp_methods_available` | ❌ Não Tem | 🔴 Alta | Verificar métodos: create, list, get, update, delete, generate |

**Relevância**: 🔴 **CRÍTICA**
- Garante que a API pública está completa
- Previne regressões em refatorações
- Documentação viva da interface

**Implementação Rust**:
```rust
#[test]
fn test_mcp_namespace_exists() {
    let client = ComposioClient::builder()
        .api_key("test")
        .build()
        .unwrap();
    
    // Verifica que o método mcp() existe e retorna o tipo correto
    let _mcp = client.mcp();
}
```


#### 2. TestMCPOperations

| Teste | Status | Relevância | Notas |
|-------|--------|------------|-------|
| `test_list_mcp_configs` | ❌ Não Tem | 🔴 Alta | Listar servidores MCP com paginação |
| `test_list_with_pagination` | ❌ Não Tem | 🟡 Média | Paginação é importante para escala |
| `test_list_with_filters` | ❌ Não Tem | 🟡 Média | Filtros por toolkit, nome, auth_config |
| `test_create_mcp_config` | ❌ Não Tem | 🔴 Alta | Criar servidor MCP |
| `test_get_nonexistent_config` | ✅ Existe | ✅ | Similar aos testes de erro existentes |
| `test_create_with_empty_toolkits` | ✅ Existe | ✅ | Validação de entrada |
| `test_generate_method_directly` | ❌ Não Tem | 🔴 Alta | Gerar URL MCP para usuário |
| `test_create_with_string_toolkits` | ❌ Não Tem | 🟡 Média | API simplificada com strings |
| `test_create_with_mixed_toolkits` | ❌ Não Tem | 🟡 Média | Flexibilidade de API |
| `test_create_response_structure` | ⚠️ Parcial | 🟡 Média | Rust tem tipos, mas sem validação completa |

**Relevância**: 🔴 **CRÍTICA**
- Operações CRUD são o core da funcionalidade MCP
- Sem isso, o SDK não pode gerenciar servidores MCP
- Python SDK tem implementação completa

**Implementação Rust Necessária**:

1. **Cliente MCP** (não existe ainda):
```rust
// Em src/client.rs
impl ComposioClient {
    pub fn mcp(&self) -> McpClient {
        McpClient::new(self)
    }
}
```

2. **McpClient struct** (criar novo arquivo `src/mcp_client.rs`):
```rust
pub struct McpClient<'a> {
    client: &'a ComposioClient,
}

impl<'a> McpClient<'a> {
    pub async fn create(&self, params: MCPCreateParams) -> Result<MCPCreateResponse>;
    pub async fn list(&self, params: MCPListParams) -> Result<MCPListResponse>;
    pub async fn get(&self, id: &str) -> Result<MCPItem>;
    pub async fn update(&self, id: &str, params: MCPUpdateParams) -> Result<MCPUpdateResponse>;
    pub async fn delete(&self, id: &str) -> Result<MCPDeleteResponse>;
    pub async fn generate(&self, user_id: &str, server_id: &str, options: Option<HashMap<String, Value>>) -> Result<MCPServerInstance>;
}
```


#### 3. TestMCPErrorHandling

| Teste | Status | Relevância | Notas |
|-------|--------|------------|-------|
| `test_invalid_config_ids` | ✅ Existe | ✅ | Rust tem testes similares em `session_creation_test.rs` |
| `test_generate_with_invalid_params` | ⚠️ Parcial | 🟡 Média | Precisa adaptar para MCP |
| `test_create_with_invalid_toolkit_config` | ✅ Existe | ✅ | Validação de entrada já existe |

**Relevância**: 🟢 **BOA COBERTURA**
- Rust já tem boa infraestrutura de tratamento de erros
- Apenas precisa adaptar para casos específicos de MCP

#### 4. TestMCPNoAuthToolkits

| Teste | Status | Relevância | Notas |
|-------|--------|------------|-------|
| `test_mcp_with_no_auth_toolkits` | ❌ Não Tem | 🔴 Alta | Caso de uso importante: toolkits sem auth |
| `test_mcp_with_string_toolkits` | ❌ Não Tem | 🟡 Média | API simplificada |

**Relevância**: 🔴 **ALTA**
- Toolkits sem autenticação (composio_search, text_to_pdf) são casos de uso comuns
- Simplifica onboarding de novos usuários
- Permite testes sem configurar OAuth

**Implementação Rust**:
```rust
#[tokio::test]
async fn test_mcp_with_no_auth_toolkits() {
    let client = create_test_client();
    
    let server = client.mcp().create(
        MCPCreateParams {
            name: "test-no-auth".to_string(),
            toolkits: vec!["composio_search".to_string(), "text_to_pdf".to_string()],
            auth_config_ids: None,
            custom_tools: Some(vec![
                "COMPOSIO_SEARCH_DUCK_DUCK_GO_SEARCH".to_string(),
                "TEXT_TO_PDF_CONVERT_TEXT_TO_PDF".to_string(),
            ]),
            managed_auth_via_composio: Some(false),
        }
    ).await.unwrap();
    
    assert!(!server.id.is_empty());
    assert_eq!(server.auth_config_ids.len(), 0);
}
```


#### 5. TestMCPRealWorldScenarios

| Teste | Status | Relevância | Notas |
|-------|--------|------------|-------|
| `test_full_workflow_with_no_auth_toolkits` | ❌ Não Tem | 🔴 Alta | Workflow completo: create → generate → use |
| `test_api_compatibility_with_typescript` | ❌ Não Tem | 🟡 Média | Garante compatibilidade entre SDKs |
| `test_full_crud_cycle` | ❌ Não Tem | 🔴 Alta | Create → Get → Update → Get → Delete |

**Relevância**: 🔴 **ALTA**
- Testes de integração end-to-end são cruciais
- Validam que o fluxo completo funciona
- Detectam problemas de integração entre componentes

**Implementação Rust**:
```rust
#[tokio::test]
async fn test_full_mcp_workflow() {
    let client = create_test_client();
    
    // 1. Create MCP server
    let server = client.mcp().create(/* ... */).await.unwrap();
    
    // 2. Generate instance for user
    let instance = client.mcp().generate(
        "test_user_123",
        &server.id,
        None
    ).await.unwrap();
    
    // 3. Verify instance
    assert_eq!(instance.user_id, "test_user_123");
    assert_eq!(instance.server_type, "streamable_http");
    assert!(!instance.url.is_empty());
    
    // 4. Cleanup
    client.mcp().delete(&server.id).await.unwrap();
}
```

---

## 📊 Resumo Quantitativo

### Cobertura por Categoria

| Categoria | Total | Existe | Diferente | Não Tem | % Cobertura |
|-----------|-------|--------|-----------|---------|-------------|
| **conftest.py** | 6 | 1 | 3 | 2 | 67% |
| **TestMCPStructure** | 2 | 0 | 0 | 2 | 0% |
| **TestMCPOperations** | 10 | 2 | 1 | 7 | 30% |
| **TestMCPErrorHandling** | 3 | 2 | 1 | 0 | 100% |
| **TestMCPNoAuthToolkits** | 2 | 0 | 0 | 2 | 0% |
| **TestMCPRealWorldScenarios** | 3 | 0 | 0 | 3 | 0% |
| **TOTAL** | 26 | 5 | 5 | 16 | 38% |


### Priorização de Implementação

#### 🔴 Prioridade CRÍTICA (Implementar Primeiro)

1. **Cliente MCP Base** (`McpClient`)
   - Métodos: create, list, get, update, delete, generate
   - Estimativa: 2-3 dias
   - Bloqueador: Sem isso, nenhum teste MCP funciona

2. **Testes de Estrutura** (`TestMCPStructure`)
   - Validar que API pública está completa
   - Estimativa: 4 horas
   - Benefício: Previne regressões

3. **Testes de Operações CRUD** (`TestMCPOperations`)
   - Criar, listar, obter, atualizar, deletar
   - Estimativa: 1-2 dias
   - Benefício: Valida funcionalidade core

4. **Workflow Completo** (`test_full_workflow`)
   - Create → Generate → Use → Delete
   - Estimativa: 4 horas
   - Benefício: Valida integração end-to-end

#### 🟡 Prioridade MÉDIA (Implementar Depois)

5. **Cleanup Automático**
   - Fixture para limpar recursos de teste
   - Estimativa: 4 horas
   - Benefício: Testes mais limpos

6. **Testes de Paginação e Filtros**
   - List com paginação e filtros
   - Estimativa: 4 horas
   - Benefício: Valida casos de uso avançados

7. **Testes de Toolkits Sem Auth**
   - Casos de uso simplificados
   - Estimativa: 4 horas
   - Benefício: Melhor cobertura de casos de uso

#### 🟢 Prioridade BAIXA (Nice to Have)

8. **Cliente Compartilhado**
   - Singleton para testes
   - Estimativa: 2 horas
   - Benefício: Testes mais rápidos

9. **Compatibilidade TypeScript**
   - Validar que APIs são consistentes
   - Estimativa: 2 horas
   - Benefício: Melhor experiência multi-linguagem

---

## 🏗️ Plano de Implementação

### Fase 1: Fundação (Semana 1)

**Objetivo**: Criar infraestrutura básica de MCP

1. **Criar `src/mcp_client.rs`**
   ```rust
   pub struct McpClient<'a> {
       client: &'a ComposioClient,
   }
   ```

2. **Implementar métodos básicos**
   - `create()` - Criar servidor MCP
   - `get()` - Obter servidor por ID
   - `list()` - Listar servidores
   - `delete()` - Deletar servidor

3. **Adicionar ao `ComposioClient`**
   ```rust
   impl ComposioClient {
       pub fn mcp(&self) -> McpClient {
           McpClient::new(self)
       }
   }
   ```


### Fase 2: Testes Básicos (Semana 1-2)

**Objetivo**: Validar funcionalidade core

1. **Criar `tests/mcp_operations_test.rs`**
   - test_mcp_namespace_exists
   - test_mcp_create
   - test_mcp_get
   - test_mcp_list
   - test_mcp_delete

2. **Criar `tests/mcp_structure_test.rs`**
   - test_mcp_methods_available
   - test_mcp_types_exist

3. **Usar WireMock para mocks**
   - Seguir padrão de `session_creation_test.rs`
   - Mockar respostas da API

### Fase 3: Funcionalidades Avançadas (Semana 2)

**Objetivo**: Implementar casos de uso avançados

1. **Método `generate()`**
   - Gerar URLs MCP para usuários
   - Suportar opções customizadas

2. **Método `update()`**
   - Atualizar configuração de servidor
   - Validar mudanças

3. **Testes de No-Auth Toolkits**
   - test_mcp_with_no_auth_toolkits
   - test_mcp_with_string_toolkits

### Fase 4: Integração e Polimento (Semana 3)

**Objetivo**: Testes end-to-end e refinamento

1. **Testes de Workflow Completo**
   - test_full_mcp_workflow
   - test_full_crud_cycle

2. **Cleanup Automático**
   - Implementar Drop trait para recursos
   - Fixture de cleanup

3. **Documentação**
   - Exemplos em `examples/mcp_usage.rs`
   - Atualizar README

---

## 🎯 Aproveitando Arquitetura Rust

### Vantagens do Rust para Implementação

#### 1. **Type Safety**

**Python**:
```python
# Pode passar qualquer coisa, erro só em runtime
mcp_config = composio.mcp.create(
    name=123,  # Deveria ser string, mas aceita
    toolkits=None  # Deveria ser lista
)
```

**Rust**:
```rust
// Erro em compile-time
let params = MCPCreateParams {
    name: 123,  // ❌ Erro: expected String, found integer
    toolkits: None,  // ❌ Erro: expected Vec<String>
};
```

**Benefício**: Menos bugs, mais confiança


#### 2. **Builder Pattern**

**Python**:
```python
# Argumentos posicionais e keyword misturados
server = composio.mcp.create(
    "my-server",
    toolkits=["github"],
    allowed_tools=["GITHUB_CREATE_ISSUE"],
    manually_manage_connections=False
)
```

**Rust** (Proposta):
```rust
// Builder pattern fluente e type-safe
let server = client.mcp()
    .create("my-server")
    .toolkits(vec!["github"])
    .allowed_tools(vec!["GITHUB_CREATE_ISSUE"])
    .manually_manage_connections(false)
    .send()
    .await?;
```

**Benefício**: API mais ergonômica e autodocumentada

#### 3. **Error Handling**

**Python**:
```python
try:
    server = composio.mcp.get("invalid_id")
except ValidationError as e:
    print(f"Error: {e}")
except Exception as e:  # Catch-all necessário
    print(f"Unknown error: {e}")
```

**Rust**:
```rust
match client.mcp().get("invalid_id").await {
    Ok(server) => println!("Server: {:?}", server),
    Err(ComposioError::ValidationError { message, .. }) => {
        println!("Validation error: {}", message)
    }
    Err(ComposioError::ApiError { status, .. }) => {
        println!("API error: {}", status)
    }
    Err(e) => println!("Error: {}", e),
}
```

**Benefício**: Tratamento de erros explícito e exaustivo

#### 4. **Async/Await Nativo**

**Python**:
```python
# asyncio pode ser confuso com sync code
import asyncio

async def test():
    server = await composio.mcp.create(...)  # Precisa await
    
# Precisa event loop
asyncio.run(test())
```

**Rust**:
```rust
// Async é first-class citizen
#[tokio::test]
async fn test_mcp_create() {
    let server = client.mcp().create(...).await?;
    // Tokio gerencia tudo automaticamente
}
```

**Benefício**: Async mais natural e performático


#### 5. **Zero-Cost Abstractions**

**Python**:
```python
# Overhead de runtime para validação
def create(self, name: str, toolkits: List[str]):
    if not isinstance(name, str):
        raise TypeError("name must be string")
    if not isinstance(toolkits, list):
        raise TypeError("toolkits must be list")
    # ... validação em runtime
```

**Rust**:
```rust
// Validação em compile-time, zero overhead em runtime
pub fn create(&self, name: impl Into<String>, toolkits: Vec<String>) {
    // Tipos garantidos pelo compilador
    // Sem overhead de validação
}
```

**Benefício**: Performance sem sacrificar segurança

#### 6. **Ownership e Lifetime**

**Python**:
```python
# Pode ter problemas com referências
server = composio.mcp.create(...)
del composio  # Server ainda referencia composio?
print(server.id)  # Pode dar erro
```

**Rust**:
```rust
// Lifetime garante que referências são válidas
pub struct McpClient<'a> {
    client: &'a ComposioClient,  // Lifetime explícito
}

// Compilador garante que client vive mais que McpClient
let server = client.mcp().create(...).await?;
// Não compila se client for dropado antes
```

**Benefício**: Sem dangling pointers ou use-after-free

#### 7. **Trait System**

**Python**:
```python
# Duck typing - erro só em runtime
def process_server(server):
    print(server.id)  # Espera que tenha .id
    print(server.name)  # Espera que tenha .name
```

**Rust**:
```rust
// Trait bounds garantem interface
trait McpServer {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}

fn process_server<T: McpServer>(server: &T) {
    println!("{}", server.id());
    println!("{}", server.name());
}
// Erro em compile-time se T não implementa McpServer
```

**Benefício**: Polimorfismo type-safe

---

## 🚀 Implementação Proposta

### Estrutura de Arquivos

```
src/
├── mcp_client.rs          # ⭐ NOVO: Cliente MCP
├── client.rs              # Adicionar método mcp()
├── models/
│   └── mcp.rs            # ✅ JÁ EXISTE: Tipos MCP
└── lib.rs                # Exportar McpClient

tests/
├── mcp_structure_test.rs  # ⭐ NOVO: Testes de estrutura
├── mcp_operations_test.rs # ⭐ NOVO: Testes CRUD
├── mcp_workflows_test.rs  # ⭐ NOVO: Testes end-to-end
└── test_helpers/
    └── mcp_fixtures.rs    # ⭐ NOVO: Fixtures compartilhadas

examples/
└── mcp_usage.rs          # ✅ JÁ EXISTE: Atualizar com novos métodos
```


### Código de Exemplo: McpClient

```rust
// src/mcp_client.rs

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
    ///     .toolkits(vec!["github", "slack"])
    ///     .allowed_tools(vec!["GITHUB_CREATE_ISSUE"])
    ///     .send()
    ///     .await?;
    /// 
    /// println!("Created server: {}", server.id);
    /// # Ok(())
    /// # }
    /// ```
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
    pub async fn update(&self, id: &str, params: MCPUpdateParams) -> Result<MCPUpdateResponse, ComposioError> {
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


### Código de Exemplo: Testes

```rust
// tests/mcp_operations_test.rs

use composio_sdk::client::ComposioClient;
use composio_sdk::models::mcp::*;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_namespace_exists() {
    let client = ComposioClient::builder()
        .api_key("test_key")
        .build()
        .unwrap();
    
    // Verifica que o método mcp() existe
    let _mcp = client.mcp();
}

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
    assert_eq!(server.toolkits, vec!["github"]);
}

#[tokio::test]
async fn test_mcp_list_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/mcp/servers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [
                {
                    "id": "mcp_1",
                    "name": "server-1",
                    "allowed_tools": [],
                    "auth_config_ids": [],
                    "toolkits": ["github"],
                    "commands": {},
                    "mcp_url": "https://mcp.composio.dev/mcp_1",
                    "toolkit_icons": {},
                    "server_instance_count": 0
                }
            ],
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

    let params = MCPListParams {
        page_no: Some(1),
        limit: Some(10),
        ..Default::default()
    };

    let response = client.mcp().list(params).await.unwrap();

    assert_eq!(response.current_page, 1);
    assert_eq!(response.items.len(), 1);
}

#[tokio::test]
async fn test_mcp_get_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v3/mcp/mcp_abc123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "mcp_abc123",
            "name": "test-server",
            "allowed_tools": [],
            "auth_config_ids": [],
            "toolkits": ["github"],
            "commands": {},
            "mcp_url": "https://mcp.composio.dev/mcp_abc123",
            "toolkit_icons": {},
            "server_instance_count": 0
        })))
        .mount(&mock_server)
        .await;

    let client = ComposioClient::builder()
        .api_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let server = client.mcp().get("mcp_abc123").await.unwrap();

    assert_eq!(server.id, "mcp_abc123");
    assert_eq!(server.name, "test-server");
}

#[tokio::test]
async fn test_mcp_generate_instance() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v3/mcp/servers/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "mcp_abc123",
            "name": "test-server",
            "type": "streamable_http",
            "url": "https://mcp.composio.dev/instance/user_123",
            "user_id": "user_123",
            "allowed_tools": ["GITHUB_CREATE_ISSUE"],
            "auth_configs": []
        })))
        .mount(&mock_server)
        .await;

    let client = ComposioClient::builder()
        .api_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let instance = client.mcp()
        .generate("user_123", "mcp_abc123", None)
        .await
        .unwrap();

    assert_eq!(instance.user_id, "user_123");
    assert_eq!(instance.server_type, "streamable_http");
    assert!(!instance.url.is_empty());
}

#[tokio::test]
async fn test_mcp_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v3/mcp/mcp_abc123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "mcp_abc123",
            "deleted": true
        })))
        .mount(&mock_server)
        .await;

    let client = ComposioClient::builder()
        .api_key("test_key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let response = client.mcp().delete("mcp_abc123").await.unwrap();

    assert_eq!(response.id, "mcp_abc123");
    assert!(response.deleted);
}

#[tokio::test]
async fn test_mcp_with_no_auth_toolkits() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v3/mcp/servers"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "mcp_noauth",
            "name": "no-auth-server",
            "allowed_tools": [
                "COMPOSIO_SEARCH_DUCK_DUCK_GO_SEARCH",
                "TEXT_TO_PDF_CONVERT_TEXT_TO_PDF"
            ],
            "auth_config_ids": [],
            "toolkits": ["composio_search", "text_to_pdf"],
            "mcp_url": "https://mcp.composio.dev/mcp_noauth",
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
        .create("no-auth-server")
        .toolkits(vec!["composio_search", "text_to_pdf"])
        .allowed_tools(vec![
            "COMPOSIO_SEARCH_DUCK_DUCK_GO_SEARCH",
            "TEXT_TO_PDF_CONVERT_TEXT_TO_PDF"
        ])
        .manually_manage_connections(false)
        .send()
        .await
        .unwrap();

    assert_eq!(server.auth_config_ids.len(), 0);
    assert_eq!(server.toolkits.len(), 2);
}
```

