# Toolkits (/reference/api-reference/toolkits)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Toolkit and tool management

# Endpoints

| Endpoint                          | Quick Link                                                                         |
| --------------------------------- | ---------------------------------------------------------------------------------- |
| `GET /api/v3/toolkits`            | [List available toolkits](/reference/api-reference/toolkits/getToolkits)           |
| `GET /api/v3/toolkits/categories` | [List toolkit categories](/reference/api-reference/toolkits/getToolkitsCategories) |
| `GET /api/v3/toolkits/{slug}`     | [Get toolkit by slug](/reference/api-reference/toolkits/getToolkitsBySlug)         |
| `POST /api/v3/toolkits/multi`     | [Fetch multiple toolkits](/reference/api-reference/toolkits/postToolkitsMulti)     |
| `GET /api/v3/toolkits/changelog`  | [Get toolkits changelog](/reference/api-reference/toolkits/getToolkitsChangelog)   |

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

# List available toolkits

**Documentation:** /reference/api-reference/toolkits/getToolkits

Retrieves a comprehensive list of toolkits of their latest versions that are available to the authenticated project. Toolkits represent integration points with external services and applications, each containing a collection of tools and triggers. This endpoint supports filtering by category and management type, as well as different sorting options.

---

## GET `/api/v3/toolkits`

**Endpoint:** `https://backend.composio.dev/api/v3/toolkits`

**Summary:** List available toolkits

Retrieves a comprehensive list of toolkits of their latest versions that are available to the authenticated project. Toolkits represent integration points with external services and applications, each containing a collection of tools and triggers. This endpoint supports filtering by category and management type, as well as different sorting options.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `category` (string): Filter toolkits by category
- `managed_by` (enum: "composio" | "all" | "project"): Filter toolkits by who manages them
- `sort_by` (enum: "usage" | "alphabetically"): Sort order for returned toolkits
- `include_deprecated` (boolean,null): Include deprecated toolkits in the response
- `search` (string): Search query to filter toolkits by name, slug, or description
- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Toolkits retrieved successfully. Returns a paginated list of available toolkits with detailed metadata.

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `slug` (string) *(required)*: URL-friendly unique identifier for the toolkit
    - `name` (string) *(required)*: Human-readable name of the toolkit
    - `auth_schemes` (array<string>): List of authentication methods supported by this toolkit
    - `composio_managed_auth_schemes` (array<string>): List of authentication methods that Composio manages for this toolkit
    - `is_local_toolkit` (boolean) *(required)*: DEPRECATED: This field is no longer meaningful and will always return false. It was previously used to indicate if a toolkit is specific to the current project.
    - `no_auth` (boolean): When true, this toolkit can be used without authentication
    - `deprecated` (object) *(required)*: Deprecated toolkit ID
      - `toolkitId` (string) *(required)*
    - `meta` (object) *(required)*: Additional metadata about the toolkit
      - `created_at` (string) *(required)*: Creation date and time of the toolkit
      - `updated_at` (string) *(required)*: Last modification date and time of the toolkit
      - `description` (string) *(required)*: Human-readable description explaining the toolkit's purpose and functionality
      - `logo` (string) *(required)*: Image URL for the toolkit's branding
      - `app_url` (string,null): Link to the toolkit's main application or service website
      - `categories` (array<object>) *(required)*: List of categories associated with this toolkit
        - Array items:
          - ...
      - `triggers_count` (number) *(required)*: Count of available triggers in this toolkit
      - `tools_count` (number) *(required)*: Count of available tools in this toolkit
      - `version` (string) *(required)*: Version of the toolkit
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
      "auth_schemes": [
        "..."
      ],
      "composio_managed_auth_schemes": [
        "..."
      ],
      "is_local_toolkit": true,
      "no_auth": true,
      "deprecated": {
        "toolkitId": "..."
      },
      "meta": {
        "created_at": "...",
        "updated_at": "...",
        "description": "...",
        "logo": "...",
        "app_url": "...",
        "categories": "...",
        "triggers_count": "...",
        "tools_count": "...",
        "version": "..."
      }
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
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

#### 404 - Not found. The toolkits you are looking for do not exist.

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
curl -X GET "https://backend.composio.dev/api/v3/toolkits" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get toolkit by slug

**Documentation:** /reference/api-reference/toolkits/getToolkitsBySlug

Retrieves comprehensive information about a specific toolkit using its unique slug identifier. This endpoint provides detailed metadata, authentication configuration options, and feature counts for the requested toolkit.

---

## GET `/api/v3/toolkits/{slug}`

**Endpoint:** `https://backend.composio.dev/api/v3/toolkits/{slug}`

**Summary:** Get toolkit by slug

Retrieves comprehensive information about a specific toolkit using its unique slug identifier. This endpoint provides detailed metadata, authentication configuration options, and feature counts for the requested toolkit.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `slug` (string) *(required)*: Toolkit slug identifier

### Query Parameters

- `version` (string): Version of the toolkit

### Responses

#### 200 - Successfully retrieved detailed information about the requested toolkit, including authentication options, metadata, and feature counts.

**Response Schema:**

- `slug` (string) *(required)*: URL-friendly unique identifier for the toolkit
- `name` (string) *(required)*: Human-readable name of the toolkit
- `enabled` (boolean) *(required)*: Indicates if this toolkit is currently enabled and available for use
- `composio_managed_auth_schemes` (array<string>): List of authentication methods that Composio manages for this toolkit
- `is_local_toolkit` (boolean) *(required)*: DEPRECATED: This field is no longer meaningful and will always return false. It was previously used to indicate if a toolkit is specific to the current project.
- `auth_config_details` (array<object>): Complete authentication configuration details for each supported auth method
  - Array items:
    - `mode` (string) *(required)*: The type of authentication mode (e.g., oauth2, basic_auth, api_key)
    - `fields` (object) *(required)*: Field groups required for different authentication stages
      - `auth_config_creation` (object) *(required)*: Form fields needed when creating an authentication configuration
        - `required` (array<object>) *(required)*
          - Array items:
            - ...
        - `optional` (array<object>) *(required)*
          - Array items:
            - ...
      - `connected_account_initiation` (object) *(required)*: Form fields needed when connecting a user account with this authentication method
        - `required` (array<object>) *(required)*
          - Array items:
            - ...
        - `optional` (array<object>) *(required)*
          - Array items:
            - ...
    - `proxy` (object): Configuration for proxying authentication requests to external services
      - `base_url` (string) *(required)*: URL to which authentication requests will be proxied
    - `name` (string) *(required)*: Display name for this authentication method
    - `deprecated_auth_provider_details` (object): Authentication URL fields for OAuth 2.0 and OAuth 1.0. We don't recommend using this field for authentication and might break post Aug 31 2025.
      - `authorization_url` (string)
      - `token_url` (string)
- `base_url` (string): If evaluation of base URL needs some connection info (like shopify), please create the connection and get the base URL from there
- `meta` (object) *(required)*: Comprehensive metadata for the toolkit including dates, descriptions, and statistics
  - `created_at` (string) *(required)*: Creation date and time of the toolkit
  - `updated_at` (string) *(required)*: Last modification date and time of the toolkit
  - `description` (string) *(required)*: Human-readable description explaining the toolkit's purpose and functionality
  - `logo` (string) *(required)*: Image URL for the toolkit's branding
  - `app_url` (string,null): Link to the toolkit's main application or service website
  - `categories` (array<object>) *(required)*: List of categories associated with this toolkit
    - Array items:
      - `name` (string) *(required)*: Human-readable category name
      - `slug` (string) *(required)*: URL-friendly identifier for the category
  - `triggers_count` (number) *(required)*: Count of available triggers in this toolkit
  - `tools_count` (number) *(required)*: Count of available tools in this toolkit
  - `version` (string) *(required)*: Version of the toolkit
  - `available_versions` (array<string>) *(required)*: Available versions of the toolkit
- `get_current_user_endpoint` (string): Endpoint to get the current user
- `get_current_user_endpoint_method` (string): HTTP method to use when calling the get current user endpoint (e.g., GET, POST)
- `deprecated` (object) *(required)*
  - `toolkitId` (string) *(required)*
  - `getCurrentUserEndpoint` (string)
  - `rawProxyInfoByAuthSchemes` (array<object>) *(required)*

**Example Response:**

```json
{
  "slug": "string",
  "name": "string",
  "enabled": true,
  "composio_managed_auth_schemes": [
    "string"
  ],
  "is_local_toolkit": true,
  "auth_config_details": [
    {
      "mode": "string",
      "fields": {
        "auth_config_creation": "...",
        "connected_account_initiation": "..."
      },
      "proxy": {
        "base_url": "..."
      },
      "name": "string",
      "deprecated_auth_provider_details": {
        "authorization_url": "...",
        "token_url": "..."
      }
    }
  ],
  "base_url": "string",
  "meta": {
    "created_at": "string",
    "updated_at": "string",
    "description": "string",
    "logo": "string",
    "app_url": null,
    "categories": [
      {
        "name": "...",
        "slug": "..."
      }
    ],
    "triggers_count": 1,
    "tools_count": 1,
    "version": "string",
    "available_versions": [
      "string"
    ]
  },
  "get_current_user_endpoint": "string",
  "get_current_user_endpoint_method": "string",
  "deprecated": {
    "toolkitId": "string",
    "getCurrentUserEndpoint": "string",
    "rawProxyInfoByAuthSchemes": [
      {}
    ]
  }
}
```

#### 400 - Bad request. The toolkit slug may be invalid or in an incorrect format.

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

#### 404 - Not found. The requested toolkit does not exist or is not accessible to the authenticated project.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error. An unexpect ed error occurred while processing the request.

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
curl -X GET "https://backend.composio.dev/api/v3/toolkits/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List toolkit categories

**Documentation:** /reference/api-reference/toolkits/getToolkitsCategories

Retrieves a comprehensive list of all available toolkit categories from their latest versions. These categories can be used to filter toolkits by type or purpose when using the toolkit listing endpoint. Categories help organize toolkits into logical groups based on their functionality or industry focus.

---

## GET `/api/v3/toolkits/categories`

**Endpoint:** `https://backend.composio.dev/api/v3/toolkits/categories`

**Summary:** List toolkit categories

Retrieves a comprehensive list of all available toolkit categories from their latest versions. These categories can be used to filter toolkits by type or purpose when using the toolkit listing endpoint. Categories help organize toolkits into logical groups based on their functionality or industry focus.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Successfully retrieved a paginated list of all available toolkit categories with their identifiers and display names.

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `name` (string) *(required)*: Display name of the toolkit category
    - `id` (string) *(required)*: URL-friendly unique identifier for the category, used for filtering toolkits
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
      "id": "string"
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
}
```

#### 400 - Bad request. The request may be invalid or in an incorrect format.

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

#### 404 - Not found. The requested resource was not found.

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
curl -X GET "https://backend.composio.dev/api/v3/toolkits/categories" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get toolkits changelog

**Documentation:** /reference/api-reference/toolkits/getToolkitsChangelog

Retrieves the last 10 versions changelog for all toolkits. This endpoint provides version history and changelog information for each toolkit.

---

## GET `/api/v3/toolkits/changelog`

**Endpoint:** `https://backend.composio.dev/api/v3/toolkits/changelog`

**Summary:** Get toolkits changelog

Retrieves the last 10 versions changelog for all toolkits. This endpoint provides version history and changelog information for each toolkit.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Toolkits changelog retrieved successfully

**Response Schema:**

- `items` (array<object>) *(required)*: List of toolkits with their changelogs
  - Array items:
    - `slug` (string) *(required)*: Toolkit slug
    - `name` (string) *(required)*: Toolkit name
    - `display_name` (string) *(required)*: Toolkit display name
    - `versions` (array<object>) *(required)*: Array of version changelogs
      - Array items:
        - `version` (string) *(required)*: Version identifier
        - `changelog` (string) *(required)*: Changelog for this version

**Example Response:**

```json
{
  "items": [
    {
      "slug": "string",
      "name": "string",
      "display_name": "string",
      "versions": [
        "..."
      ]
    }
  ]
}
```

#### 400 - Bad request - invalid parameters

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - authentication required

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - the toolkits you are looking for do not exist

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
curl -X GET "https://backend.composio.dev/api/v3/toolkits/changelog" \
  -H "x-api-key: YOUR_API_KEY"
```

# Fetch multiple toolkits

**Documentation:** /reference/api-reference/toolkits/postToolkitsMulti

Retrieves a comprehensive list of toolkits of their latest versions that are available to the authenticated project. Toolkits represent integration points with external services and applications, each containing a collection of tools and triggers. This endpoint supports filtering by category and management type, as well as different sorting options. You can optionally specify a list of toolkit slugs to fetch specific toolkits.

---

## POST `/api/v3/toolkits/multi`

**Endpoint:** `https://backend.composio.dev/api/v3/toolkits/multi`

**Summary:** Fetch multiple toolkits

Retrieves a comprehensive list of toolkits of their latest versions that are available to the authenticated project. Toolkits represent integration points with external services and applications, each containing a collection of tools and triggers. This endpoint supports filtering by category and management type, as well as different sorting options. You can optionally specify a list of toolkit slugs to fetch specific toolkits.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `toolkits` (array<string>): Array of toolkit slug identifiers to retrieve
- `category` (string): Category ID or name to filter toolkits by
- `managed_by` (enum: "composio" | "all" | "project"): Entity responsible for managing the toolkits
- `sort_by` (enum: "usage" | "alphabetically"): Determines how toolkits should be sorted in the response
- `limit` (number,null)
- `cursor` (string)

**Example:**

```json
{
  "toolkits": [
    "string"
  ],
  "category": "string",
  "managed_by": "composio",
  "sort_by": "usage",
  "limit": null,
  "cursor": "string"
}
```

### Responses

#### 200 - Toolkits retrieved successfully. Returns a paginated list of available toolkits with detailed metadata.

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `slug` (string) *(required)*: URL-friendly unique identifier for the toolkit
    - `name` (string) *(required)*: Human-readable name of the toolkit
    - `auth_schemes` (array<string>): List of authentication methods supported by this toolkit
    - `composio_managed_auth_schemes` (array<string>): List of authentication methods that Composio manages for this toolkit
    - `is_local_toolkit` (boolean) *(required)*: DEPRECATED: This field is no longer meaningful and will always return false. It was previously used to indicate if a toolkit is specific to the current project.
    - `no_auth` (boolean): When true, this toolkit can be used without authentication
    - `deprecated` (object) *(required)*: Deprecated toolkit ID
      - `toolkitId` (string) *(required)*
    - `meta` (object) *(required)*: Additional metadata about the toolkit
      - `created_at` (string) *(required)*: Creation date and time of the toolkit
      - `updated_at` (string) *(required)*: Last modification date and time of the toolkit
      - `description` (string) *(required)*: Human-readable description explaining the toolkit's purpose and functionality
      - `logo` (string) *(required)*: Image URL for the toolkit's branding
      - `app_url` (string,null): Link to the toolkit's main application or service website
      - `categories` (array<object>) *(required)*: List of categories associated with this toolkit
        - Array items:
          - ...
      - `triggers_count` (number) *(required)*: Count of available triggers in this toolkit
      - `tools_count` (number) *(required)*: Count of available tools in this toolkit
      - `version` (string) *(required)*: Version of the toolkit
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
      "auth_schemes": [
        "..."
      ],
      "composio_managed_auth_schemes": [
        "..."
      ],
      "is_local_toolkit": true,
      "no_auth": true,
      "deprecated": {
        "toolkitId": "..."
      },
      "meta": {
        "created_at": "...",
        "updated_at": "...",
        "description": "...",
        "logo": "...",
        "app_url": "...",
        "categories": "...",
        "triggers_count": "...",
        "tools_count": "...",
        "version": "..."
      }
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
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

#### 404 - Not found. The toolkits you are looking for do not exist.

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
curl -X POST "https://backend.composio.dev/api/v3/toolkits/multi" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "toolkits": [
      "string"
    ],
    "category": "string",
    "managed_by": "composio",
    "sort_by": "usage",
    "limit": null,
    "cursor": "string"
  }'
```