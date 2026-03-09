# Provider System - Explicação Completa

## 🎯 O que é o Provider System?

O Provider System é uma **camada de abstração** que permite ao Composio SDK trabalhar com **diferentes frameworks de IA** (OpenAI, Anthropic, LangChain, etc.) sem precisar reescrever código para cada um.

Pense nele como um **tradutor universal** que converte o formato padrão do Composio para o formato específico de cada framework.

---

## 🏗️ Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    Composio SDK Core                         │
│  (Gerencia ferramentas, autenticação, execução)             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Tool (formato Composio)
                     │ {
                     │   slug: "GITHUB_CREATE_ISSUE",
                     │   description: "...",
                     │   input_parameters: {...}
                     │ }
                     │
        ┌────────────┴────────────┐
        │   BaseProvider          │
        │   (Interface abstrata)  │
        │                         │
        │   wrap_tool()           │ ← Converte Tool → TTool
        │   wrap_tools()          │ ← Converte [Tool] → TToolCollection
        └────────────┬────────────┘
                     │
        ┌────────────┴────────────────────────┐
        │                                     │
┌───────▼──────────┐              ┌──────────▼─────────┐
│ NonAgenticProvider│              │ AgenticProvider    │
│ (Sem execução    │              │ (Com execução      │
│  automática)     │              │  automática)       │
└───────┬──────────┘              └──────────┬─────────┘
        │                                    │
   ┌────┴────┬────────┐              ┌──────┴──────┐
   │         │        │              │             │
┌──▼──┐  ┌──▼──┐  ┌──▼──┐      ┌───▼────┐   ┌───▼────┐
│OpenAI│  │Anthro│  │Google│    │LangChain│   │CrewAI  │
│      │  │pic   │  │      │    │         │   │        │
└──────┘  └──────┘  └──────┘    └─────────┘   └────────┘
```

---

## 📦 Tipos de Providers

### 1. **NonAgenticProvider** (Não-Agêntico)

**Uso:** Frameworks onde você controla manualmente a execução das ferramentas.

**Características:**
- Você recebe as ferramentas formatadas
- Você passa para o LLM
- Você recebe a resposta do LLM
- **Você executa manualmente** as ferramentas chamadas
- Você retorna os resultados para o LLM

**Exemplo: OpenAI Chat Completions**

```python
# 1. Obter ferramentas formatadas para OpenAI
tools = composio.tools.get(
    user_id="user_123",
    toolkits=["github"]
)
# tools = [ChatCompletionToolParam(...), ...]

# 2. Passar para OpenAI
response = openai.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Create an issue"}],
    tools=tools  # ← Formato OpenAI
)

# 3. Executar manualmente as ferramentas chamadas
if response.choices[0].message.tool_calls:
    for tool_call in response.choices[0].message.tool_calls:
        result = composio.tools.execute(
            slug=tool_call.function.name,
            arguments=json.loads(tool_call.function.arguments),
            user_id="user_123"
        )
```

**Providers NonAgentic:**
- `OpenAIProvider` - OpenAI Chat Completions
- `OpenAIResponsesProvider` - OpenAI Responses API
- `AnthropicProvider` - Anthropic Claude
- `GoogleProvider` - Google Gemini

---

### 2. **AgenticProvider** (Agêntico)

**Uso:** Frameworks que executam ferramentas automaticamente (agentic frameworks).

**Características:**
- Você passa as ferramentas para o framework
- O framework gerencia todo o loop:
  - Chama o LLM
  - Detecta tool calls
  - **Executa automaticamente** as ferramentas
  - Retorna resultados ao LLM
  - Repete até completar a tarefa

**Exemplo: OpenAI Agents SDK**

```python
# 1. Obter ferramentas formatadas (com função de execução embutida)
tools = composio.tools.get(
    user_id="user_123",
    toolkits=["github"]
)
# tools = [FunctionTool(...), ...]  ← Já tem execute embutido!

# 2. Passar para o Agent
agent = Agent(
    model="gpt-4",
    tools=tools  # ← Framework executa automaticamente
)

# 3. Executar - framework cuida de tudo
result = agent.run("Create an issue in composio/composio")
# ↑ Framework chama LLM → detecta tool call → executa → retorna → repete
```

**Providers Agentic:**
- `OpenAIAgentsProvider` - OpenAI Agents SDK
- `LangChainProvider` - LangChain
- `CrewAIProvider` - CrewAI
- `AutoGenProvider` - AutoGen

---

## 🔧 Como Funciona Internamente

### Fluxo NonAgenticProvider (OpenAI)

```python
# 1. Tool no formato Composio (universal)
composio_tool = {
    "slug": "GITHUB_CREATE_ISSUE",
    "description": "Create a new issue in a GitHub repository",
    "input_parameters": {
        "type": "object",
        "properties": {
            "owner": {"type": "string"},
            "repo": {"type": "string"},
            "title": {"type": "string"}
        }
    }
}

# 2. Provider converte para formato OpenAI
class OpenAIProvider(NonAgenticProvider):
    def wrap_tool(self, tool: Tool) -> ChatCompletionToolParam:
        return ChatCompletionToolParam(
            function=FunctionDefinition(
                name=tool.slug,  # ← GITHUB_CREATE_ISSUE
                description=tool.description,
                parameters=tool.input_parameters  # ← Schema JSON
            ),
            type="function"
        )

# 3. Resultado: formato OpenAI
openai_tool = {
    "type": "function",
    "function": {
        "name": "GITHUB_CREATE_ISSUE",
        "description": "Create a new issue...",
        "parameters": {
            "type": "object",
            "properties": {...}
        }
    }
}
```

### Fluxo AgenticProvider (OpenAI Agents)

```python
# 1. Tool no formato Composio (universal)
composio_tool = {...}  # Mesmo de cima

# 2. Provider converte + injeta função de execução
class OpenAIAgentsProvider(AgenticProvider):
    def wrap_tool(self, tool: Tool, execute_tool: Callable) -> FunctionTool:
        async def execute_wrapper(_ctx, payload):
            # Executa a ferramenta via Composio
            return await execute_tool(
                slug=tool.slug,
                arguments=json.loads(payload)
            )
        
        return FunctionTool(
            name=tool.slug,
            description=tool.description,
            params_json_schema=tool.input_parameters,
            on_invoke_tool=execute_wrapper  # ← Função de execução!
        )

# 3. Resultado: FunctionTool com execução embutida
agents_tool = FunctionTool(
    name="GITHUB_CREATE_ISSUE",
    on_invoke_tool=<function>  # ← Framework chama isso automaticamente
)
```

---

## 🎨 Generics e Type Safety

O Provider System usa **generics** para garantir type safety:

```python
TTool = TypeVar("TTool")  # Tipo individual da ferramenta
TToolCollection = TypeVar("TToolCollection")  # Coleção de ferramentas

class BaseProvider(Generic[TTool, TToolCollection]):
    def wrap_tool(self, tool: Tool) -> TTool:
        ...
    
    def wrap_tools(self, tools: Sequence[Tool]) -> TToolCollection:
        ...

# OpenAI Provider
class OpenAIProvider(
    NonAgenticProvider[
        ChatCompletionToolParam,  # ← TTool
        List[ChatCompletionToolParam]  # ← TToolCollection
    ]
):
    ...

# Composio SDK usa os tipos do provider
class Composio(Generic[TTool, TToolCollection]):
    def __init__(self, provider: BaseProvider[TTool, TToolCollection]):
        self.provider = provider
    
    def get_tools(self, ...) -> TToolCollection:
        # Retorna tipo correto baseado no provider!
        return self.provider.wrap_tools(...)
```

**Benefícios:**
- IDE autocomplete correto
- Type checking em compile-time
- Documentação automática
- Menos erros em runtime

---

## 🔄 Métodos Helper do Provider

Além de `wrap_tool` e `wrap_tools`, providers podem ter métodos helper:

### OpenAI Provider Helpers

```python
class OpenAIProvider:
    def handle_tool_calls(
        self,
        user_id: str,
        response: ChatCompletion,
    ) -> List[ToolExecutionResponse]:
        """
        Extrai tool calls da resposta OpenAI e executa automaticamente.
        """
        outputs = []
        for choice in response.choices:
            for tool_call in choice.message.tool_calls:
                result = self.execute_tool(
                    slug=tool_call.function.name,
                    arguments=json.loads(tool_call.function.arguments),
                    user_id=user_id
                )
                outputs.append(result)
        return outputs
```

**Uso:**
```python
# Sem helper (manual)
for tool_call in response.choices[0].message.tool_calls:
    result = composio.tools.execute(...)

# Com helper (automático)
results = composio.provider.handle_tool_calls(user_id, response)
```

---

## 🚀 Por que isso é importante?

### 1. **Flexibilidade**
Suporta qualquer framework de IA sem modificar o core do SDK.

### 2. **Manutenibilidade**
Adicionar suporte a um novo framework = criar um novo provider (isolado).

### 3. **Type Safety**
Tipos corretos para cada framework, detectados em compile-time.

### 4. **Experiência do Desenvolvedor**
```python
# Mesmo código, diferentes frameworks
composio = Composio(provider=OpenAIProvider())
composio = Composio(provider=AnthropicProvider())
composio = Composio(provider=LangChainProvider())
```

### 5. **Otimizações Específicas**
Cada provider pode otimizar para seu framework:
- OpenAI: `strict=True` para schemas
- Anthropic: Formato de mensagens específico
- LangChain: Integração com chains

---

## 📊 Comparação: Com vs Sem Provider System

### ❌ Sem Provider System

```python
# Código diferente para cada framework
if framework == "openai":
    tools = format_for_openai(composio_tools)
elif framework == "anthropic":
    tools = format_for_anthropic(composio_tools)
elif framework == "langchain":
    tools = format_for_langchain(composio_tools)
# ... 10+ frameworks = 10+ if/else
```

### ✅ Com Provider System

```python
# Código único, provider cuida da conversão
tools = composio.tools.get(user_id="user_123", toolkits=["github"])
# ↑ Retorna formato correto baseado no provider configurado
```

---

## 🎯 Implementação em Rust

Para implementar em Rust, usaríamos **traits** (equivalente a interfaces):

```rust
// Trait base (equivalente a BaseProvider)
pub trait Provider: Send + Sync {
    type Tool: Serialize + DeserializeOwned;
    type ToolCollection: IntoIterator<Item = Self::Tool>;
    
    fn name(&self) -> &str;
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool;
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection;
}

// Provider OpenAI
pub struct OpenAIProvider;

impl Provider for OpenAIProvider {
    type Tool = ChatCompletionToolParam;
    type ToolCollection = Vec<ChatCompletionToolParam>;
    
    fn name(&self) -> &str { "openai" }
    
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
        ChatCompletionToolParam {
            r#type: "function".to_string(),
            function: FunctionDefinition {
                name: tool.slug.clone(),
                description: tool.description.clone(),
                parameters: tool.input_parameters.clone(),
            }
        }
    }
    
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection {
        tools.iter().map(|t| self.wrap_tool(t)).collect()
    }
}

// Cliente genérico
pub struct ComposioClient<P: Provider = OpenAIProvider> {
    provider: P,
    // ...
}

// Uso
let client = ComposioClient::with_provider(OpenAIProvider);
let tools = client.get_tools("user_123", vec!["github"]).await?;
// ↑ tools: Vec<ChatCompletionToolParam> (tipo inferido!)
```

---

## 📚 Resumo

| Aspecto | Descrição |
|---------|-----------|
| **Propósito** | Adaptar ferramentas Composio para diferentes frameworks AI |
| **Tipos** | NonAgentic (manual) e Agentic (automático) |
| **Benefícios** | Flexibilidade, type safety, manutenibilidade |
| **Implementação** | Generics + herança (Python) / Traits + associated types (Rust) |
| **Frameworks** | OpenAI, Anthropic, LangChain, CrewAI, AutoGen, etc. |

O Provider System é o que torna o Composio SDK **universal** - funciona com qualquer framework de IA sem precisar reescrever código!
