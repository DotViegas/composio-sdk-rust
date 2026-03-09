# 🎯 Plano de Integração: Sistema de Versionamento de Toolkits

## 📋 Status Atual

### ✅ O que JÁ está implementado:
1. **Tipos completos** (`src/models/versioning.rs`):
   - `ToolkitVersion` (Latest | Specific)
   - `ToolkitVersions` (HashMap)
   - `ToolkitVersionParam` (Versions | Latest | None)
   - Serialização/deserialização funcionando
   - 15 testes unitários passando

2. **Utilitários completos** (`src/utils/toolkit_version.rs`):
   - `get_toolkit_version()` - Resolve versão com precedência correta
   - `get_versions_from_env()` - Extrai de variáveis de ambiente
   - `merge_toolkit_versions()` - Merge de configurações
   - 15 testes unitários passando

3. **Exemplo funcional** (`examples/toolkit_versioning.rs`):
   - Demonstra uso dos tipos e utilitários
   - Compila e executa sem erros

### ❌ O que FALTA integrar:
1. **ComposioConfig** não tem campo `toolkit_versions`
2. **ComposioClientBuilder** não aceita `toolkit_versions`
3. **SessionConfig** não tem campo `toolkit_versions`
4. **Session** não armazena `toolkit_versions`
5. **execute_tool()** não resolve versão automaticamente
6. **execute_meta_tool()** não resolve versão automaticamente

---

## 🎯 Objetivo

Integrar o sistema de versionamento já implementado com o resto do SDK, seguindo o mesmo comportamento do Python SDK.

---

## 📊 Comparação: Python vs Rust

### Python SDK (Comportamento Esperado)

```python
# 1. Configuração no nível do SDK
composio = Composio(
    api_key="key",
    toolkit_versions={
        "github": "20250906_01",
        "gmail": "latest"
    }
)

# 2. Passa para Tools e Triggers
self.tools = Tools(toolkit_versions=toolkit_versions)
self.triggers = Triggers(toolkit_versions=toolkit_versions)

# 3. Resolve versão automaticamente na execução
def execute(self, slug: str, ...):
    if not version:
        toolkit = extract_toolkit_from_slug(slug)
        version = get_toolkit_version(toolkit, self.toolkit_versions)
    # Executa com versão resolvida
```

### Rust SDK (Como deve ficar)

```rust
// 1. Configuração no nível do SDK
let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Versions(versions))
    .build()?;

// 2. Armazena em ComposioConfig
// 3. Passa para Session
// 4. Resolve versão automaticamente em execute_tool()
```

---

## 🔧 Modificações Necessárias

### 1. `src/config.rs` - Adicionar campo toolkit_versions

```rust
pub struct ComposioConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
    pub toolkit_versions: Option<ToolkitVersionParam>,  // NOVO
}

impl ComposioConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://backend.composio.dev/api/v3".to_string(),
            timeout: Duration::from_secs(30),
            retry_policy: RetryPolicy::default(),
            toolkit_versions: None,  // NOVO - default None
        }
    }
}
```

**Testes a adicionar:**
- `test_config_with_toolkit_versions()`
- `test_config_default_toolkit_versions_is_none()`

---

### 2. `src/client.rs` - Builder aceitar toolkit_versions

```rust
pub struct ComposioClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    initial_retry_delay: Option<Duration>,
    max_retry_delay: Option<Duration>,
    toolkit_versions: Option<ToolkitVersionParam>,  // NOVO
}

impl ComposioClientBuilder {
    pub fn toolkit_versions(mut self, versions: ToolkitVersionParam) -> Self {
        self.toolkit_versions = Some(versions);
        self
    }

    pub fn build(self) -> Result<ComposioClient, ComposioError> {
        // ... código existente ...
        
        if let Some(toolkit_versions) = self.toolkit_versions {
            config.toolkit_versions = Some(toolkit_versions);
        }
        
        // ... resto do código ...
    }
}
```

**Testes a adicionar:**
- `test_builder_with_toolkit_versions_latest()`
- `test_builder_with_toolkit_versions_map()`
- `test_builder_with_toolkit_versions_none()`

---

### 3. `src/models/request.rs` - Adicionar em SessionConfig

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<ToolkitVersionParam>,  // NOVO
}
```

**Testes a adicionar:**
- `test_session_config_with_toolkit_versions()`
- `test_session_config_toolkit_versions_serialization()`

---

### 4. `src/session.rs` - Armazenar e usar toolkit_versions

#### 4.1. Adicionar campo na struct Session

```rust
#[derive(Debug, Clone)]
pub struct Session {
    client: Arc<ComposioClient>,
    session_id: String,
    mcp_url: String,
    tools: Vec<String>,
    toolkit_versions: Option<ToolkitVersionParam>,  // NOVO
}
```

#### 4.2. Atualizar from_response()

```rust
impl Session {
    pub(crate) fn from_response(
        client: ComposioClient,
        response: crate::models::response::SessionResponse,
    ) -> Self {
        Self {
            client: Arc::new(client.clone()),
            session_id: response.session_id,
            mcp_url: response.mcp.url,
            tools: response.tool_router_tools,
            toolkit_versions: client.config().toolkit_versions.clone(),  // NOVO
        }
    }
}
```

#### 4.3. Criar função helper extract_toolkit_from_slug()

```rust
/// Extract toolkit slug from tool slug
///
/// Examples:
/// - "GITHUB_CREATE_ISSUE" -> "github"
/// - "GMAIL_SEND_EMAIL" -> "gmail"
/// - "COMPOSIO_SEARCH_TOOLS" -> "composio"
fn extract_toolkit_from_slug(tool_slug: &str) -> String {
    tool_slug
        .split('_')
        .next()
        .unwrap_or(tool_slug)
        .to_lowercase()
}

#[cfg(test)]
mod toolkit_extraction_tests {
    use super::*;

    #[test]
    fn test_extract_toolkit_from_slug() {
        assert_eq!(extract_toolkit_from_slug("GITHUB_CREATE_ISSUE"), "github");
        assert_eq!(extract_toolkit_from_slug("GMAIL_SEND_EMAIL"), "gmail");
        assert_eq!(extract_toolkit_from_slug("SLACK_POST_MESSAGE"), "slack");
        assert_eq!(extract_toolkit_from_slug("COMPOSIO_SEARCH_TOOLS"), "composio");
    }

    #[test]
    fn test_extract_toolkit_from_single_word() {
        assert_eq!(extract_toolkit_from_slug("GITHUB"), "github");
    }

    #[test]
    fn test_extract_toolkit_empty_string() {
        assert_eq!(extract_toolkit_from_slug(""), "");
    }
}
```

#### 4.4. Modificar execute_tool() para resolver versão

```rust
pub async fn execute_tool(
    &self,
    tool_slug: impl Into<String>,
    arguments: serde_json::Value,
) -> Result<crate::models::response::ToolExecutionResponse, ComposioError> {
    use crate::models::request::ToolExecutionRequest;
    use crate::models::response::ToolExecutionResponse;
    use crate::retry::with_retry;
    use crate::utils::toolkit_version::get_toolkit_version;  // NOVO

    let tool_slug = tool_slug.into();
    
    // NOVO: Extrair toolkit do slug
    let toolkit = extract_toolkit_from_slug(&tool_slug);
    
    // NOVO: Resolver versão usando utilitário já implementado
    let version = get_toolkit_version(
        &toolkit,
        self.toolkit_versions.as_ref()
    );
    
    let url = format!(
        "{}/tool_router/session/{}/execute",
        self.client.config().base_url,
        self.session_id
    );

    // Create request body
    let request_body = ToolExecutionRequest {
        tool_slug: tool_slug.clone(),
        arguments: Some(arguments),
        version: Some(version.as_str().to_string()),  // NOVO: Usar versão resolvida
        ..Default::default()
    };

    // ... resto do código permanece igual ...
}
```

#### 4.5. Modificar execute_meta_tool() para resolver versão

```rust
pub async fn execute_meta_tool(
    &self,
    slug: crate::models::enums::MetaToolSlug,
    arguments: serde_json::Value,
) -> Result<crate::models::response::MetaToolExecutionResponse, ComposioError> {
    use crate::models::request::MetaToolExecutionRequest;
    use crate::models::response::MetaToolExecutionResponse;
    use crate::retry::with_retry;
    use crate::utils::toolkit_version::get_toolkit_version;  // NOVO

    // NOVO: Meta tools são do toolkit "composio"
    let toolkit = "composio";
    
    // NOVO: Resolver versão
    let version = get_toolkit_version(
        toolkit,
        self.toolkit_versions.as_ref()
    );

    let url = format!(
        "{}/tool_router/session/{}/execute_meta",
        self.client.config().base_url,
        self.session_id
    );

    // Create request body
    let request_body = MetaToolExecutionRequest {
        slug,
        arguments: Some(arguments),
        // NOTA: MetaToolExecutionRequest não tem campo version
        // Isso pode precisar ser adicionado ou passado de outra forma
    };

    // ... resto do código permanece igual ...
}
```

**NOTA:** Verificar se `MetaToolExecutionRequest` precisa de campo `version`.

#### 4.6. Atualizar SessionBuilder

```rust
impl<'a> SessionBuilder<'a> {
    pub fn new(client: &'a ComposioClient, user_id: String) -> Self {
        Self {
            client,
            user_id: user_id.clone(),
            config: SessionConfig {
                user_id,
                toolkits: None,
                auth_configs: None,
                connected_accounts: None,
                manage_connections: None,
                tools: None,
                tags: None,
                workbench: None,
                toolkit_versions: client.config().toolkit_versions.clone(),  // NOVO
            },
        }
    }
    
    // NOVO: Método para override toolkit_versions
    pub fn toolkit_versions(mut self, versions: ToolkitVersionParam) -> Self {
        self.config.toolkit_versions = Some(versions);
        self
    }
}
```

**Testes a adicionar:**
- `test_session_builder_toolkit_versions()`
- `test_session_builder_inherits_client_toolkit_versions()`
- `test_session_builder_override_toolkit_versions()`

---

## 🧪 Testes de Integração

### Arquivo: `tests/toolkit_versioning_integration.rs`

```rust
use composio_sdk::{ComposioClient, ToolkitVersion, ToolkitVersionParam};
use std::collections::HashMap;

#[tokio::test]
async fn test_toolkit_versions_from_client_config() {
    let mut versions = HashMap::new();
    versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
    
    let client = ComposioClient::builder()
        .api_key("test_key")
        .toolkit_versions(ToolkitVersionParam::Versions(versions))
        .build()
        .unwrap();
    
    assert!(client.config().toolkit_versions.is_some());
}

#[tokio::test]
async fn test_toolkit_versions_inherited_by_session() {
    let mut versions = HashMap::new();
    versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
    
    let client = ComposioClient::builder()
        .api_key("test_key")
        .toolkit_versions(ToolkitVersionParam::Versions(versions))
        .build()
        .unwrap();
    
    // Session deve herdar toolkit_versions do client
    let session_builder = client.create_session("user_123");
    // Verificar que config tem toolkit_versions
}

#[tokio::test]
async fn test_toolkit_versions_from_env() {
    std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");
    
    let client = ComposioClient::builder()
        .api_key("test_key")
        .build()
        .unwrap();
    
    // Versão deve ser resolvida de env var
    
    std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
}

#[tokio::test]
async fn test_extract_toolkit_from_slug() {
    // Testar função helper
}
```

---

## 📝 Checklist de Implementação

### Fase 1: Tipos e Config (30 min)
- [ ] Adicionar `toolkit_versions` em `ComposioConfig`
- [ ] Atualizar `ComposioConfig::new()` com default `None`
- [ ] Adicionar testes para `ComposioConfig`

### Fase 2: Builder (30 min)
- [ ] Adicionar campo `toolkit_versions` em `ComposioClientBuilder`
- [ ] Implementar método `.toolkit_versions()`
- [ ] Atualizar `build()` para passar para config
- [ ] Adicionar testes para builder

### Fase 3: SessionConfig (20 min)
- [ ] Adicionar campo `toolkit_versions` em `SessionConfig`
- [ ] Adicionar testes de serialização

### Fase 4: Session (1h)
- [ ] Adicionar campo `toolkit_versions` em `Session`
- [ ] Atualizar `from_response()` para copiar do client
- [ ] Criar função `extract_toolkit_from_slug()`
- [ ] Adicionar testes para extração de toolkit

### Fase 5: execute_tool() (30 min)
- [ ] Importar `get_toolkit_version`
- [ ] Extrair toolkit do slug
- [ ] Resolver versão
- [ ] Passar versão no request
- [ ] Adicionar testes

### Fase 6: execute_meta_tool() (30 min)
- [ ] Verificar se precisa de campo `version`
- [ ] Implementar resolução de versão
- [ ] Adicionar testes

### Fase 7: SessionBuilder (20 min)
- [ ] Herdar `toolkit_versions` do client em `new()`
- [ ] Adicionar método `.toolkit_versions()` para override
- [ ] Adicionar testes

### Fase 8: Testes de Integração (1h)
- [ ] Criar `tests/toolkit_versioning_integration.rs`
- [ ] Testar fluxo completo client -> session -> execute
- [ ] Testar precedência de env vars
- [ ] Testar override em session

### Fase 9: Documentação (30 min)
- [ ] Atualizar exemplos
- [ ] Atualizar README
- [ ] Criar exemplo de uso completo

---

## ⏱️ Estimativa Total: 5-6 horas

---

## 🎯 Resultado Esperado

Após a integração, o SDK Rust terá:

1. ✅ Versionamento configurável no nível do client
2. ✅ Herança automática para sessions
3. ✅ Resolução automática de versões em execução
4. ✅ Suporte a variáveis de ambiente
5. ✅ Precedência correta (env > config > default)
6. ✅ Compatibilidade 100% com Python SDK

---

## 🚀 Próximos Passos

Após esta integração estar completa, as próximas prioridades são:

1. **File Management** (2-3 dias)
2. **Tool Modifiers** (2-3 dias)
3. **Triggers completos** (2-3 dias)
4. **Custom Tools** (2-3 dias)

---

**Pronto para começar a implementação?** 🎉
