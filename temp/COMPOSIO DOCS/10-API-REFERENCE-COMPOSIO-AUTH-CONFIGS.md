# Auth Configs (/reference/api-reference/auth-configs)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Authentication configuration management

# Endpoints

| Endpoint                                       | Quick Link                                                                                                                  |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `POST /api/v3/auth_configs`                    | [Create new authentication configuration](/reference/api-reference/auth-configs/postAuthConfigs)                            |
| `GET /api/v3/auth_configs`                     | [List authentication configurations with optional filters](/reference/api-reference/auth-configs/getAuthConfigs)            |
| `GET /api/v3/auth_configs/{nanoid}`            | [Get single authentication configuration by ID](/reference/api-reference/auth-configs/getAuthConfigsByNanoid)               |
| `PATCH /api/v3/auth_configs/{nanoid}`          | [Update an authentication configuration](/reference/api-reference/auth-configs/patchAuthConfigsByNanoid)                    |
| `DELETE /api/v3/auth_configs/{nanoid}`         | [Delete an authentication configuration](/reference/api-reference/auth-configs/deleteAuthConfigsByNanoid)                   |
| `PATCH /api/v3/auth_configs/{nanoid}/{status}` | [Enable or disable an authentication configuration](/reference/api-reference/auth-configs/patchAuthConfigsByNanoidByStatus) |

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
# Delete an authentication configuration

**Documentation:** /reference/api-reference/auth-configs/deleteAuthConfigsByNanoid

Soft-deletes an authentication configuration by marking it as deleted in the database. This operation cannot be undone.

---

## DELETE `/api/v3/auth_configs/{nanoid}`

**Endpoint:** `https://backend.composio.dev/api/v3/auth_configs/{nanoid}`

**Summary:** Delete an authentication configuration

Soft-deletes an authentication configuration by marking it as deleted in the database. This operation cannot be undone.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (authConfigId)) *(required)*: The unique identifier of the authentication configuration to delete

### Responses

#### 200 - Successfully deleted auth config

**Response Schema:**


#### 400 - Bad request - Invalid auth config ID format

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication required

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Auth config not found - The specified configuration does not exist or was already deleted

**Response Schema:**


#### 500 - Internal server error - Something went wrong on the server

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

### Example cURL Request

```bash
curl -X DELETE "https://backend.composio.dev/api/v3/auth_configs/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List authentication configurations with optional filters

**Documentation:** /reference/api-reference/auth-configs/getAuthConfigs

Retrieves all auth configs for your project. Auth configs define how users authenticate with external services (OAuth, API keys, etc.). Use filters to find configs for specific toolkits or to distinguish between Composio-managed and custom configurations.

---

## GET `/api/v3/auth_configs`

**Endpoint:** `https://backend.composio.dev/api/v3/auth_configs`

**Summary:** List authentication configurations with optional filters

Retrieves all auth configs for your project. Auth configs define how users authenticate with external services (OAuth, API keys, etc.). Use filters to find configs for specific toolkits or to distinguish between Composio-managed and custom configurations.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `is_composio_managed` (any): Whether to filter by composio managed auth configs
- `toolkit_slug` (string): Comma-separated list of toolkit slugs to filter auth configs by
- `deprecated_app_id` (string): The app id to filter by
- `deprecated_status` (string): 
- `show_disabled` (boolean,null): Show disabled auth configs
- `search` (string): Search auth configs by name or id
- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Successfully fetched auth configs

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `id` (string (authConfigId)) *(required)*: The unique ID of the authentication configuration
    - `uuid` (string) *(required)*: The UUID of the authentication configuration (for backward compatibility)
    - `type` (enum: "default" | "custom") *(required)*: The type of the authentication configuration (custom or default)
    - `toolkit` (object) *(required)*: Information about the associated integration
      - `slug` (string) *(required)*: The unique identifier of the integration app
      - `logo` (string) *(required)*: The URL to the integration app's logo image
    - `name` (string) *(required)*: The display name of the authentication configuration
    - `auth_scheme` (enum: "OAUTH2" | "OAUTH1" | "API_KEY" | ...): The authentication scheme used (e.g., OAuth2, API Key, etc.)
    - `is_composio_managed` (boolean): Whether this authentication configuration is managed by Composio or the user
    - `credentials` (object): The authentication credentials (tokens, keys, etc.) - may be partially hidden for security
    - `proxy_config` (object,null)
    - `status` (enum: "ENABLED" | "DISABLED") *(required)*: Current status of the authentication configuration
    - `created_by` (string): The identifier of the user who created the auth config
    - `created_at` (string): ISO 8601 date-time when the auth config was created
    - `last_updated_at` (string): ISO 8601 date-time when the auth config was last updated
    - `no_of_connections` (number) *(required)*: The number of active connections using this auth config
    - `expected_input_fields` (array<object,null>): Fields expected during connection initialization
    - `restrict_to_following_tools` (array<string>): Use tool_access_config instead. This field will be deprecated in the future.
    - `tool_access_config` (object) *(required)*
      - `tools_for_connected_account_creation` (array<string>): Tools used to generate the minimum required scopes for the auth config (only valid for OAuth). If passed, this will update the scopes.
      - `tools_available_for_execution` (array<string>): The actions that the user can perform on the auth config. If passed, this will update the actions that the user can perform on the auth config.
    - `shared_credentials` (object): [EXPERIMENTAL] Shared credentials that will be inherited by all connected accounts using this auth config
    - `is_enabled_for_tool_router` (boolean): Whether this auth config is enabled for tool router
    - `deprecated_params` (object): DEPRECATED: This parameter will be removed in a future version.
      - `default_connector_id` (string,null) *(required)*: Deprecated: Default connector ID
      - `member_uuid` (string): Deprecated: Member UUID
      - `toolkit_id` (string): Deprecated: Toolkit ID
      - `expected_input_fields` (array<object>): Deprecated: Fields expected during connection initialization
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

**Example Response:**

```json
{
  "items": [
    {
      "id": "string",
      "uuid": "string",
      "type": "default",
      "toolkit": {
        "slug": "...",
        "logo": "..."
      },
      "name": "string",
      "auth_scheme": "OAUTH2",
      "is_composio_managed": true,
      "credentials": {},
      "proxy_config": null,
      "status": "ENABLED",
      "created_by": "string",
      "created_at": "string",
      "last_updated_at": "string",
      "no_of_connections": 1,
      "expected_input_fields": [
        "..."
      ],
      "restrict_to_following_tools": [
        "..."
      ],
      "tool_access_config": {
        "tools_for_connected_account_creation": "...",
        "tools_available_for_execution": "..."
      },
      "shared_credentials": {},
      "is_enabled_for_tool_router": true,
      "deprecated_params": {
        "default_connector_id": "...",
        "member_uuid": "...",
        "toolkit_id": "...",
        "expected_input_fields": "..."
      }
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
}
```

#### 400 - Bad request

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

### Example cURL Request

```bash
curl -X GET "https://backend.composio.dev/api/v3/auth_configs" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get single authentication configuration by ID

**Documentation:** /reference/api-reference/auth-configs/getAuthConfigsByNanoid

Retrieves detailed information about a specific authentication configuration using its unique identifier.

---

## GET `/api/v3/auth_configs/{nanoid}`

**Endpoint:** `https://backend.composio.dev/api/v3/auth_configs/{nanoid}`

**Summary:** Get single authentication configuration by ID

Retrieves detailed information about a specific authentication configuration using its unique identifier.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (authConfigId)) *(required)*: The unique identifier of the authentication configuration to retrieve

### Responses

#### 200 - Successfully fetched auth config

**Response Schema:**

- `id` (string (authConfigId)) *(required)*: The unique ID of the authentication configuration
- `uuid` (string) *(required)*: The UUID of the authentication configuration (for backward compatibility)
- `type` (enum: "default" | "custom") *(required)*: The type of the authentication configuration (custom or default)
- `toolkit` (object) *(required)*: Information about the associated integration
  - `slug` (string) *(required)*: The unique identifier of the integration app
  - `logo` (string) *(required)*: The URL to the integration app's logo image
- `name` (string) *(required)*: The display name of the authentication configuration
- `auth_scheme` (enum: "OAUTH2" | "OAUTH1" | "API_KEY" | ...): The authentication scheme used (e.g., OAuth2, API Key, etc.)
- `is_composio_managed` (boolean): Whether this authentication configuration is managed by Composio or the user
- `credentials` (object): The authentication credentials (tokens, keys, etc.) - may be partially hidden for security
- `proxy_config` (object,null)
- `status` (enum: "ENABLED" | "DISABLED") *(required)*: Current status of the authentication configuration
- `created_by` (string): The identifier of the user who created the auth config
- `created_at` (string): ISO 8601 date-time when the auth config was created
- `last_updated_at` (string): ISO 8601 date-time when the auth config was last updated
- `no_of_connections` (number) *(required)*: The number of active connections using this auth config
- `expected_input_fields` (array<object,null>): Fields expected during connection initialization
- `restrict_to_following_tools` (array<string>): Use tool_access_config instead. This field will be deprecated in the future.
- `tool_access_config` (object) *(required)*
  - `tools_for_connected_account_creation` (array<string>): Tools used to generate the minimum required scopes for the auth config (only valid for OAuth). If passed, this will update the scopes.
  - `tools_available_for_execution` (array<string>): The actions that the user can perform on the auth config. If passed, this will update the actions that the user can perform on the auth config.
- `shared_credentials` (object): [EXPERIMENTAL] Shared credentials that will be inherited by all connected accounts using this auth config
- `is_enabled_for_tool_router` (boolean): Whether this auth config is enabled for tool router
- `deprecated_params` (object): DEPRECATED: This parameter will be removed in a future version.
  - `default_connector_id` (string,null) *(required)*: Deprecated: Default connector ID
  - `member_uuid` (string): Deprecated: Member UUID
  - `toolkit_id` (string): Deprecated: Toolkit ID
  - `expected_input_fields` (array<object>): Deprecated: Fields expected during connection initialization

**Example Response:**

```json
{
  "id": "string",
  "uuid": "string",
  "type": "default",
  "toolkit": {
    "slug": "string",
    "logo": "string"
  },
  "name": "string",
  "auth_scheme": "OAUTH2",
  "is_composio_managed": true,
  "credentials": {},
  "proxy_config": null,
  "status": "ENABLED",
  "created_by": "string",
  "created_at": "string",
  "last_updated_at": "string",
  "no_of_connections": 1,
  "expected_input_fields": [
    null
  ],
  "restrict_to_following_tools": [
    "string"
  ],
  "tool_access_config": {
    "tools_for_connected_account_creation": [],
    "tools_available_for_execution": []
  },
  "shared_credentials": {},
  "is_enabled_for_tool_router": true,
  "deprecated_params": {
    "default_connector_id": null,
    "member_uuid": "string",
    "toolkit_id": "string",
    "expected_input_fields": [
      {}
    ]
  }
}
```

#### 400 - Bad request - Invalid auth config ID format

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication required

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Auth configuration does not exist

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error - Something went wrong on the server

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

### Example cURL Request

```bash
curl -X GET "https://backend.composio.dev/api/v3/auth_configs/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# Update an authentication configuration

**Documentation:** /reference/api-reference/auth-configs/patchAuthConfigsByNanoid

Modifies an existing authentication configuration with new credentials or other settings. Only specified fields will be updated.

---

## PATCH `/api/v3/auth_configs/{nanoid}`

**Endpoint:** `https://backend.composio.dev/api/v3/auth_configs/{nanoid}`

**Summary:** Update an authentication configuration

Modifies an existing authentication configuration with new credentials or other settings. Only specified fields will be updated.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (authConfigId)) *(required)*: The unique identifier of the authentication configuration to update

### Request Body

**Schema:**

*One of:*
  - `type` (enum: "custom") *(required)*
  - `name` (string): The display name of the authentication configuration
  - `credentials` (object)
    - `scopes` (any)
    - `user_scopes` (any)
  - `proxy_config` (object,null)
  - `restrict_to_following_tools` (array<string>): Use tool_access_config instead. This field will be deprecated in the future.
  - `tool_access_config` (object)
    - `tools_for_connected_account_creation` (array<string>): Tools used to generate the minimum required scopes for the auth config (only valid for OAuth). If passed, this will update the scopes.
    - `tools_available_for_execution` (array<string>): The actions that the user can perform on the auth config. If passed, this will update the actions that the user can perform on the auth config.
  - `shared_credentials` (object): Shared credentials that will be inherited by connected accounts. For eg: this can be used to share the API key for a tool with all connected accounts using this auth config.
  - `is_enabled_for_tool_router` (boolean): Whether this auth config is enabled for tool router
  - `type` (enum: "default") *(required)*
  - `name` (string): The display name of the authentication configuration
  - `scopes` (any)
  - `restrict_to_following_tools` (array<string>): Use tool_access_config instead. This field will be deprecated in the future.
  - `tool_access_config` (object)
    - `tools_for_connected_account_creation` (array<string>): Tools used to generate the minimum required scopes for the auth config (only valid for OAuth). If passed, this will update the scopes.
    - `tools_available_for_execution` (array<string>): The actions that the user can perform on the auth config. If passed, this will update the actions that the user can perform on the auth config.
  - `shared_credentials` (object): Shared credentials that will be inherited by connected accounts. For eg: this can be used to share the API key for a tool with all connected accounts using this auth config.
  - `is_enabled_for_tool_router` (boolean): Whether this auth config is enabled for tool router

**Example:**

```json
null
```

### Responses

#### 200 - Successfully updated auth config

**Response Schema:**


#### 400 - Bad request - Invalid input or credentials format

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication required

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Auth configuration does not exist

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error - Something went wrong on the server

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

### Example cURL Request

```bash
curl -X PATCH "https://backend.composio.dev/api/v3/auth_configs/string" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d 'null'
```

# Enable or disable an authentication configuration

**Documentation:** /reference/api-reference/auth-configs/patchAuthConfigsByNanoidByStatus

Updates the status of an authentication configuration to either enabled or disabled. Disabled configurations cannot be used for new connections.

---

## PATCH `/api/v3/auth_configs/{nanoid}/{status}`

**Endpoint:** `https://backend.composio.dev/api/v3/auth_configs/{nanoid}/{status}`

**Summary:** Enable or disable an authentication configuration

Updates the status of an authentication configuration to either enabled or disabled. Disabled configurations cannot be used for new connections.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (authConfigId)) *(required)*: The unique identifier of the authentication configuration to update
- `status` (enum: "ENABLED" | "DISABLED") *(required)*: The new status to set for the auth configuration

### Responses

#### 200 - Successfully updated auth config status

**Response Schema:**


#### 400 - Bad request - Invalid auth config ID or status value

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication required

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Auth configuration does not exist

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error - Something went wrong on the server

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

### Example cURL Request

```bash
curl -X PATCH "https://backend.composio.dev/api/v3/auth_configs/string/ENABLED" \
  -H "x-api-key: YOUR_API_KEY"
```

# Create new authentication configuration

**Documentation:** /reference/api-reference/auth-configs/postAuthConfigs

Creates a new auth config for a toolkit, allowing you to use your own OAuth credentials or API keys instead of Composio-managed authentication. This is required when you want to use custom OAuth apps (bring your own client ID/secret) or configure specific authentication parameters for a toolkit.

---

## POST `/api/v3/auth_configs`

**Endpoint:** `https://backend.composio.dev/api/v3/auth_configs`

**Summary:** Create new authentication configuration

Creates a new auth config for a toolkit, allowing you to use your own OAuth credentials or API keys instead of Composio-managed authentication. This is required when you want to use custom OAuth apps (bring your own client ID/secret) or configure specific authentication parameters for a toolkit.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `toolkit` (object) *(required)*
  - `slug` (string) *(required)*: Toolkit slug to create auth config for
- `auth_config` (any)

**Example:**

```json
{
  "toolkit": {
    "slug": "string"
  },
  "auth_config": {
    "type": "use_composio_managed_auth",
    "credentials": {},
    "restrict_to_following_tools": []
  }
}
```

### Responses

#### 201 - Successfully created auth config

**Response Schema:**

- `toolkit` (object) *(required)*
  - `slug` (string) *(required)*: The unique key of the toolkit
- `auth_config` (object) *(required)*
  - `id` (string (authConfigId)) *(required)*: The auth config id of the toolkit (must be a valid auth config id)
  - `auth_scheme` (string) *(required)*: The authentication mode of the toolkit
  - `is_composio_managed` (boolean) *(required)*: Whether the auth config is managed by Composio
  - `restrict_to_following_tools` (array<string>): The tools that the user can use with the auth config

**Example Response:**

```json
{
  "toolkit": {
    "slug": "string"
  },
  "auth_config": {
    "id": "string",
    "auth_scheme": "string",
    "is_composio_managed": true,
    "restrict_to_following_tools": []
  }
}
```

#### 400 - Bad request

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

### Example cURL Request

```bash
curl -X POST "https://backend.composio.dev/api/v3/auth_configs" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "toolkit": {
      "slug": "string"
    },
    "auth_config": {
      "type": "use_composio_managed_auth",
      "credentials": {},
      "restrict_to_following_tools": []
    }
  }'
```