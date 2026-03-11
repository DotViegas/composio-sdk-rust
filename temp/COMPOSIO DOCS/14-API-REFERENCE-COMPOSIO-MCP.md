# MCP (/reference/api-reference/mcp)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

MCP server management

# Endpoints

| Endpoint                                                       | Quick Link                                                                                                                                      |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `GET /api/v3/mcp/servers`                                      | [List MCP servers with optional filters and pagination](/reference/api-reference/mcp/getMcpServers)                                             |
| `POST /api/v3/mcp/servers`                                     | [Create a new MCP server](/reference/api-reference/mcp/postMcpServers)                                                                          |
| `POST /api/v3/mcp/servers/custom`                              | [Create a new custom MCP server with multiple apps](/reference/api-reference/mcp/postMcpServersCustom)                                          |
| `POST /api/v3/mcp/servers/generate`                            | [Generate MCP URL with custom parameters](/reference/api-reference/mcp/postMcpServersGenerate)                                                  |
| `GET /api/v3/mcp/{id}`                                         | [Get MCP server details by ID](/reference/api-reference/mcp/getMcpById)                                                                         |
| `PATCH /api/v3/mcp/{id}`                                       | [Update MCP server configuration](/reference/api-reference/mcp/patchMcpById)                                                                    |
| `DELETE /api/v3/mcp/{id}`                                      | [Delete an MCP server](/reference/api-reference/mcp/deleteMcpById)                                                                              |
| `GET /api/v3/mcp/app/{appKey}`                                 | [List MCP servers for a specific app](/reference/api-reference/mcp/getMcpAppByAppKey)                                                           |
| `GET /api/v3/mcp/servers/{serverId}/instances`                 | [List all instances for an MCP server](/reference/api-reference/mcp/getMcpServersByServerIdInstances)                                           |
| `POST /api/v3/mcp/servers/{serverId}/instances`                | [Create a new MCP server instance](/reference/api-reference/mcp/postMcpServersByServerIdInstances)                                              |
| `DELETE /api/v3/mcp/servers/{serverId}/instances/{instanceId}` | [Delete an MCP server instance and associated connected accounts](/reference/api-reference/mcp/deleteMcpServersByServerIdInstancesByInstanceId) |

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
# Delete an MCP server

**Documentation:** /reference/api-reference/mcp/deleteMcpById

Performs a soft delete on a Model Control Protocol (MCP) server, making it unavailable for future use. This operation is reversible in the database but cannot be undone through the API. Any applications or services connected to this server will lose access after deletion.

---

## DELETE `/api/v3/mcp/{id}`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/{id}`

**Summary:** Delete an MCP server

Performs a soft delete on a Model Control Protocol (MCP) server, making it unavailable for future use. This operation is reversible in the database but cannot be undone through the API. Any applications or services connected to this server will lose access after deletion.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string) *(required)*: The ID of the MCP server

### Responses

#### 200 - Successfully deleted MCP server. The server has been soft-deleted and is no longer available for use.

**Response Schema:**

- `id` (string) *(required)*: Unique identifier of the MCP server to retrieve, update, or delete
- `deleted` (boolean) *(required)*: Indicates whether the MCP server was successfully deleted

**Example Response:**

```json
{
  "id": "string",
  "deleted": true
}
```

#### 400 - Bad request. The server ID parameter may be invalid or in an incorrect format.

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

#### 403 - Forbidden. The authenticated user does not have permission to delete this MCP server.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - MCP server not found. No server with the specified ID exists or it has already been deleted.

**Response Schema:**


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
curl -X DELETE "https://backend.composio.dev/api/v3/mcp/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# Delete an MCP server instance and associated connected accounts

**Documentation:** /reference/api-reference/mcp/deleteMcpServersByServerIdInstancesByInstanceId

Removes a user instance from the MCP server and deletes all connected accounts for that user that are associated with the auth configurations of this specific MCP server. Connected accounts for other auth configurations are not affected.

---

## DELETE `/api/v3/mcp/servers/{serverId}/instances/{instanceId}`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers/{serverId}/instances/{instanceId}`

**Summary:** Delete an MCP server instance and associated connected accounts

Removes a user instance from the MCP server and deletes all connected accounts for that user that are associated with the auth configurations of this specific MCP server. Connected accounts for other auth configurations are not affected.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `serverId` (string) *(required)*: The ID of the MCP server
- `instanceId` (string) *(required)*: The instance ID (user ID) to delete

### Responses

#### 200 - Successfully deleted MCP server instance and associated connected accounts.

**Response Schema:**

- `message` (string) *(required)*
- `deleted_connected_accounts` (number) *(required)*

**Example Response:**

```json
{
  "message": "string",
  "deleted_connected_accounts": 1
}
```

#### 400 - Bad request. The request parameters may be invalid.

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

#### 404 - MCP server or instance not found.

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
curl -X DELETE "https://backend.composio.dev/api/v3/mcp/servers/string/instances/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List MCP servers for a specific app

**Documentation:** /reference/api-reference/mcp/getMcpAppByAppKey

Retrieves a paginated list of Model Control Protocol (MCP) servers that are configured for a specific application or toolkit. This endpoint allows you to find all MCP server instances that have access to a particular application, such as GitHub, Slack, or Jira.

---

## GET `/api/v3/mcp/app/{appKey}`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/app/{appKey}`

**Summary:** List MCP servers for a specific app

Retrieves a paginated list of Model Control Protocol (MCP) servers that are configured for a specific application or toolkit. This endpoint allows you to find all MCP server instances that have access to a particular application, such as GitHub, Slack, or Jira.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `appKey` (string) *(required)*: The key of the app to find MCP servers for

### Query Parameters

- `name` (string): Filter MCP servers by name (case-insensitive partial match)
- `toolkits` (string): Comma-separated list of toolkit slugs to filter servers by
- `auth_config_ids` (string): Comma-separated list of auth config IDs to filter servers by
- `order_by` (enum: "created_at" | "updated_at"): Field to order results by
- `order_direction` (enum: "asc" | "desc"): Direction of ordering
- `page_no` (number,null): Page number for pagination (1-based)
- `limit` (number,null): Number of items per page (default: 10)

### Responses

#### 200 - Successfully retrieved MCP servers for the specified application. Returns a paginated list of server configurations including connection details and command instructions.

**Response Schema:**

- `items` (array<object>) *(required)*: Array of MCP server configurations
  - Array items:
    - `id` (string) *(required)*: UUID of the MCP server instance
    - `name` (string) *(required)*: User-defined descriptive name for this MCP server
    - `auth_config_ids` (array<string>) *(required)*: ID references to the auth configurations used by this server
    - `allowed_tools` (array<string>) *(required)*: Array of tool slugs that this MCP server is allowed to use
    - `mcp_url` (string) *(required)*: [DEPRECATED] Please use the URL with user_id or connected_account_id query param
    - `toolkits` (array<string>) *(required)*: Array of toolkit slugs that this MCP server is allowed to use
    - `toolkit_icons` (object) *(required)*: Object mapping each toolkit slug to its icon/logo URL for display purposes
    - `commands` (object) *(required)*: Set of command line instructions for connecting various clients to this MCP server
      - `cursor` (string) *(required)*: Command line instruction for Cursor client setup
      - `claude` (string) *(required)*: Command line instruction for Claude client setup
      - `windsurf` (string) *(required)*: Command line instruction for Windsurf client setup
    - `updated_at` (string) *(required)*: Date and time when this server configuration was last modified
    - `created_at` (string) *(required)*: Date and time when this server was initially created
    - `server_instance_count` (number) *(required)*: Total count of active user instances connected to this server
    - `managed_auth_via_composio` (boolean) *(required)*: Whether the MCP server is managed by Composio
- `total_pages` (number) *(required)*: Total number of pages in the paginated response
- `current_page` (number) *(required)*: Current page number being returned

**Example Response:**

```json
{
  "items": [
    {
      "id": "string",
      "name": "string",
      "auth_config_ids": [
        "..."
      ],
      "allowed_tools": [
        "..."
      ],
      "mcp_url": "string",
      "toolkits": [
        "..."
      ],
      "toolkit_icons": {},
      "commands": {
        "cursor": "...",
        "claude": "...",
        "windsurf": "..."
      },
      "updated_at": "string",
      "created_at": "string",
      "server_instance_count": 1,
      "managed_auth_via_composio": true
    }
  ],
  "total_pages": 1,
  "current_page": 1
}
```

#### 400 - Bad request. The query parameters may be invalid or in an incorrect format.

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

#### 403 - Forbidden. The authenticated user does not have permission to view MCP servers for this application.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Application not found. No application with the specified key exists.

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
curl -X GET "https://backend.composio.dev/api/v3/mcp/app/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get MCP server details by ID

**Documentation:** /reference/api-reference/mcp/getMcpById

Retrieves detailed configuration information for a specific Model Control Protocol (MCP) server. The returned data includes connection details, associated applications, enabled tools, and authentication configuration.

---

## GET `/api/v3/mcp/{id}`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/{id}`

**Summary:** Get MCP server details by ID

Retrieves detailed configuration information for a specific Model Control Protocol (MCP) server. The returned data includes connection details, associated applications, enabled tools, and authentication configuration.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string) *(required)*: The ID of the MCP server

### Responses

#### 200 - Successfully retrieved MCP server. Returns the complete server configuration including connection details, authentication settings, and available tools.

**Response Schema:**

- `id` (string) *(required)*: UUID of the MCP server instance
- `name` (string) *(required)*: User-defined descriptive name for this MCP server
- `auth_config_ids` (array<string>) *(required)*: ID references to the auth configurations used by this server
- `allowed_tools` (array<string>) *(required)*: Array of tool slugs that this MCP server is allowed to use
- `mcp_url` (string) *(required)*: [DEPRECATED] Please use the URL with user_id or connected_account_id query param
- `toolkits` (array<string>) *(required)*: Array of toolkit slugs that this MCP server is allowed to use
- `toolkit_icons` (object) *(required)*: Object mapping each toolkit slug to its icon/logo URL for display purposes
- `commands` (object) *(required)*: Set of command line instructions for connecting various clients to this MCP server
  - `cursor` (string) *(required)*: Command line instruction for Cursor client setup
  - `claude` (string) *(required)*: Command line instruction for Claude client setup
  - `windsurf` (string) *(required)*: Command line instruction for Windsurf client setup
- `updated_at` (string) *(required)*: Date and time when this server configuration was last modified
- `created_at` (string) *(required)*: Date and time when this server was initially created
- `server_instance_count` (number) *(required)*: Total count of active user instances connected to this server
- `managed_auth_via_composio` (boolean) *(required)*: Whether the MCP server is managed by Composio
- `deleted` (boolean) *(required)*: Whether the MCP server is deleted

**Example Response:**

```json
{
  "id": "string",
  "name": "string",
  "auth_config_ids": [
    "string"
  ],
  "allowed_tools": [
    "string"
  ],
  "mcp_url": "string",
  "toolkits": [
    "string"
  ],
  "toolkit_icons": {},
  "commands": {
    "cursor": "string",
    "claude": "string",
    "windsurf": "string"
  },
  "updated_at": "string",
  "created_at": "string",
  "server_instance_count": 1,
  "managed_auth_via_composio": true,
  "deleted": true
}
```

#### 400 - Bad request. The server ID parameter may be invalid or in an incorrect format.

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

#### 403 - Forbidden. The authenticated user does not have permission to view this MCP server.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - MCP server not found. No server with the specified ID exists or it has been deleted.

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
curl -X GET "https://backend.composio.dev/api/v3/mcp/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List MCP servers with optional filters and pagination

**Documentation:** /reference/api-reference/mcp/getMcpServers

Retrieves a paginated list of MCP servers associated with the authenticated project. Results can be filtered by name, toolkit, or authentication configuration ID. MCP servers are used to provide Model Control Protocol integration points for connecting AI assistants to your applications and services.

---

## GET `/api/v3/mcp/servers`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers`

**Summary:** List MCP servers with optional filters and pagination

Retrieves a paginated list of MCP servers associated with the authenticated project. Results can be filtered by name, toolkit, or authentication configuration ID. MCP servers are used to provide Model Control Protocol integration points for connecting AI assistants to your applications and services.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `name` (string): Filter MCP servers by name (case-insensitive partial match)
- `toolkits` (string): Comma-separated list of toolkit slugs to filter servers by
- `auth_config_ids` (string): Comma-separated list of auth config IDs to filter servers by
- `order_by` (enum: "created_at" | "updated_at"): Field to order results by
- `order_direction` (enum: "asc" | "desc"): Direction of ordering
- `page_no` (number,null): Page number for pagination (1-based)
- `limit` (number,null): Number of items per page (default: 10)

### Responses

#### 200 - Successfully retrieved MCP servers. Returns a paginated list of server configurations including connection details and command instructions.

**Response Schema:**

- `items` (array<object>) *(required)*: Array of MCP server configurations
  - Array items:
    - `id` (string) *(required)*: UUID of the MCP server instance
    - `name` (string) *(required)*: User-defined descriptive name for this MCP server
    - `auth_config_ids` (array<string>) *(required)*: ID references to the auth configurations used by this server
    - `allowed_tools` (array<string>) *(required)*: Array of tool slugs that this MCP server is allowed to use
    - `mcp_url` (string) *(required)*: [DEPRECATED] Please use the URL with user_id or connected_account_id query param
    - `toolkits` (array<string>) *(required)*: Array of toolkit slugs that this MCP server is allowed to use
    - `toolkit_icons` (object) *(required)*: Object mapping each toolkit slug to its icon/logo URL for display purposes
    - `commands` (object) *(required)*: Set of command line instructions for connecting various clients to this MCP server
      - `cursor` (string) *(required)*: Command line instruction for Cursor client setup
      - `claude` (string) *(required)*: Command line instruction for Claude client setup
      - `windsurf` (string) *(required)*: Command line instruction for Windsurf client setup
    - `updated_at` (string) *(required)*: Date and time when this server configuration was last modified
    - `created_at` (string) *(required)*: Date and time when this server was initially created
    - `server_instance_count` (number) *(required)*: Total count of active user instances connected to this server
    - `managed_auth_via_composio` (boolean) *(required)*: Whether the MCP server is managed by Composio
- `total_pages` (number) *(required)*: Total number of pages in the paginated response
- `current_page` (number) *(required)*: Current page number being returned

**Example Response:**

```json
{
  "items": [
    {
      "id": "string",
      "name": "string",
      "auth_config_ids": [
        "..."
      ],
      "allowed_tools": [
        "..."
      ],
      "mcp_url": "string",
      "toolkits": [
        "..."
      ],
      "toolkit_icons": {},
      "commands": {
        "cursor": "...",
        "claude": "...",
        "windsurf": "..."
      },
      "updated_at": "string",
      "created_at": "string",
      "server_instance_count": 1,
      "managed_auth_via_composio": true
    }
  ],
  "total_pages": 1,
  "current_page": 1
}
```

#### 400 - Bad request. The query parameters may be invalid or in an incorrect format.

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

#### 403 - Forbidden. The authenticated user does not have permission to view MCP servers for this project.

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
curl -X GET "https://backend.composio.dev/api/v3/mcp/servers" \
  -H "x-api-key: YOUR_API_KEY"
```

# List all instances for an MCP server

**Documentation:** /reference/api-reference/mcp/getMcpServersByServerIdInstances

Retrieves a paginated list of user instances (user IDs) associated with a specific Model Control Protocol (MCP) server. This endpoint supports pagination to handle servers with many instances.

---

## GET `/api/v3/mcp/servers/{serverId}/instances`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers/{serverId}/instances`

**Summary:** List all instances for an MCP server

Retrieves a paginated list of user instances (user IDs) associated with a specific Model Control Protocol (MCP) server. This endpoint supports pagination to handle servers with many instances.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `serverId` (string) *(required)*: The ID of the MCP server

### Query Parameters

- `page_no` (number,null): Page number for pagination (1-based)
- `limit` (number,null): Number of items per page (default: 20)
- `search` (string): Search instances by user ID/instance ID
- `order_by` (enum: "created_at" | "updated_at"): Field to order results by
- `order_direction` (enum: "asc" | "desc"): Direction of ordering

### Responses

#### 200 - Successfully retrieved MCP server instances. Returns the list of user IDs associated with this server.

**Response Schema:**

- `instances` (array<object>) *(required)*: List of instance objects associated with this MCP server for the current page
  - Array items:
    - `id` (string) *(required)*: UUID of the instance record
    - `instance_id` (string) *(required)*: The instance identifier (same as the user_id)
    - `mcp_server_id` (string) *(required)*: UUID of the parent MCP server
    - `created_at` (string) *(required)*: Date and time when this instance was created
    - `updated_at` (string) *(required)*: Date and time when this instance was last modified
- `server_id` (string) *(required)*: UUID of the MCP server
- `server_name` (string) *(required)*: Name of the MCP server
- `total_pages` (number) *(required)*: Total number of pages in the paginated response
- `current_page` (number) *(required)*: Current page number being returned

**Example Response:**

```json
{
  "instances": [
    {
      "id": "string",
      "instance_id": "string",
      "mcp_server_id": "string",
      "created_at": "string",
      "updated_at": "string"
    }
  ],
  "server_id": "string",
  "server_name": "string",
  "total_pages": 1,
  "current_page": 1
}
```

#### 400 - Bad request. The server ID parameter may be invalid.

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

#### 404 - MCP server not found. No server with the specified ID exists.

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
curl -X GET "https://backend.composio.dev/api/v3/mcp/servers/string/instances" \
  -H "x-api-key: YOUR_API_KEY"
```

# Update MCP server configuration

**Documentation:** /reference/api-reference/mcp/patchMcpById

Updates the configuration of an existing Model Control Protocol (MCP) server. You can modify the server name, associated applications, and enabled tools. Only the fields included in the request will be updated.

---

## PATCH `/api/v3/mcp/{id}`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/{id}`

**Summary:** Update MCP server configuration

Updates the configuration of an existing Model Control Protocol (MCP) server. You can modify the server name, associated applications, and enabled tools. Only the fields included in the request will be updated.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string) *(required)*: The ID of the MCP server

### Request Body

**Schema:**

- `name` (string): Human-readable name to identify this MCP server instance (4-30 characters, alphanumeric, spaces, and hyphens only)
- `toolkits` (array<string>): List of toolkit slugs this server should be configured to work with.
- `allowed_tools` (array<string>): List of action identifiers that should be enabled for this server
- `managed_auth_via_composio` (boolean): Whether the MCP server is managed by Composio
- `auth_config_ids` (array<string>): List of auth config IDs to use for this MCP server.

**Example:**

```json
{
  "name": "string",
  "toolkits": [
    "string"
  ],
  "allowed_tools": [
    "string"
  ],
  "managed_auth_via_composio": true,
  "auth_config_ids": [
    "string"
  ]
}
```

### Responses

#### 200 - Successfully updated MCP server. Returns the complete updated server configuration including connection details, authentication settings, and available tools.

**Response Schema:**

- `id` (string) *(required)*: UUID of the MCP server instance
- `name` (string) *(required)*: User-defined descriptive name for this MCP server
- `auth_config_ids` (array<string>) *(required)*: ID references to the auth configurations used by this server
- `allowed_tools` (array<string>) *(required)*: Array of tool slugs that this MCP server is allowed to use
- `mcp_url` (string) *(required)*: [DEPRECATED] Please use the URL with user_id or connected_account_id query param
- `toolkits` (array<string>) *(required)*: Array of toolkit slugs that this MCP server is allowed to use
- `toolkit_icons` (object) *(required)*: Object mapping each toolkit slug to its icon/logo URL for display purposes
- `commands` (object) *(required)*: Set of command line instructions for connecting various clients to this MCP server
  - `cursor` (string) *(required)*: Command line instruction for Cursor client setup
  - `claude` (string) *(required)*: Command line instruction for Claude client setup
  - `windsurf` (string) *(required)*: Command line instruction for Windsurf client setup
- `updated_at` (string) *(required)*: Date and time when this server configuration was last modified
- `created_at` (string) *(required)*: Date and time when this server was initially created
- `server_instance_count` (number) *(required)*: Total count of active user instances connected to this server
- `managed_auth_via_composio` (boolean) *(required)*: Whether the MCP server is managed by Composio
- `deleted` (boolean) *(required)*: Whether the MCP server is deleted

**Example Response:**

```json
{
  "id": "string",
  "name": "string",
  "auth_config_ids": [
    "string"
  ],
  "allowed_tools": [
    "string"
  ],
  "mcp_url": "string",
  "toolkits": [
    "string"
  ],
  "toolkit_icons": {},
  "commands": {
    "cursor": "string",
    "claude": "string",
    "windsurf": "string"
  },
  "updated_at": "string",
  "created_at": "string",
  "server_instance_count": 1,
  "managed_auth_via_composio": true,
  "deleted": true
}
```

#### 400 - Bad request. The request parameters may be invalid or in an incorrect format.

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

#### 403 - Forbidden. The authenticated user does not have permission to update this MCP server.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - MCP server not found. No server with the specified ID exists or it has been deleted.

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
curl -X PATCH "https://backend.composio.dev/api/v3/mcp/string" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "string",
    "toolkits": [
      "string"
    ],
    "allowed_tools": [
      "string"
    ],
    "managed_auth_via_composio": true,
    "auth_config_ids": [
      "string"
    ]
  }'
```

# Create a new MCP server

**Documentation:** /reference/api-reference/mcp/postMcpServers

Creates a new Model Control Protocol (MCP) server instance for the authenticated project. An MCP server provides a connection point for AI assistants to access your applications and services. The server is configured with specific authentication and tool permissions that determine what actions the connected assistants can perform.

---

## POST `/api/v3/mcp/servers`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers`

**Summary:** Create a new MCP server

Creates a new Model Control Protocol (MCP) server instance for the authenticated project. An MCP server provides a connection point for AI assistants to access your applications and services. The server is configured with specific authentication and tool permissions that determine what actions the connected assistants can perform.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `name` (string) *(required)*: Human-readable name to identify this MCP server instance (4-30 characters, alphanumeric, spaces, and hyphens only)
- `auth_config_ids` (array<string>) *(required)*: ID references to existing authentication configurations
- `no_auth_apps` (array<string>): List of NO_AUTH apps to enable for this MCP server
- `allowed_tools` (array<string>): List of tool slugs that should be allowed for this server. If not provided, all available tools for the authentication configuration will be enabled.
- `managed_auth_via_composio` (boolean): Whether the MCP server is managed by Composio

**Example:**

```json
{
  "name": "string",
  "auth_config_ids": [
    "string"
  ],
  "no_auth_apps": [
    "string"
  ],
  "allowed_tools": [],
  "managed_auth_via_composio": true
}
```

### Responses

#### 201 - MCP server created successfully. Returns the complete server configuration including connection details and command instructions.

**Response Schema:**

- `id` (string) *(required)*: UUID of the MCP server instance
- `name` (string) *(required)*: User-defined descriptive name for this MCP server
- `auth_config_ids` (array<string>) *(required)*: ID references to the auth configurations used by this server
- `allowed_tools` (array<string>) *(required)*: Array of tool slugs that this MCP server is allowed to use
- `mcp_url` (string) *(required)*: [DEPRECATED] Please use the URL with user_id or connected_account_id query param
- `toolkits` (array<string>) *(required)*: Array of toolkit slugs that this MCP server is allowed to use
- `toolkit_icons` (object) *(required)*: Object mapping each toolkit slug to its icon/logo URL for display purposes
- `commands` (object) *(required)*: Set of command line instructions for connecting various clients to this MCP server
  - `cursor` (string) *(required)*: Command line instruction for Cursor client setup
  - `claude` (string) *(required)*: Command line instruction for Claude client setup
  - `windsurf` (string) *(required)*: Command line instruction for Windsurf client setup
- `updated_at` (string) *(required)*: Date and time when this server configuration was last modified
- `created_at` (string) *(required)*: Date and time when this server was initially created
- `server_instance_count` (number) *(required)*: Total count of active user instances connected to this server
- `managed_auth_via_composio` (boolean) *(required)*: Whether the MCP server is managed by Composio

**Example Response:**

```json
{
  "id": "string",
  "name": "string",
  "auth_config_ids": [
    "string"
  ],
  "allowed_tools": [
    "string"
  ],
  "mcp_url": "string",
  "toolkits": [
    "string"
  ],
  "toolkit_icons": {},
  "commands": {
    "cursor": "string",
    "claude": "string",
    "windsurf": "string"
  },
  "updated_at": "string",
  "created_at": "string",
  "server_instance_count": 1,
  "managed_auth_via_composio": true
}
```

#### 400 - Bad request. The request body may be invalid, missing required parameters, or the auth_config_id may not exist.

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

#### 403 - Forbidden. The authenticated user does not have permission to create MCP servers for this project.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not Found. The specified auth_config_id does not exist or is not accessible to the authenticated user.

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
curl -X POST "https://backend.composio.dev/api/v3/mcp/servers" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "string",
    "auth_config_ids": [
      "string"
    ],
    "no_auth_apps": [
      "string"
    ],
    "allowed_tools": [],
    "managed_auth_via_composio": true
  }'
```

# Create a new MCP server instance

**Documentation:** /reference/api-reference/mcp/postMcpServersByServerIdInstances

Creates a new instance for a Model Control Protocol (MCP) server. This endpoint validates that the user has connected accounts for all auth configurations associated with the MCP server before creating the instance.

---

## POST `/api/v3/mcp/servers/{serverId}/instances`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers/{serverId}/instances`

**Summary:** Create a new MCP server instance

Creates a new instance for a Model Control Protocol (MCP) server. This endpoint validates that the user has connected accounts for all auth configurations associated with the MCP server before creating the instance.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `serverId` (string) *(required)*: The ID of the MCP server

### Request Body

**Schema:**

- `user_id` (string) *(required)*: The user ID (entity ID) that will be used as both the user identifier and instance ID

**Example:**

```json
{
  "user_id": "string"
}
```

### Responses

#### 201 - Successfully created MCP server instance. Returns the created instance details.

**Response Schema:**

- `id` (string) *(required)*: UUID of the instance record
- `instance_id` (string) *(required)*: The instance identifier (same as the user_id)
- `mcp_server_id` (string) *(required)*: UUID of the parent MCP server
- `created_at` (string) *(required)*: Date and time when this instance was created
- `updated_at` (string) *(required)*: Date and time when this instance was last modified

**Example Response:**

```json
{
  "id": "string",
  "instance_id": "string",
  "mcp_server_id": "string",
  "created_at": "string",
  "updated_at": "string"
}
```

#### 400 - Bad request. The request parameters may be invalid or the instance ID already exists.

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

#### 403 - Forbidden. The user does not have connected accounts for all required auth configurations.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - MCP server not found. No server with the specified ID exists.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 409 - Conflict. An instance with this ID already exists for the server.

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
curl -X POST "https://backend.composio.dev/api/v3/mcp/servers/string/instances" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "string"
  }'
```

# Create a new custom MCP server with multiple apps

**Documentation:** /reference/api-reference/mcp/postMcpServersCustom

Creates a new Model Control Protocol (MCP) server instance that can integrate with multiple applications or toolkits simultaneously. This endpoint allows you to create a server that can access tools from different applications, making it suitable for complex workflows that span multiple services.

---

## POST `/api/v3/mcp/servers/custom`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers/custom`

**Summary:** Create a new custom MCP server with multiple apps

Creates a new Model Control Protocol (MCP) server instance that can integrate with multiple applications or toolkits simultaneously. This endpoint allows you to create a server that can access tools from different applications, making it suitable for complex workflows that span multiple services.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `name` (string) *(required)*: Human-readable name to identify this custom MCP server (4-30 characters, alphanumeric, spaces, and hyphens only)
- `auth_config_ids` (array<string>): ID references to existing authentication configurations
- `toolkits` (array<string>): List of application/toolkit identifiers to enable for this server
- `allowed_tools` (array<string>): Tool identifiers to enable that aren't part of standard toolkits
- `custom_tools` (array<string>): DEPRECATED: Use allowed_tools instead. Tool identifiers to enable that aren't part of standard toolkits
- `managed_auth_via_composio` (boolean): Whether to manage authentication via Composio

**Example:**

```json
{
  "name": "string",
  "auth_config_ids": [
    "string"
  ],
  "toolkits": [
    "string"
  ],
  "allowed_tools": [
    "string"
  ],
  "custom_tools": [
    "string"
  ],
  "managed_auth_via_composio": true
}
```

### Responses

#### 201 - Custom MCP server created successfully. Returns the complete server configuration including connection details and command instructions for all specified applications.

**Response Schema:**

- `id` (string) *(required)*: Unique identifier for the newly created custom MCP server
- `name` (string) *(required)*: Human-readable name of the custom MCP server
- `auth_config_ids` (array<string>) *(required)*: ID references to the auth configurations used by this server
- `allowed_tools` (array<string>) *(required)*: List of tool identifiers that are enabled for this server
- `mcp_url` (string) *(required)*: URL endpoint for establishing connection to this MCP server
- `commands` (object) *(required)*: Set of command line instructions for connecting various clients to this MCP server
  - `cursor` (string) *(required)*: Command line instruction for Cursor client setup
  - `claude` (string) *(required)*: Command line instruction for Claude client setup
  - `windsurf` (string) *(required)*: Command line instruction for Windsurf client setup

**Example Response:**

```json
{
  "id": "string",
  "name": "string",
  "auth_config_ids": [
    "string"
  ],
  "allowed_tools": [
    "string"
  ],
  "mcp_url": "string",
  "commands": {
    "cursor": "string",
    "claude": "string",
    "windsurf": "string"
  }
}
```

#### 400 - Bad request. The request body may be invalid, missing required parameters, or contain invalid toolkit identifiers.

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

#### 403 - Forbidden. The authenticated user does not have permission to create MCP servers for this project.

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
curl -X POST "https://backend.composio.dev/api/v3/mcp/servers/custom" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "string",
    "auth_config_ids": [
      "string"
    ],
    "toolkits": [
      "string"
    ],
    "allowed_tools": [
      "string"
    ],
    "custom_tools": [
      "string"
    ],
    "managed_auth_via_composio": true
  }'
```

# Generate MCP URL with custom parameters

**Documentation:** /reference/api-reference/mcp/postMcpServersGenerate

Generates a Model Control Protocol (MCP) URL for an existing server with custom query parameters. The URL includes user-specific parameters and configuration flags that control the behavior of the MCP connection.

---

## POST `/api/v3/mcp/servers/generate`

**Endpoint:** `https://backend.composio.dev/api/v3/mcp/servers/generate`

**Summary:** Generate MCP URL with custom parameters

Generates a Model Control Protocol (MCP) URL for an existing server with custom query parameters. The URL includes user-specific parameters and configuration flags that control the behavior of the MCP connection.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `mcp_server_id` (string) *(required)*: Unique identifier of the MCP server to generate URL for
- `managed_auth_by_composio` (boolean): Flag indicating if Composio manages authentication
- `user_ids` (array<string>): List of user identifiers for whom the URL is generated
- `connected_account_ids` (array<string>): List of connected account identifiers

**Example:**

```json
{
  "mcp_server_id": "string",
  "managed_auth_by_composio": false,
  "user_ids": [
    "string"
  ],
  "connected_account_ids": [
    "string"
  ]
}
```

### Responses

#### 200 - Successfully generated MCP URL. Returns the complete URL with all specified query parameters.

**Response Schema:**

- `mcp_url` (string) *(required)*: Base MCP URL without any query parameters
- `connected_account_urls` (array<string>) *(required)*: List of URLs generated for each connected account ID
- `user_ids_url` (array<string>) *(required)*: List of URLs generated for each user ID

**Example Response:**

```json
{
  "mcp_url": "string",
  "connected_account_urls": [
    "string"
  ],
  "user_ids_url": [
    "string"
  ]
}
```

#### 400 - Bad request. The request body may be invalid or missing required parameters.

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

#### 403 - Forbidden. The authenticated user does not have permission to access this MCP server.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - MCP server not found. No server with the specified ID exists or it has been deleted.

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
curl -X POST "https://backend.composio.dev/api/v3/mcp/servers/generate" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "mcp_server_id": "string",
    "managed_auth_by_composio": false,
    "user_ids": [
      "string"
    ],
    "connected_account_ids": [
      "string"
    ]
  }'
```