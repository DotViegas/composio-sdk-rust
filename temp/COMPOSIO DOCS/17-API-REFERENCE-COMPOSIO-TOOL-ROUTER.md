# Tool Router (/reference/api-reference/tool-router)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

(Labs) Tool router endpoints

# Endpoints

| Endpoint                                                                       | Quick Link                                                                                                                                          |
| ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `POST /api/v3/tool_router/session`                                             | [Create a new tool router session](/reference/api-reference/tool-router/postToolRouterSession)                                                      |
| `POST /api/v3/tool_router/session/{session_id}/execute`                        | [Execute a tool within a tool router session](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdExecute)                         |
| `POST /api/v3/tool_router/session/{session_id}/execute_meta`                   | [Execute a meta tool within a tool router session](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdExecuteMeta)                |
| `GET /api/v3/tool_router/session/{session_id}`                                 | [Get a tool router session by ID](/reference/api-reference/tool-router/getToolRouterSessionBySessionId)                                             |
| `POST /api/v3/tool_router/session/{session_id}/link`                           | [Create a link session for a toolkit in a tool router session](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdLink)           |
| `GET /api/v3/tool_router/session/{session_id}/toolkits`                        | [Get toolkits for a tool router session](/reference/api-reference/tool-router/getToolRouterSessionBySessionIdToolkits)                              |
| `GET /api/v3/tool_router/session/{session_id}/tools`                           | [List meta tools with schemas for a tool router session](/reference/api-reference/tool-router/getToolRouterSessionBySessionIdTools)                 |
| `POST /api/v3/tool_router/session/{session_id}/search`                         | [Search for tools using a query](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdSearch)                                       |
| `GET /api/v3/tool_router/session/{session_id}/mounts/{mount_id}/items`         | [List files in a session mount](/reference/api-reference/tool-router/getToolRouterSessionBySessionIdMountsByMountIdItems)                           |
| `POST /api/v3/tool_router/session/{session_id}/mounts/{mount_id}/download_url` | [Create a presigned download URL for a mount file](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdMountsByMountIdDownloadUrl) |
| `POST /api/v3/tool_router/session/{session_id}/mounts/{mount_id}/upload_url`   | [Create a presigned upload URL for a mount file](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdMountsByMountIdUploadUrl)     |
| `POST /api/v3/tool_router/session/{session_id}/mounts/{mount_id}/delete`       | [Delete a file from a session mount](/reference/api-reference/tool-router/postToolRouterSessionBySessionIdMountsByMountIdDelete)                    |

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
# Get a tool router session by ID

**Documentation:** /reference/api-reference/tool-router/getToolRouterSessionBySessionId

Retrieves an existing tool router session by its ID. Returns the session configuration, MCP server URL, and available tools.

---

## GET `/api/v3/tool_router/session/{session_id}`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}`

**Summary:** Get a tool router session by ID

Retrieves an existing tool router session by its ID. Returns the session configuration, MCP server URL, and available tools.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session

### Responses

#### 200 - Session successfully retrieved. Returns the session details including configuration.

**Response Schema:**

- `session_id` (string (toolRouterSessionId)) *(required)*: The identifier of the session
- `mcp` (object) *(required)*
  - `type` (enum: "http") *(required)*: The type of the MCP server. Can be http
  - `url` (string (uri)) *(required)*: The URL of the MCP server
- `tool_router_tools` (array<string>) *(required)*: List of available tools in this session
- `config` (object) *(required)*: The session configuration including user, toolkits, and overrides
  - `user_id` (string) *(required)*: User identifier for this session
  - `toolkits` (any): Toolkit configuration - either enabled list or disabled list
  - `auth_configs` (object): Auth config overrides per toolkit
  - `connected_accounts` (object): Connected account overrides per toolkit
  - `manage_connections` (object): Manage connections configuration
    - `enabled` (boolean): Whether to enable the connection manager for automatic connection handling
    - `callback_url` (string (uri)): Custom callback URL for connected account auth flows
    - `enable_wait_for_connections` (boolean): Enable the COMPOSIO_WAIT_FOR_CONNECTIONS tool for polling connection status. Default false. May not work reliably with GPT models.
  - `tools` (object): Tool-level configuration per toolkit
  - `tags` (object): MCP tool annotation hints for filtering tools with enabled/disabled support. enabled: tags that the tool must have at least one of. disabled: tags that the tool must NOT have any of. Both conditions must be satisfied.
    - `enabled` (array<enum: "readOnlyHint" | "destructiveHint" | "idempotentHint" | ...>): Tags that the tool must have at least one of
    - `disabled` (array<enum: "readOnlyHint" | "destructiveHint" | "idempotentHint" | ...>): Tags that the tool must NOT have any of
  - `workbench` (object): Workbench configuration
    - `proxy_execution_enabled` (boolean): Whether proxy execution is enabled in the workbench
    - `auto_offload_threshold` (number): Character threshold after which tool execution response are saved to a file in workbench. Default is 20k.

**Example Response:**

```json
{
  "session_id": "string",
  "mcp": {
    "type": "http",
    "url": "https://example.com"
  },
  "tool_router_tools": [
    "string"
  ],
  "config": {
    "user_id": "string",
    "toolkits": null,
    "auth_configs": {},
    "connected_accounts": {},
    "manage_connections": {
      "enabled": true,
      "callback_url": "https://example.com",
      "enable_wait_for_connections": false
    },
    "tools": {},
    "tags": {
      "enabled": [
        "..."
      ],
      "disabled": [
        "..."
      ]
    },
    "workbench": {
      "proxy_execution_enabled": true
    }
  }
}
```

#### 400 - Bad request. This may occur if the session_id format is invalid, please pass this in trs_ prefix format

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized. Authentication is required or the provided credentials are invalid.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found. The session with the provided ID does not exist or has been deleted.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error. An unexpected error occurred while processing the request.

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
curl -X GET "https://backend.composio.dev/api/v3/tool_router/session/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List files in a session mount

**Documentation:** /reference/api-reference/tool-router/getToolRouterSessionBySessionIdMountsByMountIdItems

Lists files in a workbench session storage mount with cursor-based pagination. Use the download_url endpoint with the returned mount_relative_path to get a presigned download URL.

---

## GET `/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/items`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/items`

**Summary:** List files in a session mount

Lists files in a workbench session storage mount with cursor-based pagination. Use the download_url endpoint with the returned mount_relative_path to get a presigned download URL.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session
- `mount_id` (string) *(required)*: ID of the storage mount

### Query Parameters

- `cursor` (string): Pagination cursor from the previous response next_cursor field
- `limit` (number): Maximum number of files to return per page (1-500)
- `mount_relative_prefix` (string): Relative path prefix within the mount for filtering

### Responses

#### 200 - Files listed successfully

**Response Schema:**

- `items` (array<object>) *(required)*: List of files in the mount
  - Array items:
    - `mount_relative_path` (string) *(required)*: Relative file path within the mount (e.g. "report.pdf")
    - `sandbox_mount_prefix` (string) *(required)*: Absolute mount path inside the sandbox (e.g. /mnt/files)
    - `size` (number) *(required)*: File size in bytes
    - `last_modified` (string) *(required)*: ISO 8601 timestamp of last modification
- `next_cursor` (string): Cursor for the next page of results. If absent, there are no more pages.

**Example Response:**

```json
{
  "items": [
    {
      "mount_relative_path": "string",
      "sandbox_mount_prefix": "string",
      "size": 1,
      "last_modified": "string"
    }
  ],
  "next_cursor": "string"
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

#### 404 - Session or mount not found

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
curl -X GET "https://backend.composio.dev/api/v3/tool_router/session/string/mounts/string/items" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get toolkits for a tool router session

**Documentation:** /reference/api-reference/tool-router/getToolRouterSessionBySessionIdToolkits

Retrieves a cursor-paginated list of toolkits available in the tool router session. Includes toolkit metadata, composio-managed auth schemes, and connected accounts if available. Optionally filter by specific toolkit slugs.

---

## GET `/api/v3/tool_router/session/{session_id}/toolkits`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/toolkits`

**Summary:** Get toolkits for a tool router session

Retrieves a cursor-paginated list of toolkits available in the tool router session. Includes toolkit metadata, composio-managed auth schemes, and connected accounts if available. Optionally filter by specific toolkit slugs.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session

### Query Parameters

- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.
- `toolkits` (array,null): Optional comma-separated list of toolkit slugs to filter by. If provided, only these toolkits will be returned, overriding the session configuration.
- `is_connected` (boolean,null): Whether to filter by connected toolkits. If provided, only connected toolkits will be returned.
- `search` (string): Search query to filter toolkits by name, slug, or description

### Responses

#### 200 - Toolkits successfully retrieved. Returns a paginated list of toolkits with their metadata and connected accounts.

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `name` (string) *(required)*: Display name of the toolkit
    - `slug` (string) *(required)*: Unique slug identifier
    - `enabled` (boolean) *(required)*: Whether the toolkit is enabled
    - `is_no_auth` (boolean) *(required)*: Whether the toolkit is no-auth
    - `composio_managed_auth_schemes` (array<string>) *(required)*: Available Composio-managed auth schemes
    - `meta` (object) *(required)*: Toolkit metadata
      - `logo` (string (uri)) *(required)*: URL to the toolkit logo
      - `description` (string) *(required)*: Description of the toolkit
    - `connected_account` (object,null) *(required)*: Connected account if available
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

**Example Response:**

```json
{
  "items": [
    {
      "name": "string",
      "slug": "string",
      "enabled": true,
      "is_no_auth": true,
      "composio_managed_auth_schemes": [
        "..."
      ],
      "meta": {
        "logo": "...",
        "description": "..."
      },
      "connected_account": null
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
}
```

#### 400 - Bad request. This may occur if pagination parameters are invalid or the session_id format is incorrect.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized. Authentication is required or the provided credentials are invalid.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden. The session does not belong to the authenticated user.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found. The session with the provided ID does not exist or has been deleted.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error. An unexpected error occurred while processing the request.

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
curl -X GET "https://backend.composio.dev/api/v3/tool_router/session/string/toolkits" \
  -H "x-api-key: YOUR_API_KEY"
```

# List meta tools with schemas for a tool router session

**Documentation:** /reference/api-reference/tool-router/getToolRouterSessionBySessionIdTools

Returns the meta tools available in a tool router session with their complete schemas. This includes request and response schemas specific to the session context.

---

## GET `/api/v3/tool_router/session/{session_id}/tools`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/tools`

**Summary:** List meta tools with schemas for a tool router session

Returns the meta tools available in a tool router session with their complete schemas. This includes request and response schemas specific to the session context.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: Tool router session ID

### Request Body

**Schema:**


**Example:**

```json
{}
```

### Responses

#### 200 - Successfully retrieved meta tools with their complete schemas.

**Response Schema:**

- `items` (array<object>) *(required)*: List of tools with their complete schemas
  - Array items:
    - `slug` (string) *(required)*: Unique identifier for the tool
    - `name` (string) *(required)*: Human-readable display name of the tool
    - `description` (string) *(required)*: Detailed explanation of the tool's functionality and purpose
    - `toolkit` (object) *(required)*
      - `slug` (string) *(required)*: Unique identifier of the parent toolkit
      - `name` (string) *(required)*: Human-readable name of the parent toolkit
      - `logo` (string) *(required)*: URL to the toolkit logo image
    - `input_parameters` (object) *(required)*: Schema definition of required input parameters for the tool
    - `no_auth` (boolean) *(required)*: Indicates if the tool can be used without authentication
    - `available_versions` (array<string>) *(required)*: List of all available versions for this tool
    - `version` (string) *(required)*: Current version of the tool
    - `output_parameters` (object) *(required)*: Schema definition of return values from the tool
    - `scopes` (array<string>) *(required)*: List of scopes associated with the tool
    - `tags` (array<string>) *(required)*: List of tags associated with the tool for categorization and filtering
    - `human_description` (string): Human-friendly description of the tool, if available
    - `is_deprecated` (boolean) *(required)*: Indicates if this tool is deprecated and may be removed in the future
    - `deprecated` (object) *(required)*
      - `displayName` (string) *(required)*: The display name of the tool
      - `version` (string) *(required)*: Current version identifier of the tool
      - `available_versions` (array<string>) *(required)*: List of all available versions for this tool
      - `is_deprecated` (boolean) *(required)*: Indicates if this tool is deprecated and may be removed in the future
      - `toolkit` (object) *(required)*
        - `logo` (string) *(required)*: URL to the toolkit logo image

**Example Response:**

```json
{
  "items": [
    {
      "slug": "string",
      "name": "string",
      "description": "string",
      "toolkit": {
        "slug": "...",
        "name": "...",
        "logo": "..."
      },
      "input_parameters": {},
      "no_auth": true,
      "available_versions": [
        "..."
      ],
      "version": "string",
      "output_parameters": {},
      "scopes": [
        "..."
      ],
      "tags": [
        "..."
      ],
      "human_description": "string",
      "is_deprecated": true,
      "deprecated": {
        "displayName": "...",
        "version": "...",
        "available_versions": "...",
        "is_deprecated": "...",
        "toolkit": "..."
      }
    }
  ]
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

#### 403 - Forbidden

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
curl -X GET "https://backend.composio.dev/api/v3/tool_router/session/string/tools" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{}'
```

# Create a new tool router session

**Documentation:** /reference/api-reference/tool-router/postToolRouterSession

Creates a new session for the tool router feature. This endpoint initializes a new session with specified toolkits and their authentication configurations. The session provides an isolated environment for testing and managing tool routing logic with scoped MCP server access.

---

## POST `/api/v3/tool_router/session`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session`

**Summary:** Create a new tool router session

Creates a new session for the tool router feature. This endpoint initializes a new session with specified toolkits and their authentication configurations. The session provides an isolated environment for testing and managing tool routing logic with scoped MCP server access.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `user_id` (string) *(required)*: The identifier of the user who is initiating the session, ideally a unique identifier from your database like a user ID or email address
- `toolkits` (any): Toolkit configuration - specify either enable toolkits (allowlist) or disable toolkits (denylist). Mutually exclusive.
- `auth_configs` (object): The auth configs to use for the session. This will override the default behavior and use the given auth config when specific toolkits are being executed
- `connected_accounts` (object): The connected accounts to use for the session. This will override the default behaviour and use the given connected account when specific toolkits are being executed
- `manage_connections` (object): Configuration for connection management settings
  - `enable` (boolean,null): Whether to enable the connection manager for automatic connection handling. If true, we will provide a tool your agent can use to initiate connections to toolkits if it doesnt exist. If set to false, then you have to manage connections manually.
  - `callback_url` (string (uri)): The URL to redirect to after a user completes authentication for a connected account. This allows you to handle the auth callback in your own application.
  - `enable_wait_for_connections` (boolean,null): When true, the COMPOSIO_WAIT_FOR_CONNECTIONS tool is available for agents to poll connection status after sharing auth URLs. Default is false (disabled). May not work reliably with GPT models.
- `tools` (object): Tool-level configuration per toolkit - either specify enable tools (whitelist), disable tools (blacklist), or filter by MCP tags for each toolkit
- `tags` (any): Global MCP tool annotation hints for filtering. Array format is treated as enabled list. Object format supports both enabled (tool must have at least one) and disabled (tool must NOT have any) lists. Toolkit-level tags override this. Toolkit enabled/disabled lists take precedence over tag filtering.
- `workbench` (object): Configuration for workbench behavior
  - `enable_proxy_execution` (boolean): Whether proxy execution is enabled. When enabled, workbench can call URLs and APIs directly.
  - `auto_offload_threshold` (number): Character threshold for automatic offloading. When workbench response exceeds this threshold, it will be automatically offloaded. Default is picked automatically based on the response size.
- `experimental` (object): Experimental features - not stable, may be modified or removed in future versions.
  - `assistive_prompt_config` (object): Customize assistive prompt generation (e.g., timezone).
    - `user_timezone` (string): IANA timezone identifier (e.g., 'America/New_York', 'Europe/London'). Used to customize the system prompt with timezone-aware instructions.

**Example:**

```json
{
  "user_id": "string",
  "toolkits": null,
  "auth_configs": {},
  "connected_accounts": {},
  "manage_connections": {
    "enable": true,
    "enable_wait_for_connections": false
  },
  "tools": {},
  "tags": null,
  "workbench": {
    "enable_proxy_execution": true
  },
  "experimental": {
    "assistive_prompt_config": {
      "user_timezone": "string"
    }
  }
}
```

### Responses

#### 201 - Session successfully created. Returns the session ID and MCP server URL for the created session.

**Response Schema:**

- `session_id` (string (toolRouterSessionId)) *(required)*: The identifier of the session
- `mcp` (object) *(required)*
  - `type` (enum: "http") *(required)*: The type of the MCP server. Can be http
  - `url` (string (uri)) *(required)*: The URL of the MCP server
- `tool_router_tools` (array<string>) *(required)*: List of available tools in this session
- `config` (object) *(required)*: The session configuration including user, toolkits, and overrides
  - `user_id` (string) *(required)*: User identifier for this session
  - `toolkits` (any): Toolkit configuration - either enabled list or disabled list
  - `auth_configs` (object): Auth config overrides per toolkit
  - `connected_accounts` (object): Connected account overrides per toolkit
  - `manage_connections` (object): Manage connections configuration
    - `enabled` (boolean): Whether to enable the connection manager for automatic connection handling
    - `callback_url` (string (uri)): Custom callback URL for connected account auth flows
    - `enable_wait_for_connections` (boolean): Enable the COMPOSIO_WAIT_FOR_CONNECTIONS tool for polling connection status. Default false. May not work reliably with GPT models.
  - `tools` (object): Tool-level configuration per toolkit
  - `tags` (object): MCP tool annotation hints for filtering tools with enabled/disabled support. enabled: tags that the tool must have at least one of. disabled: tags that the tool must NOT have any of. Both conditions must be satisfied.
    - `enabled` (array<enum: "readOnlyHint" | "destructiveHint" | "idempotentHint" | ...>): Tags that the tool must have at least one of
    - `disabled` (array<enum: "readOnlyHint" | "destructiveHint" | "idempotentHint" | ...>): Tags that the tool must NOT have any of
  - `workbench` (object): Workbench configuration
    - `proxy_execution_enabled` (boolean): Whether proxy execution is enabled in the workbench
    - `auto_offload_threshold` (number): Character threshold after which tool execution response are saved to a file in workbench. Default is 20k.
- `experimental` (object): Experimental features including the generated system prompt. Only returned on session creation, not on GET.
  - `assistive_prompt` (string) *(required)*: The assistive system prompt to inject into your agent for optimal tool router usage

**Example Response:**

```json
{
  "session_id": "string",
  "mcp": {
    "type": "http",
    "url": "https://example.com"
  },
  "tool_router_tools": [
    "string"
  ],
  "config": {
    "user_id": "string",
    "toolkits": null,
    "auth_configs": {},
    "connected_accounts": {},
    "manage_connections": {
      "enabled": true,
      "callback_url": "https://example.com",
      "enable_wait_for_connections": false
    },
    "tools": {},
    "tags": {
      "enabled": [
        "..."
      ],
      "disabled": [
        "..."
      ]
    },
    "workbench": {
      "proxy_execution_enabled": true
    }
  },
  "experimental": {
    "assistive_prompt": "string"
  }
}
```

#### 400 - Bad request. This may occur if the user_id format is invalid, toolkit names are invalid, or auth_config_id IDs are malformed.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized. Authentication is required or the provided credentials are invalid.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden. The user is not authorized to create a tool router session.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error. An unexpected error occurred while processing the request.

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "string",
    "toolkits": null,
    "auth_configs": {},
    "connected_accounts": {},
    "manage_connections": {
      "enable": true,
      "enable_wait_for_connections": false
    },
    "tools": {},
    "tags": null,
    "workbench": {
      "enable_proxy_execution": true
    },
    "experimental": {
      "assistive_prompt_config": {
        "user_timezone": "string"
      }
    }
  }'
```

# Execute a tool within a tool router session

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdExecute

Executes a specific tool within a tool router session. The toolkit is automatically inferred from the tool slug. The tool must belong to an allowed toolkit and must not be disabled in the session configuration. This endpoint validates permissions, resolves connected accounts, and executes the tool with the session context.

---

## POST `/api/v3/tool_router/session/{session_id}/execute`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/execute`

**Summary:** Execute a tool within a tool router session

Executes a specific tool within a tool router session. The toolkit is automatically inferred from the tool slug. The tool must belong to an allowed toolkit and must not be disabled in the session configuration. This endpoint validates permissions, resolves connected accounts, and executes the tool with the session context.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: Tool router session ID (required for public API, optional for internal - injected by middleware)

### Request Body

**Schema:**

- `tool_slug` (string) *(required)*: The unique slug identifier of the tool to execute
- `arguments` (object): The arguments required by the tool

**Example:**

```json
{
  "tool_slug": "string",
  "arguments": {}
}
```

### Responses

#### 200 - Successfully executed the tool. Returns execution result, logs, and status.

**Response Schema:**

- `data` (object) *(required)*: The data returned by the tool execution
- `error` (string,null) *(required)*: Error message if the execution failed, null otherwise
- `log_id` (string) *(required)*: Unique identifier for the execution log

**Example Response:**

```json
{
  "data": {},
  "error": null,
  "log_id": "string"
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

#### 403 - Forbidden

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/execute" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "tool_slug": "string",
    "arguments": {}
  }'
```

# Execute a meta tool within a tool router session

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdExecuteMeta

Executes a Composio meta tool (COMPOSIO_*) within a tool router session.

---

## POST `/api/v3/tool_router/session/{session_id}/execute_meta`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/execute_meta`

**Summary:** Execute a meta tool within a tool router session

Executes a Composio meta tool (COMPOSIO_*) within a tool router session.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: Tool router session ID (required for public API, optional for internal - injected by middleware)

### Request Body

**Schema:**

- `slug` (enum: "COMPOSIO_SEARCH_TOOLS" | "COMPOSIO_MULTI_EXECUTE_TOOL" | "COMPOSIO_MANAGE_CONNECTIONS" | ...) *(required)*: The unique slug identifier of the meta tool to execute
- `arguments` (object): The arguments required by the meta tool

**Example:**

```json
{
  "slug": "COMPOSIO_SEARCH_TOOLS",
  "arguments": {}
}
```

### Responses

#### 200 - Successfully executed the meta tool. Returns execution result, logs, and status.

**Response Schema:**

- `data` (object) *(required)*: The data returned by the tool execution
- `error` (string,null) *(required)*: Error message if the execution failed, null otherwise
- `log_id` (string) *(required)*: Unique identifier for the execution log

**Example Response:**

```json
{
  "data": {},
  "error": null,
  "log_id": "string"
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

#### 403 - Forbidden

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/execute_meta" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "slug": "COMPOSIO_SEARCH_TOOLS",
    "arguments": {}
  }'
```

# Create a link session for a toolkit in a tool router session

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdLink

Initiates an authentication link session for a specific toolkit within a tool router session. Returns a link token and redirect URL that users can use to complete the OAuth flow.

---

## POST `/api/v3/tool_router/session/{session_id}/link`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/link`

**Summary:** Create a link session for a toolkit in a tool router session

Initiates an authentication link session for a specific toolkit within a tool router session. Returns a link token and redirect URL that users can use to complete the OAuth flow.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session

### Request Body

**Schema:**

- `toolkit` (string) *(required)*: The unique slug identifier of the toolkit to connect
- `callback_url` (string (uri)): URL where users will be redirected after completing auth

**Example:**

```json
{
  "toolkit": "string",
  "callback_url": "https://example.com"
}
```

### Responses

#### 201 - Successfully created link session. Returns link token, redirect URL, and connected account ID.

**Response Schema:**

- `link_token` (string) *(required)*: Token used to complete the authentication flow
- `redirect_url` (string (uri)) *(required)*: The URL where users should be redirected to complete OAuth
- `connected_account_id` (string (connectedAccountId)) *(required)*: The unique identifier for the connected account

**Example Response:**

```json
{
  "link_token": "string",
  "redirect_url": "https://example.com",
  "connected_account_id": "string"
}
```

#### 400 - Bad request. This may occur if the toolkit slug is invalid, request parameters are malformed, or a connected account is already defined for this toolkit in the session configuration.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized. Authentication is required or the provided credentials are invalid.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden. The session does not belong to the authenticated user.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found. The session does not exist, toolkit not found, or no auth config exists for the toolkit.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error. An unexpected error occurred while creating the link session.

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/link" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "toolkit": "string",
    "callback_url": "https://example.com"
  }'
```

# Delete a file from a session mount

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdMountsByMountIdDelete

Deletes a file from a workbench session storage mount. S3 delete is idempotent — deleting a non-existent file succeeds silently.

---

## POST `/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/delete`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/delete`

**Summary:** Delete a file from a session mount

Deletes a file from a workbench session storage mount. S3 delete is idempotent — deleting a non-existent file succeeds silently.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session
- `mount_id` (string) *(required)*: ID of the storage mount

### Request Body

**Schema:**

- `mount_relative_path` (string) *(required)*: Relative file path within the mount

**Example:**

```json
{
  "mount_relative_path": "string"
}
```

### Responses

#### 200 - File deleted successfully

**Response Schema:**

- `mount_relative_path` (string) *(required)*: Relative file path that was deleted
- `sandbox_mount_prefix` (string) *(required)*: Absolute mount path inside the sandbox (e.g. /mnt/files)

**Example Response:**

```json
{
  "mount_relative_path": "string",
  "sandbox_mount_prefix": "string"
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

#### 404 - Session or mount not found

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/mounts/string/delete" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "mount_relative_path": "string"
  }'
```
# Create a presigned download URL for a mount file

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdMountsByMountIdDownloadUrl

Generates a presigned download URL for a file in a workbench session mount. Accepts a relative path within the mount.

---

## POST `/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/download_url`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/download_url`

**Summary:** Create a presigned download URL for a mount file

Generates a presigned download URL for a file in a workbench session mount. Accepts a relative path within the mount.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session
- `mount_id` (string) *(required)*: ID of the storage mount

### Request Body

**Schema:**

- `mount_relative_path` (string) *(required)*: Relative file path within the mount

**Example:**

```json
{
  "mount_relative_path": "string"
}
```

### Responses

#### 201 - Download URL created successfully

**Response Schema:**

- `download_url` (string) *(required)*: Presigned download URL for the file
- `mount_relative_path` (string) *(required)*: Relative file path within the mount (e.g. "report.pdf")
- `sandbox_mount_prefix` (string) *(required)*: Absolute mount path inside the sandbox (e.g. /mnt/files)
- `expires_at` (string) *(required)*: ISO 8601 timestamp when the download URL expires

**Example Response:**

```json
{
  "download_url": "string",
  "mount_relative_path": "string",
  "sandbox_mount_prefix": "string",
  "expires_at": "string"
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

#### 404 - Session or mount not found

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/mounts/string/download_url" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "mount_relative_path": "string"
  }'
```

# Create a presigned upload URL for a mount file

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdMountsByMountIdUploadUrl

Generates a presigned upload URL for uploading a file to a workbench session mount. The caller should PUT the file content directly to the returned URL.

---

## POST `/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/upload_url`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/mounts/{mount_id}/upload_url`

**Summary:** Create a presigned upload URL for a mount file

Generates a presigned upload URL for uploading a file to a workbench session mount. The caller should PUT the file content directly to the returned URL.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: The unique identifier of the tool router session
- `mount_id` (string) *(required)*: ID of the storage mount

### Request Body

**Schema:**

- `mount_relative_path` (string) *(required)*: Supports subdirectories (e.g. "data/output.csv", "images/charts/chart.png")
- `mimetype` (string): MIME type of the file being uploaded

**Example:**

```json
{
  "mount_relative_path": "string",
  "mimetype": "string"
}
```

### Responses

#### 201 - Upload URL created successfully

**Response Schema:**

- `upload_url` (string) *(required)*: Presigned upload URL — PUT the file content here
- `mount_relative_path` (string) *(required)*: Relative file path within the mount (e.g. "report.pdf")
- `sandbox_mount_prefix` (string) *(required)*: Absolute mount path inside the sandbox (e.g. /mnt/files)
- `expires_at` (string) *(required)*: ISO 8601 timestamp when the upload URL expires

**Example Response:**

```json
{
  "upload_url": "string",
  "mount_relative_path": "string",
  "sandbox_mount_prefix": "string",
  "expires_at": "string"
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

#### 404 - Session or mount not found

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/mounts/string/upload_url" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "mount_relative_path": "string",
    "mimetype": "string"
  }'
```

# Search for tools using a query

**Documentation:** /reference/api-reference/tool-router/postToolRouterSessionBySessionIdSearch

Search for tools matching a given use case query within a tool router session. Returns matching tool slugs, full tool schemas, toolkit connection statuses, and workflow guidance in a predictable format.

---

## POST `/api/v3/tool_router/session/{session_id}/search`

**Endpoint:** `https://backend.composio.dev/api/v3/tool_router/session/{session_id}/search`

**Summary:** Search for tools using a query

Search for tools matching a given use case query within a tool router session. Returns matching tool slugs, full tool schemas, toolkit connection statuses, and workflow guidance in a predictable format.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `session_id` (string (toolRouterSessionId)) *(required)*: Tool router session ID (trs_*)

### Request Body

**Schema:**

- `queries` (array<object>) *(required)*: List of search queries to execute in parallel. Up to 7 queries supported.
  - Array items:
    - `use_case` (string) *(required)*: The task or use case to search tools for. Provide a detailed description to get the best results.
    - `known_fields` (string): Known field hints as key:value pairs (e.g., "channel_name:general, user_email:john@example.com")
- `model` (string): Optional model hint for search/planning behavior (e.g., "gpt-4o"). Ignored if invalid.

**Example:**

```json
{
  "queries": [
    {
      "use_case": "string",
      "known_fields": "string"
    }
  ],
  "model": "string"
}
```

### Responses

#### 200 - Successfully executed the search. Returns matching tool slugs, schemas, connection statuses, and guidance.

**Response Schema:**

- `success` (boolean) *(required)*: Whether all searches completed successfully. False if any query failed.
- `error` (string,null) *(required)*: Error message if any searches failed, null if all succeeded. Format: "X out of Y searches failed, reasons: <details>"
- `results` (array<object>) *(required)*: Per-query search results with tools, reasoning, and memory. One entry per query in request order.
  - Array items:
    - `index` (number) *(required)*: 1-based index of the query in the request
    - `use_case` (string) *(required)*: The use case that was searched
    - `execution_guidance` (string): Guidance message about the search results, particularly when a cached plan is available
    - `difficulty` (string): Task difficulty assessment (e.g., "easy - Simple single-tool operation with known parameters")
    - `recommended_plan_steps` (array<string>): Workflow steps from cached plan (only present when cached plan is available)
    - `known_pitfalls` (array<string>): Common pitfalls and considerations (only present when cached plan is available)
    - `reference_workbench_snippets` (array<object>): Reference Python code snippets for processing tool responses in the workbench (only present when cached plan is available)
      - Array items:
        - `description` (string) *(required)*: Description of what the code snippet does
        - `code` (string) *(required)*: Python code snippet for the workbench
    - `primary_tool_slugs` (array<string>) *(required)*: List of main tool slugs matching the search criteria
    - `related_tool_slugs` (array<string>) *(required)*: List of related tool slugs that might be useful
    - `toolkits` (array<string>) *(required)*: List of unique toolkit slugs used by tools in this query
    - `plan_id` (string): ID of cached plan if available
    - `error` (string,null): Error message if the search for this query failed, null otherwise. Always present for failed queries.
    - `memory` (object): Memory data relevant to this query, grouped by app. Only present for non-cached search results.
- `toolkit_connection_statuses` (array<object>) *(required)*: Connection status for all toolkits mentioned across all queries, with descriptions merged in.
  - Array items:
    - `toolkit` (string) *(required)*: The toolkit slug identifier (e.g., "gmail", "slack")
    - `description` (string) *(required)*: Description of what the toolkit does and its capabilities
    - `has_active_connection` (boolean) *(required)*: Whether an active connection exists for this toolkit
    - `connection_details` (object): Connection details including auth config and connected account IDs. Only present when has_active_connection is true.
    - `current_user_info` (object): Information about the currently connected user (email, name, etc.)
    - `status_message` (string) *(required)*: Human-readable message about the connection status and next steps
- `tool_schemas` (object) *(required)*: Deduplicated tool definitions keyed by tool_slug for O(1) lookup. Each tool appears once even if used in multiple queries.
- `time_info` (object) *(required)*: Time information for the query
  - `current_time_utc` (string) *(required)*: Current time in ISO format (UTC)
  - `current_time_utc_epoch_seconds` (number) *(required)*: Current time as Unix epoch timestamp in seconds
  - `message` (string) *(required)*: Important message about time handling and timezone considerations
- `session` (object) *(required)*: Session info for correlating meta tool calls
  - `id` (string) *(required)*: Session identifier to be passed to subsequent meta tool calls as session_id.
  - `generate_id` (boolean) *(required)*: Whether a fresh session id was generated in this call.
  - `instructions` (string) *(required)*: LLM-facing guidance on how to reuse this session id
- `next_steps_guidance` (array<string>) *(required)*: Combined workflow guidance covering connections, planner, and memory usage. Each element is a step instruction.

**Example Response:**

```json
{
  "success": true,
  "error": null,
  "results": [
    {
      "index": 1,
      "use_case": "string",
      "execution_guidance": "string",
      "difficulty": "string",
      "recommended_plan_steps": [
        "..."
      ],
      "known_pitfalls": [
        "..."
      ],
      "reference_workbench_snippets": [
        "..."
      ],
      "primary_tool_slugs": [
        "..."
      ],
      "related_tool_slugs": [
        "..."
      ],
      "toolkits": [
        "..."
      ],
      "plan_id": "string",
      "error": null,
      "memory": {}
    }
  ],
  "toolkit_connection_statuses": [
    {
      "toolkit": "string",
      "description": "string",
      "has_active_connection": true,
      "connection_details": {},
      "current_user_info": {},
      "status_message": "string"
    }
  ],
  "tool_schemas": {},
  "time_info": {
    "current_time_utc": "string",
    "current_time_utc_epoch_seconds": 1,
    "message": "string"
  },
  "session": {
    "id": "string",
    "generate_id": true,
    "instructions": "string"
  },
  "next_steps_guidance": [
    "string"
  ]
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

#### 403 - Forbidden

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
curl -X POST "https://backend.composio.dev/api/v3/tool_router/session/string/search" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "queries": [
      {
        "use_case": "string",
        "known_fields": "string"
      }
    ],
    "model": "string"
  }'
```