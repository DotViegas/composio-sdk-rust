# Provider System - Guia Visual

## 🎬 Fluxo Completo: Do Composio ao Framework

```
┌─────────────────────────────────────────────────────────────────────┐
│ 1. COMPOSIO CORE - Formato Universal                                │
│                                                                      │
│   Tool {                                                             │
│     slug: "GITHUB_CREATE_ISSUE"                                      │
│     description: "Create a GitHub issue"                             │
│     input_parameters: {                                              │
│       type: "object",                                                │
│       properties: {                                                  │
│         owner: {type: "string"},                                     │
│         repo: {type: "string"},                                      │
│         title: {type: "string"}                                      │
│       }                                                              │
│     }                                                                │
│   }                                                                  │
└──────────────────────────┬───────────────────────────────────────────┘
                           │
                           │ provider.wrap_tool(tool)
                           │
        ┌──────────────────┴──────────────────┐
        │                                     │
        ▼                                     ▼
┌───────────────────────┐          ┌────────────────────────┐
│ 2A. OPENAI FORMAT     │          │ 2B. ANTHROPIC FORMAT   │
│                       │          │                        │
│ ChatCompletionTool {  │          │ Tool {                 │
│   type: "function",   │          │   name: "GITHUB_...",  │
│   function: {         │          │   description: "...",  │
│     name: "GITHUB_...",│         │   input_schema: {      │
│     description: "...",│         │     type: "object",    │
│     parameters: {...} │          │     properties: {...}  │
│   }                   │          │   }                    │
│ }                     │          │ }                      │
└───────────────────────┘          └────────────────────────┘
```

---

## 🔄 Ciclo de Vida: NonAgentic vs Agentic

### NonAgentic (Controle Manual)

```
┌─────────────┐
│ 1. Developer│
│   Calls SDK │
└──────┬──────┘
       │
       │ composio.tools.get(user_id, toolkits=["github"])
       │
       ▼
┌──────────────────┐
│ 2. Provider      │
│   Converts Tools │
└──────┬───────────┘
       │
       │ Returns: [ChatCompletionToolParam, ...]
       │
       ▼
┌──────────────────┐
│ 3. Developer     │
│   Passes to LLM  │
└──────┬───────────┘
       │
       │ openai.chat.completions.create(tools=tools)
       │
       ▼
┌──────────────────┐
│ 4. LLM           │
│   Returns        │
│   Tool Calls     │
└──────┬───────────┘
       │
       │ response.choices[0].message.tool_calls
       │
       ▼
┌──────────────────┐
│ 5. Developer     │
│   MANUALLY       │
│   Executes Tools │
└──────┬───────────┘
       │
       │ for tool_call in tool_calls:
       │     composio.tools.execute(...)
       │
       ▼
┌──────────────────┐
│ 6. Composio      │
│   Executes &     │
│   Returns Result │
└──────┬───────────┘
       │
       │ {data: {...}, error: null, successful: true}
       │
       ▼
┌──────────────────┐
│ 7. Developer     │
│   Sends Results  │
│   Back to LLM    │
└──────────────────┘
```

### Agentic (Automático)

```
┌─────────────┐
│ 1. Developer│
│   Calls SDK │
└──────┬──────┘
       │
       │ composio.tools.get(user_id, toolkits=["github"])
       │
       ▼
┌──────────────────┐
│ 2. Provider      │
│   Converts Tools │
│   + Injects      │
│   Execute Fn     │
└──────┬───────────┘
       │
       │ Returns: [FunctionTool(on_invoke=execute_fn), ...]
       │
       ▼
┌──────────────────┐
│ 3. Developer     │
│   Passes to      │
│   Agent          │
└──────┬───────────┘
       │
       │ agent = Agent(tools=tools)
       │ agent.run("Create issue")
       │
       ▼
┌──────────────────────────────────────────────────┐
│ 4. Framework Loop (AUTOMATIC)                    │
│                                                   │
│   ┌─────────────────────────────────────┐        │
│   │ a. Call LLM                         │        │
│   └────────────┬────────────────────────┘        │
│                │                                  │
│                ▼                                  │
│   ┌─────────────────────────────────────┐        │
│   │ b. Detect Tool Calls                │        │
│   └────────────┬────────────────────────┘        │
│                │                                  │
│                ▼                                  │
│   ┌─────────────────────────────────────┐        │
│   │ c. AUTOMATICALLY Execute            │        │
│   │    (calls on_invoke_tool)           │        │
│   └────────────┬────────────────────────┘        │
│                │                                  │
│                ▼                                  │
│   ┌─────────────────────────────────────┐        │
│   │ d. Send Results Back to LLM         │        │
│   └────────────┬────────────────────────┘        │
│                │                                  │
│                ▼                                  │
│   ┌─────────────────────────────────────┐        │
│   │ e. Repeat Until Task Complete       │        │
│   └─────────────────────────────────────┘        │
│                                                   │
└───────────────────────────────────────────────────┘
       │
       │ Final result
       │
       ▼
┌──────────────────┐
│ 5. Developer     │
│   Gets Final     │
│   Result         │
└──────────────────┘
```

---

## 🎭 Exemplo Prático: Mesma Tarefa, Diferentes Providers

### Tarefa: "Create a GitHub issue titled 'Bug fix'"

#### OpenAI Provider (NonAgentic)

```python
# Setup
composio = Composio(provider=OpenAIProvider())
tools = composio.tools.get(user_id="user_123", toolkits=["github"])

# Step 1: Call LLM
response = openai.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Create issue 'Bug fix'"}],
    tools=tools  # ← [ChatCompletionToolParam, ...]
)

# Step 2: Extract tool calls
tool_call = response.choices[0].message.tool_calls[0]
# {
#   id: "call_123",
#   function: {
#     name: "GITHUB_CREATE_ISSUE",
#     arguments: '{"owner":"composio","repo":"composio","title":"Bug fix"}'
#   }
# }

# Step 3: Execute manually
result = composio.tools.execute(
    slug=tool_call.function.name,
    arguments=json.loads(tool_call.function.arguments),
    user_id="user_123"
)

# Step 4: Send back to LLM
final_response = openai.chat.completions.create(
    model="gpt-4",
    messages=[
        {"role": "user", "content": "Create issue 'Bug fix'"},
        response.choices[0].message,
        {"role": "tool", "tool_call_id": tool_call.id, "content": json.dumps(result)}
    ]
)
```

#### OpenAI Agents Provider (Agentic)

```python
# Setup
composio = Composio(provider=OpenAIAgentsProvider())
tools = composio.tools.get(user_id="user_123", toolkits=["github"])

# One call - framework handles everything!
agent = Agent(model="gpt-4", tools=tools)
result = agent.run("Create issue 'Bug fix'")

# ↑ Framework automatically:
# 1. Calls LLM
# 2. Detects tool call
# 3. Executes GITHUB_CREATE_ISSUE (via on_invoke_tool)
# 4. Sends result back to LLM
# 5. Returns final response
```

---

## 🔍 Anatomia de um Provider

### Estrutura Completa

```python
class MyCustomProvider(NonAgenticProvider[MyTool, List[MyTool]], name="my_provider"):
    """
    Custom provider for MyFramework
    """
    
    # 1. REQUIRED: Convert single tool
    def wrap_tool(self, tool: Tool) -> MyTool:
        return MyTool(
            name=tool.slug,
            description=tool.description,
            schema=tool.input_parameters
        )
    
    # 2. REQUIRED: Convert multiple tools
    def wrap_tools(self, tools: Sequence[Tool]) -> List[MyTool]:
        return [self.wrap_tool(tool) for tool in tools]
    
    # 3. OPTIONAL: Helper methods
    def handle_tool_calls(self, response: MyResponse) -> List[ToolExecutionResponse]:
        """Extract and execute tool calls from framework response"""
        results = []
        for call in response.tool_calls:
            result = self.execute_tool(  # ← Injected by SDK
                slug=call.name,
                arguments=call.arguments,
                user_id=self.user_id
            )
            results.append(result)
        return results
    
    # 4. OPTIONAL: Framework-specific optimizations
    def optimize_schema(self, schema: dict) -> dict:
        """Optimize schema for framework requirements"""
        # Remove unsupported fields
        # Add framework-specific fields
        return optimized_schema
```

---

## 📊 Matriz de Compatibilidade

| Framework | Provider Type | Auto Execute | Helper Methods | Status |
|-----------|---------------|--------------|----------------|--------|
| OpenAI Chat | NonAgentic | ❌ | `handle_tool_calls` | ✅ Stable |
| OpenAI Responses | NonAgentic | ❌ | `handle_tool_calls` | ✅ Stable |
| OpenAI Agents | Agentic | ✅ | - | ✅ Stable |
| Anthropic | NonAgentic | ❌ | `handle_tool_calls` | ✅ Stable |
| Google Gemini | NonAgentic | ❌ | `handle_tool_calls` | ✅ Stable |
| LangChain | Agentic | ✅ | - | ✅ Stable |
| CrewAI | Agentic | ✅ | - | ✅ Stable |
| AutoGen | Agentic | ✅ | - | ✅ Stable |
| LlamaIndex | Agentic | ✅ | - | ✅ Stable |

---

## 🎯 Decisão: Qual Provider Usar?

```
┌─────────────────────────────────────────┐
│ Você quer controlar o loop de execução?│
└────────────┬────────────────────────────┘
             │
      ┌──────┴──────┐
      │             │
     SIM           NÃO
      │             │
      ▼             ▼
┌─────────────┐  ┌──────────────┐
│ NonAgentic  │  │  Agentic     │
│             │  │              │
│ - OpenAI    │  │ - OpenAI     │
│   Chat      │  │   Agents     │
│ - Anthropic │  │ - LangChain  │
│ - Gemini    │  │ - CrewAI     │
│             │  │ - AutoGen    │
│             │  │              │
│ Você:       │  │ Framework:   │
│ • Chama LLM │  │ • Chama LLM  │
│ • Executa   │  │ • Executa    │
│ • Controla  │  │ • Controla   │
└─────────────┘  └──────────────┘
```

---

## 💡 Dicas de Implementação

### 1. Começar Simples

```rust
// Implementação mínima
pub trait Provider {
    type Tool;
    type ToolCollection;
    
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool;
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection;
}
```

### 2. Adicionar Helpers Gradualmente

```rust
// Adicionar métodos opcionais conforme necessário
pub trait ProviderHelpers: Provider {
    fn handle_tool_calls(&self, response: Response) -> Vec<ToolExecutionResponse> {
        // Default implementation
        vec![]
    }
}
```

### 3. Usar Associated Types

```rust
// Melhor que generics para este caso
pub trait Provider {
    type Tool: Serialize;  // ← Associated type
    type ToolCollection: IntoIterator<Item = Self::Tool>;
    
    // Métodos usam Self::Tool automaticamente
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool;
}
```

---

## 🚀 Próximos Passos

1. **Implementar trait base** em Rust
2. **Criar OpenAIProvider** como referência
3. **Testar com cliente genérico**
4. **Adicionar mais providers** conforme demanda
5. **Documentar padrões** para contribuidores

O Provider System é a fundação para um SDK verdadeiramente universal! 🎉
