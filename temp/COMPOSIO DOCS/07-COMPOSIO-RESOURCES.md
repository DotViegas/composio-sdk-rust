# General FAQs (/docs/common-faq)

**Can I white label Composio's auth screens?**

Yes. By default, OAuth consent screens show "Composio" as the app name. If you create an auth config with your own OAuth client ID and secret, users will see your app's name and logo instead.

You can also proxy the OAuth redirect through your own domain so users never see `backend.composio.dev` in the URL bar. See [White-labeling authentication](/docs/white-labeling-authentication).

**What is the difference between a tool and a toolkit?**

A **toolkit** is a collection of related tools grouped by app, for example the GitHub toolkit or the Gmail toolkit. A **tool** is a single action within a toolkit, like `GITHUB_CREATE_ISSUE` or `GMAIL_SEND_EMAIL`.

You connect to toolkits (via OAuth or API keys), and then execute individual tools within them. See [Tools and toolkits](/docs/tools-and-toolkits).

**Do I need an AI agent to use Composio?**

No. If you're building an AI agent, Composio can give it [meta tools](/docs/quickstart) that handle tool discovery and execution automatically, or you can [fetch specific tools](/docs/tools-direct/fetching-tools) and pass them to your agent directly.

If you don't have an agent, you can [execute tools directly](/docs/tools-direct/executing-tools#direct-tool-execution) from any backend.

**Where can I see all available toolkits?**

You can browse all toolkits on the [Toolkits page](/toolkits) in the docs or in the [Composio dashboard](https://platform.composio.dev?next_page=/marketplace). Both list every supported app along with available auth schemes and tools.

**What toolkits are available by default in a session?**

All toolkits in the Composio catalog are available by default. When you create a session without specifying a `toolkits` parameter, your agent can discover and use any toolkit through `COMPOSIO_SEARCH_TOOLS`.

To restrict which toolkits are available, pass `toolkits` when creating the session:

**Python:**

```python
# Only GitHub and Gmail
session = composio.create(user_id="user_123", toolkits=["github", "gmail"])

# Everything except Linear and Jira
session = composio.create(user_id="user_123", toolkits={"disable": ["linear", "jira"]})
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
// Only GitHub and Gmail
const session = await composio.create("user_123", { toolkits: ["github", "gmail"] });

// Everything except Linear and Jira
const session2 = await composio.create("user_123", { toolkits: { disable: ["linear", "jira"] } });
```

You can also list the toolkits available in a session with `session.toolkits()`. See [Enable and disable toolkits](/docs/toolkits/enable-and-disable-toolkits).

**The tool I need doesn't exist. How do I request one?**

You can request new tools or toolkits on the [tool request board](https://request.composio.dev/boards/tool-requests). The team reviews requests regularly.

**What is User ID and how should I use it?**

The User ID is how Composio identifies your end users. It maps each user to their connected accounts and sessions. Use any unique identifier from your system, like an email, database ID, or UUID.

When you create a session with `composio.create(user_id="user_123")`, all connected accounts and tool executions are scoped to that user. See [Users and sessions](/docs/users-and-sessions).

**What is the difference between an auth config and an MCP config in the dashboard?**

An **auth config** defines how users authenticate with a toolkit (OAuth credentials, scopes, API keys). You need one whenever you want users to connect to an app.

An **MCP config** creates a hosted MCP server that exposes specific tools over the Model Context Protocol. It's only needed if you're connecting to Composio from an MCP client like Claude Desktop, Cursor, or OpenAI's Responses API via MCP.

**If you're using the Composio SDK** (sessions or direct execution), you only need auth configs. The SDK handles tool delivery natively. You don't need to create an MCP config.

**If you're using an MCP client**, you need both: an auth config (so users can authenticate) and an MCP config (so the client can discover and call tools over the MCP protocol).

See [Native Tools vs MCP](/docs/native-tools-vs-mcp) for a full comparison.

**What is an auth config?**

An auth config is a blueprint that defines how authentication works for a toolkit. It specifies the auth method (OAuth2, API Key, etc.), the scopes to request, and the credentials to use.

If you're using sessions with meta tools (`composio.create()`), you don't need to create auth configs yourself. When a user connects to a toolkit, Composio automatically uses a default auth config with managed credentials. The agent handles the entire flow through `COMPOSIO_MANAGE_CONNECTIONS`.

You only need to create auth configs manually when you're [executing tools directly](/docs/tools-direct/authenticating-tools), want to [use your own OAuth app](/docs/white-labeling-authentication), need [custom scopes](/docs/auth-configuration/custom-auth-configs), or are working with a toolkit that doesn't have Composio-managed auth. See [Authentication](/docs/authentication) for the full overview.

**What authentication types does Composio support?**

Composio supports **OAuth2**, **OAuth1**, **API Key**, **Bearer Token**, and **Basic Auth**. Most toolkits use OAuth2. Each toolkit defines which auth types it supports. You can see the available schemes when creating an auth config in the dashboard.

For toolkits with Composio-managed auth, you don't need to configure anything. For toolkits without it, you'll need to create a [custom auth config](/docs/using-custom-auth-configuration). See [Authentication](/docs/authentication) for the full overview.

**How do I work with toolkit versions?**

Composio toolkits are versioned using date-based identifiers (e.g., `20251027_00`). You can pin versions at SDK initialization or per tool execution to keep behavior consistent across deployments.

This applies to direct tool execution only. Sessions with meta tools always use the latest version. See [Toolkit versioning](/docs/tools-direct/toolkit-versioning).

**Why does the API return fewer tools than the platform UI?**

When calling the Get Tools API (`GET /api/v3/tools`) without specifying a `toolkit_versions` parameter, the API defaults to the base version (`00000000_00`), which only includes tools from the initial release. The platform UI shows tools from the latest version.

To get all available tools, pass `toolkit_versions=latest`:

```bash
curl 'https://backend.composio.dev/api/v3/tools?toolkit_slug=googledrive&toolkit_versions=latest' \
  -H 'x-api-key: YOUR_API_KEY'
```

See [Toolkit versioning](/docs/tools-direct/toolkit-versioning) for more details.

**Can a user connect multiple accounts for the same app?**

Yes. For example, a user can connect both a personal and work Gmail account. Call `session.authorize()` multiple times for the same toolkit. Each session uses the most recently connected account by default, but you can pin a specific account when creating the session.

For direct tool execution, use `initiate()` with `allow_multiple: true` and pass the `connected_account_id` when executing. See [Managing multiple connected accounts](/docs/managing-multiple-connected-accounts).

**How does token refresh work?**

Composio automatically refreshes OAuth tokens before they expire. You don't need to handle this yourself.

If a refresh fails permanently (e.g., the user revoked access or the OAuth app was deleted), the connection status changes to `EXPIRED`. You can [subscribe to expiry events](/docs/subscribing-to-connection-expiry-events) to detect this and prompt users to re-authenticate. See [Connection statuses](/docs/tools-direct/authenticating-tools#connection-statuses) for more details.

**Does Composio support triggers?**

Yes. Triggers let you listen for events from connected apps, like a new email in Gmail, a new issue in GitHub, or a new message in Slack. When the event fires, Composio sends a webhook to your endpoint with the event payload.

See [Triggers](/docs/triggers) for setup and [Subscribing to triggers](/docs/setting-up-triggers/subscribing-to-events) for receiving events.

**Why are my connected account secrets showing `abcd...` or `REDACTED`?**

Connected account secrets are masked by default for security. The API returns the first 4 characters followed by `...` (e.g., `gho_...`). Values shorter than 4 characters show as `REDACTED`.

To disable masking, go to **Settings → Project Settings → Project Configuration** and turn off "Mask Connected Account Secrets", or use the Patch Project Config API.

See [Connected accounts](/docs/auth-configuration/connected-accounts#get-account-credentials) for full details.

**Where can I find execution logs?**

Every tool execution is logged in the [Composio dashboard](https://platform.composio.dev?next_page=/logs/tools) under **Logs > Tools**. Each execution response includes a `log_id` you can use to look up detailed request and response data. Trigger delivery attempts are logged separately under **Logs > Triggers**.

See [Troubleshooting tools](/docs/troubleshooting/tools) for how to use logs to debug failed executions.

**How do I redirect users back to my app after they connect?**

Pass a `callbackUrl` when calling `link()`, `initiate()`, or `session.authorize()`. After authentication, Composio redirects the user to that URL with `status=success` or `status=failed` and the `connected_account_id` appended as query parameters.

You can include your own query parameters in the callback URL (e.g., `?user_id=user_123&source=onboarding`) to carry context through the flow. Composio preserves them. See [sessions](/docs/authenticating-users/manually-authenticating#redirecting-users-after-authentication) or [direct tool setup](/docs/tools-direct/authenticating-tools#redirecting-users-after-authentication).

**How do I use my own API key for a toolkit instead of asking each user for theirs?**

For toolkits that require API keys (like Tavily, Perplexity, or ImgBB), you can create connections on behalf of your users by passing your own API key through `connectedAccounts.initiate()`. This way, your users get access to the service without needing to provide their own credentials.

Create an auth config for the toolkit, then call `initiate()` with your API key and each user's ID:

**Python:**

```python
connection = composio.connected_accounts.initiate(
  user_id="user_123",
  auth_config_id="your_auth_config_id",
  config={
    "auth_scheme": "API_KEY", "val": {"api_key": "your_own_api_key"}

)
```

**TypeScript:**

```typescript
import { Composio, AuthScheme } from '@composio/core';

declare const composio: Composio;
const connection = await composio.connectedAccounts.initiate('user_123', 'your_auth_config_id', {
  config: AuthScheme.APIKey({
    api_key: 'your_own_api_key',
  }),
});
```

Each user gets their own connected account scoped to their `user_id`, but they all share your API key under the hood. See [API key connections](/docs/tools-direct/authenticating-tools#api-key-connections) for the full setup.

**Is Composio secure? Where can I find compliance details?**

Yes. Composio is SOC 2 Type II compliant and follows industry-standard security practices for data handling, encryption, and access control. Visit the [Composio Trust Center](https://trust.composio.dev/) for detailed compliance reports, security policies, and certifications.

**What is the IP range for Composio's MCP servers?**

Composio does not have fixed IP ranges. Traffic is routed through Cloudflare and Vercel, which use dynamic IPs.

**How do I increase my quota?**

Upgrade your plan. See [pricing](https://composio.dev/pricing) for available plans and limits.

**When should I create a new session?**

Create a new session when the config changes: different toolkits, different auth config, or a different connected account. You don't need to store or manage session IDs. Just call `create()` each time. See [Users and sessions](/docs/users-and-sessions).

**How do I configure sessions with custom auth configs?**

Some toolkits require your own OAuth credentials or API keys. Create an auth config in the dashboard by selecting the toolkit, choosing the auth scheme, and entering your credentials. Then pass the auth config ID when creating a session using the `auth_configs` (or `auth_config_id`) parameter so the session uses your developer credentials instead of Composio-managed auth. See [Using custom auth configuration](/docs/using-custom-auth-configuration).

**How do I check complete logs / API calls from the SDK?**

Enable debug logging to view all API calls and network requests made by the SDK.

Python:

```bash
COMPOSIO_LOGGING_LEVEL=debug
```

TypeScript:

```bash
COMPOSIO_LOG_LEVEL=debug
```

See [Troubleshooting SDKs](/docs/troubleshooting/sdks).

**Can Composio be self-hosted?**

Yes, Composio supports self-hosting on the Enterprise plan. See [pricing](https://composio.dev/pricing) for details or [talk to us](https://calendly.com/composiohq/enterprise).

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
# Glossary (/docs/glossary)

### Auth Config

A blueprint defining how authentication works for a toolkit: auth method (OAuth2, API key, Bearer token, Basic Auth), scopes, and credentials. Created automatically by a [session](/docs/users-and-sessions) when needed. You can [create a custom one](/docs/auth-configuration/custom-auth-configs) to use your own OAuth credentials or non-default scopes.

### Auth Scheme

The authentication method used by an auth config, such as `OAUTH2`, `API_KEY`, `BEARER_TOKEN`, or `BASIC`.

### Callback URL

The URL a user is redirected to after completing an OAuth flow through a Connect Link. Passed as `callbackUrl` when initiating authentication.

### Composio API Key

A project-scoped secret used to authenticate SDK and API requests. All resources created with it are scoped to that project.

### Composio Managed Auth

The default mode where Composio provides its own OAuth app credentials for each toolkit. No setup required.

### Connect Link

A hosted page where a user authorizes access to a toolkit. Returned as a `redirect_url` from `session.authorize()` or `connectedAccounts.initiate()`. Composio manages the full OAuth flow. See [Authentication](/docs/authentication).

### Connected Account

Created when a user authenticates with a toolkit. Stores credentials (OAuth tokens or API keys) linked to a user ID. Composio automatically refreshes OAuth tokens. A user can have [multiple connected accounts](/docs/managing-multiple-connected-accounts) for the same toolkit. IDs are prefixed `ca_`.

### Connection Request

The object returned when you initiate authentication. Contains the Connect Link URL and a `waitForConnection()` method that resolves once the user completes the flow.

### Custom Tool

A user-defined tool used alongside Composio's built-in tools. Can be standalone (no auth) or toolkit-based (authenticated API requests). Stored in memory, must be recreated on restart. See [Creating custom tools](/docs/tools-direct/custom-tools).

### In-Chat Authentication

A flow where the AI agent handles authentication by calling `COMPOSIO_MANAGE_CONNECTIONS` to generate a Connect Link and send it to the user in the conversation. See [In-chat authentication](/docs/authenticating-users/in-chat-authentication).

### MCP (Model Context Protocol)

An open protocol for connecting AI models to external tools. Every session exposes `session.mcp.url` and `session.mcp.headers`, an MCP-compatible endpoint any MCP client can connect to.

### Manual Authentication

Authenticating users from your own code using `session.authorize()` or `connectedAccounts.initiate()`, as opposed to letting the AI agent handle it via in-chat authentication. See [Manual authentication](/docs/authenticating-users/manually-authenticating).

### Meta Tools

A set of tools included in every session, including `COMPOSIO_SEARCH_TOOLS`, `COMPOSIO_MANAGE_CONNECTIONS`, `COMPOSIO_MULTI_EXECUTE_TOOL`, `COMPOSIO_REMOTE_WORKBENCH`, and `COMPOSIO_REMOTE_BASH_TOOL`. They let the agent discover tools, manage auth, execute in parallel, and run code without loading hundreds of tool definitions upfront. See [Tools and toolkits](/docs/tools-and-toolkits#meta-tools).

### Modifiers

Middleware that transforms tool behavior: [schema modifiers](/docs/tools-direct/modify-tool-behavior/schema-modifiers) change a tool's schema before the agent sees it, [before-execution modifiers](/docs/tools-direct/modify-tool-behavior/before-execution-modifiers) modify arguments before a tool runs, [after-execution modifiers](/docs/tools-direct/modify-tool-behavior/after-execution-modifiers) transform the result.

### Native Tools

Tools accessed through provider packages via `session.tools()`, as opposed to connecting via MCP (`session.mcp.url`). Both methods give the agent the same capabilities, but native tools integrate directly with your AI framework.

### Organization

The top-level Composio account entity. Contains team members and projects.

### Organization API Key

A key (`x-org-api-key`) for organization-level operations like creating and managing projects. Distinct from the project-scoped Composio API Key.

### Project

An isolated environment within an organization that scopes API keys, connected accounts, auth configs, and webhooks. Resources in one project are inaccessible from another. IDs are prefixed `proj_`. See [Projects](/docs/projects).

### Proxy Execute

Making authenticated HTTP requests through a toolkit's connected account without a predefined tool. Useful for API endpoints Composio doesn't have a built-in tool for.

### Provider

An adapter package that transforms Composio tools into the format expected by an AI framework (OpenAI, Anthropic, LangChain, Vercel AI SDK, etc.). See [Providers](/docs/providers).

### Session

An ephemeral configuration object from `composio.create(userId)`. Ties together a user ID, available toolkits, auth config, and connected accounts. Immutable. Exposes `tools()`, `mcp.url`, `authorize()`, and `toolkits()`. See [Users and sessions](/docs/users-and-sessions).

### Session ID

Unique identifier for a session. Used internally by meta tools to share context across calls within the same session.

### Tool

An individual action an agent can execute. Has an input schema and output schema. Named `{TOOLKIT}_{ACTION}` (e.g., `GITHUB_CREATE_ISSUE`).

### Tool Slug

A tool's unique identifier, following the `{TOOLKIT}_{ACTION}` pattern, e.g. `GITHUB_CREATE_ISSUE`.

### Toolkit

A collection of related tools for a single external service. Users connect to a toolkit via authentication, and all its tools execute with the user's credentials.

### Toolkit Slug

The lowercase identifier for a toolkit, e.g. `github`, `gmail`, `slack`. Used when configuring sessions, fetching tools, or creating triggers.

### Toolkit Versioning

Pinning a toolkit to a specific version so your integration uses a consistent set of tools even as Composio updates definitions. See [Toolkit versioning](/docs/tools-direct/toolkit-versioning).

### Trigger

Sends structured payloads to your application when something happens in a connected app. Two delivery types: **webhook** (the app pushes events in real time, e.g. GitHub, Slack) and **polling** (Composio periodically checks for new data, e.g. Gmail). See [Triggers](/docs/triggers).

### Trigger Instance

A specific, active trigger scoped to a user's connected account.

### User ID

An identifier from your application that Composio uses to scope connected accounts, tool executions, and authorizations. Connections are fully isolated between user IDs. See [Users and sessions](/docs/users-and-sessions).

### White-Labeling

Customizing the auth experience so users see your brand during the OAuth flow. You provide your own OAuth credentials, redirect URIs, and branding. See [White-labeling authentication](/docs/white-labeling-authentication).

### Workbench

A persistent Python sandbox via the `COMPOSIO_REMOTE_WORKBENCH` meta tool. State persists across calls within a session. Used for bulk operations, data transformations, and processing large tool responses. See [Workbench](/docs/workbench).

---

📚 **More documentation:** [View all docs](https://docs.composio.dev/llms.txt) | [Glossary](https://docs.composio.dev/llms.mdx/docs/glossary) | [Cookbooks](https://docs.composio.dev/llms.mdx/cookbooks) | [API Reference](https://docs.composio.dev/llms.mdx/reference)

# Debugging Info (/docs/debugging-info)

Your debugging info is tied to your [project](/docs/projects) and it helps us trace what happened and debug the issue faster. Before reaching out, check the [troubleshooting guide](/docs/troubleshooting) and [common FAQ](/docs/common-faq) for solutions to frequent issues.

# Finding Your Debugging Info

Navigate to your project settings to find your debugging information:

![Project Settings Debugging Info](/images/project-settings.jpeg)

![Debugging Info Location](/images/debugging-info.png)

# What to Share

When reaching out for support, share these identifiers:

```
@project_id: pr_xxxxxxxxxxxxx
@org_id: ok_xxxxxxxxxxxxx
@org_member_email: your-email@example.com
```

The identifiers with `pr_` (project) and `ok_` (organization) prefixes let us quickly check your logs inside our internal tracing tools, helping us resolve issues faster.

# Getting Help

import { MessageCircle, Bug, Mail, Grip } from 'lucide-react';

- [Join Discord](https://discord.gg/composio): }>
Get basic support from the community and Composio team

- [Request Tools](https://request.composio.dev): }>
Request new tools and toolkits

- [File an Issue](https://github.com/ComposioHQ/composio/issues): }>
Report SDK issues and bugs

- [Contact Support](mailto:support@composio.dev): }>
Reach out for dedicated support channels on Growth and Enterprise plans

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
# Migrating from Direct Tools to Sessions (/docs/migration-guide/direct-to-sessions)

This guide is for developers who are using Composio's **direct tool execution** pattern and want to migrate to **sessions**. For a comparison of the two patterns, see [Sessions vs Direct Execution](/docs/sessions-vs-direct-execution).

With sessions, you don't need to manage tool fetching, authentication, or execution yourself. You create a session and it handles everything. Your existing auth configs and connected accounts carry over, so your users don't need to re-authenticate.

If you're starting fresh, skip this guide and head to the [Quickstart](/docs/quickstart).

# What changes

|                      | Direct tools                                      | Sessions                                                               |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------- |
| **Tool discovery**   | You fetch specific tools by name/toolkit          | Agent discovers tools at runtime via meta tools                        |
| **Authentication**   | You manage connect links and wait for connections | In-chat auth prompts users automatically, or use `session.authorize()` |
| **Execution**        | You call `tools.execute()` with version pinning   | Agent executes tools through `COMPOSIO_MULTI_EXECUTE_TOOL`             |
| **Toolkit versions** | You manage versions manually                      | Handled automatically                                                  |

# Migrating

#### Pass your existing auth configs to the session

You already have auth configs (e.g. `ac_github_config`, `ac_slack_config`). Pass them when creating a session so it uses your existing OAuth credentials and your users' connected accounts carry over.

**Python:**

```python
from composio import Composio

composio = Composio()

# Before: manual tool fetching with user_id
# tools = composio.tools.get(user_id="user_123", toolkits=["GITHUB", "SLACK"])

# After: create a session with your existing auth configs
session = composio.create(
    user_id="user_123",
    auth_configs={
        "github": "ac_your_github_config",
        "slack": "ac_your_slack_config"

)
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
// Before: manual tool fetching with userId
// const tools = await composio.tools.get("user_123", { toolkits: ["GITHUB", "SLACK"] });

// After: create a session with your existing auth configs
const session = await composio.create("user_123", {
  authConfigs: {
    github: "ac_your_github_config",
    slack: "ac_your_slack_config",
  },
});
```

Since the session uses the same `user_id` and auth configs, it automatically picks up existing connected accounts. No re-authentication needed.

#### Replace manual tool fetching with session tools

Instead of fetching specific tools by name, get the session's meta tools. These let the agent discover, authenticate, and execute any tool at runtime.

**Python:**

```python
# Before: manually specifying which tools to fetch
# tools = composio.tools.get(user_id="user_123", toolkits=["GITHUB"])

# After: session provides meta tools that handle discovery
tools = session.tools()
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
const session = await composio.create("user_123");
// Before: manually specifying which tools to fetch
// const tools = await composio.tools.get("user_123", { toolkits: ["GITHUB"] });

// After: session provides meta tools that handle discovery
const tools = await session.tools();
```

#### Remove manual auth flows

If you were manually creating connect links and waiting for connections, sessions handle this automatically. When a tool requires authentication, the agent prompts the user with a connect link in-chat.

**Python:**

```python
# Before: manual auth flow
# connection_request = composio.connected_accounts.link(
#     user_id="user_123",
#     auth_config_id="ac_your_github_config",
#     callback_url="https://your-app.com/callback"
# )
# redirect_url = connection_request.redirect_url

# After: auth happens automatically in-chat
# Or if you need to pre-authenticate outside of chat:
connection_request = session.authorize("github")
print(connection_request.redirect_url)
connected_account = connection_request.wait_for_connection()
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
const session = await composio.create("user_123");
// Before: manual auth flow
// const connectionRequest = await composio.connectedAccounts.link("user_123", "ac_your_github_config", {
//   callbackUrl: "https://your-app.com/callback"
// });
// const redirectUrl = connectionRequest.redirectUrl;

// After: auth happens automatically in-chat
// Or if you need to pre-authenticate outside of chat:
const connectionRequest = await session.authorize("github", {
  callbackUrl: "https://your-app.com/callback",
});
console.log(connectionRequest.redirectUrl);
const connectedAccount = await connectionRequest.waitForConnection();
```

#### Remove toolkit version management

If you were pinning toolkit versions in your code or environment variables, you can remove that. Sessions handle versioning automatically.

**Python:**

```python
# Before: manual version pinning
# composio = Composio(
#     toolkit_versions={
#         "github": "20251027_00",
#         "slack": "20251027_00",
#     }
# )

# After: no version management needed
composio = Composio()
session = composio.create(user_id="user_123")
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
// Before: manual version pinning
// const composio = new Composio({
//   toolkitVersions: {
//     github: "20251027_00",
//     slack: "20251027_00",
//   },
// });

// After: no version management needed
const composio = new Composio({ apiKey: 'your_api_key' });
const session = await composio.create("user_123");
```

# Restricting toolkits

If you were fetching tools from specific toolkits, you can restrict the session to only those toolkits:

**Python:**

```python
session = composio.create(
    user_id="user_123",
    toolkits=["github", "gmail", "slack"],
    auth_configs={
        "github": "ac_your_github_config",
        "gmail": "ac_your_gmail_config",
        "slack": "ac_your_slack_config"

)
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
const session = await composio.create("user_123", {
  toolkits: ["github", "gmail", "slack"],
  authConfigs: {
    github: "ac_your_github_config",
    gmail: "ac_your_gmail_config",
    slack: "ac_your_slack_config",
  },
});
```

# Multiple connected accounts

If your users have multiple accounts for the same toolkit (e.g., work and personal Gmail), you can specify which one to use per session:

**Python:**

```python
session = composio.create(
    user_id="user_123",
    auth_configs={
        "gmail": "ac_your_gmail_config"
    },
    connected_accounts={
        "gmail": "ca_work_gmail"

)
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
const session = await composio.create("user_123", {
  authConfigs: {
    gmail: "ac_your_gmail_config",
  },
  connectedAccounts: {
    gmail: "ca_work_gmail",
  },
});
```

If you don't specify, the most recently connected account is used. See [Managing multiple connected accounts](/docs/managing-multiple-connected-accounts) for details.

# Triggers

Triggers don't work with sessions yet. Continue using them the same way you do today with `composio.triggers.create()`, `composio.triggers.enable()`, and webhook subscriptions.

See [Triggers](/docs/triggers) for setup instructions.

# White labeling

If you've set up white-labeled OAuth screens with custom auth configs, those carry over automatically. Just pass the same auth config IDs to your session via `auth_configs` / `authConfigs`. Your users will continue to see your branding on consent screens.

See [White-labeling authentication](/docs/white-labeling-authentication) for more.

# Get started

- [Quickstart](/docs/quickstart): Build your first agent with sessions

- [Configuring Sessions](/docs/configuring-sessions): Toolkits, auth configs, account selection, and session methods

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
# Migrating from Experimental Tool Router (/docs/migration-guide/tool-router-beta)

This guide is for users who adopted the **experimental tool router** (`composio.experimental.tool_router`) during its beta period. The tool router has now graduated to a stable, first-class feature called **sessions** — with a simpler API, better auth handling, and full framework support.

If you never used `composio.experimental.tool_router`, you can skip this guide and head straight to the [Quickstart](/docs/quickstart).

# The basics

#### Upgrade composio package

Upgrade to the latest stable version:

**Python:**

```bash
pip install --upgrade composio
```

**TypeScript:**

```bash
npm install @composio/core@latest
```

#### Update session creation

**Python:**

```python
# Beta (before)
session = composio.experimental.tool_router.create_session(
    user_id="user@example.com"
)

# Stable (after)
session = composio.create(
    user_id="user@example.com"
)
```

**TypeScript:**

```typescript
// Beta (before)
const session = await composio.experimental.toolRouter.createSession('user_123');

// Stable (after)
const session = await composio.create('user_123');
```

#### Moving users (optional)

If you have existing users on tool router and you don't want them to authenticate again:

    * Tool Router will auto-detect auth configs and connected accounts it created (from the beta version).
    * If you have custom auth configs (not created by Tool Router):
      * Search for the Auth config for that connected account. See [Connected Accounts](/docs/auth-configuration/connected-accounts) to fetch existing accounts programmatically.
      * While creating a session configure to use this Auth config.
      * You need to repeat this for each toolkit you want to enable for that session.

**Python:**

```python
session = composio.create(
    user_id="user_123",
    auth_configs={
        "github": "ac_your_github_config",
        "slack": "ac_your_slack_config"

)
```

**TypeScript:**

```typescript
import { Composio } from '@composio/core';
const composio = new Composio({ apiKey: 'your_api_key' });
const session = await composio.create('user_123',

        authConfigs: {
            github: "ac_your_github_config",
            slack: "ac_your_slack_config",
        },

);
```

# Get started

- [Quickstart](/docs/quickstart): Build your first agent with the stable sessions API

- [Configuring Sessions](/docs/configuring-sessions): Restrict toolkits, set auth configs, and select connected accounts

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
# Toolkit versioning migration (/docs/migration-guide/toolkit-versioning)

Starting with Python SDK v0.9.0 and TypeScript SDK v0.2.0, manual tool execution requires explicit version specification. This is a breaking change from earlier versions where toolkit versioning was optional.

# Breaking change

Manual tool execution now requires explicit version specification. The `tools.execute()` method will fail without a version.

## Before (will fail)

**Python:**

```python
# Raises ToolVersionRequiredError
result = composio.tools.execute(
    "GITHUB_CREATE_ISSUE",
    {"user_id": "user-123", "arguments": {...}}
)
```

**TypeScript:**

```typescript
// Throws ComposioToolVersionRequiredError
const result = await composio.tools.execute({
    toolSlug: "GITHUB_CREATE_ISSUE",
    userId: "user-123",
    arguments: {...}
});
```

## After (required)

Choose one of three migration strategies:

### Option 1: Configure version at SDK level

**Python:**

```python
from composio import Composio

# Pin specific versions for each toolkit
composio = Composio(
    api_key="YOUR_API_KEY",
    toolkit_versions={
        "github": "20251027_00",
        "slack": "20251027_00",
        "gmail": "20251027_00"

)
```

**TypeScript:**

```typescript
import { Composio } from "@composio/core";

// Pin specific versions for each toolkit
const composio = new Composio({
    apiKey: "YOUR_API_KEY",
    toolkitVersions: {
        github: "20251027_00",
        slack: "20251027_00",
        gmail: "20251027_00"

});
```

### Option 2: Pass version with each execution

**Python:**

```python
# Specify version directly in execute call
result = composio.tools.execute(
    "GITHUB_LIST_STARGAZERS",
    arguments={
        "owner": "ComposioHQ",
        "repo": "composio"
    },
    user_id="user-k7334",
    version="20251027_00"  # Override version for this execution
)
print(result)
```

**TypeScript:**

```typescript
// Specify version directly in execute call
const result = await composio.tools.execute("GITHUB_LIST_STARGAZERS", {
    userId: "user-k7334",
    arguments: {
        owner: "ComposioHQ",
        repo: "composio"
    },
    version: "20251027_00"  // Override version for this execution
});
console.log(result);
```

### Option 3: Use environment variables

```bash
export COMPOSIO_TOOLKIT_VERSION_GITHUB="20251027_00"
```

# Migration checklist

1. **Audit your code**: Find all `tools.execute()` calls in your codebase
2. **Choose a strategy**: Select one of the three options above based on your needs
3. **Test thoroughly**: Verify tools work correctly with pinned versions
4. **Deploy gradually**: Roll out changes incrementally to minimize risk

# Temporary workaround

During migration, you can temporarily skip version checks (not recommended for production):

**Python:**

```python
result = composio.tools.execute(
    "GITHUB_CREATE_ISSUE",

        "user_id": "user-123",
        "arguments": {...}
    },
    dangerously_skip_version_check=True
)
```

**TypeScript:**

```typescript
const result = await composio.tools.execute({
    toolSlug: "GITHUB_CREATE_ISSUE",
    userId: "user-123",
    arguments: {...},
    dangerouslySkipVersionCheck: true
});
```

> The `dangerouslySkipVersionCheck` flag is only for migration or debugging. Never use in production.

# What to read next

- [Toolkit versioning](/docs/tools-direct/toolkit-versioning): Complete guide to version formats, resolution order, and managing versions

- [Migrate to sessions](/docs/migration-guide/direct-to-sessions): Move from direct tool execution to the sessions pattern

- [Quickstart](/docs/quickstart): Build your first agent with the sessions API

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
# Our next generation SDKs (/docs/migration-guide/new-sdk)

> This guide covers migrating from the legacy SDK (v1) to the current SDK (v3). The recommended way to use Composio is now through **sessions** — see [Migrating from Direct Tools to Sessions](/docs/migration-guide/direct-to-sessions) or the [Quickstart](/docs/quickstart) if you're starting fresh.

In the last few months, we have experienced very rapid growth in usage of our platform. As such, our team has been working hard to radically improve the performance and developer experience of our platform.

A lot of these changes have happened in the background, but we are excited to finally share our new SDKs with you that complement our new infra.

The new API features improved usability, enhanced stability, and better scalability. The SDKs built on top of it simplify the developer experience, making it easier than ever to build useful agents.

# What's new?

A lot of the changes are on the infra side, but from the SDK point of view, here is what you can expect:

* Faster and more reliable tool execution
* A simpler but more opinionated SDK
* Much more intuitive and consistent naming conventions
* A vastly improved TypeScript SDK that is meaningfully more type-safe and has full feature parity with the Python SDK

There aren't too many new flashy features here (yet) mainly because we wanted to get the bones right — but we feel we have a solid foundation to ship incredible new experiences on top very quickly.

# State of the new SDK and what is happening with the old SDKs?

Currently, the new SDKs are in a preview release. These new SDKs come almost fully formed, we do not expect many breaking changes to them but are releasing them in a preview state to get feedback and make necessary changes before locking them in.

As we lock the new SDKs in place, we will deprecate support for the old SDKs. They will continue to work for the foreseeable future but are no longer actively maintained. We will continue to push security updates and fix any critical bugs but will not support any new functionality in them.

We urge you to upgrade to the new SDKs as soon as possible.

# Nomenclature

We have updated several key terms in the SDK and API to improve clarity and consistency. The following table summarizes these changes:

| Previous Term | Current Term       | Definition                                                                                                                           |
| ------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| Actions       | Tools              | Individual operations or capabilities that can be performed by an LLM agent                                                          |
| Apps          | Toolkits           | A collection of tools grouped under a single application                                                                             |
| Integration   | Auth Config        | Configuration containing developer credentials and application-level settings such as scopes and API endpoints. Scoped to a toolkit. |
| Connection    | Connected accounts | User-linked accounts associated with a toolkit                                                                                       |
| Entity ID     | User ID            | The identifier of the user performing the action (UUID or email)                                                                     |
| Trigger       | Trigger            | An event that can be subscribed to                                                                                                   |
| Toolsets      | Providers          | LLM or agent framework that can be used with Composio to create agents                                                               |

# Switch to nano IDs from UUIDs

We have transitioned from UUIDs to nano IDs throughout the platform for the following reasons:

* **Improved readability**: UUIDs are lengthy and difficult to read
* **Better usability**: Easier to copy with a single double-click
* **Better organization**: Nano IDs allow us to distinguish between different resource types through prefixes

| Feature           | Nano ID Prefix | Example           |
| ----------------- | -------------- | ----------------- |
| Connected Account | `ca_`          | `ca_8x9w2l3k5m`   |
| Auth Config       | `ac_`          | `ac_1234567890`   |
| Trigger           | `ti_`          | `ti_So9EQf8XnAcy` |

> Nano IDs are short, unique, and prefixed to indicate the resource type.

# SDK Changes

Upgrade to the latest SDK version using the appropriate package manager:

**Python:**

```bash
pip install -U composio
```

**TypeScript:**

```bash
npm install @composio/core
```

Both SDKs now implement proper namespacing for each concept.

## User ID scoping

The concept of `entity_id` has been expanded and renamed to `user_id`.

All operations are now scoped to a user ID, including:

* Fetching tools
* Initiating connections
* Executing tools
* Managing triggers

This change provides explicit specification of the user for whom the action is being performed. When a user may have multiple accounts (such as work and personal Gmail connections), you can use the more specific connected account ID.

## Replacing ToolSets with Providers

We have deprecated "toolsets" in favor of "providers". This change allows Composio to provide deeper standardization for tool implementation across different frameworks.

Previously, you needed to import and use a framework-specific `ComposioToolSet` class:

**Python (previous):**

```python
from composio_openai import ComposioToolSet, Action, App
from openai import OpenAI

toolset = ComposioToolSet()
```

**TypeScript (previous):**

```typescript
import { OpenAIToolSet } from 'composio-core';

const toolset = new OpenAIToolSet();
```

The SDK structure is now framework-agnostic and includes the OpenAI provider out of the box:

**Python (current):**

```python
from composio import Composio
# from composio_langchain import LangchainProvider

composio = Composio()
# composio = Composio(provider=LangchainProvider())

tools = composio.tools.get(
    user_id="0001",
    tools=["LINEAR_CREATE_LINEAR_ISSUE", "GITHUB_CREATE_COMMIT"]
)
# tools returned is formatted for the provider. by default, OpenAI.
```

**TypeScript (current):**

```typescript
import { Composio } from '@composio/core';
// import { VercelProvider } from '@composio/vercel';

const composio = new Composio({
  // provider: new VercelProvider(),
});
// Can specify other providers too, like OpenAI, Anthropic, Vercel AI SDK.

const tools = await composio.tools.get('user@example.com', {
  tools: ['LINEAR_CREATE_LINEAR_ISSUE', 'GITHUB_CREATE_COMMIT'],
});
// tools returned is formatted for the provider. by default, OpenAI.
```

You can now use the same tools across any framework with our unified interface, or create custom toolsets for frameworks we don't yet support.

Read more about [providers in our documentation](/docs/providers/openai) and explore the [complete list of available providers](/docs/providers/openai).

## Fetching and filtering tools

Previously, you could filter tools by:

* Apps
* Action names (tool names)
* Tags

You could also specify an `important` flag to retrieve the most important tools:

**Python (previous):**

```python
from composio_openai import ComposioToolSet, Action, App
from openai import OpenAI

toolset = ComposioToolSet()
client = OpenAI()

tools = toolset.get_tools(
    actions=[Action.GITHUB_GET_THE_AUTHENTICATED_USER], check_connected_accounts=True
)

tools = toolset.get_tools(apps=[App.GITHUB, App.LINEAR, App.SLACK], check_connected_accounts=True)
```

**TypeScript (previous):**

```typescript
import { OpenAIToolSet } from 'composio-core';

const toolset = new OpenAIToolSet();

const tools_1 = await toolset.getTools({ apps: ['GITHUB'] });
const tools_2 = await toolset.getTools({
  actions: ['GITHUB_GET_THE_AUTHENTICATED_USER', 'LINEAR_CREATE_LINEAR_ISSUE'],
});
```

You can now filter tools by:

* Toolkits
* Tool slugs
* Limit parameter
* Search query

The `important` flag has been removed. Instead, tools are returned in order of importance by default:

> Since `user_id` is now explicitly required, the `check_connected_accounts` flag is no longer necessary.

**Python (current):**

```python
from composio import Composio

composio = Composio()

user_id = "user@acme.org"

tools_1 = composio.tools.get(user_id=user_id, toolkits=["GITHUB", "LINEAR"])

tools_2 = composio.tools.get(user_id=user_id, toolkits=["SLACK"], limit=5)  # Default limit=20

tools_3 = composio.tools.get(
    user_id=user_id,
    tools=["GITHUB_CREATE_AN_ISSUE", "GITHUB_CREATE_AN_ISSUE_COMMENT", "GITHUB_CREATE_A_COMMIT"],
)

tools_4 = composio.tools.get(user_id="john", search="hackernews posts")
```

**TypeScript (current):**

```typescript
import { Composio } from '@composio/core';

const userId = 'user@acme.org';

const composio = new Composio();

const tools_1 = await composio.tools.get(userId, {
  toolkits: ['GITHUB', 'LINEAR'],
});

const tools_2 = await composio.tools.get(userId, {
  toolkits: ['GITHUB'],
  limit: 5, // Default limit=20
});

const tools_3 = await composio.tools.get(userId, {
  tools: ['GITHUB_CREATE_AN_ISSUE', 'GITHUB_CREATE_AN_ISSUE_COMMENT', 'GITHUB_CREATE_A_COMMIT'],
});

const tools_4 = await composio.tools.get(userId, {
  search: 'hackernews posts',
});
```

## Fetching raw tool data

To examine the raw schema definition of a tool for understanding input/output parameters or building custom logic around tool definitions, use the following methods:

**Python (current):**

```python
from composio import Composio

composio = Composio()

tool = composio.tools.get_raw_composio_tool_by_slug("HACKERNEWS_GET_LATEST_POSTS")

print(tool.model_dump_json())
```

**TypeScript (current):**

```typescript
import { Composio } from '@composio/core';

const composio = new Composio();

const tool = await composio.tools.getRawComposioToolBySlug('GITHUB_GET_OCTOCAT');

console.log(JSON.stringify(tool, null, 2));
```

## Executing tools

Tool execution remains largely unchanged, with `user_id` now explicitly required.

For agentic frameworks, the tool object returned from `tools.get` is now the respective framework's native tool object. Tool call execution is handled by the agentic framework itself.

> For non-agentic frameworks, Composio provides a helper function to execute tool calls.

**Python v3:**

```python
from composio import Composio
from openai import OpenAI

openai_client = OpenAI()
composio = Composio()

tools = composio.tools.get(user_id="user@acme.com", tools=["GITHUB_GET_THE_ZEN_OF_GITHUB"])
response = openai_client.chat.completions.create(
    model="gpt-4.1",
    messages=[{"role": "user", "content": "gimme some zen."}],
    tools=tools,
)

result = composio.provider.handle_tool_calls(user_id="user@acme.com", response=response)
print(result)
```

**TypeScript v3:**

```typescript
import { Composio } from '@composio/core';
import { AnthropicProvider } from '@composio/anthropic';
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();
const composio = new Composio({
  provider: new AnthropicProvider(),
});

const userId = 'user@example.com';
const tools = await composio.tools.get(userId, {
  toolkits: ['GMAIL'],
});

const msg = await anthropic.messages.create({
  model: 'claude-3-7-sonnet-latest',
  tools: tools,
  messages: [

      role: 'user',
      content: "Say hi to 'soham@composio.dev'",
    },
  ],
  max_tokens: 1024,
});

const result = await composio.provider.handleToolCalls(userId, msg);
console.log('Tool results:', result);
```

For more information on executing tools for different frameworks, see [Replacing ToolSets with Providers](#replacing-toolsets-with-providers).

## Tool Modifiers (formerly Tool Processors)

Tool processors have been renamed to *tool modifiers* and now provide an improved developer experience. The implementation is now available in TypeScript too! (previously Python-only).

```python title="Python (previous)"
from composio_openai import ComposioToolSet, Action

toolset = ComposioToolSet()

def my_schema_processor(schema: dict) -> dict: ...
def my_preprocessor(inputs: dict) -> dict: ...
def my_postprocessor(result: dict) -> dict: ...

# Get tools with the modified schema
processed_tools = toolset.get_tools(
    actions=[Action.GMAIL_SEND_EMAIL],
    processors={
        # Applied BEFORE the LLM sees the schema
        "schema": {Action.SOME_ACTION: my_schema_processor},
        # Applied BEFORE the tool executes
        "pre": {Action.SOME_ACTION: my_preprocessor},
        # Applied AFTER the tool executes, BEFORE the result is returned
        "post": {Action.SOME_ACTION: my_postprocessor},
    },
)
```
| Previous           | Current                  |
| ------------------ | ------------------------ |
| `pre` processor    | `beforeExecute` modifier |
| `post` processor   | `afterExecute` modifier  |
| `schema` processor | `schema` modifier        |

The modifiers now leverage language-specific features to provide a more natural developer experience.

While tool processors could previously be applied during SDK initialization, tool fetching, and tool execution, we have restructured them as follows:

* **Chat Completion providers**: Modifiers are specified and applied during tool execution
* **Agentic frameworks**: Modifiers are specified and applied during tool fetching

### Schema Modifiers

The following example demonstrates schema modifier usage, applicable across all providers:

**Python (current):**

```python
from composio import Composio, schema_modifier
from composio.types import Tool

user_id = "your@email.com"

@schema_modifier(tools=["HACKERNEWS_GET_LATEST_POSTS"])
def modify_schema(
    tool: str,
    toolkit: str,
    schema: Tool,
) -> Tool:
    _ = schema.input_parameters["properties"].pop("page", None)
    schema.input_parameters["required"] = ["size"]
    return schema

tools = composio.tools.get(
    user_id=user_id,
    tools=["HACKERNEWS_GET_LATEST_POSTS", "HACKERNEWS_GET_USER"],
    modifiers=[
        modify_schema,
    ]
)
```
**TypeScript (current):**

```typescript
// @noErrors
import { Composio } from '@composio/core';
import { OpenAI } from 'openai';

const userId = 'your@email.com';
const composio = new Composio();

// Schema modifier to delete the `page` argument from the `HACKERNEWS_GET_LATEST_POSTS` tool
const tools = await composio.tools.get(
  userId,

    tools: ['HACKERNEWS_GET_LATEST_POSTS', 'HACKERNEWS_GET_USER'],
  },

    modifySchema: ({ toolSlug, toolkitSlug, schema }) => {
      if (toolSlug === 'HACKERNEWS_GET_LATEST_POSTS') {
        const { inputParameters } = schema;
        if (inputParameters?.properties) {
          delete inputParameters.properties['page'];

        inputParameters.required = ['size'];

      return schema;
    },

);

console.log(JSON.stringify(tools, null, 2));
```
### Before Modifiers

The following example shows creating and using a before modifier for a Chat Completion provider. For agentic frameworks, view the [complete before modifier documentation](/docs/tools-direct/modify-tool-behavior/before-execution-modifiers):

**Python (current):**

```python
@before_execute(tools=["HACKERNEWS_GET_LATEST_POSTS"])
def before_execute_modifier(
    tool: str,
    toolkit: str,
    params: ToolExecuteParams,
) -> ToolExecuteParams:
    params["arguments"]["size"] = 1
    return params

# Get tools
tools = composio.tools.get(user_id=user_id, slug="HACKERNEWS_GET_LATEST_POSTS")
```
**TypeScript (current):**

```typescript
// @noErrors
const result_1 = await composio.tools.execute(
  'HACKERNEWS_GET_LATEST_POSTS',

    userId,
    arguments: JSON.parse(toolArgs),
  },

    beforeExecute: ({ toolSlug, toolkitSlug, params }) => {
      if (toolSlug === 'HACKERNEWS_GET_LATEST_POSTS') {
        params.arguments.size = 1;

      console.log(params);
      return params;
    },

);
```
### After Modifiers

The following example shows creating and using an after modifier for a Chat Completion provider. For agentic frameworks, view the [complete after modifier documentation](/docs/tools-direct/modify-tool-behavior/after-execution-modifiers):

**Python (current):**

```python
@after_execute(tools=["HACKERNEWS_GET_USER"])
def after_execute_modifier(
    tool: str,
    toolkit: str,
    response: ToolExecutionResponse,
) -> ToolExecutionResponse:
    return {
        **response,
        "data": {
            "karma": response["data"]["karma"],
        },

tools = composio.tools.get(user_id=user_id, slug="HACKERNEWS_GET_USER")
```
**TypeScript (current):**

```typescript
// @noErrors
const result_2 = await composio.tools.execute(
  'HACKERNEWS_GET_USER',

    userId,
    arguments: JSON.parse(toolArgs),
  },

    afterExecute: ({ toolSlug, toolkitSlug, result }) => {
      if (toolSlug === 'HACKERNEWS_GET_USER') {
        const { data } = result;
        const { karma } = data.response_data as { karma: number };
        return {
          ...result,
          data: { karma },
        };

      return result;
    },

);
```
## Custom Tools

The SDK continues to support custom tools. [Creating tools from your methods](/docs/tools-direct/custom-tools#creating-a-custom-tool) remains possible. We recommend reviewing the [detailed custom tools documentation](/docs/tools-direct/custom-tools#creating-a-custom-tool) for more information.

Due to changes in the SDK architecture, creating custom tools that use Composio's managed authentication has been modified. In the previous SDK, you could create a custom tool as follows:

**Python (previous):**

```python
# Python Example using execute_request
from composio import action, ComposioToolSet
import typing as t

toolset = ComposioToolSet()

@action(toolname="github") # Associate with GitHub app for auth
def get_github_repo_topics(
    owner: t.Annotated[str, "Repository owner username"],
    repo: t.Annotated[str, "Repository name"],
    execute_request: t.Callable # Injected by Composio
) -> dict:
    """Gets the topics associated with a specific GitHub repository."""
    response_data = execute_request(
        endpoint=f"/repos/{owner}/{repo}/topics", # API path relative to base URL
        method="GET"
    )
    if isinstance(response_data, dict):
        return {"topics": response_data.get("names", [])}
```
**TypeScript (previous):**

```typescript
// @noErrors
import { OpenAIToolSet, type ActionExecutionResDto } from "composio-core";
import { z } from "zod";

const toolset = new OpenAIToolSet();

await toolset.createAction({
    actionName: "get_github_repo_topics",
    toolName: "github",
    description: "Gets the topics associated with a specific GitHub repository.",
    inputParams: z.object({
        owner: z.string().describe("Repository owner username"),
        repo: z.string().describe("Repository name"),
    }),
    callback: async (inputParams, _authCredentials, executeRequest): Promise => {
         const { owner, repo } = inputParams as { owner: string, repo: string };
         const response = await executeRequest({
             endpoint: `/repos/${owner}/${repo}/topics`,
             method: "GET",
             parameters: [],
         });

         const topics = (response as any)?.names ?? [];
         return { successful: true, data: { topics: topics } };

});
```
The *execute tool request* method handles injection of the appropriate base URL and authentication credentials for the tool:

**Python (current):**

```python
from pydantic import BaseModel, Field
from composio import Composio
from composio.core.models.custom_tools import ExecuteRequestFn

composio = Composio()

class GetIssueInfoInput(BaseModel):
    issue_number: int = Field(
        ...,
        description="The number of the issue to get information about",
    )

# function name will be used as slug
@composio.tools.custom_tool(toolkit="github")
def get_issue_info(
    request: GetIssueInfoInput,
    execute_request: ExecuteRequestFn,
    auth_credentials: dict,
) -> dict:
    """Get information about a GitHub issue."""
    response = execute_request(
        endpoint=f"/repos/composiohq/composio/issues/{request.issue_number}",
        method="GET",
        parameters=[

                "name": "Accept",
                "value": "application/vnd.github.v3+json",
                "type": "header",
            },

                "name": "Authorization",
                "value": f"Bearer {auth_credentials['access_token']}",
                "type": "header",
            },
        ],
    )
    return {"data": response.data}
```
**TypeScript (current):**

```typescript
// @noErrors
import { Composio } from "@composio/core";
import z from "zod";

const composio = new Composio();

const tool = await composio.tools.createCustomTool({
    slug: 'GITHUB_STAR_COMPOSIOHQ_REPOSITORY',
    name: 'Github star composio repositories',
    toolkitSlug: 'github',
    description: 'Star any specificied repo of `composiohq` user',
    inputParams: z.object({
      repository: z.string().describe('The repository to star'),
      page: z.number().optional().describe('Pagination page number'),
      customHeader: z.string().optional().describe('Custom header'),
    }),
    execute: async (input, connectionConfig, executeToolRequest) => {
      const result = await executeToolRequest({
        endpoint: `/user/starred/composiohq/${input.repository}`,
        method: 'PUT',
        body: {},
        parameters: [

            name: 'page',
            value: input.page?.toString() || '1',
            in: 'query',
          },

            name: 'x-custom-header',
            value: input.customHeader || 'default-value',
            in: 'header',
          },
        ],
      });
      return result;
    },
  });
```
For more information, including executing custom tools and defining custom headers and query parameters, refer to the [Custom Tools](/docs/tools-direct/custom-tools) documentation.

## Auth configs (formerly integrations)

Integrations are now called *auth configs*. While the terminology has changed, the underlying concept remains the same.

Auth configs store the configuration required for authentication with a given toolkit, including OAuth developer credentials, configurable base URLs, and scopes.

Auth configs now use nano IDs instead of UUIDs:

| Previous (UUID) Example                | Current (Nano ID) Example |
| :------------------------------------- | :------------------------ |
| `b7a9c1e2-3f4d-4a6b-8c2e-1d2f3a4b5c6d` | `ac_8x9w2l3k5m`           |

We recommend storing auth config nano IDs in your database for connecting users to the appropriate auth configuration.

For most use cases, you will create auth configs through the dashboard, and this process remains unchanged. Read more about [creating auth configs](/docs/tools-direct/authenticating-tools#creating-an-auth-config) and [customizing auth configs](/docs/auth-configuration/custom-auth-configs).

Creating auth configs programmatically in the previous SDK:

**Python (previous):**

```python
from composio_openai import App, ComposioToolSet

toolset = ComposioToolSet()

integration = toolset.create_integration(
    app=App.GITHUB,
    auth_mode="OAUTH2",
    use_composio_oauth_app=True,
    # For use_composio_oauth_app=False, you can provide your own OAuth app credentials here
    # auth_config={
    #     "client_id": "123456",
    #     "client_secret": "123456"
    # }

)
print(integration.id)
```
**TypeScript (previous):**

```typescript
// @noErrors
import { OpenAIToolSet } from "composio-core";

const composioToolset = new OpenAIToolSet();

const integration = await composioToolset.integrations.create({
    name: "gmail_integration",
    appUniqueKey: "gmail",
    forceNewIntegration: true,
    useComposioAuth: false,
    // For useComposioAuth: false, you can provide your own OAuth app credentials here
    // authScheme: "OAUTH2",
    // authConfig: {
    //     clientId: "123456",
    //     clientSecret: "123456"
    // }
})

console.log(integration.id)
```
Creating auth configs programmatically in the current SDK:

**Python (current):**

```python
from composio import Composio

composio = Composio()

# Use composio managed auth
auth_config = composio.auth_configs.create(
    toolkit="notion",
    options={
        "type": "use_composio_managed_auth",
        # "type": "use_custom_auth",
        # "auth_scheme": "OAUTH2",
        # "credentials": {
        #     "client_id": "1234567890",
        #     "client_secret": "1234567890",
        #     "oauth_redirect_uri": "https://backend.composio.dev/api/v3/toolkits/auth/callback",
        # },
    },
)
print(auth_config)
```
**TypeScript (current):**

```typescript
// @noErrors
import { Composio } from '@composio/core';

const composio = new Composio();

const authConfig = await composio.authConfigs.create('LINEAR', {
  name: 'Linear',
  type: 'use_composio_managed_auth',
  //   type: "use_custom_auth",
  //   credentials: {
  //     client_id: "1234567890",
  //     client_secret: "1234567890",
  //     oauth_redirect_uri: "https://backend.composio.dev/api/v3/toolkits/auth/callback",
  // },
});

console.log(authConfig);
```
For using custom authentication credentials, refer to the [Programmatic Auth Configs](/docs/auth-configuration/programmatic-auth-configs) documentation.

> The callback URL for creating custom OAuth configs is now `https://backend.composio.dev/api/v3/toolkits/auth/callback`. The previous URL was `https://backend.composio.dev/api/v1/auth-apps/add`.

## Connected accounts / User IDs

The primary change in connected accounts and user IDs is that user IDs are now a more prominent concept compared to entities in previous versions.

We have simplified the process of connecting a user to a toolkit. Instead of multiple methods and parameters for initiating a connection, both the SDK and API now require only a `user_id` and `auth_config_id` to initiate a connection.

This approach is more explicit and works well with the ability for developers to have multiple auth configs for a given toolkit.

Connected accounts now use nano IDs instead of UUIDs:

| Previous (UUID) Example                | Current (Nano ID) Example |
| :------------------------------------- | :------------------------ |
| `b7a9c1e2-3f4d-4a6b-8c2e-1d2f3a4b5c6d` | `ca_8x9w2l3k5m`           |

Previously, you might have initiated a connection like this:

**Python (previous):**

```python
from composio_openai import ComposioToolSet

toolset = ComposioToolSet()
user_id = "your_user_unique_id"
google_integration_id = "0000-0000"

entity = toolset.get_entity(id=user_id)

try:
    print(f"Initiating OAuth connection for entity {entity.id}...")
    connection_request = toolset.initiate_connection(
        integration_id=google_integration_id,
        entity_id=user_id,
        # Optionally add: redirect_url="https://yourapp.com/final-destination"
        # if you want user sent somewhere specific *after* Composio finishes.
    )

    # Check if a redirect URL was provided (expected for OAuth)
    if connection_request.redirectUrl:
        print(f"Received redirect URL: {connection_request.redirectUrl}")
    else:
        print("Error: Expected a redirectUrl for OAuth flow but didn't receive one.")

except Exception as e:
    print(f"Error initiating connection: {e}")
```
**TypeScript (previous):**

```typescript
// @noErrors
import { OpenAIToolSet } from "composio-core";

const toolset = new OpenAIToolSet();
const userId = "your_user_unique_id";
const googleIntegrationId = "0000-0000";

console.log(`Initiating OAuth connection for entity ${userId}...`);
const connectionRequest = await toolset.connectedAccounts.initiate({
    integrationId: googleIntegrationId,
    entityId: userId,
    // Optionally add: redirectUri: "https://yourapp.com/final-destination"
    // if you want user sent somewhere specific *after* Composio finishes.
});

// Check if a redirect URL was provided (expected for OAuth)
if (connectionRequest?.redirectUrl) {
    console.log(`Received redirect URL: ${connectionRequest.redirectUrl}`);
    // Proceed to Step 2: Redirect the user
    // Return or pass connectionRequest to the next stage
} else {
    console.error("Error: Expected a redirectUrl for OAuth flow but didn't receive one.");

```
The current process for initiating a connection is as follows:

**Python (current):**

```python
from composio import Composio

linear_auth_config_id = "ac_1234"
user_id = "user@email.com"
composio = Composio()

# Create a new connected account
connection_request = composio.connected_accounts.initiate(
    user_id=user_id,
    auth_config_id=linear_auth_config_id,
)
print(connection_request.redirect_url)

# Wait for the connection to be established
connected_account = connection_request.wait_for_connection()
print(connected_account)
```
**TypeScript (current):**

```typescript
// @noErrors
import { Composio } from "@composio/core";

const composio = new Composio();
const linearAuthConfigId = "ac_1234";
const userId = "user@email.com";

// Initiate the OAuth connection request
const connRequest = await composio.connectedAccounts.initiate(userId, linearAuthConfigId);

const { redirectUrl, id } = connRequest;
console.log(redirectUrl);

// Wait for the connection to be established
await connRequest.waitForConnection();

// If you only have the connection request ID, you can also wait using:
await composio.connectedAccounts.waitForConnection(id);
```
## Triggers

Composio continues to support listening to application events using triggers through WebSockets and webhooks.

### Creating triggers

The process for creating triggers and specifying their configuration has been redesigned for improved clarity and intuitiveness.

Some triggers require configuration, such as repository names for GitHub triggers or channel names for Slack triggers. The process usually follows the pattern of fetching the trigger type and then creating the trigger with the appropriate configuration.

**Python (current):**

```python
from composio import Composio

composio = Composio()

user_id = "user@example.com"
trigger_config = composio.triggers.get_type("GITHUB_COMMIT_EVENT")
print(trigger_config.config)
## Trigger Config
# {
#     "properties": {
#         "owner": {
#             "description": "Owner of the repository",
#             "title": "Owner",
#             "type": "string"
#         },
#         "repo": {
#             "description": "Repository name",
#             "title": "Repo",
#             "type": "string"
#         }
#     },
#     "required": ["owner", "repo"],
#     "title": "WebhookConfigSchema",
#     "type": "object"

trigger = composio.triggers.create(
    slug="GITHUB_COMMIT_EVENT",
    user_id=user_id,
    trigger_config={"repo": "composiohq", "owner": "composio"},
)
print(trigger)

# Managing triggers

composio.triggers.enable(id="ti_abcd123")
```
**TypeScript (current):**

```typescript
// @noErrors
import { Composio } from '@composio/core';

const composio = new Composio();

const userId = 'user@acme.com';
// Fetch the trigger details
const triggerType = await composio.triggers.getType('GITHUB_COMMIT_EVENT');
console.log(JSON.stringify(triggerType.config, null, 2));
/*--- Trigger config ---

  "properties": {
    "owner": {
      "description": "Owner of the repository",
      "title": "Owner",
      "type": "string"
    },
    "repo": {
      "description": "Repository name",
      "title": "Repo",
      "type": "string"

  },
  "required": ["owner", "repo"],
  "title": "WebhookConfigSchema",
  "type": "object"

*/

const createResponse = await composio.triggers.create(userId, 'GITHUB_COMMIT_EVENT', {
  triggerConfig: {
    owner: 'composiohq',
    repo: 'composio',
  },
});
console.log(createResponse);
```
### Enabling/Disabling triggers

You can enable or disable triggers through either the SDK or the dashboard. The dashboard process remains unchanged.

Managing triggers with the SDK:

**Python:**

```python
# Disable a trigger instance
disabled_instance = composio.triggers.disable(trigger_id="ti_abcd123")
print(disabled_instance)
```
**TypeScript:**

```typescript
// @noErrors
await composio.triggers.disable("ti_abcd123");
```
If needed, the trigger can be enabled again.

**Python:**

```python
# Enable a trigger instance
enabled_instance = composio.triggers.enable(trigger_id="ti_abcd123")
print(enabled_instance)
```
**TypeScript:**

```typescript
// @noErrors
await composio.triggers.enable("ti_abcd123");
```
### Listening to triggers

We recommend listening to triggers through webhooks. The following are example routes for Next.js and FastAPI.

For development, you can also [listen to triggers through the SDK](/docs/setting-up-triggers/subscribing-to-events#sdk-subscriptions).

**FastAPI:**

```python title="app/route.py"
from fastapi import FastAPI, Request, HTTPException
from typing import Dict, Any
import uvicorn
import json
import hmac
import hashlib
import base64
import os

def verify_webhook_signature(request: Request, body: bytes) -> bool:
    """Verify Composio webhook signature"""
    webhook_signature = request.headers.get("webhook-signature")
    webhook_id = request.headers.get("webhook-id")
    webhook_timestamp = request.headers.get("webhook-timestamp")
    webhook_secret = os.getenv("COMPOSIO_WEBHOOK_SECRET")

    if not all([webhook_signature, webhook_id, webhook_timestamp, webhook_secret]):
        raise HTTPException(status_code=400, detail="Missing required webhook headers or secret")

    if not webhook_signature.startswith("v1,"):
        raise HTTPException(status_code=401, detail="Invalid signature format")

    received = webhook_signature[3:]
    signing_string = f"{webhook_id}.{webhook_timestamp}.{body.decode()}"
    expected = base64.b64encode(
        hmac.new(webhook_secret.encode(), signing_string.encode(), hashlib.sha256).digest()
    ).decode()

    if not hmac.compare_digest(received, expected):
        raise HTTPException(status_code=401, detail="Invalid webhook signature")

    return True

@app.post("/webhook")
async def webhook_handler(request: Request):
    payload = await request.json()

    trigger_type = payload.get("type")
    event_data = payload.get("data", {})

    if trigger_type == "github_star_added_event":
        repo_name = event_data.get("repository_name")
        starred_by = event_data.get("starred_by")
        print(f"Repository {repo_name} starred by {starred_by}")
        # Add your business logic here

    return {"status": "success", "message": "Webhook processed"}
```
**Next.js:**

```typescript title="app/api/webhook/route.ts"
// @noErrors
import type { NextApiRequest, NextApiResponse } from 'next';
import { TriggerEvent } from '@composio/core';
import crypto from 'crypto';

type GitHubStarEventData = {
  repository_name: string;
  repository_url: string;
  starred_by: string;
  starred_at: string;
};

function verifyWebhookSignature(
  req: NextApiRequest,
  body: string
): boolean {
  const signature = req.headers['webhook-signature'] as string | undefined;
  const msgId = req.headers['webhook-id'] as string | undefined;
  const timestamp = req.headers['webhook-timestamp'] as string | undefined;
  const secret = process.env.COMPOSIO_WEBHOOK_SECRET;

  if (!signature || !msgId || !timestamp || !secret) {
    throw new Error('Missing required webhook headers or secret');

  if (!signature.startsWith('v1,')) {
    throw new Error('Invalid signature format');

  const received = signature.slice(3);
  const signingString = `${msgId}.${timestamp}.${body}`;
  const expected = crypto
    .createHmac('sha256', secret)
    .update(signingString)
    .digest('base64');

  return crypto.timingSafeEqual(Buffer.from(received), Buffer.from(expected));

export default async function webhookHandler(req: NextApiRequest, res: NextApiResponse) {
  const payload = req.body;

  if (payload.type === 'github_star_added_event') {
    const event: TriggerEvent = {
      type: payload.type,
      timestamp: payload.timestamp,
      data: payload.data
    };

    console.log(`Repository ${event.data.repository_name} starred by ${event.data.starred_by}`);
    // Add your business logic here

  res.status(200).json({
    status: 'success',
    message: 'Webhook processed'
  });

```

# Coming Soon

## Local tools

Previously, the Python SDK included *[local tools](https://github.com/ComposioHQ/composio/tree/master/python/composio/tools/local)*. These were tools defined within the SDK and consisted of local shell and code-related tools such as "clipboard", "sqltool", and "shelltool".

This feature is currently in development for both Python and TypeScript SDKs, with newly created tools built for improved agent accuracy.

This feature is currently in development for both Python and TypeScript SDKs.

# API Endpoints

The following table lists important API endpoints that have changed. You can use this reference to quickly find the new v3 API endpoint for migration:

> This list is not exhaustive. Please refer to the [API Reference](/reference) for the complete list of endpoints.

## Toolkits (formerly Apps)

| Previous Endpoint                  | Current Endpoint                  |
| :--------------------------------- | :-------------------------------- |
| `GET /api/v1/apps`                 | `GET /api/v3/toolkits`            |
| `GET /api/v1/apps/list/categories` | `GET /api/v3/toolkits/categories` |
| `GET /api/v1/apps/{appName}`       | `GET /api/v3/toolkits/{slug}`     |

## Tools (formerly Actions)

| Previous Endpoint                                    | Current Endpoint                               |
| :--------------------------------------------------- | :--------------------------------------------- |
| `GET /api/v2/actions`                                | `GET /api/v3/tools`                            |
| `GET /api/v2/actions/list/enums`                     | `GET /api/v3/tools/enum`                       |
| `GET /api/v2/actions/{actionId}`                     | `GET /api/v3/tools/{tool_slug}`                |
| `POST /api/v2/actions/{actionId}/execute`            | `POST /api/v3/tools/execute/{tool_slug}`       |
| `POST /api/v2/actions/{actionId}/execute/get.inputs` | `POST /api/v3/tools/execute/{tool_slug}/input` |
| `POST /api/v2/actions/proxy`                         | `POST /api/v3/tools/execute/proxy`             |

## Auth Configs (formerly Integrations/Connectors)

| Previous Endpoint                             | Current Endpoint                       |
| :-------------------------------------------- | :------------------------------------- |
| `GET /api/v1/integrations`                    | `GET /api/v3/auth_configs`             |
| `POST /api/v1/integrations`                   | `POST /api/v3/auth_configs`            |
| `GET /api/v1/integrations/{integrationId}`    | `GET /api/v3/auth_configs/{nanoid}`    |
| `PATCH /api/v1/integrations/{integrationId}`  | `PATCH /api/v3/auth_configs/{nanoid}`  |
| `DELETE /api/v1/integrations/{integrationId}` | `DELETE /api/v3/auth_configs/{nanoid}` |
| `POST /api/v2/integrations/create`            | `POST /api/v3/auth_configs`            |

## Connected Accounts (formerly Connections)

| Previous Endpoint                                                | Current Endpoint                                   |
| :--------------------------------------------------------------- | :------------------------------------------------- |
| `GET /api/v1/connectedAccounts`                                  | `GET /api/v3/connected_accounts`                   |
| `POST /api/v1/connectedAccounts`                                 | `POST /api/v3/connected_accounts`                  |
| `POST /api/v2/connectedAccounts/initiateConnection`              | `POST /api/v3/connected_accounts`                  |
| `GET /api/v1/connectedAccounts/{connectedAccountId}`             | `GET /api/v3/connected_accounts/{nanoid}`          |
| `DELETE /api/v1/connectedAccounts/{connectedAccountId}`          | `DELETE /api/v3/connected_accounts/{nanoid}`       |
| `POST /api/v1/connectedAccounts/{connectedAccountId}/disable`    | `PATCH /api/v3/connected_accounts/{nanoId}/status` |
| `POST /api/v1/connectedAccounts/{connectedAccountId}/enable`     | `PATCH /api/v3/connected_accounts/{nanoId}/status` |
| `POST /api/v1/connectedAccounts/{connectedAccountId}/reinitiate` | `POST /api/v3/connected_accounts/{nanoid}/refresh` |

## Triggers

| Previous Endpoint                                                 | Current Endpoint                                      |
| :---------------------------------------------------------------- | :---------------------------------------------------- |
| `GET /api/v1/triggers`                                            | `GET /api/v3/triggers_types`                          |
| `GET /api/v1/triggers/list/enums`                                 | `GET /api/v3/triggers_types/list/enum`                |
| `GET /api/v2/triggers/{triggerName}`                              | `GET /api/v3/triggers_types/{slug}`                   |
| `GET /api/v1/triggers/active_triggers`                            | `GET /api/v3/trigger_instances/active`                |
| `POST /api/v1/triggers/enable/{connectedAccountId}/{triggerName}` | `POST /api/v3/trigger_instances/{slug}/upsert`        |
| `DELETE /api/v1/triggers/instance/{triggerInstanceId}`            | `DELETE /api/v3/trigger_instances/manage/{triggerId}` |
| `PATCH /api/v1/triggers/instance/{triggerId}/status`              | `PATCH /api/v3/trigger_instances/manage/{triggerId}`  |

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
# API (/docs/troubleshooting/api)

> For a complete list of error codes and their meanings, see [Errors Reference](/reference/errors).

# Reporting API issues

When reporting API issues to support, provide the following:

* **cURL command**: Include the exact cURL to reproduce the issue

* **Request ID**: Add `x-request-id: <uuid>` header to your request and share the UUID (generate at [uuidgenerator.net](https://www.uuidgenerator.net/))

```bash
curl 'https://backend.composio.dev/api/v3/tools' \
  -H 'x-api-key: YOUR_API_KEY' \
  -H 'x-request-id: YOUR_UUID_HERE'
```

* **Error details**: Share the complete error message. For example:

```json
"error": {
  "message": "Validation error while processing request",
  "error_code": 10400,
  "suggested_fix": "Please check the payload.",
  "errors": [
    "Error in payload.text.arguments: Only one of 'text' or 'arguments' must be provided"
  ]

```

* **Reproduction steps**: Include any steps needed to reproduce the issue

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
# Authentication (/docs/troubleshooting/authentication)

# Using Composio's default OAuth app

When using our default OAuth configuration:

* **Don't add additional scopes** - They may not be approved in our OAuth app
* Use only the pre-configured scopes provided

# Using custom OAuth apps

Ensure your OAuth app is configured correctly:

* **Redirect URL**: Must match exactly what's configured in your OAuth provider
* **Scopes**: Auth config scopes must match your OAuth app configuration
* **Credentials**: Verify client ID and secret are correct

For setup guides by toolkit: [OAuth Configuration Guides](https://composio.dev/oauth)

# Common authentication issues

* **Invalid redirect URI**: Check the callback URL matches exactly
* **Scope mismatch**: Ensure requested scopes are configured in both auth config and OAuth app
* **Expired tokens**: Try refreshing the connection
* **Rate limits**: Some providers limit authentication attempts
* **Credentials showing as `REDACTED`**: The "Mask Connected Account Secrets" setting is enabled on your account, which redacts all sensitive credential data. Navigate to **Settings → Project Settings → Project Configuration** and disable "Mask Connected Account Secrets" to view actual values

# Reporting authentication issues

When reporting to support, provide:

* **Error message**: Complete error details and screenshots. For example:

![Example: This app is blocked error screen shown by Google when un-verified sensitive scopes are configured](/images/troubleshooting/troubleshooting-auth-app-blocked.png)

* **Auth config and account IDs**: Include the `authConfigId` used for the request and, if a connection already exists, the `connected_account_id`

![Finding authConfigId and connected account ID in the Composio dashboard](/images/troubleshooting/troubleshooting-auth-config-id.png)

* **OAuth provider**: Which service you're trying to connect

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
# CLI (/docs/troubleshooting/cli)

# Command not found

Verify the CLI is installed and in your PATH:

```bash
which composio
```

If not found, reinstall:

```bash
curl -fsSL https://composio.dev/install | bash
```

Or add to PATH:

```bash
echo 'export PATH="$HOME/.composio:$PATH"' >> ~/.bashrc && source ~/.bashrc
```

# Authentication errors

Check current authentication:

```bash
composio whoami
```

Re-authenticate if needed:

```bash
composio logout
composio login
```

For CI/CD, use environment variable:

```bash
export COMPOSIO_API_KEY="your-api-key"
```

# Type generation issues

## Project type not detected

Use language-specific commands:

```bash
composio ts generate  # TypeScript
composio py generate  # Python
```

## Output directory missing

Specify output directory explicitly:

```bash
composio generate --output-dir ./my-types
```

# Debug CLI issues

Enable debug logging:

```bash
composio --log-level debug [command]
```

Check version compatibility:

```bash
composio version
```

# Common issues

* **API key not found**: Run `composio login`
* **Project type detection fails**: Use language-specific commands or ensure you're in project root
* **Network timeout**: Check internet connection and proxy settings
* **Permission denied**: Check directory write permissions

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)
* **GitHub**: [Create an issue](https://github.com/ComposioHQ/composio/issues/new?labels=bug)

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
# Dashboard (/docs/troubleshooting/dashboard)

# Reporting dashboard issues

When reporting dashboard issues to support, provide:

* **Screenshot or recording**: Visual evidence of the issue
* **Error details**: Complete error message shown in the UI
* **Network logs**: Check browser DevTools (Network tab) for failing API calls and share:
  * Request ID (`x-request-id` response header)
  * Failed endpoint URL
  * Status code and error response

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
# MCP (/docs/troubleshooting/mcp)

# Connected account not found error

This error occurs when MCP cannot find a valid connected account for authentication:

* **Specify an account**: Provide either `connected_account_id` or `user_id` in your MCP configuration
* **Default behavior**: Without specification, MCP uses `user_id=default`. If multiple connections exist with the same user\_id, the most recent is used
* **Verification checklist**:
  * Account status is `ACTIVE` (not deleted)
  * Account belongs to the same auth config used to create the MCP server

Learn more: [Single Toolkit MCP](/docs/single-toolkit-mcp)

# Getting 404 errors

Verify your URL format matches one of these patterns:

* `https://mcp.composio.dev/composio/server//mcp`
* `https://apollo-<randomID>-composio.vercel.app/v3/mcp/`
* `https://apollo.composio.dev/v3/mcp/`

# Testing and debugging

If experiencing issues, test your MCP server with:

* [Postman MCP Requests](https://learning.postman.com/docs/postman-ai-developer-tools/mcp-requests/create/)
* [MCP Inspector](https://modelcontextprotocol.io/docs/tools/inspector)

This helps identify whether the issue is with your MCP client or the server.

# Reporting MCP issues

When reporting to support, provide:

* **Error message**: Complete error details

* **MCP server URL**: The exact URL you're connecting to and the corresponding `mcp_server_id`

![Example MCP server URL shown in the Composio dashboard](/images/troubleshooting/troubleshooting-mcp-server-url.png)

* **Testing results**: Whether issue reproduces in MCP Inspector/Postman or only in specific client

* **Connected account ID**: If facing connection issues

![Example connected account ID used for MCP authentication](/images/troubleshooting/troubleshooting-mcp-connected-account-id.png)

* **Reproduction steps**: Clear steps to reproduce the issue

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
# SDKs (/docs/troubleshooting/sdks)

# Debug network issues

Enable debug logging to see API calls and identify if issues are SDK or API related. When contacting support, share the full debug logs **along with** the `x-request-id`.

**Python:**

```bash
# Set environment variable
COMPOSIO_LOGGING_LEVEL=debug
```

**TypeScript:**

```bash
// Set environment variable
COMPOSIO_LOG_LEVEL=debug
```

Look for `x-request-id` in the debug output to share with support.

# Check SDK version

Ensure you're using the latest version:

**Python:**

```bash
pip install --upgrade composio
```

**TypeScript:**

```bash
npm install @composio/core@latest
```

Check current version:

* Python: [PyPI](https://pypi.org/project/composio/)
* TypeScript: [npm](https://www.npmjs.com/package/@composio/core)

# Common issues

* **Type errors or parameter confusion**: Search [DeepWiki](https://deepwiki.com/ComposioHQ/composio) or use the Ask AI assistant
* **Tool-specific issues**: Check the [specific tool's documentation](/toolkits)
* **Bug reporting**: Create a [GitHub issue](https://github.com/ComposioHQ/composio/issues/new?labels=bug) with debug logs and reproduction steps

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
# Tools & Toolkits (/docs/troubleshooting/tools)

# Tool execution failures (401/403 errors)

Authentication and permission errors typically occur due to:

## Missing scopes

Check if your connection has the required scopes using this API:

```bash
curl --location 'https://backend.composio.dev/api/v3/tools/get_scopes_required' \
--header 'x-api-key: YOUR_COMPOSIO_API_KEY' \
--header 'Content-Type: application/json' \
--data '{
    "tools": [
        "SLACK_SENDS_A_MESSAGE_TO_A_SLACK_CHANNEL",
        "SLACK_CREATE_A_REMINDER"
    ]
}'
```

## Insufficient permissions

Verify the connected account has necessary permissions:

* **Admin requirements**: Some tools require admin-level access
* **Paid accounts**: Certain toolkits need paid subscriptions
  * Example: MS Teams requires Microsoft 365 account + Azure AD tenant

# API returning fewer tools than expected

If the Get Tools API (`GET /api/v3/tools`) returns fewer tools than what's shown on the [platform UI](https://platform.composio.dev), you likely need to specify a toolkit version.

When `toolkit_versions` is not provided, the API defaults to the base version (`00000000_00`), which only includes tools from the initial release. Tools added in later versions won't appear.

To get all available tools, pass `toolkit_versions=latest`:

```bash
curl 'https://backend.composio.dev/api/v3/tools?toolkit_slug=googledrive&toolkit_versions=latest' \
  -H 'x-api-key: YOUR_API_KEY'
```

See [Toolkit versioning](/docs/tools-direct/toolkit-versioning) for more details.

# Tool not working

* Check [tool-specific documentation](/toolkits) for requirements
* Verify the connected account is active and properly authenticated
* Test with the latest SDK version

# Reporting tool issues

When reporting to support, provide:

* **Error message**: Complete error details. For example:

```json
"data": {
  "message": "{\n  \"error\": {\n    \"code\": 404,\n    \"message\": \"Requested entity was not found.\",\n    \"errors\": [\n      {\n        \"message\": \"Requested entity was not found.\",\n        \"domain\": \"global\",\n        \"reason\": \"notFound\"\n      }\n    ],\n    \"status\": \"NOT_FOUND\"\n  }\n}\n",
  "status_code": 404
},
"successful": false,
"error": "404 Client Error: Not Found for url: https://gmail.googleapis.com/gmail/v1/users/me/messages/123456?format=full",
"log_id": "log_tVqomBgg11-t"

```

* **Log ID**: From the error response (`log_id` field), or find it in the dashboard Logs page:

![Example tool execution log showing log_id in the Composio dashboard](/images/troubleshooting/troubleshooting-tool-log-id.png)

* **Tool name**: Exact tool slug being executed (for example, `GMAIL_FETCH_MESSAGE_BY_MESSAGE_ID`)

* **Connected account ID**: Account used for execution (for example, `ca_xxx`)

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
# Triggers (/docs/troubleshooting/triggers)

# Unable to create trigger

Check the error message — the connected account might not have sufficient permissions or the required OAuth scopes. Ensure the user has authenticated with the necessary scopes for the trigger.

# Not receiving payloads

* **Polling triggers** (e.g., Gmail): These check for new events at intervals (\~1 min for Gmail). Expect small delays.
* **Webhook URL**: Ensure your URL is publicly accessible and returns a `2xx` status code.
* **Trigger status**: Verify the trigger is enabled, not disabled.
* **Logs**: Check the [trigger logs](https://platform.composio.dev?next_page=/logs/triggers) in the dashboard for delivery attempts and errors.

# Type errors with trigger payloads

Use `getType()` / `get_type()` to inspect the exact payload schema for a trigger type. This shows you the fields and types you should expect.

# Reporting issues

When contacting support, include:

* **Trigger ID** and **connected account ID** — find these under Active Triggers in the dashboard:

![Trigger ID and connected account ID in the dashboard](/images/troubleshooting/troubleshooting-trigger-id.png)

* **Error message** and **reproduction steps**

# Getting help

* **Email**: [support@composio.dev](mailto:support@composio.dev)
* **Discord**: [#support-form](https://discord.com/channels/1170785031560646836/1268871288156323901)

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
