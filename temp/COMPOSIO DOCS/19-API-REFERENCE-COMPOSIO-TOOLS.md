# Tools (/reference/api-reference/tools)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Tool execution endpoints

# Endpoints

| Endpoint                                       | Quick Link                                                                                                   |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `GET /api/v3/tools`                            | [List available tools](/reference/api-reference/tools/getTools)                                              |
| `GET /api/v3/tools/enum`                       | [Get tool enum list](/reference/api-reference/tools/getToolsEnum)                                            |
| `GET /api/v3/tools/{tool_slug}`                | [Get tool by slug](/reference/api-reference/tools/getToolsByToolSlug)                                        |
| `POST /api/v3/tools/execute/{tool_slug}`       | [Execute tool](/reference/api-reference/tools/postToolsExecuteByToolSlug)                                    |
| `POST /api/v3/tools/execute/{tool_slug}/input` | [Generate tool inputs from natural language](/reference/api-reference/tools/postToolsExecuteByToolSlugInput) |
| `POST /api/v3/tools/execute/proxy`             | [Execute proxy request](/reference/api-reference/tools/postToolsExecuteProxy)                                |

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
# List available tools

**Documentation:** /reference/api-reference/tools/getTools

Retrieve a paginated list of available tools with comprehensive filtering, sorting and search capabilities. Use query parameters to narrow down results by toolkit, tags, or search terms.

---

## GET `/api/v3/tools`

**Endpoint:** `https://backend.composio.dev/api/v3/tools`

**Summary:** List available tools

Retrieve a paginated list of available tools with comprehensive filtering, sorting and search capabilities. Use query parameters to narrow down results by toolkit, tags, or search terms.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `toolkit_slug` (string): The slug of the toolkit to filter by
- `tool_slugs` (string): Comma-separated list of specific tool slugs to retrieve (overrides other filters)
- `auth_config_ids` (any): Comma-separated list of auth config IDs to filter tools by
- `important` (enum: "true" | "false"): Filter to only show important/featured tools (set to "true" to enable)
- `tags` (array<string>): Filter tools by one or more tags (can be specified multiple times)
- `scopes` (array,null): Array of scopes to filter tools by)
- `query` (string): Full-text search query to filter tools by name, slug, or description. Applied as a soft filter on top of other filters.
- `search` (string): Deprecated: use "query" instead. Free-text search query to find tools by name, description, or functionality.
- `include_deprecated` (boolean): Include deprecated tools in the response
- `toolkit_versions` (any): Toolkit version specification. Use "latest" for latest versions or bracket notation for specific versions per toolkit.
- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Successfully retrieved tools list with pagination information

**Response Schema:**

- `items` (array<object>) *(required)*
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
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

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
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
}
```

#### 400 - Bad request - Invalid query parameters

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication credentials are missing or invalid

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - User does not have permission to access tools

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - The requested resource was not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 429 - Too many requests - Rate limit exceeded

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
curl -X GET "https://backend.composio.dev/api/v3/tools" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get tool by slug

**Documentation:** /reference/api-reference/tools/getToolsByToolSlug

Retrieve detailed information about a specific tool using its slug identifier. This endpoint returns full metadata about a tool including input/output parameters, versions, and toolkit information.

---

## GET `/api/v3/tools/{tool_slug}`

**Endpoint:** `https://backend.composio.dev/api/v3/tools/{tool_slug}`

**Summary:** Get tool by slug

Retrieve detailed information about a specific tool using its slug identifier. This endpoint returns full metadata about a tool including input/output parameters, versions, and toolkit information.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `tool_slug` (string) *(required)*: The unique slug identifier of the tool

### Query Parameters

- `version` (string): Optional version of the tool to retrieve
- `toolkit_versions` (any): Toolkit version specification. Use "latest" for latest versions or bracket notation for specific versions per toolkit.

### Responses

#### 200 - Successfully retrieved tool details

**Response Schema:**

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
  "slug": "string",
  "name": "string",
  "description": "string",
  "toolkit": {
    "slug": "string",
    "name": "string",
    "logo": "string"
  },
  "input_parameters": {},
  "no_auth": true,
  "available_versions": [
    "string"
  ],
  "version": "string",
  "output_parameters": {},
  "scopes": [
    "string"
  ],
  "tags": [
    "string"
  ],
  "human_description": "string",
  "is_deprecated": true,
  "deprecated": {
    "displayName": "string",
    "version": "string",
    "available_versions": [
      "string"
    ],
    "is_deprecated": true,
    "toolkit": {
      "logo": "string"
    }
  }
}
```

#### 400 - Bad request - Invalid tool slug format

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication credentials are missing or invalid

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Tool with the specified slug does not exist

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
curl -X GET "https://backend.composio.dev/api/v3/tools/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get tool enum list

**Documentation:** /reference/api-reference/tools/getToolsEnum

Retrieve a list of all available tool enumeration values (tool slugs) from latest version of each toolkit. This endpoint returns a comma-separated string of tool slugs that can be used in other API calls.

---

## GET `/api/v3/tools/enum`

**Endpoint:** `https://backend.composio.dev/api/v3/tools/enum`

**Summary:** Get tool enum list

Retrieve a list of all available tool enumeration values (tool slugs) from latest version of each toolkit. This endpoint returns a comma-separated string of tool slugs that can be used in other API calls.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Successfully retrieved tool enum list

**Response Schema:**


**Example Response:**

```json
[
  "string"
]
```

#### 400 - Bad request - Invalid request parameters

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication credentials are missing or invalid

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - User does not have permission to access tool enums

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Tool enum list not found

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
curl -X GET "https://backend.composio.dev/api/v3/tools/enum" \
  -H "x-api-key: YOUR_API_KEY"
```

# Execute tool

**Documentation:** /reference/api-reference/tools/postToolsExecuteByToolSlug

Execute a specific tool operation with provided arguments and authentication. This is the primary endpoint for integrating with third-party services and executing tools. You can provide structured arguments or use natural language processing by providing a text description of what you want to accomplish.

---

## POST `/api/v3/tools/execute/{tool_slug}`

**Endpoint:** `https://backend.composio.dev/api/v3/tools/execute/{tool_slug}`

**Summary:** Execute tool

Execute a specific tool operation with provided arguments and authentication. This is the primary endpoint for integrating with third-party services and executing tools. You can provide structured arguments or use natural language processing by providing a text description of what you want to accomplish.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `tool_slug` (string) *(required)*: The tool slug to execute

### Request Body

**Schema:**

- `connected_account_id` (string): Unique identifier for the connected account to use for authentication
- `entity_id` (string): Deprecated: please use user_id instead. Entity identifier for multi-entity connected accounts (e.g. multiple repositories, organizations)
- `user_id` (string): User id for multi-user connected accounts (e.g. multiple users, organizations)
- `version` (string): Tool version to execute (defaults to "00000000_00" if not specified)
- `custom_auth_params` (object): Custom authentication parameters for tools that support parameterized authentication
  - `base_url` (string): The base URL (root address) what you should use while making http requests to the connected account. For example, for gmail, it would be 'https://gmail.googleapis.com'
  - `parameters` (array<object>)
    - Array items:
      - `name` (string) *(required)*: The name of the parameter. For example, 'x-api-key', 'Content-Type', etc.
      - `in` (enum: "query" | "header") *(required)*: The location of the parameter. Can be 'query' or 'header'.
      - `value` (any) *(required)*: The value of the parameter. For example, '1234567890', 'application/json', etc.
  - `body` (object): The body to be sent to the endpoint for authentication. This is a JSON object. Note: This is very rarely needed and is only required by very few apps.
- `custom_connection_data` (object): Custom connection data for tools that support custom connection data
  - `authScheme` (enum: "OAUTH2" | "DCR_OAUTH" | "API_KEY" | ...) *(required)*
  - `toolkitSlug` (string) *(required)*
  - `val` (object) *(required)*
    - `subdomain` (string)
    - `your-domain` (string)
    - `region` (string)
    - `shop` (string)
    - `account_url` (string)
    - `COMPANYDOMAIN` (string)
    - `extension` (string)
    - `form_api_base_url` (string)
    - `instanceEndpoint` (string)
    - `api_url` (string)
    - `borneo_dashboard_url` (string)
    - `proxy_username` (string)
    - `proxy_password` (string)
    - `domain` (string)
    - `version` (string)
    - `dc` (string)
    - `site_name` (string)
    - `instanceName` (string)
    - `account_id` (string)
    - `your_server` (string)
    - `server_location` (string)
    - `base_url` (string)
    - `state_prefix` (string): The oauth2 state prefix for the connection
    - `long_redirect_url` (boolean): Whether to return the redirect url without shortening
    - `access_token` (string) *(required)*
    - `id_token` (string)
    - `token_type` (string)
    - `refresh_token` (string,null)
    - `expires_in` (any)
    - `scope` (any)
    - `webhook_signature` (string)
    - `authed_user` (object): for slack user scopes
      - `access_token` (string)
      - `scope` (string)
    - `client_id` (string): Dynamically registered client ID
    - `client_secret` (string): Dynamically registered client secret
    - `client_id_issued_at` (number)
    - `client_secret_expires_at` (number)
    - `generic_api_key` (string)
    - `api_key` (string)
    - `bearer_token` (string)
    - `basic_encoded` (string)
    - `username` (string)
    - `password` (string)
    - `token` (string)
    - `oauth_token` (string)
    - `oauth_token_secret` (string)
    - `oauth_verifier` (string)
    - `consumer_key` (string)
    - `redirectUrl` (string)
    - `callback_url` (string)
    - `application_id` (string)
    - `installation_id` (string)
    - `private_key` (string)
    - `credentials_json` (string)
- `arguments` (object): Key-value pairs of arguments required by the tool (mutually exclusive with text)
- `text` (string): Natural language description of the task to perform (mutually exclusive with arguments)
- `allow_tracing` (boolean,null): Deprecated. Enable debug tracing for tool execution (useful for debugging)

**Example:**

```json
{
  "connected_account_id": "string",
  "entity_id": "string",
  "user_id": "string",
  "version": "string",
  "custom_auth_params": {
    "base_url": "string",
    "parameters": [
      {
        "name": "...",
        "in": "...",
        "value": "..."
      }
    ],
    "body": {}
  },
  "custom_connection_data": {
    "authScheme": "OAUTH2",
    "toolkitSlug": "string",
    "val": {
      "subdomain": "string",
      "your-domain": "string",
      "region": "string",
      "shop": "string",
      "account_url": "string",
      "COMPANYDOMAIN": "string",
      "extension": "string",
      "form_api_base_url": "string",
      "instanceEndpoint": "string",
      "api_url": "string",
      "borneo_dashboard_url": "string",
      "proxy_username": "string",
      "proxy_password": "string",
      "domain": "string",
      "version": "string",
      "dc": "string",
      "site_name": "string",
      "instanceName": "string",
      "account_id": "string",
      "your_server": "string",
      "server_location": "string",
      "base_url": "string",
      "state_prefix": "string",
      "long_redirect_url": true,
      "access_token": "string",
      "id_token": "string",
      "token_type": "string",
      "refresh_token": null,
      "expires_in": null,
      "scope": null,
      "webhook_signature": "string",
      "authed_user": {
        "access_token": "...",
        "scope": "..."
      },
      "client_id": "string",
      "client_secret": "string",
      "client_id_issued_at": 1,
      "client_secret_expires_at": 1,
      "generic_api_key": "string",
      "api_key": "string",
      "bearer_token": "string",
      "basic_encoded": "string",
      "username": "string",
      "password": "string",
      "token": "string",
      "oauth_token": "string",
      "oauth_token_secret": "string",
      "oauth_verifier": "string",
      "consumer_key": "string",
      "redirectUrl": "string",
      "callback_url": "string",
      "application_id": "string",
      "installation_id": "string",
      "private_key": "string",
      "credentials_json": "string"
    }
  },
  "arguments": {},
  "text": "string",
  "allow_tracing": null
}
```

### Responses

#### 200 - Successfully executed action and received response

**Response Schema:**

- `data` (object) *(required)*: Tool execution output data that varies based on the specific tool
- `error` (string,null) *(required)*: Error message if the tool execution was not successful (null if successful)
- `successful` (boolean) *(required)*: Indicates if the tool execution was successful
- `session_info` (object,null): Optional session information for tools that return session context
- `log_id` (string): Unique identifier for the execution log (useful for debugging and support)

**Example Response:**

```json
{
  "data": {},
  "error": null,
  "successful": true,
  "session_info": null,
  "log_id": "string"
}
```

#### 400 - Bad request - Invalid request parameters, missing required arguments, or conflicting parameters

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication credentials are missing or invalid

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - Connected account does not have permission to execute this tool or access the requested resource

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Tool or connected account not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 410 - Gone - Tool has been deprecated and is no longer available

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 413 - Payload too large - Request or response payload exceeds size limits

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 422 - Unprocessable entity - Invalid state of the connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 429 - Rate limit exceeded - Too many requests to the tool or underlying API

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

#### 501 - Not implemented

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 502 - Bad gateway - Error communicating with the tool provider API

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 503 - Upstream service unavailable - Tool provider API is currently down or unavailable

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
curl -X POST "https://backend.composio.dev/api/v3/tools/execute/string" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "connected_account_id": "string",
    "entity_id": "string",
    "user_id": "string",
    "version": "string",
    "custom_auth_params": {
      "base_url": "string",
      "parameters": [
        {
          "name": "...",
          "in": "...",
          "value": "..."
        }
      ],
      "body": {}
    },
    "custom_connection_data": {
      "authScheme": "OAUTH2",
      "toolkitSlug": "string",
      "val": {
        "subdomain": "string",
        "your-domain": "string",
        "region": "string",
        "shop": "string",
        "account_url": "string",
        "COMPANYDOMAIN": "string",
        "extension": "string",
        "form_api_base_url": "string",
        "instanceEndpoint": "string",
        "api_url": "string",
        "borneo_dashboard_url": "string",
        "proxy_username": "string",
        "proxy_password": "string",
        "domain": "string",
        "version": "string",
        "dc": "string",
        "site_name": "string",
        "instanceName": "string",
        "account_id": "string",
        "your_server": "string",
        "server_location": "string",
        "base_url": "string",
        "state_prefix": "string",
        "long_redirect_url": true,
        "access_token": "string",
        "id_token": "string",
        "token_type": "string",
        "refresh_token": null,
        "expires_in": null,
        "scope": null,
        "webhook_signature": "string",
        "authed_user": {
          "access_token": "...",
          "scope": "..."
        },
        "client_id": "string",
        "client_secret": "string",
        "client_id_issued_at": 1,
        "client_secret_expires_at": 1,
        "generic_api_key": "string",
        "api_key": "string",
        "bearer_token": "string",
        "basic_encoded": "string",
        "username": "string",
        "password": "string",
        "token": "string",
        "oauth_token": "string",
        "oauth_token_secret": "string",
        "oauth_verifier": "string",
        "consumer_key": "string",
        "redirectUrl": "string",
        "callback_url": "string",
        "application_id": "string",
        "installation_id": "string",
        "private_key": "string",
        "credentials_json": "string"
      }
    },
    "arguments": {},
    "text": "string",
    "allow_tracing": null
  }'
```

# Generate tool inputs from natural language

**Documentation:** /reference/api-reference/tools/postToolsExecuteByToolSlugInput

Uses AI to translate a natural language description into structured arguments for a specific tool. This endpoint is useful when you want to let users describe what they want to do in plain language instead of providing structured parameters.

---

## POST `/api/v3/tools/execute/{tool_slug}/input`

**Endpoint:** `https://backend.composio.dev/api/v3/tools/execute/{tool_slug}/input`

**Summary:** Generate tool inputs from natural language

Uses AI to translate a natural language description into structured arguments for a specific tool. This endpoint is useful when you want to let users describe what they want to do in plain language instead of providing structured parameters.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `tool_slug` (string) *(required)*: The tool slug to generate inputs for

### Request Body

**Schema:**

- `text` (string) *(required)*: Natural language description of what you want to accomplish with this tool
- `custom_description` (string): Custom description of the tool to help guide the LLM in generating more accurate inputs
- `system_prompt` (string): System prompt to control and guide the behavior of the LLM when generating inputs
- `version` (string): Tool version to use when generating inputs (defaults to "latest" if not specified)

**Example:**

```json
{
  "text": "string",
  "custom_description": "string",
  "system_prompt": "string",
  "version": "string"
}
```

### Responses

#### 200 - Successfully generated structured inputs for the action based on natural language description

**Response Schema:**

- `arguments` (object): Key-value pairs of arguments required by the tool to accomplish the described task
- `error` (string): Error message if the arguments could not be generated (null if successful)

**Example Response:**

```json
{
  "arguments": {},
  "error": "string"
}
```

#### 400 - Bad request - Invalid input parameters or insufficient description to generate tool arguments

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication credentials are missing or invalid

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - The specified tool does not exist

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 422 - Unprocessable entity - Invalid state of the connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 429 - Too many requests - Rate limit exceeded for natural language processing

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error - AI processing failed or other server error occurred

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
curl -X POST "https://backend.composio.dev/api/v3/tools/execute/string/input" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "string",
    "custom_description": "string",
    "system_prompt": "string",
    "version": "string"
  }'
```

# Execute proxy request

**Documentation:** /reference/api-reference/tools/postToolsExecuteProxy

Proxy an HTTP request to a third-party API using connected account credentials. This endpoint allows making authenticated API calls to external services while abstracting away authentication details.

---

## POST `/api/v3/tools/execute/proxy`

**Endpoint:** `https://backend.composio.dev/api/v3/tools/execute/proxy`

**Summary:** Execute proxy request

Proxy an HTTP request to a third-party API using connected account credentials. This endpoint allows making authenticated API calls to external services while abstracting away authentication details.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `connected_account_id` (string (connectedAccountId)): The ID of the connected account to use for authentication (if not provided, will use the default account for the project)
- `endpoint` (string) *(required)*: The API endpoint to call (absolute URL or path relative to base URL of the connected account)
- `method` (enum: "GET" | "POST" | "PUT" | ...) *(required)*: The HTTP method to use for the request
- `body` (object,null): The request body (for POST, PUT, and PATCH requests)
- `binary_body` (any): Binary body to send. For binary upload via URL: use {url: "https://...", content_type?: "..."}. For binary upload via base64: use {base64: "...", content_type?: "..."}.
- `parameters` (array<object>): Additional HTTP headers or query parameters to include in the request
  - Array items:
    - `name` (string) *(required)*: Parameter name
    - `value` (string) *(required)*: Parameter value
    - `type` (enum: "header" | "query") *(required)*: Parameter type (header or query)
- `custom_connection_data` (object)
  - `authScheme` (enum: "OAUTH2" | "DCR_OAUTH" | "API_KEY" | ...) *(required)*
  - `toolkitSlug` (string) *(required)*
  - `val` (object) *(required)*
    - `subdomain` (string)
    - `your-domain` (string)
    - `region` (string)
    - `shop` (string)
    - `account_url` (string)
    - `COMPANYDOMAIN` (string)
    - `extension` (string)
    - `form_api_base_url` (string)
    - `instanceEndpoint` (string)
    - `api_url` (string)
    - `borneo_dashboard_url` (string)
    - `proxy_username` (string)
    - `proxy_password` (string)
    - `domain` (string)
    - `version` (string)
    - `dc` (string)
    - `site_name` (string)
    - `instanceName` (string)
    - `account_id` (string)
    - `your_server` (string)
    - `server_location` (string)
    - `base_url` (string)
    - `state_prefix` (string): The oauth2 state prefix for the connection
    - `long_redirect_url` (boolean): Whether to return the redirect url without shortening
    - `access_token` (string) *(required)*
    - `id_token` (string)
    - `token_type` (string)
    - `refresh_token` (string,null)
    - `expires_in` (any)
    - `scope` (any)
    - `webhook_signature` (string)
    - `authed_user` (object): for slack user scopes
      - `access_token` (string)
      - `scope` (string)
    - `client_id` (string): Dynamically registered client ID
    - `client_secret` (string): Dynamically registered client secret
    - `client_id_issued_at` (number)
    - `client_secret_expires_at` (number)
    - `generic_api_key` (string)
    - `api_key` (string)
    - `bearer_token` (string)
    - `basic_encoded` (string)
    - `username` (string)
    - `password` (string)
    - `token` (string)
    - `oauth_token` (string)
    - `oauth_token_secret` (string)
    - `oauth_verifier` (string)
    - `consumer_key` (string)
    - `redirectUrl` (string)
    - `callback_url` (string)
    - `application_id` (string)
    - `installation_id` (string)
    - `private_key` (string)
    - `credentials_json` (string)

**Example:**

```json
{
  "connected_account_id": "string",
  "endpoint": "string",
  "method": "GET",
  "body": null,
  "binary_body": null,
  "parameters": [
    {
      "name": "string",
      "value": "string",
      "type": "header"
    }
  ],
  "custom_connection_data": {
    "authScheme": "OAUTH2",
    "toolkitSlug": "string",
    "val": {
      "subdomain": "string",
      "your-domain": "string",
      "region": "string",
      "shop": "string",
      "account_url": "string",
      "COMPANYDOMAIN": "string",
      "extension": "string",
      "form_api_base_url": "string",
      "instanceEndpoint": "string",
      "api_url": "string",
      "borneo_dashboard_url": "string",
      "proxy_username": "string",
      "proxy_password": "string",
      "domain": "string",
      "version": "string",
      "dc": "string",
      "site_name": "string",
      "instanceName": "string",
      "account_id": "string",
      "your_server": "string",
      "server_location": "string",
      "base_url": "string",
      "state_prefix": "string",
      "long_redirect_url": true,
      "access_token": "string",
      "id_token": "string",
      "token_type": "string",
      "refresh_token": null,
      "expires_in": null,
      "scope": null,
      "webhook_signature": "string",
      "authed_user": {
        "access_token": "...",
        "scope": "..."
      },
      "client_id": "string",
      "client_secret": "string",
      "client_id_issued_at": 1,
      "client_secret_expires_at": 1,
      "generic_api_key": "string",
      "api_key": "string",
      "bearer_token": "string",
      "basic_encoded": "string",
      "username": "string",
      "password": "string",
      "token": "string",
      "oauth_token": "string",
      "oauth_token_secret": "string",
      "oauth_verifier": "string",
      "consumer_key": "string",
      "redirectUrl": "string",
      "callback_url": "string",
      "application_id": "string",
      "installation_id": "string",
      "private_key": "string",
      "credentials_json": "string"
    }
  }
}
```

### Responses

#### 200 - Successfully executed proxy request and received response

**Response Schema:**

- `data` (object,null): The response data returned from the proxied API
- `binary_data` (object): Binary body response data. Present when the response is a binary file.
  - `url` (string) *(required)*: URL to download binary content
  - `content_type` (string) *(required)*: Content-Type of the binary data
  - `size` (number) *(required)*: File size in bytes
  - `expires_at` (string): ISO 8601 timestamp when the URL expires
- `status` (number) *(required)*: The HTTP status code returned from the proxied API
- `headers` (object): The HTTP headers returned from the proxied API

**Example Response:**

```json
{
  "data": null,
  "binary_data": {
    "url": "string",
    "content_type": "string",
    "size": 1,
    "expires_at": "string"
  },
  "status": 1,
  "headers": {}
}
```

#### 400 - Bad request - Invalid request parameters or body format

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication credentials are missing or invalid

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - Connected account does not have permission to access the requested resource

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Resource not found or connected account not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 422 - Unprocessable entity - Invalid state of the connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 429 - Too many requests - Rate limit exceeded on the connected account or proxy service

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

#### 501 - Not implemented - This operation is not supported for the requested connected account or authentication scheme

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 502 - Bad gateway - Error communicating with the upstream API

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 504 - Gateway timeout - Upstream API timed out

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
curl -X POST "https://backend.composio.dev/api/v3/tools/execute/proxy" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "connected_account_id": "string",
    "endpoint": "string",
    "method": "GET",
    "body": null,
    "binary_body": null,
    "parameters": [
      {
        "name": "string",
        "value": "string",
        "type": "header"
      }
    ],
    "custom_connection_data": {
      "authScheme": "OAUTH2",
      "toolkitSlug": "string",
      "val": {
        "subdomain": "string",
        "your-domain": "string",
        "region": "string",
        "shop": "string",
        "account_url": "string",
        "COMPANYDOMAIN": "string",
        "extension": "string",
        "form_api_base_url": "string",
        "instanceEndpoint": "string",
        "api_url": "string",
        "borneo_dashboard_url": "string",
        "proxy_username": "string",
        "proxy_password": "string",
        "domain": "string",
        "version": "string",
        "dc": "string",
        "site_name": "string",
        "instanceName": "string",
        "account_id": "string",
        "your_server": "string",
        "server_location": "string",
        "base_url": "string",
        "state_prefix": "string",
        "long_redirect_url": true,
        "access_token": "string",
        "id_token": "string",
        "token_type": "string",
        "refresh_token": null,
        "expires_in": null,
        "scope": null,
        "webhook_signature": "string",
        "authed_user": {
          "access_token": "...",
          "scope": "..."
        },
        "client_id": "string",
        "client_secret": "string",
        "client_id_issued_at": 1,
        "client_secret_expires_at": 1,
        "generic_api_key": "string",
        "api_key": "string",
        "bearer_token": "string",
        "basic_encoded": "string",
        "username": "string",
        "password": "string",
        "token": "string",
        "oauth_token": "string",
        "oauth_token_secret": "string",
        "oauth_verifier": "string",
        "consumer_key": "string",
        "redirectUrl": "string",
        "callback_url": "string",
        "application_id": "string",
        "installation_id": "string",
        "private_key": "string",
        "credentials_json": "string"
      }
    }
  }'
```