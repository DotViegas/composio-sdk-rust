# Files (/reference/api-reference/files)

{/* Auto-generated from OpenAPI spec. Do not edit directly. */}

File management

# Endpoints

| Endpoint                            | Quick Link                                                                                                  |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `GET /api/v3/files/list`            | [List files with optional app and action filters](/reference/api-reference/files/getFilesList)              |
| `POST /api/v3/files/upload/request` | [Create presigned URL for request file upload to S3](/reference/api-reference/files/postFilesUploadRequest) |

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
# List files with optional app and action filters

**Documentation:** /reference/api-reference/files/getFilesList

Retrieves a list of files associated with the authenticated project. Results can be filtered by toolkit and tool slugs.

---

## GET `/api/v3/files/list`

**Endpoint:** `https://backend.composio.dev/api/v3/files/list`

**Summary:** List files with optional app and action filters

Retrieves a list of files associated with the authenticated project. Results can be filtered by toolkit and tool slugs.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Query Parameters

- `toolkit_slug` (string): Filter files by app slug. Example: "file-converter"
- `tool_slug` (string): Filter files by action slug. Example: "convert-to-pdf"
- `limit` (number,null): Number of items per page, max allowed is 1000
- `cursor` (string): Cursor for pagination. The cursor is a base64 encoded string of the page and limit. The page is the page number and the limit is the number of items per page. The cursor is used to paginate through the items. The cursor is not required for the first page.

### Responses

#### 200 - Successfully retrieved files

**Response Schema:**

- `items` (array<object>) *(required)*
  - Array items:
    - `toolkit_slug` (string) *(required)*: Slug of the app where this file belongs to. Example: "file-converter"
    - `tool_slug` (string) *(required)*: Slug of the action where this file belongs to. Example: "convert-to-pdf"
    - `filename` (string) *(required)*: Name of the original file. Example: "document.docx"
    - `mimetype` (string) *(required)*: Mime type of the original file. Example: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    - `md5` (string) *(required)*: MD5 hash of the file for integrity verification. Example: "d41d8cd98f00b204e9800998ecf8427e"
- `next_cursor` (string,null)
- `total_pages` (number) *(required)*
- `current_page` (number) *(required)*
- `total_items` (number) *(required)*

**Example Response:**

```json
{
  "items": [
    {
      "toolkit_slug": "string",
      "tool_slug": "string",
      "filename": "string",
      "mimetype": "string",
      "md5": "string"
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
curl -X GET "https://backend.composio.dev/api/v3/files/list" \
  -H "x-api-key: YOUR_API_KEY"
```

# Create presigned URL for request file upload to S3

**Documentation:** /reference/api-reference/files/postFilesUploadRequest

Generates a presigned URL for uploading a file to S3. This endpoint handles deduplication by checking if a file with the same MD5 hash already exists.

---

## POST `/api/v3/files/upload/request`

**Endpoint:** `https://backend.composio.dev/api/v3/files/upload/request`

**Summary:** Create presigned URL for request file upload to S3

Generates a presigned URL for uploading a file to S3. This endpoint handles deduplication by checking if a file with the same MD5 hash already exists.

### Authentication

**ApiKeyAuth** - API Key in `header` header `x-api-key` OR **UserApiKeyAuth** - API Key in `header` header `x-user-api-key`

### Request Body

**Schema:**

- `toolkit_slug` (string) *(required)*: Slug of the app where this file belongs to. Example: "gmail", "slack", "github"
- `tool_slug` (string) *(required)*: Slug of the action where this file belongs to. Example: "GMAIL_SEND_EMAIL", "SLACK_UPLOAD_FILE"
- `filename` (string) *(required)*: Name of the original file. Example: "quarterly_report.pdf"
- `mimetype` (string) *(required)*: Mime type of the original file. Example: "application/pdf", "image/png"
- `md5` (string) *(required)*: MD5 hash of the file for deduplication and integrity verification. Example: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6"

**Example:**

```json
{
  "toolkit_slug": "string",
  "tool_slug": "string",
  "filename": "string",
  "mimetype": "string",
  "md5": "string"
}
```

### Responses

#### 200 - Successfully created upload URL for request file

**Response Schema:**

- `id` (string) *(required)*: ID of the request file. Example: "req_file_9mZn4qR8sXwT"
- `key` (string) *(required)*: Object storage upload location. Example: "projects/prj_xyz789/requests/slack/SLACK_UPLOAD_FILE/document_9mZn4q.docx"
- `new_presigned_url` (string) *(required)*: Presigned URL for upload. Example: "https://storage.composio.dev/projects/prj_xyz789/requests/slack/document_9mZn4q.docx?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Expires=3600..."
- `newPresignedUrl` (string) *(required)*: [DEPRECATED] Use new_presigned_url instead. Presigned URL for upload. Example: "https://storage.composio.dev/projects/prj_xyz789/requests/slack/document_9mZn4q.docx?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Expires=3600..."
- `type` (enum: "new") *(required)*: [DEPRECATED] Indicates this is a new file that needs to be uploaded
- `metadata` (object) *(required)*
  - `storage_backend` (enum: "s3" | "azure_blob_storage") *(required)*: Storage backend used for the file. If this is azure, use `x-ms-blob-type` header to set the blob type to `BlockBlob` while uploading the file

**Example Response:**

```json
{
  "id": "string",
  "key": "string",
  "new_presigned_url": "string",
  "newPresignedUrl": "string",
  "type": "new",
  "metadata": {
    "storage_backend": "s3"
  }
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

#### 404 - Not Found

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

#### 429 - Too Many Requests

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

#### 501 - Not Implemented

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
curl -X POST "https://backend.composio.dev/api/v3/files/upload/request" \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "toolkit_slug": "string",
    "tool_slug": "string",
    "filename": "string",
    "mimetype": "string",
    "md5": "string"
  }'
```