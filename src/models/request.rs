//! Request models for Composio API
//!
//! This module contains all request body structures used when making API calls
//! to the Composio Tool Router. These models are serialized to JSON and sent
//! in HTTP request bodies.
//!
//! # Main Request Types
//!
//! - [`SessionConfig`] - Configuration for creating a Tool Router session
//! - [`ToolExecutionRequest`] - Request to execute a tool
//! - [`MetaToolExecutionRequest`] - Request to execute a meta tool
//! - [`LinkRequest`] - Request to create an authentication link
//!
//! # Configuration Types
//!
//! - [`ToolkitFilter`] - Enable or disable specific toolkits
//! - [`ToolsConfig`] - Per-toolkit tool filtering
//! - [`ToolFilter`] - Enable or disable specific tools within a toolkit
//! - [`TagsConfig`] - Tag-based tool filtering (readOnlyHint, destructiveHint, etc.)
//! - [`WorkbenchConfig`] - Workbench execution settings
//! - [`ManageConnectionsConfig`] - Connection management settings
//!
//! # Example
//!
//! ```rust
//! use composio_sdk::models::{SessionConfig, ToolkitFilter};
//!
//! let config = SessionConfig {
//!     user_id: "user_123".to_string(),
//!     toolkits: Some(ToolkitFilter::Enable(vec!["github".to_string()])),
//!     auth_configs: None,
//!     connected_accounts: None,
//!     manage_connections: None,
//!     tools: None,
//!     tags: None,
//!     workbench: None,
//! };
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::enums::{AuthScheme, MetaToolSlug, TagType};

/// Configuration for creating a Tool Router session
///
/// This struct defines all the options available when creating a new session.
/// Sessions provide scoped access to tools and toolkits for a specific user.
///
/// # Fields
///
/// * `user_id` - User identifier for session isolation (required)
/// * `toolkits` - Optional toolkit filter (enable or disable specific toolkits)
/// * `auth_configs` - Optional per-toolkit auth config overrides
/// * `connected_accounts` - Optional per-toolkit connected account selection
/// * `manage_connections` - Optional connection management configuration
/// * `tools` - Optional per-toolkit tool filtering
/// * `tags` - Optional tag-based tool filtering
/// * `workbench` - Optional workbench configuration
/// * `experimental` - Optional experimental features configuration
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::{SessionConfig, ToolkitFilter};
/// use std::collections::HashMap;
///
/// let config = SessionConfig {
///     user_id: "user_123".to_string(),
///     toolkits: Some(ToolkitFilter::Enable(vec!["github".to_string(), "gmail".to_string()])),
///     auth_configs: {
///         let mut map = HashMap::new();
///         map.insert("github".to_string(), "ac_custom_config".to_string());
///         Some(map)
///     },
///     connected_accounts: None,
///     manage_connections: None,
///     tools: None,
///     tags: None,
///     workbench: None,
///     experimental: None,
///     toolkit_versions: None,
///  };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<ToolkitFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_configs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_accounts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_connections: Option<ManageConnectionsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workbench: Option<WorkbenchConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<ExperimentalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<super::versioning::ToolkitVersionParam>,
}

/// Configuration for connection management
///
/// Controls whether the agent automatically prompts users with Connect Links
/// during chat when authentication is needed (in-chat authentication).
///
/// # Variants
///
/// * `Bool(bool)` - Simple boolean flag (true = enabled, false = disabled)
/// * `Detailed` - Detailed configuration with additional options
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::ManageConnectionsConfig;
///
/// // Simple boolean
/// let simple = ManageConnectionsConfig::Bool(true);
///
/// // Detailed configuration
/// let detailed = ManageConnectionsConfig::Detailed {
///     enabled: true,
///     enable_wait_for_connections: Some(true),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManageConnectionsConfig {
    /// Simple boolean flag
    Bool(bool),
    /// Detailed configuration
    Detailed {
        enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        enable_wait_for_connections: Option<bool>,
    },
}

/// Toolkit filter for enabling or disabling toolkits
///
/// Controls which toolkits are accessible in a session. By default, all toolkits
/// are accessible via COMPOSIO_SEARCH_TOOLS. Use this filter to restrict access.
///
/// # Variants
///
/// * `Enable(Vec<String>)` - Only allow specified toolkits (allowlist)
/// * `Disable { disable: Vec<String> }` - Allow all except specified toolkits (denylist)
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::ToolkitFilter;
///
/// // Enable only GitHub and Gmail
/// let enable = ToolkitFilter::Enable(vec!["github".to_string(), "gmail".to_string()]);
///
/// // Disable Exa and Firecrawl
/// let disable = ToolkitFilter::Disable {
///     disable: vec!["exa".to_string(), "firecrawl".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolkitFilter {
    Enable(Vec<String>),
    Disable { disable: Vec<String> },
}

/// Configuration for per-toolkit tool filtering
/// Maps toolkit names to their tool filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig(pub HashMap<String, ToolFilter>);

/// Tool filter for a specific toolkit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolFilter {
    /// Enable specific tools
    Enable { enable: Vec<String> },
    /// Disable specific tools
    Disable { disable: Vec<String> },
    /// Shorthand: array of tool names to enable
    EnableList(Vec<String>),
}

/// Configuration for tag-based tool filtering
/// Tags are MCP annotation hints for filtering tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsConfig {
    /// Tags that the tool must have at least one of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<TagType>>,
    /// Tags that the tool must NOT have any of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<TagType>>,
}

/// Configuration for workbench
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "proxy_execution_enabled")]
    pub proxy_execution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_offload_threshold: Option<u32>,
}

/// Configuration for assistive prompt generation
///
/// Experimental feature for generating timezone-aware assistive prompts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistivePromptConfig {
    /// IANA timezone identifier (e.g., "America/New_York", "Europe/London")
    /// for timezone-aware assistive prompts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_timezone: Option<String>,
}

/// Experimental configuration for Tool Router sessions
///
/// Note: These features are experimental and may be modified or removed in future versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentalConfig {
    /// Configuration for assistive prompt generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistive_prompt: Option<AssistivePromptConfig>,
}

/// Request to execute a tool
///
/// This struct contains all parameters needed to execute a tool through the Composio API.
/// It supports various authentication methods and execution modes.
///
/// # Fields
///
/// * `tool_slug` - The slug of the tool to execute (required)
/// * `arguments` - Arguments to pass to the tool (optional)
/// * `connected_account_id` - ID of the connected account to use for authentication (optional)
/// * `custom_auth_params` - Custom authentication parameters (optional)
/// * `custom_connection_data` - Custom connection data, takes priority over custom_auth_params (optional)
/// * `user_id` - User ID to execute the tool for (optional)
/// * `text` - Natural language text to pass to the tool (optional, mutually exclusive with arguments)
/// * `version` - Version of the tool to execute (optional, overrides SDK-level toolkit versions)
/// * `dangerously_skip_version_check` - Skip version check for 'latest' version (optional, dangerous!)
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::ToolExecutionRequest;
/// use serde_json::json;
///
/// let request = ToolExecutionRequest {
///     tool_slug: "GITHUB_CREATE_ISSUE".to_string(),
///     arguments: Some(json!({
///         "owner": "composio",
///         "repo": "composio",
///         "title": "Test issue"
///     })),
///     user_id: Some("user_123".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct ToolExecutionRequest {
    /// Tool slug to execute
    pub tool_slug: String,
    
    /// Arguments to pass to the tool (mutually exclusive with text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    
    /// Connected account ID to use for authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    
    /// Custom authentication parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth_params: Option<serde_json::Value>,
    
    /// Custom connection data (takes priority over custom_auth_params)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connection_data: Option<serde_json::Value>,
    
    /// User ID to execute the tool for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    
    /// Natural language text to pass to the tool (mutually exclusive with arguments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    
    /// Version of the tool to execute (overrides SDK-level toolkit versions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    /// Skip version check for 'latest' version (dangerous - may cause unexpected behavior)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_skip_version_check: Option<bool>,
}

/// Request to execute a meta tool
#[derive(Debug, Clone, Serialize)]
pub struct MetaToolExecutionRequest {
    pub slug: MetaToolSlug,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}

/// Request to create an authentication link
#[derive(Debug, Clone, Serialize)]
pub struct LinkRequest {
    pub toolkit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

// ============================================================================
// Auth Config Request Types
// ============================================================================

/// Parameters for creating an authentication configuration
///
/// Auth configs define how users authenticate with external services.
/// They can use Composio's managed auth or custom OAuth apps.
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::{AuthConfigCreateParams, AuthConfigData, AuthScheme};
/// use serde_json::json;
///
/// let params = AuthConfigCreateParams {
///     toolkit: "github".to_string(),
///     auth_config: AuthConfigData {
///         auth_type: AuthScheme::Oauth2,
///         credentials: json!({
///             "client_id": "your_client_id",
///             "client_secret": "your_client_secret",
///             "scopes": ["repo", "user"]
///         }),
///         restrict_to_following_tools: None,
///     },
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigCreateParams {
    /// Toolkit slug (e.g., "github", "gmail")
    pub toolkit: String,
    /// Authentication configuration data
    pub auth_config: AuthConfigData,
}

/// Authentication configuration data
#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigData {
    /// Type of authentication scheme
    #[serde(rename = "type")]
    pub auth_type: AuthScheme,
    /// Credentials for the authentication (structure varies by auth_type)
    pub credentials: serde_json::Value,
    /// Optional list of tool slugs to restrict this auth config to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_to_following_tools: Option<Vec<String>>,
}

/// Parameters for listing authentication configurations
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::AuthConfigListParams;
///
/// let params = AuthConfigListParams {
///     is_composio_managed: Some(false),
///     toolkit_slug: Some("github".to_string()),
///     show_disabled: Some(false),
///     search: None,
///     limit: Some(20),
///     cursor: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct AuthConfigListParams {
    /// Filter by Composio-managed auth configs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_composio_managed: Option<bool>,
    /// Filter by toolkit slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slug: Option<String>,
    /// Include disabled auth configs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_disabled: Option<bool>,
    /// Search by name or ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Parameters for updating an authentication configuration
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::AuthConfigUpdateParams;
/// use serde_json::json;
///
/// let params = AuthConfigUpdateParams {
///     name: Some("My GitHub App".to_string()),
///     credentials: Some(json!({
///         "scopes": ["repo", "user", "admin:org"]
///     })),
///     proxy_config: None,
///     tool_access_config: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct AuthConfigUpdateParams {
    /// New name for the auth config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
    /// Proxy configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_config: Option<serde_json::Value>,
    /// Tool access configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_access_config: Option<serde_json::Value>,
}

// ============================================================================
// Connected Account Request Types
// ============================================================================

/// Parameters for creating a connected account
///
/// Connected accounts represent user connections to external services.
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::{ConnectedAccountCreateParams, AuthConfigReference, ConnectionData};
///
/// let params = ConnectedAccountCreateParams {
///     auth_config: AuthConfigReference {
///         id: "ac_abc123".to_string(),
///     },
///     connection: ConnectionData {
///         state: None,
///         data: None,
///         user_id: "user_123".to_string(),
///         callback_url: Some("https://myapp.com/callback".to_string()),
///     },
///     validate_credentials: Some(true),
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ConnectedAccountCreateParams {
    /// Reference to the auth config to use
    pub auth_config: AuthConfigReference,
    /// Connection data
    pub connection: ConnectionData,
    /// Whether to validate credentials immediately
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_credentials: Option<bool>,
}

/// Reference to an authentication configuration
#[derive(Debug, Clone, Serialize)]
pub struct AuthConfigReference {
    /// Auth config ID
    pub id: String,
}

/// Connection data for creating a connected account
#[derive(Debug, Clone, Serialize)]
pub struct ConnectionData {
    /// Connection state (varies by auth scheme)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    /// Additional connection data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// User ID this connection belongs to
    pub user_id: String,
    /// Callback URL for OAuth flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

/// Parameters for listing connected accounts
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::ConnectedAccountListParams;
///
/// let params = ConnectedAccountListParams {
///     toolkit_slugs: Some(vec!["github".to_string(), "gmail".to_string()]),
///     statuses: Some(vec!["ACTIVE".to_string()]),
///     user_ids: Some(vec!["user_123".to_string()]),
///     cursor: None,
///     limit: Some(50),
///     auth_config_ids: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Default)]
pub struct ConnectedAccountListParams {
    /// Filter by toolkit slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slugs: Option<Vec<String>>,
    /// Filter by connection statuses (ACTIVE, EXPIRED, FAILED, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Filter by user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// Filter by auth config IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_ids: Option<Vec<String>>,
}

// ============================================================================
// Tool Proxy Request Types
// ============================================================================

/// Parameters for executing a proxy request
///
/// Proxy requests allow you to make authenticated API calls to external services
/// without predefined tool schemas.
///
/// # Example
///
/// ```rust
/// use composio_sdk::models::ToolProxyParams;
/// use serde_json::json;
///
/// let params = ToolProxyParams {
///     endpoint: "/repos/owner/repo/issues".to_string(),
///     method: Some("POST".to_string()),
///     headers: Some(json!({"Accept": "application/vnd.github.v3+json"})),
///     body: Some(json!({"title": "Bug report", "body": "Description"})),
///     query_params: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ToolProxyParams {
    /// API endpoint (relative or absolute URL)
    pub endpoint: String,
    /// HTTP method (GET, POST, PUT, DELETE, PATCH, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Custom headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// Request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    /// Query parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_session_config_minimal_serialization() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: None,
            connected_accounts: None,
            manage_connections: None,
            tools: None,
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("user_123"));
        assert!(!json.contains("toolkits"));
        assert!(!json.contains("auth_configs"));
    }

    #[test]
    fn test_session_config_with_toolkits_enable() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: Some(ToolkitFilter::Enable(vec!["github".to_string(), "gmail".to_string()])),
            auth_configs: None,
            connected_accounts: None,
            manage_connections: None,
            tools: None,
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed["toolkits"].is_array());
        let toolkits = parsed["toolkits"].as_array().unwrap();
        assert_eq!(toolkits.len(), 2);
    }

    #[test]
    fn test_session_config_with_toolkits_disable() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: Some(ToolkitFilter::Disable {
                disable: vec!["exa".to_string(), "firecrawl".to_string()],
            }),
            auth_configs: None,
            connected_accounts: None,
            manage_connections: None,
            tools: None,
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed["toolkits"].is_object());
        assert!(parsed["toolkits"]["disable"].is_array());
    }

    #[test]
    fn test_session_config_with_auth_configs() {
        let mut auth_configs = HashMap::new();
        auth_configs.insert("github".to_string(), "ac_custom".to_string());
        
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: Some(auth_configs),
            connected_accounts: None,
            manage_connections: None,
            tools: None,
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["auth_configs"]["github"], "ac_custom");
    }

    #[test]
    fn test_session_config_with_manage_connections_bool() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: None,
            connected_accounts: None,
            manage_connections: Some(ManageConnectionsConfig::Bool(true)),
            tools: None,
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["manage_connections"], true);
    }

    #[test]
    fn test_session_config_with_manage_connections_detailed() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: None,
            connected_accounts: None,
            manage_connections: Some(ManageConnectionsConfig::Detailed {
                enabled: true,
                enable_wait_for_connections: Some(false),
            }),
            tools: None,
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["manage_connections"]["enabled"], true);
        assert_eq!(parsed["manage_connections"]["enable_wait_for_connections"], false);
    }

    #[test]
    fn test_session_config_with_tools() {
        let mut tools_map = HashMap::new();
        tools_map.insert(
            "github".to_string(),
            ToolFilter::EnableList(vec!["GITHUB_CREATE_ISSUE".to_string()]),
        );
        
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: None,
            connected_accounts: None,
            manage_connections: None,
            tools: Some(ToolsConfig(tools_map)),
            tags: None,
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed["tools"]["github"].is_array());
    }

    #[test]
    fn test_session_config_with_tags() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: None,
            connected_accounts: None,
            manage_connections: None,
            tools: None,
            tags: Some(TagsConfig {
                enabled: Some(vec![TagType::ReadOnlyHint]),
                disabled: Some(vec![TagType::DestructiveHint]),
            }),
            workbench: None,
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed["tags"]["enabled"].is_array());
        assert!(parsed["tags"]["disabled"].is_array());
    }

    #[test]
    fn test_session_config_with_workbench() {
        let config = SessionConfig {
            user_id: "user_123".to_string(),
            toolkits: None,
            auth_configs: None,
            connected_accounts: None,
            manage_connections: None,
            tools: None,
            tags: None,
            workbench: Some(WorkbenchConfig {
                proxy_execution: Some(true),
                auto_offload_threshold: Some(1000),
            }),
            experimental: None,
            toolkit_versions: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["workbench"]["proxy_execution"], true);
        assert_eq!(parsed["workbench"]["auto_offload_threshold"], 1000);
    }

    #[test]
    fn test_toolkit_filter_enable_serialization() {
        let filter = ToolkitFilter::Enable(vec!["github".to_string(), "gmail".to_string()]);
        let json = serde_json::to_string(&filter).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_toolkit_filter_disable_serialization() {
        let filter = ToolkitFilter::Disable {
            disable: vec!["exa".to_string()],
        };
        let json = serde_json::to_string(&filter).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.is_object());
        assert!(parsed["disable"].is_array());
    }

    #[test]
    fn test_tool_filter_enable_serialization() {
        let filter = ToolFilter::Enable {
            enable: vec!["GITHUB_CREATE_ISSUE".to_string()],
        };
        let json = serde_json::to_string(&filter).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.is_object());
        assert!(parsed["enable"].is_array());
    }

    #[test]
    fn test_tool_filter_disable_serialization() {
        let filter = ToolFilter::Disable {
            disable: vec!["GITHUB_DELETE_REPO".to_string()],
        };
        let json = serde_json::to_string(&filter).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.is_object());
        assert!(parsed["disable"].is_array());
    }

    #[test]
    fn test_tool_filter_enable_list_serialization() {
        let filter = ToolFilter::EnableList(vec!["GITHUB_CREATE_ISSUE".to_string()]);
        let json = serde_json::to_string(&filter).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.is_array());
    }

    #[test]
    fn test_tool_execution_request_serialization() {
        let request = ToolExecutionRequest {
            tool_slug: "GITHUB_CREATE_ISSUE".to_string(),
            arguments: Some(serde_json::json!({
                "owner": "composio",
                "repo": "composio",
                "title": "Test issue"
            })),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["tool_slug"], "GITHUB_CREATE_ISSUE");
        assert!(parsed["arguments"].is_object());
        assert_eq!(parsed["arguments"]["owner"], "composio");
    }

    #[test]
    fn test_tool_execution_request_without_arguments() {
        let request = ToolExecutionRequest {
            tool_slug: "GITHUB_GET_USER".to_string(),
            arguments: None,
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["tool_slug"], "GITHUB_GET_USER");
        assert!(parsed.get("arguments").is_none());
    }

    #[test]
    fn test_meta_tool_execution_request_serialization() {
        let request = MetaToolExecutionRequest {
            slug: MetaToolSlug::ComposioSearchTools,
            arguments: Some(serde_json::json!({
                "query": "create a GitHub issue"
            })),
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["slug"], "COMPOSIO_SEARCH_TOOLS");
        assert!(parsed["arguments"].is_object());
    }

    #[test]
    fn test_link_request_serialization() {
        let request = LinkRequest {
            toolkit: "github".to_string(),
            callback_url: Some("https://example.com/callback".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["toolkit"], "github");
        assert_eq!(parsed["callback_url"], "https://example.com/callback");
    }

    #[test]
    fn test_link_request_without_callback() {
        let request = LinkRequest {
            toolkit: "gmail".to_string(),
            callback_url: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["toolkit"], "gmail");
        assert!(parsed.get("callback_url").is_none());
    }

    #[test]
    fn test_tags_config_serialization() {
        let config = TagsConfig {
            enabled: Some(vec![TagType::ReadOnlyHint, TagType::IdempotentHint]),
            disabled: Some(vec![TagType::DestructiveHint]),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert!(parsed["enabled"].is_array());
        assert!(parsed["disabled"].is_array());
        assert_eq!(parsed["enabled"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["disabled"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_workbench_config_serialization() {
        let config = WorkbenchConfig {
            proxy_execution: Some(true),
            auto_offload_threshold: Some(500),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["proxy_execution"], true);
        assert_eq!(parsed["auto_offload_threshold"], 500);
    }

    #[test]
    fn test_workbench_config_partial_serialization() {
        let config = WorkbenchConfig {
            proxy_execution: Some(false),
            auto_offload_threshold: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["proxy_execution"], false);
        assert!(parsed.get("auto_offload_threshold").is_none());
    }
}
