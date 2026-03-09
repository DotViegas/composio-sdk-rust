//! Anthropic provider for Claude API
//!
//! This module provides the Anthropic provider implementation, which converts
//! Composio tools to Anthropic's tool format for Claude.
//!
//! # Example
//!
//! ```no_run
//! use composio_sdk::{ComposioClient, providers::AnthropicProvider};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ComposioClient::with_provider(AnthropicProvider::new())
//!     .api_key("your_key")
//!     .build()?;
//!
//! let session = client
//!     .create_session("user_123")
//!     .toolkits(vec!["github"])
//!     .send()
//!     .await?;
//!
//! // Get tools in Anthropic format
//! let tools = session.get_provider_tools().await?;
//! // tools: Vec<AnthropicTool>
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use crate::providers::Provider;
use crate::models::response::ToolSchema;

/// Anthropic tool format
///
/// Represents a tool in Anthropic's Claude API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicTool {
    /// The name of the tool
    pub name: String,
    /// A description of what the tool does
    pub description: String,
    /// The input schema for the tool (JSON Schema)
    pub input_schema: serde_json::Value,
}

/// Anthropic provider for Claude API
///
/// Converts Composio tools to Anthropic's tool format.
///
/// # Example
///
/// ```no_run
/// use composio_sdk::{ComposioClient, providers::AnthropicProvider};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = AnthropicProvider::new();
///
/// let client = ComposioClient::with_provider(provider)
///     .api_key("your_key")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct AnthropicProvider;

impl AnthropicProvider {
    /// Create a new Anthropic provider
    ///
    /// # Example
    ///
    /// ```rust
    /// use composio_sdk::providers::AnthropicProvider;
    ///
    /// let provider = AnthropicProvider::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Provider for AnthropicProvider {
    type Tool = AnthropicTool;
    type ToolCollection = Vec<AnthropicTool>;
    
    fn name(&self) -> &str {
        "anthropic"
    }
    
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
        AnthropicTool {
            name: tool.slug.clone(),
            description: tool.description.clone(),
            input_schema: tool.input_parameters.clone(),
        }
    }
    
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection {
        tools.iter().map(|t| self.wrap_tool(t)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_tool() -> ToolSchema {
        ToolSchema {
            slug: "GITHUB_CREATE_ISSUE".to_string(),
            name: "Create GitHub Issue".to_string(),
            description: "Create a new issue in a GitHub repository".to_string(),
            toolkit: "github".to_string(),
            input_parameters: json!({
                "type": "object",
                "properties": {
                    "owner": {"type": "string"},
                    "repo": {"type": "string"},
                    "title": {"type": "string"}
                },
                "required": ["owner", "repo", "title"]
            }),
            output_parameters: json!({}),
            version: "1.0.0".to_string(),
            available_versions: vec!["1.0.0".to_string()],
            is_deprecated: false,
            no_auth: false,
            scopes: vec![],
            tags: vec![],
        }
    }

    #[test]
    fn test_anthropic_provider_name() {
        let provider = AnthropicProvider::new();
        assert_eq!(provider.name(), "anthropic");
    }

    #[test]
    fn test_wrap_tool() {
        let provider = AnthropicProvider::new();
        let tool = create_test_tool();
        
        let wrapped = provider.wrap_tool(&tool);
        
        assert_eq!(wrapped.name, "GITHUB_CREATE_ISSUE");
        assert_eq!(wrapped.description, "Create a new issue in a GitHub repository");
        assert!(wrapped.input_schema.is_object());
    }

    #[test]
    fn test_wrap_tools() {
        let provider = AnthropicProvider::new();
        let tools = vec![create_test_tool(), create_test_tool()];
        
        let wrapped = provider.wrap_tools(tools);
        
        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[0].name, "GITHUB_CREATE_ISSUE");
    }

    #[test]
    fn test_serialization() {
        let provider = AnthropicProvider::new();
        let tool = create_test_tool();
        let wrapped = provider.wrap_tool(&tool);
        
        let json = serde_json::to_string(&wrapped).unwrap();
        assert!(json.contains("GITHUB_CREATE_ISSUE"));
        assert!(json.contains("input_schema"));
    }
}
