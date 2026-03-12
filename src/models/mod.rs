//! Data models for Composio API
//!
//! This module contains all request and response models for the Composio Tool Router API,
//! as well as enums for various API types.
//!
//! # Organization
//!
//! - [`request`] - Request models for API calls
//! - [`response`] - Response models from API calls
//! - [`enums`] - Enums for meta tool slugs, tag types, and auth schemes
//!
//! # Examples
//!
//! ```rust
//! use composio_sdk::models::{SessionConfig, ToolkitFilter, MetaToolSlug};
//!
//! // Create a session configuration
//! let config = SessionConfig {
//!     user_id: "user_123".to_string(),
//!     toolkits: Some(ToolkitFilter::Enable(vec!["github".to_string()])),
//!     auth_configs: None,
//!     connected_accounts: None,
//!     manage_connections: Some(true),
//!     tools: None,
//!     tags: None,
//!     workbench: None,
//! };
//! ```

pub mod auth_configs;
pub mod base;
pub mod cli;
pub mod connected_accounts;
pub mod custom_tools;
pub mod enums;
pub mod files;
pub mod internal;
pub mod link;
pub mod mcp;
pub mod migration;
pub mod modifiers;
pub mod project;
pub mod request;
pub mod response;
pub mod telemetry;
pub mod toolkits;
pub mod tools;
pub mod triggers;
pub mod versioning;
pub mod webhook_events;

// ============================================================================
// Base Resources
// ============================================================================

/// Base resource trait
pub use base::Resource;

/// Base resource structure
pub use base::BaseResource;

/// Telemetry context
pub use base::TelemetryContext;

/// Environment type
pub use base::Environment;

// ============================================================================
// Auth Configs
// ============================================================================

// Note: AuthConfigs resource is pending full HTTP client integration

/// Auth config list parameters
pub use auth_configs::AuthConfigListParams;

/// Auth config list response
pub use auth_configs::AuthConfigListResponse;

/// Auth config information
pub use auth_configs::AuthConfigInfo;

/// Auth config create parameters
pub use auth_configs::AuthConfigCreateParams;

/// Auth config options
pub use auth_configs::AuthConfigOptions;

/// Auth config create response
pub use auth_configs::AuthConfigCreateResponse;

/// Auth config
pub use auth_configs::AuthConfig;

/// Auth config retrieve response
pub use auth_configs::AuthConfigRetrieveResponse;

/// Auth config update parameters
pub use auth_configs::AuthConfigUpdateParams;

/// Default credentials
pub use auth_configs::DefaultCredentials;

/// Auth config update response
pub use auth_configs::AuthConfigUpdateResponse;

/// Auth config status enum
pub use auth_configs::AuthConfigStatus;

/// Auth config delete response
pub use auth_configs::AuthConfigDeleteResponse;

/// Auth config status update response
pub use auth_configs::AuthConfigStatusUpdateResponse;

// ============================================================================
// CLI
// ============================================================================

/// CLI session status.
pub use cli::CliSessionStatus;

/// CLI linked account payload.
pub use cli::CliLinkedAccount;

/// CLI get-session params.
pub use cli::CliGetSessionParams;

/// CLI create-session response.
pub use cli::CliCreateSessionResponse;

/// CLI get-session response.
pub use cli::CliGetSessionResponse;

// ============================================================================
// Connected Accounts
// ============================================================================

// Note: ConnectedAccounts resource is pending full HTTP client integration

/// Connection request
pub use connected_accounts::ConnectionRequest;

/// Connection status
pub use connected_accounts::ConnectionStatus;

/// Connection error
pub use connected_accounts::ConnectionError;

/// Authentication scheme
pub use connected_accounts::AuthScheme;

/// Connection state
pub use connected_accounts::ConnectionState;

/// Auth scheme helper
pub use connected_accounts::AuthSchemeHelper;

/// Global auth scheme helper instance
pub use connected_accounts::AUTH_SCHEME;

/// Initiate connection parameters
pub use connected_accounts::InitiateConnectionParams;

/// Link connection parameters
pub use connected_accounts::LinkConnectionParams;

/// Connected account list parameters
pub use connected_accounts::ConnectedAccountListParams;

/// Connected account information
pub use connected_accounts::ConnectedAccountInfo;

/// Connected account list response
pub use connected_accounts::ConnectedAccountListResponse;

/// Connected account refresh parameters
pub use connected_accounts::ConnectedAccountRefreshParams;

/// Connected account refresh response
pub use connected_accounts::ConnectedAccountRefreshResponse;

/// Connected account update status parameters
pub use connected_accounts::ConnectedAccountUpdateStatusParams;

/// Connected account delete response
pub use connected_accounts::ConnectedAccountDeleteResponse;

/// Direct connected account link create params
pub use link::ConnectedAccountLinkCreateParams;

/// Direct connected account link create response
pub use link::ConnectedAccountLinkCreateResponse;

/// Connected account update status response
pub use connected_accounts::ConnectedAccountUpdateStatusResponse;

/// Default wait timeout for connections
pub use connected_accounts::DEFAULT_WAIT_TIMEOUT;

// ============================================================================
// Tools
// ============================================================================

// Note: Tools resource is pending full HTTP client integration

/// Tool execution response
pub use tools::ToolExecutionResponse;

/// Tool execution parameters
pub use tools::ToolExecuteParams as ToolsExecuteParams;

/// Custom authentication parameters for tools
pub use tools::CustomAuthParams as ToolsCustomAuthParams;

/// Custom connection data for tools
pub use tools::CustomConnectionData as ToolsCustomConnectionData;

/// Tool list parameters
pub use tools::ToolListParams;

/// Tool information
pub use tools::ToolInfo;

/// Toolkit reference
pub use tools::ToolkitRef;

/// Tool list response
pub use tools::ToolListResponse;

/// Tool enum response
pub use tools::ToolRetrieveEnumResponse;

/// Tool proxy parameters
pub use tools::ToolProxyParams;

/// HTTP method
pub use tools::HttpMethod;

/// Proxy parameter
pub use tools::ProxyParameter;

/// Parameter location
pub use tools::ParameterLocation;

/// Tool proxy response
pub use tools::ToolProxyResponse;

/// Tool input generation parameters
pub use tools::ToolInputGenerationParams;

/// Tool input generation response
pub use tools::ToolInputGenerationResponse;

/// Custom tool definition (legacy, use custom_tools module instead)
pub use tools::CustomToolDefinition;

/// Custom tool execution request (legacy, use custom_tools module instead)
pub use tools::CustomToolExecutionRequest;

// ============================================================================
// Custom Tools (New Implementation)
// ============================================================================

/// Custom tool with execution logic
pub use custom_tools::CustomTool;

/// Custom tools registry for managing user-defined tools
pub use custom_tools::CustomToolsRegistry;

/// Trait for executing proxy requests
pub use custom_tools::ExecuteRequestFn;

/// Trait for custom tool executors
pub use custom_tools::CustomToolExecutor;

// ============================================================================
// Modifiers
// ============================================================================

/// Tool execution parameters
pub use modifiers::ToolExecuteParams;

/// Custom authentication parameters
pub use modifiers::CustomAuthParams;

/// Custom connection data
pub use modifiers::CustomConnectionData;

/// Modifier type enumeration
pub use modifiers::ModifierType;

/// Before execute trait
pub use modifiers::BeforeExecute;

/// After execute trait
pub use modifiers::AfterExecute;

/// Schema modifier trait
pub use modifiers::SchemaModifier;

/// Before execute meta trait
pub use modifiers::BeforeExecuteMeta;

/// After execute meta trait
pub use modifiers::AfterExecuteMeta;

/// Modifier structure
pub use modifiers::Modifier;

/// Collection of modifiers
pub use modifiers::Modifiers;

/// Apply before execute modifiers
pub use modifiers::apply_before_execute_modifiers;

/// Apply after execute modifiers
pub use modifiers::apply_after_execute_modifiers;

/// Apply schema modifiers
pub use modifiers::apply_schema_modifiers;

/// Apply before execute meta modifiers
pub use modifiers::apply_before_execute_meta_modifiers;

/// Apply after execute meta modifiers
pub use modifiers::apply_after_execute_meta_modifiers;

// ============================================================================
// Telemetry
// ============================================================================

/// Telemetry data structure
pub use telemetry::TelemetryData;

/// Error data for telemetry
pub use telemetry::ErrorData;

/// Source data for telemetry
pub use telemetry::SourceData;

/// Metadata for telemetry
pub use telemetry::Metadata;

/// Event type enumeration
pub use telemetry::EventType;

/// Event tuple
pub use telemetry::Event;

/// Push telemetry event
pub use telemetry::push_event;

/// Create telemetry event
pub use telemetry::create_event;

// ============================================================================
// File Handling
// ============================================================================

/// File upload functionality
pub use files::FileUploadable;

/// File download functionality
pub use files::FileDownloadable;

/// File helper for schema processing
pub use files::FileHelper;

/// File list query parameters.
pub use files::FileListParams;

/// File list item.
pub use files::FileListItem;

/// File list response.
pub use files::FileListResponse;

/// File upload request parameters.
pub use files::FileCreatePresignedUrlParams;

/// File upload metadata.
pub use files::FileCreatePresignedUrlMetadata;

/// File storage backend.
pub use files::FileStorageBackend;

/// File upload response
pub use files::FileUploadResponse;

/// File upload response alias for endpoint parity.
pub use files::FileCreatePresignedUrlResponse;

// ============================================================================
// Internal SDK
// ============================================================================

/// Internal SDK resource
pub use internal::Internal;

/// SDK realtime credentials response
pub use internal::SDKRealtimeCredentialsResponse;

// ============================================================================
// Migration
// ============================================================================

/// Migration resource type for UUID conversion.
pub use migration::MigrationResourceType;

/// Migration get-nanoid query parameters.
pub use migration::MigrationGetNanoIdParams;

/// Migration get-nanoid response.
pub use migration::MigrationGetNanoIdResponse;

// ============================================================================
// MCP (Model Context Protocol)
// ============================================================================

/// MCP toolkit configuration
pub use mcp::MCPToolkitConfig;

/// MCP server instance
pub use mcp::MCPServerInstance;

/// Complete MCP server information
pub use mcp::MCPItem;

/// Paginated MCP list response
pub use mcp::MCPListResponse;

/// MCP create response
pub use mcp::MCPCreateResponse;

/// MCP update response
pub use mcp::MCPUpdateResponse;

/// MCP delete response
pub use mcp::MCPDeleteResponse;

/// MCP generate URL response
pub use mcp::MCPGenerateUrlResponse;

/// MCP app-scoped list response
pub use mcp::MCPRetrieveAppResponse;

/// MCP custom create response
pub use mcp::MCPCustomCreateResponse;

/// MCP create parameters
pub use mcp::MCPCreateParams;

/// MCP update parameters
pub use mcp::MCPUpdateParams;

/// MCP list parameters
pub use mcp::MCPListParams;

/// MCP generate URL parameters
pub use mcp::MCPGenerateUrlParams;

/// MCP custom create parameters
pub use mcp::MCPCustomCreateParams;

/// MCP app-scoped list query parameters
pub use mcp::MCPRetrieveAppParams;

// ============================================================================
// Project
// ============================================================================

/// Project log visibility setting
pub use project::ProjectLogVisibilitySetting;

/// Project config response
pub use project::ProjectConfigResponse;

/// Project config update parameters
pub use project::ProjectConfigUpdateParams;

// ============================================================================
// Enums
// ============================================================================

/// Meta tool slugs for the 5 core Composio meta tools
pub use enums::MetaToolSlug;

/// Tag types for filtering tools by behavior hints
pub use enums::TagType;

// Note: AuthScheme is now exported from connected_accounts module

// ============================================================================
// Versioning Types
// ============================================================================

/// Toolkit version type
pub use versioning::ToolkitVersion;

/// Map of toolkit versions
pub use versioning::ToolkitVersions;

/// Toolkit version parameter for configuration
pub use versioning::ToolkitVersionParam;

/// Constant for "latest" version
pub use versioning::TOOLKIT_LATEST_VERSION;

// ============================================================================
// Request Models
// ============================================================================

/// Configuration for creating a Tool Router session
pub use request::SessionConfig;

/// Configuration for connection management
pub use request::ManageConnectionsConfig;

/// Toolkit filter for enabling or disabling specific toolkits
pub use request::ToolkitFilter;

/// Configuration for per-toolkit tool filtering
pub use request::ToolsConfig;

/// Tool filter for a specific toolkit
pub use request::ToolFilter;

/// Configuration for tag-based tool filtering
pub use request::TagsConfig;

/// Configuration for workbench execution
pub use request::WorkbenchConfig;

/// Configuration for assistive prompt generation (experimental)
pub use request::AssistivePromptConfig;

/// Experimental configuration for Tool Router sessions
pub use request::ExperimentalConfig;

/// Request to execute a tool
pub use request::ToolExecutionRequest;

/// Request to execute a meta tool
pub use request::MetaToolExecutionRequest;

/// Request to create an authentication link
pub use request::LinkRequest;

// Note: AuthConfigCreateParams, AuthConfigListParams, and AuthConfigUpdateParams
// are now exported from auth_configs module with more complete implementations

/// Authentication configuration data (legacy, use auth_configs module)
pub use request::AuthConfigData;

/// Parameters for creating a connected account
pub use request::ConnectedAccountCreateParams;

/// Reference to an authentication configuration
pub use request::AuthConfigReference;

/// Connection data for creating a connected account
pub use request::ConnectionData;

// Note: ConnectedAccountListParams is now exported from connected_accounts module

// Note: ToolProxyParams is now exported from tools module

// ============================================================================
// Response Models
// ============================================================================

/// Response from session creation
pub use response::SessionResponse;

/// MCP server information
pub use response::McpInfo;

/// Tool schema information
pub use response::ToolSchema;

// Note: ToolExecutionResponse is now exported from tools module

/// Response from meta tool execution
pub use response::MetaToolExecutionResponse;

/// Response from listing toolkits
pub use toolkits::ToolkitListResponse;

/// Information about a toolkit
pub use toolkits::ToolkitItem as ToolkitInfo;

/// Metadata about a toolkit
pub use toolkits::ToolkitMeta;

// Note: ConnectedAccountInfo, ConnectedAccountListResponse, and
// ConnectedAccountUpdateStatusResponse are now exported from connected_accounts module

/// Response from creating an auth link
pub use response::LinkResponse;

/// MCP server type for Tool Router sessions
pub use response::ToolRouterMcpServerType;

/// MCP server configuration for Tool Router sessions
pub use response::ToolRouterMcpServerConfig;

/// Experimental features in Tool Router session response
pub use response::ToolRouterSessionExperimental;

/// Auth config information for a toolkit connection
pub use response::ToolkitConnectionAuthConfig;

/// Connected account information for a toolkit
pub use response::ToolkitConnectedAccount;

/// Connection information for a toolkit
pub use response::ToolkitConnection;

/// Connection state of a toolkit in a Tool Router session
pub use response::ToolkitConnectionState;

/// Details of toolkit connections in a Tool Router session
pub use response::ToolkitConnectionsDetails;

/// Error response from API
pub use response::ErrorResponse;

// Note: AuthConfigCreateResponse, AuthConfigListResponse, AuthConfigRetrieveResponse,
// AuthConfigDetail, and AuthConfigInfo are now exported from auth_configs module
// with more complete implementations

/// Response from creating a connected account
pub use response::ConnectedAccountCreateResponse;

// Note: ConnectedAccountListResponse is exported from connected_accounts module

/// Detailed information about a connected account
pub use response::ConnectedAccountDetail;

/// Response from retrieving a single connected account
pub use response::ConnectedAccountRetrieveResponse;

// Note: ConnectedAccountUpdateStatusResponse is exported from connected_accounts module

// Note: ToolProxyResponse is now exported from tools module

/// Response from creating or updating a trigger instance
pub use response::TriggerInstanceUpsertResponse;

// Triggers types
pub use triggers::{
    TriggerConnectedAccount, TriggerCreateParams, TriggerCreateResponse, TriggerEvent,
    TriggerInstance, TriggerInstanceListParams, TriggerInstanceListResponse, TriggerMetadata,
    TriggerToolkitRef, TriggerType, TriggerTypeListParams, TriggerTypeListResponse,
    TriggerTypeRetrieveEnumResponse, TriggerTypeRetrieveParams, VerifyWebhookResult,
    WebhookVerifyParams, WebhookVersion,
};

// ============================================================================
// Webhook Events
// ============================================================================

/// Webhook event types
pub use webhook_events::WebhookEventType;

// Note: ConnectionStatus is already exported from connected_accounts module
// webhook_events::ConnectionStatus is the same enum

/// Connected account toolkit information
pub use webhook_events::ConnectedAccountToolkit;

/// Connected account auth config
pub use webhook_events::ConnectedAccountAuthConfig;

/// Connection state value (webhook-specific)
pub use webhook_events::ConnectionStateVal;

// Note: ConnectionState is already exported from connected_accounts module
// webhook_events::ConnectionState is a different struct for webhook payloads

/// Single connected account detailed response
pub use webhook_events::SingleConnectedAccountDetailedResponse;

/// Webhook connection metadata
pub use webhook_events::WebhookConnectionMetadata;

/// Connection expired event
pub use webhook_events::ConnectionExpiredEvent;

/// Webhook event union type
pub use webhook_events::WebhookEvent;

/// Check if payload is a connection expired event
pub use webhook_events::is_connection_expired_event;

// ============================================================================
// Toolkits
// ============================================================================

// Note: Toolkits resource is pending full HTTP client integration

/// Toolkit list parameters
pub use toolkits::ToolkitListParams;

/// Sort by options
pub use toolkits::SortBy;

/// Managed by options
pub use toolkits::ManagedBy;

/// Toolkit retrieve response
pub use toolkits::ToolkitRetrieveResponse;

/// Toolkit retrieve parameters
pub use toolkits::ToolkitRetrieveParams;

/// Authentication configuration detail
pub use toolkits::AuthConfigDetail;

/// Authentication configuration fields
pub use toolkits::AuthConfigFields;

/// Authentication field set
pub use toolkits::AuthFieldSet;

/// Authentication field
pub use toolkits::AuthField;

/// Toolkit categories response
pub use toolkits::ToolkitCategoriesResponse;

/// Toolkit category
pub use toolkits::ToolkitCategory;

/// Authorization parameters
pub use toolkits::AuthorizeParams;
