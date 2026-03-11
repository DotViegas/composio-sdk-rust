# Projects (/reference/api-reference/projects)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Projects API endpoints

# Endpoints

| Endpoint                                                      | Quick Link                                                                                                                   |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `GET /api/v3/org/project/config`                              | [Get project configuration](/reference/api-reference/projects/getOrgProjectConfig)                                           |
| `PATCH /api/v3/org/project/config`                            | [Update project configuration](/reference/api-reference/projects/patchOrgProjectConfig)                                      |
| `POST /api/v3/org/owner/project/new`                          | [Create a new project](/reference/api-reference/projects/postOrgOwnerProjectNew)                                             |
| `GET /api/v3/org/owner/project/list`                          | [List all projects](/reference/api-reference/projects/getOrgOwnerProjectList)                                                |
| `GET /api/v3/org/owner/project/{nano_id}`                     | [Get project details by ID With Org Api key](/reference/api-reference/projects/getOrgOwnerProjectByNanoId)                   |
| `DELETE /api/v3/org/owner/project/{nano_id}`                  | [Delete a project](/reference/api-reference/projects/deleteOrgOwnerProjectByNanoId)                                          |
| `POST /api/v3/org/owner/project/{nano_id}/regenerate_api_key` | [Delete and generate new API key for project](/reference/api-reference/projects/postOrgOwnerProjectByNanoIdRegenerateApiKey) |

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

# Delete a project

**Documentation:** /reference/api-reference/projects/deleteOrgOwnerProjectByNanoId

Soft-deletes a project within the organization by its unique identifier. When a project is deleted, it is marked as deleted but not immediately removed from the database. This operation affects all resources associated with the project including API keys, webhook configurations, and connected services. This action cannot be undone through the API.

---

## DELETE `/api/v3/org/owner/project/{nano_id}`

**Endpoint:** `https://backend.composio.dev/api/v3/org/owner/project/{nano_id}`

**Summary:** Delete a project

Soft-deletes a project within the organization by its unique identifier. When a project is deleted, it is marked as deleted but not immediately removed from the database. This operation affects all resources associated with the project including API keys, webhook configurations, and connected services. This action cannot be undone through the API.

### Authentication

**OrgApiKeyAuth** - API Key in `header` header `x-org-api-key`

### Path Parameters

- `nano_id` (string (projectId)) *(required)*: Unique identifier (Nano ID) of the project to delete

### Responses

#### 200 - Project successfully deleted. The project has been marked as deleted in the system.

**Response Schema:**

- `status` (enum: "success") *(required)*: Status indicating successful deletion

**Example Response:**

```json
{
  "status": "success"
}
```

#### 400 - Bad request. The project ID may be invalid or in an incorrect format.

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

#### 403 - Forbidden. You do not have permission to delete this project.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found. The specified project does not exist or has already been deleted.

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
curl -X DELETE "https://backend.composio.dev/api/v3/org/owner/project/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get project details by ID With Org Api key

**Documentation:** /reference/api-reference/projects/getOrgOwnerProjectByNanoId

Retrieves detailed information about a specific project using its unique identifier. This endpoint provides complete project configuration including webhook URLs, creation and update timestamps, and webhook secrets. Use this endpoint to inspect project settings or verify project configuration.

---

## GET `/api/v3/org/owner/project/{nano_id}`

**Endpoint:** `https://backend.composio.dev/api/v3/org/owner/project/{nano_id}`

**Summary:** Get project details by ID With Org Api key

Retrieves detailed information about a specific project using its unique identifier. This endpoint provides complete project configuration including webhook URLs, creation and update timestamps, and webhook secrets. Use this endpoint to inspect project settings or verify project configuration.

### Authentication

**OrgApiKeyAuth** - API Key in `header` header `x-org-api-key`

### Path Parameters

- `nano_id` (string (projectId)) *(required)*: Unique identifier (Nano ID) of the project to retrieve

### Responses

#### 200 - Project retrieved successfully. Returns a complete project object with all configuration details and associated API keys.

**Response Schema:**

- `id` (string (projectId)) *(required)*: Unique identifier for the project
- `org_id` (string (orgId)) *(required)*: Identifier of the organization that owns this project
- `name` (string) *(required)*: Name of the project
- `email` (string) *(required)*: Email address associated with the project
- `created_at` (string) *(required)*: ISO timestamp when the project was created
- `updated_at` (string) *(required)*: ISO timestamp when the project was last updated
- `webhook_url` (string,null (uri)) *(required)*: DEPRECATED: Use GET /api/v3/webhook_subscriptions instead. Legacy project-level webhook URL.
- `event_webhook_url` (string,null (uri)) *(required)*: DEPRECATED: No longer used.
- `webhook_secret` (string,null) *(required)*: DEPRECATED: Use GET /api/v3/webhook_subscriptions instead. Legacy project-level webhook secret.
- `triggers_enabled` (boolean): Whether triggers are enabled for this project
- `last_subscribed_at` (string,null (date-time)): ISO timestamp when the project last subscribed to updates
- `is_new_webhook` (boolean): Deprecated: Please refer to webhook_version instead. True indicates if the webhook configuration is using the previous new format (V2). False indicates the oldest format (V1)
- `webhook_version` (enum: "V1" | "V2" | "V3") *(required)*: Payload format version for Pusher real-time events only. For webhook configuration, use GET /api/v3/webhook_subscriptions.
- `deleted` (boolean) *(required)*: Whether this project has been soft-deleted
- `api_keys` (array<object>) *(required)*: Array of API keys for the project, including their properties
  - Array items:
    - `id` (string) *(required)*: Unique identifier for the API key
    - `name` (string) *(required)*: User-defined name for the API key
    - `key` (string) *(required)*: The actual API key value used for authentication
    - `created_at` (string (date-time)) *(required)*: ISO 8601 timestamp when the API key was created

**Example Response:**

```json
{
  "id": "string",
  "org_id": "string",
  "name": "string",
  "email": "string",
  "created_at": "string",
  "updated_at": "string",
  "webhook_url": null,
  "event_webhook_url": null,
  "webhook_secret": null,
  "triggers_enabled": true,
  "last_subscribed_at": null,
  "is_new_webhook": true,
  "webhook_version": "V1",
  "deleted": true,
  "api_keys": [
    {
      "id": "string",
      "name": "string",
      "key": "string",
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

#### 400 - Bad request. This may occur if the project ID format is invalid.

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

#### 404 - Not found. The specified project does not exist or you do not have access to it.

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
curl -X GET "https://backend.composio.dev/api/v3/org/owner/project/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List all projects

**Documentation:** /reference/api-reference/projects/getOrgOwnerProjectList

Retrieves all projects belonging to the authenticated organization. Projects are returned in descending order of creation date (newest first). This endpoint is useful for displaying project selection in dashboards or for integrations that need to list all available projects.

---

## GET `/api/v3/org/owner/project/list`

**Endpoint:** `https://backend.composio.dev/api/v3/org/owner/project/list`

**Summary:** List all projects

Retrieves all projects belonging to the authenticated organization. Projects are returned in descending order of creation date (newest first). This endpoint is useful for displaying project selection in dashboards or for integrations that need to list all available projects.

### Authentication

**OrgApiKeyAuth** - API Key in `header` header `x-org-api-key`

### Query Parameters

- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Projects retrieved successfully with pagination. Returns an array of projects with pagination info.

**Response Schema:**

- `data` (array<object>) *(required)*
  - Array items:
    - `id` (string (projectId)) *(required)*: Unique identifier for the project
    - `org_id` (string (orgId)) *(required)*: Identifier of the organization that owns this project
    - `name` (string) *(required)*: Name of the project
    - `email` (string) *(required)*: Email address associated with the project
    - `created_at` (string) *(required)*: ISO timestamp when the project was created
    - `updated_at` (string) *(required)*: ISO timestamp when the project was last updated
    - `webhook_url` (string,null (uri)) *(required)*: DEPRECATED: Use GET /api/v3/webhook_subscriptions instead. Legacy project-level webhook URL.
    - `event_webhook_url` (string,null (uri)) *(required)*: DEPRECATED: No longer used.
    - `webhook_secret` (string,null) *(required)*: DEPRECATED: Use GET /api/v3/webhook_subscriptions instead. Legacy project-level webhook secret.
    - `triggers_enabled` (boolean): Whether triggers are enabled for this project
    - `last_subscribed_at` (string,null (date-time)): ISO timestamp when the project last subscribed to updates
    - `is_new_webhook` (boolean): Deprecated: Please refer to webhook_version instead. True indicates if the webhook configuration is using the previous new format (V2). False indicates the oldest format (V1)
    - `webhook_version` (enum: "V1" | "V2" | "V3") *(required)*: Payload format version for Pusher real-time events only. For webhook configuration, use GET /api/v3/webhook_subscriptions.
    - `deleted` (boolean) *(required)*: Whether this project has been soft-deleted
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

**Example Response:**

```json
{
  "data": [
    {
      "id": "string",
      "org_id": "string",
      "name": "string",
      "email": "string",
      "created_at": "string",
      "updated_at": "string",
      "webhook_url": null,
      "event_webhook_url": null,
      "webhook_secret": null,
      "triggers_enabled": true,
      "last_subscribed_at": null,
      "is_new_webhook": true,
      "webhook_version": "V1",
      "deleted": true
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
}
```

#### 400 - Bad request. This may occur if there are invalid query parameters or the request is malformed.

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
curl -X GET "https://backend.composio.dev/api/v3/org/owner/project/list" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get project configuration

**Documentation:** /reference/api-reference/projects/getOrgProjectConfig

Retrieves the current project configuration including 2FA settings.

---

## GET `/api/v3/org/project/config`

**Endpoint:** `https://backend.composio.dev/api/v3/org/project/config`

**Summary:** Get project configuration

Retrieves the current project configuration including 2FA settings.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Project configuration retrieved successfully.

**Response Schema:**

- `is_2FA_enabled` (boolean) *(required)*
- `logo_url` (string)
- `display_name` (string)
- `mask_secret_keys_in_connected_account` (boolean) *(required)*
- `log_visibility_setting` (enum: "show_all" | "dont_store_data") *(required)*
- `require_mcp_api_key` (boolean)
- `is_composio_link_enabled_for_managed_auth` (boolean): Whether to enable composio link for managed authentication. This key will be deprecated in the future. Please don't use this key.
- `signed_url_file_expiry_in_seconds` (number)

**Example Response:**

```json
{
  "is_2FA_enabled": true,
  "logo_url": "string",
  "display_name": "string",
  "mask_secret_keys_in_connected_account": true,
  "log_visibility_setting": "show_all",
  "require_mcp_api_key": true,
  "is_composio_link_enabled_for_managed_auth": true,
  "signed_url_file_expiry_in_seconds": 1
}
```

#### 400 - Bad request. The project configuration data may be invalid.

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

#### 404 - Project not found. The specified project does not exist or has been deleted.

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
curl -X GET "https://backend.composio.dev/api/v3/org/project/config" \
  -H "x-api-key: YOUR_API_KEY"
```

# Update project configuration

**Documentation:** /reference/api-reference/projects/patchOrgProjectConfig

Updates the project configuration settings.

---

## PATCH `/api/v3/org/project/config`

**Endpoint:** `https://backend.composio.dev/api/v3/org/project/config`

**Summary:** Update project configuration

Updates the project configuration settings.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `is_2FA_enabled` (boolean)
- `logo_url` (string)
- `display_name` (string)
- `mask_secret_keys_in_connected_account` (boolean)
- `log_visibility_setting` (enum: "show_all" | "dont_store_data")
- `require_mcp_api_key` (boolean)
- `is_composio_link_enabled_for_managed_auth` (boolean): Whether to enable composio link for managed authentication. This key will be deprecated in the future. Please don't use this key.
- `signed_url_file_expiry_in_seconds` (number)

**Example:**

```json
{
  "is_2FA_enabled": true,
  "logo_url": "string",
  "display_name": "string",
  "mask_secret_keys_in_connected_account": true,
  "log_visibility_setting": "show_all",
  "require_mcp_api_key": true,
  "is_composio_link_enabled_for_managed_auth": true,
  "signed_url_file_expiry_in_seconds": 1
}
```

### Responses

#### 200 - Project configuration updated successfully.

**Response Schema:**

- `is_2FA_enabled` (boolean) *(required)*
- `logo_url` (string)
- `display_name` (string)
- `mask_secret_keys_in_connected_account` (boolean) *(required)*
- `log_visibility_setting` (enum: "show_all" | "dont_store_data") *(required)*
- `require_mcp_api_key` (boolean)
- `is_composio_link_enabled_for_managed_auth` (boolean): Whether to enable composio link for managed authentication. This key will be deprecated in the future. Please don't use this key.
- `signed_url_file_expiry_in_seconds` (number)

**Example Response:**

```json
{
  "is_2FA_enabled": true,
  "logo_url": "string",
  "display_name": "string",
  "mask_secret_keys_in_connected_account": true,
  "log_visibility_setting": "show_all",
  "require_mcp_api_key": true,
  "is_composio_link_enabled_for_managed_auth": true,
  "signed_url_file_expiry_in_seconds": 1
}
```

#### 400 - Bad request. The configuration data may be invalid or missing.

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

#### 404 - Project not found. The specified project does not exist or has been deleted.

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
curl -X PATCH "https://backend.composio.dev/api/v3/org/project/config" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "is_2FA_enabled": true,
    "logo_url": "string",
    "display_name": "string",
    "mask_secret_keys_in_connected_account": true,
    "log_visibility_setting": "show_all",
    "require_mcp_api_key": true,
    "is_composio_link_enabled_for_managed_auth": true,
    "signed_url_file_expiry_in_seconds": 1
  }'
```

# Delete and generate new API key for project

**Documentation:** /reference/api-reference/projects/postOrgOwnerProjectByNanoIdRegenerateApiKey

Generates a new API key for the specified project, invalidating any existing API keys for that project. This operation creates a fresh API key with a new random name and key value. All existing API keys for this project will be marked as deleted.

---

## POST `/api/v3/org/owner/project/{nano_id}/regenerate_api_key`

**Endpoint:** `https://backend.composio.dev/api/v3/org/owner/project/{nano_id}/regenerate_api_key`

**Summary:** Delete and generate new API key for project

Generates a new API key for the specified project, invalidating any existing API keys for that project. This operation creates a fresh API key with a new random name and key value. All existing API keys for this project will be marked as deleted.

### Authentication

**OrgApiKeyAuth** - API Key in `header` header `x-org-api-key`

### Path Parameters

- `nano_id` (string (projectId)) *(required)*: Unique identifier (Nano ID) of the project to regenerate API key for

### Responses

#### 200 - API key regenerated successfully. Returns the new API key details.

**Response Schema:**

- `api_key` (object) *(required)*: The newly generated API key for this project
  - `id` (string) *(required)*: Unique identifier for the API key
  - `name` (string) *(required)*: Name of the API key
  - `key` (string) *(required)*: The newly generated API key value
  - `created_at` (string) *(required)*: ISO timestamp when the API key was created
- `message` (string) *(required)*: Success message

**Example Response:**

```json
{
  "api_key": {
    "id": "string",
    "name": "string",
    "key": "string",
    "created_at": "string"
  },
  "message": "string"
}
```

#### 400 - Bad request. The project ID may be invalid or in an incorrect format.

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

#### 404 - Not found. The specified project does not exist or you do not have access to it.

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
curl -X POST "https://backend.composio.dev/api/v3/org/owner/project/string/regenerate_api_key" \
  -H "x-api-key: YOUR_API_KEY"
```

# Create a new project

**Documentation:** /reference/api-reference/projects/postOrgOwnerProjectNew

Creates a new project within the authenticated user's organization using the specified name. Projects are isolated environments within your organization, each with their own API keys, webhook configurations, and resources. Use this endpoint to create additional projects for different environments (e.g., development, staging, production) or for separate applications.

---

## POST `/api/v3/org/owner/project/new`

**Endpoint:** `https://backend.composio.dev/api/v3/org/owner/project/new`

**Summary:** Create a new project

Creates a new project within the authenticated user's organization using the specified name. Projects are isolated environments within your organization, each with their own API keys, webhook configurations, and resources. Use this endpoint to create additional projects for different environments (e.g., development, staging, production) or for separate applications.

### Authentication

**OrgApiKeyAuth** - API Key in `header` header `x-org-api-key`

### Request Body

**Schema:**

- `name` (string) *(required)*: A unique name for your project that follows the required format rules
- `should_create_api_key` (boolean): Whether to create an API key for the project. If true, the API key will be created and returned in the response.
- `config` (object): Configuration for the project
  - `is_2FA_enabled` (boolean) *(required)*
  - `logo_url` (string)
  - `display_name` (string)
  - `mask_secret_keys_in_connected_account` (boolean) *(required)*
  - `log_visibility_setting` (enum: "show_all" | "dont_store_data") *(required)*
  - `require_mcp_api_key` (boolean)
  - `is_composio_link_enabled_for_managed_auth` (boolean): Whether to enable composio link for managed authentication. This key will be deprecated in the future. Please don't use this key.
  - `signed_url_file_expiry_in_seconds` (number)

**Example:**

```json
{
  "name": "string",
  "should_create_api_key": false,
  "config": {
    "is_2FA_enabled": true,
    "logo_url": "string",
    "display_name": "string",
    "mask_secret_keys_in_connected_account": true,
    "log_visibility_setting": "show_all",
    "require_mcp_api_key": true,
    "is_composio_link_enabled_for_managed_auth": true,
    "signed_url_file_expiry_in_seconds": 1
  }
}
```

### Responses

#### 200 - Project successfully created. Returns the complete project object with generated IDs, webhook secrets, and configuration.

**Response Schema:**

- `id` (string (projectId)) *(required)*: Unique identifier for the project
- `name` (string): Name of the project
- `api_key` (string,null) *(required)*: API key for the project

**Example Response:**

```json
{
  "id": "string",
  "name": "string",
  "api_key": null
}
```

#### 400 - Bad request. This may occur if the project name format is invalid, too short, or contains invalid characters.

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
curl -X POST "https://backend.composio.dev/api/v3/org/owner/project/new" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "string",
    "should_create_api_key": false,
    "config": {
      "is_2FA_enabled": true,
      "logo_url": "string",
      "display_name": "string",
      "mask_secret_keys_in_connected_account": true,
      "log_visibility_setting": "show_all",
      "require_mcp_api_key": true,
      "is_composio_link_enabled_for_managed_auth": true,
      "signed_url_file_expiry_in_seconds": 1
    }
  }'
```