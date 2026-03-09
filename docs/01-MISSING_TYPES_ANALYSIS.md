# Análise de Tipos Faltantes - Python vs Rust

## 📋 Visão Geral

Este documento compara os tipos definidos no SDK Python (`temp/composio/client/types.py`) com a implementação atual em Rust (`src/models/`).

## ✅ Tipos Já Implementados

### Enums Básicos
- ✅ `MetaToolSlug` - 5 meta tools principais
- ✅ `TagType` - Filtros de comportamento (ReadOnly, Destructive, etc.)
- ✅ `AuthScheme` - **PARCIAL** (faltam 6 esquemas específicos)

### Modelos de Request
- ✅ `SessionConfig` - Configuração de sessão
- ✅ `ToolExecutionRequest` - Execução de ferramentas
- ✅ `MetaToolExecutionRequest` - Execução de meta tools
- ✅ `LinkRequest` - Criação de links de autenticação
- ✅ `ToolkitFilter` - Filtros de toolkit
- ✅ `ToolsConfig` - Configuração de ferramentas
- ✅ `TagsConfig` - Configuração de tags
- ✅ `WorkbenchConfig` - Configuração do workbench

### Modelos de Response
- ✅ `SessionResponse` - Resposta de criação de sessão
- ✅ `ToolExecutionResponse` - Resposta de execução
- ✅ `ToolkitListResponse` - Lista de toolkits
- ✅ `ToolkitInfo` - Informações de toolkit
- ✅ `ConnectedAccountInfo` - Informações de conta conectada
- ✅ `LinkResponse` - Resposta de link de autenticação
- ✅ `ErrorResponse` - Resposta de erro

## ❌ Tipos Faltantes

### 1. Esquemas de Autenticação Adicionais

**Localização no Python:** `temp/composio/client/types.py` linhas 35-46

**Faltam no Rust:**
```rust
// Adicionar ao enum AuthScheme em src/models/enums.rs
pub enum AuthScheme {
    // ... existentes ...
    
    // ❌ FALTAM:
    Snowflake,              // Para Snowflake Data Warehouse
    CalcomAuth,             // Para Cal.com
    BillcomAuth,            // Para Bill.com
    ComposioLink,           // Link de autenticação Composio
    BasicWithJwt,           // Basic Auth + JWT
    GoogleServiceAccount,   // Google Service Account
    NoAuth,                 // Sem autenticação
}
```

**Impacto:** Sem esses tipos, você não consegue autenticar com serviços específicos que usam esses métodos.

### 2. Parâmetros de Auth Config

**Localização no Python:** Importados de `composio_client.types`

**Faltam no Rust:**

#### 2.1 Auth Config Create
```rust
// Criar em src/models/request.rs

/// Parâmetros para criar uma configuração de autenticação
#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigCreateParams {
    pub toolkit: String,
    pub auth_config: AuthConfigData,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigData {
    #[serde(rename = "type")]
    pub auth_type: AuthScheme,
    pub credentials: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_to_following_tools: Option<Vec<String>>,
}
```

#### 2.2 Auth Config List
```rust
// Criar em src/models/request.rs

/// Parâmetros para listar configurações de autenticação
#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_composio_managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
```

#### 2.3 Auth Config Update
```rust
// Criar em src/models/request.rs

/// Parâmetros para atualizar uma configuração de autenticação
#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_access_config: Option<serde_json::Value>,
}
```

### 3. Respostas de Auth Config

**Faltam no Rust:**

```rust
// Criar em src/models/response.rs

/// Resposta ao criar uma configuração de autenticação
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfigCreateResponse {
    pub toolkit: ToolkitInfo,
    pub auth_config: AuthConfigInfo,
}

/// Resposta ao listar configurações de autenticação
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfigListResponse {
    pub items: Vec<AuthConfigInfo>,
    pub next_cursor: Option<String>,
    pub total_pages: u32,
    pub current_page: u32,
    pub total_items: u32,
}

/// Resposta ao recuperar uma configuração de autenticação
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfigRetrieveResponse {
    pub id: String,
    pub uuid: String,
    #[serde(rename = "type")]
    pub auth_type: String,
    pub toolkit: String,
    pub name: String,
    pub auth_scheme: AuthScheme,
    pub credentials: serde_json::Value,
    pub status: String,
    pub created_at: String,
    pub no_of_connections: u32,
}

/// Informações sobre uma configuração de autenticação
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfigInfo {
    pub id: String,
    pub auth_scheme: AuthScheme,
    pub is_composio_managed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_to_following_tools: Option<Vec<String>>,
}
```

### 4. Parâmetros de Connected Account

**Faltam no Rust:**

```rust
// Criar em src/models/request.rs

/// Parâmetros para criar uma conta conectada
#[derive(Debug, Clone, Serialize)]
pub struct ConnectedAccountCreateParams {
    pub auth_config: AuthConfigReference,
    pub connection: ConnectionData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_credentials: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigReference {
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

/// Parâmetros para listar contas conectadas
#[derive(Debug, Clone, Serialize)]
pub struct ConnectedAccountListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slugs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}
```

### 5. Respostas de Connected Account

**Faltam no Rust:**

```rust
// Criar em src/models/response.rs

/// Resposta ao criar uma conta conectada
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectedAccountCreateResponse {
    pub id: String,
    pub connection_data: serde_json::Value,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

/// Resposta ao listar contas conectadas
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectedAccountListResponse {
    pub items: Vec<ConnectedAccountDetail>,
    pub next_cursor: Option<String>,
    pub total_pages: u32,
    pub current_page: u32,
    pub total_items: u32,
}

/// Detalhes completos de uma conta conectada
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectedAccountDetail {
    pub toolkit: String,
    pub auth_config: String,
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    pub is_disabled: bool,
}

/// Resposta ao recuperar uma conta conectada
pub type ConnectedAccountRetrieveResponse = ConnectedAccountDetail;

/// Resposta ao atualizar status de uma conta conectada
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectedAccountUpdateStatusResponse {
    pub success: bool,
}
```

### 6. Parâmetros de Tool Proxy

**Faltam no Rust:**

```rust
// Criar em src/models/request.rs

/// Parâmetros para executar uma requisição proxy
#[derive(Debug, Clone, Serialize)]
pub struct ToolProxyParams {
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<serde_json::Value>,
}
```

### 7. Respostas de Tool Proxy

**Faltam no Rust:**

```rust
// Criar em src/models/response.rs

/// Resposta de uma requisição proxy
#[derive(Debug, Clone, Deserialize)]
pub struct ToolProxyResponse {
    pub data: serde_json::Value,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

### 8. Parâmetros de Trigger

**Faltam no Rust:**

```rust
// Criar em src/models/request.rs

/// Resposta ao criar/atualizar um trigger
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerInstanceUpsertResponse {
    pub id: String,
    pub trigger_name: String,
    pub connected_account_id: String,
    pub user_id: String,
    pub trigger_config: serde_json::Value,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
}
```

### 9. Tipo `NotGiven`

**Conceito do Python:**
```python
from composio_client import NotGiven

# Uso:
def create_session(
    user_id: str,
    toolkits: Union[List[str], NotGiven] = NOT_GIVEN
):
    # Se toolkits é NOT_GIVEN, não envia no JSON
    # Se toolkits é None, envia null
    # Se toolkits é uma lista, envia a lista
```

**Equivalente em Rust:**
```rust
// Criar em src/models/mod.rs

/// Representa um valor que pode ser:
/// - Fornecido (Some(T))
/// - Explicitamente nulo (None)
/// - Não fornecido (NotGiven)
#[derive(Debug, Clone)]
pub enum MaybeValue<T> {
    /// Valor fornecido
    Given(T),
    /// Explicitamente nulo
    Null,
    /// Não fornecido (não serializar)
    NotGiven,
}

impl<T: Serialize> Serialize for MaybeValue<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MaybeValue::Given(value) => value.serialize(serializer),
            MaybeValue::Null => serializer.serialize_none(),
            MaybeValue::NotGiven => serializer.serialize_none(), // Ou skip
        }
    }
}
```

## 📊 Estatísticas

### Tipos Implementados
- ✅ **Enums:** 3/3 (100%) - mas AuthScheme está incompleto
- ✅ **Request Models:** 8/15 (53%)
- ✅ **Response Models:** 8/20 (40%)

### Tipos Faltantes
- ❌ **Auth Config:** 0/6 tipos
- ❌ **Connected Account:** 0/5 tipos
- ❌ **Tool Proxy:** 0/2 tipos
- ❌ **Trigger:** 0/1 tipo
- ❌ **Utility Types:** 0/1 (NotGiven)

### Total
- ✅ **Implementado:** 19 tipos
- ❌ **Faltando:** 22 tipos
- 📊 **Progresso:** 46% completo

## 🎯 Prioridades de Implementação

### Prioridade ALTA (Funcionalidade Core)
1. ✅ Completar `AuthScheme` enum (adicionar 7 variantes)
2. ❌ Implementar Auth Config (create, list, update, retrieve)
3. ❌ Implementar Connected Account (create, list, retrieve, update status)

### Prioridade MÉDIA (Funcionalidade Avançada)
4. ❌ Implementar Tool Proxy (params e response)
5. ❌ Implementar Trigger Instance (upsert response)

### Prioridade BAIXA (Utilidades)
6. ❌ Implementar `MaybeValue<T>` (equivalente a NotGiven)

## 📝 Próximos Passos

1. **Completar AuthScheme** - Adicionar as 7 variantes faltantes
2. **Criar módulo auth_config** - Implementar todos os tipos relacionados
3. **Expandir connected_account** - Adicionar tipos de request/response faltantes
4. **Adicionar tool_proxy** - Para chamadas diretas à API
5. **Adicionar trigger_instance** - Para gerenciamento de triggers
6. **Implementar MaybeValue** - Para melhor controle de serialização

## 🔗 Referências

- Python SDK: `temp/composio/client/types.py`
- Rust Enums: `src/models/enums.rs`
- Rust Requests: `src/models/request.rs`
- Rust Responses: `src/models/response.rs`
- Documentação Composio: `COMPOSIO DOCS/`
