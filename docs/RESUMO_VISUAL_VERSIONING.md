# 🎨 Resumo Visual: Sistema de Versionamento

## 📊 Status Atual vs Necessário

```
┌─────────────────────────────────────────────────────────────┐
│                    PYTHON SDK (Referência)                  │
├─────────────────────────────────────────────────────────────┤
│ ✅ temp/composio/core/types.py                              │
│    ├─ ToolkitLatestVersion = "latest"                       │
│    ├─ ToolkitVersion = "latest" | "20250906_01"             │
│    ├─ ToolkitVersions = {"github": "latest", ...}           │
│    └─ ToolkitVersionParam = Versions | Latest | None        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    RUST SDK (Atual)                         │
├─────────────────────────────────────────────────────────────┤
│ ✅ src/models/response.rs                                   │
│    └─ ToolSchema { version, available_versions, ... }       │
│                                                              │
│ ✅ src/models/request.rs                                    │
│    └─ ToolExecutionRequest { version, ... }                 │
│                                                              │
│ ❌ FALTA: Tipos centralizados de versionamento              │
│ ❌ FALTA: Integração com SessionConfig                      │
│ ❌ FALTA: Integração com ComposioConfig                     │
│ ❌ FALTA: Utilitários de gerenciamento                      │
└─────────────────────────────────────────────────────────────┘
```

---

## 🏗️ Arquitetura Proposta

```
src/
├── models/
│   ├── mod.rs
│   ├── enums.rs          ✅ Já existe
│   ├── request.rs        ✅ Já existe (precisa atualizar)
│   ├── response.rs       ✅ Já existe
│   └── versioning.rs     ❌ CRIAR NOVO
│
├── utils/                ❌ CRIAR NOVO DIRETÓRIO
│   ├── mod.rs            ❌ CRIAR NOVO
│   └── toolkit_version.rs ❌ CRIAR NOVO
│
├── config.rs             ✅ Já existe (precisa atualizar)
├── session.rs            ✅ Já existe (precisa atualizar)
└── lib.rs                ✅ Já existe (precisa atualizar)
```

---

## 🔄 Fluxo de Dados

```
┌──────────────────────────────────────────────────────────────┐
│                    USUÁRIO                                   │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────────┐
│  ComposioConfig                                              │
│  ├─ api_key: "xxx"                                           │
│  └─ toolkit_versions: ToolkitVersionParam                    │
│     ├─ Latest                                                │
│     ├─ Versions({"github": "v1", "gmail": "latest"})        │
│     └─ None                                                  │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────────┐
│  SessionConfig                                               │
│  ├─ user_id: "user_123"                                      │
│  ├─ toolkits: ["github", "gmail"]                           │
│  └─ toolkit_versions: ToolkitVersionParam (herdado)          │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────────┐
│  get_toolkit_version("github", versions)                     │
│  ├─ 1. Verifica COMPOSIO_TOOLKIT_VERSION_GITHUB             │
│  ├─ 2. Verifica no ToolkitVersionParam                      │
│  ├─ 3. Verifica COMPOSIO_TOOLKIT_VERSION                    │
│  └─ 4. Retorna "latest" (padrão)                            │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────────┐
│  API Request                                                 │
│  POST /tool_router/session/{id}/execute                      │
│  {                                                           │
│    "tool_slug": "GITHUB_CREATE_ISSUE",                      │
│    "version": "20250906_01",  ← Versão resolvida            │
│    "arguments": {...}                                        │
│  }                                                           │
└──────────────────────────────────────────────────────────────┘
```

---

## 📦 Tipos de Dados

### ToolkitVersion (Enum)
```rust
enum ToolkitVersion {
    Latest,              // "latest"
    Specific(String),    // "20250906_01"
}
```

**Exemplos:**
```rust
ToolkitVersion::Latest
ToolkitVersion::Specific("20250906_01".to_string())
```

---

### ToolkitVersions (HashMap)
```rust
type ToolkitVersions = HashMap<String, ToolkitVersion>;
```

**Exemplo:**
```rust
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Latest);
versions.insert("gmail".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
```

---

### ToolkitVersionParam (Enum)
```rust
enum ToolkitVersionParam {
    Versions(ToolkitVersions),  // Mapa específico
    Latest,                     // "latest" para todos
    None,                       // Não especificar
}
```

**Exemplos:**
```rust
// Opção 1: Latest para todos
ToolkitVersionParam::Latest

// Opção 2: Versões específicas
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Specific("v1".to_string()));
ToolkitVersionParam::Versions(versions)

// Opção 3: Não especificar (usar padrão do servidor)
ToolkitVersionParam::None
```

---

## 🎯 Casos de Uso

### Caso 1: Desenvolvimento (sempre a mais nova)
```rust
let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Latest)
    .build()?;
```

### Caso 2: Produção (versões fixas)
```rust
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
versions.insert("gmail".to_string(), ToolkitVersion::Specific("20250801_01".to_string()));

let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Versions(versions))
    .build()?;
```

### Caso 3: Via Variáveis de Ambiente
```bash
export COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01
export COMPOSIO_TOOLKIT_VERSION_GMAIL=20250801_01
```

```rust
// O SDK pega automaticamente das variáveis de ambiente
let client = ComposioClient::builder()
    .api_key("key")
    .build()?;
```

### Caso 4: Misto (algumas fixas, outras latest)
```rust
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
versions.insert("gmail".to_string(), ToolkitVersion::Latest);
versions.insert("slack".to_string(), ToolkitVersion::Latest);

let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Versions(versions))
    .build()?;
```

---

## 🔍 Ordem de Precedência

```
┌─────────────────────────────────────────────────────────────┐
│  Qual versão usar para o toolkit "github"?                  │
└─────────────────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  1️⃣ Variável de Ambiente Específica                         │
│     COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01             │
│     ✅ Se existir, usa esta                                 │
└────────────────────┬────────────────────────────────────────┘
                     │ Não existe
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  2️⃣ Parâmetro toolkit_versions                              │
│     ToolkitVersionParam::Versions({"github": "v1"})         │
│     ✅ Se existir no mapa, usa esta                         │
└────────────────────┬────────────────────────────────────────┘
                     │ Não existe
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  3️⃣ Variável de Ambiente Global                             │
│     COMPOSIO_TOOLKIT_VERSION=20250906_01                    │
│     ✅ Se existir, usa esta para TODOS os toolkits          │
└────────────────────┬────────────────────────────────────────┘
                     │ Não existe
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  4️⃣ Padrão: "latest"                                        │
│     ✅ Sempre usa a versão mais recente                     │
└─────────────────────────────────────────────────────────────┘
```

---

## 📝 Checklist Visual

```
Fase 1: Tipos Base
├─ [ ] Criar src/models/versioning.rs
│   ├─ [ ] ToolkitVersion enum
│   ├─ [ ] ToolkitVersions type
│   ├─ [ ] ToolkitVersionParam enum
│   └─ [ ] Testes unitários
│
├─ [ ] Atualizar src/models/mod.rs
│   └─ [ ] Adicionar pub use versioning::*
│
└─ [ ] Atualizar src/lib.rs
    └─ [ ] Exportar tipos públicos

Fase 2: Integração Config
├─ [ ] Atualizar src/config.rs
│   └─ [ ] Adicionar campo toolkit_versions
│
├─ [ ] Atualizar src/models/request.rs
│   └─ [ ] Adicionar campo toolkit_versions em SessionConfig
│
└─ [ ] Atualizar src/session.rs
    └─ [ ] SessionBuilder aceitar versões

Fase 3: Utilitários
├─ [ ] Criar src/utils/mod.rs
│
└─ [ ] Criar src/utils/toolkit_version.rs
    ├─ [ ] get_toolkit_version()
    ├─ [ ] merge_toolkit_versions()
    ├─ [ ] get_versions_from_env()
    └─ [ ] Testes unitários

Fase 4: Integração Execução
├─ [ ] Atualizar execute_tool()
│   └─ [ ] Usar versão resolvida
│
└─ [ ] Atualizar execute_meta_tool()
    └─ [ ] Usar versão resolvida

Fase 5: Docs e Testes
├─ [ ] Criar exemplo examples/toolkit_versioning.rs
├─ [ ] Atualizar README.md
├─ [ ] Testes de integração
└─ [ ] Documentação inline
```

---

## 🎓 Analogia Simples

Pense no versionamento como **versões de um aplicativo no celular**:

```
📱 GitHub App
├─ v1.0 (antiga) - Tem 50 recursos
├─ v2.0 (atual) - Tem 100 recursos
└─ "latest" (sempre a mais nova)

Sem versionamento:
❌ Seu código usa "GitHub App"
❌ Amanhã atualiza para v2.0
❌ Seu código quebra porque mudou

Com versionamento:
✅ Seu código usa "GitHub App v1.0"
✅ Amanhã atualiza para v2.0
✅ Seu código continua usando v1.0
✅ Quando você quiser, atualiza para v2.0
```

---

## 🚀 Começar Agora?

Quer que eu implemente a **Fase 1** (Tipos Base)?

Vou criar:
1. `src/models/versioning.rs` - Tipos completos
2. Atualizar `src/models/mod.rs` - Exports
3. Atualizar `src/lib.rs` - Exports públicos
4. Testes unitários

Posso começar? 🎯
