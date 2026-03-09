# Implementação de Tools no Rust SDK

## Resumo

Este documento descreve a implementação completa da funcionalidade de Tools no Composio Rust SDK, traduzida do SDK Python oficial.

## Arquitetura

### Padrão Seguido

Seguimos o padrão arquitetural do SDK Python oficial:

**Python SDK:**
```python
composio = Composio(api_key="...")
tools = composio.tools

# List tools
tools_list = tools.get_raw_composio_tools(toolkits=["github"])

# Get specific tool
tool = tools.get_raw_composio_tool_by_slug("GITHUB_CREATE_ISSUE")

# Execute tool
result = tools.execute(
    slug="GITHUB_CREATE_ISSUE",
    arguments={"title": "Bug fix", "body": "..."},
    user_id="user_123"
)

# Proxy execution
result = tools.proxy(
    endpoint="/repos/owner/repo",
    method="GET",
    connected_account_id="ca_123"
)
```

**Rust SDK:**
```rust
let client = ComposioClient::builder().api_key("...").build()?;

// List tools
let params = ToolListParams {
    toolkit_slug: Some("github".to_string()),
    ..Default::default()
};
let tools = client.list_tools(params).await?;

// Get specific tool
let tool = client.get_tool("GITHUB_CREATE_ISSUE").await?;

// Execute tool
let params = ToolExecuteParams {
    slug: "GITHUB_CREATE_ISSUE".to_string(),
    arguments: /* ... */,
    user_id: Some("user_123".to_string()),
    ..Default::default()
};
let result = client.execute_tool(params).await?;

// Proxy execution
let params = ToolProxyParams {
    endpoint: "/repos/owner/repo".to_string(),
    method: HttpMethod::Get,
    connected_account_id: Some("ca_123".to_string()),
    ..Default::default()
};
let result = client.proxy_tool(params).await?;
```

### Localização do Código

1. **Estruturas de Dados**: `src/models/tools.rs`
   - `ToolExecutionResponse`
   - `ToolExecuteParams`
   - `CustomAuthParams`
   - `CustomConnectionData`
   - `ToolListParams`
   - `ToolInfo`
   - `ToolkitRef`
   - `ToolListResponse`
   - `ToolProxyParams`
   - `HttpMethod`
   - `ProxyParameter`
   - `ParameterLocation`
   - `ToolProxyResponse`
   - `ToolInputGenerationParams`
   - `ToolInputGenerationResponse`
   - `CustomToolDefinition`
   - `CustomToolExecutionRequest`

2. **Métodos Públicos**: `src/client.rs` (ComposioClient)
   - `list_tools()`
   - `get_tool()`
   - `execute_tool()`
   - `proxy_tool()`
   - `generate_tool_inputs()`

## Funcionalidades Implementadas

### 1. Listar Tools (`list_tools`)

Lista todos os tools disponíveis com suporte a filtros e paginação.

**Parâmetros:**
- `tool_slugs`: Filtrar por slugs específicos
- `toolkit_slug`: Filtrar por toolkit
- `search`: Busca por nome/descrição
- `scopes`: Filtrar por escopos OAuth
- `tags`: Filtrar por tags (e.g., "write", "read")
- `importance`: Filtrar por importância
- `show_deprecated`: Incluir tools deprecados
- `limit`: Número máximo de resultados
- `cursor`: Cursor de paginação
- `toolkit_versions`: Versões de toolkit

**Exemplo:**
```rust
let params = ToolListParams {
    toolkit_slug: Some("github".to_string()),
    tags: Some(vec!["write".to_string()]),
    limit: Some(20),
    ..Default::default()
};

let tools = client.list_tools(params).await?;
for tool in tools.items {
    println!("{} ({})", tool.name, tool.slug);
    println!("  Scopes: {:?}", tool.scopes);
    println!("  Tags: {:?}", tool.tags);
}
```

### 2. Obter Tool Específico (`get_tool`)

Recupera informações detalhadas sobre um tool específico.

**Parâmetros:**
- `slug`: Identificador do tool (e.g., "GITHUB_CREATE_ISSUE")

**Retorna:**
- Informações completas incluindo:
  - Nome e descrição
  - Toolkit associado
  - Esquemas de input/output (JSON Schema)
  - Escopos OAuth necessários
  - Tags de categorização
  - Versão e versões disponíveis
  - Status de deprecação
  - Se requer autenticação

**Exemplo:**
```rust
let tool = client.get_tool("GITHUB_CREATE_ISSUE").await?;
println!("Name: {}", tool.name);
println!("Description: {}", tool.description);
println!("Version: {}", tool.version);
println!("Input schema: {}", tool.input_parameters);
println!("Output schema: {}", tool.output_parameters);
```

### 3. Executar Tool (`execute_tool`)

Executa um tool com os argumentos fornecidos.

**Funcionalidade:**
1. Valida os argumentos contra o schema do tool
2. Resolve a versão do toolkit (se não especificada)
3. Verifica se a versão é 'latest' (requer skip check)
4. Executa o tool com autenticação do usuário
5. Retorna o resultado ou erro

**Parâmetros:**
- `slug`: Slug do tool
- `arguments`: Argumentos estruturados (HashMap)
- `connected_account_id`: ID da conta conectada (opcional)
- `custom_auth_params`: Parâmetros de auth customizados (opcional)
- `custom_connection_data`: Dados de conexão customizados (opcional)
- `user_id`: ID do usuário (opcional)
- `text`: Texto em linguagem natural (opcional)
- `version`: Versão do tool (opcional, resolve automaticamente)
- `dangerously_skip_version_check`: Pular verificação de versão (opcional)

**Retorna:**
- `ToolExecutionResponse` com:
  - `data`: Dados de saída do tool
  - `error`: Mensagem de erro (se falhou)
  - `successful`: Se a execução foi bem-sucedida
  - `log_id`: ID do log para debugging

**Exemplo:**
```rust
let mut arguments = HashMap::new();
arguments.insert("owner".to_string(), serde_json::json!("composio"));
arguments.insert("repo".to_string(), serde_json::json!("composio"));
arguments.insert("title".to_string(), serde_json::json!("Bug fix"));
arguments.insert("body".to_string(), serde_json::json!("Fix login issue"));

let params = ToolExecuteParams {
    slug: "GITHUB_CREATE_ISSUE".to_string(),
    arguments,
    user_id: Some("user_123".to_string()),
    version: Some("1.0.0".to_string()),
    ..Default::default()
};

let result = client.execute_tool(params).await?;
if result.successful {
    println!("Issue created: {:?}", result.data);
} else {
    eprintln!("Error: {}", result.error.unwrap());
}
```

### 4. Proxy Execution (`proxy_tool`)

Faz uma chamada HTTP autenticada para uma API de terceiros.

**Funcionalidade:**
- Permite chamar endpoints que não têm tools predefinidos
- Injeta automaticamente a autenticação da conta conectada
- Suporta todos os métodos HTTP
- Permite headers e query params customizados

**Parâmetros:**
- `endpoint`: Endpoint da API (relativo ou absoluto)
- `method`: Método HTTP (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- `body`: Corpo da requisição (opcional)
- `connected_account_id`: ID da conta conectada (opcional)
- `parameters`: Headers/query params adicionais (opcional)
- `custom_connection_data`: Dados de conexão customizados (opcional)

**Retorna:**
- `ToolProxyResponse` com:
  - `data`: Dados da resposta
  - `status`: Código de status HTTP
  - `headers`: Headers da resposta
  - `binary_data`: Dados binários (base64, opcional)
  - `successful`: Se a requisição foi bem-sucedida
  - `error`: Mensagem de erro (se falhou)

**Exemplo:**
```rust
let params = ToolProxyParams {
    endpoint: "/repos/composio/composio".to_string(),
    method: HttpMethod::Get,
    connected_account_id: Some("ca_123".to_string()),
    body: None,
    parameters: Some(vec![
        ProxyParameter {
            name: "per_page".to_string(),
            value: "10".to_string(),
            location: ParameterLocation::Query,
        }
    ]),
    custom_connection_data: None,
};

let result = client.proxy_tool(params).await?;
println!("Status: {}", result.status);
println!("Data: {:?}", result.data);
```

### 5. Gerar Inputs de Tool (`generate_tool_inputs`)

Converte linguagem natural em argumentos estruturados usando IA.

**Funcionalidade:**
- Usa IA para interpretar descrições em linguagem natural
- Gera argumentos estruturados que correspondem ao schema do tool
- Útil para interfaces de usuário conversacionais

**Parâmetros:**
- `tool_slug`: Slug do tool
- `text`: Descrição em linguagem natural
- `custom_tool_description`: Descrição customizada do tool (opcional)
- `custom_system_prompt`: Prompt de sistema customizado (opcional)

**Retorna:**
- `ToolInputGenerationResponse` com:
  - `arguments`: Argumentos gerados (HashMap)
  - `error`: Mensagem de erro (se falhou)
  - `successful`: Se a geração foi bem-sucedida

**Exemplo:**
```rust
let params = ToolInputGenerationParams {
    tool_slug: "GITHUB_CREATE_ISSUE".to_string(),
    text: "Create an issue about fixing the login bug in the composio repo".to_string(),
    custom_tool_description: None,
    custom_system_prompt: None,
};

let result = client.generate_tool_inputs(params).await?;
if result.successful {
    if let Some(arguments) = result.arguments {
        println!("Generated arguments:");
        for (key, value) in arguments {
            println!("  {}: {}", key, value);
        }
        
        // Use os argumentos gerados para executar o tool
        let execute_params = ToolExecuteParams {
            slug: "GITHUB_CREATE_ISSUE".to_string(),
            arguments,
            user_id: Some("user_123".to_string()),
            ..Default::default()
        };
        let execution_result = client.execute_tool(execute_params).await?;
    }
}
```

## Comparação com Python SDK

### Python (Original)

```python
from composio import Composio

composio = Composio(api_key="...")
tools = composio.tools

# List tools
tools_list = tools.get_raw_composio_tools(
    toolkits=["github"],
    search="create issue",
    limit=20
)

# Get tool
tool = tools.get_raw_composio_tool_by_slug("GITHUB_CREATE_ISSUE")

# Execute tool
result = tools.execute(
    slug="GITHUB_CREATE_ISSUE",
    arguments={
        "owner": "composio",
        "repo": "composio",
        "title": "Bug fix"
    },
    user_id="user_123",
    version="1.0.0"
)

# Proxy execution
result = tools.proxy(
    endpoint="/repos/composio/composio",
    method="GET",
    connected_account_id="ca_123"
)
```

### Rust (Implementado)

```rust
use composio_sdk::client::ComposioClient;
use composio_sdk::models::tools::{ToolListParams, ToolExecuteParams, ToolProxyParams, HttpMethod};
use std::collections::HashMap;

let client = ComposioClient::builder()
    .api_key("...")
    .build()?;

// List tools
let params = ToolListParams {
    toolkit_slug: Some("github".to_string()),
    search: Some("create issue".to_string()),
    limit: Some(20),
    ..Default::default()
};
let tools = client.list_tools(params).await?;

// Get tool
let tool = client.get_tool("GITHUB_CREATE_ISSUE").await?;

// Execute tool
let mut arguments = HashMap::new();
arguments.insert("owner".to_string(), serde_json::json!("composio"));
arguments.insert("repo".to_string(), serde_json::json!("composio"));
arguments.insert("title".to_string(), serde_json::json!("Bug fix"));

let params = ToolExecuteParams {
    slug: "GITHUB_CREATE_ISSUE".to_string(),
    arguments,
    user_id: Some("user_123".to_string()),
    version: Some("1.0.0".to_string()),
    ..Default::default()
};
let result = client.execute_tool(params).await?;

// Proxy execution
let params = ToolProxyParams {
    endpoint: "/repos/composio/composio".to_string(),
    method: HttpMethod::Get,
    connected_account_id: Some("ca_123".to_string()),
    body: None,
    parameters: None,
    custom_connection_data: None,
};
let result = client.proxy_tool(params).await?;
```

## Detalhes de Implementação

### Resolução Automática de Versão

O SDK resolve automaticamente a versão do toolkit quando não especificada:

1. Extrai o toolkit do slug do tool (e.g., "GITHUB_CREATE_ISSUE" → "github")
2. Consulta a configuração de versões do cliente
3. Usa a versão configurada ou "latest" como fallback
4. Valida se "latest" requer `dangerously_skip_version_check=true`

```rust
// Versão explícita
let params = ToolExecuteParams {
    slug: "GITHUB_CREATE_ISSUE".to_string(),
    arguments,
    version: Some("20250906_01".to_string()),
    ..Default::default()
};

// Versão automática (usa configuração do cliente)
let params = ToolExecuteParams {
    slug: "GITHUB_CREATE_ISSUE".to_string(),
    arguments,
    version: None, // Resolve automaticamente
    ..Default::default()
};
```

### Tratamento de Erros

Todos os métodos retornam `Result<T, ComposioError>` e incluem:
- Retry automático para erros transitórios
- Validação de parâmetros
- Mensagens de erro descritivas
- Verificação de versão "latest"

### Serialização/Deserialização

Todas as estruturas implementam:
- `Serialize` e `Deserialize` (serde)
- `Debug` e `Clone`
- Campos opcionais com `#[serde(skip_serializing_if = "Option::is_none")]`
- `Default` para structs de parâmetros

### Testes

Testes unitários incluídos em `src/models/tools.rs`:
- Serialização/deserialização de structs
- Enums HTTP method e parameter location
- Valores padrão
- Estruturas de dados

## Exemplo Completo

Veja `examples/tools_usage.rs` para um exemplo completo demonstrando:
1. Listagem de tools com filtros
2. Busca de tools
3. Obtenção de detalhes de tool
4. Filtragem por tags
5. Filtragem por escopos
6. Execução de tool
7. Proxy execution
8. Geração de inputs com IA
9. Listagem de tools específicos por slug
10. Paginação

**Executar exemplo:**
```bash
export COMPOSIO_API_KEY="your_api_key"
cargo run --example tools_usage
```

## Funcionalidades Não Implementadas

As seguintes funcionalidades do Python SDK não foram implementadas (são específicas do Python):

1. **Custom Tools**: Registro de tools customizados em memória
   - Python: `@composio.tools.custom_tool` decorator
   - Rust: Requer arquitetura diferente (trait-based)

2. **Modifiers**: Decoradores para modificar comportamento
   - Python: `@before_execute`, `@after_execute`, `@schema_modifier`
   - Rust: Pode ser implementado com traits e closures

3. **Provider Integration**: Integração com frameworks específicos
   - Python: `OpenAIProvider`, `AnthropicProvider`, etc.
   - Rust: Requer implementação de traits genéricos

4. **File Helper**: Upload/download automático de arquivos
   - Python: `FileHelper` class
   - Rust: Pode ser implementado separadamente

5. **Tool Router Meta Tools**: Execução via sessão
   - Python: `get_raw_tool_router_meta_tools()`
   - Rust: Já implementado em `Session::get_meta_tools()`

## Próximos Passos

Funcionalidades adicionais que podem ser implementadas:

1. **Custom Tools System**: Sistema de registro de tools customizados
2. **Modifiers System**: Sistema de modificadores (before/after/schema)
3. **Provider Traits**: Traits genéricos para integração com frameworks
4. **File Helper**: Upload/download automático de arquivos
5. **Tool Validation**: Validação de argumentos contra schemas
6. **Tool Caching**: Cache de schemas de tools
7. **Batch Execution**: Execução de múltiplos tools em paralelo

## Referências

- **Arquivo Python Original**: `temp/composio/core/models/tools.py`
- **Documentação Composio**: [Composio Docs](https://docs.composio.dev)
- **API Reference**: [Tools API](https://docs.composio.dev/api-reference/tools)
- **Guia de Tools**: [Direct Tool Execution](https://docs.composio.dev/guides/direct-execution)
