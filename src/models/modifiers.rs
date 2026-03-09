//! Tool modifiers for customizing tool behavior
//!
//! This module provides functionality to modify tool schemas, execution parameters,
//! and execution responses. Modifiers allow you to customize tool behavior without
//! changing the underlying tool definitions.
//!
//! # Modifier Types
//!
//! - [`BeforeExecute`] - Modify parameters before tool execution
//! - [`AfterExecute`] - Modify response after tool execution
//! - [`SchemaModifier`] - Modify tool schema before agent sees it
//! - [`BeforeExecuteMeta`] - Modify parameters before meta tool execution (session context)
//! - [`AfterExecuteMeta`] - Modify response after meta tool execution (session context)
//!
//! # Examples
//!
//! ```rust
//! use composio_sdk::models::modifiers::{Modifier, ModifierType};
//! use std::collections::HashMap;
//!
//! // Create a before_execute modifier
//! let modifier = Modifier::new(
//!     ModifierType::BeforeExecute,
//!     vec!["GITHUB_CREATE_ISSUE".to_string()],
//!     vec![],
//! );
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::response::{ToolExecutionResponse, ToolSchema};

/// Parameters for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecuteParams {
    /// Tool slug to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    
    /// Whether to allow tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tracing: Option<bool>,
    
    /// Tool execution arguments
    pub arguments: HashMap<String, serde_json::Value>,
    
    /// Connected account ID to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,
    
    /// Custom authentication parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth_params: Option<CustomAuthParams>,
    
    /// Custom connection data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connection_data: Option<CustomConnectionData>,
    
    /// Entity ID (deprecated, use user_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    
    /// Natural language text for tool execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    
    /// User ID for scoping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    
    /// Tool version to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    /// Skip version check (dangerous)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_skip_version_check: Option<bool>,
}

impl ToolExecuteParams {
    /// Create new tool execution parameters with required slug
    pub fn new(slug: impl Into<String>, arguments: HashMap<String, serde_json::Value>) -> Self {
        Self {
            slug: Some(slug.into()),
            allow_tracing: None,
            arguments,
            connected_account_id: None,
            custom_auth_params: None,
            custom_connection_data: None,
            entity_id: None,
            text: None,
            user_id: None,
            version: None,
            dangerously_skip_version_check: None,
        }
    }
    
    /// Get the tool slug (panics if not set)
    pub fn slug(&self) -> &str {
        self.slug.as_ref().expect("Tool slug is required for execution")
    }
}

/// Custom authentication parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAuthParams {
    /// Base URL for API requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    
    /// Custom headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    
    /// Custom query parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, String>>,
}

/// Custom connection data for various auth schemes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomConnectionData {
    /// Authentication scheme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_scheme: Option<String>,
    
    /// Connection parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Type of modifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierType {
    /// Modifier called before tool execution
    BeforeExecute,
    
    /// Modifier called after tool execution
    AfterExecute,
    
    /// Modifier for tool schema
    Schema,
    
    /// Modifier called before meta tool execution (session context)
    BeforeExecuteMeta,
    
    /// Modifier called after meta tool execution (session context)
    AfterExecuteMeta,
}

/// Trait for before execute modifiers
pub trait BeforeExecute: Send + Sync {
    /// Modify parameters before tool execution
    ///
    /// # Arguments
    ///
    /// * `tool` - Tool slug (e.g., "GITHUB_CREATE_ISSUE")
    /// * `toolkit` - Toolkit slug (e.g., "github")
    /// * `params` - Execution parameters to modify
    ///
    /// # Returns
    ///
    /// Modified execution parameters
    fn modify(&self, tool: &str, toolkit: &str, params: ToolExecuteParams) -> ToolExecuteParams;
}

/// Trait for after execute modifiers
pub trait AfterExecute: Send + Sync {
    /// Modify response after tool execution
    ///
    /// # Arguments
    ///
    /// * `tool` - Tool slug (e.g., "GITHUB_CREATE_ISSUE")
    /// * `toolkit` - Toolkit slug (e.g., "github")
    /// * `response` - Execution response to modify
    ///
    /// # Returns
    ///
    /// Modified execution response
    fn modify(
        &self,
        tool: &str,
        toolkit: &str,
        response: ToolExecutionResponse,
    ) -> ToolExecutionResponse;
}

/// Trait for schema modifiers
pub trait SchemaModifier: Send + Sync {
    /// Modify tool schema
    ///
    /// # Arguments
    ///
    /// * `tool` - Tool slug (e.g., "GITHUB_CREATE_ISSUE")
    /// * `toolkit` - Toolkit slug (e.g., "github")
    /// * `schema` - Tool schema to modify
    ///
    /// # Returns
    ///
    /// Modified tool schema
    fn modify(&self, tool: &str, toolkit: &str, schema: ToolSchema) -> ToolSchema;
}

/// Trait for before execute meta modifiers (session context)
pub trait BeforeExecuteMeta: Send + Sync {
    /// Modify parameters before meta tool execution
    ///
    /// # Arguments
    ///
    /// * `tool` - Tool slug
    /// * `toolkit` - Toolkit slug
    /// * `session_id` - Session ID for context
    /// * `params` - Parameters to modify
    ///
    /// # Returns
    ///
    /// Modified parameters
    fn modify(
        &self,
        tool: &str,
        toolkit: &str,
        session_id: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> HashMap<String, serde_json::Value>;
}

/// Trait for after execute meta modifiers (session context)
pub trait AfterExecuteMeta: Send + Sync {
    /// Modify response after meta tool execution
    ///
    /// # Arguments
    ///
    /// * `tool` - Tool slug
    /// * `toolkit` - Toolkit slug
    /// * `session_id` - Session ID for context
    /// * `response` - Response to modify
    ///
    /// # Returns
    ///
    /// Modified response
    fn modify(
        &self,
        tool: &str,
        toolkit: &str,
        session_id: &str,
        response: ToolExecutionResponse,
    ) -> ToolExecutionResponse;
}

/// A modifier that can be applied to tools
pub struct Modifier {
    /// Type of modifier
    pub modifier_type: ModifierType,
    
    /// List of tool slugs this modifier applies to (empty = all tools)
    pub tools: Vec<String>,
    
    /// List of toolkit slugs this modifier applies to (empty = all toolkits)
    pub toolkits: Vec<String>,
    
    /// The actual modifier function
    modifier_fn: ModifierFunction,
}

/// Enum to hold different types of modifier functions
enum ModifierFunction {
    BeforeExecute(Box<dyn BeforeExecute>),
    AfterExecute(Box<dyn AfterExecute>),
    Schema(Box<dyn SchemaModifier>),
    BeforeExecuteMeta(Box<dyn BeforeExecuteMeta>),
    AfterExecuteMeta(Box<dyn AfterExecuteMeta>),
}

impl Modifier {
    /// Create a new before_execute modifier
    pub fn before_execute<F>(tools: Vec<String>, toolkits: Vec<String>, modifier: F) -> Self
    where
        F: BeforeExecute + 'static,
    {
        Self {
            modifier_type: ModifierType::BeforeExecute,
            tools,
            toolkits,
            modifier_fn: ModifierFunction::BeforeExecute(Box::new(modifier)),
        }
    }

    /// Create a new after_execute modifier
    pub fn after_execute<F>(tools: Vec<String>, toolkits: Vec<String>, modifier: F) -> Self
    where
        F: AfterExecute + 'static,
    {
        Self {
            modifier_type: ModifierType::AfterExecute,
            tools,
            toolkits,
            modifier_fn: ModifierFunction::AfterExecute(Box::new(modifier)),
        }
    }

    /// Create a new schema modifier
    pub fn schema<F>(tools: Vec<String>, toolkits: Vec<String>, modifier: F) -> Self
    where
        F: SchemaModifier + 'static,
    {
        Self {
            modifier_type: ModifierType::Schema,
            tools,
            toolkits,
            modifier_fn: ModifierFunction::Schema(Box::new(modifier)),
        }
    }

    /// Create a new before_execute_meta modifier
    pub fn before_execute_meta<F>(tools: Vec<String>, toolkits: Vec<String>, modifier: F) -> Self
    where
        F: BeforeExecuteMeta + 'static,
    {
        Self {
            modifier_type: ModifierType::BeforeExecuteMeta,
            tools,
            toolkits,
            modifier_fn: ModifierFunction::BeforeExecuteMeta(Box::new(modifier)),
        }
    }

    /// Create a new after_execute_meta modifier
    pub fn after_execute_meta<F>(tools: Vec<String>, toolkits: Vec<String>, modifier: F) -> Self
    where
        F: AfterExecuteMeta + 'static,
    {
        Self {
            modifier_type: ModifierType::AfterExecuteMeta,
            tools,
            toolkits,
            modifier_fn: ModifierFunction::AfterExecuteMeta(Box::new(modifier)),
        }
    }

    /// Check if this modifier should be applied to the given tool/toolkit
    fn should_apply(&self, tool: &str, toolkit: &str) -> bool {
        // If no tools or toolkits specified, apply to all
        if self.tools.is_empty() && self.toolkits.is_empty() {
            return true;
        }

        // Check if tool or toolkit matches
        self.tools.contains(&tool.to_string()) || self.toolkits.contains(&toolkit.to_string())
    }

    /// Apply the modifier to tool execution parameters
    pub fn apply_to_params(
        &self,
        tool: &str,
        toolkit: &str,
        params: ToolExecuteParams,
    ) -> Result<ToolExecuteParams, String> {
        if !self.should_apply(tool, toolkit) {
            return Ok(params);
        }

        match &self.modifier_fn {
            ModifierFunction::BeforeExecute(modifier) => Ok(modifier.modify(tool, toolkit, params)),
            _ => Err("Modifier type mismatch: expected BeforeExecute".to_string()),
        }
    }

    /// Apply the modifier to tool execution response
    pub fn apply_to_response(
        &self,
        tool: &str,
        toolkit: &str,
        response: ToolExecutionResponse,
    ) -> Result<ToolExecutionResponse, String> {
        if !self.should_apply(tool, toolkit) {
            return Ok(response);
        }

        match &self.modifier_fn {
            ModifierFunction::AfterExecute(modifier) => {
                Ok(modifier.modify(tool, toolkit, response))
            }
            _ => Err("Modifier type mismatch: expected AfterExecute".to_string()),
        }
    }

    /// Apply the modifier to tool schema
    pub fn apply_to_schema(
        &self,
        tool: &str,
        toolkit: &str,
        schema: ToolSchema,
    ) -> Result<ToolSchema, String> {
        if !self.should_apply(tool, toolkit) {
            return Ok(schema);
        }

        match &self.modifier_fn {
            ModifierFunction::Schema(modifier) => Ok(modifier.modify(tool, toolkit, schema)),
            _ => Err("Modifier type mismatch: expected Schema".to_string()),
        }
    }

    /// Apply the modifier to meta tool parameters
    pub fn apply_to_meta_params(
        &self,
        tool: &str,
        toolkit: &str,
        session_id: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, serde_json::Value>, String> {
        if !self.should_apply(tool, toolkit) {
            return Ok(params);
        }

        match &self.modifier_fn {
            ModifierFunction::BeforeExecuteMeta(modifier) => {
                Ok(modifier.modify(tool, toolkit, session_id, params))
            }
            _ => Err("Modifier type mismatch: expected BeforeExecuteMeta".to_string()),
        }
    }

    /// Apply the modifier to meta tool response
    pub fn apply_to_meta_response(
        &self,
        tool: &str,
        toolkit: &str,
        session_id: &str,
        response: ToolExecutionResponse,
    ) -> Result<ToolExecutionResponse, String> {
        if !self.should_apply(tool, toolkit) {
            return Ok(response);
        }

        match &self.modifier_fn {
            ModifierFunction::AfterExecuteMeta(modifier) => {
                Ok(modifier.modify(tool, toolkit, session_id, response))
            }
            _ => Err("Modifier type mismatch: expected AfterExecuteMeta".to_string()),
        }
    }
}

/// Collection of modifiers
pub type Modifiers = Vec<Modifier>;

/// Apply modifiers to tool execution parameters
pub fn apply_before_execute_modifiers(
    modifiers: &Modifiers,
    tool: &str,
    toolkit: &str,
    mut params: ToolExecuteParams,
) -> Result<ToolExecuteParams, String> {
    for modifier in modifiers {
        if modifier.modifier_type == ModifierType::BeforeExecute {
            params = modifier.apply_to_params(tool, toolkit, params)?;
        }
    }
    Ok(params)
}

/// Apply modifiers to tool execution response
pub fn apply_after_execute_modifiers(
    modifiers: &Modifiers,
    tool: &str,
    toolkit: &str,
    mut response: ToolExecutionResponse,
) -> Result<ToolExecutionResponse, String> {
    for modifier in modifiers {
        if modifier.modifier_type == ModifierType::AfterExecute {
            response = modifier.apply_to_response(tool, toolkit, response)?;
        }
    }
    Ok(response)
}

/// Apply modifiers to tool schema
pub fn apply_schema_modifiers(
    modifiers: &Modifiers,
    tool: &str,
    toolkit: &str,
    mut schema: ToolSchema,
) -> Result<ToolSchema, String> {
    for modifier in modifiers {
        if modifier.modifier_type == ModifierType::Schema {
            schema = modifier.apply_to_schema(tool, toolkit, schema)?;
        }
    }
    Ok(schema)
}

/// Apply modifiers to meta tool parameters
pub fn apply_before_execute_meta_modifiers(
    modifiers: &Modifiers,
    tool: &str,
    toolkit: &str,
    session_id: &str,
    mut params: HashMap<String, serde_json::Value>,
) -> Result<HashMap<String, serde_json::Value>, String> {
    for modifier in modifiers {
        if modifier.modifier_type == ModifierType::BeforeExecuteMeta {
            params = modifier.apply_to_meta_params(tool, toolkit, session_id, params)?;
        }
    }
    Ok(params)
}

/// Apply modifiers to meta tool response
pub fn apply_after_execute_meta_modifiers(
    modifiers: &Modifiers,
    tool: &str,
    toolkit: &str,
    session_id: &str,
    mut response: ToolExecutionResponse,
) -> Result<ToolExecutionResponse, String> {
    for modifier in modifiers {
        if modifier.modifier_type == ModifierType::AfterExecuteMeta {
            response = modifier.apply_to_meta_response(tool, toolkit, session_id, response)?;
        }
    }
    Ok(response)
}
