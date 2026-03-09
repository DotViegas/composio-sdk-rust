//! Custom Tools module
//!
//! This module provides functionality for creating and managing custom tools.
//! Custom tools can be:
//! - Simple tools without authentication
//! - Toolkit-based tools with authentication and proxy execution
//!
//! # Examples
//!
//! ## Simple Custom Tool
//! ```no_run
//! use composio::CustomToolsRegistry;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = composio::ComposioClient::builder().api_key("key").build()?;
//! let mut registry = CustomToolsRegistry::new(client.into());
//!
//! registry.register_simple(
//!     "calculate_sum",
//!     "Calculate the sum of two numbers",
//!     json!({
//!         "type": "object",
//!         "properties": {
//!             "a": {"type": "number"},
//!             "b": {"type": "number"}
//!         },
//!         "required": ["a", "b"]
//!     }),
//!     |request| {
//!         let a = request["a"].as_f64().unwrap_or(0.0);
//!         let b = request["b"].as_f64().unwrap_or(0.0);
//!         Ok(json!({"result": a + b}))
//!     }
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Toolkit-Based Custom Tool
//! ```no_run
//! use composio::CustomToolsRegistry;
//! use serde_json::json;
//! use std::collections::HashMap;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = composio::ComposioClient::builder().api_key("key").build()?;
//! let mut registry = CustomToolsRegistry::new(client.into());
//!
//! registry.register_with_auth(
//!     "create_custom_issue",
//!     "Create a custom GitHub issue",
//!     "github",
//!     json!({
//!         "type": "object",
//!         "properties": {
//!             "title": {"type": "string"},
//!             "body": {"type": "string"}
//!         },
//!         "required": ["title"]
//!     }),
//!     |request, execute_request, _auth_credentials| {
//!         execute_request.execute(
//!             "/repos/owner/repo/issues",
//!             "POST",
//!             Some(request),
//!             None,
//!             None,
//!         )
//!     }
//! );
//! # Ok(())
//! # }
//! ```

use crate::client::ComposioClient;
use crate::error::ComposioError;
use crate::models::response::ToolProxyResponse;
use crate::models::tools::{ProxyParameter, ToolInfo, ToolkitRef};
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;

/// Function for executing proxy requests to external APIs
#[async_trait]
pub trait ExecuteRequestFn: Send + Sync {
    /// Execute a proxy request
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint (relative or absolute URL)
    /// * `method` - HTTP method (GET, POST, PUT, DELETE, PATCH)
    /// * `body` - Request body (optional)
    /// * `connected_account_id` - Connected account to use for auth (optional)
    /// * `parameters` - Additional headers/query parameters (optional)
    async fn execute(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<JsonValue>,
        connected_account_id: Option<&str>,
        parameters: Option<Vec<ProxyParameter>>,
    ) -> Result<ToolProxyResponse, ComposioError>;
}

/// Executor for custom tools
///
/// This trait abstracts over different types of custom tool executors:
/// - Simple executors (no authentication)
/// - Authenticated executors (with proxy and credentials)
#[async_trait]
pub trait CustomToolExecutor: Send + Sync {
    /// Execute the custom tool
    ///
    /// # Arguments
    /// * `request` - Input arguments as JSON
    /// * `execute_request` - Optional proxy executor for authenticated calls
    /// * `auth_credentials` - Optional authentication credentials
    async fn execute(
        &self,
        request: JsonValue,
        execute_request: Option<&dyn ExecuteRequestFn>,
        auth_credentials: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, ComposioError>;
}

/// Simple executor that doesn't require authentication
struct SimpleExecutor<F>
where
    F: Fn(JsonValue) -> Result<JsonValue, ComposioError> + Send + Sync,
{
    func: F,
}

#[async_trait]
impl<F> CustomToolExecutor for SimpleExecutor<F>
where
    F: Fn(JsonValue) -> Result<JsonValue, ComposioError> + Send + Sync,
{
    async fn execute(
        &self,
        request: JsonValue,
        _execute_request: Option<&dyn ExecuteRequestFn>,
        _auth_credentials: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, ComposioError> {
        (self.func)(request)
    }
}

/// Authenticated executor that requires proxy and credentials
struct AuthenticatedExecutor<F>
where
    F: Fn(JsonValue, &dyn ExecuteRequestFn, &HashMap<String, JsonValue>) -> Result<JsonValue, ComposioError>
        + Send
        + Sync,
{
    func: F,
}

#[async_trait]
impl<F> CustomToolExecutor for AuthenticatedExecutor<F>
where
    F: Fn(JsonValue, &dyn ExecuteRequestFn, &HashMap<String, JsonValue>) -> Result<JsonValue, ComposioError>
        + Send
        + Sync,
{
    async fn execute(
        &self,
        request: JsonValue,
        execute_request: Option<&dyn ExecuteRequestFn>,
        auth_credentials: Option<&HashMap<String, JsonValue>>,
    ) -> Result<JsonValue, ComposioError> {
        let execute_request = execute_request
            .ok_or_else(|| ComposioError::InvalidInput("Execute request function required".to_string()))?;
        let auth_credentials = auth_credentials
            .ok_or_else(|| ComposioError::InvalidInput("Auth credentials required".to_string()))?;
        
        (self.func)(request, execute_request, auth_credentials)
    }
}

/// Custom tool definition with execution logic
pub struct CustomTool {
    /// Tool slug (unique identifier, uppercase)
    pub slug: String,
    
    /// Tool name (human-readable)
    pub name: String,
    
    /// Tool description
    pub description: String,
    
    /// Toolkit slug (if toolkit-based)
    pub toolkit: Option<String>,
    
    /// Input parameters schema (JSON Schema)
    pub input_schema: JsonValue,
    
    /// Output schema (optional)
    pub output_schema: Option<JsonValue>,
    
    /// Whether authentication is required
    pub requires_auth: bool,
    
    /// Executor function
    executor: Box<dyn CustomToolExecutor>,
    
    /// Client for API operations
    client: Arc<ComposioClient>,
}

impl CustomTool {
    /// Create a simple custom tool without authentication
    ///
    /// # Arguments
    /// * `name` - Tool name (will be converted to uppercase slug)
    /// * `description` - Tool description
    /// * `input_schema` - JSON Schema for input parameters
    /// * `executor` - Function to execute the tool
    /// * `client` - Composio client
    pub fn new_simple<F>(
        name: &str,
        description: &str,
        input_schema: JsonValue,
        executor: F,
        client: Arc<ComposioClient>,
    ) -> Self
    where
        F: Fn(JsonValue) -> Result<JsonValue, ComposioError> + Send + Sync + 'static,
    {
        let slug = name.to_uppercase().replace(' ', "_");
        
        Self {
            slug,
            name: name.to_string(),
            description: description.to_string(),
            toolkit: None,
            input_schema,
            output_schema: None,
            requires_auth: false,
            executor: Box::new(SimpleExecutor { func: executor }),
            client,
        }
    }
    
    /// Create a toolkit-based custom tool with authentication
    ///
    /// # Arguments
    /// * `name` - Tool name (will be prefixed with toolkit and converted to uppercase)
    /// * `description` - Tool description
    /// * `toolkit` - Toolkit slug (e.g., "github")
    /// * `input_schema` - JSON Schema for input parameters
    /// * `executor` - Function to execute the tool (receives proxy executor and credentials)
    /// * `client` - Composio client
    pub fn new_with_auth<F>(
        name: &str,
        description: &str,
        toolkit: &str,
        input_schema: JsonValue,
        executor: F,
        client: Arc<ComposioClient>,
    ) -> Self
    where
        F: Fn(JsonValue, &dyn ExecuteRequestFn, &HashMap<String, JsonValue>) -> Result<JsonValue, ComposioError>
            + Send
            + Sync
            + 'static,
    {
        let toolkit_upper = toolkit.to_uppercase();
        let name_upper = name.to_uppercase().replace(' ', "_");
        let slug = format!("{}_{}", toolkit_upper, name_upper);
        let full_name = format!("{}_{}", toolkit.to_lowercase(), name);
        
        Self {
            slug,
            name: full_name,
            description: description.to_string(),
            toolkit: Some(toolkit.to_string()),
            input_schema,
            output_schema: None,
            requires_auth: true,
            executor: Box::new(AuthenticatedExecutor { func: executor }),
            client,
        }
    }
    
    /// Execute the custom tool
    ///
    /// # Arguments
    /// * `arguments` - Input arguments
    /// * `user_id` - User ID (required for authenticated tools)
    pub async fn execute(
        &self,
        arguments: HashMap<String, JsonValue>,
        user_id: Option<&str>,
    ) -> Result<JsonValue, ComposioError> {
        let request = JsonValue::Object(
            arguments.into_iter()
                .map(|(k, v)| (k, v))
                .collect()
        );
        
        if self.requires_auth {
            let user_id = user_id.ok_or_else(|| {
                ComposioError::InvalidInput("user_id required for authenticated tools".to_string())
            })?;
            
            let auth_credentials = self.get_auth_credentials(user_id).await?;
            
            // Create proxy executor
            let proxy_executor = ProxyExecutor {
                client: self.client.clone(),
                toolkit: self.toolkit.clone().unwrap(),
            };
            
            self.executor.execute(
                request,
                Some(&proxy_executor),
                Some(&auth_credentials),
            ).await
        } else {
            self.executor.execute(request, None, None).await
        }
    }
    
    /// Get authentication credentials for a user
    async fn get_auth_credentials(&self, user_id: &str) -> Result<HashMap<String, JsonValue>, ComposioError> {
        let toolkit = self.toolkit.as_ref()
            .ok_or_else(|| ComposioError::InvalidInput("Toolkit required for auth".to_string()))?;
        
        // Get connected accounts for this toolkit and user
        let params = crate::models::connected_accounts::ConnectedAccountListParams {
            user_ids: Some(vec![user_id.to_string()]),
            toolkit_slugs: Some(vec![toolkit.clone()]),
            statuses: Some(vec![crate::models::connected_accounts::ConnectionStatus::Active]),
            ..Default::default()
        };
        
        let accounts = self.client.list_connected_accounts(params).await?;
        
        if accounts.items.is_empty() {
            return Err(ComposioError::ValidationError(format!(
                "No active connected accounts found for toolkit {} and user {}",
                toolkit, user_id
            )));
        }
        
        // Get most recent account
        let account = accounts.items.into_iter()
            .max_by(|a, b| a.created_at.cmp(&b.created_at))
            .unwrap();
        
        // Extract credentials from state
        if let Some(state) = account.state {
            Ok(serde_json::from_value(state)?)
        } else {
            Err(ComposioError::ValidationError(
                "Connected account has no state data".to_string()
            ))
        }
    }
    
    /// Convert to ToolInfo format (for API compatibility)
    pub fn to_tool_info(&self) -> ToolInfo {
        ToolInfo {
            slug: self.slug.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            input_parameters: self.input_schema.clone(),
            output_parameters: self.output_schema.clone().unwrap_or(JsonValue::Object(Default::default())),
            scopes: vec![],
            version: "1.0.0".to_string(),
            available_versions: vec![],
            toolkit: ToolkitRef {
                slug: self.toolkit.clone().unwrap_or_else(|| "custom".to_string()).to_uppercase(),
                name: Some(self.toolkit.clone().unwrap_or_else(|| "custom".to_string())),
                logo: None,
            },
            is_deprecated: false,
            no_auth: !self.requires_auth,
            tags: vec![],
        }
    }
}

/// Proxy executor implementation
struct ProxyExecutor {
    #[allow(dead_code)]
    client: Arc<ComposioClient>,
    #[allow(dead_code)]
    toolkit: String,
}

#[async_trait]
impl ExecuteRequestFn for ProxyExecutor {
    async fn execute(
        &self,
        _endpoint: &str,
        _method: &str,
        _body: Option<JsonValue>,
        _connected_account_id: Option<&str>,
        _parameters: Option<Vec<ProxyParameter>>,
    ) -> Result<ToolProxyResponse, ComposioError> {
        // TODO: Implement actual proxy execution via HTTP
        // This would call the /api/v3/tools/execute/proxy endpoint
        Err(ComposioError::InvalidInput(
            "Proxy execution not yet fully implemented - requires proxy API endpoint".to_string()
        ))
    }
}

/// Registry for managing custom tools
pub struct CustomToolsRegistry {
    tools: HashMap<String, Arc<CustomTool>>,
    client: Arc<ComposioClient>,
}

impl CustomToolsRegistry {
    /// Create a new custom tools registry
    pub fn new(client: Arc<ComposioClient>) -> Self {
        Self {
            tools: HashMap::new(),
            client,
        }
    }
    
    /// Register a simple custom tool without authentication
    ///
    /// # Arguments
    /// * `name` - Tool name
    /// * `description` - Tool description
    /// * `input_schema` - JSON Schema for input parameters
    /// * `executor` - Function to execute the tool
    ///
    /// # Returns
    /// Arc reference to the registered tool
    pub fn register_simple<F>(
        &mut self,
        name: &str,
        description: &str,
        input_schema: JsonValue,
        executor: F,
    ) -> Arc<CustomTool>
    where
        F: Fn(JsonValue) -> Result<JsonValue, ComposioError> + Send + Sync + 'static,
    {
        let tool = Arc::new(CustomTool::new_simple(
            name,
            description,
            input_schema,
            executor,
            self.client.clone(),
        ));
        
        self.tools.insert(tool.slug.clone(), tool.clone());
        tool
    }
    
    /// Register a toolkit-based custom tool with authentication
    ///
    /// # Arguments
    /// * `name` - Tool name
    /// * `description` - Tool description
    /// * `toolkit` - Toolkit slug
    /// * `input_schema` - JSON Schema for input parameters
    /// * `executor` - Function to execute the tool
    ///
    /// # Returns
    /// Arc reference to the registered tool
    pub fn register_with_auth<F>(
        &mut self,
        name: &str,
        description: &str,
        toolkit: &str,
        input_schema: JsonValue,
        executor: F,
    ) -> Arc<CustomTool>
    where
        F: Fn(JsonValue, &dyn ExecuteRequestFn, &HashMap<String, JsonValue>) -> Result<JsonValue, ComposioError>
            + Send
            + Sync
            + 'static,
    {
        let tool = Arc::new(CustomTool::new_with_auth(
            name,
            description,
            toolkit,
            input_schema,
            executor,
            self.client.clone(),
        ));
        
        self.tools.insert(tool.slug.clone(), tool.clone());
        tool
    }
    
    /// Get a custom tool by slug
    pub fn get(&self, slug: &str) -> Option<Arc<CustomTool>> {
        self.tools.get(slug).cloned()
    }
    
    /// Execute a custom tool
    ///
    /// # Arguments
    /// * `slug` - Tool slug
    /// * `arguments` - Input arguments
    /// * `user_id` - User ID (required for authenticated tools)
    pub async fn execute(
        &self,
        slug: &str,
        arguments: HashMap<String, JsonValue>,
        user_id: Option<&str>,
    ) -> Result<JsonValue, ComposioError> {
        let tool = self.get(slug)
            .ok_or_else(|| ComposioError::ValidationError(format!("Custom tool {} not found", slug)))?;
        
        tool.execute(arguments, user_id).await
    }
    
    /// List all registered custom tools
    pub fn list(&self) -> Vec<Arc<CustomTool>> {
        self.tools.values().cloned().collect()
    }
    
    /// Get all custom tools as ToolInfo (for API compatibility)
    pub fn list_as_tools(&self) -> Vec<ToolInfo> {
        self.tools.values()
            .map(|tool| tool.to_tool_info())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_custom_tool_simple() {
        let client = Arc::new(
            ComposioClient::builder()
                .api_key("test_key")
                .build()
                .unwrap()
        );
        
        let tool = CustomTool::new_simple(
            "calculate sum",
            "Calculate the sum of two numbers",
            json!({
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                }
            }),
            |request| {
                let a = request["a"].as_f64().unwrap_or(0.0);
                let b = request["b"].as_f64().unwrap_or(0.0);
                Ok(json!({"result": a + b}))
            },
            client,
        );
        
        assert_eq!(tool.slug, "CALCULATE_SUM");
        assert_eq!(tool.name, "calculate sum");
        assert!(!tool.requires_auth);
        assert!(tool.toolkit.is_none());
    }
    
    #[test]
    fn test_custom_tool_with_auth() {
        let client = Arc::new(
            ComposioClient::builder()
                .api_key("test_key")
                .build()
                .unwrap()
        );
        
        let tool = CustomTool::new_with_auth(
            "create issue",
            "Create a GitHub issue",
            "github",
            json!({
                "type": "object",
                "properties": {
                    "title": {"type": "string"}
                }
            }),
            |_request, _execute_request, _auth_credentials| {
                Ok(json!({"id": 123}))
            },
            client,
        );
        
        assert_eq!(tool.slug, "GITHUB_CREATE_ISSUE");
        assert_eq!(tool.name, "github_create issue");
        assert!(tool.requires_auth);
        assert_eq!(tool.toolkit, Some("github".to_string()));
    }
    
    #[test]
    fn test_registry() {
        let client = Arc::new(
            ComposioClient::builder()
                .api_key("test_key")
                .build()
                .unwrap()
        );
        
        let mut registry = CustomToolsRegistry::new(client);
        
        registry.register_simple(
            "test_tool",
            "A test tool",
            json!({"type": "object"}),
            |_request| Ok(json!({"success": true}))
        );
        
        assert!(registry.get("TEST_TOOL").is_some());
        assert_eq!(registry.list().len(), 1);
    }
}
