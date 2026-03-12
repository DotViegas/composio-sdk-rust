//! MCP (Model Context Protocol) module for Composio SDK.
//!
//! This module provides MCP server operations for creating, managing, and generating
//! MCP server instances for users.
//!
//! # Overview
//!
//! MCP servers provide connection points for AI assistants to access applications.
//! This module allows you to:
//! - Create MCP server configurations with specific toolkits
//! - List and filter existing MCP servers
//! - Update MCP server configurations
//! - Delete MCP servers
//! - Generate user-specific MCP server URLs
//!
//! # Example
//!
//! ```rust,no_run
//! use composio_sdk::{Composio, models::mcp::MCPToolkitConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let composio = Composio::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! // Create an MCP server
//! let server = composio.mcp().create(
//!     "my-mcp-server",
//!     vec!["github".to_string(), "slack".to_string()],
//!     None,
//!     None,
//! ).await?;
//!
//! // Generate a user-specific URL
//! let instance = composio.mcp().generate(
//!     "user_123",
//!     &server.id,
//!     None,
//! ).await?;
//!
//! println!("MCP URL: {}", instance.url);
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Data Types (matching TypeScript/Python specification)
// ============================================================================

/// MCP toolkit configuration
///
/// Specifies a toolkit and optionally an auth config to use for the MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPToolkitConfig {
    /// Toolkit slug (e.g., "github", "slack")
    pub toolkit: String,

    /// Optional auth config ID to use for this toolkit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_id: Option<String>,
}

impl MCPToolkitConfig {
    /// Create a new toolkit configuration
    pub fn new(toolkit: impl Into<String>) -> Self {
        Self {
            toolkit: toolkit.into(),
            auth_config_id: None,
        }
    }

    /// Set the auth config ID
    pub fn with_auth_config(mut self, auth_config_id: impl Into<String>) -> Self {
        self.auth_config_id = Some(auth_config_id.into());
        self
    }
}

impl From<String> for MCPToolkitConfig {
    fn from(toolkit: String) -> Self {
        Self::new(toolkit)
    }
}

impl From<&str> for MCPToolkitConfig {
    fn from(toolkit: &str) -> Self {
        Self::new(toolkit)
    }
}

/// MCP Server Instance data structure
///
/// Represents a user-specific MCP server instance with connection details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerInstance {
    /// Server instance ID
    pub id: String,

    /// Human-readable server name
    pub name: String,

    /// Server type (typically "streamable_http")
    #[serde(rename = "type")]
    pub server_type: String,

    /// User-specific connection URL
    pub url: String,

    /// Associated user ID
    pub user_id: String,

    /// Available tools for the user
    pub allowed_tools: Vec<String>,

    /// Associated auth configurations
    pub auth_configs: Vec<String>,
}

/// Complete MCP server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPItem {
    /// Unique server identifier
    pub id: String,

    /// Human-readable server name
    pub name: String,

    /// Array of enabled tool identifiers
    pub allowed_tools: Vec<String>,

    /// Array of auth configuration IDs
    pub auth_config_ids: Vec<String>,

    /// Array of toolkit names
    pub toolkits: Vec<String>,

    /// Setup commands for different clients
    pub commands: HashMap<String, String>,

    /// Server connection URL
    pub mcp_url: String,

    /// Map of toolkit icons
    pub toolkit_icons: HashMap<String, String>,

    /// Number of active instances
    pub server_instance_count: i32,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Last update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// Whether the server is soft-deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Whether auth is managed by Composio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_auth_via_composio: Option<bool>,
}

/// Paginated list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPListResponse {
    /// Array of MCP server objects
    pub items: Vec<MCPItem>,

    /// Current page number
    pub current_page: i32,

    /// Total number of pages
    pub total_pages: i32,
}

/// Response from creating an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPCreateResponse {
    /// Server ID
    pub id: String,

    /// Server name
    pub name: String,

    /// Allowed tools
    pub allowed_tools: Vec<String>,

    /// Auth config IDs
    pub auth_config_ids: Vec<String>,

    /// Toolkits
    pub toolkits: Vec<String>,

    /// MCP URL
    pub mcp_url: String,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Response from updating an MCP server
pub type MCPUpdateResponse = MCPCreateResponse;

/// Response from deleting an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPDeleteResponse {
    /// Server ID that was deleted
    pub id: String,

    /// Whether the deletion was successful
    pub deleted: bool,
}

/// Response from generating MCP URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPGenerateUrlResponse {
    /// Base MCP URL without query parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_url: Option<String>,

    /// Array of connected-account-specific URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_urls: Option<Vec<String>>,

    /// Array of user-specific URLs
    pub user_ids_url: Vec<String>,
}

/// Response from retrieving an app-scoped MCP server list
pub type MCPRetrieveAppResponse = MCPListResponse;

/// Response from creating a custom MCP server
pub type MCPCustomCreateResponse = MCPCreateResponse;

// ============================================================================
// Request Types
// ============================================================================

/// Parameters for creating an MCP server
#[derive(Debug, Clone, Serialize)]
pub struct MCPCreateParams {
    /// Server name
    pub name: String,

    /// Auth config IDs
    pub auth_config_ids: Vec<String>,

    /// Allowed tool slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,

    /// No-auth app slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_auth_apps: Option<Vec<String>>,

    /// DEPRECATED: custom toolkit slugs (use no_auth_apps/auth_config_ids)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(
        note = "Prefer no_auth_apps/auth_config_ids to match MCP create endpoint contract"
    )]
    pub toolkits: Option<Vec<String>>,

    /// DEPRECATED: custom tools (use allowed_tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Prefer allowed_tools to match MCP create endpoint contract")]
    pub custom_tools: Option<Vec<String>>,

    /// Whether to use Composio managed auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_auth_via_composio: Option<bool>,
}

/// Parameters for updating an MCP server
#[derive(Debug, Clone, Serialize)]
pub struct MCPUpdateParams {
    /// Optional new name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional toolkit slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<Vec<String>>,

    /// Optional auth config IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_ids: Option<Vec<String>>,

    /// Optional allowed tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,

    /// Optional custom tools (deprecated; alias of allowed tools behavior)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tools: Option<Vec<String>>,

    /// Optional managed auth flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_auth_via_composio: Option<bool>,
}

/// Parameters for listing MCP servers
#[derive(Debug, Clone, Default, Serialize)]
pub struct MCPListParams {
    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,

    /// Maximum items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Filter by toolkit name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<String>,

    /// Filter by auth configuration ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_ids: Option<String>,

    /// Filter by server name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Order by field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Order direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<String>,
}

/// Parameters for generating MCP URLs
#[derive(Debug, Clone, Serialize)]
pub struct MCPGenerateUrlParams {
    /// MCP server ID
    pub mcp_server_id: String,

    /// Connected account IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_ids: Option<Vec<String>>,

    /// User IDs to generate URLs for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,

    /// Whether to use Composio managed auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_auth_by_composio: Option<bool>,
}

/// Parameters for creating a custom MCP server
#[derive(Debug, Clone, Serialize)]
pub struct MCPCustomCreateParams {
    /// Server name
    pub name: String,

    /// Allowed tool slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,

    /// Auth config IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_ids: Option<Vec<String>>,

    /// Custom tools (deprecated; alias of allowed_tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tools: Option<Vec<String>>,

    /// Whether to use Composio managed auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_auth_via_composio: Option<bool>,

    /// Toolkit slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<Vec<String>>,
}

/// Query parameters for app-scoped MCP server listing
pub type MCPRetrieveAppParams = MCPListParams;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_toolkit_config_new() {
        let config = MCPToolkitConfig::new("github");
        assert_eq!(config.toolkit, "github");
        assert!(config.auth_config_id.is_none());
    }

    #[test]
    fn test_mcp_toolkit_config_with_auth() {
        let config = MCPToolkitConfig::new("github").with_auth_config("ac_123");

        assert_eq!(config.toolkit, "github");
        assert_eq!(config.auth_config_id, Some("ac_123".to_string()));
    }

    #[test]
    fn test_mcp_toolkit_config_from_string() {
        let config: MCPToolkitConfig = "slack".into();
        assert_eq!(config.toolkit, "slack");
        assert!(config.auth_config_id.is_none());
    }

    #[test]
    fn test_mcp_server_instance_serialization() {
        let instance = MCPServerInstance {
            id: "mcp_123".to_string(),
            name: "Test Server".to_string(),
            server_type: "streamable_http".to_string(),
            url: "https://mcp.composio.dev/test".to_string(),
            user_id: "user_123".to_string(),
            allowed_tools: vec!["GITHUB_CREATE_ISSUE".to_string()],
            auth_configs: vec!["ac_123".to_string()],
        };

        let json = serde_json::to_string(&instance).unwrap();
        assert!(json.contains("mcp_123"));
        assert!(json.contains("Test Server"));
        assert!(json.contains("streamable_http"));
    }

    #[test]
    fn test_mcp_server_instance_deserialization() {
        let json = r#"{
            "id": "mcp_456",
            "name": "My Server",
            "type": "streamable_http",
            "url": "https://mcp.url",
            "user_id": "user_456",
            "allowed_tools": ["SLACK_SEND_MESSAGE"],
            "auth_configs": ["ac_456"]
        }"#;

        let instance: MCPServerInstance = serde_json::from_str(json).unwrap();
        assert_eq!(instance.id, "mcp_456");
        assert_eq!(instance.name, "My Server");
        assert_eq!(instance.server_type, "streamable_http");
        assert_eq!(instance.url, "https://mcp.url");
        assert_eq!(instance.user_id, "user_456");
        assert_eq!(instance.allowed_tools.len(), 1);
        assert_eq!(instance.auth_configs.len(), 1);
    }

    #[test]
    fn test_mcp_list_params_default() {
        let params = MCPListParams::default();
        assert!(params.page_no.is_none());
        assert!(params.limit.is_none());
        assert!(params.toolkits.is_none());
        assert!(params.auth_config_ids.is_none());
        assert!(params.name.is_none());
    }

    #[test]
    fn test_mcp_create_params_serialization() {
        let params = MCPCreateParams {
            name: "Test Server".to_string(),
            auth_config_ids: vec!["ac_123".to_string()],
            allowed_tools: Some(vec!["GITHUB_CREATE_ISSUE".to_string()]),
            no_auth_apps: Some(vec!["notion".to_string()]),
            toolkits: Some(vec!["github".to_string()]),
            custom_tools: Some(vec!["GITHUB_CREATE_ISSUE".to_string()]),
            managed_auth_via_composio: Some(true),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("Test Server"));
        assert!(json.contains("github"));
        assert!(json.contains("ac_123"));
        assert!(json.contains("allowed_tools"));
        assert!(json.contains("no_auth_apps"));
    }

    #[test]
    fn test_mcp_generate_params_serialization() {
        let params = MCPGenerateUrlParams {
            mcp_server_id: "mcp_123".to_string(),
            connected_account_ids: Some(vec!["ca_1".to_string()]),
            user_ids: Some(vec!["user_123".to_string()]),
            managed_auth_by_composio: Some(true),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("connected_account_ids"));
        assert!(json.contains("user_ids"));
    }

    #[test]
    fn test_mcp_generate_response_deserialization() {
        let json = r#"{
            "mcp_url": "https://mcp.example.com/base",
            "connected_account_urls": ["https://mcp.example.com?connected_account_id=ca_1"],
            "user_ids_url": ["https://mcp.example.com?user_id=user_123"]
        }"#;

        let response: MCPGenerateUrlResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.mcp_url.as_deref(),
            Some("https://mcp.example.com/base")
        );
        assert_eq!(response.connected_account_urls.unwrap().len(), 1);
        assert_eq!(response.user_ids_url.len(), 1);
    }

    #[test]
    fn test_mcp_delete_response_deserialization() {
        let json = r#"{
            "id": "mcp_789",
            "deleted": true
        }"#;

        let response: MCPDeleteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "mcp_789");
        assert!(response.deleted);
    }
}
