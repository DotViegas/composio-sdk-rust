# Triggers (/reference/api-reference/triggers)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Trigger management and execution

# Endpoints

| Endpoint                                              | Quick Link                                                                                              |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `POST /api/v3/trigger_instances/{slug}/upsert`        | [Create or update a trigger](/reference/api-reference/triggers/postTriggerInstancesBySlugUpsert)        |
| `GET /api/v3/trigger_instances/active`                | [List active triggers](/reference/api-reference/triggers/getTriggerInstancesActive)                     |
| `DELETE /api/v3/trigger_instances/manage/{triggerId}` | [Delete a trigger](/reference/api-reference/triggers/deleteTriggerInstancesManageByTriggerId)           |
| `PATCH /api/v3/trigger_instances/manage/{triggerId}`  | [Enable or disable a trigger](/reference/api-reference/triggers/patchTriggerInstancesManageByTriggerId) |
| `GET /api/v3/triggers_types/list/enum`                | [List trigger type enums](/reference/api-reference/triggers/getTriggersTypesListEnum)                   |
| `GET /api/v3/triggers_types/{slug}`                   | [Get trigger type by slug](/reference/api-reference/triggers/getTriggersTypesBySlug)                    |
| `GET /api/v3/triggers_types`                          | [List trigger types](/reference/api-reference/triggers/getTriggersTypes)                                |

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
# Delete a trigger

**Documentation:** /reference/api-reference/triggers/deleteTriggerInstancesManageByTriggerId

Permanently deletes a trigger instance. This stops the trigger from listening for events and removes it from your project. Use the PATCH endpoint with status "disable" if you want to temporarily pause a trigger instead.

---

## DELETE `/api/v3/trigger_instances/manage/{triggerId}`

**Endpoint:** `https://backend.composio.dev/api/v3/trigger_instances/manage/{triggerId}`

**Summary:** Delete a trigger

Permanently deletes a trigger instance. This stops the trigger from listening for events and removes it from your project. Use the PATCH endpoint with status "disable" if you want to temporarily pause a trigger instead.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `triggerId` (string (triggerInstanceId)) *(required)*: The ID of the trigger instance to delete

### Responses

#### 200 - Successfully deleted the trigger instance

**Response Schema:**

- `trigger_id` (string (triggerInstanceId)) *(required)*: The ID of the deleted trigger instance

**Example Response:**

```json
{
  "trigger_id": "string"
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

#### 404 - Trigger instance not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 409 - Trigger instance already deleted

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 410 - Trigger instance already gone

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
curl -X DELETE "https://backend.composio.dev/api/v3/trigger_instances/manage/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List active triggers

**Documentation:** /reference/api-reference/triggers/getTriggerInstancesActive

Retrieves all active trigger instances for your project. Triggers listen for events from connected accounts (e.g., new emails, Slack messages, GitHub commits) and can invoke webhooks or workflows. Use filters to find triggers for specific users, connected accounts, or trigger types.

---

## GET `/api/v3/trigger_instances/active`

**Endpoint:** `https://backend.composio.dev/api/v3/trigger_instances/active`

**Summary:** List active triggers

Retrieves all active trigger instances for your project. Triggers listen for events from connected accounts (e.g., new emails, Slack messages, GitHub commits) and can invoke webhooks or workflows. Use filters to find triggers for specific users, connected accounts, or trigger types.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `user_ids` (array,null): Array of user IDs to filter triggers by
- `connected_account_ids` (array,null): Array of connected account IDs to filter triggers by
- `auth_config_ids` (array,null): Array of auth config IDs to filter triggers by
- `trigger_ids` (array,null): Array of trigger IDs to filter triggers by
- `trigger_names` (array,null): Array of trigger names to filter triggers by. Case-insensitive (internally normalized to uppercase).
- `connectedAccountIds` (array,null): DEPRECATED: This parameter will be removed in a future version. Please use connected_account_ids instead.
- `authConfigIds` (array,null): DEPRECATED: This parameter will be removed in a future version. Please use auth_config_ids instead.
- `triggerIds` (array,null): DEPRECATED: This parameter will be removed in a future version. Please use trigger_ids instead.
- `show_disabled` (boolean,null): When set to true, includes disabled triggers in the response.
- `triggerNames` (array,null): DEPRECATED: This parameter will be removed in a future version. Please use trigger_names instead.
- `showDisabled` (boolean,null): DEPRECATED: This parameter will be removed in a future version. Please use show_disabled instead.
- `deprecatedConnectedAccountUuids` (array,null): DEPRECATED: This parameter will be removed in a future version. Please use connected_account_ids instead.
- `deprecatedAuthConfigUuids` (array,null): DEPRECATED: This parameter will be removed in a future version. Please use auth_config_ids instead.
- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Successfully retrieved active trigger instances

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `uuid` (string): Unique identifier of the trigger instance
    - `id` (string (triggerInstanceId)) *(required)*: Nano ID of the trigger instance
    - `connected_account_id` (string) *(required)*: ID of the connected account this trigger is associated with
    - `trigger_name` (string) *(required)*: Name of the trigger
    - `connected_account_uuid` (string) *(required)*: UUID of the connected account this trigger is associated with
    - `user_id` (string) *(required)*: ID of the user this trigger is associated with
    - `trigger_data` (string): Additional data associated with the trigger instance
    - `trigger_config` (object) *(required)*: Configuration for the trigger
    - `state` (object) *(required)*: State of the trigger instance
    - `updated_at` (string) *(required)*: ISO 8601 timestamp when the trigger instance was updated
    - `disabled_at` (string,null) *(required)*: ISO 8601 timestamp when the trigger instance was disabled, if applicable
    - `disabledAt` (string,null) *(required)*: DEPRECATED: This parameter will be removed in a future version. Please use disabled_at instead.
    - `connectedAccountId` (string) *(required)*: DEPRECATED: This parameter will be removed in a future version. Please use connected_account_id instead.
    - `triggerName` (string) *(required)*: DEPRECATED: This parameter will be removed in a future version. Please use trigger_name instead.
    - `updatedAt` (string) *(required)*: DEPRECATED: This parameter will be removed in a future version. Please use updated_at instead.
    - `triggerConfig` (object) *(required)*: DEPRECATED: This parameter will be removed in a future version. Please use trigger_config instead.
    - `deprecated` (object): Deprecated fields for the trigger instance
      - `createdAt` (string) *(required)*: Deprecated created_at for the trigger instance
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

**Example Response:**

```json
{
  "items": [
    {
      "uuid": "string",
      "id": "string",
      "connected_account_id": "string",
      "trigger_name": "string",
      "connected_account_uuid": "string",
      "user_id": "string",
      "trigger_data": "string",
      "trigger_config": {},
      "state": {},
      "updated_at": "string",
      "disabled_at": null,
      "disabledAt": null,
      "connectedAccountId": "string",
      "triggerName": "string",
      "updatedAt": "string",
      "triggerConfig": {},
      "deprecated": {
        "createdAt": "..."
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
curl -X GET "https://backend.composio.dev/api/v3/trigger_instances/active" \
  -H "x-api-key: YOUR_API_KEY"
```

# List trigger types

**Documentation:** /reference/api-reference/triggers/getTriggersTypes

Retrieve a list of available trigger types with optional filtering by toolkit. Results are paginated and can be filtered by toolkit.

---

## GET `/api/v3/triggers_types`

**Endpoint:** `https://backend.composio.dev/api/v3/triggers_types`

**Summary:** List trigger types

Retrieve a list of available trigger types with optional filtering by toolkit. Results are paginated and can be filtered by toolkit.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `toolkit_slugs` (array,null): Array of toolkit slugs to filter triggers by
- `toolkit_versions` (any): Toolkit version specification. Use "latest" for latest versions or bracket notation for specific versions per toolkit.
- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Successfully retrieved triggers

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `slug` (string) *(required)*: Unique identifier for the trigger type
    - `name` (string) *(required)*: Human-readable name of the trigger
    - `description` (string) *(required)*: Detailed description of what the trigger does
    - `instructions` (string) *(required)*: Step-by-step instructions on how to set up and use this trigger
    - `type` (enum: "webhook" | "poll") *(required)*: The trigger mechanism - either webhook (event-based) or poll (scheduled check)
    - `toolkit` (object) *(required)*: Information about the toolkit that provides this trigger
      - `slug` (string) *(required)*: Unique identifier for the parent toolkit
      - `name` (string) *(required)*: Deprecated: Use slug instead
      - `logo` (string) *(required)*: Logo of the toolkit
    - `config` (object) *(required)*: Configuration schema required to set up this trigger
    - `payload` (object) *(required)*: Schema of the data payload this trigger will deliver when it fires
    - `version` (string) *(required)*: Version of the trigger type
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
      "instructions": "string",
      "type": "webhook",
      "toolkit": {
        "slug": "...",
        "name": "...",
        "logo": "..."
      },
      "config": {},
      "payload": {},
      "version": "string"
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
curl -X GET "https://backend.composio.dev/api/v3/triggers_types" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get trigger type by slug

**Documentation:** /reference/api-reference/triggers/getTriggersTypesBySlug

Retrieve detailed information about a specific trigger type using its slug identifier

---

## GET `/api/v3/triggers_types/{slug}`

**Endpoint:** `https://backend.composio.dev/api/v3/triggers_types/{slug}`

**Summary:** Get trigger type by slug

Retrieve detailed information about a specific trigger type using its slug identifier

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `slug` (string) *(required)*: The unique slug identifier for the trigger type. Case-insensitive (internally normalized to uppercase).

### Query Parameters

- `toolkit_versions` (any): Toolkit version specification. Use "latest" for latest versions or bracket notation for specific versions per toolkit.

### Responses

#### 200 - Successfully retrieved trigger type

**Response Schema:**

- `slug` (string) *(required)*: Unique identifier for the trigger type
- `name` (string) *(required)*: Human-readable name of the trigger
- `description` (string) *(required)*: Detailed description of what the trigger does
- `instructions` (string) *(required)*: Step-by-step instructions on how to set up and use this trigger
- `type` (enum: "webhook" | "poll") *(required)*: The trigger mechanism - either webhook (event-based) or poll (scheduled check)
- `toolkit` (object) *(required)*: Information about the toolkit that provides this trigger
  - `slug` (string) *(required)*: Unique identifier for the parent toolkit
  - `name` (string) *(required)*: Deprecated: Use slug instead
  - `logo` (string) *(required)*: Logo of the toolkit
- `config` (object) *(required)*: Configuration schema required to set up this trigger
- `payload` (object) *(required)*: Schema of the data payload this trigger will deliver when it fires
- `version` (string) *(required)*: Version of the trigger type

**Example Response:**

```json
{
  "slug": "string",
  "name": "string",
  "description": "string",
  "instructions": "string",
  "type": "webhook",
  "toolkit": {
    "slug": "string",
    "name": "string",
    "logo": "string"
  },
  "config": {},
  "payload": {},
  "version": "string"
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
curl -X GET "https://backend.composio.dev/api/v3/triggers_types/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List trigger type enums

**Documentation:** /reference/api-reference/triggers/getTriggersTypesListEnum

Retrieves a list of all available trigger type enum values that can be used across the API from latest versions of the toolkit only

---

## GET `/api/v3/triggers_types/list/enum`

**Endpoint:** `https://backend.composio.dev/api/v3/triggers_types/list/enum`

**Summary:** List trigger type enums

Retrieves a list of all available trigger type enum values that can be used across the API from latest versions of the toolkit only

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Successfully retrieved trigger enum list

**Response Schema:**


**Example Response:**

```json
[
  "string"
]
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
curl -X GET "https://backend.composio.dev/api/v3/triggers_types/list/enum" \
  -H "x-api-key: YOUR_API_KEY"
```

# Enable or disable a trigger

**Documentation:** /reference/api-reference/triggers/patchTriggerInstancesManageByTriggerId

Updates the status of a trigger instance to enable or disable it. Disabling a trigger pauses event listening without deleting the trigger configuration. Re-enabling restores the trigger to its active state. Use this for temporary maintenance or to control trigger execution.

---

## PATCH `/api/v3/trigger_instances/manage/{triggerId}`

**Endpoint:** `https://backend.composio.dev/api/v3/trigger_instances/manage/{triggerId}`

**Summary:** Enable or disable a trigger

Updates the status of a trigger instance to enable or disable it. Disabling a trigger pauses event listening without deleting the trigger configuration. Re-enabling restores the trigger to its active state. Use this for temporary maintenance or to control trigger execution.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `triggerId` (string (triggerInstanceId)) *(required)*: The ID of the trigger instance to update

### Request Body

**Schema:**

- `status` (enum: "enable" | "disable") *(required)*

**Example:**

```json
{
  "status": "enable"
}
```

### Responses

#### 200 - Successfully updated trigger status

**Response Schema:**

- `status` (enum: "success") *(required)*: Status of the operation

**Example Response:**

```json
{
  "status": "success"
}
```

#### 400 - Bad Request

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

#### 404 - Trigger instance not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 409 - Trigger instance already enabled/disabled

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 410 - Trigger instance already gone

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 500 - Internal Server Error

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
curl -X PATCH "https://backend.composio.dev/api/v3/trigger_instances/manage/string" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "enable"
  }'
```

# Create or update a trigger

**Documentation:** /reference/api-reference/triggers/postTriggerInstancesBySlugUpsert

Creates a new trigger instance or updates an existing one with the same configuration. Triggers listen for events from external services (webhooks or polling) and can invoke your workflows. If a matching trigger already exists and is disabled, it will be re-enabled. Requires a connected account ID to associate the trigger with a specific user connection.

---

## POST `/api/v3/trigger_instances/{slug}/upsert`

**Endpoint:** `https://backend.composio.dev/api/v3/trigger_instances/{slug}/upsert`

**Summary:** Create or update a trigger

Creates a new trigger instance or updates an existing one with the same configuration. Triggers listen for events from external services (webhooks or polling) and can invoke your workflows. If a matching trigger already exists and is disabled, it will be re-enabled. Requires a connected account ID to associate the trigger with a specific user connection.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `slug` (string) *(required)*: The slug of the trigger instance. Case-insensitive (internally normalized to uppercase).

### Request Body

**Schema:**

- `connectedAuthId` (string (connectedAccountId)): DEPRECATED: This parameter will be removed in a future version. Please use connected_account_id instead.
- `triggerConfig` (object): DEPRECATED: This parameter will be removed in a future version. Please use trigger_config instead.
- `connected_account_id` (string (connectedAccountId)): Connected account nanoid
- `trigger_config` (object): Trigger configuration
- `version` (string): DEPRECATED: This parameter will be removed in a future version. Please use toolkit_versions instead.
- `toolkit_versions` (any): Toolkit version specification. Supports "latest" string or a record mapping toolkit slugs to specific versions.

**Example:**

```json
{
  "connectedAuthId": "string",
  "triggerConfig": {},
  "connected_account_id": "string",
  "trigger_config": {},
  "version": "string",
  "toolkit_versions": null
}
```

### Responses

#### 200 - Successfully upserted trigger instance

**Response Schema:**

- `trigger_id` (string) *(required)*: ID of the updated trigger
- `deprecated` (object) *(required)*
  - `uuid` (string) *(required)*: ID of the updated trigger

**Example Response:**

```json
{
  "trigger_id": "string",
  "deprecated": {
    "uuid": "string"
  }
}
```

#### 201 - Successfully created trigger instance

**Response Schema:**

- `trigger_id` (string) *(required)*: ID of the updated trigger
- `deprecated` (object) *(required)*
  - `uuid` (string) *(required)*: ID of the updated trigger

**Example Response:**

```json
{
  "trigger_id": "string",
  "deprecated": {
    "uuid": "string"
  }
}
```

#### 204 - No content

**Response Schema:**


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

#### 404 - Trigger instance not found

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 410 - Gone

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
curl -X POST "https://backend.composio.dev/api/v3/trigger_instances/string/upsert" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "connectedAuthId": "string",
    "triggerConfig": {},
    "connected_account_id": "string",
    "trigger_config": {},
    "version": "string",
    "toolkit_versions": null
  }'
```