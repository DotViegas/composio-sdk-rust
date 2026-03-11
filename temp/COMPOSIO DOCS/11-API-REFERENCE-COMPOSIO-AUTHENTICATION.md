# Authentication (/reference/api-reference/authentication)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

Authentication related endpoints

# Endpoints

| Endpoint                        | Quick Link                                                                                         |
| ------------------------------- | -------------------------------------------------------------------------------------------------- |
| `GET /api/v3/auth/session/info` | [Get current user session information](/reference/api-reference/authentication/getAuthSessionInfo) |

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
# Get current user session information

**Documentation:** /reference/api-reference/authentication/getAuthSessionInfo

Retrieves detailed information about the current authenticated user session, including project details, organization membership, and API key information if applicable. This endpoint is useful for verifying authentication status and retrieving contextual information about the authenticated user and their access privileges.

---

## GET `/api/v3/auth/session/info`

**Endpoint:** `https://backend.composio.dev/api/v3/auth/session/info`

**Summary:** Get current user session information

Retrieves detailed information about the current authenticated user session, including project details, organization membership, and API key information if applicable. This endpoint is useful for verifying authentication status and retrieving contextual information about the authenticated user and their access privileges.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Responses

#### 200 - Session is valid and active. Returns detailed information about the authenticated user, their project, and organization.

**Response Schema:**

- `project` (object,null) *(required)*: Details of the current active project (null if accessing with org-level credentials)
- `api_key` (object,null) *(required)*: Details of the API key used for authentication (null if using session auth)
- `org_member` (object) *(required)*: Information about the authenticated user
  - `id` (string) *(required)*: UUID identifier for the organization member
  - `email` (string) *(required)*: Email address of the authenticated user
  - `name` (string) *(required)*: Display name of the authenticated user
  - `role` (string) *(required)*: Access role of the authenticated user within the organization

**Example Response:**

```json
{
  "project": null,
  "api_key": null,
  "org_member": {
    "id": "string",
    "email": "string",
    "name": "string",
    "role": "string"
  }
}
```

#### 400 - Bad request. The request format may be invalid or missing required parameters.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 401 - Unauthorized. Authentication is required or the provided credentials are invalid or expired.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 403 - Forbidden. The authenticated user does not have permission to access this resource.

**Response Schema:**

- `error` (object) *(required)*
  - `message` (string) *(required)*
  - `code` (number) *(required)*
  - `slug` (string) *(required)*
  - `status` (number) *(required)*
  - `request_id` (string)
  - `suggested_fix` (string)
  - `errors` (array<string>)

#### 404 - Not found. The requested project, organization, or member resource could not be found.

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
curl -X GET "https://backend.composio.dev/api/v3/auth/session/info" \
  -H "x-api-key: YOUR_API_KEY"
```