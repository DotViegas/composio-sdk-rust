# COMPOSIO Client Audit Report (Incremental)

## Audit Scope & Progress

This report is being produced incrementally, resource-by-resource, as requested.

- Total Python resources discovered under `temp/composio_client/resources`: 13 logical resources (`auth_configs`, `cli`, `connected_accounts`, `files`, `link`, `mcp`, `migration`, `project`, `tool_router`, `toolkits`, `tools`, `trigger_instances`, `triggers_types`).
- Completed in this increment: **11/13** (`tools`, `toolkits`, `triggers_types`, `trigger_instances`, `connected_accounts`, `auth_configs`, `files`, `link`, `tool_router`, `mcp`, `migration`).
- Pending: 2/13.

## Resource 1: `tools` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/tools.py`
  - `temp/composio_client/types/tool_list_params.py`
  - `temp/composio_client/types/tool_execute_params.py`
  - `temp/composio_client/types/tool_get_input_params.py`
  - `temp/composio_client/types/tool_proxy_params.py`
- Rust:
  - `src/client.rs`
  - `src/models/tools.rs`
  - `src/models/modifiers.rs`

### Python `tools` surface (baseline)

The Python resource exposes the following operations:

1. `retrieve(tool_slug, toolkit_versions?, version?)`
2. `list(auth_config_ids?, cursor?, important?, include_deprecated?, limit?, scopes?, search?, tags?, tool_slugs?, toolkit_slug?, toolkit_versions?)`
3. `execute(tool_slug, allow_tracing?, arguments?, connected_account_id?, custom_auth_params?, custom_connection_data?, entity_id?, text?, user_id?, version?)`
4. `get_input(tool_slug, text, custom_description?, system_prompt?, version?)`
5. `proxy(endpoint, method, binary_body?, body?, connected_account_id?, custom_connection_data?, parameters?)`
6. `retrieve_enum()`

### Rust `tools` surface (current)

The Rust SDK exposes equivalent methods via `ComposioClient`:

1. `get_tool(slug)`
2. `list_tools(ToolListParams)`
3. `execute_tool(ToolExecuteParams)`
4. `generate_tool_inputs(ToolInputGenerationParams)`
5. `proxy_tool(ToolProxyParams)`

No `retrieve_enum` equivalent was found in Rust.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Retrieve tool by slug | `retrieve` | `get_tool` | FULL | Endpoint-aligned (`/api/v3/tools/{slug}`). |
| List tools | `list` | `list_tools` | PARTIAL | Core support exists, but query-field naming/coverage differs (details below). |
| Execute tool | `execute` | `execute_tool` | PARTIAL | Core support exists; Rust adds local version guard behavior. |
| Generate tool input from text | `get_input` | `generate_tool_inputs` | PARTIAL | Endpoint exists, but body field names differ. |
| Proxy request execution | `proxy` | `proxy_tool` | PARTIAL | Core support exists; Python supports `binary_body`, Rust currently does not. |
| Retrieve enum of tool slugs | `retrieve_enum` | _missing_ | MISSING_IN_RUST | No Rust method for `/api/v3/tools/enum`. |

### Detailed findings for `tools`

#### 1) `list` parameter mismatch (PARTIAL)

- Python supports `auth_config_ids`, `important`, and `include_deprecated`.
- Rust `ToolListParams` instead uses `importance` and `show_deprecated`, and has no `auth_config_ids` field.
- This introduces contract drift risk when matching server-side query semantics.

#### 2) `get_input` payload key mismatch (PARTIAL)

- Python request model uses `custom_description` and `system_prompt`.
- Rust `ToolInputGenerationParams` uses `custom_tool_description` and `custom_system_prompt` and serializes these keys accordingly.
- If backend expects Python/OpenAPI keys, Rust payload keys may not be interpreted as intended.

#### 3) `proxy` missing `binary_body` in Rust (PARTIAL)

- Python `ToolProxyParams` supports `binary_body` for URL/base64 binary uploads.
- Rust `ToolProxyParams` currently has no `binary_body` field; only `body`, `parameters`, and connection metadata.

#### 4) `execute` behavior divergence: version handling (PARTIAL)

- Python passes through provided fields directly.
- Rust applies additional local behavior:
  - auto-resolves version using toolkit config when absent,
  - blocks version=`latest` unless `dangerously_skip_version_check=true`.
- This is a deliberate safety policy but diverges from Python transport behavior.

### Recommended remediation order for `tools`

1. Add Rust support for `/api/v3/tools/enum` (`retrieve_enum` parity).
2. Align `list_tools` query model with OpenAPI/Python names (or support aliases):
   - add `auth_config_ids`, `important`, `include_deprecated`.
3. Align `generate_tool_inputs` body keys with OpenAPI (`custom_description`, `system_prompt`) while preserving backwards-compatible aliases if needed.
4. Add `binary_body` to Rust `ToolProxyParams` and request serialization.
5. Decide and document whether Rust's extra version guard in `execute_tool` should remain as a Rust-specific safety enhancement or be made opt-in for closer parity.

## Resource 2: `toolkits` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/toolkits.py`
  - `temp/composio_client/types/toolkit_list_params.py`
  - `temp/composio_client/types/toolkit_retrieve_params.py`
  - `temp/composio_client/types/toolkit_list_response.py`
  - `temp/composio_client/types/toolkit_retrieve_response.py`
  - `temp/composio_client/types/toolkit_retrieve_categories_response.py`
- Rust:
  - `src/client.rs`
  - `src/models/toolkits.rs`

### Python `toolkits` surface (baseline)

The Python resource exposes:

1. `retrieve(slug, version?)`
2. `list(category?, cursor?, include_deprecated?, limit?, managed_by?, search?, sort_by?)`
3. `retrieve_categories()`

### Rust `toolkits` surface (current)

The Rust SDK exposes:

1. `get_toolkit(slug)`
2. `list_toolkits(ToolkitListParams)`
3. `list_toolkit_categories()`

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Retrieve toolkit by slug | `retrieve` | `get_toolkit` | PARTIAL | Same endpoint family, but Rust has no `version` query parameter. |
| List toolkits | `list` | `list_toolkits` | PARTIAL | Core filters align, but deprecation flag naming differs (`include_deprecated` vs `show_deprecated`). |
| Retrieve toolkit categories | `retrieve_categories` | `list_toolkit_categories` | PARTIAL | Endpoint aligns, but Rust category/response model fields differ from Python/OpenAPI schema. |

### Detailed findings for `toolkits`

#### 1) Missing `version` support on toolkit retrieval (PARTIAL)

- Python supports `retrieve(slug, version=...)` and serializes this query through `ToolkitRetrieveParams`.
- Rust `get_toolkit(slug)` does not expose a `version` argument and always calls `/api/v3/toolkits/{slug}` without query parameters.

#### 2) List query naming drift for deprecated filter (PARTIAL)

- Python/OpenAPI uses `include_deprecated`.
- Rust `ToolkitListParams`/`list_toolkits` uses `show_deprecated` and serializes `show_deprecated=...`.
- This can lead to behavioral drift if the backend only honors `include_deprecated`.

#### 3) Categories response model divergence (PARTIAL)

- Python category items are `{id, name}` and response includes pagination fields (`current_page`, `total_items`, `total_pages`, `next_cursor`, `items`).
- Rust `ToolkitCategoriesResponse` contains only `items`, and `ToolkitCategory` models `{name, display_name?, count?}` (no `id`).
- This limits schema parity and may hide useful pagination/category identifiers.

#### 4) Toolkit retrieve/list response field coverage differences (PARTIAL)

- Python models include additional fields such as `status`, `enabled`, `deprecated.toolkitId`, and richer nested metadata/auth detail structures.
- Rust models cover core fields but omit several of these advanced/legacy fields.
- Deserialization may still succeed for current usage, but surface parity is not complete.

### Recommended remediation order for `toolkits`

1. Add optional `version` support to Rust `get_toolkit`.
2. Align deprecated-filter query naming to `include_deprecated` (or support both names with clear precedence).
3. Expand Rust `ToolkitCategoriesResponse` / `ToolkitCategory` to include `id` and pagination metadata.
4. Review `ToolkitRetrieveResponse`/`ToolkitItem` coverage for missing high-value fields (`status`, `enabled`, etc.) and add non-breaking optional fields.

## Resource 3: `triggers_types` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/triggers_types.py`
  - `temp/composio_client/types/triggers_type_list_params.py`
  - `temp/composio_client/types/triggers_type_retrieve_params.py`
  - `temp/composio_client/types/triggers_type_list_response.py`
  - `temp/composio_client/types/triggers_type_retrieve_response.py`
  - `temp/composio_client/types/triggers_type_retrieve_enum_response.py`
- Rust:
  - `src/client.rs`
  - `src/models/triggers.rs`

### Python `triggers_types` surface (baseline)

The Python resource exposes:

1. `retrieve(slug, toolkit_versions?)`
2. `list(cursor?, limit?, toolkit_slugs?, toolkit_versions?)`
3. `retrieve_enum()`

### Rust `triggers_types` surface (current)

The Rust SDK exposes:

1. `get_trigger_type(slug)`
2. `list_trigger_types(TriggerTypeListParams)`

No dedicated enum retrieval method for trigger type slugs was found in Rust.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Retrieve trigger type by slug | `retrieve` | `get_trigger_type` | PARTIAL | Endpoint family aligns, but Rust does not expose `toolkit_versions` query input. |
| List trigger types | `list` | `list_trigger_types` | FULL | Query/filter surface and endpoint shape are aligned (`cursor`, `limit`, `toolkit_slugs`, `toolkit_versions`). |
| Retrieve trigger-type enum list | `retrieve_enum` | _missing_ | MISSING_IN_RUST | No Rust method for `/api/v3/triggers_types/list/enum`. |

### Detailed findings for `triggers_types`

#### 1) Missing `retrieve_enum` parity endpoint (MISSING_IN_RUST)

- Python exposes `retrieve_enum()` mapped to `GET /api/v3/triggers_types/list/enum`.
- Rust has no equivalent client method to fetch this enum list.

#### 2) Missing `toolkit_versions` support for single trigger retrieval (PARTIAL)

- Python `retrieve` accepts `toolkit_versions` and passes it as query.
- Rust `get_trigger_type(slug)` does not accept toolkit version resolution inputs and always calls `/api/v3/triggers_types/{slug}` without query params.

#### 3) Trigger type response model coverage drift (PARTIAL)

- Python retrieve/list models include a required `status` field and strongly-typed trigger mechanism (`Literal["webhook", "poll"]`).
- Rust models keep `trigger_type` as free-form `String` and do not model `status` explicitly in `TriggerType`.
- This reduces strict schema parity and may miss lifecycle-state semantics surfaced by the API.

### Recommended remediation order for `triggers_types`

1. Add Rust method for `GET /api/v3/triggers_types/list/enum`.
2. Extend `get_trigger_type` to optionally accept/pass `toolkit_versions`.
3. Improve `TriggerType` model parity by adding optional `status` and considering a typed enum for trigger mechanism (`webhook`/`poll`) with safe fallback.

## Resource 4: `trigger_instances` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/trigger_instances/trigger_instances.py`
  - `temp/composio_client/resources/trigger_instances/manage.py`
  - `temp/composio_client/resources/trigger_instances/__init__.py`
  - `temp/composio_client/types/trigger_instance_list_active_params.py`
  - `temp/composio_client/types/trigger_instance_upsert_params.py`
  - `temp/composio_client/types/trigger_instance_list_active_response.py`
  - `temp/composio_client/types/trigger_instance_upsert_response.py`
  - `temp/composio_client/types/trigger_instances/manage_update_params.py`
  - `temp/composio_client/types/trigger_instances/manage_update_response.py`
  - `temp/composio_client/types/trigger_instances/manage_delete_response.py`
- Rust:
  - `src/client.rs`
  - `src/models/triggers.rs`

### Python `trigger_instances` surface (baseline)

The Python domain exposes operations across the root and nested `manage` resource:

1. `trigger_instances.list_active(...)`
2. `trigger_instances.upsert(slug, connected_account_id?, connectedAuthId?, toolkit_versions?, trigger_config?/triggerConfig?, version?)`
3. `trigger_instances.manage.update(trigger_id, status={"enable"|"disable"})`
4. `trigger_instances.manage.delete(trigger_id)`

### Rust `trigger_instances` surface (current)

The Rust SDK exposes trigger-instance lifecycle methods via `ComposioClient`:

1. `list_active_triggers(TriggerInstanceListParams)`
2. `create_trigger(TriggerCreateParams)` (maps to upsert endpoint)
3. `enable_trigger(trigger_id)`
4. `disable_trigger(trigger_id)`
5. `delete_trigger(trigger_id)`

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| List active trigger instances | `list_active` | `list_active_triggers` | PARTIAL | Core filters exist, but Python also supports `user_ids` and multiple deprecated alias query names. |
| Upsert trigger instance | `upsert` | `create_trigger` | PARTIAL | Endpoint parity exists (`/trigger_instances/{slug}/upsert`), but request contract/behavior differs. |
| Update trigger state (enable/disable) | `manage.update` | `enable_trigger` / `disable_trigger` | PARTIAL | Rust splits one generic update method into two dedicated methods and discards response payload. |
| Delete trigger instance | `manage.delete` | `delete_trigger` | PARTIAL | Endpoint parity exists, but Rust returns `()` while Python models a response body with `trigger_id`. |

### Contract mismatches

#### 1) `list_active` request contract mismatch (PARTIAL)

- Python supports `user_ids` and both canonical/deprecated aliases for several filters (`auth_config_ids` vs `authConfigIds`, `trigger_ids` vs `triggerIds`, etc.).
- Rust `TriggerInstanceListParams` supports only canonical snake_case filters and does **not** expose `user_ids`.
- This removes a server-supported filter and limits compatibility with existing alias-based integrations.

#### 2) `upsert` request contract mismatch (PARTIAL)

- Python supports both canonical and deprecated request fields (`connected_account_id` + `connectedAuthId`, `trigger_config` + `triggerConfig`, `toolkit_versions`, and deprecated `version`).
- Rust `TriggerCreateParams` supports `connected_account_id`, `user_id`, `trigger_config`, `toolkit_versions` but not deprecated aliases.
- Rust additionally models `toolkit_versions` as `Option<String>`, while Python allows `str | dict | None`; per-toolkit version maps are not representable in Rust today.

#### 3) Manage-update/delete response contract mismatch (PARTIAL)

- Python models explicit responses for state update (`{"status": "success"}`) and delete (`{"trigger_id": ...}`).
- Rust `enable_trigger`, `disable_trigger`, and `delete_trigger` all return `Result<(), ComposioError>`, discarding response bodies.
- This limits parity for callers that need operation metadata/confirmation values from the API response payload.

### Behavior mismatches

#### 1) Auto account resolution behavior in Rust `create_trigger` (PARTIAL)

- Python `upsert` behaves as a direct transport wrapper for supplied fields.
- Rust `create_trigger` adds client-side behavior: when only `user_id` is provided, it fetches trigger type, resolves toolkit, lists accounts, selects the most recent account, and injects `connected_account_id` before calling upsert.
- This convenience behavior is useful, but it is not a 1:1 behavioral match with Python and introduces extra network calls and selection semantics.

#### 2) Update API shape divergence (PARTIAL)

- Python exposes one `manage.update(trigger_id, status)` operation.
- Rust exposes two opinionated wrappers (`enable_trigger` and `disable_trigger`) instead of a single generic update call.
- Functionally equivalent for known statuses, but less extensible if server adds new state actions.

### Response/type drift

#### 1) Active-list item field drift (PARTIAL)

- Python response model includes fields such as `connected_account_uuid`, `trigger_data`, and `deprecated.createdAt` aliases.
- Rust `TriggerInstance` includes core fields but does not model several Python fields (e.g., `connected_account_uuid`, nested deprecated block).
- Rust response shape is therefore narrower than Python/OpenAPI for trigger-instance listing.

#### 2) Upsert response shape drift (PARTIAL)

- Python upsert response exposes `{ trigger_id, deprecated: { uuid } }`.
- Rust `TriggerCreateResponse` expects a richer trigger-instance object (`id`, `trigger_name`, `connected_account_id`, `trigger_config`, etc.).
- If backend follows the Python/OpenAPI shape, Rust deserialization may be brittle or semantically inconsistent.

#### 3) Management response typing drift (PARTIAL)

- Python has typed response models for manage update/delete.
- Rust management methods do not surface response types to callers.

### Recommended remediation order for `trigger_instances`

1. Add `user_ids` to Rust `TriggerInstanceListParams` and preserve canonical server query names; optionally add compatibility aliases where practical.
2. Expand Rust upsert/create request typing for `toolkit_versions` to support both string and per-toolkit map representation.
3. Reconcile upsert response model against current API contract (`trigger_id` + deprecated UUID) and ensure non-breaking deserialization.
4. Add a generic trigger-instance manage update method that accepts status action (while retaining `enable_trigger`/`disable_trigger` convenience wrappers).
5. Return typed payloads (or optional typed variants) for manage update/delete methods instead of discarding response bodies.

## Resource 5: `connected_accounts` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/connected_accounts.py`
  - `temp/composio_client/types/connected_account_list_params.py`
  - `temp/composio_client/types/connected_account_create_params.py`
  - `temp/composio_client/types/connected_account_refresh_params.py`
  - `temp/composio_client/types/connected_account_update_status_params.py`
  - `temp/composio_client/types/connected_account_list_response.py`
  - `temp/composio_client/types/connected_account_retrieve_response.py`
  - `temp/composio_client/types/connected_account_create_response.py`
  - `temp/composio_client/types/connected_account_refresh_response.py`
  - `temp/composio_client/types/connected_account_update_status_response.py`
  - `temp/composio_client/types/connected_account_delete_response.py`
- Rust:
  - `src/client.rs`
  - `src/models/connected_accounts.rs`

### Python `connected_accounts` surface (baseline)

The Python resource exposes:

1. `create(auth_config, connection, validate_credentials?)`
2. `retrieve(nanoid)`
3. `list(auth_config_ids?, connected_account_ids?, cursor?, limit?, order_by?, order_direction?, statuses?, toolkit_slugs?, user_ids?)`
4. `delete(nanoid)`
5. `refresh(nanoid, query_redirect_url?, body_redirect_url?, validate_credentials?)`
6. `update_status(nano_id, enabled)`

### Rust `connected_accounts` surface (current)

The Rust SDK exposes:

1. `list_connected_accounts(ConnectedAccountListParams)`
2. `get_connected_account(account_id)`
3. Connection-link convenience flow (`authorize_toolkit` + internal `initiate_connection`) via `POST /api/v3/connected_accounts`

No first-class Rust methods were found for `connected_accounts` delete/refresh/update-status lifecycle endpoints.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Create connected account | `create` | `initiate_connection` path via `authorize_toolkit` (indirect) | PARTIAL | Rust supports creation for link/OAuth flow use-cases, but not the full Python `create(auth_config, connection, validate_credentials?)` contract. |
| Retrieve connected account | `retrieve` | `get_connected_account` | FULL | Endpoint-aligned (`/api/v3/connected_accounts/{id}`). |
| List connected accounts | `list` | `list_connected_accounts` | PARTIAL | Core filters exist, but filter names/semantics and pagination/control surface differ. |
| Delete connected account | `delete` | _missing_ | MISSING_IN_RUST | No Rust public method for `DELETE /api/v3/connected_accounts/{id}`. |
| Refresh / reauthorize connected account | `refresh` | _missing_ | MISSING_IN_RUST | No Rust public method for `POST /api/v3/connected_accounts/{id}/refresh`. |
| Enable/disable connected account | `update_status` | _missing_ | MISSING_IN_RUST | No Rust public method for `PATCH /api/v3/connected_accounts/{id}/status`. |

### Contract mismatches

#### 1) List filters and pagination/control mismatch (PARTIAL)

- Python list supports `order_by` constrained to `created_at|updated_at`, `order_direction` constrained to `asc|desc`, and status filters as explicit literals.
- Rust exposes these as free-form `Option<String>` values and introduces `show_disabled` in request params, which is not part of Python `connected_account_list_params`.
- This weakens compile-time contract guarantees and creates request-surface drift.

#### 2) Create payload/auth-config linkage mismatch (PARTIAL)

- Python `create` contract is explicit: caller provides `auth_config` and `connection` payload blocks (plus optional credential validation).
- Rust primarily supports a convenience initiation flow (`initiate_connection`) used by `authorize_toolkit`, with a narrower request shape (`user_id`, `auth_config_id`, callback/config options).
- Result: Rust cannot directly express the full Python create contract for account-management style onboarding.

#### 3) Missing account-management endpoint contracts (MISSING_IN_RUST)

- Python defines explicit contracts for delete, refresh (including redirect-url semantics), and status transitions via `enabled`.
- Rust currently lacks public API surfaces for these endpoints.

### Behavior mismatches

#### 1) Reconnect/reauthorize behavior gap (MISSING_IN_RUST)

- Python `refresh` supports reauthentication flow restart and can return a fresh redirect URL.
- Rust has no direct refresh/reauthorize method, forcing consumers to reconstruct behavior outside SDK abstractions.

#### 2) Status/state transition control gap (MISSING_IN_RUST)

- Python offers explicit enable/disable account status transitions via `update_status(enabled)`.
- Rust has no equivalent connected-account state transition API (unlike trigger-instance state helpers).

#### 3) Account lifecycle management gap (MISSING_IN_RUST)

- Python supports soft delete of connected accounts.
- Rust provides no connected-account delete method.

### Response/type drift

#### 1) Connected account response richness drift (PARTIAL)

- Python connected-account response models (`list/retrieve/create`) include richer typed unions for connection `state`, redirect/auth-transition fields, deprecated aliases, and auth-config linkage details.
- Rust `ConnectedAccountInfo` is a narrower struct with `state: Option<serde_json::Value>` and fewer explicit fields.
- The Rust model is less expressive for provider-specific state and transition metadata.

#### 2) Pagination model drift on list (PARTIAL)

- Python list response models numeric pagination fields (`current_page`, `total_items`, `total_pages`) plus `next_cursor`.
- Rust includes these fields but typed as optional and simplified; this is generally compatible but less strict than Python schema expectations.

#### 3) Missing typed responses for lifecycle endpoints (MISSING_IN_RUST)

- Python has dedicated response models for delete/refresh/update-status.
- Rust has no equivalent endpoint methods, and therefore no typed responses for these lifecycle actions.

### Recommended remediation order for `connected_accounts`

1. Add Rust public methods for connected-account lifecycle endpoints:
   - `delete_connected_account(id)`
   - `refresh_connected_account(id, ...)`
   - `update_connected_account_status(id, enabled)`
2. Align list request contract with Python/OpenAPI strict enums for `order_by` and `order_direction`; review whether Rust-only `show_disabled` belongs in this domain.
3. Add a direct Rust `create_connected_account(...)` API that supports auth-config linkage semantics comparable to Python’s `create(auth_config, connection, ...)`.
4. Expand `ConnectedAccountInfo` and related response models with additional optional typed fields to better capture state transitions and auth metadata.
5. Add/align typed response structs for refresh/status/delete operations once endpoint methods exist.

## Resource 6: `auth_configs` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/auth_configs.py`
  - `temp/composio_client/types/auth_config_list_params.py`
  - `temp/composio_client/types/auth_config_create_params.py`
  - `temp/composio_client/types/auth_config_update_params.py`
  - `temp/composio_client/types/auth_config_list_response.py`
  - `temp/composio_client/types/auth_config_create_response.py`
  - `temp/composio_client/types/auth_config_retrieve_response.py`
- Rust:
  - `src/client.rs`
  - `src/models/auth_configs.rs`

### Python `auth_configs` surface (baseline)

The Python resource exposes:

1. `create(toolkit, auth_config?)`
2. `retrieve(nanoid)`
3. `update(nanoid, ...)` (typed variants)
4. `list(cursor?, deprecated_app_id?, deprecated_status?, is_composio_managed?, limit?, search?, show_disabled?, toolkit_slug?)`
5. `delete(nanoid)`
6. `update_status(status={"ENABLED"|"DISABLED"}, nanoid)`

### Rust `auth_configs` surface (current)

Rust currently exposes auth-config HTTP operations as **internal client helpers** (not public API):

1. `list_auth_configs(...)` (internal)
2. `create_auth_config(...)` (internal)

Additional public Rust helper methods (`get_connected_account_initiation_fields`, `get_auth_config_creation_fields`) derive field schemas from toolkit metadata, but are not direct auth-config CRUD endpoint wrappers.

No Rust public methods were found for `retrieve`, `update`, `delete`, or `update_status` auth-config endpoints.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Create auth config | `create` | `create_auth_config` (internal) | PARTIAL | Endpoint implemented in Rust, but not publicly exposed as first-class auth-config API. |
| Retrieve auth config | `retrieve` | _missing_ | MISSING_IN_RUST | No Rust method for `GET /api/v3/auth_configs/{id}`. |
| Update auth config | `update` | _missing_ | MISSING_IN_RUST | No Rust method for `PATCH /api/v3/auth_configs/{id}`. |
| List auth configs | `list` | `list_auth_configs` (internal) | PARTIAL | Core listing exists internally, but not public and filter coverage differs. |
| Delete auth config | `delete` | _missing_ | MISSING_IN_RUST | No Rust method for `DELETE /api/v3/auth_configs/{id}`. |
| Update auth config status | `update_status` | _missing_ | MISSING_IN_RUST | No Rust method for `PATCH /api/v3/auth_configs/{id}/{status}`. |

### Contract mismatches

#### 1) Public API coverage mismatch for CRUD/status endpoints (MISSING_IN_RUST)

- Python provides full lifecycle coverage (create/retrieve/update/list/delete/update_status) as public resource methods.
- Rust only has internal helpers for create/list and lacks public endpoint wrappers for retrieve/update/delete/status transition operations.

#### 2) List filtering/pagination semantics mismatch (PARTIAL)

- Python list supports deprecated filters (`deprecated_app_id`, `deprecated_status`) and `is_composio_managed` typed as `str|bool`.
- Rust list helper supports a reduced subset (`toolkit_slug`, `is_composio_managed`, `show_disabled`, `search`, `limit`, `cursor`) and omits deprecated filters.
- Rust model types are stricter/different from Python in places, which may reduce compatibility with legacy API usage.

#### 3) Update contract variant mismatch (MISSING_IN_RUST)

- Python `update` supports typed variants and rich payload fields for both custom/default modes (`credentials`, `proxy_config`, `tool_access_config`, `shared_credentials`, `is_enabled_for_tool_router`, `restrict_to_following_tools`, `scopes`).
- Rust has data types for update params in `models/auth_configs.rs`, but no client endpoint method to send them.

#### 4) Status transition endpoint contract gap (MISSING_IN_RUST)

- Python uses explicit status-in-path contract (`PATCH /api/v3/auth_configs/{nanoid}/{status}` where status is `ENABLED|DISABLED`).
- Rust has no equivalent status transition method for auth configs.

### Behavior mismatches

#### 1) Toolkit-link convenience vs direct auth-config management (PARTIAL)

- Python exposes direct auth-config management as first-class operations.
- Rust emphasizes toolkit authorization convenience (`get_or_create_auth_config` inside `authorize_toolkit`) rather than complete direct auth-config lifecycle control.
- This changes how consumers manage auth configs and hides some server capabilities.

#### 2) Enable/disable lifecycle control gap (MISSING_IN_RUST)

- Python can explicitly enable/disable auth configs via `update_status`.
- Rust cannot directly perform this transition through a public SDK method.

### Response/type drift

#### 1) Retrieve/list field coverage drift (PARTIAL)

- Python retrieve/list responses include detailed fields such as `status`, `type`, `no_of_connections`, `tool_access_config`, `is_enabled_for_tool_router`, `shared_credentials`, and deprecated compatibility blocks.
- Rust `AuthConfigInfo`/related models are narrower and simplify several structures.
- Rust field coverage is therefore incomplete relative to Python/OpenAPI response richness.

#### 2) Create response structure drift (PARTIAL)

- Python create response includes both `auth_config` and top-level `toolkit` block.
- Rust create response currently models only `auth_config`.
- This can drop useful linkage metadata unless fetched separately.

#### 3) Missing typed responses for update/delete/status endpoints (MISSING_IN_RUST)

- Python defines typed response behavior for update/delete/status flows (even when generic/object-like for some operations).
- Rust has no endpoint methods, so callers cannot access corresponding typed lifecycle responses via SDK.

### Recommended remediation order for `auth_configs`

1. Promote auth-config operations to public Rust client API with full lifecycle coverage:
   - list/create/retrieve/update/delete/update_status.
2. Add missing endpoint implementations for retrieve/update/delete/status transitions.
3. Align list filter semantics with Python/OpenAPI (including legacy/deprecated filters where backward compatibility is desired).
4. Expand Rust auth-config response models with high-value optional fields (`status`, `type`, `no_of_connections`, `tool_access_config`, `is_enabled_for_tool_router`, `shared_credentials`, etc.).
5. Align create response model to include toolkit linkage metadata where provided by API.

## Resource 7: `files` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/files.py`
  - `temp/composio_client/types/file_list_params.py`
  - `temp/composio_client/types/file_list_response.py`
  - `temp/composio_client/types/file_create_presigned_url_params.py`
  - `temp/composio_client/types/file_create_presigned_url_response.py`
- Rust:
  - `src/models/files.rs`
  - `src/utils/mimetypes.rs`
  - `src/config.rs`
  - `src/client.rs`

### Python `files` surface (baseline)

The Python resource exposes:

1. `list(cursor?, limit?, tool_slug?, toolkit_slug?)`
2. `create_presigned_url(filename, md5, mimetype, tool_slug, toolkit_slug)`

### Rust `files` surface (current)

The Rust SDK currently exposes file handling primarily via helper models/utilities:

1. `FileUploadable::from_path(...)` (local-path or URL input)
2. `FileUploadable::from_url(...)`
3. `FileDownloadable::download(...)`
4. internal presigned-request helper path (`request_upload_url` in `src/models/files.rs`)
5. MIME helper utilities (`src/utils/mimetypes.rs`)
6. client config toggles for auto file handling (`file_download_dir`, `auto_upload_download_files`)

No Rust public client methods were found for the Files REST resource endpoints (`/api/v3/files/list`, `/api/v3/files/upload/request`).

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| List files | `files.list` | _missing_ | MISSING_IN_RUST | No Rust endpoint wrapper for `GET /api/v3/files/list`. |
| Request presigned upload URL | `files.create_presigned_url` | internal helper in `FileUploadable::request_upload_url` | PARTIAL | Rust supports the underlying call but only through file-helper workflow, not as public Files API. |
| End-to-end upload flow | presigned URL request only (upload performed by caller) | `FileUploadable::from_path/from_url` handles request+upload | PARTIAL | Rust adds higher-level behavior and direct S3 PUT flow. |
| File download helper | _not in files resource_ | `FileDownloadable::download` | RUST_ONLY | Rust includes helper not represented as Python `files` resource operation. |

### Contract mismatches

#### 1) Missing Files endpoint wrappers in Rust client (MISSING_IN_RUST)

- Python exposes explicit Files resource methods for list and presigned URL creation.
- Rust has no public `ComposioClient` methods for files list/retrieve/delete style operations, and no dedicated Files resource surface.

#### 2) Pagination/filter contract gap for file listing (MISSING_IN_RUST)

- Python list supports `cursor`, `limit`, `tool_slug`, `toolkit_slug`.
- Rust lacks corresponding endpoint method/params; therefore filtering and pagination semantics cannot be used through Rust Files API.

#### 3) Presigned-request response contract coverage drift (PARTIAL)

- Python response includes nested `metadata.storage_backend` and deprecated alias handling for `newPresignedUrl`.
- Rust `FileUploadResponse` currently models only `id`, `key`, `type`, and `new_presigned_url` (no metadata field).
- This drops storage-backend-specific upload hints (e.g., Azure blob header requirements).

### Behavior mismatches

#### 1) Upload flow abstraction difference (PARTIAL)

- Python `create_presigned_url` only returns presigned info; caller performs upload semantics externally.
- Rust `FileUploadable::from_path/from_url` performs an opinionated full flow: MIME detection/fetching, MD5 calculation, presigned request, and S3 upload.
- This is convenient but not behaviorally identical to Python’s resource-level API contract.

#### 2) URL input and file-from-path helper behavior (RUST_ONLY/PARTIAL)

- Rust `from_path` auto-detects URL-like strings and routes to `from_url`.
- Python files resource does not provide equivalent file-from-path helper behavior in this layer.
- This is a Rust-specific convenience path with additional implicit behavior.

#### 3) Dedup/hash semantics handling location (PARTIAL)

- Python presigned endpoint expects caller-provided MD5 and documents dedup behavior server-side.
- Rust helpers automatically compute MD5 (local file or fetched bytes) before requesting upload URL.
- Behavior is functionally aligned but abstraction boundary differs.

### Response/type drift

#### 1) List response typing gap (MISSING_IN_RUST)

- Python defines typed list response with file metadata fields (`filename`, `md5`, `mimetype`, `tool_slug`, `toolkit_slug`) and pagination.
- Rust has no corresponding list endpoint model/wrapper in current client surface.

#### 2) Presigned response metadata drift (PARTIAL)

- Python models `metadata.storage_backend` and alias compatibility for `newPresignedUrl`.
- Rust `FileUploadResponse` omits metadata and alias/backward-compat fields.

#### 3) Upload/download helper types are not Files resource types (PARTIAL)

- Rust `FileUploadable`/`FileDownloadable` are helper abstractions with `name/mimetype/s3key` and `name/mimetype/s3url`.
- They are useful operationally but do not map 1:1 to Python Files resource request/response models.

### Recommended remediation order for `files`

1. Add public Rust Files resource methods for:
   - `list_files(...)` (`GET /api/v3/files/list`)
   - `create_file_presigned_url(...)` (`POST /api/v3/files/upload/request`)
2. Introduce typed Rust request/response models aligned with Python/OpenAPI, including presigned `metadata.storage_backend`.
3. Keep Rust helper abstractions (`FileUploadable`, `FileDownloadable`) but clearly layer them on top of public endpoint wrappers.
4. Add compatibility handling for response alias fields where needed (`newPresignedUrl`).
5. Evaluate whether file metadata/content-type and storage-backend-specific upload headers should be surfaced explicitly in helper APIs.

## Resource 8: `link` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/link.py`
  - `temp/composio_client/types/link_create_params.py`
  - `temp/composio_client/types/link_create_response.py`
- Rust:
  - `src/session.rs`
  - `src/client.rs`
  - `src/models/request.rs`
  - `src/models/response.rs`
  - `src/models/connected_accounts.rs`

### Python `link` surface (baseline)

The Python resource exposes:

1. `link.create(auth_config_id, user_id, callback_url?, connection_data?)`

Endpoint: `POST /api/v3/connected_accounts/link`.

### Rust `link` surface (current)

The Rust SDK exposes link-related functionality through different entry points:

1. `Session::create_auth_link(toolkit, callback_url?)` → `POST /tool_router/session/{session_id}/link`
2. `ComposioClient::authorize_toolkit(user_id, toolkit)` convenience flow (internal auth-config lookup + connected-account initiation)
3. internal `initiate_connection(user_id, auth_config_id, callback_url)` → `POST /api/v3/connected_accounts`

No Rust public method was found for direct `POST /api/v3/connected_accounts/link` with Python-equivalent request contract.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Create auth link session (direct link endpoint) | `link.create` | _missing_ | MISSING_IN_RUST | No Rust wrapper for `/api/v3/connected_accounts/link`. |
| Create auth link via session context | _not in this resource_ | `Session::create_auth_link` | RUST_ONLY | Rust provides Tool Router session-specific link flow not represented by Python `link` resource contract. |
| Toolkit authorization convenience | _not in this resource_ | `authorize_toolkit` | RUST_ONLY | Rust convenience flow creates/initiates connection without exposing Python link-token contract directly. |

### Contract mismatches

#### 1) Request contract mismatch for link creation (MISSING_IN_RUST)

- Python requires explicit `auth_config_id` and `user_id`, with optional `callback_url` and rich `connection_data` payload.
- Rust `Session::create_auth_link` instead accepts `toolkit` + optional `callback_url` and sends `LinkRequest { toolkit, callback_url }` to a different endpoint.
- Rust lacks a direct request model matching Python `link.create` fields (`auth_config_id`, `user_id`, `connection_data`).

#### 2) Endpoint contract mismatch (MISSING_IN_RUST)

- Python `link` resource uses `/api/v3/connected_accounts/link`.
- Rust link-like APIs use `/tool_router/session/{session_id}/link` (session path) or `/api/v3/connected_accounts` (connection initiation), changing server contract and prerequisites.

#### 3) Auth-config and connected-account linkage semantics drift (PARTIAL)

- Python direct link creation binds link session to explicit `auth_config_id` and user.
- Rust convenience path often infers/creates auth config (`authorize_toolkit` -> `get_or_create_auth_config`) before initiating connection.
- This shifts linkage semantics from explicit caller control (Python) to inferred workflow (Rust convenience).

### Behavior mismatches

#### 1) Link/session creation flow divergence (PARTIAL)

- Python exposes a direct link-session creation operation with returned link token + expiry metadata.
- Rust `Session::create_auth_link` depends on existing Tool Router session context (`session_id`) and toolkit-scoped link generation.
- Rust convenience flow (`authorize_toolkit`) may bypass link-token concepts in favor of direct connection initiation response.

#### 2) Callback/redirect semantics divergence (PARTIAL)

- Python contract includes callback plus optional connection prefill data in a single endpoint.
- Rust session link supports callback URL but does not expose Python `connection_data` prefill semantics in this flow.

#### 3) Ephemeral token semantics exposure gap (MISSING_IN_RUST)

- Python response includes `expires_at` for link-token lifetime semantics.
- Rust `LinkResponse` lacks `expires_at`, reducing visibility into one-time/ephemeral link expiry behavior.

### Response/type drift

#### 1) Link-create response richness drift (PARTIAL)

- Python `LinkCreateResponse` includes `connected_account_id`, `expires_at`, `link_token`, `redirect_url`.
- Rust `LinkResponse` includes `link_token`, `redirect_url`, `connected_account_id` but omits `expires_at`.

#### 2) Session-link response vs direct-link response mapping drift (PARTIAL)

- Rust response type is modeled for Tool Router session link endpoint behavior.
- Python type is tied to direct connected-accounts link endpoint.
- Fields overlap partially, but semantic scope differs.

#### 3) Missing typed direct-link request/response models (MISSING_IN_RUST)

- Python provides typed link params/response for direct endpoint.
- Rust lacks dedicated public typed request/response models for `/api/v3/connected_accounts/link`.

### Recommended remediation order for `link`

1. Add public Rust method for direct link creation endpoint parity:
   - `create_link(auth_config_id, user_id, callback_url?, connection_data?)` -> `POST /api/v3/connected_accounts/link`.
2. Add Rust request/response models aligned with Python/OpenAPI, including `expires_at` in response.
3. Preserve existing Rust session-based link helpers (`Session::create_auth_link`) as higher-level convenience, but clearly document endpoint/contract differences.
4. Consider exposing both explicit-link and convenience-toolkit flows in docs with guidance on when to use each.
5. Validate callback and redirect field semantics across both flows and align naming/typing where possible.

## Resource 9: `tool_router` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/tool_router/tool_router.py`
  - `temp/composio_client/resources/tool_router/session.py`
  - `temp/composio_client/resources/tool_router/__init__.py`
  - `temp/composio_client/types/tool_router_create_session_params.py`
  - `temp/composio_client/types/tool_router_create_session_response.py`
  - `temp/composio_client/types/tool_router/session_create_params.py`
  - `temp/composio_client/types/tool_router/session_create_response.py`
  - `temp/composio_client/types/tool_router/session_retrieve_response.py`
  - `temp/composio_client/types/tool_router/session_execute_params.py`
  - `temp/composio_client/types/tool_router/session_execute_response.py`
  - `temp/composio_client/types/tool_router/session_execute_meta_params.py`
  - `temp/composio_client/types/tool_router/session_execute_meta_response.py`
  - `temp/composio_client/types/tool_router/session_link_params.py`
  - `temp/composio_client/types/tool_router/session_link_response.py`
  - `temp/composio_client/types/tool_router/session_toolkits_params.py`
  - `temp/composio_client/types/tool_router/session_toolkits_response.py`
  - `temp/composio_client/types/tool_router/session_tools_response.py`
- Rust:
  - `src/client.rs`
  - `src/session.rs`
  - `src/models/request.rs`
  - `src/models/response.rs`

### Python `tool_router` surface (baseline)

Python exposes both legacy and current nested session routes:

1. `tool_router.create_session(...)` (legacy labs endpoint)
2. `tool_router.session.create(...)`
3. `tool_router.session.retrieve(session_id)`
4. `tool_router.session.execute(session_id, tool_slug, arguments)`
5. `tool_router.session.execute_meta(session_id, slug, arguments?)`
6. `tool_router.session.link(session_id, toolkit, callback_url?)`
7. `tool_router.session.toolkits(session_id, cursor?, is_connected?, limit?, search?, toolkits?)`
8. `tool_router.session.tools(session_id)`

### Rust `tool_router` surface (current)

Rust exposes Tool Router functionality via `ComposioClient` + `Session` abstractions:

1. `ComposioClient::create_session(user_id)` -> `SessionBuilder::send()` (`POST /tool_router/session`)
2. `ComposioClient::get_session(session_id)` (`GET /tool_router/session/{id}`)
3. `Session::execute_tool(tool_slug, arguments)` (`POST /tool_router/session/{id}/execute`)
4. `Session::execute_meta_tool(slug, arguments)` (`POST /tool_router/session/{id}/execute_meta`)
5. `Session::create_auth_link(toolkit, callback_url?)` (`POST /tool_router/session/{id}/link`)
6. `Session::list_toolkits().send()` (`GET /tool_router/session/{id}/toolkits` with filters)
7. `Session::get_meta_tools()` (`GET /tool_router/session/{id}/tools`)

No Rust equivalent was found for legacy `POST /api/v3/labs/tool_router/session`.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Create Tool Router session (current endpoint) | `tool_router.session.create` | `create_session(...).send()` | FULL | Endpoint and core config semantics align via session builder. |
| Retrieve Tool Router session | `tool_router.session.retrieve` | `get_session` | FULL | Endpoint-aligned retrieval semantics. |
| Execute routed tool | `tool_router.session.execute` | `Session::execute_tool` | FULL | Same route family and core contract. |
| Execute meta tool | `tool_router.session.execute_meta` | `Session::execute_meta_tool` | FULL | Same route family and meta-tool execution semantics. |
| Create session-scoped auth link | `tool_router.session.link` | `Session::create_auth_link` | PARTIAL | Core contract aligns; response field richness differs (see drift). |
| List session toolkits (filtered/paginated) | `tool_router.session.toolkits` | `Session::list_toolkits().send()` | FULL | Cursor/limit/search/is_connected/toolkits filter support present. |
| Get session tools schemas | `tool_router.session.tools` | `Session::get_meta_tools` | FULL | Endpoint and purpose align. |
| Legacy labs session creation | `tool_router.create_session` | _missing_ | MISSING_IN_RUST | No Rust wrapper for `/api/v3/labs/tool_router/session`. |

### Contract mismatches

#### 1) Missing legacy labs endpoint coverage (MISSING_IN_RUST)

- Python still exposes `tool_router.create_session` at `/api/v3/labs/tool_router/session` with legacy request/response shapes.
- Rust does not provide an equivalent method.

#### 2) Session-link response contract mismatch (PARTIAL)

- Python `session.link` response includes `link_token`, `redirect_url`, `connected_account_id`, and `expires_at`.
- Rust `LinkResponse` currently omits `expires_at`.

#### 3) Session model field coverage mismatch (PARTIAL)

- Python session response models include richer typed structures for toolkits/tools listings and session metadata variants.
- Rust models cover core fields but are generally simplified and less exhaustive in optional/deprecated compatibility fields.

### Behavior mismatches

#### 1) Builder abstraction vs direct resource calls (PARTIAL)

- Python exposes explicit resource methods with `session_id` parameters.
- Rust uses object-bound `Session` methods after creation/retrieval.
- Behavior is largely equivalent, but invocation style and lifecycle ergonomics differ.

#### 2) Convenience layering for routing flows (RUST_ONLY/PARTIAL)

- Rust adds higher-level conveniences (`SessionBuilder`, provider tool wrapping, `get_provider_tools`) around Tool Router sessions.
- Python resource layer stays closer to generated endpoint wrappers.
- Useful but introduces SDK-specific behavioral abstractions not mirrored in Python.

### Response/type drift

#### 1) Link response expiry drift (PARTIAL)

- Python session-link response includes explicit `expires_at` for ephemeral link/token lifecycle.
- Rust `LinkResponse` does not include this field.

#### 2) Toolkit/session listing model richness drift (PARTIAL)

- Python session toolkit response models include nested connected account/auth-config detail typing and date-time fields.
- Rust `ToolkitListResponse`/`ToolkitInfo` models are cleaner but narrower.

#### 3) Legacy create-session response drift (MISSING_IN_RUST)

- Python legacy create-session response includes fields like `chat_session_mcp_url` and `tool_router_instance_mcp_url`.
- Rust has no corresponding legacy endpoint wrapper/type surface.

### Recommended remediation order for `tool_router`

1. Decide whether legacy labs endpoint support is required; if yes, add explicit Rust method/type parity for `/api/v3/labs/tool_router/session`.
2. Add optional `expires_at` to Rust link response models used in session link flows.
3. Expand Rust Tool Router response models with additional optional fields where high-value session/toolkit metadata is currently dropped.
4. Keep Rust builder/session conveniences, but document endpoint-equivalent mappings for parity clarity.

## Resource 10: `mcp` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/mcp/mcp.py`
  - `temp/composio_client/resources/mcp/generate.py`
  - `temp/composio_client/resources/mcp/custom.py`
  - `temp/composio_client/resources/mcp/__init__.py`
  - `temp/composio_client/types/mcp_create_params.py`
  - `temp/composio_client/types/mcp_update_params.py`
  - `temp/composio_client/types/mcp_list_params.py`
  - `temp/composio_client/types/mcp_retrieve_app_params.py`
  - `temp/composio_client/types/mcp_create_response.py`
  - `temp/composio_client/types/mcp_update_response.py`
  - `temp/composio_client/types/mcp_list_response.py`
  - `temp/composio_client/types/mcp_retrieve_response.py`
  - `temp/composio_client/types/mcp_retrieve_app_response.py`
  - `temp/composio_client/types/mcp_delete_response.py`
  - `temp/composio_client/types/mcp/generate_url_params.py`
  - `temp/composio_client/types/mcp/generate_url_response.py`
  - `temp/composio_client/types/mcp/custom_create_params.py`
  - `temp/composio_client/types/mcp/custom_create_response.py`
- Rust:
  - `src/models/mcp.rs`
  - `src/client.rs`
  - `src/lib.rs`

### Python `mcp` surface (baseline)

Python exposes primary MCP and nested MCP sub-resources:

1. `mcp.create(...)` (`POST /api/v3/mcp/servers`)
2. `mcp.retrieve(id)` (`GET /api/v3/mcp/{id}`)
3. `mcp.update(id, ...)` (`PATCH /api/v3/mcp/{id}`)
4. `mcp.list(...)` (`GET /api/v3/mcp/servers`)
5. `mcp.delete(id)` (`DELETE /api/v3/mcp/{id}`)
6. `mcp.retrieve_app(...)` (`GET /api/v3/mcp/servers/app`)
7. `mcp.generate.create(...)` (`POST /api/v3/mcp/servers/generate`)
8. `mcp.custom.create(...)` (`POST /api/v3/mcp/servers/custom`)

### Rust `mcp` surface (current)

Rust currently provides MCP **data models/types** in `src/models/mcp.rs` (create/update/list/generate params and responses), but no public `ComposioClient` HTTP methods for MCP endpoint operations in `src/client.rs`.

This means Rust has typed MCP structures but lacks endpoint-level client operation granularity present in Python’s MCP resources and nested routes.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| Create MCP server | `mcp.create` | _missing endpoint method_ | MISSING_IN_RUST | Rust has `MCPCreateParams/MCPCreateResponse` types but no client method. |
| Retrieve MCP server | `mcp.retrieve` | _missing endpoint method_ | MISSING_IN_RUST | No `GET /api/v3/mcp/{id}` wrapper in Rust client. |
| Update MCP server | `mcp.update` | _missing endpoint method_ | MISSING_IN_RUST | Types exist, operation method missing. |
| List MCP servers | `mcp.list` | _missing endpoint method_ | MISSING_IN_RUST | Filtering/pagination not exposed via client operation. |
| Delete MCP server | `mcp.delete` | _missing endpoint method_ | MISSING_IN_RUST | No delete wrapper. |
| Retrieve app-scoped MCP listing | `mcp.retrieve_app` | _missing endpoint method_ | MISSING_IN_RUST | No equivalent helper endpoint wrapper. |
| Generate MCP URLs/instances | `mcp.generate.create` | _missing endpoint method_ | MISSING_IN_RUST | Rust has `MCPGenerateUrlParams/Response` types, no operation wrapper. |
| Create custom MCP server | `mcp.custom.create` | _missing endpoint method_ | MISSING_IN_RUST | Rust has model coverage but no nested resource operation. |

### Contract mismatches

#### 1) Nested MCP route structure not represented in Rust operations (MISSING_IN_RUST)

- Python has explicit nested resources (`mcp.generate`, `mcp.custom`) with distinct endpoint contracts.
- Rust has no equivalent operational namespace and no client methods for nested MCP flows.

#### 2) Server/session/config creation semantics mismatch (MISSING_IN_RUST)

- Python supports multiple MCP creation semantics: standard server create, custom server create, and generated user/account URL workflows.
- Rust currently cannot execute these semantics through public client operations despite having some request/response structs.

#### 3) Request payload contract drift in types (PARTIAL)

- Python `mcp.create` requires `auth_config_ids` and supports `allowed_tools`, `managed_auth_via_composio`, `no_auth_apps`.
- Rust `MCPCreateParams` is toolkit-oriented (`toolkits`, optional `auth_config_ids`, `custom_tools`) and diverges from Python endpoint payload shape.
- Rust type contract appears partially closer to `custom.create` semantics than `mcp.create` semantics.

#### 4) Helper endpoint contract gap (MISSING_IN_RUST)

- Python exposes `mcp.retrieve_app` and generation endpoints with their own filter/request contracts.
- Rust provides no corresponding endpoint methods.

### Behavior mismatches

#### 1) Sync generated-client behavior vs Rust convenience/type-only layer (MISSING_IN_RUST)

- Python generated resource methods directly execute HTTP operations for all MCP flows.
- Rust currently behaves as a type/convenience layer without executable MCP operation coverage in client APIs.

#### 2) Operation granularity mismatch (MISSING_IN_RUST)

- Python offers per-endpoint granularity (standard create, custom create, generate URLs, retrieve app listing).
- Rust cannot mirror this granularity operationally because endpoint wrappers are absent.

### Response/type drift

#### 1) Generated artifact/model richness mismatch (PARTIAL)

- Python MCP response models include richer/consistent fields for commands, timestamps, managed-auth flags, and paginated list surfaces across endpoints.
- Rust models cover many fields, but not all endpoint-specific variants and nested response structures one-to-one.

#### 2) Naming and shape drift across create/update/generate models (PARTIAL)

- Python distinguishes standard vs custom vs generate response contracts with dedicated typed models.
- Rust model set is broader in some areas but does not map cleanly to all Python endpoint contracts (and lacks endpoint binding), increasing ambiguity.

#### 3) Missing endpoint-bound typed responses in client layer (MISSING_IN_RUST)

- Python ties typed models directly to callable operations.
- Rust types are currently not bound to executable MCP client methods, reducing practical parity.

### Recommended remediation order for `mcp`

1. Add public Rust client operations for core MCP endpoints:
   - create/retrieve/update/list/delete.
2. Add nested MCP operation groups (or clearly named methods) for:
   - custom create,
   - URL generation,
   - app-scoped retrieval (`retrieve_app`).
3. Align Rust request structs with actual endpoint payload contracts (separate standard-create vs custom-create vs generate models where needed).
4. Bind response models to endpoint methods and add integration tests validating request/response shapes against expected OpenAPI semantics.
5. Preserve convenience/model abstractions, but ensure operation granularity matches Python nested route structure.

## Resource 11: `migration` (Python) vs Rust SDK parity

### Files inspected

- Python:
  - `temp/composio_client/resources/migration.py`
  - `temp/composio_client/types/migration_retrieve_nanoid_params.py`
  - `temp/composio_client/types/migration_retrieve_nanoid_response.py`
- Rust:
  - `src/client.rs`
  - `src/models/response.rs`
  - `src/models/auth_configs.rs`
  - `src/models/triggers.rs`

### Python `migration` surface (baseline)

The Python resource exposes a focused migration helper endpoint:

1. `migration.retrieve_nanoid(type, uuid)` (`GET /api/v3/migration/get-nanoid`)

Purpose: convert legacy UUID identifiers to NanoId identifiers for v3 migration workflows.

### Rust `migration` surface (current)

No dedicated Rust migration resource or migration endpoint wrappers were found.

Rust models still include some UUID fields for backward compatibility in various response structures, but there is no direct client API for UUID→NanoId conversion.

### Parity matrix (resource-level)

| Capability | Python | Rust | Status | Notes |
|---|---|---|---|---|
| UUID → NanoId conversion | `migration.retrieve_nanoid` | _missing_ | MISSING_IN_RUST | No Rust method for `/api/v3/migration/get-nanoid`. |
| Migration params typing | `MigrationRetrieveNanoidParams` | _missing_ | MISSING_IN_RUST | No dedicated request type for migration conversion endpoint. |
| Migration response typing | `MigrationRetrieveNanoidResponse` | _missing_ | MISSING_IN_RUST | No dedicated response type for conversion endpoint. |

### Contract mismatches

#### 1) Missing migration endpoint wrapper (MISSING_IN_RUST)

- Python provides direct endpoint contract for migration conversion with required `type` and `uuid` query params.
- Rust has no equivalent endpoint wrapper in `ComposioClient`.

#### 2) Missing typed migration request contract (MISSING_IN_RUST)

- Python constrains `type` to enum-like literals (`CONNECTED_ACCOUNT`, `AUTH_CONFIG`, `TRIGGER_INSTANCE`) and requires `uuid`.
- Rust lacks a migration request model and cannot enforce this contract at compile time for this endpoint.

#### 3) Missing typed migration response contract (MISSING_IN_RUST)

- Python returns typed payload containing `nanoid`.
- Rust has no dedicated response model for this endpoint.

### Behavior mismatches

#### 1) Migration job/start/run semantics not exposed in Rust (MISSING_IN_RUST)

- Python exposes at least one explicit migration operation (`retrieve_nanoid`) for identifier migration workflows.
- Rust exposes no migration operation layer (job/start/run/helper semantics absent in client API).

#### 2) Admin/helper migration operation gap (MISSING_IN_RUST)

- Python resource acts as a migration helper utility endpoint.
- Rust has no equivalent convenience/helper operation for migration support.

### Response/type drift

#### 1) Migration-specific typed model coverage gap (MISSING_IN_RUST)

- Python has dedicated migration request/response models.
- Rust has no migration-specific model module or endpoint-bound types.

#### 2) UUID compatibility fields without conversion helper (PARTIAL)

- Rust responses in several domains still include `uuid`-related fields for compatibility.
- Without a migration conversion endpoint wrapper, Rust users must implement external mapping logic when only UUIDs are available.

### Recommended remediation order for `migration`

1. Add public Rust migration endpoint wrapper:
   - `get_nanoid(resource_type, uuid)` -> `GET /api/v3/migration/get-nanoid`.
2. Add strongly typed Rust request/response models for migration conversion semantics.
3. Add tests validating enum/resource-type serialization and expected response mapping (`nanoid`).
4. Document migration usage path in Rust SDK guides for legacy UUID consumers.

## Next resource queued

- `project` (next incremental audit pass).
