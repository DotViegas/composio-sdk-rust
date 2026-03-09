//! # Composio Rust SDK
//!
//! A minimal, type-safe Rust SDK for the Composio Tool Router REST API.
//!
//! This SDK enables ZeroClaw and other Rust applications to interact with external services
//! through Composio's Tool Router API, providing session management, tool execution,
//! and authentication handling with a minimal memory footprint (~2 MB).
//!
//! ## Features
//!
//! - **Session Management**: Create and manage Tool Router sessions for users
//! - **Tool Execution**: Execute tools and meta tools with automatic retry logic
//! - **Type Safety**: Comprehensive type definitions for all API requests and responses
//! - **Error Handling**: Detailed error types with actionable error messages
//! - **Async/Await**: Built on tokio for efficient async operations
//! - **Memory Efficient**: Minimal memory footprint suitable for resource-constrained environments
//!
//! ## Quick Start
//!
//! ```no_run
//! use composio_sdk::{ComposioClient, MetaToolSlug};
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize client with API key
//!     let client = ComposioClient::builder()
//!         .api_key(std::env::var("COMPOSIO_API_KEY")?)
//!         .build()?;
//!
//!     // Create a session for a user
//!     let session = client
//!         .create_session("user_123")
//!         .toolkits(vec!["github", "gmail"])
//!         .manage_connections(true)
//!         .send()
//!         .await?;
//!
//!     println!("Session ID: {}", session.session_id());
//!     println!("MCP URL: {}", session.mcp_url());
//!
//!     // Execute a tool
//!     let result = session
//!         .execute_tool(
//!             "GITHUB_CREATE_ISSUE",
//!             json!({
//!                 "owner": "composio",
//!                 "repo": "composio",
//!                 "title": "Test issue",
//!                 "body": "Created via Rust SDK"
//!             })
//!         )
//!         .await?;
//!
//!     println!("Result: {:?}", result.data);
//!
//!     // Execute a meta tool
//!     let search_result = session
//!         .execute_meta_tool(
//!             MetaToolSlug::ComposioSearchTools,
//!             json!({
//!                 "query": "create a GitHub issue"
//!             })
//!         )
//!         .await?;
//!
//!     println!("Search result: {:?}", search_result.data);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Session Configuration
//!
//! Sessions can be configured with various options:
//!
//! ```no_run
//! # use composio_sdk::ComposioClient;
//! # async fn example(client: ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
//! let session = client
//!     .create_session("user_123")
//!     .toolkits(vec!["github", "gmail"])           // Enable specific toolkits
//!     .disable_toolkits(vec!["exa", "firecrawl"])  // Or disable specific toolkits
//!     .auth_config("github", "ac_custom_config")   // Use custom auth config
//!     .connected_account("gmail", "ca_work_email") // Select specific account
//!     .manage_connections(true)                     // Enable in-chat auth
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The SDK provides comprehensive error handling:
//!
//! ```no_run
//! # use composio_sdk::{ComposioClient, ComposioError};
//! # use serde_json::json;
//! # async fn example(client: ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
//! # let session = client.create_session("user_123").send().await?;
//! match session.execute_tool("INVALID_TOOL", json!({})).await {
//!     Ok(result) => println!("Success: {:?}", result),
//!     Err(ComposioError::ApiError { status, message, suggested_fix, .. }) => {
//!         eprintln!("API error ({}): {}", status, message);
//!         if let Some(fix) = suggested_fix {
//!             eprintln!("Suggested fix: {}", fix);
//!         }
//!     }
//!     Err(ComposioError::NetworkError(e)) => {
//!         eprintln!("Network error: {}", e);
//!     }
//!     Err(e) => {
//!         eprintln!("Other error: {}", e);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuration
//!
//! Customize SDK behavior with [`ComposioConfig`]:
//!
//! ```no_run
//! use composio_sdk::ComposioClient;
//! use std::time::Duration;
//!
//! let client = ComposioClient::builder()
//!     .api_key("your-api-key")
//!     .base_url("https://backend.composio.dev/api/v3")
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(5)
//!     .build()?;
//! # Ok::<(), composio_sdk::ComposioError>(())
//! ```

// ============================================================================
// Module Declarations
// ============================================================================

/// Client implementation for the Composio API
pub mod client;

/// Configuration types for the SDK
pub mod config;

/// Error types and error handling utilities
pub mod error;

/// Data models for API requests and responses
pub mod models;

/// Retry logic and exponential backoff utilities
pub mod retry;

/// Session management and tool execution
pub mod session;

/// Wizard instruction generation from Composio Skills
pub mod wizard;

/// Skills integration for copying and managing Composio Skills
pub mod skills_integration;

/// Meta tools - Native Rust implementations
pub mod meta_tools;

/// Utility functions for the SDK
pub mod utils;

/// Provider system for framework-specific tool formatting
pub mod providers;

// ============================================================================
// SDK Metadata
// ============================================================================

/// SDK version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// SDK name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Get SDK version information
///
/// # Examples
///
/// ```rust
/// use composio_sdk;
///
/// println!("Using Composio SDK v{}", composio_sdk::VERSION);
/// ```
pub fn version() -> &'static str {
    VERSION
}

// ============================================================================
// Core Client and Configuration
// ============================================================================

/// Main client for interacting with the Composio API
pub use client::ComposioClient;

/// Builder for constructing a [`ComposioClient`] with custom configuration
pub use client::ComposioClientBuilder;

/// Configuration for the Composio SDK
pub use config::ComposioConfig;

// ============================================================================
// Provider System
// ============================================================================

/// Provider trait for framework-specific tool formatting
///
/// See the [`providers`] module documentation for detailed usage examples.
pub use providers::Provider;

/// OpenAI provider for Chat Completions API
pub use providers::OpenAIProvider;

/// Anthropic provider for Claude API
pub use providers::AnthropicProvider;

// ============================================================================
// Logging Utilities
// ============================================================================

/// Logging utilities for the SDK
///
/// This module provides logging configuration with verbosity control,
/// message truncation, and environment-based setup.
///
/// See the [`utils::logging`] module documentation for detailed usage examples.
pub use utils::logging::{
    setup as setup_logging, setup_from_env as setup_logging_from_env,
    get_verbosity, set_verbosity, truncate_message, LogLevel, Verbosity, WithLogger,
    ENV_COMPOSIO_LOGGING_LEVEL, ENV_COMPOSIO_LOG_VERBOSITY,
};

// ============================================================================
// Session Management
// ============================================================================

/// A Tool Router session for a specific user
pub use session::Session;

/// Builder for constructing a session with custom configuration
pub use session::SessionBuilder;

// ============================================================================
// Error Types
// ============================================================================

/// Main error type for the SDK
pub use error::ComposioError;

/// Detailed error information from API error responses
pub use error::ErrorDetail;

// ============================================================================
// Request Models
// ============================================================================

/// Configuration for creating a Tool Router session
pub use models::SessionConfig;

/// Toolkit filter for enabling or disabling specific toolkits
pub use models::ToolkitFilter;

/// Configuration for per-toolkit tool filtering
pub use models::ToolsConfig;

/// Tool filter for a specific toolkit
pub use models::ToolFilter;

/// Configuration for tag-based tool filtering
pub use models::TagsConfig;

/// Configuration for workbench execution
pub use models::WorkbenchConfig;

/// Request to execute a tool
pub use models::ToolExecutionRequest;

/// Request to execute a meta tool
pub use models::MetaToolExecutionRequest;

/// Request to create an authentication link
pub use models::LinkRequest;

/// Parameters for creating an authentication configuration
pub use models::AuthConfigCreateParams;

/// Authentication configuration data
pub use models::AuthConfigData;

/// Parameters for listing authentication configurations
pub use models::AuthConfigListParams;

/// Parameters for updating an authentication configuration
pub use models::AuthConfigUpdateParams;

/// Parameters for creating a connected account
pub use models::ConnectedAccountCreateParams;

/// Reference to an authentication configuration
pub use models::AuthConfigReference;

/// Connection data for creating a connected account
pub use models::ConnectionData;

/// Parameters for listing connected accounts
pub use models::ConnectedAccountListParams;

/// Parameters for executing a proxy request
pub use models::ToolProxyParams;

// ============================================================================
// Response Models
// ============================================================================

/// Response from session creation
pub use models::SessionResponse;

/// MCP server information
pub use models::McpInfo;

/// Tool schema information
pub use models::ToolSchema;

/// Response from tool execution
pub use models::ToolExecutionResponse;

/// Response from meta tool execution
pub use models::MetaToolExecutionResponse;

/// Response from listing toolkits
pub use models::ToolkitListResponse;

/// Information about a toolkit
pub use models::ToolkitInfo;

/// Metadata about a toolkit
pub use models::ToolkitMeta;

/// Information about a connected account
pub use models::ConnectedAccountInfo;

/// Response from creating an auth link
pub use models::LinkResponse;

/// Error response from API
pub use models::ErrorResponse;

/// Response from creating an authentication configuration
pub use models::AuthConfigCreateResponse;

/// Response from listing authentication configurations
pub use models::AuthConfigListResponse;

/// Response from retrieving a single authentication configuration
pub use models::AuthConfigRetrieveResponse;

/// Information about an authentication configuration
pub use models::AuthConfigInfo;

/// Response from creating a connected account
pub use models::ConnectedAccountCreateResponse;

/// Response from listing connected accounts
pub use models::ConnectedAccountListResponse;

/// Detailed information about a connected account
pub use models::ConnectedAccountDetail;

/// Response from retrieving a single connected account
pub use models::ConnectedAccountRetrieveResponse;

/// Response from updating connected account status
pub use models::ConnectedAccountUpdateStatusResponse;

/// Response from a proxy request
pub use models::ToolProxyResponse;

/// Response from creating or updating a trigger instance
pub use models::TriggerInstanceUpsertResponse;

// ============================================================================
// Enums
// ============================================================================

/// Meta tool slugs for the 5 core Composio meta tools
pub use models::MetaToolSlug;

/// Tag types for filtering tools by behavior hints
pub use models::TagType;

/// Authentication schemes supported by toolkits
pub use models::AuthScheme;

// ============================================================================
// Versioning Types
// ============================================================================

/// Toolkit version type
pub use models::ToolkitVersion;

/// Map of toolkit versions
pub use models::ToolkitVersions;

/// Toolkit version parameter for configuration
pub use models::ToolkitVersionParam;

/// Constant for "latest" version
pub use models::TOOLKIT_LATEST_VERSION;

// ============================================================================
// Webhook Events
// ============================================================================

/// Webhook event types and utilities
///
/// This module provides strongly-typed definitions for webhook events,
/// enabling type-safe handling of events like connection expiration.
///
/// See the [`models::webhook_events`] module documentation for detailed usage examples.
pub use models::webhook_events::{
    ConnectionExpiredEvent, ConnectionState, ConnectionStatus,
    SingleConnectedAccountDetailedResponse, WebhookConnectionMetadata, WebhookEvent,
    WebhookEventType, is_connection_expired_event,
};

// ============================================================================
// Tool Modifiers
// ============================================================================

/// Tool modifier traits and utilities
///
/// This module provides functionality to modify tool schemas, execution parameters,
/// and execution responses. Modifiers allow you to customize tool behavior without
/// changing the underlying tool definitions.
///
/// See the [`models::modifiers`] module documentation for detailed usage examples.
pub use models::modifiers::{
    AfterExecute, BeforeExecute, SchemaModifier, BeforeExecuteMeta, AfterExecuteMeta,
    Modifier, Modifiers, ToolExecuteParams, CustomAuthParams, CustomConnectionData,
};

// ============================================================================
// Custom Tools
// ============================================================================

/// Custom tools functionality
///
/// This module provides functionality for creating and managing custom tools.
/// Custom tools can be simple tools without authentication or toolkit-based
/// tools with authentication and proxy execution.
///
/// See the [`models::custom_tools`] module documentation for detailed usage examples.
pub use models::custom_tools::{
    CustomTool, CustomToolsRegistry, ExecuteRequestFn,
};

// ============================================================================
// Triggers
// ============================================================================

/// Trigger event types and utilities
///
/// This module provides functionality to manage triggers in Composio.
/// Triggers are event listeners that notify your application when specific
/// events occur in connected services.
///
/// See the [`models::triggers`] module documentation for detailed usage examples.
pub use models::triggers::{
    TriggerEvent, TriggerMetadata, TriggerConnectedAccount,
    TriggerType, TriggerInstance, WebhookVersion,
};

// ============================================================================
// Connected Accounts
// ============================================================================

/// Connected accounts management
///
/// This module provides functionality to manage connected accounts,
/// which represent user connections to external services through Composio.
///
/// See the [`models::connected_accounts`] module documentation for detailed usage examples.
pub use models::connected_accounts::{
    ConnectionRequest, ConnectionStatus as ConnectedAccountStatus,
    AuthScheme as ConnectedAccountAuthScheme, AUTH_SCHEME as auth_scheme,
};

// ============================================================================
// Wizard Module (Skills Integration)
// ============================================================================

/// Wizard instruction generation utilities
///
/// This module provides tools for extracting Composio Skills content and
/// generating wizard instructions for AI agents based on official best practices.
///
/// See the [`wizard`] module documentation for detailed usage examples.
pub use wizard::{
    Impact, InstructionValidator, Rule, SkillsExtractor, ValidationResult,
    WizardInstructionGenerator,
};

// ============================================================================
// Skills Integration Module
// ============================================================================

/// Skills integration error types and utilities
///
/// This module provides error handling for Skills integration operations,
/// including file I/O errors, YAML parsing errors, validation failures,
/// and security violations.
///
/// See the [`skills_integration`] module documentation for detailed usage examples.
pub use skills_integration::{SkillsCopyResult, SkillsError, SkillsValidation};
