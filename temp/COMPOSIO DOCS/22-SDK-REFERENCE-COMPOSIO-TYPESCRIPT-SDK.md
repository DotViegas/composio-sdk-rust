# TypeScript SDK Reference (/reference/sdk-reference/typescript)

# Installation

**npm:**

```bash
npm install @composio/core
```

**pnpm:**

```bash
pnpm add @composio/core
```

**yarn:**

```bash
yarn add @composio/core
```

**bun:**

```bash
bun add @composio/core
```

# Classes

| Class                                                                         | Description                                             |
| ----------------------------------------------------------------------------- | ------------------------------------------------------- |
| [`Composio`](/reference/sdk-reference/typescript/composio)                    | This is the core class for Composio.                    |
| [`AuthConfigs`](/reference/sdk-reference/typescript/auth-configs)             | AuthConfigs class                                       |
| [`ConnectedAccounts`](/reference/sdk-reference/typescript/connected-accounts) | ConnectedAccounts class                                 |
| [`MCP`](/reference/sdk-reference/typescript/mcp)                              | MCP (Model Control Protocol) class                      |
| [`Toolkits`](/reference/sdk-reference/typescript/toolkits)                    | Toolkits class                                          |
| [`Tools`](/reference/sdk-reference/typescript/tools)                          | This class is used to manage tools in the Composio SDK. |
| [`Triggers`](/reference/sdk-reference/typescript/triggers)                    | Trigger (Instance) class                                |

# Quick Start

```typescript
import { Composio } from '@composio/core';

const composio = new Composio({
  apiKey: process.env.COMPOSIO_API_KEY
});

// Get tools for a user
const tools = await composio.tools.get('user-123', {
  toolkits: ['github']
});

// Execute a tool
const result = await composio.tools.execute('GITHUB_GET_REPOS', {
  userId: 'user-123',
  arguments: { owner: 'composio' }
});
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
# Composio (/reference/sdk-reference/typescript/composio)

# Constructor

## constructor()

Creates a new instance of the Composio SDK.

The constructor initializes the SDK with the provided configuration options,
sets up the API client, and initializes all core models (tools, toolkits, etc.).

```typescript
constructor(config?: ComposioConfig): Composio
```

**Parameters**

| Name      | Type             | Description                                |
| --------- | ---------------- | ------------------------------------------ |
| `config?` | `ComposioConfig` | Configuration options for the Composio SDK |

**Returns**

`Composio`

**Example**

```typescript
// Initialize with default configuration
const composio = new Composio();

// Initialize with custom API key and base URL
const composio = new Composio({
  apiKey: 'your-api-key',
  baseURL: 'https://api.composio.dev'
});

// Initialize with custom provider
const composio = new Composio({
  apiKey: 'your-api-key',
  provider: new CustomProvider()
});
```

***

# Properties

| Name                | Type                                         | Description                                                                      |
| ------------------- | -------------------------------------------- | -------------------------------------------------------------------------------- |
| `authConfigs`       | `AuthConfigs`                                | Manage authentication configurations for toolkits                                |
| `connectedAccounts` | `ConnectedAccounts`                          | Manage authenticated connections                                                 |
| `create`            | `object`                                     | Creates a new tool router session for a user.                                    |
| `files`             | `Files`                                      | Upload and download files                                                        |
| `mcp`               | `MCP`                                        | Model Context Protocol server management                                         |
| `provider`          | `TProvider`                                  | The tool provider instance used for wrapping tools in framework-specific formats |
| `toolkits`          | `Toolkits`                                   | Retrieve toolkit metadata and authorize user connections                         |
| `toolRouter`        | `ToolRouter`                                 | Experimental feature, use with caution                                           |
| `tools`             | `Tools`                                      | List, retrieve, and execute tools                                                |
| `triggers`          | `Triggers`                                   | Manage webhook triggers and event subscriptions                                  |
| `use`               | `(id: string) => Promise` | Use an existing tool router session                                              |

# Methods

## createSession()

Creates a new instance of the Composio SDK with custom request options while preserving the existing configuration.
This method is particularly useful when you need to:

* Add custom headers for specific requests
* Track request contexts with unique identifiers
* Override default request behavior for a subset of operations

The new instance inherits all configuration from the parent instance (apiKey, baseURL, provider, etc.)
but allows you to specify custom request options that will be used for all API calls made through this session.

```typescript
createSession(options?: { headers?: ComposioRequestHeaders }): Composio
```

**Parameters**

| Name       | Type     |
| ---------- | -------- |
| `options?` | `object` |

**Returns**

`Composio` — A new Composio instance with the custom request options applied.

**Example**

```typescript
// Create a base Composio instance
const composio = new Composio({
  apiKey: 'your-api-key'
});

// Create a session with request tracking headers
const composioWithCustomHeaders = composio.createSession({
  headers: {
    'x-request-id': '1234567890',
    'x-correlation-id': 'session-abc-123',
    'x-custom-header': 'custom-value'

});

// Use the session for making API calls with the custom headers
await composioWithCustomHeaders.tools.list();
```

***

## flush()

Flush any pending telemetry and wait for it to complete.

In Node.js-compatible environments, telemetry is automatically flushed on process exit.
However, in environments like Cloudflare Workers that don't support process exit events,
you should call this method manually to ensure all telemetry is sent.

```typescript
async flush(): Promise<void>
```

**Returns**

`Promise<void>` — A promise that resolves when all pending telemetry has been sent.

**Example**

```typescript
// In a Cloudflare Worker, use ctx.waitUntil to ensure telemetry is flushed
export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext) {
    const composio = new Composio({ apiKey: env.COMPOSIO_API_KEY });

    // Do your work...
    const result = await composio.tools.execute(...);

    // Ensure telemetry flushes before worker terminates
    ctx.waitUntil(composio.flush());

    return new Response(JSON.stringify(result));

};
```

***

## getClient()

Get the Composio SDK client.

```typescript
getClient(): Composio
```

**Returns**

`Composio` — The Composio API client.

***

## getConfig()

Get the configuration SDK is initialized with

```typescript
getConfig(): ComposioConfig
```

**Returns**

`ComposioConfig` — The configuration SDK is initialized with

***

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
# AuthConfigs (/reference/sdk-reference/typescript/auth-configs)

# Usage

Access this class through the `composio.authConfigs` property:

```typescript
const composio = new Composio({ apiKey: 'your-api-key' });
const result = await composio.authConfigs.list();
```

# Methods

## create()

Create a new auth config

```typescript
async create(toolkit: string, options: object): Promise<{ authScheme: string; id: string; isComposioManaged: boolean; toolkit: string }>
```

**Parameters**

| Name      | Type     | Description                            |
| --------- | -------- | -------------------------------------- |
| `toolkit` | `string` | Unique identifier of the toolkit       |
| `options` | `object` | Options for creating a new auth config |

**Returns**

`Promise<...>` — Created auth config

**Example**

```typescript
const authConfig = await authConfigs.create('my-toolkit', {
  type: AuthConfigTypes.CUSTOM,
  name: 'My Custom Auth Config',
  authScheme: AuthSchemeTypes.API_KEY,
  credentials: {
    apiKey: '1234567890',
  },
});
```

***

## delete()

Deletes an authentication configuration.

This method permanently removes an auth config from the Composio platform.
This action cannot be undone and will prevent any connected accounts that use
this auth config from functioning.

```typescript
async delete(nanoid: string): Promise
```

**Parameters**

| Name     | Type     | Description                                        |
| -------- | -------- | -------------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the auth config to delete |

**Returns**

`Promise` — The deletion response

**Example**

```typescript
// Delete an auth config
await composio.authConfigs.delete('auth_abc123');
```

***

## disable()

Disables an authentication configuration.

This is a convenience method that calls updateStatus with 'DISABLED'.
When disabled, the auth config cannot be used to create new connected accounts
or authenticate with third-party services, but existing connections may continue to work.

```typescript
async disable(nanoid: string): Promise
```

**Parameters**

| Name     | Type     | Description                                         |
| -------- | -------- | --------------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the auth config to disable |

**Returns**

`Promise` — The updated auth config details

**Example**

```typescript
// Disable an auth config
await composio.authConfigs.disable('auth_abc123');
```

***

## enable()

Enables an authentication configuration.

This is a convenience method that calls updateStatus with 'ENABLED'.
When enabled, the auth config can be used to create new connected accounts
and authenticate with third-party services.

```typescript
async enable(nanoid: string): Promise
```

**Parameters**

| Name     | Type     | Description                                        |
| -------- | -------- | -------------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the auth config to enable |

**Returns**

`Promise` — The updated auth config details

**Example**

```typescript
// Enable an auth config
await composio.authConfigs.enable('auth_abc123');
```

***

## get()

Retrieves a specific authentication configuration by its ID.

This method fetches detailed information about a single auth config
and transforms the response to the SDK's standardized format.

```typescript
async get(nanoid: string): Promise<...>
```

**Parameters**

| Name     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the auth config to retrieve |

**Returns**

`Promise<...>` — The auth config details

**Example**

```typescript
// Get an auth config by ID
const authConfig = await composio.authConfigs.get('auth_abc123');
console.log(authConfig.name); // e.g., 'GitHub Auth'
console.log(authConfig.toolkit.slug); // e.g., 'github'
```

***

## list()

Lists authentication configurations based on provided filter criteria.

This method retrieves auth configs from the Composio API, transforms them to the SDK format,
and supports filtering by various parameters.

```typescript
async list(query?: { cursor?: string; isComposioManaged?: boolean; limit?: number; toolkit?: string }): Promise<...>
```

**Parameters**

| Name     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| `query?` | `object` | Optional query parameters for filtering auth configs |

**Returns**

`Promise<...>` — A paginated list of auth configurations

**Example**

```typescript
// List all auth configs
const allConfigs = await composio.authConfigs.list();

// List auth configs for a specific toolkit
const githubConfigs = await composio.authConfigs.list({
  toolkit: 'github'
});

// List Composio-managed auth configs
const managedConfigs = await composio.authConfigs.list({
  isComposioManaged: true
});
```

***

## update()

Updates an existing authentication configuration.

This method allows you to modify properties of an auth config such as credentials,
scopes, or tool restrictions. The update type (custom or default) determines which
fields can be updated.

```typescript
async update(nanoid: string, data: object): Promise
```

**Parameters**

| Name     | Type     | Description                                                    |
| -------- | -------- | -------------------------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the auth config to update             |
| `data`   | `object` | The data to update, which can be either custom or default type |

**Returns**

`Promise` — The updated auth config

**Example**

```typescript
// Update a custom auth config with new credentials
const updatedConfig = await composio.authConfigs.update('auth_abc123', {
  type: 'custom',
  credentials: {
    apiKey: 'new-api-key-value'

});

// Update a default auth config with new scopes
const updatedConfig = await composio.authConfigs.update('auth_abc123', {
  type: 'default',
  scopes: ['read:user', 'repo']
});
```

***

## updateStatus()

Updates the status of an authentication configuration.

This method allows you to enable or disable an auth config. When disabled,
the auth config cannot be used to create new connected accounts or authenticate
with third-party services.

```typescript
async updateStatus(status: 'ENABLED' | 'DISABLED', nanoid: string): Promise
```

**Parameters**

| Name     | Type        | Description                              |                                             |
| -------- | ----------- | ---------------------------------------- | ------------------------------------------- |
| `status` | \`'ENABLED' | 'DISABLED'\`                             | The status to set ('ENABLED' or 'DISABLED') |
| `nanoid` | `string`    | The unique identifier of the auth config |                                             |

**Returns**

`Promise` — The updated auth config details

**Example**

```typescript
// Disable an auth config
await composio.authConfigs.updateStatus('DISABLED', 'auth_abc123');

// Enable an auth config
await composio.authConfigs.updateStatus('ENABLED', 'auth_abc123');
```

***

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
# ConnectedAccounts (/reference/sdk-reference/typescript/connected-accounts)

# Usage

Access this class through the `composio.connectedAccounts` property:

```typescript
const composio = new Composio({ apiKey: 'your-api-key' });
const result = await composio.connectedAccounts.list();
```

# Methods

## delete()

Deletes a connected account.

This method permanently removes a connected account from the Composio platform.
This action cannot be undone and will revoke any access tokens associated with the account.

```typescript
async delete(nanoid: string): Promise
```

**Parameters**

| Name     | Type     | Description                                              |
| -------- | -------- | -------------------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the connected account to delete |

**Returns**

`Promise` — The deletion response

**Example**

```typescript
// Delete a connected account
await composio.connectedAccounts.delete('conn_abc123');
```

***

## disable()

Disable a connected account

```typescript
async disable(nanoid: string): Promise
```

**Parameters**

| Name     | Type     | Description                                |
| -------- | -------- | ------------------------------------------ |
| `nanoid` | `string` | Unique identifier of the connected account |

**Returns**

`Promise` — Updated connected account details

**Example**

```typescript
// Disable a connected account
const disabledAccount = await composio.connectedAccounts.disable('conn_abc123');
console.log(disabledAccount.isDisabled); // true

// You can also use updateStatus with a reason
// const disabledAccount = await composio.connectedAccounts.updateStatus('conn_abc123', {
//   enabled: false,
//   reason: 'No longer needed'
// });
```

***

## enable()

Enable a connected account

```typescript
async enable(nanoid: string): Promise
```

**Parameters**

| Name     | Type     | Description                                |
| -------- | -------- | ------------------------------------------ |
| `nanoid` | `string` | Unique identifier of the connected account |

**Returns**

`Promise` — Updated connected account details

**Example**

```typescript
// Enable a previously disabled connected account
const enabledAccount = await composio.connectedAccounts.enable('conn_abc123');
console.log(enabledAccount.isDisabled); // false
```

***

## get()

Retrieves a specific connected account by its ID.

This method fetches detailed information about a single connected account
and transforms the response to the SDK's standardized format.

```typescript
async get(nanoid: string): Promise<...>
```

**Parameters**

| Name     | Type     | Description                                    |
| -------- | -------- | ---------------------------------------------- |
| `nanoid` | `string` | The unique identifier of the connected account |

**Returns**

`Promise<...>` — The connected account details

**Example**

```typescript
// Get a connected account by ID
const account = await composio.connectedAccounts.get('conn_abc123');
console.log(account.status); // e.g., 'ACTIVE'
console.log(account.toolkit.slug); // e.g., 'github'
```

***

## initiate()

Compound function to create a new connected account.
This function creates a new connected account and returns a connection request.
Users can then wait for the connection to be established using the `waitForConnection` method.

```typescript
async initiate(userId: string, authConfigId: string, options?: object): Promise
```

**Parameters**

| Name           | Type     | Description                                  |
| -------------- | -------- | -------------------------------------------- |
| `userId`       | `string` | User ID of the connected account             |
| `authConfigId` | `string` | Auth config ID of the connected account      |
| `options?`     | `object` | Options for creating a new connected account |

**Returns**

`Promise` — Connection request object

**Example**

```typescript
// For OAuth2 authentication
const connectionRequest = await composio.connectedAccounts.initiate(
  'user_123',
  'auth_config_123',

    callbackUrl: 'https://your-app.com/callback',
    config: AuthScheme.OAuth2({
      access_token: 'your_access_token',
      token_type: 'Bearer'
    })

);

// For API Key authentication
const connectionRequest = await composio.connectedAccounts.initiate(
  'user_123',
  'auth_config_123',

    config: AuthScheme.ApiKey({
      api_key: 'your_api_key'
    })

);

// For Basic authentication
const connectionRequest = await composio.connectedAccounts.initiate(
  'user_123',
  'auth_config_123',

    config: AuthScheme.Basic({
      username: 'your_username',
      password: 'your_password'
    })

);
```

***

## link()

```typescript
async link(userId: string, authConfigId: string, options?: { callbackUrl?: string }): Promise
```

**Parameters**

| Name           | Type     | Description                                                                      |
| -------------- | -------- | -------------------------------------------------------------------------------- |
| `userId`       | `string` | \{string} - The external user ID to create the connected account for.            |
| `authConfigId` | `string` | \{string} - The auth config ID to create the connected account for.              |
| `options?`     | `object` | \{CreateConnectedAccountOptions} - Options for creating a new connected account. |

**Returns**

`Promise` — Connection request object

**Example**

```typescript
// create a connection request and redirect the user to the redirect url
const connectionRequest = await composio.connectedAccounts.link('user_123', 'auth_config_123');
const redirectUrl = connectionRequest.redirectUrl;
console.log(`Visit: ${redirectUrl} to authenticate your account`);

// Wait for the connection to be established
const connectedAccount = await connectionRequest.waitForConnection()
```

```typescript
// create a connection request and redirect the user to the redirect url
const connectionRequest = await composio.connectedAccounts.link('user_123', 'auth_config_123', {
  callbackUrl: 'https://your-app.com/callback'
});
const redirectUrl = connectionRequest.redirectUrl;
console.log(`Visit: ${redirectUrl} to authenticate your account`);

// Wait for the connection to be established
const connectedAccount = await composio.connectedAccounts.waitForConnection(connectionRequest.id);
```

***

## list()

Lists all connected accounts based on provided filter criteria.

This method retrieves connected accounts from the Composio API with optional filtering.

```typescript
async list(query?: object): Promise<...>
```

**Parameters**

| Name     | Type     | Description                                                |
| -------- | -------- | ---------------------------------------------------------- |
| `query?` | `object` | Optional query parameters for filtering connected accounts |

**Returns**

`Promise<...>` — A paginated list of connected accounts

**Example**

```typescript
// List all connected accounts
const allAccounts = await composio.connectedAccounts.list();

// List accounts for a specific user
const userAccounts = await composio.connectedAccounts.list({
  userIds: ['user123']
});

// List accounts for a specific toolkit
const githubAccounts = await composio.connectedAccounts.list({
  toolkitSlugs: ['github']
});
```

***

## refresh()

Refreshes a connected account's authentication credentials.

This method attempts to refresh OAuth tokens or other credentials associated with
the connected account. This is useful when a token has expired or is about to expire.

```typescript
async refresh(nanoid: string, options?: { redirectUrl?: string; validateCredentials?: boolean }): Promise
```

**Parameters**

| Name       | Type     | Description                                               |
| ---------- | -------- | --------------------------------------------------------- |
| `nanoid`   | `string` | The unique identifier of the connected account to refresh |
| `options?` | `object` |                                                           |

**Returns**

`Promise` — The response containing the refreshed account details

**Example**

```typescript
// Refresh a connected account's credentials
const refreshedAccount = await composio.connectedAccounts.refresh('conn_abc123');
```

***

## updateStatus()

Update the status of a connected account

```typescript
async updateStatus(nanoid: string, params: ConnectedAccountUpdateStatusParams): Promise
```

**Parameters**

| Name     | Type                                 | Description                                |
| -------- | ------------------------------------ | ------------------------------------------ |
| `nanoid` | `string`                             | Unique identifier of the connected account |
| `params` | `ConnectedAccountUpdateStatusParams` | Parameters for updating the status         |

**Returns**

`Promise` — Updated connected account details

**Example**

```typescript
// Enable a connected account
const updatedAccount = await composio.connectedAccounts.updateStatus('conn_abc123', {
  enabled: true
});

// Disable a connected account with a reason
const disabledAccount = await composio.connectedAccounts.updateStatus('conn_abc123', {
  enabled: false,
  reason: 'Token expired'
});
```

***

## waitForConnection()

Waits for a connection request to complete and become active.

This method continuously polls the Composio API to check the status of a connection
until it either becomes active, enters a terminal error state, or times out.

```typescript
async waitForConnection(connectedAccountId: string, timeout?: number): Promise<...>
```

**Parameters**

| Name                 | Type     | Description                                                |
| -------------------- | -------- | ---------------------------------------------------------- |
| `connectedAccountId` | `string` | The ID of the connected account to wait for                |
| `timeout?`           | `number` | Maximum time to wait in milliseconds (default: 60 seconds) |

**Returns**

`Promise<...>` — The finalized connected account data

**Example**

```typescript
// Wait for a connection to complete with default timeout
const connectedAccount = await composio.connectedAccounts.waitForConnection('conn_123abc');

// Wait with a custom timeout of 2 minutes
const connectedAccount = await composio.connectedAccounts.waitForConnection('conn_123abc', 120000);
```

***

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
# MCP (/reference/sdk-reference/typescript/mcp)

# Usage

Access this class through the `composio.mcp` property:

```typescript
const composio = new Composio({ apiKey: 'your-api-key' });
const result = await composio.mcp.list();
```

# Properties

| Name     | Type       |
| -------- | ---------- |
| `client` | `Composio` |

# Methods

## create()

Create a new MCP configuration.

```typescript
async create(name: string, mcpConfig: object): Promise
```

**Parameters**

| Name        | Type     |
| ----------- | -------- |
| `name`      | `string` |
| `mcpConfig` | `object` |

**Returns**

`Promise` — Created server details with instance getter

**Example**

```typescript
const server = await composio.mcpConfig.create("personal-mcp-server", {
  toolkits: ["github", "slack"],
  allowedTools: ["GMAIL_FETCH_EMAILS", "SLACK_SEND_MESSAGE"],
  manuallyManageConnections: false

});

const server = await composio.mcpConfig.create("personal-mcp-server", {
  toolkits: [{ toolkit: "gmail", authConfigId: "ac_243434343" }],
  allowedTools: ["GMAIL_FETCH_EMAILS"],
  manuallyManageConnections: false

});
```

***

## delete()

Delete an MCP server configuration permanently

```typescript
async delete(serverId: string): Promise<{ deleted: boolean; id: string }>
```

**Parameters**

| Name       | Type     | Description                                       |
| ---------- | -------- | ------------------------------------------------- |
| `serverId` | `string` | The unique identifier of the MCP server to delete |

**Returns**

`Promise<...>` — Confirmation object with server ID and deletion status

**Example**

```typescript
// Delete an MCP server by ID
const result = await composio.experimental.mcp.delete("mcp_12345");

if (result.deleted) {
  console.log(`Server ${result.id} has been successfully deleted`);
} else {
  console.log(`Failed to delete server ${result.id}`);

// Example with error handling
try {
  const result = await composio.experimental.mcp.delete("mcp_12345");
  console.log("Deletion successful:", result);
} catch (error) {
  console.error("Failed to delete MCP server:", error.message);

// Delete and verify from list
await composio.experimental.mcp.delete("mcp_12345");
const servers = await composio.experimental.mcp.list({});
const serverExists = servers.items.some(server => server.id === "mcp_12345");
console.log("Server still exists:", serverExists); // Should be false
```

***

## generate()

Get server URLs for an existing MCP server.
The response is wrapped according to the provider's specifications.

```typescript
async generate(userId: string, mcpConfigId: string, options?: { manuallyManageConnections?: boolean }): Promise<...>
```

**Parameters**

| Name          | Type     | Description                                                                    |
| ------------- | -------- | ------------------------------------------------------------------------------ |
| `userId`      | `string` | \{string} external user id from your database for whom you want the server for |
| `mcpConfigId` | `string` | \{string} config id of the MCPConfig for which you want to create a server for |
| `options?`    | `object` | \{object} additional options                                                   |

**Returns**

`Promise<...>`

**Example**

```typescript
import { Composio } from "@composio/code";

const composio = new Composio();
const mcp = await composio.experimental.mcp.generate("default", "<mcp_config_id>");
```

***

## get()

Retrieve detailed information about a specific MCP server by its ID

```typescript
async get(serverId: string): Promise<...>
```

**Parameters**

| Name       | Type     | Description                                         |
| ---------- | -------- | --------------------------------------------------- |
| `serverId` | `string` | The unique identifier of the MCP server to retrieve |

**Returns**

`Promise<...>` — Complete MCP server details including configuration, tools, and metadata

**Example**

```typescript
// Get a specific MCP server by ID
const server = await composio.experimental.mcp.get("mcp_12345");

console.log(server.name); // "My Personal MCP Server"
console.log(server.allowedTools); // ["GITHUB_CREATE_ISSUE", "SLACK_SEND_MESSAGE"]
console.log(server.toolkits); // ["github", "slack"]
console.log(server.serverInstanceCount); // 3

// Access setup commands for different clients
console.log(server.commands.claude); // Claude setup command
console.log(server.commands.cursor); // Cursor setup command
console.log(server.commands.windsurf); // Windsurf setup command

// Use the MCP URL for direct connections
const mcpUrl = server.MCPUrl;
```

***

## list()

List the MCP servers with optional filtering and pagination

```typescript
async list(options: { authConfigs: string[]; limit: number; name?: string; page: number; toolkits: string[] }): Promise<...>
```

**Parameters**

| Name      | Type     | Description                      |
| --------- | -------- | -------------------------------- |
| `options` | `object` | Filtering and pagination options |

**Returns**

`Promise<...>` — Paginated list of MCP servers with metadata

**Example**

```typescript
// List all MCP servers
const allServers = await composio.experimental.mcp.list({});

// List with pagination
const pagedServers = await composio.experimental.mcp.list({
  page: 2,
  limit: 5
});

// Filter by toolkit
const githubServers = await composio.experimental.mcp.list({
  toolkits: ['github', 'slack']
});

// Filter by name
const namedServers = await composio.experimental.mcp.list({
  name: 'personal'
});
```

***

## update()

Update an existing MCP server configuration with new settings

```typescript
async update(serverId: string, config: object): Promise<...>
```

**Parameters**

| Name       | Type     | Description                                       |
| ---------- | -------- | ------------------------------------------------- |
| `serverId` | `string` | The unique identifier of the MCP server to update |
| `config`   | `object` | Update configuration parameters                   |

**Returns**

`Promise<...>` — Updated MCP server configuration with all details

**Example**

```typescript
// Update server name only
const updatedServer = await composio.experimental.mcp.update("mcp_12345", {
  name: "My Updated MCP Server"
});

// Update toolkits and tools
const serverWithNewTools = await composio.experimental.mcp.update("mcp_12345", {
  toolkits: [

      toolkit: "github",
      authConfigId: "auth_abc123",
      allowedTools: ["GITHUB_CREATE_ISSUE", "GITHUB_LIST_REPOS"]
    },

      toolkit: "slack",
      authConfigId: "auth_xyz789",
      allowedTools: ["SLACK_SEND_MESSAGE", "SLACK_LIST_CHANNELS"]

  ]
});

// Update connection management setting
const serverWithManualAuth = await composio.experimental.mcp.update("mcp_12345", {
  name: "Manual Auth Server",
  manuallyManageConnections: true
});

// Complete update example
const fullyUpdatedServer = await composio.experimental.mcp.update("mcp_12345", {
  name: "Production MCP Server",
  toolkits: [

      toolkit: "gmail",
      authConfigId: "auth_gmail_prod",

  ],
  allowedTools: ["GMAIL_SEND_EMAIL", "GMAIL_FETCH_EMAILS"]
  manuallyManageConnections: false
});

console.log("Updated server:", fullyUpdatedServer.name);
console.log("New tools:", fullyUpdatedServer.allowedTools);
```

***

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
# Toolkits (/reference/sdk-reference/typescript/toolkits)

# Usage

Access this class through the `composio.toolkits` property:

```typescript
const composio = new Composio({ apiKey: 'your-api-key' });
const result = await composio.toolkits.list();
```

# Methods

## authorize()

Authorizes a user to use a toolkit.
This method will create an auth config if one doesn't exist and initiate a connection request.

```typescript
async authorize(userId: string, toolkitSlug: string, authConfigId?: string): Promise
```

**Parameters**

| Name            | Type     | Description                          |
| --------------- | -------- | ------------------------------------ |
| `userId`        | `string` | The user id of the user to authorize |
| `toolkitSlug`   | `string` | The slug of the toolkit to authorize |
| `authConfigId?` | `string` |                                      |

**Returns**

`Promise` — The connection request object

**Example**

```typescript
const connectionRequest = await composio.toolkits.authorize(userId, 'github');
```

***

## get()

Retrieves a specific toolkit by its slug identifier.

**Overload 1**

```typescript
async get(slug: string): Promise<...>
```

**Parameters**

| Name   | Type     | Description                                           |
| ------ | -------- | ----------------------------------------------------- |
| `slug` | `string` | The unique slug identifier of the toolkit to retrieve |

**Returns**

`Promise<...>` — The toolkit object with detailed information

**Overload 2**

```typescript
async get(query?: object): Promise<...>
```

**Parameters**

| Name     | Type     | Description                             |
| -------- | -------- | --------------------------------------- |
| `query?` | `object` | The query parameters to filter toolkits |

**Returns**

`Promise<...>` — A paginated list of toolkits matching the query criteria

**Example**

```typescript
// Get a specific toolkit
const githubToolkit = await composio.toolkits.get('github');
console.log(githubToolkit.name); // GitHub
console.log(githubToolkit.authConfigDetails); // Authentication configuration details
```

***

## getAuthConfigCreationFields()

Retrieves the fields required for creating an auth config for a toolkit.

```typescript
async getAuthConfigCreationFields(toolkitSlug: string, authScheme: AuthSchemeType, options: { requiredOnly?: boolean }): Promise<...>
```

**Parameters**

| Name          | Type             | Description                                        |
| ------------- | ---------------- | -------------------------------------------------- |
| `toolkitSlug` | `string`         | The slug of the toolkit to retrieve the fields for |
| `authScheme`  | `AuthSchemeType` | The auth scheme to retrieve the fields for         |
| `options`     | `object`         |                                                    |

**Returns**

`Promise<...>` — The fields required for creating an auth config

***

## getConnectedAccountInitiationFields()

Retrieves the fields required for initiating a connected account for a toolkit.

```typescript
async getConnectedAccountInitiationFields(toolkitSlug: string, authScheme: AuthSchemeType, options: { requiredOnly?: boolean }): Promise<...>
```

**Parameters**

| Name          | Type             | Description                                        |
| ------------- | ---------------- | -------------------------------------------------- |
| `toolkitSlug` | `string`         | The slug of the toolkit to retrieve the fields for |
| `authScheme`  | `AuthSchemeType` | The auth scheme to retrieve the fields for         |
| `options`     | `object`         |                                                    |

**Returns**

`Promise<...>` — The fields required for initiating a connected account

***

## listCategories()

Retrieves all toolkit categories available in the Composio SDK.

This method fetches the complete list of categories from the Composio API
and transforms the response to use camelCase property naming.

```typescript
async listCategories(): Promise<...>
```

**Returns**

`Promise<...>` — The list of toolkit categories

**Example**

```typescript
// Get all toolkit categories
const categories = await composio.toolkits.listCategories();
console.log(categories.items); // Array of category objects
```

***

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
# Tools (/reference/sdk-reference/typescript/tools)

# Usage

Access this class through the `composio.tools` property:

```typescript
const composio = new Composio({ apiKey: 'your-api-key' });
const result = await composio.tools.list();
```

# Methods

## createCustomTool()

Creates a custom tool that can be used within the Composio SDK.

Custom tools allow you to extend the functionality of Composio with your own implementations
while keeping a consistent interface for both built-in and custom tools.

```typescript
async createCustomTool(body: CustomToolOptions): Promise<...>
```

**Parameters**

| Name   | Type                   | Description                           |
| ------ | ---------------------- | ------------------------------------- |
| `body` | `CustomToolOptions` | The configuration for the custom tool |

**Returns**

`Promise<...>` — The created custom tool

**Example**

```typescript
// creating a custom tool with a toolkit
await composio.tools.createCustomTool({
  name: 'My Custom Tool',
  description: 'A custom tool that does something specific',
  slug: 'MY_CUSTOM_TOOL',
  userId: 'default',
  connectedAccountId: '123',
  toolkitSlug: 'github',
  inputParameters: z.object({
    param1: z.string().describe('First parameter'),
  }),
  execute: async (input, connectionConfig, executeToolRequest) => {
    // Custom logic here
    return { data: { result: 'Success!' } };

});
```

```typescript
// creating a custom tool without a toolkit
await composio.tools.createCustomTool({
  name: 'My Custom Tool',
  description: 'A custom tool that does something specific',
  slug: 'MY_CUSTOM_TOOL',
  inputParameters: z.object({
    param1: z.string().describe('First parameter'),
  }),
  execute: async (input) => {
    // Custom logic here
    return { data: { result: 'Success!' } };

});
```

***

## execute()

Executes a given tool with the provided parameters.

This method calls the Composio API or a custom tool handler to execute the tool and returns the response.
It automatically determines whether to use a custom tool or a Composio API tool based on the slug.

**Version Control:**
By default, manual tool execution requires a specific toolkit version. If the version resolves to "latest",
the execution will throw a `ComposioToolVersionRequiredError` unless `dangerouslySkipVersionCheck` is set to `true`.
This helps prevent unexpected behavior when new toolkit versions are released.

```typescript
async execute(slug: string, body: object, modifiers?: ExecuteToolModifiers): Promise<...>
```

**Parameters**

| Name         | Type                   | Description                                             |
| ------------ | ---------------------- | ------------------------------------------------------- |
| `slug`       | `string`               | The slug/ID of the tool to be executed                  |
| `body`       | `object`               | The parameters to be passed to the tool                 |
| `modifiers?` | `ExecuteToolModifiers` | Optional modifiers to transform the request or response |

**Returns**

`Promise<...>` — - The response from the tool execution

**Example**

```typescript
const result = await composio.tools.execute('GITHUB_GET_REPOS', {
  userId: 'default',
  version: '20250909_00',
  arguments: { owner: 'composio' }
});
```

```typescript
const result = await composio.tools.execute('HACKERNEWS_GET_USER', {
  userId: 'default',
  arguments: { userId: 'pg' },
  dangerouslySkipVersionCheck: true // Allows execution with "latest" version
});
```

```typescript
// If toolkitVersions are set during Composio initialization, no need to pass version
const composio = new Composio({ toolkitVersions: { github: '20250909_00' } });
const result = await composio.tools.execute('GITHUB_GET_REPOS', {
  userId: 'default',
  arguments: { owner: 'composio' }
});
```

```typescript
const result = await composio.tools.execute('GITHUB_GET_ISSUES', {
  userId: 'default',
  version: '20250909_00',
  arguments: { owner: 'composio', repo: 'sdk' }
}, {
  beforeExecute: ({ toolSlug, toolkitSlug, params }) => {
    console.log(`Executing ${toolSlug} from ${toolkitSlug}`);
    return params;
  },
  afterExecute: ({ toolSlug, toolkitSlug, result }) => {
    console.log(`Completed ${toolSlug}`);
    return result;

});
```

***

## executeMetaTool()

Executes a composio meta tool based on tool router session

```typescript
async executeMetaTool(toolSlug: string, body: { arguments?: Record<string, unknown>; sessionId: string }, modifiers?: SessionExecuteMetaModifiers): Promise<...>
```

**Parameters**

| Name         | Type                          | Description                        |
| ------------ | ----------------------------- | ---------------------------------- |
| `toolSlug`   | `string`                      | The slug of the tool to execute    |
| `body`       | `object`                      | The execution parameters           |
| `modifiers?` | `SessionExecuteMetaModifiers` | The modifiers to apply to the tool |

**Returns**

`Promise<...>` — The response from the tool execution

***

## get()

Get a list of tools from Composio based on filters.
This method fetches the tools from the Composio API and wraps them using the provider.

**Overload 1**

```typescript
async get(userId: string, filters: ToolListParams, options?: ProviderOptions): Promise
```

**Parameters**

| Name       | Type              | Description                                   |
| ---------- | ----------------- | --------------------------------------------- |
| `userId`   | `string`          | The user id to get the tools for              |
| `filters`  | `ToolListParams`  | The filters to apply when fetching tools      |
| `options?` | `ProviderOptions` | Optional provider options including modifiers |

**Returns**

`Promise` — The wrapped tools collection

**Overload 2**

```typescript
async get(userId: string, slug: string, options?: ProviderOptions): Promise
```

**Parameters**

| Name       | Type              | Description                                   |
| ---------- | ----------------- | --------------------------------------------- |
| `userId`   | `string`          | The user id to get the tool for               |
| `slug`     | `string`          | The slug of the tool to fetch                 |
| `options?` | `ProviderOptions` | Optional provider options including modifiers |

**Returns**

`Promise` — The wrapped tool

**Example**

```typescript
// Get tools from the GitHub toolkit
const tools = await composio.tools.get('default', {
  toolkits: ['github'],
  limit: 10
});

// Get tools with search
const searchTools = await composio.tools.get('default', {
  search: 'user',
  limit: 10
});

// Get a specific tool by slug
const hackerNewsUserTool = await composio.tools.get('default', 'HACKERNEWS_GET_USER');

// Get a tool with schema modifications
const tool = await composio.tools.get('default', 'GITHUB_GET_REPOS', {
  modifySchema: (toolSlug, toolkitSlug, schema) => {
    // Customize the tool schema
    return {...schema, description: 'Custom description'};

});
```

***

## getInput()

Fetches the input parameters for a given tool.

This method is used to get the input parameters for a tool before executing it.

```typescript
async getInput(slug: string, body: ToolGetInputParams): Promise
```

**Parameters**

| Name   | Type                 | Description                             |
| ------ | -------------------- | --------------------------------------- |
| `slug` | `string`             | The ID of the tool to find input for    |
| `body` | `ToolGetInputParams` | The parameters to be passed to the tool |

**Returns**

`Promise` — The input parameters schema for the specified tool

**Example**

```typescript
// Get input parameters for a specific tool
const inputParams = await composio.tools.getInput('GITHUB_CREATE_ISSUE', {
  userId: 'default'
});
console.log(inputParams.schema);
```

***

## getRawComposioToolBySlug()

Retrieves a specific tool by its slug from the Composio API.

This method fetches a single tool in raw format without provider-specific wrapping,
providing direct access to the tool's schema and metadata. Tool versions are controlled
at the Composio SDK initialization level through the `toolkitVersions` configuration.

```typescript
async getRawComposioToolBySlug(slug: string, options?: ToolRetrievalOptions): Promise<...>
```

**Parameters**

| Name       | Type                   | Description                                                    |
| ---------- | ---------------------- | -------------------------------------------------------------- |
| `slug`     | `string`               | The unique identifier of the tool (e.g., 'GITHUB\_GET\_REPOS') |
| `options?` | `ToolRetrievalOptions` | Optional configuration for tool retrieval                      |

**Returns**

`Promise<...>` — The requested tool with its complete schema and metadata

**Example**

```typescript
// Get a tool by slug
const tool = await composio.tools.getRawComposioToolBySlug('GITHUB_GET_REPOS');
console.log(tool.name, tool.description);

// Get a tool with schema transformation
const customizedTool = await composio.tools.getRawComposioToolBySlug(
  'SLACK_SEND_MESSAGE',

    modifySchema: ({ toolSlug, toolkitSlug, schema }) => {
      return {
        ...schema,
        description: `Enhanced ${schema.description} with custom modifications`,
        customMetadata: {
          lastModified: new Date().toISOString(),
          toolkit: toolkitSlug

      };

);

// Get a custom tool (will check custom tools first)
const customTool = await composio.tools.getRawComposioToolBySlug('MY_CUSTOM_TOOL');

// Access tool properties
const githubTool = await composio.tools.getRawComposioToolBySlug('GITHUB_CREATE_ISSUE');
console.log({
  slug: githubTool.slug,
  name: githubTool.name,
  toolkit: githubTool.toolkit?.name,
  version: githubTool.version,
  availableVersions: githubTool.availableVersions,
  inputParameters: githubTool.inputParameters
});
```

***

## getRawComposioTools()

Lists all tools available in the Composio SDK including custom tools.

This method fetches tools from the Composio API in raw format and combines them with
any registered custom tools. The response can be filtered and modified as needed.
It provides access to the underlying tool data without provider-specific wrapping.

```typescript
async getRawComposioTools(query: ToolListParams, options?: SchemaModifierOptions): Promise
```

**Parameters**

| Name       | Type                    | Description                                     |
| ---------- | ----------------------- | ----------------------------------------------- |
| `query`    | `ToolListParams`        | Query parameters to filter the tools (required) |
| `options?` | `SchemaModifierOptions` | Optional configuration for tool retrieval       |

**Returns**

`Promise` — List of tools matching the query criteria

**Example**

```typescript
// Get tools from specific toolkits
const githubTools = await composio.tools.getRawComposioTools({
  toolkits: ['github'],
  limit: 10
});

// Get specific tools by slug
const specificTools = await composio.tools.getRawComposioTools({
  tools: ['GITHUB_GET_REPOS', 'HACKERNEWS_GET_USER']
});

// Get tools from specific toolkits
const githubTools = await composio.tools.getRawComposioTools({
  toolkits: ['github'],
  limit: 10
});

// Get tools with schema transformation
const customizedTools = await composio.tools.getRawComposioTools({
  toolkits: ['github'],
  limit: 5
}, {
  modifySchema: ({ toolSlug, toolkitSlug, schema }) => {
    // Add custom properties to tool schema
    return {
      ...schema,
      customProperty: `Modified ${toolSlug} from ${toolkitSlug}`,
      tags: [...(schema.tags || []), 'customized']
    };

});

// Search for tools
const searchResults = await composio.tools.getRawComposioTools({
  search: 'user management'
});

// Get tools by authentication config
const authSpecificTools = await composio.tools.getRawComposioTools({
  authConfigIds: ['auth_config_123']
});
```

***

## getRawToolRouterMetaTools()

Fetches the meta tools for a tool router session.
This method fetches the meta tools from the Composio API and transforms them to the expected format.
It provides access to the underlying meta tool data without provider-specific wrapping.

```typescript
async getRawToolRouterMetaTools(sessionId: string, options?: SchemaModifierOptions): Promise
```

**Parameters**

| Name        | Type                    | Description                                                        |
| ----------- | ----------------------- | ------------------------------------------------------------------ |
| `sessionId` | `string`                | \{string} The session id to get the meta tools for                 |
| `options?`  | `SchemaModifierOptions` | \{SchemaModifierOptions} Optional configuration for tool retrieval |

**Returns**

`Promise` — The list of meta tools

**Example**

```typescript
const metaTools = await composio.tools.getRawToolRouterMetaTools('session_123');
console.log(metaTools);
```

***

## getToolsEnum()

Fetches the list of all available tools in the Composio SDK.

This method is mostly used by the CLI to get the list of tools.
No filtering is done on the tools, the list is cached in the backend, no further optimization is required.

```typescript
async getToolsEnum(): Promise
```

**Returns**

`Promise` — The complete list of all available tools with their metadata

**Example**

```typescript
// Get all available tools as an enum
const toolsEnum = await composio.tools.getToolsEnum();
console.log(toolsEnum.items);
```

***

## proxyExecute()

Proxies a custom request to a toolkit/integration.

This method allows sending custom requests to a specific toolkit or integration
when you need more flexibility than the standard tool execution methods provide.

```typescript
async proxyExecute(body: object): Promise
```

**Parameters**

| Name   | Type     | Description                                                                 |
| ------ | -------- | --------------------------------------------------------------------------- |
| `body` | `object` | The parameters for the proxy request including toolkit slug and custom data |

**Returns**

`Promise` — The response from the proxied request

**Example**

```typescript
// Send a custom request to a toolkit
const response = await composio.tools.proxyExecute({
  toolkitSlug: 'github',
  userId: 'default',
  data: {
    endpoint: '/repos/owner/repo/issues',
    method: 'GET'

});
console.log(response.data);
```

***

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
# Triggers (/reference/sdk-reference/typescript/triggers)

# Usage

Access this class through the `composio.triggers` property:

```typescript
const composio = new Composio({ apiKey: 'your-api-key' });
const result = await composio.triggers.list();
```

# Methods

## create()

Create a new trigger instance for a user
If the connected account id is not provided, the first connected account for the user and toolkit will be used

```typescript
async create(userId: string, slug: string, body?: { connectedAccountId?: string; triggerConfig?: Record<string, unknown> }): Promise<{ triggerId: string }>
```

**Parameters**

| Name     | Type     | Description                                   |
| -------- | -------- | --------------------------------------------- |
| `userId` | `string` | The user id of the trigger instance           |
| `slug`   | `string` | The slug of the trigger instance              |
| `body?`  | `object` | The parameters to create the trigger instance |

**Returns**

`Promise<...>` — The created trigger instance

***

## delete()

Delete a trigger instance

```typescript
async delete(triggerId: string): Promise<{ triggerId: string }>
```

**Parameters**

| Name        | Type     | Description                      |
| ----------- | -------- | -------------------------------- |
| `triggerId` | `string` | The slug of the trigger instance |

**Returns**

`Promise<...>`

***

## disable()

Disable a trigger instance

```typescript
async disable(triggerId: string): Promise
```

**Parameters**

| Name        | Type     | Description                    |
| ----------- | -------- | ------------------------------ |
| `triggerId` | `string` | The id of the trigger instance |

**Returns**

`Promise` — The updated trigger instance

***

## enable()

Enable a trigger instance

```typescript
async enable(triggerId: string): Promise
```

**Parameters**

| Name        | Type     | Description                    |
| ----------- | -------- | ------------------------------ |
| `triggerId` | `string` | The id of the trigger instance |

**Returns**

`Promise` — The updated trigger instance

***

## getType()

Retrieve a trigger type by its slug for the provided version of the app
Use the global toolkit versions param when initializing composio to pass a toolkitversion

```typescript
async getType(slug: string): Promise<...>
```

**Parameters**

| Name   | Type     | Description                  |
| ------ | -------- | ---------------------------- |
| `slug` | `string` | The slug of the trigger type |

**Returns**

`Promise<...>` — The trigger type object

***

## listActive()

Fetch list of all the active triggers

```typescript
async listActive(query?: object): Promise<...>
```

**Parameters**

| Name     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| `query?` | `object` | The query parameters to filter the trigger instances |

**Returns**

`Promise<...>` — List of trigger instances

**Example**

```typescript
const triggers = await triggers.listActive({
  authConfigIds: ['123'],
  connectedAccountIds: ['456'],
});
```

***

## listEnum()

Fetches the list of all the available trigger enums

This method is used by the CLI where filters are not required.

```typescript
async listEnum(): Promise
```

**Returns**

`Promise`

***

## listTypes()

List all the trigger types

```typescript
async listTypes(query?: { cursor?: string; limit?: number | null; toolkits?: string[] | null }): Promise<...>
```

**Parameters**

| Name     | Type     | Description                                      |
| -------- | -------- | ------------------------------------------------ |
| `query?` | `object` | The query parameters to filter the trigger types |

**Returns**

`Promise<...>` — The list of trigger types

***

## subscribe()

Subscribe to all the triggers

```typescript
async subscribe(fn: object, filters: object): Promise<void>
```

**Parameters**

| Name      | Type     | Description                                     |
| --------- | -------- | ----------------------------------------------- |
| `fn`      | `object` | The function to call when a trigger is received |
| `filters` | `object` | The filters to apply to the triggers            |

**Returns**

`Promise<void>`

**Example**

```typescript
triggers.subscribe((data) => {
  console.log(data);
}, );
```

***

## unsubscribe()

Unsubscribe from all the triggers

```typescript
async unsubscribe(): Promise<void>
```

**Returns**

`Promise<void>`

**Example**

```typescript
composio.trigger.subscribe((data) => {
  console.log(data);
});

await triggers.unsubscribe();
```

***

## update()

Update an existing trigger instance

```typescript
async update(triggerId: string, body: { status: 'enable' | 'disable' }): Promise<{ status: 'success' }>
```

**Parameters**

| Name        | Type     | Description                                   |
| ----------- | -------- | --------------------------------------------- |
| `triggerId` | `string` | The Id of the trigger instance                |
| `body`      | `object` | The parameters to update the trigger instance |

**Returns**

`Promise<...>` — The updated trigger instance response

***

## verifyWebhook()

Verify an incoming webhook payload and signature.

This method validates that the webhook request is authentic by:

1. Verifying the HMAC-SHA256 signature matches the payload using the correct signing format
2. Optionally checking that the webhook timestamp is within the tolerance window

The signature is computed as: `HMAC-SHA256(${webhookId}.${webhookTimestamp}.${payload}, secret)`
and is expected in the format: `v1,base64EncodedSignature`

```typescript
async verifyWebhook(params: object): Promise
```

**Parameters**

| Name     | Type     | Description                 |
| -------- | -------- | --------------------------- |
| `params` | `object` | The verification parameters |

**Returns**

`Promise` — The verified and parsed webhook payload with version information

**Example**

```typescript
// In an Express.js webhook handler
app.post('/webhook', express.raw({ type: 'application/json' }), async (req, res) => {
  try {
    const result = await composio.triggers.verifyWebhook({
      payload: req.body.toString(),
      signature: req.headers['webhook-signature'] as string,
      webhookId: req.headers['webhook-id'] as string,
      webhookTimestamp: req.headers['webhook-timestamp'] as string,
      secret: process.env.COMPOSIO_WEBHOOK_SECRET!,
    });

    // Process the verified payload
    console.log('Webhook version:', result.version);
    console.log('Received trigger:', result.payload.triggerSlug);
    res.status(200).send('OK');
  } catch (error) {
    console.error('Webhook verification failed:', error);
    res.status(401).send('Unauthorized');

});
```

***

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
