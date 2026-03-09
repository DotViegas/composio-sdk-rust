# Comparação: custom_tools.py (Python) → Rust

## 📊 Status Atual

**Python (`custom_tools.py`)**: ✅ Completo
**Rust**: ⚠️ **Parcialmente implementado** - Faltam componentes principais

## 🔍 Análise do Arquivo Python

### Componentes Principais

1. **Protocols (Type Hints)**
   - `ExecuteRequestFn` - Função para executar requests proxy
   - `CustomToolProtocol` - Tool sem autenticação
   - `CustomToolWithProxyProtocol` - Tool com autenticação e proxy

2. **CustomTool Class**
   - Wrapper para funções customizadas
   - Parse automático de parâmetros via `inspect`
   - Geração de schema a partir de Pydantic models
   - Suporte para tools com/sem toolkit
   - Execução com autenticação automática

3. **CustomTools Class**
   - Registry de custom tools
   - Decorator `@register` para registrar tools
   - Método `execute()` para executar tools

## 📋 Mapeamento Python → Rust

| Python | Rust (Atual) | Rust (Necessário) | Status |
|--------|--------------|-------------------|--------|
| `ExecuteRequestFn` | - | `ExecuteRequestFn` trait | ❌ Falta |
| `CustomToolProtocol` | - | `CustomToolFn` trait | ❌ Falta |
| `CustomToolWithProxyProtocol` | - | `CustomToolWithProxyFn` trait | ❌ Falta |
| `CustomTool` | `CustomToolDefinition` | `CustomTool` struct completo | ⚠️ Parcial |
| `CustomTools` | - | `CustomToolsRegistry` | ❌ Falta |
| `register()` decorator | - | Macro ou builder pattern | ❌ Falta |

## 🎯 O Que Já Existe no Rust

### ✅ `CustomToolDefinition` (src/models/tools.rs)
```rust
pub struct CustomToolDefinition {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
    pub toolkit: Option<String>,
    pub requires_auth: bool,
}
```

**Limitações**:
- Apenas definição estática
- Não tem lógica de execução
- Não tem registry
- Não tem integração com client

### ✅ `CustomToolExecutionRequest` (src/models/tools.rs)
```rust
pub struct CustomToolExecutionRequest {
    pub slug: String,
    pub arguments: HashMap<String, serde_json::Value>,
    pub user_id: Option<String>,
    pub connected_account_id: Option<String>,
}
```

**Limitações**:
- Apenas estrutura de request
- Não tem lógica de execução

## 🚧 O Que Precisa Ser Implementado

### 1. Traits para Custom Tools

```rust
/// Função de execução de proxy request
pub trait ExecuteRequestFn: Send + Sync {
    fn execute(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<serde_json::Value>,
        connected_account_id: Option<&str>,
        parameters: Option<Vec<ProxyParameter>>,
    ) -> Result<ToolProxyResponse, ComposioError>;
}

/// Custom tool sem autenticação
pub trait CustomToolFn: Send + Sync {
    fn execute(&self, request: serde_json::Value) -> Result<serde_json::Value, ComposioError>;
}

/// Custom tool com autenticação e proxy
pub trait CustomToolWithProxyFn: Send + Sync {
    fn execute(
        &self,
        request: serde_json::Value,
        execute_request: &dyn ExecuteRequestFn,
        auth_credentials: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, ComposioError>;
}
```

### 2. CustomTool Struct Completo

```rust
pub struct CustomTool {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub toolkit: Option<String>,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
    pub requires_auth: bool,
    
    // Função de execução (Box para trait object)
    executor: Box<dyn CustomToolExecutor>,
    
    // Client para operações
    client: Arc<ComposioClient>,
}

impl CustomTool {
    pub fn new_simple<F>(
        name: &str,
        description: &str,
        input_schema: serde_json::Value,
        executor: F,
        client: Arc<ComposioClient>,
    ) -> Self
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value, ComposioError> + Send + Sync + 'static
    {
        // Implementação
    }
    
    pub fn new_with_auth<F>(
        name: &str,
        description: &str,
        toolkit: &str,
        input_schema: serde_json::Value,
        executor: F,
        client: Arc<ComposioClient>,
    ) -> Self
    where
        F: Fn(serde_json::Value, &dyn ExecuteRequestFn, &HashMap<String, serde_json::Value>) 
           -> Result<serde_json::Value, ComposioError> + Send + Sync + 'static
    {
        // Implementação
    }
    
    pub async fn execute(
        &self,
        arguments: HashMap<String, serde_json::Value>,
        user_id: Option<&str>,
    ) -> Result<serde_json::Value, ComposioError> {
        // Implementação
    }
    
    fn get_auth_credentials(&self, user_id: &str) -> Result<HashMap<String, serde_json::Value>, ComposioError> {
        // Buscar connected account mais recente
    }
    
    pub fn to_tool_info(&self) -> Tool {
        // Converter para Tool (formato da API)
    }
}
```

### 3. CustomToolsRegistry

```rust
pub struct CustomToolsRegistry {
    tools: HashMap<String, Arc<CustomTool>>,
    client: Arc<ComposioClient>,
}

impl CustomToolsRegistry {
    pub fn new(client: Arc<ComposioClient>) -> Self {
        Self {
            tools: HashMap::new(),
            client,
        }
    }
    
    pub fn register_simple<F>(
        &mut self,
        name: &str,
        description: &str,
        input_schema: serde_json::Value,
        executor: F,
    ) -> Arc<CustomTool>
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value, ComposioError> + Send + Sync + 'static
    {
        let tool = Arc::new(CustomTool::new_simple(
            name,
            description,
            input_schema,
            executor,
            self.client.clone(),
        ));
        
        self.tools.insert(tool.slug.clone(), tool.clone());
        tool
    }
    
    pub fn register_with_auth<F>(
        &mut self,
        name: &str,
        description: &str,
        toolkit: &str,
        input_schema: serde_json::Value,
        executor: F,
    ) -> Arc<CustomTool>
    where
        F: Fn(serde_json::Value, &dyn ExecuteRequestFn, &HashMap<String, serde_json::Value>) 
           -> Result<serde_json::Value, ComposioError> + Send + Sync + 'static
    {
        let tool = Arc::new(CustomTool::new_with_auth(
            name,
            description,
            toolkit,
            input_schema,
            executor,
            self.client.clone(),
        ));
        
        self.tools.insert(tool.slug.clone(), tool.clone());
        tool
    }
    
    pub fn get(&self, slug: &str) -> Option<Arc<CustomTool>> {
        self.tools.get(slug).cloned()
    }
    
    pub async fn execute(
        &self,
        slug: &str,
        arguments: HashMap<String, serde_json::Value>,
        user_id: Option<&str>,
    ) -> Result<serde_json::Value, ComposioError> {
        let tool = self.get(slug)
            .ok_or_else(|| ComposioError::NotFound(format!("Custom tool {} not found", slug)))?;
        
        tool.execute(arguments, user_id).await
    }
    
    pub fn list(&self) -> Vec<Arc<CustomTool>> {
        self.tools.values().cloned().collect()
    }
}
```

## 🔄 Diferenças de Abordagem

### Python: Decorators + Inspection
```python
@composio.tools.register(toolkit="github")
def my_tool(request: MyRequest) -> MyResponse:
    """Tool description"""
    # Implementation
```

### Rust: Builder Pattern + Closures
```rust
registry.register_simple(
    "my_tool",
    "Tool description",
    input_schema,
    |request| {
        // Implementation
        Ok(response)
    }
);
```

**Por que essa diferença?**
- Python usa decorators e reflexão em runtime
- Rust não tem reflexão, então usamos closures e builders
- Rust requer tipos explícitos (não pode inferir schema automaticamente)

## 📝 Exemplo de Uso (Rust)

### Tool Simples (Sem Autenticação)
```rust
use composio::CustomToolsRegistry;
use serde_json::json;

let mut registry = CustomToolsRegistry::new(client.clone());

// Registrar tool
registry.register_simple(
    "calculate_sum",
    "Calculate the sum of two numbers",
    json!({
        "type": "object",
        "properties": {
            "a": {"type": "number"},
            "b": {"type": "number"}
        },
        "required": ["a", "b"]
    }),
    |request| {
        let a = request["a"].as_f64().unwrap();
        let b = request["b"].as_f64().unwrap();
        Ok(json!({"result": a + b}))
    }
);

// Executar tool
let result = registry.execute(
    "CALCULATE_SUM",
    HashMap::from([
        ("a".to_string(), json!(5)),
        ("b".to_string(), json!(3)),
    ]),
    None,
).await?;
```

### Tool com Autenticação
```rust
registry.register_with_auth(
    "create_custom_issue",
    "Create a custom GitHub issue",
    "github",
    json!({
        "type": "object",
        "properties": {
            "title": {"type": "string"},
            "body": {"type": "string"}
        },
        "required": ["title"]
    }),
    |request, execute_request, auth_credentials| {
        // Usar execute_request para fazer chamadas autenticadas
        let response = execute_request.execute(
            "/repos/owner/repo/issues",
            "POST",
            Some(request),
            None,
            None,
        )?;
        
        Ok(response.data)
    }
);
```

## 🎯 Plano de Implementação

### Fase 1: Estruturas Base ✅
- [x] `CustomToolDefinition` (já existe)
- [x] `CustomToolExecutionRequest` (já existe)

### Fase 2: Traits e Executors ❌
- [ ] `ExecuteRequestFn` trait
- [ ] `CustomToolExecutor` trait (unificado)
- [ ] Implementações de executor

### Fase 3: CustomTool Completo ❌
- [ ] Struct `CustomTool` com executor
- [ ] Método `execute()` com autenticação
- [ ] Método `get_auth_credentials()`
- [ ] Conversão para `Tool`

### Fase 4: Registry ❌
- [ ] Struct `CustomToolsRegistry`
- [ ] Métodos `register_simple()` e `register_with_auth()`
- [ ] Método `execute()`
- [ ] Método `list()`

### Fase 5: Integração ❌
- [ ] Integrar com `ComposioClient`
- [ ] Adicionar ao `Session`
- [ ] Testes unitários
- [ ] Exemplos de uso

## 💡 Desafios Específicos do Rust

1. **Sem Reflexão**: Não podemos inferir schemas automaticamente
   - Solução: Usuário fornece schema JSON explicitamente

2. **Trait Objects**: Precisamos de `Box<dyn Trait>` para armazenar closures
   - Solução: Usar trait objects com `Send + Sync`

3. **Lifetimes**: Closures podem capturar referências
   - Solução: Usar `'static` e `Arc` para compartilhar dados

4. **Async**: Executores podem ser async
   - Solução: Usar `async_trait` ou `Box<dyn Future>`

## 🚀 Próximos Passos

1. **Criar arquivo `src/models/custom_tools.rs`**
2. **Implementar traits base**
3. **Implementar `CustomTool` struct**
4. **Implementar `CustomToolsRegistry`**
5. **Adicionar testes**
6. **Criar exemplo de uso**
7. **Integrar com `ComposioClient`**

## 📚 Referências

- Arquivo Python: `temp/composio/core/models/custom_tools.py`
- Arquivo Rust (atual): `src/models/tools.rs` (parcial)
- Arquivo Rust (novo): `src/models/custom_tools.rs` (a criar)
