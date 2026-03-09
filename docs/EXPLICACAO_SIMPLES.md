# 🎓 Explicação Simples: Sistema de Versionamento de Toolkits

## 🤔 O que é?

O sistema de versionamento permite que você controle **qual versão** de cada toolkit (GitHub, Gmail, Slack, etc.) seu aplicativo vai usar.

Pense nisso como escolher qual versão de um aplicativo você quer instalar no seu celular - você pode usar a mais recente ("latest") ou uma versão específica ("20250906_01").

---

## 🎯 Por que é importante?

### Problema sem versionamento:
```
Seu app funciona hoje ✅
Composio atualiza o toolkit do GitHub 🔄
Seu app quebra amanhã ❌
```

### Solução com versionamento:
```
Seu app usa GitHub versão "20250906_01" ✅
Composio atualiza o toolkit do GitHub 🔄
Seu app continua funcionando ✅ (usa a versão antiga)
Você testa a nova versão quando quiser 🧪
Você atualiza quando estiver pronto 🚀
```

---

## 📚 Conceitos Básicos

### 1. ToolkitVersion (Versão de um Toolkit)

```rust
// Duas opções:
ToolkitVersion::Latest              // Sempre usa a mais recente
ToolkitVersion::Specific("20250906_01")  // Usa uma versão específica
```

**Analogia**: É como escolher entre "sempre atualizar automaticamente" ou "ficar nesta versão".

### 2. ToolkitVersionParam (Configuração de Versões)

```rust
// Três opções:

// Opção 1: Usar "latest" para TODOS os toolkits
ToolkitVersionParam::Latest

// Opção 2: Especificar versão para cada toolkit
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
versions.insert("gmail".to_string(), ToolkitVersion::Latest);
ToolkitVersionParam::Versions(versions)

// Opção 3: Não especificar (usa "latest" por padrão)
ToolkitVersionParam::None
```

**Analogia**: É como configurar atualizações automáticas no seu celular - você pode escolher "sempre atualizar tudo", "escolher app por app", ou "deixar no padrão".

---

## 🔄 Como funciona?

### Passo 1: Configurar no Client

```rust
let client = ComposioClient::builder()
    .api_key("sua_chave")
    .toolkit_versions(ToolkitVersionParam::Latest)  // Configuração aqui
    .build()?;
```

### Passo 2: Session herda automaticamente

```rust
let session = client.create_session("user_123").send().await?;
// Session já tem as versões configuradas!
```

### Passo 3: Execução resolve versão automaticamente

```rust
// Quando você executa uma ferramenta:
session.execute_tool("GITHUB_CREATE_ISSUE", args).await?;

// Internamente acontece:
// 1. Extrai "github" de "GITHUB_CREATE_ISSUE"
// 2. Verifica variável de ambiente COMPOSIO_TOOLKIT_VERSION_GITHUB
// 3. Se não tiver, usa a configuração do client
// 4. Se não tiver, usa COMPOSIO_TOOLKIT_VERSION (global)
// 5. Se não tiver, usa "latest"
// 6. Passa a versão resolvida para a API
```

---

## 🎚️ Ordem de Precedência

Imagine uma hierarquia de "quem manda mais":

```
1. COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01  👑 Rei (manda mais)
2. Client config: toolkit_versions               🤴 Príncipe
3. COMPOSIO_TOOLKIT_VERSION=latest               👨 Cidadão
4. Default: "latest"                             🐣 Padrão
```

**Exemplo prático:**
```bash
# Você configura no código:
client.toolkit_versions(github: "20250801_01")

# Mas define variável de ambiente:
export COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01

# Resultado: Usa 20250906_01 (env var vence!)
```

---

## 💡 Casos de Uso

### Caso 1: Desenvolvimento (sempre atualizado)
```rust
let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Latest)
    .build()?;
```
**Quando usar**: Desenvolvimento local, testes, protótipos

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
**Quando usar**: Produção, aplicações críticas, quando precisa de estabilidade

### Caso 3: Híbrido (alguns fixos, outros latest)
```rust
let mut versions = HashMap::new();
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
versions.insert("gmail".to_string(), ToolkitVersion::Latest);

let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Versions(versions))
    .build()?;
```
**Quando usar**: Quando alguns toolkits são críticos e outros podem atualizar

### Caso 4: Override por usuário
```rust
// Cliente padrão
let client = ComposioClient::builder()
    .api_key("key")
    .toolkit_versions(ToolkitVersionParam::Latest)
    .build()?;

// Usuário específico precisa de versão antiga
let mut user_versions = HashMap::new();
user_versions.insert("github".to_string(), ToolkitVersion::Specific("20250801_01".to_string()));

let session = client
    .create_session("user_vip")
    .toolkit_versions(ToolkitVersionParam::Versions(user_versions))
    .send()
    .await?;
```
**Quando usar**: Quando diferentes usuários precisam de versões diferentes

---

## 🔍 Debugging

### Como saber qual versão está sendo usada?

```rust
use composio_sdk::utils::toolkit_version::get_toolkit_version;

let version = get_toolkit_version("github", client.config().toolkit_versions.as_ref());
println!("Usando versão: {}", version.as_str());
```

### Como testar uma nova versão?

```bash
# Opção 1: Variável de ambiente (temporário)
export COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01
cargo run

# Opção 2: Mudar no código (permanente)
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
```

---

## ⚠️ Armadilhas Comuns

### ❌ Erro 1: Usar "latest" em produção
```rust
// RUIM para produção
let client = ComposioClient::builder()
    .toolkit_versions(ToolkitVersionParam::Latest)
    .build()?;
```
**Por quê?** Seu app pode quebrar quando Composio atualizar.

### ❌ Erro 2: Esquecer de atualizar versões
```rust
// Versão muito antiga
versions.insert("github".to_string(), ToolkitVersion::Specific("20240101_01".to_string()));
```
**Por quê?** Você perde novos recursos e correções de bugs.

### ❌ Erro 3: Não testar antes de atualizar
```rust
// Atualizar direto em produção
versions.insert("github".to_string(), ToolkitVersion::Latest);
```
**Por quê?** Pode ter breaking changes.

---

## ✅ Boas Práticas

### 1. Desenvolvimento: Use "latest"
```rust
#[cfg(debug_assertions)]
let versions = ToolkitVersionParam::Latest;
```

### 2. Produção: Use versões específicas
```rust
#[cfg(not(debug_assertions))]
let versions = ToolkitVersionParam::Versions(production_versions);
```

### 3. Teste novas versões em staging
```bash
# Staging
export COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01

# Produção (só depois de testar)
versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
```

### 4. Documente as versões usadas
```rust
// versions.toml
// github = "20250906_01"  # Testado em 2026-03-08
// gmail = "20250801_01"   # Testado em 2026-02-15
```

---

## 🎓 Resumo para Iniciantes

1. **O que é?** Sistema para controlar versões de toolkits
2. **Por que usar?** Evitar que seu app quebre com atualizações
3. **Como usar?** Configurar no client builder
4. **Quando usar "latest"?** Desenvolvimento e testes
5. **Quando usar versões específicas?** Produção
6. **Como testar?** Variáveis de ambiente
7. **Como debugar?** `get_toolkit_version()`

---

## 🚀 Próximos Passos

1. Leia o exemplo completo: `examples/toolkit_versioning_integration.rs`
2. Execute: `cargo run --example toolkit_versioning_integration`
3. Experimente com variáveis de ambiente
4. Teste em seu projeto

---

**Dúvidas?** Consulte:
- `docs/01-INTEGRATION_PLAN.md` - Plano técnico detalhado
- `docs/01-CHANGES_IMPLEMENTED.md` - O que foi implementado
- `examples/toolkit_versioning.rs` - Exemplo básico
- `examples/toolkit_versioning_integration.rs` - Exemplo completo

---

**Lembre-se**: Versionamento é como um seguro - você espera nunca precisar, mas fica feliz quando tem! 🛡️
