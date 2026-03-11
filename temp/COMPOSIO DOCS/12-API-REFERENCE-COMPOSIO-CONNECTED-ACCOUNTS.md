# Connected Accounts (/reference/api-reference/connected-accounts)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Connected account management

# Endpoints

| Endpoint                                           | Quick Link                                                                                                                         |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `GET /api/v3/connected_accounts`                   | [List connected accounts with optional filters](/reference/api-reference/connected-accounts/getConnectedAccounts)                  |
| `POST /api/v3/connected_accounts`                  | [Create a new connected account](/reference/api-reference/connected-accounts/postConnectedAccounts)                                |
| `GET /api/v3/connected_accounts/{nanoid}`          | [Get connected account details by ID](/reference/api-reference/connected-accounts/getConnectedAccountsByNanoid)                    |
| `DELETE /api/v3/connected_accounts/{nanoid}`       | [Delete a connected account](/reference/api-reference/connected-accounts/deleteConnectedAccountsByNanoid)                          |
| `PATCH /api/v3/connected_accounts/{nanoId}/status` | [Enable or disable a connected account](/reference/api-reference/connected-accounts/patchConnectedAccountsByNanoIdStatus)          |
| `POST /api/v3/connected_accounts/{nanoid}/refresh` | [Refresh authentication for a connected account](/reference/api-reference/connected-accounts/postConnectedAccountsByNanoidRefresh) |
| `POST /api/v3/connected_accounts/link`             | [Create a new auth link session](/reference/api-reference/connected-accounts/postConnectedAccountsLink)                            |

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
# Delete a connected account

**Documentation:** /reference/api-reference/connected-accounts/deleteConnectedAccountsByNanoid

Soft-deletes a connected account by marking it as deleted in the database. This prevents the account from being used for API calls but preserves the record for audit purposes.

---

## DELETE `/api/v3/connected_accounts/{nanoid}`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts/{nanoid}`

**Summary:** Delete a connected account

Soft-deletes a connected account by marking it as deleted in the database. This prevents the account from being used for API calls but preserves the record for audit purposes.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (connectedAccountId)) *(required)*: The unique identifier (nanoid) of the connected account

### Responses

#### 200 - Successfully deleted the connected account. The account is marked as deleted but retained in the database for historical purposes.

**Response Schema:**

- `success` (boolean) *(required)*: Indicates whether the connected account was successfully deleted

**Example Response:**

```json
{
  "success": true
}
```

#### 400 - Bad request - Invalid nanoid format or other validation error

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication failed

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - Insufficient permissions to delete this connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Connected account not found - The specified account does not exist or has already been deleted

**Response Schema:**


#### 500 - Internal server error - Failed to delete the connected account due to a server-side issue

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
curl -X DELETE "https://backend.composio.dev/api/v3/connected_accounts/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List connected accounts with optional filters

**Documentation:** /reference/api-reference/connected-accounts/getConnectedAccounts

Retrieves all connected accounts for your project. Connected accounts represent authenticated user connections to external services (e.g., a user's Gmail account, Slack workspace). Filter by toolkit, status, user ID, or auth config to find specific connections.

---

## GET `/api/v3/connected_accounts`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts`

**Summary:** List connected accounts with optional filters

Retrieves all connected accounts for your project. Connected accounts represent authenticated user connections to external services (e.g., a user's Gmail account, Slack workspace). Filter by toolkit, status, user ID, or auth config to find specific connections.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `toolkit_slugs` (array,null): The toolkit slugs of the connected accounts
- `statuses` (array,null): The status of the connected account
- `cursor` (string,null): The cursor to paginate through the connected accounts
- `limit` (number,null): The limit of the connected accounts to return
- `user_ids` (array,null): The user ids of the connected accounts
- `auth_config_ids` (array,null): The auth config ids of the connected accounts
- `connected_account_ids` (array,null): The connected account ids to filter by
- `order_by` (enum: "created_at" | "updated_at"): The order by of the connected accounts
- `order_direction` (enum: "asc" | "desc"): The order direction of the connected accounts

### Responses

#### 200 - Successfully retrieved connected accounts

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `toolkit` (object) *(required)*
      - `slug` (string) *(required)*: The slug of the toolkit
    - `auth_config` (object) *(required)*
      - `id` (string (authConfigId)) *(required)*: The id of the auth config
      - `auth_scheme` (enum: "OAUTH2" | "OAUTH1" | "API_KEY" | ...) *(required)*: the authScheme is part of the connection state use it there
      - `is_composio_managed` (boolean) *(required)*: Whether the auth config is managed by Composio
      - `is_disabled` (boolean) *(required)*: Whether the auth config is disabled
      - `deprecated` (object)
        - `uuid` (string (uuid)) *(required)*: The uuid of the auth config
    - `id` (string (connectedAccountId)) *(required)*: The id of the connection
    - `user_id` (string) *(required)*: This is deprecated, we will not be providing userId from this api anymore, you will only be able to read via userId not get it back
    - `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*: The status of the connection
    - `created_at` (string) *(required)*: The created at of the connection
    - `updated_at` (string) *(required)*: The updated at of the connection
    - `state` (object) *(required)*: The state of the connection
      - `authScheme` (enum: "OAUTH1" | "OAUTH2" | "API_KEY" | ...) *(required)*
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
        - `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*
        - `oauth_token` (string)
        - `authUri` (string)
        - `oauth_token_secret` (string)
        - `redirectUrl` (string)
        - `callbackUrl` (string)
        - `oauth_verifier` (string)
        - `consumer_key` (string)
        - `callback_url` (string)
        - `error` (string)
        - `error_description` (string)
        - `expired_at` (string)
    - `data` (object) *(required)*: This is deprecated, use `state` instead
    - `status_reason` (string,null) *(required)*: The reason the connection status changed. Possible reasons: Connection initiation did not complete within 10 minutes, Permanent auth error during token refresh, Max auth failures reached, OAuth callback failed during token exchange, Connection status updated by user, Auth config is disabled
    - `is_disabled` (boolean) *(required)*: Whether the connection is disabled
    - `test_request_endpoint` (string): The endpoint to make test request for verification
    - `deprecated` (object)
      - `labels` (array<string>) *(required)*: The labels of the connection
      - `uuid` (string (uuid)) *(required)*: The uuid of the connection
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

**Example Response:**

```json
{
  "items": [
    {
      "toolkit": {
        "slug": "..."
      },
      "auth_config": {
        "id": "...",
        "auth_scheme": "...",
        "is_composio_managed": "...",
        "is_disabled": "...",
        "deprecated": "..."
      },
      "id": "string",
      "user_id": "string",
      "status": "INITIALIZING",
      "created_at": "string",
      "updated_at": "string",
      "state": {
        "authScheme": "...",
        "val": "..."
      },
      "data": {},
      "status_reason": null,
      "is_disabled": true,
      "test_request_endpoint": "string",
      "deprecated": {
        "labels": "...",
        "uuid": "..."
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

#### 422 - Unprocessable entity

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
curl -X GET "https://backend.composio.dev/api/v3/connected_accounts" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get connected account details by ID

**Documentation:** /reference/api-reference/connected-accounts/getConnectedAccountsByNanoid

Retrieves comprehensive details of a connected account, including authentication configuration, connection status, and all parameters needed for API requests.

---

## GET `/api/v3/connected_accounts/{nanoid}`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts/{nanoid}`

**Summary:** Get connected account details by ID

Retrieves comprehensive details of a connected account, including authentication configuration, connection status, and all parameters needed for API requests.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (connectedAccountId)) *(required)*: The unique identifier (nanoid) of the connected account

### Responses

#### 200 - Successfully retrieved connected account details with all authentication parameters and connection status

**Response Schema:**

- `toolkit` (object) *(required)*
  - `slug` (string) *(required)*: The slug of the toolkit
- `auth_config` (object) *(required)*
  - `id` (string (authConfigId)) *(required)*: The id of the auth config
  - `auth_scheme` (enum: "OAUTH2" | "OAUTH1" | "API_KEY" | ...) *(required)*: the authScheme is part of the connection state use it there
  - `is_composio_managed` (boolean) *(required)*: Whether the auth config is managed by Composio
  - `is_disabled` (boolean) *(required)*: Whether the auth config is disabled
  - `deprecated` (object)
    - `uuid` (string (uuid)) *(required)*: The uuid of the auth config
- `id` (string (connectedAccountId)) *(required)*: The id of the connection
- `user_id` (string) *(required)*: This is deprecated, we will not be providing userId from this api anymore, you will only be able to read via userId not get it back
- `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*: The status of the connection
- `created_at` (string) *(required)*: The created at of the connection
- `updated_at` (string) *(required)*: The updated at of the connection
- `state` (object) *(required)*: The state of the connection
  - `authScheme` (enum: "OAUTH1" | "OAUTH2" | "API_KEY" | ...) *(required)*
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
    - `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*
    - `oauth_token` (string)
    - `authUri` (string)
    - `oauth_token_secret` (string)
    - `redirectUrl` (string)
    - `callbackUrl` (string)
    - `oauth_verifier` (string)
    - `consumer_key` (string)
    - `callback_url` (string)
    - `error` (string)
    - `error_description` (string)
    - `expired_at` (string)
- `data` (object) *(required)*: This is deprecated, use `state` instead
- `status_reason` (string,null) *(required)*: The reason the connection status changed. Possible reasons: Connection initiation did not complete within 10 minutes, Permanent auth error during token refresh, Max auth failures reached, OAuth callback failed during token exchange, Connection status updated by user, Auth config is disabled
- `is_disabled` (boolean) *(required)*: Whether the connection is disabled
- `test_request_endpoint` (string): The endpoint to make test request for verification
- `deprecated` (object)
  - `labels` (array<string>) *(required)*: The labels of the connection
  - `uuid` (string (uuid)) *(required)*: The uuid of the connection
- `params` (object) *(required)*: The initialization data of the connection, including configuration parameters

**Example Response:**

```json
{
  "toolkit": {
    "slug": "string"
  },
  "auth_config": {
    "id": "string",
    "auth_scheme": "OAUTH2",
    "is_composio_managed": true,
    "is_disabled": true,
    "deprecated": {
      "uuid": "550e8400-e29b-41d4-a716-446655440000"
    }
  },
  "id": "string",
  "user_id": "string",
  "status": "INITIALIZING",
  "created_at": "string",
  "updated_at": "string",
  "state": {
    "authScheme": "OAUTH1",
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
      "status": "INITIALIZING",
      "oauth_token": "string",
      "authUri": "string",
      "oauth_token_secret": "string",
      "redirectUrl": "string",
      "callbackUrl": "string",
      "oauth_verifier": "string",
      "consumer_key": "string",
      "callback_url": "string",
      "error": "string",
      "error_description": "string",
      "expired_at": "string"
    }
  },
  "data": {},
  "status_reason": null,
  "is_disabled": true,
  "test_request_endpoint": "string",
  "deprecated": {
    "labels": [
      "string"
    ],
    "uuid": "550e8400-e29b-41d4-a716-446655440000"
  },
  "params": {}
}
```

#### 400 - Bad request - Invalid nanoid format or other validation error

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication failed

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - Insufficient permissions to access this connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Connected account does not exist or was deleted

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error - Failed to retrieve connected account details

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

### Example cURL Request

```bash
curl -X GET "https://backend.composio.dev/api/v3/connected_accounts/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# Enable or disable a connected account

**Documentation:** /reference/api-reference/connected-accounts/patchConnectedAccountsByNanoIdStatus

Updates the status of a connected account to either enabled (active) or disabled (inactive). Disabled accounts cannot be used for API calls but remain in the database.

---

## PATCH `/api/v3/connected_accounts/{nanoId}/status`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts/{nanoId}/status`

**Summary:** Enable or disable a connected account

Updates the status of a connected account to either enabled (active) or disabled (inactive). Disabled accounts cannot be used for API calls but remain in the database.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoId` (string (connectedAccountId)) *(required)*: The unique identifier of the connected account

### Request Body

**Schema:**

- `enabled` (boolean) *(required)*: Set to true to enable the account or false to disable it

**Example:**

```json
{
  "enabled": true
}
```

### Responses

#### 200 - Successfully updated the connected account status. If enabled=true, the account status is set to ACTIVE; if enabled=false, the status is set to INACTIVE.

**Response Schema:**

- `success` (boolean) *(required)*: Indicates whether the connected account status was successfully updated

**Example Response:**

```json
{
  "success": true
}
```

#### 400 - Bad request - Invalid nanoid format or invalid request body

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication failed

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - Insufficient permissions to update this connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Connected account not found - The specified account does not exist or has been deleted

**Response Schema:**


#### 500 - Internal server error - Failed to update the connected account status due to a server-side issue

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
curl -X PATCH "https://backend.composio.dev/api/v3/connected_accounts/string/status" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": true
  }'
```

# Create a new connected account

**Documentation:** /reference/api-reference/connected-accounts/postConnectedAccounts

Initiates a new connection to an external service for a user. For OAuth-based toolkits, this returns a redirect URL to complete authentication. For API key-based toolkits, provide the credentials directly in the request body. Use the `user_id` field to associate the connection with a specific user in your system.

---

## POST `/api/v3/connected_accounts`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts`

**Summary:** Create a new connected account

Initiates a new connection to an external service for a user. For OAuth-based toolkits, this returns a redirect URL to complete authentication. For API key-based toolkits, provide the credentials directly in the request body. Use the `user_id` field to associate the connection with a specific user in your system.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `auth_config` (object) *(required)*
  - `id` (string (authConfigId)) *(required)*: The auth config id of the app (must be a valid auth config id)
- `connection` (object) *(required)*
  - `state` (object): The state of the connected account
    - `authScheme` (enum: "OAUTH1" | "OAUTH2" | "API_KEY" | ...) *(required)*
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
      - `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*
      - `oauth_token` (string)
      - `authUri` (string)
      - `oauth_token_secret` (string)
      - `redirectUrl` (string)
      - `callbackUrl` (string)
      - `oauth_verifier` (string)
      - `consumer_key` (string)
      - `callback_url` (string)
      - `error` (string)
      - `error_description` (string)
      - `expired_at` (string)
  - `data` (object): DEPRECATED: This parameter will be removed in a future version. Please use state instead.
  - `user_id` (string): The user id of the connected account
  - `callback_url` (string (uri)): The URL to redirect to after connection completion
  - `redirect_uri` (string (uri)): DEPRECATED: This parameter will be removed in a future version. Please use callback_url instead.
  - `deprecated_is_v1_rerouted` (boolean): DEPRECATED: This parameter will be removed in a future version.
- `validate_credentials` (boolean): [EXPERIMENTAL] Whether to validate the provided credentials, validates only for API Key Auth scheme

**Example:**

```json
{
  "auth_config": {
    "id": "string"
  },
  "connection": {
    "state": {
      "authScheme": "OAUTH1",
      "val": {
        "subdomain": "...",
        "your-domain": "...",
        "region": "...",
        "shop": "...",
        "account_url": "...",
        "COMPANYDOMAIN": "...",
        "extension": "...",
        "form_api_base_url": "...",
        "instanceEndpoint": "...",
        "api_url": "...",
        "borneo_dashboard_url": "...",
        "proxy_username": "...",
        "proxy_password": "...",
        "domain": "...",
        "version": "...",
        "dc": "...",
        "site_name": "...",
        "instanceName": "...",
        "account_id": "...",
        "your_server": "...",
        "server_location": "...",
        "base_url": "...",
        "status": "...",
        "oauth_token": "...",
        "authUri": "...",
        "oauth_token_secret": "...",
        "redirectUrl": "...",
        "callbackUrl": "...",
        "oauth_verifier": "...",
        "consumer_key": "...",
        "callback_url": "...",
        "error": "...",
        "error_description": "...",
        "expired_at": "..."
      }
    },
    "data": {},
    "user_id": "default",
    "callback_url": "https://example.com",
    "redirect_uri": "https://example.com",
    "deprecated_is_v1_rerouted": false
  },
  "validate_credentials": false
}
```

### Responses

#### 201 - Successfully created connected account

**Response Schema:**

- `id` (string (connectedAccountId)) *(required)*: The id of the connected account
- `connectionData` (object) *(required)*: The connection data of the connected account
  - `authScheme` (enum: "OAUTH1" | "OAUTH2" | "API_KEY" | ...) *(required)*
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
    - `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*
    - `oauth_token` (string)
    - `authUri` (string)
    - `oauth_token_secret` (string)
    - `redirectUrl` (string)
    - `callbackUrl` (string)
    - `oauth_verifier` (string)
    - `consumer_key` (string)
    - `callback_url` (string)
    - `error` (string)
    - `error_description` (string)
    - `expired_at` (string)
- `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*: DEPRECATED: This field will be removed in a future version
- `redirect_url` (string,null) *(required)*: DEPRECATED: This field will be removed in a future version
- `redirect_uri` (string,null) *(required)*: DEPRECATED: This field will be removed in a future version
- `deprecated` (object) *(required)*: DEPRECATED: This field will be removed in a future version. Please use id and auth_config.id instead.
  - `uuid` (string (uuid)) *(required)*: The uuid of the connected account
  - `authConfigUuid` (string (uuid)) *(required)*: The uuid of the auth config

**Example Response:**

```json
{
  "id": "string",
  "connectionData": {
    "authScheme": "OAUTH1",
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
      "status": "INITIALIZING",
      "oauth_token": "string",
      "authUri": "string",
      "oauth_token_secret": "string",
      "redirectUrl": "string",
      "callbackUrl": "string",
      "oauth_verifier": "string",
      "consumer_key": "string",
      "callback_url": "string",
      "error": "string",
      "error_description": "string",
      "expired_at": "string"
    }
  },
  "status": "INITIALIZING",
  "redirect_url": null,
  "redirect_uri": null,
  "deprecated": {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "authConfigUuid": "550e8400-e29b-41d4-a716-446655440000"
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

### Example cURL Request

```bash
curl -X POST "https://backend.composio.dev/api/v3/connected_accounts" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "auth_config": {
      "id": "string"
    },
    "connection": {
      "state": {
        "authScheme": "OAUTH1",
        "val": {
          "subdomain": "...",
          "your-domain": "...",
          "region": "...",
          "shop": "...",
          "account_url": "...",
          "COMPANYDOMAIN": "...",
          "extension": "...",
          "form_api_base_url": "...",
          "instanceEndpoint": "...",
          "api_url": "...",
          "borneo_dashboard_url": "...",
          "proxy_username": "...",
          "proxy_password": "...",
          "domain": "...",
          "version": "...",
          "dc": "...",
          "site_name": "...",
          "instanceName": "...",
          "account_id": "...",
          "your_server": "...",
          "server_location": "...",
          "base_url": "...",
          "status": "...",
          "oauth_token": "...",
          "authUri": "...",
          "oauth_token_secret": "...",
          "redirectUrl": "...",
          "callbackUrl": "...",
          "oauth_verifier": "...",
          "consumer_key": "...",
          "callback_url": "...",
          "error": "...",
          "error_description": "...",
          "expired_at": "..."
        }
      },
      "data": {},
      "user_id": "default",
      "callback_url": "https://example.com",
      "redirect_uri": "https://example.com",
      "deprecated_is_v1_rerouted": false
    },
    "validate_credentials": false
  }'
```

# Refresh authentication for a connected account

**Documentation:** /reference/api-reference/connected-accounts/postConnectedAccountsByNanoidRefresh

Initiates a new authentication flow for a connected account when credentials have expired or become invalid. This may generate a new authentication URL for OAuth flows or refresh tokens for other auth schemes.

---

## POST `/api/v3/connected_accounts/{nanoid}/refresh`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts/{nanoid}/refresh`

**Summary:** Refresh authentication for a connected account

Initiates a new authentication flow for a connected account when credentials have expired or become invalid. This may generate a new authentication URL for OAuth flows or refresh tokens for other auth schemes.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `nanoid` (string (connectedAccountId)) *(required)*: The unique identifier of the connected account

### Query Parameters

- `redirect_url` (string (uri)): 

### Request Body

**Schema:**

- `redirect_url` (string (uri))
- `validate_credentials` (boolean): [EXPERIMENTAL] Whether to validate the provided credentials, validates only for API Key Auth scheme

**Example:**

```json
{
  "redirect_url": "https://example.com",
  "validate_credentials": false
}
```

### Responses

#### 200 - Successfully refreshed the connected account authentication. For OAuth flows, a new redirect URL is provided.

**Response Schema:**

- `id` (string) *(required)*: The unique identifier of the connected account
- `status` (enum: "INITIALIZING" | "INITIATED" | "ACTIVE" | ...) *(required)*: The current status of the connected account (e.g., active, pending, failed)
- `redirect_url` (string,null) *(required)*: The URL to which the user should be redirected to complete the authentication process (null for auth schemes that do not require redirection)

**Example Response:**

```json
{
  "id": "string",
  "status": "INITIALIZING",
  "redirect_url": null
}
```

#### 400 - Bad request - Invalid nanoid format or the account cannot be refreshed in its current state

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized - Authentication failed

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden - Insufficient permissions to refresh this connected account

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found - Connected account does not exist or was deleted

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal server error - Failed to refresh the connected account authentication

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

### Example cURL Request

```bash
curl -X POST "https://backend.composio.dev/api/v3/connected_accounts/string/refresh" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "redirect_url": "https://example.com",
    "validate_credentials": false
  }'
```

# Create a new auth link session

**Documentation:** /reference/api-reference/connected-accounts/postConnectedAccountsLink

Creates a new authentication link session that users can use to connect their accounts

---

## POST `/api/v3/connected_accounts/link`

**Endpoint:** `https://backend.composio.dev/api/v3/connected_accounts/link`

**Summary:** Create a new auth link session

Creates a new authentication link session that users can use to connect their accounts

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `auth_config_id` (string (authConfigId)) *(required)*: The auth config id to create a link for
- `user_id` (string) *(required)*: The user id to create a link for
- `callback_url` (string): The callback url to create a link for
- `connection_data` (object): Optional data to pre-fill connection fields with default values
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
  - `oauth_token` (string)
  - `authUri` (string)
  - `oauth_token_secret` (string)
  - `redirectUrl` (string)
  - `callbackUrl` (string)
  - `oauth_verifier` (string)
  - `consumer_key` (string)
  - `callback_url` (string)
  - `error` (string)
  - `error_description` (string)
  - `expired_at` (string)
  - `state_prefix` (string): The oauth2 state prefix for the connection
  - `long_redirect_url` (boolean): Whether to return the redirect url without shortening
  - `code_verifier` (string)
  - `finalRedirectUri` (string)
  - `webhook_signature` (string)
  - `access_token` (string)
  - `id_token` (string)
  - `token_type` (string)
  - `refresh_token` (string,null)
  - `expires_in` (any)
  - `scope` (any)
  - `authed_user` (object): for slack user scopes
    - `access_token` (string)
    - `scope` (string)
  - `generic_api_key` (string)
  - `api_key` (string)
  - `bearer_token` (string)
  - `basic_encoded` (string)
  - `username` (string)
  - `password` (string)
  - `token` (string)
  - `composio_link_redirect_url` (string)
  - `credentials_json` (string)
  - `sessionId` (string)
  - `devKey` (string)
  - `application_id` (string)
  - `installation_id` (string)
  - `private_key` (string)
  - `client_id` (string): Dynamically registered client ID
  - `client_secret` (string): Dynamically registered client secret
  - `client_id_issued_at` (number)
  - `client_secret_expires_at` (number)

**Example:**

```json
{
  "auth_config_id": "string",
  "user_id": "string",
  "callback_url": "string",
  "connection_data": {
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
    "oauth_token": "string",
    "authUri": "string",
    "oauth_token_secret": "string",
    "redirectUrl": "string",
    "callbackUrl": "string",
    "oauth_verifier": "string",
    "consumer_key": "string",
    "callback_url": "string",
    "error": "string",
    "error_description": "string",
    "expired_at": "string",
    "state_prefix": "string",
    "long_redirect_url": true,
    "code_verifier": "string",
    "finalRedirectUri": "string",
    "webhook_signature": "string",
    "access_token": "string",
    "id_token": "string",
    "token_type": "string",
    "refresh_token": null,
    "expires_in": null,
    "scope": null,
    "authed_user": {
      "access_token": "string",
      "scope": "string"
    },
    "generic_api_key": "string",
    "api_key": "string",
    "bearer_token": "string",
    "basic_encoded": "string",
    "username": "string",
    "password": "string",
    "token": "string",
    "composio_link_redirect_url": "string",
    "credentials_json": "string",
    "sessionId": "string",
    "devKey": "string",
    "application_id": "string",
    "installation_id": "string",
    "private_key": "string",
    "client_id": "string",
    "client_secret": "string",
    "client_id_issued_at": 1,
    "client_secret_expires_at": 1
  }
}
```

### Responses

#### 201 - Successfully created auth link

**Response Schema:**

- `link_token` (string) *(required)*: The generated link token for the auth session
- `redirect_url` (string) *(required)*: The redirect URI to send users to for authentication
- `expires_at` (string) *(required)*: ISO timestamp when the link expires
- `connected_account_id` (string (connectedAccountId)) *(required)*: The connected account ID that was created

**Example Response:**

```json
{
  "link_token": "string",
  "redirect_url": "string",
  "expires_at": "string",
  "connected_account_id": "string"
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

#### 404 - Auth config not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 422 - Unprocessable entity

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

### Example cURL Request

```bash
curl -X POST "https://backend.composio.dev/api/v3/connected_accounts/link" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "auth_config_id": "string",
    "user_id": "string",
    "callback_url": "string",
    "connection_data": {
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
      "oauth_token": "string",
      "authUri": "string",
      "oauth_token_secret": "string",
      "redirectUrl": "string",
      "callbackUrl": "string",
      "oauth_verifier": "string",
      "consumer_key": "string",
      "callback_url": "string",
      "error": "string",
      "error_description": "string",
      "expired_at": "string",
      "state_prefix": "string",
      "long_redirect_url": true,
      "code_verifier": "string",
      "finalRedirectUri": "string",
      "webhook_signature": "string",
      "access_token": "string",
      "id_token": "string",
      "token_type": "string",
      "refresh_token": null,
      "expires_in": null,
      "scope": null,
      "authed_user": {
        "access_token": "string",
        "scope": "string"
      },
      "generic_api_key": "string",
      "api_key": "string",
      "bearer_token": "string",
      "basic_encoded": "string",
      "username": "string",
      "password": "string",
      "token": "string",
      "composio_link_redirect_url": "string",
      "credentials_json": "string",
      "sessionId": "string",
      "devKey": "string",
      "application_id": "string",
      "installation_id": "string",
      "private_key": "string",
      "client_id": "string",
      "client_secret": "string",
      "client_id_issued_at": 1,
      "client_secret_expires_at": 1
    }
  }'
```