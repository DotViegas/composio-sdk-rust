//! Provider system for framework-specific tool formatting
//!
//! This module provides a trait-based system for converting Composio tools
//! to framework-specific formats (OpenAI, Anthropic, etc.).
//!
//! # Overview
//!
//! The provider system enables the SDK to work with different AI frameworks
//! by providing a common interface for tool conversion. Each provider implements
//! the `Provider` trait and defines how to convert Composio's universal tool
//! format to the framework's specific format.
//!
//! # Example
//!
//! ```no_run
//! use composio_sdk::{ComposioClient, providers::OpenAIProvider};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Use default OpenAI provider
//! let client = ComposioClient::builder()
//!     .api_key("your_key")
//!     .build()?;
//!
//! // Or specify provider explicitly
//! let client = ComposioClient::with_provider(OpenAIProvider::new())
//!     .api_key("your_key")
//!     .build()?;
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use crate::models::response::ToolSchema;

/// Provider trait for converting Composio tools to framework-specific formats
///
/// This trait enables the SDK to work with different AI frameworks by providing
/// a common interface for tool conversion. Implementations define how to convert
/// Composio's universal tool format to the framework's specific format.
///
/// # Type Parameters
///
/// * `Tool` - The framework-specific tool type (e.g., `ChatCompletionToolParam` for OpenAI)
/// * `ToolCollection` - The collection type returned by `wrap_tools`
///
/// # Example
///
/// ```rust
/// use composio_sdk::providers::Provider;
/// use composio_sdk::models::response::ToolSchema;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct MyTool {
///     name: String,
///     description: String,
/// }
///
/// struct MyProvider;
///
/// impl Provider for MyProvider {
///     type Tool = MyTool;
///     type ToolCollection = Vec<MyTool>;
///     
///     fn name(&self) -> &str {
///         "my_provider"
///     }
///     
///     fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool {
///         MyTool {
///             name: tool.slug.clone(),
///             description: tool.description.clone(),
///         }
///     }
///     
///     fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection {
///         tools.iter().map(|t| self.wrap_tool(t)).collect()
///     }
/// }
/// ```
pub trait Provider: Send + Sync + 'static {
    /// The individual tool type for this provider
    type Tool: Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone;
    
    /// The collection type returned by wrap_tools
    type ToolCollection: IntoIterator<Item = Self::Tool> + Send + Sync;
    
    /// Get the provider name
    ///
    /// Returns a unique identifier for this provider (e.g., "openai", "anthropic").
    fn name(&self) -> &str;
    
    /// Convert a single Composio tool to the provider's format
    ///
    /// Takes a `ToolSchema` (Composio's universal format) and converts it
    /// to the framework-specific tool format.
    ///
    /// # Arguments
    ///
    /// * `tool` - The Composio tool schema to convert
    ///
    /// # Returns
    ///
    /// The tool in the provider's specific format
    fn wrap_tool(&self, tool: &ToolSchema) -> Self::Tool;
    
    /// Convert multiple Composio tools to the provider's format
    ///
    /// Takes a vector of `ToolSchema` and converts them to the framework-specific
    /// tool collection format.
    ///
    /// # Arguments
    ///
    /// * `tools` - Vector of Composio tool schemas to convert
    ///
    /// # Returns
    ///
    /// Collection of tools in the provider's specific format
    fn wrap_tools(&self, tools: Vec<ToolSchema>) -> Self::ToolCollection;
}

/// OpenAI provider implementation
pub mod openai;

/// Anthropic provider implementation
pub mod anthropic;

// Re-export providers for convenience
pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;
