//! Authentication configuration management
//!
//! This module provides functionality to manage authentication configurations
//! for toolkits in the Composio platform.
//!
//! Note: This is a translation of the Python SDK's auth_configs.py module.
//! Full HTTP client integration is pending.

use serde::{Deserialize, Serialize};

/// Auth config status used for update-status path operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuthConfigStatus {
    /// Enable auth config
    Enabled,
    /// Disable auth config
    Disabled,
}

impl AuthConfigStatus {
    /// Return wire-format path segment value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "ENABLED",
            Self::Disabled => "DISABLED",
        }
    }
}

/// Authentication configuration list parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthConfigListParams {
    /// Filter by whether the auth config is Composio-managed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_composio_managed: Option<bool>,

    /// Filter by toolkit slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slug: Option<String>,

    /// Whether to show disabled auth configs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_disabled: Option<bool>,

    /// Search query for filtering by name or ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Authentication configuration list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigListResponse {
    /// List of auth configs
    pub items: Vec<AuthConfigInfo>,

    /// Next cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    /// Total number of pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u32>,

    /// Current page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<u32>,

    /// Total number of items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<u32>,
}

/// Authentication configuration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigInfo {
    /// Unique identifier (NanoID)
    pub id: String,

    /// UUID identifier (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    /// Type of auth config
    #[serde(rename = "type")]
    pub config_type: String,

    /// Toolkit information
    pub toolkit: ToolkitInfo,

    /// Name of the auth config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Authentication scheme
    pub auth_scheme: String,

    /// Credentials (may be masked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,

    /// Status (ENABLED/DISABLED)
    pub status: String,

    /// Creation timestamp
    pub created_at: String,

    /// Number of connections using this auth config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_of_connections: Option<u32>,

    /// Tool access configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_access_config: Option<serde_json::Value>,
}

/// Toolkit information in auth config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitInfo {
    /// Toolkit slug
    pub slug: String,

    /// Toolkit name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Authentication configuration create parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigCreateParams {
    /// Toolkit slug
    pub toolkit: String,

    /// Authentication configuration options
    pub options: AuthConfigOptions,
}

/// Authentication configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthConfigOptions {
    /// Custom auth config with full credentials
    #[serde(rename = "custom")]
    Custom {
        /// Authentication scheme
        auth_scheme: String,

        /// Credentials for the auth config
        credentials: serde_json::Value,

        /// Tool access restrictions
        #[serde(skip_serializing_if = "Option::is_none")]
        restrict_to_following_tools: Option<Vec<String>>,
    },

    /// Default auth config with scopes only
    #[serde(rename = "default")]
    Default {
        /// OAuth scopes
        #[serde(skip_serializing_if = "Option::is_none")]
        scopes: Option<Vec<String>>,

        /// User-level scopes
        #[serde(skip_serializing_if = "Option::is_none")]
        user_scopes: Option<Vec<String>>,

        /// Tool access restrictions
        #[serde(skip_serializing_if = "Option::is_none")]
        restrict_to_following_tools: Option<Vec<String>>,
    },
}

/// Authentication configuration create response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigCreateResponse {
    /// Created auth config
    pub auth_config: AuthConfig,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Unique identifier
    pub id: String,

    /// Toolkit information
    pub toolkit: ToolkitInfo,

    /// Authentication scheme
    pub auth_scheme: String,

    /// Whether this is Composio-managed
    pub is_composio_managed: bool,

    /// Tool access restrictions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_to_following_tools: Option<Vec<String>>,
}

/// Authentication configuration retrieve response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigRetrieveResponse {
    /// Auth config ID
    pub id: String,

    /// UUID (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    /// Type of auth config
    #[serde(rename = "type")]
    pub config_type: String,

    /// Toolkit information
    pub toolkit: ToolkitInfo,

    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Authentication scheme
    pub auth_scheme: String,

    /// Credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,

    /// Proxy configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_config: Option<serde_json::Value>,

    /// Expected input fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_input_fields: Option<Vec<String>>,

    /// Shared credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_credentials: Option<serde_json::Value>,
}

/// Authentication configuration update parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthConfigUpdateParams {
    /// Update custom auth config
    #[serde(rename = "custom")]
    Custom {
        /// Updated credentials
        #[serde(skip_serializing_if = "Option::is_none")]
        credentials: Option<serde_json::Value>,

        /// Updated proxy config
        #[serde(skip_serializing_if = "Option::is_none")]
        proxy_config: Option<serde_json::Value>,

        /// Updated tool access config
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_access_config: Option<serde_json::Value>,

        /// Updated shared credentials
        #[serde(skip_serializing_if = "Option::is_none")]
        shared_credentials: Option<serde_json::Value>,

        /// Whether enabled for tool router
        #[serde(skip_serializing_if = "Option::is_none")]
        is_enabled_for_tool_router: Option<bool>,
    },

    /// Update default auth config
    #[serde(rename = "default")]
    Default {
        /// Updated credentials (scopes)
        #[serde(skip_serializing_if = "Option::is_none")]
        credentials: Option<DefaultCredentials>,

        /// Whether enabled for tool router
        #[serde(skip_serializing_if = "Option::is_none")]
        is_enabled_for_tool_router: Option<bool>,
    },
}

/// Default credentials (scopes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultCredentials {
    /// OAuth scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// User-level scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_scopes: Option<Vec<String>>,
}

/// Authentication configuration update response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigUpdateResponse {
    /// Success status
    pub success: bool,

    /// Updated auth config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<AuthConfigInfo>,
}

/// Authentication configuration delete response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigDeleteResponse {
    /// Success status
    pub success: bool,

    /// Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Authentication configuration status update response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigStatusUpdateResponse {
    /// Success status
    pub success: bool,

    /// Updated status
    pub status: String,
}

// Note: The AuthConfigs resource implementation is pending full HTTP client integration.
// The data structures above are ready for use once the client supports generic HTTP methods.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_list_params_default() {
        let params = AuthConfigListParams::default();
        assert!(params.is_composio_managed.is_none());
        assert!(params.toolkit_slug.is_none());
    }

    #[test]
    fn test_auth_config_options_custom_serialization() {
        let options = AuthConfigOptions::Custom {
            auth_scheme: "OAUTH2".to_string(),
            credentials: serde_json::json!({"client_id": "test"}),
            restrict_to_following_tools: None,
        };

        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains("custom"));
        assert!(json.contains("OAUTH2"));
    }

    #[test]
    fn test_auth_config_status_serialization() {
        let enabled = AuthConfigStatus::Enabled;
        let json = serde_json::to_string(&enabled).unwrap();
        assert_eq!(json, "\"ENABLED\"");
        assert_eq!(enabled.as_str(), "ENABLED");
    }

    #[test]
    fn test_auth_config_update_response_deserialization() {
        let payload = r#"{"success":true}"#;
        let response: AuthConfigUpdateResponse = serde_json::from_str(payload).unwrap();
        assert!(response.success);
    }

    #[test]
    fn test_auth_config_delete_response_deserialization() {
        let payload = r#"{"success":true,"message":"deleted"}"#;
        let response: AuthConfigDeleteResponse = serde_json::from_str(payload).unwrap();
        assert!(response.success);
        assert_eq!(response.message.as_deref(), Some("deleted"));
    }

    #[test]
    fn test_auth_config_status_update_response_deserialization() {
        let payload = r#"{"success":true,"status":"DISABLED"}"#;
        let response: AuthConfigStatusUpdateResponse = serde_json::from_str(payload).unwrap();
        assert!(response.success);
        assert_eq!(response.status, "DISABLED");
    }

    #[test]
    fn test_auth_config_options_default_serialization() {
        let options = AuthConfigOptions::Default {
            scopes: Some(vec!["repo".to_string()]),
            user_scopes: None,
            restrict_to_following_tools: None,
        };

        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains("default"));
        assert!(json.contains("repo"));
    }
}
