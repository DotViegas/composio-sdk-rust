//! Toolkits management
//!
//! This module provides functionality to manage toolkits in Composio.
//! Toolkits are collections of tools that can be used to perform various tasks.
//! They're conceptualized as a set of tools. Ex: Github toolkit can perform
//! Github actions via its collection of tools.
//!
//! # Overview
//!
//! Toolkits represent integration points with external services. Each toolkit
//! contains a collection of related tools and triggers.

use serde::{Deserialize, Serialize};

/// Toolkit list parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolkitListParams {
    /// Filter by category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort by usage or alphabetically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortBy>,

    /// Filter by management type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,

    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Show deprecated toolkits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_deprecated: Option<bool>,
}

/// Sort by options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    /// Sort by usage
    Usage,
    /// Sort alphabetically
    Alphabetically,
}

/// Managed by options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManagedBy {
    /// Composio-managed toolkits
    Composio,
    /// All toolkits
    All,
    /// Project-managed toolkits
    Project,
}

/// Toolkit list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitListResponse {
    /// List of toolkits
    pub items: Vec<ToolkitItem>,

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

/// Toolkit item in list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitItem {
    /// Toolkit slug
    pub slug: String,

    /// Toolkit name
    pub name: String,

    /// Toolkit description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Toolkit logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    /// Supported authentication schemes
    #[serde(default)]
    pub auth_schemes: Vec<String>,

    /// Composio-managed authentication schemes
    #[serde(default)]
    pub composio_managed_auth_schemes: Vec<String>,

    /// Whether authentication is required
    #[serde(default)]
    pub no_auth: bool,

    /// Toolkit metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ToolkitMeta>,

    /// Whether this is a local toolkit (deprecated)
    #[serde(default)]
    pub is_local_toolkit: bool,
}

/// Toolkit metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitMeta {
    /// Toolkit description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Toolkit logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    /// Toolkit categories
    #[serde(default)]
    pub categories: Vec<String>,

    /// Number of tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_count: Option<u32>,

    /// Number of triggers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers_count: Option<u32>,

    /// Toolkit version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Toolkit retrieve response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitRetrieveResponse {
    /// Toolkit slug
    pub slug: String,

    /// Toolkit name
    pub name: String,

    /// Toolkit description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Toolkit logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    /// Supported authentication schemes
    #[serde(default)]
    pub auth_schemes: Vec<String>,

    /// Composio-managed authentication schemes
    #[serde(default)]
    pub composio_managed_auth_schemes: Vec<String>,

    /// Whether authentication is required
    #[serde(default)]
    pub no_auth: bool,

    /// Toolkit metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ToolkitMeta>,

    /// Authentication configuration details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_details: Option<Vec<AuthConfigDetail>>,

    /// Base URL for API requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Endpoint to get current user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_current_user_endpoint: Option<String>,
}

/// Authentication configuration detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigDetail {
    /// Authentication mode/scheme
    pub mode: String,

    /// Authentication fields
    pub fields: AuthConfigFields,

    /// Whether this is the default auth scheme
    #[serde(default)]
    pub is_default: bool,
}

/// Authentication configuration fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigFields {
    /// Fields for connected account initiation
    pub connected_account_initiation: AuthFieldSet,

    /// Fields for auth config creation
    pub auth_config_creation: AuthFieldSet,
}

/// Set of authentication fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFieldSet {
    /// Required fields
    #[serde(default)]
    pub required: Vec<AuthField>,

    /// Optional fields
    #[serde(default)]
    pub optional: Vec<AuthField>,
}

/// Authentication field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthField {
    /// Field name
    pub name: String,

    /// Field display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Field type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,

    /// Whether field is required
    #[serde(default)]
    pub required: bool,

    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Expected values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_values: Option<Vec<String>>,
}

/// Toolkit categories response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitCategoriesResponse {
    /// List of categories
    pub items: Vec<ToolkitCategory>,
}

/// Toolkit category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitCategory {
    /// Category name
    pub name: String,

    /// Category display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Number of toolkits in this category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Authorization parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeParams {
    /// User ID to authorize
    pub user_id: String,

    /// Toolkit slug
    pub toolkit: String,

    /// Optional auth config ID (if not provided, will be auto-created)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolkit_list_params_default() {
        let params = ToolkitListParams::default();
        assert!(params.category.is_none());
        assert!(params.cursor.is_none());
        assert!(params.limit.is_none());
    }

    #[test]
    fn test_sort_by_serialization() {
        let sort = SortBy::Usage;
        let json = serde_json::to_string(&sort).unwrap();
        assert_eq!(json, "\"usage\"");

        let sort = SortBy::Alphabetically;
        let json = serde_json::to_string(&sort).unwrap();
        assert_eq!(json, "\"alphabetically\"");
    }

    #[test]
    fn test_managed_by_serialization() {
        let managed = ManagedBy::Composio;
        let json = serde_json::to_string(&managed).unwrap();
        assert_eq!(json, "\"composio\"");

        let managed = ManagedBy::All;
        let json = serde_json::to_string(&managed).unwrap();
        assert_eq!(json, "\"all\"");

        let managed = ManagedBy::Project;
        let json = serde_json::to_string(&managed).unwrap();
        assert_eq!(json, "\"project\"");
    }

    #[test]
    fn test_toolkit_item_deserialization() {
        let json = r#"{
            "slug": "github",
            "name": "GitHub",
            "description": "GitHub integration",
            "logo": "https://example.com/logo.png",
            "auth_schemes": ["OAUTH2"],
            "composio_managed_auth_schemes": ["OAUTH2"],
            "no_auth": false,
            "is_local_toolkit": false
        }"#;

        let item: ToolkitItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.slug, "github");
        assert_eq!(item.name, "GitHub");
        assert_eq!(item.auth_schemes.len(), 1);
        assert!(!item.no_auth);
    }

    #[test]
    fn test_toolkit_meta() {
        let meta = ToolkitMeta {
            description: Some("Test toolkit".to_string()),
            logo: Some("https://example.com/logo.png".to_string()),
            categories: vec!["development".to_string()],
            tools_count: Some(50),
            triggers_count: Some(10),
            version: Some("1.0.0".to_string()),
        };

        assert_eq!(meta.tools_count, Some(50));
        assert_eq!(meta.triggers_count, Some(10));
        assert_eq!(meta.categories.len(), 1);
    }

    #[test]
    fn test_auth_field() {
        let field = AuthField {
            name: "client_id".to_string(),
            display_name: Some("Client ID".to_string()),
            description: Some("OAuth client ID".to_string()),
            field_type: Some("string".to_string()),
            required: true,
            default: None,
            expected_values: None,
        };

        assert_eq!(field.name, "client_id");
        assert!(field.required);
    }

    #[test]
    fn test_auth_field_set() {
        let field_set = AuthFieldSet {
            required: vec![AuthField {
                name: "api_key".to_string(),
                display_name: None,
                description: None,
                field_type: Some("string".to_string()),
                required: true,
                default: None,
                expected_values: None,
            }],
            optional: vec![],
        };

        assert_eq!(field_set.required.len(), 1);
        assert_eq!(field_set.optional.len(), 0);
    }

    #[test]
    fn test_toolkit_retrieve_response_deserialization() {
        let json = r#"{
            "slug": "github",
            "name": "GitHub",
            "auth_schemes": ["OAUTH2"],
            "composio_managed_auth_schemes": ["OAUTH2"],
            "no_auth": false
        }"#;

        let response: ToolkitRetrieveResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.slug, "github");
        assert_eq!(response.name, "GitHub");
        assert_eq!(response.auth_schemes.len(), 1);
    }

    #[test]
    fn test_authorize_params() {
        let params = AuthorizeParams {
            user_id: "user_123".to_string(),
            toolkit: "github".to_string(),
            auth_config_id: Some("ac_456".to_string()),
        };

        assert_eq!(params.user_id, "user_123");
        assert_eq!(params.toolkit, "github");
        assert!(params.auth_config_id.is_some());
    }
}
