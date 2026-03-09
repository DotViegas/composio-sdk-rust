# Guia de Migração: Python SDK → Rust SDK

Este guia ajuda desenvolvedores que conhecem o Python SDK a migrar para o Rust SDK.

## Conceitos Equivalentes

### Criação de Cliente

**Python:**
```python
from composio import Composio

composio = Composio(api_key="your-api-key")
```

**Rust:**
```rust
use composio_sdk::ComposioClient;

let client = ComposioClient::builder()
    .api_key("your-api-key")
    .build()?;
```

### Criação de Sessão

**Python:**
```python
session = composio.create(
    user_id="user_123",
    toolkits=["github", "gmail"],
    experimental={
        "assistive_prompt": {
            "user_timezone": "America/New_York"
        }
    }
)
```

**Rust:**
```rust
let session = client
    .create_session("user_123")
    .toolkits(vec!["github", "gmail"])
    .experimental(Some("America/New_York".to_string()))
    .send()
    .await?;
```

### Execução de Tool

**Python:**
```python
result = session.execute_tool(
    "GITHUB_CREATE_ISSUE",
    arguments={
        "owner": "composio",
        "repo": "composio",
        "title": "Test issue"
    }
)
```

**Rust:**
```rust
use serde_json::json;

let result = session
    .execute_tool(
        "GITHUB_CREATE_ISSUE",
        json!({
            "owner": "composio",
            "repo": "composio",
            "title": "Test issue"
        })
    )
    .await?;
```

### Listagem de Toolkits

**Python:**
```python
toolkits = session.toolkits(
    is_connected=True,
    limit=10
)

for toolkit in toolkits.items:
    print(f"{toolkit.name}: {toolkit.connection.is_active}")
```

**Rust:**
```rust
let toolkits = session
    .list_toolkits()
    .is_connected(true)
    .limit(10)
    .send()
    .await?;

for toolkit in toolkits.items {
    if let Some(connection) = toolkit.connection {
        println!("{}: {}", toolkit.name, connection.is_active);
    }
}
```

### Criação de Auth Link

**Python:**
```python
link = session.authorize(
    toolkit="github",
    callback_url="https://example.com/callback"
)
print(f"Redirect to: {link.redirect_url}")
```

**Rust:**
```rust
let link = session
    .create_auth_link(
        "github",
        Some("https://example.com/callback".to_string())
    )
    .await?;

println!("Redirect to: {}", link.redirect_url);
```

### Meta Tools

**Python:**
```python
from composio.core.models.enums import MetaToolSlug

result = session.execute_meta_tool(
    MetaToolSlug.COMPOSIO_SEARCH_TOOLS,
    arguments={"query": "create a GitHub issue"}
)
```

**Rust:**
```rust
use composio_sdk::models::enums::MetaToolSlug;
use serde_json::json;

let result = session
    .execute_meta_tool(
        MetaToolSlug::ComposioSearchTools,
        json!({"query": "create a GitHub issue"})
    )
    .await?;
```

## Diferenças Principais

### 1. Tratamento de Erros

**Python:**
```python
try:
    result = session.execute_tool("TOOL_SLUG", args)
except ComposioError as e:
    print(f"Error: {e}")
```

**Rust:**
```rust
match session.execute_tool("TOOL_SLUG", args).await {
    Ok(result) => println!("Success: {:?}", result),
    Err(e) => eprintln!("Error: {}", e),
}

// Ou usando o operador ?
let result = session.execute_tool("TOOL_SLUG", args).await?;
```

### 2. Async/Await

**Python:**
```python
# Python usa async/await nativamente
async def main():
    result = await session.execute_tool(...)
```

**Rust:**
```rust
// Rust requer runtime async (Tokio)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = session.execute_tool(...).await?;
    Ok(())
}
```

### 3. Tipos Opcionais

**Python:**
```python
# Python usa None
callback_url = None
# ou
callback_url = "https://example.com"
```

**Rust:**
```rust
// Rust usa Option<T>
let callback_url: Option<String> = None;
// ou
let callback_url = Some("https://example.com".to_string());
```

### 4. Builder Pattern

**Python:**
```python
# Python usa argumentos nomeados
session = composio.create(
    user_id="user_123",
    toolkits=["github"],
    manage_connections=True
)
```

**Rust:**
```rust
// Rust usa builder pattern
let session = client
    .create_session("user_123")
    .toolkits(vec!["github"])
    .manage_connections(true)
    .send()
    .await?;
```

## Tabela de Conversão de Tipos

| Python | Rust |
|--------|------|
| `str` | `String` ou `&str` |
| `int` | `i32`, `i64`, `u32`, `u64` |
| `float` | `f32`, `f64` |
| `bool` | `bool` |
| `None` | `Option::None` |
| `dict` | `HashMap<K, V>` |
| `list` | `Vec<T>` |
| `Optional[T]` | `Option<T>` |
| `Union[A, B]` | `enum` |

## Enums e Constantes

**Python:**
```python
from composio.core.models.enums import MetaToolSlug, TagType

slug = MetaToolSlug.COMPOSIO_SEARCH_TOOLS
tag = TagType.READ_ONLY_HINT
```

**Rust:**
```rust
use composio_sdk::models::enums::{MetaToolSlug, TagType};

let slug = MetaToolSlug::ComposioSearchTools;
let tag = TagType::ReadOnlyHint;
```

## Configurações Avançadas

### Workbench

**Python:**
```python
session = composio.create(
    user_id="user_123",
    workbench={
        "enable_proxy_execution": True,
        "auto_offload_threshold": 1000
    }
)
```

**Rust:**
```rust
let session = client
    .create_session("user_123")
    .workbench(Some(true), Some(1000))
    .send()
    .await?;
```

### Tags

**Python:**
```python
from composio.core.models.enums import TagType

session = composio.create(
    user_id="user_123",
    tags={
        "enable": [TagType.READ_ONLY_HINT],
        "disable": [TagType.DESTRUCTIVE_HINT]
    }
)
```

**Rust:**
```rust
use composio_sdk::models::enums::TagType;

let session = client
    .create_session("user_123")
    .tags(
        Some(vec![TagType::ReadOnlyHint]),
        Some(vec![TagType::DestructiveHint])
    )
    .send()
    .await?;
```

### Auth Configs

**Python:**
```python
session = composio.create(
    user_id="user_123",
    auth_configs={
        "github": "ac_custom_config"
    }
)
```

**Rust:**
```rust
let session = client
    .create_session("user_123")
    .auth_config("github", "ac_custom_config")
    .send()
    .await?;
```

## Dicas de Performance

### 1. Reutilize o Cliente

**Python:**
```python
# Crie uma vez, use várias vezes
composio = Composio(api_key="key")
```

**Rust:**
```rust
// Clone é barato (Arc interno)
let client = ComposioClient::builder().api_key("key").build()?;
let client_clone = client.clone();
```

### 2. Batch Operations

**Python:**
```python
# Use multi-execute para operações em lote
result = session.execute_meta_tool(
    MetaToolSlug.COMPOSIO_MULTI_EXECUTE_TOOL,
    arguments={"tools": [...]}
)
```

**Rust:**
```rust
// Mesmo padrão
let result = session
    .execute_meta_tool(
        MetaToolSlug::ComposioMultiExecuteTool,
        json!({"tools": [...]})
    )
    .await?;
```

### 3. Async Concurrency

**Rust tem vantagem aqui:**
```rust
use tokio::try_join;

// Execute múltiplas operações em paralelo
let (result1, result2, result3) = try_join!(
    session.execute_tool("TOOL1", args1),
    session.execute_tool("TOOL2", args2),
    session.execute_tool("TOOL3", args3)
)?;
```

## Debugging

### Python
```python
import logging
logging.basicConfig(level=logging.DEBUG)
```

### Rust
```rust
// Use o feature local-debug
// Cargo.toml: composio-sdk = { version = "0.1", features = ["local-debug"] }

// Ou configure tracing manualmente
use tracing_subscriber;
tracing_subscriber::fmt::init();
```

## Checklist de Migração

- [ ] Instalar Rust e Cargo
- [ ] Adicionar `composio-sdk` ao `Cargo.toml`
- [ ] Converter imports Python para `use` statements Rust
- [ ] Adicionar `#[tokio::main]` ao main
- [ ] Converter `try/except` para `Result<T, E>`
- [ ] Adicionar `.await?` em chamadas async
- [ ] Converter `None` para `Option::None`
- [ ] Usar builder pattern para configurações
- [ ] Adicionar type annotations onde necessário
- [ ] Testar com `cargo test`
- [ ] Compilar com `cargo build --release`

## Recursos Adicionais

- [Documentação Rust SDK](../README.md)
- [Exemplos](../examples/)
- [Testes](../tests/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## Suporte

Se encontrar problemas na migração:
1. Verifique os exemplos em `examples/`
2. Consulte os testes em `tests/`
3. Leia a documentação em `docs/`
4. Abra uma issue no GitHub
