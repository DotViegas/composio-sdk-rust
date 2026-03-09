//! OpenAI provider for Chat Completions API
//!
//! This module provides the OpenAI provider implementation, which converts
//! Composio tools to OpenAI's `ChatCompletionToolParam` format.
//!
//! # Example
//!
//! ```no_run
//! use composio_sdk::{ComposioClient, providers::OpenAIProvider};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ComposioClient::with_provider(OpenAIProvider::new())
//!     .api_key("your_key")
//!     .build()?;
//!
//! let session = client
//!     .create_session("user_123")
//!     .toolkits(vec!["github"])
//!     .send()
//!     .await?;
//!
//! // Get tools in OpenAI format
//! let tools = session.get_provider_tools().await?;
//! // tools: Vec<ChatCompletionToolParam>
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use crate::providers::Provider;
use crate::models::response::ToolSchema;

/// OpenAI tool format (ChatCompletionToolParam)
///
/// Represents a tool in OpenAI's Chat Completions API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionToolParam {
    /// The type of the tool (always "function" for function calling)
    pub r#type: String,
    /// The function definition
    pub function: FunctionDefinition,
}

/// Function definition for OpenAI tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// The name of the function
    pub name: String,
    /// A description of what the function does
    pub description: String,
    /// The parameters the function accepts (JSON Schema)
    pub parameters: serde_json::Value,
    /// Whether to enable strict schema validation (OpenAI feature)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// OpenAI provider for Chat Completions API
///
/// Converts Composio tools to OpenAI's ChatCompletionToolParam format.
/// Supports optional strict schema validation.
///
/// # Example
///
/// ```no_run
/// use composio_sdk::{ComposioClient, providers::OpenAIProvider};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Default provider (no strict validation)
/// let provider = OpenAIProvider::new();
///
/// // With strict validation enabled
/// let provider = OpenAIProvider::new().with_strict(true);
///
/// let client = ComposioClient::with_provider(provider)
///     .api_key("your_key")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct OpenAIProvider {
    /// Whether to use strict schema validation
    strict: bool,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider with default settings
    ///
    /// # Example
    ///
    /// ```rust
    /// use composio_sdk::providers::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new();
    /// ```
    pub fn new() -> Self {
        Self { strict: false }
    }
    
    /// Enable or disable strict schema validation
    ///
    /// When enabled, OpenAI will enforce strict schema validation on tool calls.
    /// This is an OpenAI-specific feature that ensures tool calls match the schema exactly.
    ///
    /// # Arguments
    ///
    /// * `strict` - Whether to enable strict validation
    ///
    /// # Example
    ///
    /// ```rust
    /// use composio_sdk::providers::OpenAIProvider;
    ///
    /// let provider = OpenAIProvider::new().with_strict(true);
    /// ```
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
}

impl Default for OpenAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider for OpenAIProvider {
    type Tool = ChatCompletionToolParam;
    type ToolCollection = Vec<ChatCompletionToolParam>;
    
    fn name(&self) -> &str {
        "openai"
    }
    
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
        ChatCompletionToolParam {
            r#type: "function".to_string(),
            function: FunctionDefinition {
                name: tool.slug.clone(),
                description: tool.description.clone(),
                parameters: tool.input_parameters.clone(),
                strict: if self.strict { Some(true) } else { None },
            },
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
    fn test_openai_provider_name() {
        let provider = OpenAIProvider::new();
        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_wrap_tool_basic() {
        let provider = OpenAIProvider::new();
        let tool = create_test_tool();
        
        let wrapped = provider.wrap_tool(&tool);
        
        assert_eq!(wrapped.r#type, "function");
        assert_eq!(wrapped.function.name, "GITHUB_CREATE_ISSUE");
        assert_eq!(wrapped.function.description, "Create a new issue in a GitHub repository");
        assert!(wrapped.function.strict.is_none());
    }

    #[test]
    fn test_wrap_tool_with_strict() {
        let provider = OpenAIProvider::new().with_strict(true);
        let tool = create_test_tool();
        
        let wrapped = provider.wrap_tool(&tool);
        
        assert_eq!(wrapped.function.strict, Some(true));
    }

    #[test]
    fn test_wrap_tools() {
        let provider = OpenAIProvider::new();
        let tools = vec![create_test_tool(), create_test_tool()];
        
        let wrapped = provider.wrap_tools(tools);
        
        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[0].function.name, "GITHUB_CREATE_ISSUE");
    }

    #[test]
    fn test_serialization() {
        let provider = OpenAIProvider::new();
        let tool = create_test_tool();
        let wrapped = provider.wrap_tool(&tool);
        
        let json = serde_json::to_string(&wrapped).unwrap();
        assert!(json.contains("function"));
        assert!(json.contains("GITHUB_CREATE_ISSUE"));
    }
}
