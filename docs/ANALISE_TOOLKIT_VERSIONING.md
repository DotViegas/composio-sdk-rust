# 📊 Análise: Sistema de Versionamento de Toolkits

## 🎯 Objetivo
Implementar o sistema de versionamento de toolkits do Python SDK no Rust SDK.

---

## 📋 O que o Python SDK tem (`temp/composio/core/types.py`)

### Tipos Definidos:

```python
# 1. ToolkitLatestVersion - Literal "latest"
ToolkitLatestVersion = te.Literal["latest"]

# 2. ToolkitVersion - Pode ser "latest" OU uma string de versão
ToolkitVersion = t.Union[ToolkitLatestVersion, str]

# 3. ToolkitVersions - Dicionário de versões por toolkit
ToolkitVersions = t.Dict[str, ToolkitVersion]

# 4. ToolkitVersionParam - Parâmetro flexível
ToolkitVersionParam = t.Union[ToolkitVersions, ToolkitLatestVersion, None]
```

### Onde é usado no Python:
- `composio/__init__.py` - Exporta os tipos
- `composio/types.py` - Re-exporta
- `composio/sdk.py` - Usa `ToolkitVersionParam` na config
- `composio/utils/toolkit_version.py` - Gerencia versões
- `composio/core/models/tools.py` - Controla versões de ferramentas
- `composio/core/models/triggers.py` - Controla versões de triggers

---

## ✅ O que o Rust SDK JÁ TEM

### 1. **Estrutura de Versão em ToolSchema** ✅
**Arquivo:** `src/models/response.rs`

```rust
pub struct ToolSchema {
    pub version: String,              // ✅ Versão atual
    pub available_versions: Vec<String>, // ✅ Versões disponíveis
    pub is_deprecated: bool,          // ✅ Flag de depreciação
    // ... outros campos
}
```

### 2. **Parâmetros de Versão em ToolExecutionRequest** ✅
**Arquivo:** `src/models/request.rs`

```rust
pub struct ToolExecutionRequest {
    pub version: Option<String>,  // ✅ Versão específica para executar
    pub dangerously_skip_version_check: Option<bool>, // ✅ Skip de validação
    // ... outros campos
}
```

### 3. **Versão em ToolkitMeta** ✅
**Arquivo:** `src/models/response.rs`

```rust
pub struct ToolkitMeta {
    pub version: String,  // ✅ Versão do toolkit
    // ... outros campos
}
```

---

## ❌ O que FALTA Implementar

### 1. **Tipos de Versionamento Centralizados** ❌

Não existe um módulo central para tipos de versionamento como no Python.

**Solução:** Criar `src/models/versioning.rs`

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Versão "latest" de um toolkit
pub const TOOLKIT_LATEST_VERSION: &str = "latest";

/// Versão de um toolkit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolkitVersion {
    /// Sempre usar a versão mais recente
    Latest,
    /// Versão específica (ex: "20250906_01")
    Specific(String),
}

/// Mapa de versões por toolkit
/// Exemplo: {"github": Latest, "gmail": Specific("20250906_01")}
pub type ToolkitVersions = HashMap<String, ToolkitVersion>;

/// Parâmetro de versão para configuração
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolkitVersionParam {
    /// Mapa de versões específicas por toolkit
    Versions(ToolkitVersions),
    /// Usar "latest" para todos os toolkits
    Latest,
    /// Não especificar versão (usar padrão do servidor)
    None,
}

impl Default for ToolkitVersionParam {
    fn default() -> Self {
        Self::None
    }
}

impl ToolkitVersion {
    /// Converte para string para enviar na API
    pub fn as_str(&self) -> &str {
        match self {
            Self::Latest => TOOLKIT_LATEST_VERSION,
            Self::Specific(version) => version.as_str(),
        }
    }
}
```

### 2. **Integração com SessionConfig** ❌

**Arquivo:** `src/models/request.rs`

Adicionar campo `toolkit_versions`:

```rust
pub struct SessionConfig {
    pub user_id: String,
    pub toolkits: Option<ToolkitFilter>,
    pub auth_configs: Option<HashMap<String, String>>,
    pub connected_accounts: Option<HashMap<String, String>>,
    pub manage_connections: Option<ManageConnectionsConfig>,
    pub tools: Option<ToolsConfig>,
    pub tags: Option<TagsConfig>,
    pub workbench: Option<WorkbenchConfig>,
    
    // ❌ FALTA ADICIONAR:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<ToolkitVersionParam>,
}
```

### 3. **Integração com ComposioConfig** ❌

**Arquivo:** `src/config.rs`

Adicionar campo `toolkit_versions`:

```rust
pub struct ComposioConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
    
    // ❌ FALTA ADICIONAR:
    pub toolkit_versions: Option<ToolkitVersionParam>,
}
```

### 4. **Utilitários de Gerenciamento de Versão** ❌

Criar `src/utils/toolkit_version.rs`:

```rust
use crate::models::versioning::{ToolkitVersion, ToolkitVersionParam, ToolkitVersions};
use std::collections::HashMap;
use std::env;

/// Obtém a versão para um toolkit específico
pub fn get_toolkit_version(
    toolkit_slug: &str,
    toolkit_versions: Option<&ToolkitVersionParam>,
) -> ToolkitVersion {
    // 1. Verificar variável de ambiente específica
    if let Ok(version) = env::var(format!("COMPOSIO_TOOLKIT_VERSION_{}", toolkit_slug.to_uppercase())) {
        return ToolkitVersion::Specific(version);
    }
    
    // 2. Verificar no parâmetro fornecido
    if let Some(versions) = toolkit_versions {
        match versions {
            ToolkitVersionParam::Latest => return ToolkitVersion::Latest,
            ToolkitVersionParam::Versions(map) => {
                if let Some(version) = map.get(toolkit_slug) {
                    return version.clone();
                }
            }
            ToolkitVersionParam::None => {}
        }
    }
    
    // 3. Verificar variável de ambiente global
    if let Ok(version) = env::var("COMPOSIO_TOOLKIT_VERSION") {
        return ToolkitVersion::Specific(version);
    }
    
    // 4. Padrão: Latest
    ToolkitVersion::Latest
}

/// Mescla configurações de versão de múltiplas fontes
pub fn merge_toolkit_versions(
    default: Option<ToolkitVersionParam>,
    override_versions: Option<ToolkitVersionParam>,
) -> ToolkitVersionParam {
    match (default, override_versions) {
        (_, Some(override_val)) => override_val,
        (Some(default_val), None) => default_val,
        (None, None) => ToolkitVersionParam::None,
    }
}

/// Extrai versões de variáveis de ambiente
pub fn get_versions_from_env() -> ToolkitVersions {
    let mut versions = HashMap::new();
    
    for (key, value) in env::vars() {
        if let Some(toolkit) = key.strip_prefix("COMPOSIO_TOOLKIT_VERSION_") {
            versions.insert(
                toolkit.to_lowercase(),
                ToolkitVersion::Specific(value),
            );
        }
    }
    
    versions
}
```

### 5. **Módulo Utils** ❌

Criar `src/utils/mod.rs`:

```rust
//! Utility functions for the Composio SDK

pub mod toolkit_version;

pub use toolkit_version::{
    get_toolkit_version,
    merge_toolkit_versions,
    get_versions_from_env,
};
```

---

## 📝 Plano de Implementação

### Fase 1: Criar Tipos Base ✅ (Pronto para implementar)
1. ✅ Criar `src/models/versioning.rs`
2. ✅ Adicionar exports em `src/models/mod.rs`
3. ✅ Adicionar exports em `src/lib.rs`

### Fase 2: Integrar com Configuração
4. ⏳ Adicionar `toolkit_versions` em `SessionConfig`
5. ⏳ Adicionar `toolkit_versions` em `ComposioConfig`
6. ⏳ Atualizar `SessionBuilder` para aceitar versões

### Fase 3: Criar Utilitários
7. ⏳ Criar `src/utils/mod.rs`
8. ⏳ Criar `src/utils/toolkit_version.rs`
9. ⏳ Adicionar testes unitários

### Fase 4: Integrar com Execução
10. ⏳ Usar versões em `execute_tool()`
11. ⏳ Usar versões em `execute_meta_tool()`
12. ⏳ Adicionar validação de versões

### Fase 5: Documentação e Testes
13. ⏳ Adicionar exemplos de uso
14. ⏳ Atualizar README
15. ⏳ Testes de integração

---

## 🎓 Explicação Didática

### Por que precisamos disso?

Imagine que você tem um aplicativo do GitHub:
- **Versão antiga (20240101_01)**: Tem 50 ferramentas
- **Versão nova (20250906_01)**: Tem 100 ferramentas
- **"latest"**: Sempre pega a mais nova

**Sem versionamento:**
- ❌ Seu código pode quebrar quando o Composio atualiza
- ❌ Você não controla qual versão usar
- ❌ Difícil reproduzir bugs

**Com versionamento:**
- ✅ Você escolhe: "Quero sempre a versão 20250906_01"
- ✅ Ou: "Quero sempre a mais nova (latest)"
- ✅ Ou: "GitHub na v1, Gmail na v2"
- ✅ Seu código fica previsível

### Como funciona?

```rust
// Exemplo 1: Usar "latest" para tudo
let config = ComposioConfig::new("api_key")
    .toolkit_versions(ToolkitVersionParam::Latest);

// Exemplo 2: Versões específicas
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
versions.insert("gmail".to_string(), ToolkitVersion::Latest);

let config = ComposioConfig::new("api_key")
    .toolkit_versions(ToolkitVersionParam::Versions(versions));

// Exemplo 3: Via variável de ambiente
// COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01
// O SDK pega automaticamente!
```

---

## ✅ Checklist de Implementação

- [ ] Criar `src/models/versioning.rs`
- [ ] Atualizar `src/models/mod.rs`
- [ ] Atualizar `src/models/request.rs` (SessionConfig)
- [ ] Atualizar `src/config.rs` (ComposioConfig)
- [ ] Criar `src/utils/mod.rs`
- [ ] Criar `src/utils/toolkit_version.rs`
- [ ] Atualizar `src/lib.rs` (exports)
- [ ] Atualizar `src/session.rs` (usar versões)
- [ ] Adicionar testes unitários
- [ ] Adicionar exemplo de uso
- [ ] Atualizar documentação

---

## 🚀 Próximos Passos

1. **Revisar esta análise** - Você entendeu tudo?
2. **Implementar Fase 1** - Criar os tipos base
3. **Testar** - Garantir que compila e funciona
4. **Continuar** - Fases 2, 3, 4, 5

Quer que eu comece implementando a **Fase 1**?
