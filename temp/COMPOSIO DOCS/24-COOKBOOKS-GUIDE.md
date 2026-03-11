# Templates (/cookbooks/templates)

Clone, customize, and deploy. Each template is a fully working project you can use as a starting point.

  - [TrustClaw](https://www.trustclaw.app/): Secure AI assistant that automates tasks across 1000+ apps with OAuth authentication and sandboxed execution.

  - [Data Analyst Agent](https://github.com/ComposioHQ/data-analyst-agent): Multi-framework AI agent that connects to HubSpot, Google Sheets, and Attio to analyze data, generate insights, and create visualizations.

  - [Open ChatGPT Atlas](https://github.com/ComposioHQ/open-chatgpt-atlas): Research and analyze topics with AI-powered deep research capabilities.

  - [Open Gamma](https://github.com/ComposioHQ/open-gamma): AI agent that generates presentation slides in Google Slides from natural language prompts.

# Direct Tool Execution

  - [Gmail Agent with FastAPI](https://github.com/ComposioHQ/composio-fastapi): Build and serve a Gmail AI agent using Composio's managed authentication, OpenAI, and FastAPI.

  - [Perplexity Email Assistant](https://github.com/ComposioHQ/open-perplexity-email-assistant): Email assistant that listens for incoming emails via triggers and automatically executes instructions using Tool Router, LangGraph, and MCP.

  - [Personalised Onboarding Agent](https://github.com/ComposioHQ/open-poke): Intelligent agent that researches users by analyzing their Gmail data and web presence, then engages in personalized conversations.

  - [Open Gumloop](https://github.com/ComposioHQ/open-gumloop): Visual drag-and-drop agent workflow builder for creating and executing AI-powered pipelines using a node-based interface.

  - [Open Email Agent](https://github.com/ComposioHQ/open-email-assistant): Chat-based email assistant that lets you search, read, reply to, and manage Gmail using natural language.

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
# Build a Chat App (/cookbooks/chat-app)

Give your AI agent access to Gmail, GitHub, Slack, Notion, and 1000+ other apps. This tutorial builds a Next.js chat app that handles tool discovery, user authentication, and action execution in about **30 lines of code**.

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/chat-app)

# What you'll build

* A chat app where your agent can find and use tools across 1000+ apps
* In-chat OAuth authentication so users can connect apps directly in the conversation
* A tool call display that shows what the agent is doing in real time

# Prerequisites

* [Bun](https://bun.sh) (or Node.js 18+)
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

**Stack:** Next.js, Vercel AI SDK, OpenAI, Composio

# Create the project

Create a new Next.js app and install the dependencies:

```bash
bunx create-next-app composio-chat --yes
cd composio-chat
bun add @composio/core @composio/vercel @ai-sdk/openai ai @ai-sdk/react
```

> Get your `COMPOSIO_API_KEY` from [Settings](https://platform.composio.dev/settings) and `OPENAI_API_KEY` from [OpenAI](https://platform.openai.com/api-keys).

Add your API keys to a `.env.local` file in the project root:

```bash title=".env.local"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
Start the dev server:

```bash
bun dev
```
# Build the backend

A **session** scopes tools and credentials to a specific user. Create `app/api/chat/route.ts`:

```ts title="app/api/chat/route.ts"
import { openai } from "@ai-sdk/openai";
import { Composio } from "@composio/core";
import { VercelProvider } from "@composio/vercel";
import {
  streamText,
  convertToModelMessages,
  generateId,
  stepCountIs,
  type UIMessage,
} from "ai";

const composio = new Composio({ provider: new VercelProvider() });

export async function POST(req: Request) {
  const { messages }: { messages: UIMessage[] } = await req.json();

  const session = await composio.create("user_123");
  const tools = await session.tools();

  const result = streamText({
    model: openai("gpt-5.2"),
    system: "You are a helpful assistant. Use Composio tools to help the user.",
    messages: await convertToModelMessages(messages),
    tools,
    stopWhen: stepCountIs(10),
  });

  return result.toUIMessageStreamResponse({
    originalMessages: messages,
    generateMessageId: () => generateId(),
  });

```
That's the entire backend. `composio.create("user_123")` creates a session that lets the agent search for tools, authenticate users, and execute actions. This ID scopes all connections and credentials to this user. In production, use the user ID from your auth system.

# Build the chat UI

Replace `app/page.tsx`:

```tsx title="app/page.tsx"
"use client";

import { useState } from "react";
import { useChat } from "@ai-sdk/react";
import { DefaultChatTransport } from "ai";

export default function Chat() {
  const [input, setInput] = useState("");
  const { messages, sendMessage, status } = useChat({
    transport: new DefaultChatTransport({ api: "/api/chat" }),
  });

  const isLoading = status === "streaming" || status === "submitted";

  return (
    <main className="max-w-2xl mx-auto p-6">
      <h1 className="text-2xl font-bold mb-6">Composio Chat</h1>

      
        {messages.length === 0 && (
          <p className="text-gray-400 text-center py-12">
            Try: &quot;Star the composio repo on GitHub&quot;
          </p>
        )}
        {messages.map((m) => (
          
            <span className="font-semibold shrink-0">
              {m.role === "user" ? "You:" : "Agent:"}
            </span>
            
              {m.parts.map((part, i) =>
                part.type === "text" ? (
                  <span key={i}>
                    {String(part.text)
                      .split(/(https?:\/\/[^\s)]+)/g)
                      .map((segment, j) =>
                        segment.match(/^https?:\/\//) ? (
                          <a key={j} href={segment} target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">{segment}</a>
                        ) : (
                          segment
                        )
                      )}
                  </span>
                ) : null
              )}
            
          
        ))}
        {isLoading && (
          <p className="text-gray-400 text-sm">Thinking...</p>
        )}
      

      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (!input.trim()) return;
          sendMessage({ text: input });
          setInput("");
        }}
        className="flex gap-2"
      >
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Ask me anything..."
          disabled={isLoading}
          className="flex-1 p-3 border border-gray-300 rounded-lg"
        />
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-3 bg-white text-black font-medium rounded-lg hover:bg-gray-200 transition-colors disabled:opacity-50"
        >
          Send
        </button>
      </form>
    </main>
  );

```
Go to [localhost:3000](http://localhost:3000). You should see the chat interface.

# Try a tool call

Send this message:

```
Star the composio repo on GitHub
```
Here's what happens:

1. The agent searches for relevant tools, finds `GITHUB_STAR_REPO`, and checks your connection status.
2. You haven't connected GitHub yet, so the agent generates an auth link and shows it to you.
3. Click the link, authorize with GitHub, and tell the agent you're done.
4. The agent executes `GITHUB_STAR_REPO` with your credentials.

Discovery, authentication, and execution all happened automatically. You didn't write any tool-specific code.

> This in-chat authentication flow works out of the box. For production apps, you can [authenticate users ahead of time](/docs/authenticating-users/manually-authenticating) during onboarding.

# Show tool calls in the UI

Tool calls happen invisibly by default. Add a component that shows the tool name and status so you can see what the agent is doing.

Create `components/ToolCallDisplay.tsx`:

```tsx title="components/ToolCallDisplay.tsx"
"use client";

import { useState } from "react";

export function ToolCallDisplay({
  toolName,
  input,
  output,
  isLoading,
}: {
  toolName: string;
  input: unknown;
  output?: unknown;
  isLoading: boolean;
}) {
  const [expanded, setExpanded] = useState(false);

  return (
    
      <button
        onClick={() => setExpanded(!expanded)}
        className={`inline-flex items-center gap-2 rounded-md border px-3 py-1.5 text-xs transition-colors ${
          isLoading
            ? "border-gray-200 bg-gray-50 text-gray-500 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-400"
            : "border-gray-200 bg-gray-50 text-gray-700 hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-300 dark:hover:bg-gray-800"
        }`}
      >
        {isLoading ? (
          <span className="inline-block h-3 w-3 animate-spin rounded-full border border-gray-300 border-t-gray-600 dark:border-gray-600 dark:border-t-gray-300" />
        ) : (
          <span className="text-green-600 dark:text-green-400">✓</span>
        )}
        <code className="font-mono">{toolName}</code>
        {!isLoading && <span className="text-gray-400">{expanded ? "▴" : "▾"}</span>}
      </button>

      {expanded && !isLoading && (
        <pre className="mt-1 ml-1 max-h-40 max-w-full overflow-auto whitespace-pre-wrap break-words rounded-md border border-gray-200 bg-gray-50 p-2 text-xs text-gray-600 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-400">
          {output != null
            ? String(JSON.stringify(output, null, 2))
            : String(JSON.stringify(input, null, 2))}
        </pre>
      )}
    
  );

```
Update `app/page.tsx` to render tool calls:

```tsx title="app/page.tsx"
// @errors: 2307
"use client";

import { useState } from "react";
import { useChat } from "@ai-sdk/react";
import { DefaultChatTransport, getToolName, isToolUIPart } from "ai";
import { ToolCallDisplay } from "../components/ToolCallDisplay";

export default function Chat() {
  const [input, setInput] = useState("");
  const { messages, sendMessage, status } = useChat({
    transport: new DefaultChatTransport({ api: "/api/chat" }),
  });

  const isLoading = status === "streaming" || status === "submitted";

  return (
    <main className="max-w-2xl mx-auto p-6">
      <h1 className="text-2xl font-bold mb-6">Composio Chat</h1>

      
        {messages.length === 0 && (
          <p className="text-gray-400 text-center py-12">
            Try: &quot;Star the composio repo on GitHub&quot;
          </p>
        )}
        {messages.map((m) => (
          
            <span className="font-semibold shrink-0">
              {m.role === "user" ? "You:" : "Agent:"}
            </span>
            
              {m.parts.map((part, i) => {
                if (part.type === "text") {
                  return (
                    <span key={i}>
                      {String(part.text)
                        .split(/(https?:\/\/[^\s)]+)/g)
                        .map((segment, j) =>
                          segment.match(/^https?:\/\//) ? (
                            <a key={j} href={segment} target="_blank" rel="noopener noreferrer" className="text-blue-600 underline">{segment}</a>
                          ) : (
                            segment
                          )
                        )}
                    </span>
                  );

                if (isToolUIPart(part)) {
                  return (
                    
                  );

                return null;
              })}
            
          
        ))}
        {isLoading && (
          <p className="text-gray-400 text-sm">Thinking...</p>
        )}
      

      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (!input.trim()) return;
          sendMessage({ text: input });
          setInput("");
        }}
        className="flex gap-2"
      >
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Ask me anything..."
          disabled={isLoading}
          className="flex-1 p-3 border border-gray-300 rounded-lg"
        />
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-3 bg-white text-black font-medium rounded-lg hover:bg-gray-200 transition-colors disabled:opacity-50"
        >
          Send
        </button>
      </form>
    </main>
  );

```

Tool calls now show inline with a spinner while running and a checkmark when complete. Click to expand and see the raw output.

# How it works

The session handles tool discovery and execution at runtime. Instead of loading thousands of tool definitions into your agent's context, the agent searches for what it needs, authenticates the user if necessary, and executes the action. Token refresh and credential management are handled automatically.

Try a few more requests:

* "Summarize my emails from today"
* "What's on my calendar this week?"
* "Create a GitHub issue in my repo"

# Take it further

- [Build an App Connections Dashboard](/cookbooks/app-connections-dashboard): 
A dedicated page where users manage their app connections

- [Managing multiple connected accounts](/docs/managing-multiple-connected-accounts): 
Let users connect multiple accounts for the same toolkit

- [Configuring sessions](/docs/configuring-sessions): 
Lock down which toolkits your agent can access

- [How Composio works](/docs/how-composio-works): 
Understand sessions, tool discovery, and execution under the hood

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
# Build an App Connections Dashboard (/cookbooks/app-connections-dashboard)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/app-connections-dashboard)

[Build a Chat App](/cookbooks/chat-app) handles authentication in-chat. That works for getting started, but production apps need a dedicated page where users manage their connections. This cookbook builds a dashboard where users can see all available apps, connect via OAuth, and disconnect at any time.

# What you'll build

* A connections dashboard showing all available apps and their auth status
* Connect and disconnect buttons that handle OAuth flows

# Prerequisites

* [Bun](https://bun.sh) (or Node.js 18+)
* [Composio API key](https://platform.composio.dev/settings)

**Stack:** Next.js, `@composio/core`

# Create the project

```bash
bunx create-next-app composio-dashboard --yes
cd composio-dashboard
bun add @composio/core
```

Add your API key to a `.env.local` file:

```bash title=".env.local"
COMPOSIO_API_KEY=your_composio_api_key
```
# Setting up the client

Create `app/api/connections/route.ts`. Initialize the Composio client and set `dynamic = "force-dynamic"` so Next.js always fetches fresh connection data.

```ts no-twoslash title="app/api/connections/route.ts"
import { Composio } from "@composio/core";

const composio = new Composio();

export const dynamic = "force-dynamic";
```
`"user_123"` is a placeholder. In production, replace it with the authenticated user's ID from your auth system. See [Users & Sessions](/docs/users-and-sessions) for details.

# List connections

The `GET` handler creates a session and returns every toolkit's name, logo, and connection status. Toolkits where `isNoAuth` is `true` don't require authentication, so we filter them out.

```ts no-twoslash title="app/api/connections/route.ts"
export async function GET() {
  const session = await composio.create("user_123");
  const { items } = await session.toolkits({ limit: 50 });

  return Response.json({
    // Filter out toolkits that don't require authentication
    toolkits: items
      .filter((t) => !t.isNoAuth)
      .map((t) => ({
        slug: t.slug,
        name: t.name,
        logo: t.logo,
        isConnected: t.connection?.isActive ?? false,
        connectedAccountId: t.connection?.connectedAccount?.id,
      })),
  });

```
`session.toolkits()` returns each toolkit with a `connection` object. When `connection.isActive` is `true`, the user has already authorized that app. We also return the `connectedAccountId` so the frontend can disconnect later.

> `session.toolkits()` paginates results. This example fetches up to 50. Use the `nextCursor` value from the response to fetch the next page.

# Connect an app

The `POST` handler starts an OAuth flow for a given toolkit and returns the redirect URL.

```ts no-twoslash title="app/api/connections/route.ts"
export async function POST(req: Request) {
  const { toolkit }: { toolkit: string } = await req.json();
  const origin = new URL(req.url).origin;
  const session = await composio.create("user_123");
  const connectionRequest = await session.authorize(toolkit, {
    callbackUrl: origin,
  });

  return Response.json({ redirectUrl: connectionRequest.redirectUrl });

```
`session.authorize(toolkit)` creates a connection request with a `callbackUrl`. After the user completes OAuth, they get redirected back to your app.

# Disconnect an app

Create `app/api/connections/disconnect/route.ts`. This endpoint takes a `connectedAccountId` and deletes it:

```ts title="app/api/connections/disconnect/route.ts"
import { Composio } from "@composio/core";

const composio = new Composio();

export async function POST(req: Request) {
  const { connectedAccountId }: { connectedAccountId: string } =
    await req.json();
  await composio.connectedAccounts.delete(connectedAccountId);
  return Response.json({ success: true });

```
`composio.connectedAccounts.delete()` is a standalone SDK method. No session or provider needed.

# Build the dashboard

Replace `app/page.tsx` with a dashboard that shows connection cards:

```tsx title="app/page.tsx"
"use client";

import { useEffect, useState } from "react";

type Toolkit = {
  slug: string;
  name: string;
  logo?: string;
  isConnected: boolean;
  connectedAccountId?: string;
};

export default function Dashboard() {
  const [toolkits, setToolkits] = useState
              )}
              
                <p className="font-medium">{t.name}</p>
                <p
                  className={`text-xs ${
                    t.isConnected ? "text-green-600" : "text-gray-400"
                  }`}
                >
                  {t.isConnected ? "Connected" : "Not connected"}
                </p>
              
            
            {t.isConnected ? (
              <button
                onClick={() => disconnect(t.connectedAccountId!)}
                className="px-3 py-1.5 text-sm border rounded hover:bg-gray-50"
              >
                Disconnect
              </button>
            ) : (
              <button
                onClick={() => connect(t.slug)}
                className="px-3 py-1.5 text-sm bg-black text-white rounded hover:bg-gray-800"
              >
                Connect
              </button>
            )}
          
        ))}
      
    </main>
  );

```
The page fetches toolkit connection status on load. Clicking **Connect** redirects the user to OAuth. After they authorize, the `callbackUrl` brings them back to the dashboard with the connection now active. Clicking **Disconnect** deletes the connection and refreshes the list.

# Complete route

The full `app/api/connections/route.ts` with both handlers:

```ts title="app/api/connections/route.ts"
// region setup
import { Composio } from "@composio/core";

const composio = new Composio();

export const dynamic = "force-dynamic";
// endregion setup

// region list
export async function GET() {
  const session = await composio.create("user_123");
  const { items } = await session.toolkits({ limit: 50 });

  return Response.json({
    // Filter out toolkits that don't require authentication
    toolkits: items
      .filter((t) => !t.isNoAuth)
      .map((t) => ({
        slug: t.slug,
        name: t.name,
        logo: t.logo,
        isConnected: t.connection?.isActive ?? false,
        connectedAccountId: t.connection?.connectedAccount?.id,
      })),
  });

// endregion list

// region connect
export async function POST(req: Request) {
  const { toolkit }: { toolkit: string } = await req.json();
  const origin = new URL(req.url).origin;
  const session = await composio.create("user_123");
  const connectionRequest = await session.authorize(toolkit, {
    callbackUrl: origin,
  });

  return Response.json({ redirectUrl: connectionRequest.redirectUrl });

// endregion connect

```
# Running the app

```bash
bun dev
```

Open [localhost:3000](http://localhost:3000). You'll see all available apps with their connection status.

1. Click **Connect** on GitHub. You'll be redirected to GitHub's OAuth page.
2. Authorize the app. You'll land back on the dashboard with GitHub showing "Connected."
3. Click **Disconnect** on GitHub. The connection is removed immediately.

# Take it further

- [Managing multiple connected accounts](/docs/managing-multiple-connected-accounts): 
Let users connect multiple accounts for the same toolkit

- [Connected accounts](/docs/auth-configuration/connected-accounts): 
List, refresh, disable, and delete user connections

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
# Basic FastAPI Server (/cookbooks/fast-api)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/fast-api)

This cookbook builds a FastAPI server where users can chat with an AI agent that has access to over 1000 tools like Gmail, GitHub, Slack, Notion, and more. Along the way we'll add endpoints to manage which apps a user has connected.

# Prerequisites

* Python 3.10+
* [UV](https://docs.astral.sh/uv/getting-started/installation/)
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-fastapi && cd composio-fastapi
uv init && uv add composio composio-openai-agents openai-agents fastapi uvicorn pydantic
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
# Initializing the clients

`Composio` takes an `OpenAIAgentsProvider` so that when we ask for tools later, they come back in the format the OpenAI Agents SDK expects.

```py
from agents import Agent, Runner
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider
from fastapi import FastAPI
from pydantic import BaseModel

composio = Composio(provider=OpenAIAgentsProvider())

app = FastAPI()
```
# Chat endpoint

The core of the server is a `/chat` endpoint. A user sends a message, and the agent responds, using whatever tools it needs.

`composio.create()` creates a **session** scoped to a user. Calling `session.tools()` returns a set of meta tools that let the agent discover tools across all apps, handle OAuth when needed, and execute actions. We pass these to an `Agent` and let `Runner.run_sync` handle the agentic loop.

```py
class ChatRequest(BaseModel):
    user_id: str
    message: str

@app.post("/chat")
def chat(request: ChatRequest):
    """Send a message to an AI agent with access to all tools."""
    session = composio.create(user_id=request.user_id)
    tools = session.tools()

    agent = Agent(
        name="Assistant",
        instructions="You are a helpful assistant. Use tools to help the user.",
        tools=tools,
    )

    result = Runner.run_sync(starting_agent=agent, input=request.message)
    return {"response": result.final_output}
```
> If the user hasn't connected the app they're trying to use, the agent will automatically
return an authentication link in its response. The user can complete OAuth and then retry.

That's a working agent. But in most apps you'll also want to manage connections outside of chat, for example to show users what's connected or let them connect new apps from a settings page.

# Checking connections

`session.toolkits()` returns every toolkit in the session along with its connection status. We can expose this as a simple GET endpoint so your frontend can render a connections UI.

```py
@app.get("/connections/{user_id}")
def list_connections(user_id: str):
    """List all toolkits and their connection status for a user."""
    session = composio.create(user_id=user_id)
    toolkits = session.toolkits()
    return [
        {"toolkit": t.slug, "connected": t.connection.is_active if t.connection else False}
        for t in toolkits.items
    ]
```
If you just need to check a single toolkit, say before kicking off a workflow that requires Gmail, you can scope the session to that toolkit:

```py
@app.get("/connections/{user_id}/{toolkit}")
def check_connection(user_id: str, toolkit: str):
    """Check if a specific toolkit is connected for a user."""
    session = composio.create(user_id=user_id, toolkits=[toolkit])
    toolkits = session.toolkits()
    for t in toolkits.items:
        if t.slug == toolkit:
            return {"toolkit": toolkit, "connected": t.connection.is_active if t.connection else False}
    return {"toolkit": toolkit, "connected": False}
```
# Connecting an app

When a user wants to connect a new app, `session.authorize()` starts the OAuth flow and returns a redirect URL. Your frontend sends the user there, and once they complete auth, they're connected.

```py
class ConnectRequest(BaseModel):
    user_id: str

@app.post("/connect/{toolkit}")
def connect_toolkit(toolkit: str, request: ConnectRequest):
    """Start OAuth for a toolkit. Returns a URL to redirect the user to."""
    session = composio.create(user_id=request.user_id, toolkits=[toolkit])
    connection_request = session.authorize(toolkit)
    return {"redirect_url": connection_request.redirect_url}
```
# Complete app

Here's everything together:

```py
# region setup
from agents import Agent, Runner
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider
from fastapi import FastAPI
from pydantic import BaseModel

composio = Composio(provider=OpenAIAgentsProvider())

app = FastAPI()
# endregion setup

# region chat
class ChatRequest(BaseModel):
    user_id: str
    message: str

@app.post("/chat")
def chat(request: ChatRequest):
    """Send a message to an AI agent with access to all tools."""
    session = composio.create(user_id=request.user_id)
    tools = session.tools()

    agent = Agent(
        name="Assistant",
        instructions="You are a helpful assistant. Use tools to help the user.",
        tools=tools,
    )

    result = Runner.run_sync(starting_agent=agent, input=request.message)
    return {"response": result.final_output}
# endregion chat

# region list-connections
@app.get("/connections/{user_id}")
def list_connections(user_id: str):
    """List all toolkits and their connection status for a user."""
    session = composio.create(user_id=user_id)
    toolkits = session.toolkits()
    return [
        {"toolkit": t.slug, "connected": t.connection.is_active if t.connection else False}
        for t in toolkits.items
    ]
# endregion list-connections

# region check-connection
@app.get("/connections/{user_id}/{toolkit}")
def check_connection(user_id: str, toolkit: str):
    """Check if a specific toolkit is connected for a user."""
    session = composio.create(user_id=user_id, toolkits=[toolkit])
    toolkits = session.toolkits()
    for t in toolkits.items:
        if t.slug == toolkit:
            return {"toolkit": toolkit, "connected": t.connection.is_active if t.connection else False}
    return {"toolkit": toolkit, "connected": False}
# endregion check-connection

# region connect
class ConnectRequest(BaseModel):
    user_id: str

@app.post("/connect/{toolkit}")
def connect_toolkit(toolkit: str, request: ConnectRequest):
    """Start OAuth for a toolkit. Returns a URL to redirect the user to."""
    session = composio.create(user_id=request.user_id, toolkits=[toolkit])
    connection_request = session.authorize(toolkit)
    return {"redirect_url": connection_request.redirect_url}
# endregion connect

```
# Running the server

```bash
uv run --env-file .env uvicorn main:app --reload
```
The server runs at `http://localhost:8000`. Visit `/docs` for the interactive Swagger UI.

# Testing with cURL

## Chat with the agent

```bash
curl -X POST http://localhost:8000/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id": "user_123", "message": "Star the composiohq/composio repo on GitHub"}'
```
## List all connections

```bash
curl http://localhost:8000/connections/user_123
```
## Check a specific connection

```bash
curl http://localhost:8000/connections/user_123/gmail
```
## Connect an app

```bash
curl -X POST http://localhost:8000/connect/gmail \
  -H "Content-Type: application/json" \
  -d '{"user_id": "user_123"}'
```

Open the `redirect_url` from the response in your browser to complete OAuth.

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
# Basic Hono Server (/cookbooks/hono)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/hono)

This cookbook builds a Hono.js server where users can chat with an AI agent that has access to over 1000 tools like Gmail, GitHub, Slack, Notion, and more. Along the way we'll add endpoints to manage which apps a user has connected.

# Prerequisites

* Node.js 18+
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-hono && cd composio-hono
npm init -y
npm install hono @hono/node-server openai @composio/core @composio/openai tsx dotenv
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
# Initializing the clients

`Composio` takes an `OpenAIProvider` so that when we ask for tools later, they come back in the format OpenAI expects.

```ts no-twoslash
const composio = new Composio({ provider: new OpenAIProvider() });
const openai = new OpenAI();

const app = new Hono();
```
# Chat endpoint

The core of the server is a `/chat` endpoint. A user sends a message, and the agent responds, using whatever tools it needs.

`composio.create()` creates a **session** scoped to a user. Calling `session.tools()` returns a set of meta tools that let the agent discover tools across all apps, handle OAuth when needed, and execute actions. We pass these to OpenAI and loop until the model stops calling tools.

```ts no-twoslash
app.post("/chat", async (c) => {
  const { userId, message } = await c.req.json();

  const session = await composio.create(userId);
  const tools = await session.tools();

  const messages: OpenAI.ChatCompletionMessageParam[] = [
    { role: "system", content: "You are a helpful assistant. Use tools to help the user." },
    { role: "user", content: message },
  ];

  while (true) {
    const response = await openai.chat.completions.create({
      model: "gpt-5.2",
      tools,
      messages,
    });

    const choice = response.choices[0];
    if (!choice.message.tool_calls?.length) {
      return c.json({ response: choice.message.content });

    messages.push(choice.message);
    const toolResults = await composio.provider.handleToolCalls(userId, response);
    messages.push(...toolResults);

});
```
> If the user hasn't connected the app they're trying to use, the agent will automatically
return an authentication link in its response. The user can complete OAuth and then retry.

That's a working agent. But in most apps you'll also want to manage connections outside of chat, for example to show users what's connected or let them connect new apps from a settings page.

# Checking connections

`session.toolkits()` returns every toolkit in the session along with its connection status. We can expose this as a simple GET endpoint so your frontend can render a connections UI.

```ts no-twoslash
app.get("/connections/:userId", async (c) => {
  const userId = c.req.param("userId");

  const session = await composio.create(userId);
  const toolkits = await session.toolkits();

  return c.json(
    toolkits.items.map((t) => ({
      toolkit: t.slug,
      connected: t.connection?.isActive ?? false,
    }))
  );
});
```
If you just need to check a single toolkit, say before kicking off a workflow that requires Gmail, you can scope the session to that toolkit:

```ts no-twoslash
app.get("/connections/:userId/:toolkit", async (c) => {
  const userId = c.req.param("userId");
  const toolkit = c.req.param("toolkit");

  const session = await composio.create(userId, { toolkits: [toolkit] });
  const result = await session.toolkits();
  const match = result.items.find((t) => t.slug === toolkit);

  return c.json({ toolkit, connected: match?.connection?.isActive ?? false });
});
```
# Connecting an app

When a user wants to connect a new app, `session.authorize()` starts the OAuth flow and returns a redirect URL. Your frontend sends the user there, and once they complete auth, they're connected.

```ts no-twoslash
app.post("/connect/:toolkit", async (c) => {
  const toolkit = c.req.param("toolkit");
  const { userId } = await c.req.json();

  const session = await composio.create(userId, { toolkits: [toolkit] });
  const connectionRequest = await session.authorize(toolkit);

  return c.json({ redirectUrl: connectionRequest.redirectUrl });
});
```
# Complete app

Here's everything together:

```ts title="server.ts"
import "dotenv/config";
import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { OpenAI } from "openai";
import { Composio } from "@composio/core";
import { OpenAIProvider } from "@composio/openai";

// #region setup
const composio = new Composio({ provider: new OpenAIProvider() });
const openai = new OpenAI();

const app = new Hono();
// #endregion setup

// #region chat
app.post("/chat", async (c) => {
  const { userId, message } = await c.req.json();

  const session = await composio.create(userId);
  const tools = await session.tools();

  const messages: OpenAI.ChatCompletionMessageParam[] = [
    { role: "system", content: "You are a helpful assistant. Use tools to help the user." },
    { role: "user", content: message },
  ];

  while (true) {
    const response = await openai.chat.completions.create({
      model: "gpt-5.2",
      tools,
      messages,
    });

    const choice = response.choices[0];
    if (!choice.message.tool_calls?.length) {
      return c.json({ response: choice.message.content });

    messages.push(choice.message);
    const toolResults = await composio.provider.handleToolCalls(userId, response);
    messages.push(...toolResults);

});
// #endregion chat

// #region list-connections
app.get("/connections/:userId", async (c) => {
  const userId = c.req.param("userId");

  const session = await composio.create(userId);
  const toolkits = await session.toolkits();

  return c.json(
    toolkits.items.map((t) => ({
      toolkit: t.slug,
      connected: t.connection?.isActive ?? false,
    }))
  );
});
// #endregion list-connections

// #region check-connection
app.get("/connections/:userId/:toolkit", async (c) => {
  const userId = c.req.param("userId");
  const toolkit = c.req.param("toolkit");

  const session = await composio.create(userId, { toolkits: [toolkit] });
  const result = await session.toolkits();
  const match = result.items.find((t) => t.slug === toolkit);

  return c.json({ toolkit, connected: match?.connection?.isActive ?? false });
});
// #endregion check-connection

// #region connect
app.post("/connect/:toolkit", async (c) => {
  const toolkit = c.req.param("toolkit");
  const { userId } = await c.req.json();

  const session = await composio.create(userId, { toolkits: [toolkit] });
  const connectionRequest = await session.authorize(toolkit);

  return c.json({ redirectUrl: connectionRequest.redirectUrl });
});
// #endregion connect

serve({ fetch: app.fetch, port: 8000 });
console.log("Server running on http://localhost:8000");

```
# Running the server

```bash
npx tsx server.ts
```
The server runs at `http://localhost:8000`.

# Testing with cURL

## Chat with the agent

```bash
curl -X POST http://localhost:8000/chat \
  -H "Content-Type: application/json" \
  -d '{"userId": "user_123", "message": "Star the composiohq/composio repo on GitHub"}'
```
## List all connections

```bash
curl http://localhost:8000/connections/user_123
```
## Check a specific connection

```bash
curl http://localhost:8000/connections/user_123/gmail
```
## Connect an app

```bash
curl -X POST http://localhost:8000/connect/gmail \
  -H "Content-Type: application/json" \
  -d '{"userId": "user_123"}'
```

Open the `redirectUrl` from the response in your browser to complete OAuth.

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
# Background Agent (/cookbooks/background-agent)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/background-agent)

This cookbook builds a background agent that runs a "morning sweep" across your apps. It checks GitHub for PRs that need your review, finds unanswered emails in Gmail, and posts a summary to Slack. One script, three apps, zero human input.

Run it manually, put it on a cron, or deploy it. The agent does the rest.

# Prerequisites

* Node.js 18+
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-background-agent && cd composio-background-agent
npm init -y
npm install ai @ai-sdk/openai @composio/core @composio/vercel dotenv tsx
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
# Initializing the clients

`Composio` takes a `VercelProvider` so that tools come back as Vercel AI SDK tools with built-in execution. No manual tool-call loop needed.

```ts no-twoslash
const composio = new Composio({ provider: new VercelProvider() });
const userId = process.env.COMPOSIO_USER_ID ?? "default";
const model = openai("gpt-5.2");
```
# The sweep

The entire agent is one function. Create a session, get tools, and give the model a prompt describing what to do. `generateText` with `stopWhen: stepCountIs(15)` lets the model call as many tools as it needs, discovering available actions, authenticating with apps, and executing across GitHub, Gmail, and Slack, all in a single run.

```ts no-twoslash
async function sweep() {
  const session = await composio.create(userId);
  const tools = await session.tools();

  const { text } = await generateText({
    model,
    tools,
    stopWhen: stepCountIs(15),
    prompt: `You are an autonomous background agent running a scheduled sweep.

Do the following, in order:

1. **GitHub**:Find pull requests where my review is requested.
   List each PR with its title, repo, and link.

2. **Gmail**:Find emails from the last 24 hours that I haven't replied to.
   List each with sender and subject.

3. **Slack**: Post a concise summary of your findings to the #morning-sweep channel.
   Format it as a digest with sections for PRs and emails.
   If nothing was found, say so.

If an app is not connected, skip it and note it in the summary.
Be concise and actionable.`,
  });

  console.log(text);

sweep();
```
That's it. The agent figures out which tools to call, handles pagination, formats the results, and posts to Slack. Composio's meta-tools handle tool discovery and auth inline. If an app isn't connected, the agent skips it gracefully.

# Complete script

```ts no-twoslash title="sweep.ts"
import "dotenv/config";
import { openai } from "@ai-sdk/openai";
import { generateText, stepCountIs } from "ai";
import { Composio } from "@composio/core";
import { VercelProvider } from "@composio/vercel";

// #region setup
const composio = new Composio({ provider: new VercelProvider() });
const userId = process.env.COMPOSIO_USER_ID ?? "default";
const model = openai("gpt-5.2");
// #endregion setup

// #region sweep
async function sweep() {
  const session = await composio.create(userId);
  const tools = await session.tools();

  const { text } = await generateText({
    model,
    tools,
    stopWhen: stepCountIs(15),
    prompt: `You are an autonomous background agent running a scheduled sweep.

Do the following, in order:

1. **GitHub**:Find pull requests where my review is requested.
   List each PR with its title, repo, and link.

2. **Gmail**:Find emails from the last 24 hours that I haven't replied to.
   List each with sender and subject.

3. **Slack**: Post a concise summary of your findings to the #morning-sweep channel.
   Format it as a digest with sections for PRs and emails.
   If nothing was found, say so.

If an app is not connected, skip it and note it in the summary.
Be concise and actionable.`,
  });

  console.log(text);

sweep();
// #endregion sweep

```
# Running the sweep

```bash
npx tsx sweep.ts
```
Example output:

```
## GitHub: PRs where your review is requested (3)
- fix: robust platform detection (ComposioHQ/composio)
  https://github.com/ComposioHQ/composio/pull/2712
- docs: add retry to health check workflow (ComposioHQ/composio)
  https://github.com/ComposioHQ/composio/pull/2711
- chore(deps): bump next (ComposioHQ/composio)
  https://github.com/ComposioHQ/composio/pull/2473

## Gmail: Emails from last 24h you haven't replied to (1)
- Stephanie M. - CoreWeave - We'd love to chat!

## Slack: Digest posted to #morning-sweep
```
> If an app isn't connected yet, the agent will return an authentication link in the output. Open it in your browser to complete OAuth, then run the sweep again.

# Running on a schedule

The agent runs once and exits, which makes it perfect for cron jobs. Pick whichever scheduler you prefer.

## crontab (every day at 8am)

```bash
crontab -e
```
```
0 8 * * * cd /path/to/composio-background-agent && npx tsx sweep.ts >> sweep.log 2>&1
```
## GitHub Actions

```yaml title=".github/workflows/sweep.yml"
name: Morning Sweep
on:
  schedule:
    - cron: '0 8 * * *' # 8am UTC daily
  workflow_dispatch: # manual trigger

jobs:
  sweep:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      - run: npm install
      - run: npx tsx sweep.ts
        env:
          COMPOSIO_API_KEY: ${{ secrets.COMPOSIO_API_KEY }}
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
```

# Take it further

The prompt is the agent's brain. Change it to change what it does. Some ideas:

* **Triage support tickets**: scan Zendesk for unassigned tickets, categorize them, assign to the right team in Linear
* **Daily standup prep**: pull yesterday's commits from GitHub, completed tasks from Linear, and draft a standup summary
* **Invoice follow-up**: check QuickBooks for overdue invoices, draft follow-up emails in Gmail

- [Hono Server](/cookbooks/hono): Wrap this agent in an HTTP server so multiple users can interact with it

- [Vercel AI SDK Provider](/docs/providers/vercel): Learn more about how Composio tools work with the Vercel AI SDK

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
# Support Knowledge Agent (/cookbooks/support-agent)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/support-agent)

This cookbook builds an **agentic RAG** system: an interactive CLI agent that triages support issues by pulling context from Notion docs, Datadog monitors, and GitHub issues. It uses scoped sessions, multi-turn chat with streaming, and a structured system prompt.

# Prerequisites

* Python 3.10+
* [UV](https://docs.astral.sh/uv/getting-started/installation/)
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-support-agent && cd composio-support-agent
uv init && uv add composio composio-openai-agents openai-agents
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
# Setting up the client

`Composio` takes an `OpenAIAgentsProvider` so that tools come back in the format the OpenAI Agents SDK expects. We also import the streaming event types we'll need for real-time output.

```py
import asyncio

from agents import Agent, Runner
from agents.stream_events import RawResponsesStreamEvent
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider
from openai.types.responses import ResponseTextDeltaEvent

composio = Composio(provider=OpenAIAgentsProvider())
```
# Defining the agent

The system prompt tells the agent what tools it has and how to behave. It knows about Datadog, Notion, and GitHub, and decides on its own which to use based on the question.

```py
SYSTEM_PROMPT = """You are a Support Knowledge Agent. Use your tools to help the user triage issues, find documentation, and manage incidents. Call tools first, then respond with what you found. Be concise."""

def create_agent(tools) -> Agent:
    return Agent(
        name="Support Knowledge Agent",
        model="gpt-5.2",
        instructions=SYSTEM_PROMPT,
        tools=tools,
    )
```
# Chat loop with streaming

The chat loop creates a session scoped to three toolkits: `datadog`, `notion`, and `github`. The agent only sees tools from these services. `Runner.run_streamed` streams tokens as they arrive so you see the response in real time. Message history is tracked in a list for multi-turn context.

```py
async def main():
    user_id = "default"
    session = composio.create(
        user_id=user_id,
        toolkits=["datadog", "notion", "github"],
    )
    tools = session.tools()
    agent = create_agent(tools)

    messages = []
    print("Support Knowledge Agent (type 'quit' to exit)")
    print("-" * 50)

    while True:
        user_input = input("\nYou: ").strip()
        if not user_input or user_input.lower() == "quit":
            break

        messages.append({"role": "user", "content": user_input})

        print("\nAgent: ", end="", flush=True)
        result = Runner.run_streamed(starting_agent=agent, input=messages, max_turns=30)
        async for event in result.stream_events():
            if isinstance(event, RawResponsesStreamEvent) and isinstance(event.data, ResponseTextDeltaEvent):
                print(event.data.delta, end="", flush=True)
        print()

        messages.append({"role": "assistant", "content": result.final_output})

asyncio.run(main())
```
> If a toolkit isn't connected yet, the agent will automatically return an authentication link in its response. The user can complete OAuth and then retry.

# Complete script

Here's everything together:

```py
# region setup
import asyncio

from agents import Agent, Runner
from agents.stream_events import RawResponsesStreamEvent
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider
from openai.types.responses import ResponseTextDeltaEvent

composio = Composio(provider=OpenAIAgentsProvider())
# endregion setup

# region agent
SYSTEM_PROMPT = """You are a Support Knowledge Agent. Use your tools to help the user triage issues, find documentation, and manage incidents. Call tools first, then respond with what you found. Be concise."""

def create_agent(tools) -> Agent:
    return Agent(
        name="Support Knowledge Agent",
        model="gpt-5.2",
        instructions=SYSTEM_PROMPT,
        tools=tools,
    )
# endregion agent

# region chat
async def main():
    user_id = "default"
    session = composio.create(
        user_id=user_id,
        toolkits=["datadog", "notion", "github"],
    )
    tools = session.tools()
    agent = create_agent(tools)

    messages = []
    print("Support Knowledge Agent (type 'quit' to exit)")
    print("-" * 50)

    while True:
        user_input = input("\nYou: ").strip()
        if not user_input or user_input.lower() == "quit":
            break

        messages.append({"role": "user", "content": user_input})

        print("\nAgent: ", end="", flush=True)
        result = Runner.run_streamed(starting_agent=agent, input=messages, max_turns=30)
        async for event in result.stream_events():
            if isinstance(event, RawResponsesStreamEvent) and isinstance(event.data, ResponseTextDeltaEvent):
                print(event.data.delta, end="", flush=True)
        print()

        messages.append({"role": "assistant", "content": result.final_output})

asyncio.run(main())
# endregion chat

```
# Running the agent

```bash
uv run --env-file .env python main.py
```
The agent starts an interactive chat. Type a message and watch the response stream in. Type `quit` to exit.

```
Support Knowledge Agent (type 'quit' to exit)
--------------------------------------------

You: The payments service is returning 500 errors. Can you check what's going on?

Agent: I checked Datadog and found an active alert on the payments-api monitor...
```

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
# Gmail Labeler (/cookbooks/gmail-labeler)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/gmail-labeler)

This cookbook builds a Python script that connects to Gmail, listens for new messages using Composio triggers, and uses Claude to label each email automatically. The agent is scoped to Gmail tools only using a **scoped session**.

# Prerequisites

* Python 3.10+
* [UV](https://docs.astral.sh/uv/getting-started/installation/)
* [Composio API key](https://platform.composio.dev/settings)
* [Anthropic API key](https://console.anthropic.com/settings/keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-gmail-labeler && cd composio-gmail-labeler
uv init && uv add composio composio-claude-agent-sdk claude-agent-sdk
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
ANTHROPIC_API_KEY=your_anthropic_api_key
```
# Setting up the client

`Composio` takes a `ClaudeAgentSDKProvider` so that tools come back in the format the Claude Agent SDK expects.

```py
import asyncio

from composio import Composio
from composio_claude_agent_sdk import ClaudeAgentSDKProvider
from claude_agent_sdk import query, ClaudeAgentOptions, create_sdk_mcp_server

composio = Composio(provider=ClaudeAgentSDKProvider())
```
# Connecting to Gmail

Before labeling emails, the user needs to connect their Gmail account. The `connect` function creates a scoped session with `toolkits=["gmail"]` and checks the connection status with `session.toolkits()`. If Gmail is not connected, `session.authorize("gmail")` starts the OAuth flow and returns a URL for the user to visit. `wait_for_connection()` blocks until they complete it.

```py
def connect(user_id: str):
    """Check if Gmail is connected. If not, start OAuth and wait."""
    session = composio.create(user_id=user_id, toolkits=["gmail"])
    toolkits = session.toolkits()

    for t in toolkits.items:
        if t.slug == "gmail" and t.connection and t.connection.is_active:
            print("Gmail is already connected.")
            return

    connection_request = session.authorize("gmail")
    print(f"Open this URL to connect Gmail:\n{connection_request.redirect_url}")
    connection_request.wait_for_connection()
    print("Connected.")
```
# Labeling with Claude

For each incoming email, `label_email` fetches Gmail tools from `session.tools()`, wraps them in an MCP server using `create_sdk_mcp_server()`, and passes them to a Claude agent. The agent lists existing labels, picks the best fit or creates a new one, and applies it to the email.

```py
async def label_email(session, message_id: str, subject: str, body: str):
    """Use Claude to label an incoming email."""
    tools = session.tools()
    tool_server = create_sdk_mcp_server(name="composio", version="1.0.0", tools=tools)

    prompt = f"""You received a new email. Analyze it and apply an appropriate label.

Message ID: {message_id}
Subject: {subject}
Body: {body}

Steps:
1. List the existing Gmail labels.
2. Decide which label fits best, or create a new label if none fit.
3. Apply the label to this email using its message ID."""

    options = ClaudeAgentOptions(
        system_prompt="You are an email organizer. Label incoming emails with the most appropriate Gmail label.",
        permission_mode="bypassPermissions",
        mcp_servers={"composio": tool_server},
    )

    async for message in query(prompt=prompt, options=options):
        print(message)
```
# Listening for emails

The `listen` function creates a trigger for new Gmail messages and subscribes to events over WebSocket. When a new email arrives, the handler calls `label_email` to classify and label it.

```py
def listen(user_id: str):
    """Create a trigger, subscribe to events, and label incoming emails."""
    session = composio.create(user_id=user_id, toolkits=["gmail"])

    trigger = composio.triggers.create(
        slug="GMAIL_NEW_GMAIL_MESSAGE",
        user_id=user_id,
        trigger_config={},
    )
    print(f"Trigger created: {trigger.trigger_id}")

    loop = asyncio.new_event_loop()
    subscription = composio.triggers.subscribe()

    @subscription.handle(trigger_id=trigger.trigger_id)
    def handle_event(data):
        payload = data.get("payload", {})
        print(f"New email: {payload.get('subject', 'No subject')}")
        try:
            loop.run_until_complete(
                label_email(
                    session,
                    message_id=payload.get("id", ""),
                    subject=payload.get("subject", ""),
                    body=payload.get("message_text", ""),
                )
            )
        except Exception as e:
            print(f"Error labeling email: {e}")

    print("Listening for new emails...")
    subscription.wait_forever()
```
> SDK subscriptions are ideal for local development and testing. For production, use [webhooks](/docs/setting-up-triggers/subscribing-to-events) to receive trigger events at a URL endpoint.

# Complete script

Here is everything together:

```py
import sys

# region setup
import asyncio

from composio import Composio
from composio_claude_agent_sdk import ClaudeAgentSDKProvider
from claude_agent_sdk import query, ClaudeAgentOptions, create_sdk_mcp_server

composio = Composio(provider=ClaudeAgentSDKProvider())
# endregion setup

# region connect
def connect(user_id: str):
    """Check if Gmail is connected. If not, start OAuth and wait."""
    session = composio.create(user_id=user_id, toolkits=["gmail"])
    toolkits = session.toolkits()

    for t in toolkits.items:
        if t.slug == "gmail" and t.connection and t.connection.is_active:
            print("Gmail is already connected.")
            return

    connection_request = session.authorize("gmail")
    print(f"Open this URL to connect Gmail:\n{connection_request.redirect_url}")
    connection_request.wait_for_connection()
    print("Connected.")
# endregion connect

# region label
async def label_email(session, message_id: str, subject: str, body: str):
    """Use Claude to label an incoming email."""
    tools = session.tools()
    tool_server = create_sdk_mcp_server(name="composio", version="1.0.0", tools=tools)

    prompt = f"""You received a new email. Analyze it and apply an appropriate label.

Message ID: {message_id}
Subject: {subject}
Body: {body}

Steps:
1. List the existing Gmail labels.
2. Decide which label fits best, or create a new label if none fit.
3. Apply the label to this email using its message ID."""

    options = ClaudeAgentOptions(
        system_prompt="You are an email organizer. Label incoming emails with the most appropriate Gmail label.",
        permission_mode="bypassPermissions",
        mcp_servers={"composio": tool_server},
    )

    async for message in query(prompt=prompt, options=options):
        print(message)
# endregion label

# region listen
def listen(user_id: str):
    """Create a trigger, subscribe to events, and label incoming emails."""
    session = composio.create(user_id=user_id, toolkits=["gmail"])

    trigger = composio.triggers.create(
        slug="GMAIL_NEW_GMAIL_MESSAGE",
        user_id=user_id,
        trigger_config={},
    )
    print(f"Trigger created: {trigger.trigger_id}")

    loop = asyncio.new_event_loop()
    subscription = composio.triggers.subscribe()

    @subscription.handle(trigger_id=trigger.trigger_id)
    def handle_event(data):
        payload = data.get("payload", {})
        print(f"New email: {payload.get('subject', 'No subject')}")
        try:
            loop.run_until_complete(
                label_email(
                    session,
                    message_id=payload.get("id", ""),
                    subject=payload.get("subject", ""),
                    body=payload.get("message_text", ""),
                )
            )
        except Exception as e:
            print(f"Error labeling email: {e}")

    print("Listening for new emails...")
    subscription.wait_forever()
# endregion listen

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python main.py connect <user_id>")
        print("  python main.py listen <user_id>")
        sys.exit(1)

    command = sys.argv[1]

    if command == "connect":
        uid = sys.argv[2] if len(sys.argv) > 2 else "default"
        connect(uid)
    elif command == "listen":
        uid = sys.argv[2] if len(sys.argv) > 2 else "default"
        listen(uid)
    else:
        print(f"Unknown command: {command}")
        print("Use 'connect' or 'listen'.")
        sys.exit(1)

```
# Running the script

First, connect your Gmail account:

```bash
uv run --env-file .env python main.py connect default
```
If Gmail is not connected yet, you will get an OAuth URL. Open it in your browser and authorize the app. If already connected, the script prints "Gmail is already connected."

Then start the listener:

```bash
uv run --env-file .env python main.py listen default
```

Send yourself an email and watch the terminal. The agent will receive the event, inspect the email, and apply a label.

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
# Slack Summarizer (/cookbooks/slack-summariser)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/slack-summariser)

This cookbook builds a standalone Python script that connects to a Slack workspace, fetches recent messages from a channel, and summarizes the key topics, decisions, and action items. We'll use **scoped sessions** to limit the agent to Slack tools only.

# Prerequisites

* Python 3.10+
* [UV](https://docs.astral.sh/uv/getting-started/installation/)
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-slack-summarizer && cd composio-slack-summarizer
uv init && uv add composio composio-openai-agents openai-agents
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
# Setting up the client

`Composio` takes an `OpenAIAgentsProvider` so that tools come back in the format the OpenAI Agents SDK expects.

```py
from agents import Agent, Runner
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider

composio = Composio(provider=OpenAIAgentsProvider())
```
# Connecting to Slack

Before summarizing, we need to make sure the user has connected their Slack workspace. The `connect` function creates a scoped session with `toolkits=["slack"]` and checks the connection status via `session.toolkits()`. If Slack is not connected, `session.authorize("slack")` starts the OAuth flow and returns a URL for the user to visit. `wait_for_connection()` blocks until they complete it.

```py
def connect(user_id: str):
    """Check if Slack is connected. If not, start OAuth and wait."""
    session = composio.create(user_id=user_id, toolkits=["slack"])
    toolkits = session.toolkits()

    for t in toolkits.items:
        if t.slug == "slack" and t.connection and t.connection.is_active:
            print("Slack is already connected.")
            return

    connection_request = session.authorize("slack")
    print(f"Open this URL to connect Slack:\n{connection_request.redirect_url}")
    connection_request.wait_for_connection()
    print("Connected.")
```
# Summarizing a channel

With Slack connected, the `summarize` function creates a session and grabs the tools. We hand them to an agent with focused summarization instructions. `Runner.run_sync` handles the agentic loop: the agent calls the Slack tool to fetch messages and produces a summary.

```py
def summarize(user_id: str, channel: str):
    """Fetch recent messages from a Slack channel and summarize them."""
    session = composio.create(user_id=user_id, toolkits=["slack"])
    tools = session.tools()

    agent = Agent(
        name="Slack Summarizer",
        instructions=(
            "You summarize Slack channel messages. "
            "Fetch recent messages from the given channel and provide a concise summary "
            "of the key topics, decisions, and action items."
        ),
        tools=tools,
    )

    result = Runner.run_sync(
        starting_agent=agent,
        input=f"Summarize the recent messages from the #{channel} channel.",
    )
    print(result.final_output)
```
# Complete script

Here is everything together:

```py
import sys

# region setup
from agents import Agent, Runner
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider

composio = Composio(provider=OpenAIAgentsProvider())
# endregion setup

# region connect
def connect(user_id: str):
    """Check if Slack is connected. If not, start OAuth and wait."""
    session = composio.create(user_id=user_id, toolkits=["slack"])
    toolkits = session.toolkits()

    for t in toolkits.items:
        if t.slug == "slack" and t.connection and t.connection.is_active:
            print("Slack is already connected.")
            return

    connection_request = session.authorize("slack")
    print(f"Open this URL to connect Slack:\n{connection_request.redirect_url}")
    connection_request.wait_for_connection()
    print("Connected.")
# endregion connect

# region summarize
def summarize(user_id: str, channel: str):
    """Fetch recent messages from a Slack channel and summarize them."""
    session = composio.create(user_id=user_id, toolkits=["slack"])
    tools = session.tools()

    agent = Agent(
        name="Slack Summarizer",
        instructions=(
            "You summarize Slack channel messages. "
            "Fetch recent messages from the given channel and provide a concise summary "
            "of the key topics, decisions, and action items."
        ),
        tools=tools,
    )

    result = Runner.run_sync(
        starting_agent=agent,
        input=f"Summarize the recent messages from the #{channel} channel.",
    )
    print(result.final_output)
# endregion summarize

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python main.py connect <user_id>")
        print("  python main.py summarize <user_id> <channel>")
        sys.exit(1)

    command = sys.argv[1]

    if command == "connect":
        user_id = sys.argv[2] if len(sys.argv) > 2 else "default"
        connect(user_id)
    elif command == "summarize":
        user_id = sys.argv[2] if len(sys.argv) > 2 else "default"
        channel = sys.argv[3] if len(sys.argv) > 3 else "general"
        summarize(user_id, channel)
    else:
        print(f"Unknown command: {command}")
        print("Use 'connect' or 'summarize'.")
        sys.exit(1)

```
# Running the script

First, connect your Slack workspace:

```bash
uv run --env-file .env python main.py connect default
```
If Slack is not connected yet, you'll get an OAuth URL. Open it in your browser and authorize the app. If you've already connected, the script will print "Slack is already connected."

Then run the summarizer:

```bash
uv run --env-file .env python main.py summarize default general
```

Replace `general` with any channel name you want to summarize.

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
# Supabase SQL Agent (/cookbooks/supabase-sql-agent)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/supabase-sql-agent)

This cookbook builds a CLI agent that connects to your Supabase account and lets you query your databases using natural language. The agent translates plain English into SQL, runs it against your project, and explains the results. We'll use **scoped sessions** to limit the agent to Supabase tools only.

# Prerequisites

* Python 3.10+
* [UV](https://docs.astral.sh/uv/getting-started/installation/)
* [Composio API key](https://platform.composio.dev/settings)
* [OpenAI API key](https://platform.openai.com/api-keys)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-supabase-agent && cd composio-supabase-agent
uv init && uv add composio composio-openai-agents openai-agents
```

Add your API keys to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
OPENAI_API_KEY=your_openai_api_key
```
# Setting up the client

`Composio` takes an `OpenAIAgentsProvider` so that tools come back in the format the OpenAI Agents SDK expects.

```py
from agents import Agent, Runner
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider

composio = Composio(provider=OpenAIAgentsProvider())
```
# Connecting to Supabase

Before running queries, the user needs to connect their Supabase account. The `connect` function creates a scoped session with `toolkits=["supabase"]` and checks the connection status via `session.toolkits()`. If Supabase is not connected, `session.authorize("supabase")` starts the OAuth flow and returns a URL for the user to visit. `wait_for_connection()` blocks until they complete it.

```py
def connect(user_id: str):
    """Check if Supabase is connected. If not, start OAuth and wait."""
    session = composio.create(user_id=user_id, toolkits=["supabase"])
    toolkits = session.toolkits()

    for t in toolkits.items:
        if t.slug == "supabase" and t.connection and t.connection.is_active:
            print("Supabase is already connected.")
            return

    connection_request = session.authorize("supabase")
    print(f"Open this URL to connect Supabase:\n{connection_request.redirect_url}")
    connection_request.wait_for_connection()
    print("Connected.")
```
# Querying with natural language

With Supabase connected, the `query` function creates a session scoped to `toolkits=["supabase"]` and grabs the tools. We hand them to an agent and run an interactive loop where the user types questions and the agent translates them into SQL.

```py
def query(user_id: str):
    """Interactive loop: ask questions in natural language, get SQL results."""
    session = composio.create(user_id=user_id, toolkits=["supabase"])
    tools = session.tools()

    agent = Agent(
        name="Supabase SQL Agent",
        instructions=(
            "You are a Supabase SQL assistant. You help users query their Supabase "
            "databases using natural language. First list the user's projects to find "
            "the right database, then translate their questions into SQL queries and "
            "run them. Always explain the results clearly."
        ),
        tools=tools,
    )

    print("Supabase SQL Agent (type 'exit' to quit)")
    print("-" * 45)

    last_response_id = None
    while True:
        user_input = input("you > ")
        if user_input.strip().lower() == "exit":
            break

        result = Runner.run_sync(
            starting_agent=agent,
            input=user_input,
            previous_response_id=last_response_id,
        )
        last_response_id = result.last_response_id
        print(f"agent > {result.final_output}\n")
```
# Complete script

Here is everything together:

```py
import sys

# region setup
from agents import Agent, Runner
from composio import Composio
from composio_openai_agents import OpenAIAgentsProvider

composio = Composio(provider=OpenAIAgentsProvider())
# endregion setup

# region connect
def connect(user_id: str):
    """Check if Supabase is connected. If not, start OAuth and wait."""
    session = composio.create(user_id=user_id, toolkits=["supabase"])
    toolkits = session.toolkits()

    for t in toolkits.items:
        if t.slug == "supabase" and t.connection and t.connection.is_active:
            print("Supabase is already connected.")
            return

    connection_request = session.authorize("supabase")
    print(f"Open this URL to connect Supabase:\n{connection_request.redirect_url}")
    connection_request.wait_for_connection()
    print("Connected.")
# endregion connect

# region query
def query(user_id: str):
    """Interactive loop: ask questions in natural language, get SQL results."""
    session = composio.create(user_id=user_id, toolkits=["supabase"])
    tools = session.tools()

    agent = Agent(
        name="Supabase SQL Agent",
        instructions=(
            "You are a Supabase SQL assistant. You help users query their Supabase "
            "databases using natural language. First list the user's projects to find "
            "the right database, then translate their questions into SQL queries and "
            "run them. Always explain the results clearly."
        ),
        tools=tools,
    )

    print("Supabase SQL Agent (type 'exit' to quit)")
    print("-" * 45)

    last_response_id = None
    while True:
        user_input = input("you > ")
        if user_input.strip().lower() == "exit":
            break

        result = Runner.run_sync(
            starting_agent=agent,
            input=user_input,
            previous_response_id=last_response_id,
        )
        last_response_id = result.last_response_id
        print(f"agent > {result.final_output}\n")
# endregion query

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python main.py connect <user_id>")
        print("  python main.py query <user_id>")
        sys.exit(1)

    command = sys.argv[1]

    if command == "connect":
        user_id = sys.argv[2] if len(sys.argv) > 2 else "default"
        connect(user_id)
    elif command == "query":
        user_id = sys.argv[2] if len(sys.argv) > 2 else "default"
        query(user_id)
    else:
        print(f"Unknown command: {command}")
        print("Use 'connect' or 'query'.")
        sys.exit(1)

```
# Running the script

First, connect your Supabase account:

```bash
uv run --env-file .env python main.py connect default
```
If Supabase is not connected yet, you'll get an OAuth URL. Open it in your browser and authorize the app.

Then start the interactive query agent:

```bash
uv run --env-file .env python main.py query default
```
Try asking things like:

```
you > List all my Supabase projects
you > Show me all tables in my default project
you > How many users signed up this week?
you > What are the top 10 most recent orders?
```

Type `exit` to quit.

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
# Tool Type Generator (/cookbooks/tool-generator)

[View source on GitHub](https://github.com/ComposioHQ/composio/tree/next/docs/examples/tool-generator)

This cookbook shows how to access the raw tool definitions and toolkit metadata that Composio exposes. If you are building a platform on top of Composio, say a workflow builder or an IDE plugin, you will need to display tool schemas, auth requirements, and input/output parameters to your users. This is how you get that data.

# Prerequisites

* Python 3.10+
* [UV](https://docs.astral.sh/uv/getting-started/installation/)
* [Composio API key](https://platform.composio.dev/settings)

# Project setup

Create a new project and install dependencies:

```bash
mkdir composio-tool-generator && cd composio-tool-generator
uv init && uv add composio
```

Add your API key to a `.env` file:

```bash title=".env"
COMPOSIO_API_KEY=your_composio_api_key
```
# Setting up the client

No provider is needed here. We are reading tool metadata directly from the Composio API, not passing tools to an AI framework.

```py
from composio import Composio

composio = Composio()
```
# Raw tool definitions

Composio has two internal representations for tools: **raw definitions** and **provider definitions**. Raw definitions are framework-agnostic JSON schemas with `input_parameters`, `output_parameters`, scopes, and tags. Provider definitions translate these into the format a specific framework expects (OpenAI, Anthropic, etc.).

For building platform UIs, the raw definitions are what you want. `composio.tools.get_raw_composio_tools()` returns them for an entire toolkit. Each tool has a standard JSON Schema for its inputs and outputs, so you can render forms, validate user input, or generate types from it.

```py
def list_tools(toolkit_slug: str):
    """Fetch raw tool definitions for a toolkit and print them."""
    tools = composio.tools.get_raw_composio_tools(toolkits=[toolkit_slug])

    for tool in tools:
        print(f"\n--- {tool.slug} ---")
        print(f"Name: {tool.name}")
        print(f"Description: {tool.description}")

        params = tool.input_parameters or {}
        required = params.get("required", [])
        for name, schema in params.get("properties", {}).items():
            tag = "required" if name in required else "optional"
            print(f"  [{tag}] {name}: {schema.get('type', 'any')} - {schema.get('description', '')}")
```
# Toolkit metadata

A toolkit groups related tools under one integration. `composio.toolkits.get()` returns metadata about the toolkit itself, including its supported auth schemes. Each auth scheme lists the fields required for two stages: creating an auth config (platform setup) and initiating a connected account (end-user connection). This is useful if you need to build your own auth configuration UI.

```py
def toolkit_info(toolkit_slug: str):
    """Fetch toolkit metadata including auth schemes."""
    toolkit = composio.toolkits.get(toolkit_slug)

    print(f"Name: {toolkit.name}")
    print(f"Slug: {toolkit.slug}")

    if not toolkit.auth_config_details:
        print("Auth: none (no auth required)")
        return

    for scheme in toolkit.auth_config_details:
        print(f"\nAuth scheme: {scheme.mode}")

        if not scheme.fields:
            continue

        creation = scheme.fields.auth_config_creation
        if creation:
            print("  Auth config creation:")
            for field in creation.required or []:
                print(f"    [required] {field.name}: {field.description}")
            for field in creation.optional or []:
                print(f"    [optional] {field.name}: {field.description}")

        connection = scheme.fields.connected_account_initiation
        if connection:
            print("  Account connection:")
            for field in connection.required or []:
                print(f"    [required] {field.name}: {field.description}")
            for field in connection.optional or []:
                print(f"    [optional] {field.name}: {field.description}")
```
# Exporting to JSON

For code generation or static analysis, you can export the full raw schemas to a JSON file. Each entry includes the `input_parameters` and `output_parameters` as standard JSON Schema objects, which you can feed into tools like `json-schema-to-typescript` or `datamodel-code-generator` to produce typed interfaces.

```py
def export(toolkit_slug: str, output_file: str):
    """Export raw tool definitions to a JSON file."""
    tools = composio.tools.get_raw_composio_tools(toolkits=[toolkit_slug])

    data = []
    for tool in tools:
        data.append({
            "slug": tool.slug,
            "name": tool.name,
            "description": tool.description,
            "input_parameters": tool.input_parameters or {},
            "output_parameters": tool.output_parameters or {},
            "scopes": getattr(tool, "scopes", []),
            "tags": getattr(tool, "tags", []),
            "no_auth": getattr(tool, "no_auth", False),
        })

    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, default=str)

    print(f"Exported {len(data)} tools to {output_file}")
```
# Complete script

Here is everything together:

```py
import json
import sys

# region setup
from composio import Composio

composio = Composio()
# endregion setup

# region list-tools
def list_tools(toolkit_slug: str):
    """Fetch raw tool definitions for a toolkit and print them."""
    tools = composio.tools.get_raw_composio_tools(toolkits=[toolkit_slug])

    for tool in tools:
        print(f"\n--- {tool.slug} ---")
        print(f"Name: {tool.name}")
        print(f"Description: {tool.description}")

        params = tool.input_parameters or {}
        required = params.get("required", [])
        for name, schema in params.get("properties", {}).items():
            tag = "required" if name in required else "optional"
            print(f"  [{tag}] {name}: {schema.get('type', 'any')} - {schema.get('description', '')}")
# endregion list-tools

# region toolkit-info
def toolkit_info(toolkit_slug: str):
    """Fetch toolkit metadata including auth schemes."""
    toolkit = composio.toolkits.get(toolkit_slug)

    print(f"Name: {toolkit.name}")
    print(f"Slug: {toolkit.slug}")

    if not toolkit.auth_config_details:
        print("Auth: none (no auth required)")
        return

    for scheme in toolkit.auth_config_details:
        print(f"\nAuth scheme: {scheme.mode}")

        if not scheme.fields:
            continue

        creation = scheme.fields.auth_config_creation
        if creation:
            print("  Auth config creation:")
            for field in creation.required or []:
                print(f"    [required] {field.name}: {field.description}")
            for field in creation.optional or []:
                print(f"    [optional] {field.name}: {field.description}")

        connection = scheme.fields.connected_account_initiation
        if connection:
            print("  Account connection:")
            for field in connection.required or []:
                print(f"    [required] {field.name}: {field.description}")
            for field in connection.optional or []:
                print(f"    [optional] {field.name}: {field.description}")
# endregion toolkit-info

# region export
def export(toolkit_slug: str, output_file: str):
    """Export raw tool definitions to a JSON file."""
    tools = composio.tools.get_raw_composio_tools(toolkits=[toolkit_slug])

    data = []
    for tool in tools:
        data.append({
            "slug": tool.slug,
            "name": tool.name,
            "description": tool.description,
            "input_parameters": tool.input_parameters or {},
            "output_parameters": tool.output_parameters or {},
            "scopes": getattr(tool, "scopes", []),
            "tags": getattr(tool, "tags", []),
            "no_auth": getattr(tool, "no_auth", False),
        })

    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, default=str)

    print(f"Exported {len(data)} tools to {output_file}")
# endregion export

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python main.py list-tools <toolkit>")
        print("  python main.py toolkit-info <toolkit>")
        print("  python main.py export <toolkit> <output.json>")
        sys.exit(1)

    command = sys.argv[1]

    if command == "list-tools":
        slug = sys.argv[2] if len(sys.argv) > 2 else "gmail"
        list_tools(slug)
    elif command == "toolkit-info":
        slug = sys.argv[2] if len(sys.argv) > 2 else "gmail"
        toolkit_info(slug)
    elif command == "export":
        slug = sys.argv[2] if len(sys.argv) > 2 else "gmail"
        out = sys.argv[3] if len(sys.argv) > 3 else "output.json"
        export(slug, out)
    else:
        print(f"Unknown command: {command}")
        print("Use 'list-tools', 'toolkit-info', or 'export'.")
        sys.exit(1)

```
# Running the script

List all tools in a toolkit:

```bash
uv run --env-file .env python main.py list-tools gmail
```
Inspect a toolkit's auth schemes:

```bash
uv run --env-file .env python main.py toolkit-info gmail
```
Export raw tool schemas to a JSON file:

```bash
uv run --env-file .env python main.py export gmail tools.json
```

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
