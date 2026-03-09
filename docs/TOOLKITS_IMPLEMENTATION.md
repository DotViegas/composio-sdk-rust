# Implementação de Toolkits no Rust SDK

## Resumo

Este documento descreve a implementação completa da funcionalidade de Toolkits no Composio Rust SDK, traduzida do SDK Python oficial.

## Arquitetura

### Padrão Seguido

Seguimos o padrão arquitetural do SDK Python oficial:

**Python SDK:**
```python
composio = Composio(api_key="...")
composio.toolkits.list()
composio.toolkits.get("github")
composio.toolkits.authorize(user_id="...", toolkit="...")
```

**Rust SDK:**
```rust
let client = ComposioClient::builder().api_key("...").build()?;
client.list_toolkits(params).await?;
client.get_toolkit("github").await?;
client.authorize_toolkit("user_123", "github").await?;
```

### Localização do Código

1. **Estruturas de Dados**: `src/models/toolkits.rs`
   - `ToolkitListParams`
   - `ToolkitListResponse`
   - `ToolkitItem`
   - `ToolkitRetrieveResponse`
   - `ToolkitMeta`
   - `AuthConfigDetail`
   - `AuthConfigFields`
   - `AuthFieldSet`
   - `AuthField`
   - `ToolkitCategoriesResponse`
   - `ToolkitCategory`
   - Enums: `SortBy`, `ManagedBy`

2. **Métodos Públicos**: `src/client.rs` (ComposioClient)
   - `list_toolkits()`
   - `get_toolkit()`
   - `list_toolkit_categories()`
   - `authorize_toolkit()`
   - `get_connected_account_initiation_fields()`
   - `get_auth_config_creation_fields()`

3. **Métodos Auxiliares Privados**: `src/client.rs`
   - `get_or_create_auth_config()`
   - `initiate_connection()`
   - `list_auth_configs()`
   - `create_auth_config()`

## Funcionalidades Implementadas

### 1. Listar Toolkits (`list_toolkits`)

Lista todos os toolkits disponíveis com suporte a filtros e paginação.

**Parâmetros:**
- `category`: Filtrar por categoria (e.g., "communication", "crm")
- `cursor`: Cursor de paginação
- `limit`: Número máximo de resultados
- `sort_by`: Ordenar por "usage" ou "alphabetically"
- `managed_by`: Filtrar por tipo de gerenciamento ("composio", "all", "project")
- `search`: Busca por nome/slug/descrição
- `show_deprecated`: Incluir toolkits deprecados

**Exemplo:**
```rust
let params = ToolkitListParams {
    category: Some("communication".to_string()),
    limit: Some(20),
    sort_by: Some(SortBy::Alphabetically),
    ..Default::default()
};

let toolkits = client.list_toolkits(params).await?;
for toolkit in toolkits.items {
    println!("{} ({})", toolkit.name, toolkit.slug);
}
```

### 2. Obter Toolkit Específico (`get_toolkit`)

Recupera informações detalhadas sobre um toolkit específico.

**Parâmetros:**
- `slug`: Identificador do toolkit (e.g., "github", "gmail", "slack")

**Retorna:**
- Informações completas incluindo:
  - Esquemas de autenticação suportados
  - Detalhes de configuração de auth
  - Metadados (logo, descrição, categorias)
  - Contagem de tools e triggers
  - Base URL e endpoint de usuário atual

**Exemplo:**
```rust
let github = client.get_toolkit("github").await?;
println!("Name: {}", github.name);
println!("Auth schemes: {:?}", github.auth_schemes);
println!("Tools count: {}", github.meta.unwrap().tools_count.unwrap());
```

### 3. Listar Categorias (`list_toolkit_categories`)

Lista todas as categorias de toolkits disponíveis.

**Retorna:**
- Lista de categorias com:
  - Nome da categoria
  - Nome de exibição
  - Contagem de toolkits na categoria

**Exemplo:**
```rust
let categories = client.list_toolkit_categories().await?;
for category in categories.items {
    println!("{}: {} toolkits", category.name, category.count.unwrap_or(0));
}
```

### 4. Autorizar Usuário (`authorize_toolkit`)

Cria um link de autenticação para um usuário se conectar a um toolkit.

**Funcionalidade:**
1. Busca ou cria um auth config para o toolkit
2. Inicia uma conexão para o usuário
3. Retorna um `ConnectionRequest` com URL de redirecionamento

**Parâmetros:**
- `user_id`: ID do usuário
- `toolkit`: Slug do toolkit

**Retorna:**
- `ConnectionRequest` com:
  - `id`: ID da conexão
  - `status`: Status da conexão
  - `redirect_url`: URL para o usuário visitar

**Exemplo:**
```rust
let connection = client.authorize_toolkit("user_123", "github").await?;
if let Some(url) = connection.redirect_url {
    println!("Visit: {}", url);
}
```

### 5. Obter Campos de Iniciação de Conta (`get_connected_account_initiation_fields`)

Recupera os campos necessários para iniciar uma conexão.

**Parâmetros:**
- `toolkit`: Slug do toolkit
- `auth_scheme`: Esquema de autenticação (e.g., "OAUTH2", "API_KEY")
- `required_only`: Se true, retorna apenas campos obrigatórios

**Retorna:**
- Lista de `AuthField` com:
  - Nome do campo
  - Nome de exibição
  - Descrição
  - Tipo
  - Se é obrigatório
  - Valor padrão
  - Valores esperados

**Exemplo:**
```rust
let fields = client.get_connected_account_initiation_fields(
    "github",
    "OAUTH2",
    false
).await?;

for field in fields {
    println!("{}: {}", field.name, field.required);
}
```

### 6. Obter Campos de Criação de Auth Config (`get_auth_config_creation_fields`)

Recupera os campos necessários para criar uma configuração de autenticação.

**Parâmetros:**
- `toolkit`: Slug do toolkit
- `auth_scheme`: Esquema de autenticação
- `required_only`: Se true, retorna apenas campos obrigatórios

**Retorna:**
- Lista de `AuthField` (mesma estrutura acima)

**Exemplo:**
```rust
let fields = client.get_auth_config_creation_fields(
    "github",
    "OAUTH2",
    true
).await?;

for field in fields {
    println!("Required: {}", field.name);
}
```

## Comparação com Python SDK

### Python (Original)

```python
from composio import Composio

composio = Composio(api_key="...")

# List toolkits
toolkits = composio.toolkits.list(
    category="communication",
    limit=20
)

# Get toolkit
github = composio.toolkits.get("github")

# Authorize user
connection = composio.toolkits.authorize(
    user_id="user_123",
    toolkit="github"
)

# Get fields
fields = composio.toolkits.get_connected_account_initiation_fields(
    toolkit="github",
    auth_scheme="OAUTH2",
    required_only=False
)
```

### Rust (Implementado)

```rust
use composio_sdk::client::ComposioClient;
use composio_sdk::models::toolkits::ToolkitListParams;

let client = ComposioClient::builder()
    .api_key("...")
    .build()?;

// List toolkits
let params = ToolkitListParams {
    category: Some("communication".to_string()),
    limit: Some(20),
    ..Default::default()
};
let toolkits = client.list_toolkits(params).await?;

// Get toolkit
let github = client.get_toolkit("github").await?;

// Authorize user
let connection = client.authorize_toolkit("user_123", "github").await?;

// Get fields
let fields = client.get_connected_account_initiation_fields(
    "github",
    "OAUTH2",
    false
).await?;
```

## Detalhes de Implementação

### Tratamento de Erros

Todos os métodos retornam `Result<T, ComposioError>` e incluem:
- Retry automático para erros transitórios
- Validação de parâmetros
- Mensagens de erro descritivas

### Paginação

Suporte completo a paginação com:
- `cursor`: Cursor para próxima página
- `limit`: Número de itens por página
- `next_cursor`: Cursor retornado na resposta

### Serialização/Deserialização

Todas as estruturas implementam:
- `Serialize` e `Deserialize` (serde)
- `Debug` e `Clone`
- Campos opcionais com `#[serde(skip_serializing_if = "Option::is_none")]`

### Testes

Testes unitários incluídos em `src/models/toolkits.rs`:
- Serialização/deserialização de enums
- Valores padrão
- Estruturas de dados

## Exemplo Completo

Veja `examples/toolkits_usage.rs` para um exemplo completo demonstrando:
1. Listagem de toolkits com filtros
2. Filtragem por categoria
3. Filtragem por tipo de gerenciamento
4. Obtenção de detalhes de toolkit
5. Listagem de categorias
6. Obtenção de campos de conexão
7. Obtenção de campos de auth config
8. Autorização de usuário
9. Busca de toolkits

**Executar exemplo:**
```bash
export COMPOSIO_API_KEY="your_api_key"
cargo run --example toolkits_usage
```

## Próximos Passos

Funcionalidades adicionais que podem ser implementadas:
1. Métodos de atualização de auth configs
2. Métodos de gerenciamento de connected accounts
3. Suporte a webhooks de toolkits
4. Cache de metadados de toolkits
5. Validação de campos de auth

## Referências

- **Arquivo Python Original**: `temp/composio/core/models/toolkits.py`
- **Documentação Composio**: [Composio Docs](https://docs.composio.dev)
- **API Reference**: [Toolkits API](https://docs.composio.dev/api-reference/toolkits)
