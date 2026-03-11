# Python SDK Reference (/reference/sdk-reference/python)

# Python SDK Reference

Complete API reference for the `composio` Python package.

# Installation

```bash
pip install composio
```

Or with uv:

```bash
uv add composio
```

# Classes

| Class                                                                     | Description                                                                         |
| ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| [`Composio`](/reference/sdk-reference/python/composio)                    | Composio SDK for Python.  Generic parameters: TTool: The individual tool type re... |
| [`Tools`](/reference/sdk-reference/python/tools)                          | Tools class definition  This class is used to manage tools in the Composio SDK. ... |
| [`Toolkits`](/reference/sdk-reference/python/toolkits)                    | Toolkits are a collectiono of tools that can be used to perform various tasks. T... |
| [`Triggers`](/reference/sdk-reference/python/triggers)                    | Triggers (instance) class                                                           |
| [`ConnectedAccounts`](/reference/sdk-reference/python/connected-accounts) | Manage connected accounts.  This class is used to manage connected accounts in t... |
| [`AuthConfigs`](/reference/sdk-reference/python/auth-configs)             | Manage authentication configurations.                                               |
| [`MCP`](/reference/sdk-reference/python/mcp)                              | MCP (Model Control Protocol) class. Provides enhanced MCP server operations  Thi... |

# Quick Start

```python
from composio import Composio

composio = Composio(api_key="your-api-key")

# Get tools for a user
tools = composio.tools.get("user-123", toolkits=["github"])

# Execute a tool
result = composio.tools.execute(
    "GITHUB_GET_REPOS",
    arguments={"owner": "composio"},
    user_id="user-123"
)
```

# Decorators

## before\_execute

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/_modifiers.py#L183)

```python
@before_execute(modifier: BeforeExecute | None = ..., tools: List[str | None] = ..., toolkits: List[str | None] = ...)
def my_modifier(...):
    ...
```

## after\_execute

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/_modifiers.py#L144)

```python
@after_execute(modifier: AfterExecute | None = ..., tools: List[str | None] = ..., toolkits: List[str | None] = ...)
def my_modifier(...):
    ...
```

## schema\_modifier

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/_modifiers.py#L222)

```python
@schema_modifier(modifier: SchemaModifier | None = ..., tools: List[str | None] = ..., toolkits: List[str | None] = ...)
def my_modifier(...):
    ...
```

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# Composio (/reference/sdk-reference/python/composio)

# Properties

| Name                                                                       | Type                |
| -------------------------------------------------------------------------- | ------------------- |
| [`tools`](/reference/sdk-reference/python/tools)                           | `Tools`             |
| [`toolkits`](/reference/sdk-reference/python/toolkits)                     | `Toolkits`          |
| [`triggers`](/reference/sdk-reference/python/triggers)                     | `Triggers`          |
| [`auth_configs`](/reference/sdk-reference/python/auth-configs)             | `AuthConfigs`       |
| [`connected_accounts`](/reference/sdk-reference/python/connected-accounts) | `ConnectedAccounts` |
| [`mcp`](/reference/sdk-reference/python/mcp)                               | `MCP`               |

[View source](https://github.com/composiohq/composio/blob/next/python/composio/sdk.py#L46)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# Tools (/reference/sdk-reference/python/tools)

# Methods

## get\_raw\_composio\_tool\_by\_slug()

Returns schema for the given tool slug.

```python
def get_raw_composio_tool_by_slug(slug: str) -> Tool
```

**Parameters**

| Name   | Type  |
| ------ | ----- |
| `slug` | `str` |

**Returns**

`Tool`

***

## get\_raw\_composio\_tools()

Get a list of tool schemas based on the provided filters.

```python
def get_raw_composio_tools(tools: list[str | None] = ..., search: str | None = ..., toolkits: list[str | None] = ..., scopes: List[str | None] = ..., limit: int | None = ...) -> list[Tool]
```

**Parameters**

| Name        | Type                |
| ----------- | ------------------- |
| `tools?`    | `list[str \| None]` |
| `search?`   | `str \| None`       |
| `toolkits?` | `list[str \| None]` |
| `scopes?`   | `List[str \| None]` |
| `limit?`    | `int \| None`       |

**Returns**

`list[Tool]`

***

## get\_raw\_tool\_router\_meta\_tools()

Fetches the meta tools for a tool router session.  This method fetches the meta tools from the Composio API and transforms them to the expected format. It provides access to the underlying meta tool data without provider-specific wrapping.

```python
def get_raw_tool_router_meta_tools(session_id: str, modifiers: 'Modifiers' | None = ...) -> list[Tool]
```

**Parameters**

| Name         | Type                  |
| ------------ | --------------------- |
| `session_id` | `str`                 |
| `modifiers?` | `'Modifiers' \| None` |

**Returns**

`list[Tool]` — The list of meta tools

**Example**

````python
```python
from composio import Composio

composio = Composio()
tools_model = composio.tools

# Get meta tools for a session
meta_tools = tools_model.get_raw_tool_router_meta_tools("session_123")
print(meta_tools)

# Get meta tools with schema modifiers
from composio.core.models import schema_modifier

@schema_modifier
def modify_schema(tool: str, toolkit: str, schema):
# Customize the schema
schema.description = f"Modified: {schema.description}"
return schema

meta_tools = tools_model.get_raw_tool_router_meta_tools(
"session_123",
modifiers=[modify_schema]
)
````
````

--------- | ------------------- |
| `user_id`    | `str`               |
| `slug?`      | `str \| None`       |
| `tools?`     | `list[str \| None]` |
| `search?`    | `str \| None`       |
| `toolkits?`  | `list[str \| None]` |
| `scopes?`    | `List[str \| None]` |
| `modifiers?` | `Modifiers \| None` |
| `limit?`     | `int \| None`       |

**Returns**

`TToolCollection` — Provider-specific tool collection (TToolCollection).

***

## execute()

Execute a tool with the provided parameters.  This method calls the Composio API or a custom tool handler to execute the tool and returns the response. It automatically determines whether to use a custom tool or a Composio API tool based on the slug.

```python
def execute(slug: str, arguments: Dict, connected_account_id: str | None = ..., custom_auth_params: tool_execute_params.CustomAuthParams | None = ..., custom_connection_data: tool_execute_params.CustomConnectionData | None = ..., user_id: str | None = ..., text: str | None = ..., version: str | None = ..., dangerously_skip_version_check: bool | None = ..., modifiers: Modifiers | None = ...) -> ToolExecutionResponse
```

**Parameters**

| Name                              | Type                                               |
| --------------------------------- | -------------------------------------------------- |
| `slug`                            | `str`                                              |
| `arguments`                       | `Dict`                                             |
| `connected_account_id?`           | `str \| None`                                      |
| `custom_auth_params?`             | `tool_execute_params.CustomAuthParams \| None`     |
| `custom_connection_data?`         | `tool_execute_params.CustomConnectionData \| None` |
| `user_id?`                        | `str \| None`                                      |
| `text?`                           | `str \| None`                                      |
| `version?`                        | `str \| None`                                      |
| `dangerously_skip_version_check?` | `bool \| None`                                     |
| `modifiers?`                      | `Modifiers \| None`                                |

**Returns**

`ToolExecutionResponse` — The response from the tool.

***

## proxy()

Proxy a tool call to the Composio API

```python
def proxy(endpoint: str, method: Literal['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD'], body: object | None = ..., connected_account_id: str | None = ..., parameters: List[tool_proxy_params.Parameter | None] = ..., custom_connection_data: tool_proxy_params.CustomConnectionData | None = ...) -> tool_proxy_response.ToolProxyResponse
```

**Parameters**

| Name                      | Type                                                       |
| ------------------------- | ---------------------------------------------------------- |
| `endpoint`                | `str`                                                      |
| `method`                  | `Literal['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD']` |
| `body?`                   | `object \| None`                                           |
| `connected_account_id?`   | `str \| None`                                              |
| `parameters?`             | `List[tool_proxy_params.Parameter \| None]`                |
| `custom_connection_data?` | `tool_proxy_params.CustomConnectionData \| None`           |

**Returns**

`tool_proxy_response.ToolProxyResponse`

***

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/tools.py#L80)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# Toolkits (/reference/sdk-reference/python/toolkits)

# Methods

## list()

List all toolkits.

```python
def list(category: str | None = ..., cursor: str | None = ..., limit: float | None = ..., sort_by: Literal['usage', 'alphabetically' | None] = ..., managed_by: Literal['composio', 'all', 'project' | None] = ...) -> toolkit_list_response.ToolkitListResponse
```

**Parameters**

| Name          | Type                                            |
| ------------- | ----------------------------------------------- |
| `category?`   | `str \| None`                                   |
| `cursor?`     | `str \| None`                                   |
| `limit?`      | `float \| None`                                 |
| `sort_by?`    | `Literal['usage', 'alphabetically' \| None]`    |
| `managed_by?` | `Literal['composio', 'all', 'project' \| None]` |

**Returns**

`toolkit_list_response.ToolkitListResponse`

***

## get()

```python
def get(slug: str | None = ..., query: toolkit_list_params.ToolkitListParams | None = ...) -> Union[toolkit_retrieve_response.ToolkitRetrieveResponse, ...
```

**Parameters**

| Name     | Type                                            |
| -------- | ----------------------------------------------- |
| `slug?`  | `str \| None`                                   |
| `query?` | `toolkit_list_params.ToolkitListParams \| None` |

**Returns**

`Union[toolkit_retrieve_response.ToolkitRetrieveResponse, ...`

***

## list\_categories()

List all categories of toolkits.

```python
def list_categories()
```

***

## authorize()

Authorize a user to a toolkit  If auth config is not found, it will be created using composio managed auth.

```python
def authorize(user_id: str, toolkit: str)
```

**Parameters**

| Name      | Type  |
| --------- | ----- |
| `user_id` | `str` |
| `toolkit` | `str` |

***

## get\_connected\_account\_initiation\_fields()

Get the required property for a given toolkit and auth scheme.

```python
def get_connected_account_initiation_fields(toolkit: str, auth_scheme: AuthSchemeL, required_only: bool = ...) -> AuthFieldsT
```

**Parameters**

| Name             | Type          |
| ---------------- | ------------- |
| `toolkit`        | `str`         |
| `auth_scheme`    | `AuthSchemeL` |
| `required_only?` | `bool`        |

**Returns**

`AuthFieldsT`

***

## get\_auth\_config\_creation\_fields()

Get the required property for a given toolkit and auth scheme.

```python
def get_auth_config_creation_fields(toolkit: str, auth_scheme: AuthSchemeL, required_only: bool = ...) -> AuthFieldsT
```

**Parameters**

| Name             | Type          |
| ---------------- | ------------- |
| `toolkit`        | `str`         |
| `auth_scheme`    | `AuthSchemeL` |
| `required_only?` | `bool`        |

**Returns**

`AuthFieldsT`

***

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/toolkits.py#L26)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# Triggers (/reference/sdk-reference/python/triggers)

# Methods

## get\_type()

Get a trigger type by its slug Uses the global toolkit version provided when initializing composio instance to fetch trigger for specific toolkit version

```python
def get_type(slug: str) -> TriggersTypeRetrieveResponse
```

**Parameters**

| Name   | Type  |
| ------ | ----- |
| `slug` | `str` |

**Returns**

`TriggersTypeRetrieveResponse` — The trigger type

***

## list\_active()

List all active triggers

```python
def list_active(trigger_ids: list[str | None] = ..., trigger_names: list[str | None] = ..., auth_config_ids: list[str | None] = ..., connected_account_ids: list[str | None] = ..., show_disabled: bool | None = ..., limit: int | None = ..., cursor: str | None = ...)
```

**Parameters**

| Name                     | Type                |
| ------------------------ | ------------------- |
| `trigger_ids?`           | `list[str \| None]` |
| `trigger_names?`         | `list[str \| None]` |
| `auth_config_ids?`       | `list[str \| None]` |
| `connected_account_ids?` | `list[str \| None]` |
| `show_disabled?`         | `bool \| None`      |
| `limit?`                 | `int \| None`       |
| `cursor?`                | `str \| None`       |

***

## list()

List all the trigger types.

```python
def list(cursor: str | None = ..., limit: int | None = ..., toolkit_slugs: list[str | None] = ...)
```

**Parameters**

| Name             | Type                |
| ---------------- | ------------------- |
| `cursor?`        | `str \| None`       |
| `limit?`         | `int \| None`       |
| `toolkit_slugs?` | `list[str \| None]` |

***

## create()

Create a trigger instance

```python
def create(slug: str, user_id: str | None = ..., connected_account_id: str | None = ..., trigger_config: Dict[str, Any | None] = ...) -> trigger_instance_upsert_response.TriggerInstanceUpsertRes...
```

**Parameters**

| Name                    | Type                     |
| ----------------------- | ------------------------ |
| `slug`                  | `str`                    |
| `user_id?`              | `str \| None`            |
| `connected_account_id?` | `str \| None`            |
| `trigger_config?`       | `Dict[str, Any \| None]` |

**Returns**

`trigger_instance_upsert_response.TriggerInstanceUpsertRes...` — The trigger instance

***

## subscribe()

Subscribe to a trigger and receive trigger events.

```python
def subscribe(timeout: float = ...) -> TriggerSubscription
```

**Parameters**

| Name       | Type    |
| ---------- | ------- |
| `timeout?` | `float` |

**Returns**

`TriggerSubscription` — The trigger subscription handler.

***

## verify\_webhook()

Verify an incoming webhook payload and signature.  This method validates that the webhook request is authentic by: 1. Validating the webhook timestamp is within the tolerance window 2. Verifying the HMAC-SHA256 signature using the correct algorithm 3. Parsing the payload and detecting the webhook version (V1, V2, or V3)

```python
def verify_webhook(id: str, payload: str, secret: str, signature: str, timestamp: str, tolerance: int = ...) -> VerifyWebhookResult
```

**Parameters**

| Name         | Type  |
| ------------ | ----- |
| `id`         | `str` |
| `payload`    | `str` |
| `secret`     | `str` |
| `signature`  | `str` |
| `timestamp`  | `str` |
| `tolerance?` | `int` |

**Returns**

`VerifyWebhookResult` — VerifyWebhookResult containing version, normalized payload, and raw payload :raises WebhookSignatureVerificationError: If the signature verification fails :raises WebhookPayloadError: If the payload cannot be parsed or is invalid

**Example**

```python
# In a Flask webhook handler
    @app.route('/webhook', methods=['POST'])
    def webhook():
        try:
            result = composio.triggers.verify_webhook(
                id=request.headers.get('webhook-id', ''),
                payload=request.get_data(as_text=True),
                signature=request.headers.get('webhook-signature', ''),
                timestamp=request.headers.get('webhook-timestamp', ''),
                secret=os.environ['COMPOSIO_WEBHOOK_SECRET'],
            )

            # Process the verified payload
            print(f"Version: {result['version']}")
            print(f"Received trigger: {result['payload']['trigger_slug']}")
            return 'OK', 200
        except WebhookSignatureVerificationError:
            return 'Unauthorized', 401
```

***

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/triggers.py#L701)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# ConnectedAccounts (/reference/sdk-reference/python/connected-accounts)

# Methods

## initiate()

Compound function to create a new connected account. This function creates a new connected account and returns a connection request.  Users can then wait for the connection to be established using the `wait_for_connection` method.

```python
def initiate(user_id: str, auth_config_id: str, callback_url: str | None = ..., allow_multiple: bool = ..., config: connected_account_create_params.ConnectionState | None = ...) -> ConnectionRequest
```

**Parameters**

| Name              | Type                                                      |
| ----------------- | --------------------------------------------------------- |
| `user_id`         | `str`                                                     |
| `auth_config_id`  | `str`                                                     |
| `callback_url?`   | `str \| None`                                             |
| `allow_multiple?` | `bool`                                                    |
| `config?`         | `connected_account_create_params.ConnectionState \| None` |

**Returns**

`ConnectionRequest` — The connection request.

***

## link()

Create a Composio Connect Link for a user to connect their account to a given auth config.  This method will return an external link which you can use for the user to connect their account.

```python
def link(user_id: str, auth_config_id: str, callback_url: str | None = ...) -> ConnectionRequest
```

**Parameters**

| Name             | Type          |
| ---------------- | ------------- |
| `user_id`        | `str`         |
| `auth_config_id` | `str`         |
| `callback_url?`  | `str \| None` |

**Returns**

`ConnectionRequest` — Connection request object.

**Example**

```python
# Create a connection request and redirect the user to the redirect url
    connection_request = composio.connected_accounts.link('user_123', 'auth_config_123')
    redirect_url = connection_request.redirect_url
    print(f"Visit: {redirect_url} to authenticate your account")

    # Wait for the connection to be established
    connected_account = connection_request.wait_for_connection()

    # Create a connection request with callback URL
    connection_request = composio.connected_accounts.link(
        'user_123',
        'auth_config_123',
        callback_url='https://your-app.com/callback'
    )
    redirect_url = connection_request.redirect_url
    print(f"Visit: {redirect_url} to authenticate your account")

    # Wait for the connection to be established
    connected_account = composio.connected_accounts.wait_for_connection(connection_request.id)
```

***

## wait\_for\_connection()

Wait for connected account with given ID to be active

```python
def wait_for_connection(id: str, timeout: float | None = ...) -> connected_account_retrieve_response.ConnectedAccountRetri...
```

**Parameters**

| Name       | Type            |
| ---------- | --------------- |
| `id`       | `str`           |
| `timeout?` | `float \| None` |

**Returns**

`connected_account_retrieve_response.ConnectedAccountRetri...`

***

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/connected_accounts.py#L300)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# AuthConfigs (/reference/sdk-reference/python/auth-configs)

# Methods

## list()

Lists authentication configurations based on provided filter criteria.

```python
def list(query: auth_config_list_params.AuthConfigListParams = ...) -> auth_config_list_response.AuthConfigListResponse
```

**Parameters**

| Name     | Type                                           |
| -------- | ---------------------------------------------- |
| `query?` | `auth_config_list_params.AuthConfigListParams` |

**Returns**

`auth_config_list_response.AuthConfigListResponse`

***

## create()

Create a new auth config

```python
def create(toolkit: str, options: auth_config_create_params.AuthConfig) -> auth_config_create_response.AuthConfig
```

**Parameters**

| Name      | Type                                   |
| --------- | -------------------------------------- |
| `toolkit` | `str`                                  |
| `options` | `auth_config_create_params.AuthConfig` |

**Returns**

`auth_config_create_response.AuthConfig` — The created auth config.

***

## get()

Retrieves a specific authentication configuration by its ID

```python
def get(nanoid: str) -> auth_config_retrieve_response.AuthConfigRetrieveResponse
```

**Parameters**

| Name     | Type  |
| -------- | ----- |
| `nanoid` | `str` |

**Returns**

`auth_config_retrieve_response.AuthConfigRetrieveResponse` — The retrieved auth config.

***

## update()

Updates an existing authentication configuration.  This method allows you to modify properties of an auth config such as credentials, scopes, or tool restrictions. The update type (custom or default) determines which fields can be updated.

```python
def update(nanoid: str, options: auth_config_update_params.AuthConfigUpdateParams) -> Dict
```

**Parameters**

| Name      | Type                                               |
| --------- | -------------------------------------------------- |
| `nanoid`  | `str`                                              |
| `options` | `auth_config_update_params.AuthConfigUpdateParams` |

**Returns**

`Dict` — The updated auth config.

***

## delete()

Deletes an existing authentication configuration.

```python
def delete(nanoid: str) -> Dict
```

**Parameters**

| Name     | Type  |
| -------- | ----- |
| `nanoid` | `str` |

**Returns**

`Dict` — The deleted auth config.

***

## enable()

Enables an existing authentication configuration.

```python
def enable(nanoid: str) -> Dict
```

**Parameters**

| Name     | Type  |
| -------- | ----- |
| `nanoid` | `str` |

**Returns**

`Dict` — The enabled auth config.

***

## disable()

Disables an existing authentication configuration.

```python
def disable(nanoid: str) -> Dict
```

**Parameters**

| Name     | Type  |
| -------- | ----- |
| `nanoid` | `str` |

**Returns**

`Dict` — The disabled auth config.

***

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/auth_configs.py#L18)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
# MCP (/reference/sdk-reference/python/mcp)

# Methods

## create()

Create a new MCP server configuration with specified toolkits and authentication settings.

```python
def create(name: str, toolkits: List[Union[ConfigToolkit, str]], manually_manage_connections: bool = ..., allowed_tools: List[str | None] = ...) -> MCPCreateResponse
```

**Parameters**

| Name                           | Type                              |
| ------------------------------ | --------------------------------- |
| `name`                         | `str`                             |
| `toolkits`                     | `List[Union[ConfigToolkit, str]]` |
| `manually_manage_connections?` | `bool`                            |
| `allowed_tools?`               | `List[str \| None]`               |

**Returns**

`MCPCreateResponse` — Created server details with generate method

**Example**

```python
>>> # Using toolkit configuration objects with auth
    >>> server = composio.experimental.mcp.create(
    ...     'personal-mcp-server',
    ...     toolkits=[
    ...         {
    ...             'toolkit': 'github',
    ...             'auth_config_id': 'ac_xyz',
    ...         },
    ...         {
    ...             'toolkit': 'slack',
    ...             'auth_config_id': 'ac_abc',
    ...         },
    ...     ],
    ...     allowed_tools=['GITHUB_CREATE_ISSUE', 'GITHUB_LIST_REPOS', 'SLACK_SEND_MESSAGE'],
    ...     manually_manage_connections=False
    ... )
    >>>
    >>> # Using simple toolkit names (most common usage)
    >>> server = composio.experimental.mcp.create(
    ...     'simple-mcp-server',
    ...     toolkits=['composio_search', 'text_to_pdf'],
    ...     allowed_tools=['COMPOSIO_SEARCH_DUCK_DUCK_GO_SEARCH', 'TEXT_TO_PDF_CONVERT_TEXT_TO_PDF']
    ... )
    >>>
    >>> # Using all tools from toolkits (default behavior)
    >>> server = composio.experimental.mcp.create(
    ...     'all-tools-server',
    ...     toolkits=['composio_search', 'text_to_pdf']
    ...     # allowed_tools=None means all tools from these toolkits
    ... )
    >>>
    >>> # Get server instance for a user
    >>> mcp = server.generate('user_12345')
```

***

## list()

List MCP servers with optional filtering and pagination.

```python
def list(page_no: int | None = ..., limit: int | None = ..., toolkits: str | None = ..., auth_config_ids: str | None = ..., name: str | None = ..., order_by: Literal['created_at', 'updated_at' | None] = ..., order_direction: Literal['asc', 'desc' | None] = ...) -> MCPListResponse
```

**Parameters**

| Name               | Type                                          |
| ------------------ | --------------------------------------------- |
| `page_no?`         | `int \| None`                                 |
| `limit?`           | `int \| None`                                 |
| `toolkits?`        | `str \| None`                                 |
| `auth_config_ids?` | `str \| None`                                 |
| `name?`            | `str \| None`                                 |
| `order_by?`        | `Literal['created_at', 'updated_at' \| None]` |
| `order_direction?` | `Literal['asc', 'desc' \| None]`              |

**Returns**

`MCPListResponse` — Paginated list of MCP servers

**Example**

```python
>>> # List all servers
    >>> all_servers = composio.experimental.mcp.list()
    >>>
    >>> # List with pagination
    >>> paged_servers = composio.experimental.mcp.list(page_no=2, limit=5)
    >>>
    >>> # Filter by toolkit
    >>> github_servers = composio.experimental.mcp.list(toolkits='github', name='personal')
```

***

## get()

Retrieve detailed information about a specific MCP server/config.

```python
def get(server_id: str)
```

**Parameters**

| Name        | Type  |
| ----------- | ----- |
| `server_id` | `str` |

**Example**

```python
>>> server = composio.experimental.mcp.get('mcp_12345')
    >>>
    >>> print(server['name'])  # "My Personal MCP Server"
    >>> print(server['allowed_tools'])  # ["GITHUB_CREATE_ISSUE", "SLACK_SEND_MESSAGE"]
    >>> print(server['toolkits'])  # ["github", "slack"]
    >>> print(server['server_instance_count'])  # 3
```

***

## update()

Update an existing MCP server configuration.

```python
def update(server_id: str, name: str | None = ..., toolkits: List[Union[ConfigToolkit, str | None]] = ..., manually_manage_connections: bool | None = ..., allowed_tools: List[str | None] = ...)
```

**Parameters**

| Name                           | Type                                      |
| ------------------------------ | ----------------------------------------- |
| `server_id`                    | `str`                                     |
| `name?`                        | `str \| None`                             |
| `toolkits?`                    | `List[Union[ConfigToolkit, str \| None]]` |
| `manually_manage_connections?` | `bool \| None`                            |
| `allowed_tools?`               | `List[str \| None]`                       |

**Example**

```python
>>> # Update server name only
    >>> updated_server = composio.experimental.mcp.update(
    ...     'mcp_12345',
    ...     name='My Updated MCP Server'
    ... )
    >>>
    >>> # Update toolkits and tools
    >>> server_with_new_tools = composio.experimental.mcp.update(
    ...     'mcp_12345',
    ...     toolkits=['github', 'slack'],
    ...     allowed_tools=['GITHUB_CREATE_ISSUE', 'SLACK_SEND_MESSAGE']
    ... )
    >>>
    >>> # Update with auth configs
    >>> server_with_auth = composio.experimental.mcp.update(
    ...     'mcp_12345',
    ...     toolkits=[
    ...         {'toolkit': 'github', 'auth_config_id': 'auth_abc123'},
    ...         {'toolkit': 'slack', 'auth_config_id': 'auth_def456'}
    ...     ],
    ...     allowed_tools=['GITHUB_CREATE_ISSUE', 'SLACK_SEND_MESSAGE'],
    ...     manually_manage_connections=False
    ... )
```

***

## delete()

Permanently delete an MCP server configuration.

```python
def delete(server_id: str) -> Dict[str, Any]
```

**Parameters**

| Name        | Type  |
| ----------- | ----- |
| `server_id` | `str` |

**Returns**

`Dict[str, Any]` — Deletion result

**Example**

```python
>>> # Delete a server
    >>> result = composio.experimental.mcp.delete('mcp_12345')
    >>>
    >>> if result['deleted']:
    ...     print(f"Server {result['id']} has been successfully deleted")
    >>> else:
    ...     print(f"Failed to delete server {result['id']}")
```

***

## generate()

Get server URLs for an existing MCP server.  This matches the TypeScript implementation exactly.

```python
def generate(user_id: str, mcp_config_id: str, manually_manage_connections: bool | None = ...) -> MCPServerInstance
```

**Parameters**

| Name                           | Type           |
| ------------------------------ | -------------- |
| `user_id`                      | `str`          |
| `mcp_config_id`                | `str`          |
| `manually_manage_connections?` | `bool \| None` |

**Returns**

`MCPServerInstance` — MCP server instance

**Example**

```python
>>> mcp = composio.experimental.mcp.generate(
    ...     'user_12345',
    ...     'mcp_67890',
    ...     manually_manage_connections=False
    ... )
    >>>
    >>> print(mcp['url'])  # Server URL for the user
    >>> print(mcp['allowed_tools'])  # Available tools
```

***

[View source](https://github.com/composiohq/composio/blob/next/python/composio/core/models/mcp.py#L88)

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

---

# Composio SDK — Instructions for AI Code Generators

**Purpose:** Enforce only the **current** and **correct** patterns for integrating [Composio](https://composio.dev/) tools into AI agents.
**Scope:** All AI-generated code using Composio must follow these guardrails.

---

## 1. Correct Composio Integration

Composio supports two integration modes: **Native Tools** (with a provider package) and **MCP** (no provider package needed).

### Native Tools

```python
# ✅ CORRECT — Python (defaults to OpenAI)
from composio import Composio

composio = Composio()
session = composio.create(user_id="user_123")
tools = session.tools()
# Pass tools to your agent/LLM framework
```

```typescript
// ✅ CORRECT — TypeScript (defaults to OpenAI)
import { Composio } from "@composio/core";

const composio = new Composio();
const session = await composio.create("user_123");
const tools = await session.tools();
// Pass tools to your agent/LLM framework
```

For other providers, pass the provider explicitly. Provider packages follow the naming convention: `composio_<provider>` for Python, `@composio/<provider>` for TypeScript.

### MCP

Use `session.mcp.url` and `session.mcp.headers` with any MCP-compatible client (Claude Desktop, Cursor, OpenAI Agents, etc.). No provider package needed.

---

## 2. CRITICAL INSTRUCTIONS FOR AI MODELS

### 2.1 — ALWAYS DO THE FOLLOWING

1. **Use `composio.create(user_id)`** to create a session — this is the entry point for all Composio integrations.
2. **Use `session.tools()`** for native tool integration or **`session.mcp.url`** for MCP integration.
3. **Import** from the correct provider package (`composio_<provider>` for Python, `@composio/<provider>` for TypeScript).
4. **Pass `user_id` to `composio.create()`**, not to individual tool calls.

### 2.2 — NEVER DO THE FOLLOWING

1. **Do not** instruct users to manually create auth configs or connected accounts before running.
2. **Do not** import from `composio_openai` / `@composio/openai` when using OpenAI Agents SDK — use `composio_openai_agents` / `@composio/openai-agents` instead.

### 2.3 — DISCOURAGED (do not use unless user explicitly asks)

Do **not** generate `composio.tools.get()`, `composio.tools.execute()`, or `composio.provider.handle_tool_calls()` unless the user explicitly requests direct tool execution. These are a supported but **not recommended** low-level interface.


---

## Terminology Migration (old → current)

If you encounter these terms in error messages, old documentation, or user prompts, translate them to the current equivalents. **Do not use the old terms in generated code or explanations.**

| Old term (v1/v2) | Current term (v3) | In code |
|---|---|---|
| entity ID | user ID | `user_id` parameter |
| actions | tools | e.g., `GITHUB_CREATE_ISSUE` is a *tool* |
| apps / appType | toolkits | e.g., `github` is a *toolkit* |
| integration / integration ID | auth config / auth config ID | `auth_config_id` parameter |
| connection | connected account | `connected_accounts` namespace |
| ComposioToolSet / OpenAIToolSet | `Composio` class with a provider | `Composio(provider=...)` |
| toolset | provider | e.g., `OpenAIProvider` |

If a user says "entity ID", they mean `user_id`. If they say "integration", they mean "auth config". Always respond using the current terminology.
