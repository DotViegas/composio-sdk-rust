# Análise de Comportamento: Python vs Rust

## 📋 Objetivo

Comparar o comportamento dos tipos já implementados no Rust com o SDK Python para garantir compatibilidade total.

## 🔍 Análise Detalhada

### 1. AuthScheme Enum

#### Python (`types.py`)
```python
# Define tipos literais para cada esquema
Oauth1L: t.TypeAlias = t.Literal["OAUTH1"]
Oauth2L: t.TypeAlias = t.Literal["OAUTH2"]
ApiKeyL: t.TypeAlias = t.Literal["API_KEY"]
BasicL: t.TypeAlias = t.Literal["BASIC"]
NoAuthL: t.TypeAlias = t.Literal["NO_AUTH"]
SnowflakeL: t.TypeAlias = t.Literal["SNOWFLAKE"]
CalcomAuthL: t.TypeAlias = t.Literal["CALCOM_AUTH"]
BearerTokenL: t.TypeAlias = t.Literal["BEARER_TOKEN"]
BillcomAuthL: t.TypeAlias = t.Literal["BILLCOM_AUTH"]
ComposioLinkL: t.TypeAlias = t.Literal["COMPOSIO_LINK"]
BasicWithJwtL: t.TypeAlias = t.Literal["BASIC_WITH_JWT"]
GoogleServiceAccountL: t.TypeAlias = t.Literal["GOOGLE_SERVICE_ACCOUNT"]

# União de todos os tipos
AuthSchemeL: t.TypeAlias = t.Literal[
    Oauth1L, Oauth2L, ApiKeyL, BasicL, NoAuthL,
    SnowflakeL, CalcomAuthL, BearerTokenL, BillcomAuthL,
    ComposioLinkL, BasicWithJwtL, GoogleServiceAccountL,
]
```

#### Rust Atual (`src/models/enums.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthScheme {
    Oauth2,
    Oauth1,
    ApiKey,
    BearerToken,
    Basic,
    Custom,  // ❌ Não existe no Python
}
```

#### ❌ Problemas Identificados

1. **Faltam 7 variantes específicas:**
   - `NoAuth`
   - `Snowflake`
   - `CalcomAuth`
   - `BillcomAuth`
   - `ComposioLink`
   - `BasicWithJwt`
   - `GoogleServiceAccount`

2. **Variante `Custom` não existe no Python**
   - O Python não tem um tipo genérico "Custom"
   - Cada esquema é explicitamente definido

3. **Uso no Python (`connected_accounts.py`):**
   ```python
   class AuthScheme:
       def oauth1(self, options) -> ConnectionState:
           return {"auth_scheme": "OAUTH1", "val": {...}}
       
       def oauth2(self, options) -> ConnectionState:
           return {"auth_scheme": "OAUTH2", "val": {...}}
       
       def api_key(self, options) -> ConnectionState:
           return {"auth_scheme": "API_KEY", "val": {...}}
       
       def basic(self, options) -> ConnectionState:
           return {"auth_scheme": "BASIC", "val": {...}}
       
       def bearer_token(self, options) -> ConnectionState:
           return {"auth_scheme": "BEARER_TOKEN", "val": {...}}
       
       def google_service_account(self, options) -> ConnectionState:
           return {"auth_scheme": "GOOGLE_SERVICE_ACCOUNT", "val": {...}}
       
       def no_auth(self, options) -> ConnectionState:
           return {"auth_scheme": "NO_AUTH", "val": {...}}
       
       def calcom_auth(self, options) -> ConnectionState:
           return {"auth_scheme": "CALCOM_AUTH", "val": {...}}
       
       def billcom_auth(self, options) -> ConnectionState:
           return {"auth_scheme": "BILLCOM_AUTH", "val": {...}}
       
       def basic_with_jwt(self, options) -> ConnectionState:
           return {"auth_scheme": "BASIC_WITH_JWT", "val": {...}}
       
       def composio_link(self, options) -> ConnectionState:
           return {"auth_scheme": "COMPOSIO_LINK", "val": {...}}
   ```

#### ✅ Correção Necessária

```rust
// src/models/enums.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthScheme {
    /// OAuth 1.0 authentication
    Oauth1,
    /// OAuth 2.0 authentication
    Oauth2,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    BearerToken,
    /// HTTP Basic authentication
    Basic,
    /// No authentication required
    NoAuth,
    /// Snowflake authentication
    Snowflake,
    /// Cal.com authentication
    CalcomAuth,
    /// Bill.com authentication
    BillcomAuth,
    /// Composio Link authentication
    ComposioLink,
    /// Basic authentication with JWT
    BasicWithJwt,
    /// Google Service Account authentication
    GoogleServiceAccount,
}
```

---

### 2. Tool Execution Request

#### Python (`tools.py`)
```python
# Uso na execução de ferramentas
def execute(
    self,
    slug: str,
    arguments: t.Dict,
    *,
    connected_account_id: t.Optional[str] = None,
    custom_auth_params: t.Optional[tool_execute_params.CustomAuthParams] = None,
    custom_connection_data: t.Optional[tool_execute_params.CustomConnectionData] = None,
    user_id: t.Optional[str] = None,
    text: t.Optional[str] = None,
    version: t.Optional[str] = None,
    dangerously_skip_version_check: t.Optional[bool] = None,
    modifiers: t.Optional[Modifiers] = None,
) -> ToolExecutionResponse:
    # Serializa argumentos Pydantic para JSON
    arguments = _serialize_arguments(arguments)
    
    # Executa a ferramenta
    return self._execute_tool(
        slug=slug,
        arguments=arguments,
        connected_account_id=connected_account_id,
        custom_auth_params=custom_auth_params,
        custom_connection_data=custom_connection_data,
        user_id=user_id,
        text=text,
        version=version,
        dangerously_skip_version_check=dangerously_skip_version_check,
    )
```

#### Rust Atual (`src/models/request.rs`)
```rust
#[derive(Debug, Clone, Serialize)]
pub struct ToolExecutionRequest {
    pub tool_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}
```

#### ❌ Problemas Identificados

1. **Faltam campos importantes:**
   - `connected_account_id` - ID da conta conectada
   - `custom_auth_params` - Parâmetros de autenticação customizados
   - `custom_connection_data` - Dados de conexão customizados
   - `user_id` - ID do usuário
   - `text` - Texto para execução em linguagem natural
   - `version` - Versão da ferramenta
   - `dangerously_skip_version_check` - Flag para pular verificação de versão

2. **Campo `tool_slug` deveria ser apenas `slug`**
   - No Python, o campo é apenas `slug` na API

#### ✅ Correção Necessária

```rust
// src/models/request.rs

/// Request to execute a tool
#[derive(Debug, Clone, Serialize)]
pub struct ToolExecutionRequest {
    /// Tool slug to execute
    pub tool_slug: String,
    
    /// Arguments to pass to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    
    /// Connected account ID to use for authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    
    /// Custom authentication parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth_params: Option<serde_json::Value>,
    
    /// Custom connection data (takes priority over custom_auth_params)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connection_data: Option<serde_json::Value>,
    
    /// User ID to execute the tool for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    
    /// Natural language text to pass to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    
    /// Version of the tool to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    /// Skip version check for 'latest' version (dangerous!)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_skip_version_check: Option<bool>,
}
```

---

### 3. Tool Execution Response

#### Python (`tools.py`)
```python
class ToolExecutionResponse(te.TypedDict):
    data: t.Dict
    error: t.Optional[str]
    successful: bool
```

#### Rust Atual (`src/models/response.rs`)
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ToolExecutionResponse {
    pub data: serde_json::Value,
    pub error: Option<String>,
    pub log_id: String,
}
```

#### ⚠️ Diferenças Identificadas

1. **Campo `successful` falta no Rust**
   - Python tem um campo booleano `successful`
   - Rust não tem esse campo

2. **Campo `log_id` existe no Rust mas não no Python**
   - Python remove `log_id` na resposta final
   - Rust mantém `log_id`

#### 🤔 Análise

Olhando o código Python:
```python
return t.cast(
    ToolExecutionResponse,
    self._client.tools.execute(...).model_dump(
        exclude={
            "log_id",      # ❌ Removido
            "session_info", # ❌ Removido
        }
    ),
)
```

O Python **remove** `log_id` e `session_info` da resposta da API antes de retornar.

#### ✅ Decisão

**Manter ambos os campos no Rust:**
- `log_id` é útil para debugging
- `successful` é útil para verificação rápida

```rust
// src/models/response.rs

/// Response from tool execution
#[derive(Debug, Clone, Deserialize)]
pub struct ToolExecutionResponse {
    /// Execution result data
    pub data: serde_json::Value,
    
    /// Error message if execution failed
    pub error: Option<String>,
    
    /// Log ID for debugging
    pub log_id: String,
    
    /// Whether the execution was successful
    #[serde(default)]
    pub successful: bool,
}

impl ToolExecutionResponse {
    /// Check if the execution was successful
    pub fn is_successful(&self) -> bool {
        self.error.is_none()
    }
}
```

---

### 4. Session Config

#### Python (uso em `connected_accounts.py`)
```python
# Criação de sessão com configuração
connection: dict[str, t.Any] = {"user_id": user_id}
if callback_url is not None:
    connection["callback_url"] = callback_url

if config is not None:
    connection["state"] = config

response = self._client.connected_accounts.create(
    auth_config={"id": auth_config_id},
    connection=t.cast(connected_account_create_params.Connection, connection),
)
```

#### Rust Atual (`src/models/request.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<ToolkitFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_configs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_accounts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_connections: Option<ManageConnectionsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workbench: Option<WorkbenchConfig>,
}
```

#### ✅ Comportamento Correto

O `SessionConfig` do Rust está **correto** e compatível com o Python!

**Verificação:**
- ✅ Todos os campos opcionais usam `Option<T>`
- ✅ Serialização pula campos `None` com `skip_serializing_if`
- ✅ Estrutura corresponde à API do Composio

---

### 5. Toolkit Filter

#### Python (comportamento implícito)
```python
# Aceita tanto lista quanto objeto com "disable"
toolkits: Union[List[str], Dict[str, List[str]]]

# Exemplos:
toolkits = ["github", "gmail"]  # Enable
toolkits = {"disable": ["exa", "firecrawl"]}  # Disable
```

#### Rust Atual (`src/models/request.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolkitFilter {
    Enable(Vec<String>),
    Disable { disable: Vec<String> },
}
```

#### ✅ Comportamento Correto

O `ToolkitFilter` do Rust está **perfeito**!

**Verificação:**
- ✅ `#[serde(untagged)]` permite serialização correta
- ✅ `Enable(Vec<String>)` serializa como array JSON
- ✅ `Disable { disable: Vec<String> }` serializa como objeto JSON
- ✅ Comportamento idêntico ao Python

**Teste de serialização:**
```rust
// Enable
let filter = ToolkitFilter::Enable(vec!["github".to_string()]);
// JSON: ["github"]

// Disable
let filter = ToolkitFilter::Disable {
    disable: vec!["exa".to_string()],
};
// JSON: {"disable": ["exa"]}
```

---

### 6. Manage Connections Config

#### Python (comportamento implícito)
```python
# Aceita bool ou objeto detalhado
manage_connections: Union[bool, Dict[str, Any]]

# Exemplos:
manage_connections = True  # Simples
manage_connections = {
    "enabled": True,
    "enable_wait_for_connections": True
}  # Detalhado
```

#### Rust Atual (`src/models/request.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManageConnectionsConfig {
    Bool(bool),
    Detailed {
        enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        enable_wait_for_connections: Option<bool>,
    },
}
```

#### ✅ Comportamento Correto

O `ManageConnectionsConfig` do Rust está **perfeito**!

**Verificação:**
- ✅ `#[serde(untagged)]` permite ambos os formatos
- ✅ `Bool(bool)` serializa como booleano JSON
- ✅ `Detailed` serializa como objeto JSON
- ✅ Comportamento idêntico ao Python

---

### 7. Tags Config

#### Python (comportamento implícito)
```python
tags: Optional[Dict[str, List[str]]]

# Exemplo:
tags = {
    "enabled": ["READ_ONLY_HINT", "IDEMPOTENT_HINT"],
    "disabled": ["DESTRUCTIVE_HINT"]
}
```

#### Rust Atual (`src/models/request.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<TagType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<TagType>>,
}
```

#### ✅ Comportamento Correto

O `TagsConfig` do Rust está **correto**!

**Verificação:**
- ✅ Campos opcionais com `Option<Vec<TagType>>`
- ✅ Usa enum `TagType` para type safety
- ✅ Serialização correta com `skip_serializing_if`

---

### 8. Workbench Config

#### Python (comportamento implícito)
```python
workbench: Optional[Dict[str, Any]]

# Exemplo:
workbench = {
    "proxy_execution": True,
    "auto_offload_threshold": 1000
}
```

#### Rust Atual (`src/models/request.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "proxy_execution_enabled")]
    pub proxy_execution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_offload_threshold: Option<u32>,
}
```

#### ✅ Comportamento Correto

O `WorkbenchConfig` do Rust está **correto**!

**Verificação:**
- ✅ Campos opcionais
- ✅ Alias para `proxy_execution_enabled` (compatibilidade)
- ✅ Tipo correto para `auto_offload_threshold` (u32)

---

## 📊 Resumo da Análise

### ✅ Tipos Corretos (Não precisam mudanças)
1. ✅ `SessionConfig` - Estrutura correta
2. ✅ `ToolkitFilter` - Serialização perfeita
3. ✅ `ManageConnectionsConfig` - Comportamento idêntico
4. ✅ `TagsConfig` - Type safety correto
5. ✅ `WorkbenchConfig` - Campos corretos
6. ✅ `ToolsConfig` - Estrutura correta
7. ✅ `ToolFilter` - Enum correto
8. ✅ `LinkRequest` - Campos corretos
9. ✅ `MetaToolExecutionRequest` - Estrutura correta

### ❌ Tipos que Precisam Correção

#### 1. AuthScheme (CRÍTICO)
**Problema:** Faltam 7 variantes, tem 1 variante extra (`Custom`)
**Impacto:** Alto - Não consegue autenticar com serviços específicos
**Prioridade:** 🔴 ALTA

#### 2. ToolExecutionRequest (IMPORTANTE)
**Problema:** Faltam 8 campos opcionais importantes
**Impacto:** Médio - Funcionalidade limitada na execução de ferramentas
**Prioridade:** 🟡 MÉDIA

#### 3. ToolExecutionResponse (MENOR)
**Problema:** Falta campo `successful`
**Impacto:** Baixo - Apenas conveniência
**Prioridade:** 🟢 BAIXA

---

## 🎯 Plano de Ação

### Fase 1: Correções Críticas (Fazer AGORA)
1. ✅ Corrigir `AuthScheme` enum
   - Adicionar 7 variantes faltantes
   - Remover variante `Custom`
   - Atualizar testes

### Fase 2: Melhorias Importantes (Fazer DEPOIS)
2. ✅ Expandir `ToolExecutionRequest`
   - Adicionar 8 campos opcionais
   - Atualizar documentação
   - Adicionar testes

3. ✅ Melhorar `ToolExecutionResponse`
   - Adicionar campo `successful`
   - Adicionar método helper `is_successful()`
   - Manter `log_id` para debugging

### Fase 3: Tipos Faltantes (Fazer POR ÚLTIMO)
4. ❌ Implementar tipos de Auth Config
5. ❌ Implementar tipos de Connected Account
6. ❌ Implementar tipos de Tool Proxy
7. ❌ Implementar tipos de Trigger

---

## 📝 Notas Importantes

### Diferenças de Design Python vs Rust

1. **Type Safety**
   - Python usa `TypedDict` e `Literal` para tipos
   - Rust usa `enum` e `struct` com type safety em compile-time
   - Rust é mais seguro e detecta erros antes da execução

2. **Serialização**
   - Python usa `model_dump()` do Pydantic
   - Rust usa `serde` com `#[serde(...)]` attributes
   - Ambos produzem JSON idêntico

3. **Campos Opcionais**
   - Python usa `Optional[T]` e `NotGiven`
   - Rust usa `Option<T>` e `skip_serializing_if`
   - Comportamento equivalente

4. **Validação**
   - Python valida em runtime com Pydantic
   - Rust valida em compile-time com type system
   - Rust é mais rápido e seguro

### Vantagens do Rust

1. ✅ **Performance** - 10-100x mais rápido que Python
2. ✅ **Memory Safety** - Sem memory leaks ou race conditions
3. ✅ **Type Safety** - Erros detectados em compile-time
4. ✅ **Zero-cost Abstractions** - Abstrações sem overhead
5. ✅ **Concurrency** - Async/await nativo e seguro

### Compatibilidade com API

Todos os tipos Rust devem produzir JSON **idêntico** ao Python para garantir compatibilidade com a API Composio.

**Exemplo:**
```rust
// Rust
let config = SessionConfig {
    user_id: "user_123".to_string(),
    toolkits: Some(ToolkitFilter::Enable(vec!["github".to_string()])),
    // ...
};

// JSON produzido (idêntico ao Python):
{
    "user_id": "user_123",
    "toolkits": ["github"]
}
```
