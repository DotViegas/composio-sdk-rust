//! Tools management
//!
//! This module provides functionality to manage and execute tools in Composio.
//! Tools are individual actions that can be performed on external services.
//!
//! # Overview
//!
//! Tools represent specific actions like "create GitHub issue", "send email", etc.
//! They can be:
//! - Composio-managed tools (from the platform)
//! - Custom tools (user-defined)
//! - Meta tools (for tool discovery and management)
//!
//! # Type Organization
//!
//! This module re-exports types from other modules to maintain a clean API:
//! - `ToolExecutionResponse` - from `models::response`
//! - `ToolExecuteParams` - from `models::modifiers`
//! - `CustomAuthParams` - from `models::modifiers`
//! - `CustomConnectionData` - from `models::modifiers`
//!
//! This organization follows the Python SDK's `types.py` pattern where common
//! types are re-exported from their canonical locations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export types from other modules to maintain compatibility
pub use crate::models::response::ToolExecutionResponse;
pub use crate::models::modifiers::{
    ToolExecuteParams, CustomAuthParams, CustomConnectionData
};

/// Tool list parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolListParams {
    /// Filter by specific tool slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_slugs: Option<Vec<String>>,

    /// Filter by toolkit slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slug: Option<String>,

    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Filter by scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// Filter by tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Filter by importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,

    /// Show deprecated tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_deprecated: Option<bool>,

    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Toolkit versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<String>,
}

/// Tool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    /// Tool slug
    pub slug: String,

    /// Tool name
    pub name: String,

    /// Tool description
    pub description: String,

    /// Toolkit information
    pub toolkit: ToolkitRef,

    /// Input parameters schema
    pub input_parameters: serde_json::Value,

    /// Output parameters schema
    pub output_parameters: serde_json::Value,

    /// Required OAuth scopes
    #[serde(default)]
    pub scopes: Vec<String>,

    /// Tool tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Tool version
    pub version: String,

    /// Available versions
    #[serde(default)]
    pub available_versions: Vec<String>,

    /// Whether the tool is deprecated
    #[serde(default)]
    pub is_deprecated: bool,

    /// Whether authentication is required
    #[serde(default)]
    pub no_auth: bool,
}

/// Toolkit reference in tool info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolkitRef {
    /// Toolkit slug
    pub slug: String,

    /// Toolkit name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Toolkit logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}

/// Tool list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResponse {
    /// List of tools
    pub items: Vec<ToolInfo>,

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

/// Tool proxy parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProxyParams {
    /// API endpoint to call
    pub endpoint: String,

    /// HTTP method
    pub method: HttpMethod,

    /// Request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,

    /// Connected account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,

    /// Additional parameters (headers, query params)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ProxyParameter>>,

    /// Custom connection data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connection_data: Option<CustomConnectionData>,
}

/// HTTP method enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

/// Proxy parameter (header or query param)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyParameter {
    /// Parameter name
    pub name: String,

    /// Parameter value
    pub value: String,

    /// Where to include the parameter
    #[serde(rename = "in")]
    pub location: ParameterLocation,
}

/// Parameter location enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    /// Include in headers
    Header,

    /// Include in query parameters
    Query,

    /// Include in path
    Path,

    /// Include in body
    Body,
}

/// Tool proxy response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProxyResponse {
    /// Response data
    pub data: serde_json::Value,

    /// HTTP status code
    pub status: u16,

    /// Response headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Binary data (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_data: Option<String>,

    /// Whether the request was successful
    pub successful: bool,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Tool input generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputGenerationParams {
    /// Tool slug
    pub tool_slug: String,

    /// Natural language description
    pub text: String,

    /// Custom tool description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tool_description: Option<String>,

    /// Custom system prompt (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_system_prompt: Option<String>,
}

/// Tool input generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputGenerationResponse {
    /// Generated arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,

    /// Error message if generation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Whether generation was successful
    pub successful: bool,
}

/// Custom tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomToolDefinition {
    /// Tool slug (unique identifier)
    pub slug: String,

    /// Tool name
    pub name: String,

    /// Tool description
    pub description: String,

    /// Input parameters schema
    pub input_schema: serde_json::Value,

    /// Output schema (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,

    /// Toolkit slug (if toolkit-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit: Option<String>,

    /// Whether authentication is required
    #[serde(default)]
    pub requires_auth: bool,
}

/// Custom tool execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomToolExecutionRequest {
    /// Tool slug
    pub slug: String,

    /// Execution arguments
    pub arguments: HashMap<String, serde_json::Value>,

    /// User ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Connected account ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
}

// Note: The Tools resource implementation is pending full HTTP client integration.
// The data structures above are ready for use once the client supports generic HTTP methods.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_execution_response_reexport() {
        // Test that re-exported type works
        let response = ToolExecutionResponse {
            data: serde_json::json!({"result": "success"}),
            error: None,
            successful: true,
            log_id: Some("log_123".to_string()),
            session_info: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("log_123"));
    }

    #[test]
    fn test_http_method_serialization() {
        let method = HttpMethod::Post;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"POST\"");
    }

    #[test]
    fn test_parameter_location_serialization() {
        let location = ParameterLocation::Header;
        let json = serde_json::to_string(&location).unwrap();
        assert_eq!(json, "\"header\"");
    }

    #[test]
    fn test_tool_list_params_default() {
        let params = ToolListParams::default();
        assert!(params.tool_slugs.is_none());
        assert!(params.toolkit_slug.is_none());
        assert!(params.search.is_none());
    }

    #[test]
    fn test_tool_execute_params_reexport() {
        // Test that re-exported type works
        let mut arguments = HashMap::new();
        arguments.insert("title".to_string(), serde_json::json!("Test"));

        let params = ToolExecuteParams {
            allow_tracing: None,
            arguments,
            connected_account_id: Some("ca_123".to_string()),
            custom_auth_params: None,
            custom_connection_data: None,
            entity_id: None,
            text: None,
            user_id: Some("user_456".to_string()),
            version: Some("1.0.0".to_string()),
            dangerously_skip_version_check: Some(false),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("ca_123"));
        assert!(json.contains("user_456"));
    }

    #[test]
    fn test_tool_proxy_params() {
        let params = ToolProxyParams {
            endpoint: "/api/v1/users".to_string(),
            method: HttpMethod::Get,
            body: None,
            connected_account_id: Some("ca_123".to_string()),
            parameters: Some(vec![ProxyParameter {
                name: "Authorization".to_string(),
                value: "Bearer token".to_string(),
                location: ParameterLocation::Header,
            }]),
            custom_connection_data: None,
        };

        assert_eq!(params.method, HttpMethod::Get);
        assert_eq!(params.endpoint, "/api/v1/users");
        assert!(params.parameters.is_some());
    }

    #[test]
    fn test_custom_tool_definition() {
        let tool = CustomToolDefinition {
            slug: "my_custom_tool".to_string(),
            name: "My Custom Tool".to_string(),
            description: "A custom tool for testing".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "input": {"type": "string"}
                }
            }),
            output_schema: None,
            toolkit: None,
            requires_auth: false,
        };

        assert_eq!(tool.slug, "my_custom_tool");
        assert!(!tool.requires_auth);
    }

    #[test]
    fn test_tool_info_deserialization() {
        let json = r#"{
            "slug": "GITHUB_CREATE_ISSUE",
            "name": "Create Issue",
            "description": "Create a new issue",
            "toolkit": {
                "slug": "github",
                "name": "GitHub"
            },
            "input_parameters": {},
            "output_parameters": {},
            "scopes": ["repo"],
            "tags": ["write"],
            "version": "1.0.0",
            "available_versions": ["1.0.0"],
            "is_deprecated": false,
            "no_auth": false
        }"#;

        let tool: ToolInfo = serde_json::from_str(json).unwrap();
        assert_eq!(tool.slug, "GITHUB_CREATE_ISSUE");
        assert_eq!(tool.toolkit.slug, "github");
        assert_eq!(tool.scopes.len(), 1);
    }
}
