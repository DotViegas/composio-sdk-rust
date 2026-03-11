# Webhooks (/reference/api-reference/webhooks)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Webhook configuration

# Endpoints

| Endpoint                                                | Quick Link                                                                                          |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `POST /api/v3/webhook_subscriptions`                    | [Create webhook subscription](/reference/api-reference/webhooks/postWebhookSubscriptions)           |
| `GET /api/v3/webhook_subscriptions`                     | [List webhook subscriptions](/reference/api-reference/webhooks/getWebhookSubscriptions)             |
| `GET /api/v3/webhook_subscriptions/{id}`                | [Get webhook subscription](/reference/api-reference/webhooks/getWebhookSubscriptionsById)           |
| `PATCH /api/v3/webhook_subscriptions/{id}`              | [Update webhook subscription](/reference/api-reference/webhooks/patchWebhookSubscriptionsById)      |
| `DELETE /api/v3/webhook_subscriptions/{id}`             | [Delete webhook subscription](/reference/api-reference/webhooks/deleteWebhookSubscriptionsById)     |
| `POST /api/v3/webhook_subscriptions/{id}/rotate_secret` | [Rotate webhook secret](/reference/api-reference/webhooks/postWebhookSubscriptionsByIdRotateSecret) |
| `GET /api/v3/webhook_subscriptions/event_types`         | [List available event types](/reference/api-reference/webhooks/getWebhookSubscriptionsEventTypes)   |

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

# Delete webhook subscription

**Documentation:** /reference/api-reference/webhooks/deleteWebhookSubscriptionsById

Permanently deletes a webhook subscription. This action cannot be undone.

---

## DELETE `/api/v3/webhook_subscriptions/{id}`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions/{id}`

**Summary:** Delete webhook subscription

Permanently deletes a webhook subscription. This action cannot be undone.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string (webhookSubscriptionId)) *(required)*: Webhook subscription ID

### Responses

#### 200 - Webhook subscription deleted

**Response Schema:**

- `success` (boolean) *(required)*

**Example Response:**

```json
{
  "success": true
}
```

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

#### 404 - Subscription not found

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
curl -X DELETE "https://backend.composio.dev/api/v3/webhook_subscriptions/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List webhook subscriptions

**Documentation:** /reference/api-reference/webhooks/getWebhookSubscriptions

Lists all webhook subscriptions for the authenticated project with pagination. Currently limited to one subscription per project.

---

## GET `/api/v3/webhook_subscriptions`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions`

**Summary:** List webhook subscriptions

Lists all webhook subscriptions for the authenticated project with pagination. Currently limited to one subscription per project.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Webhook subscriptions retrieved

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `id` (string (webhookSubscriptionId)) *(required)*: Unique subscription ID
    - `webhook_url` (string) *(required)*: Webhook destination URL
    - `version` (enum: "V1" | "V2" | "V3") *(required)*: Webhook payload version
    - `enabled_events` (array<string>) *(required)*: Subscribed event types
    - `secret` (string) *(required)*: Masked signing secret (full secret only on create/rotate)
    - `created_at` (string) *(required)*: ISO 8601 timestamp
    - `updated_at` (string) *(required)*: ISO 8601 timestamp
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
      "webhook_url": "string",
      "version": "V1",
      "enabled_events": [
        "..."
      ],
      "secret": "string",
      "created_at": "string",
      "updated_at": "string"
    }
  ],
  "next_cursor": null,
  "total_pages": 1,
  "current_page": 1,
  "total_items": 1
}
```

#### 400 - Invalid pagination

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
curl -X GET "https://backend.composio.dev/api/v3/webhook_subscriptions" \
  -H "x-api-key: YOUR_API_KEY"
```

# Get webhook subscription

**Documentation:** /reference/api-reference/webhooks/getWebhookSubscriptionsById

Retrieves a webhook subscription by ID.

---

## GET `/api/v3/webhook_subscriptions/{id}`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions/{id}`

**Summary:** Get webhook subscription

Retrieves a webhook subscription by ID.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string (webhookSubscriptionId)) *(required)*: Webhook subscription ID

### Responses

#### 200 - Webhook subscription retrieved

**Response Schema:**

- `id` (string (webhookSubscriptionId)) *(required)*: Unique subscription ID
- `webhook_url` (string) *(required)*: Webhook destination URL
- `version` (enum: "V1" | "V2" | "V3") *(required)*: Webhook payload version
- `enabled_events` (array<string>) *(required)*: Subscribed event types
- `secret` (string) *(required)*: Masked signing secret (full secret only on create/rotate)
- `created_at` (string) *(required)*: ISO 8601 timestamp
- `updated_at` (string) *(required)*: ISO 8601 timestamp

**Example Response:**

```json
{
  "id": "string",
  "webhook_url": "string",
  "version": "V1",
  "enabled_events": [
    "string"
  ],
  "secret": "string",
  "created_at": "string",
  "updated_at": "string"
}
```

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

#### 404 - Subscription not found

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
curl -X GET "https://backend.composio.dev/api/v3/webhook_subscriptions/string" \
  -H "x-api-key: YOUR_API_KEY"
```

# List available event types

**Documentation:** /reference/api-reference/webhooks/getWebhookSubscriptionsEventTypes

Returns all event types that can be subscribed to, along with their supported webhook versions.

---

## GET `/api/v3/webhook_subscriptions/event_types`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions/event_types`

**Summary:** List available event types

Returns all event types that can be subscribed to, along with their supported webhook versions.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Event types retrieved

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `event_type` (string) *(required)*: Event type identifier
    - `description` (string) *(required)*: Human-readable description of the event
    - `supported_versions` (array<enum: "V1" | "V2" | "V3">) *(required)*: Webhook versions that support this event

**Example Response:**

```json
{
  "items": [
    {
      "event_type": "string",
      "description": "string",
      "supported_versions": [
        "..."
      ]
    }
  ]
}
```

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
curl -X GET "https://backend.composio.dev/api/v3/webhook_subscriptions/event_types" \
  -H "x-api-key: YOUR_API_KEY"
```

# Update webhook subscription

**Documentation:** /reference/api-reference/webhooks/patchWebhookSubscriptionsById

Updates a webhook subscription. At least one field must be provided.

---

## PATCH `/api/v3/webhook_subscriptions/{id}`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions/{id}`

**Summary:** Update webhook subscription

Updates a webhook subscription. At least one field must be provided.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string (webhookSubscriptionId)) *(required)*: Webhook subscription ID

### Request Body

**Schema:**

- `webhook_url` (string (uri)): HTTPS URL to receive webhook events
- `enabled_events` (array<string>): Array of event types to subscribe to
- `version` (enum: "V1" | "V2" | "V3"): Webhook payload version

**Example:**

```json
{
  "webhook_url": "https://example.com",
  "enabled_events": [
    "string"
  ],
  "version": "V1"
}
```

### Responses

#### 200 - Webhook subscription updated

**Response Schema:**

- `id` (string (webhookSubscriptionId)) *(required)*: Unique subscription ID
- `webhook_url` (string) *(required)*: Webhook destination URL
- `version` (enum: "V1" | "V2" | "V3") *(required)*: Webhook payload version
- `enabled_events` (array<string>) *(required)*: Subscribed event types
- `secret` (string) *(required)*: Masked signing secret (full secret only on create/rotate)
- `created_at` (string) *(required)*: ISO 8601 timestamp
- `updated_at` (string) *(required)*: ISO 8601 timestamp

**Example Response:**

```json
{
  "id": "string",
  "webhook_url": "string",
  "version": "V1",
  "enabled_events": [
    "string"
  ],
  "secret": "string",
  "created_at": "string",
  "updated_at": "string"
}
```

#### 400 - Invalid request

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

#### 404 - Subscription not found

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
curl -X PATCH "https://backend.composio.dev/api/v3/webhook_subscriptions/string" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "webhook_url": "https://example.com",
    "enabled_events": [
      "string"
    ],
    "version": "V1"
  }'
```

# Create webhook subscription

**Documentation:** /reference/api-reference/webhooks/postWebhookSubscriptions

Creates a webhook subscription for the authenticated project. Only one subscription is allowed per project. The signing secret is returned only on creation.

---

## POST `/api/v3/webhook_subscriptions`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions`

**Summary:** Create webhook subscription

Creates a webhook subscription for the authenticated project. Only one subscription is allowed per project. The signing secret is returned only on creation.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `webhook_url` (string (uri)) *(required)*: HTTPS URL to receive webhook events
- `enabled_events` (array<string>) *(required)*: Array of event types to subscribe to
- `version` (enum: "V1" | "V2" | "V3"): Webhook payload version

**Example:**

```json
{
  "webhook_url": "https://example.com",
  "enabled_events": [
    "string"
  ],
  "version": "V3"
}
```

### Responses

#### 201 - Webhook subscription created

**Response Schema:**

- `id` (string (webhookSubscriptionId)) *(required)*: Unique subscription ID
- `webhook_url` (string) *(required)*: Webhook destination URL
- `version` (enum: "V1" | "V2" | "V3") *(required)*: Webhook payload version
- `enabled_events` (array<string>) *(required)*: Subscribed event types
- `secret` (string) *(required)*: Signing secret for HMAC verification
- `created_at` (string) *(required)*: ISO 8601 timestamp
- `updated_at` (string) *(required)*: ISO 8601 timestamp

**Example Response:**

```json
{
  "id": "string",
  "webhook_url": "string",
  "version": "V1",
  "enabled_events": [
    "string"
  ],
  "secret": "string",
  "created_at": "string",
  "updated_at": "string"
}
```

#### 400 - Invalid request

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

#### 409 - Subscription already exists

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
curl -X POST "https://backend.composio.dev/api/v3/webhook_subscriptions" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "webhook_url": "https://example.com",
    "enabled_events": [
      "string"
    ],
    "version": "V3"
  }'
```

# Rotate webhook secret

**Documentation:** /reference/api-reference/webhooks/postWebhookSubscriptionsByIdRotateSecret

Generates a new signing secret for the webhook subscription. The new secret is returned only in this response.

---

## POST `/api/v3/webhook_subscriptions/{id}/rotate_secret`

**Endpoint:** `https://backend.composio.dev/api/v3/webhook_subscriptions/{id}/rotate_secret`

**Summary:** Rotate webhook secret

Generates a new signing secret for the webhook subscription. The new secret is returned only in this response.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Path Parameters

- `id` (string (webhookSubscriptionId)) *(required)*: Webhook subscription ID

### Responses

#### 200 - Webhook secret rotated

**Response Schema:**

- `id` (string (webhookSubscriptionId)) *(required)*: Subscription ID
- `secret` (string) *(required)*: New signing secret

**Example Response:**

```json
{
  "id": "string",
  "secret": "string"
}
```

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

#### 404 - Subscription not found

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
curl -X POST "https://backend.composio.dev/api/v3/webhook_subscriptions/string/rotate_secret" \
  -H "x-api-key: YOUR_API_KEY"
```