//! OpenAPI schema utilities
//!
//! This module provides utilities for working with OpenAPI schemas,
//! including type conversion, validation, and schema manipulation.
//!
//! # Features
//!
//! - Convert OpenAPI types to Rust types
//! - Handle composite types (oneOf, anyOf, allOf)
//! - Validate JSON values against schemas
//! - Generate type-safe parameter structures

use crate::error::ComposioError;
use serde_json::{json, Value};
use std::collections::HashMap;

/// OpenAPI primitive types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiType {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object,
}

impl OpenApiType {
    /// Parse OpenAPI type from string
    pub fn from_str(s: &str) -> Result<Self, ComposioError> {
        match s {
            "null" => Ok(Self::Null),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "array" => Ok(Self::Array),
            "object" => Ok(Self::Object),
            _ => Err(ComposioError::InvalidSchema(format!(
                "Unknown OpenAPI type: {}",
                s
            ))),
        }
    }

    /// Get Rust type name as string
    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Self::Null => "Option<()>",
            Self::Boolean => "bool",
            Self::Integer => "i64",
            Self::Number => "f64",
            Self::String => "String",
            Self::Array => "Vec<Value>",
            Self::Object => "HashMap<String, Value>",
        }
    }
}

/// OpenAPI schema wrapper
#[derive(Debug, Clone)]
pub struct OpenApiSchema {
    schema: Value,
}

impl OpenApiSchema {
    /// Create a new schema from JSON value
    pub fn new(schema: Value) -> Self {
        Self { schema }
    }

    /// Get the type of this schema
    pub fn get_type(&self) -> Result<OpenApiType, ComposioError> {
        if let Some(type_str) = self.schema.get("type").and_then(|v| v.as_str()) {
            OpenApiType::from_str(type_str)
        } else if self.schema.get("oneOf").is_some()
            || self.schema.get("anyOf").is_some()
            || self.schema.get("allOf").is_some()
        {
            // Composite types are treated as generic objects
            Ok(OpenApiType::Object)
        } else {
            // No type specified - treat as Any (object)
            Ok(OpenApiType::Object)
        }
    }

    /// Check if this schema has an enum constraint
    pub fn is_enum(&self) -> bool {
        self.schema.get("enum").is_some()
    }

    /// Get enum values if this is an enum schema
    pub fn get_enum_values(&self) -> Option<Vec<Value>> {
        self.schema
            .get("enum")
            .and_then(|v| v.as_array())
            .map(|arr| arr.clone())
    }

    /// Check if this is a composite schema (oneOf, anyOf, allOf)
    pub fn is_composite(&self) -> bool {
        self.schema.get("oneOf").is_some()
            || self.schema.get("anyOf").is_some()
            || self.schema.get("allOf").is_some()
    }

    /// Get composite schemas
    pub fn get_composite_schemas(&self) -> Option<(CompositeType, Vec<OpenApiSchema>)> {
        if let Some(schemas) = self.schema.get("oneOf").and_then(|v| v.as_array()) {
            return Some((
                CompositeType::OneOf,
                schemas.iter().map(|s| OpenApiSchema::new(s.clone())).collect(),
            ));
        }
        if let Some(schemas) = self.schema.get("anyOf").and_then(|v| v.as_array()) {
            return Some((
                CompositeType::AnyOf,
                schemas.iter().map(|s| OpenApiSchema::new(s.clone())).collect(),
            ));
        }
        if let Some(schemas) = self.schema.get("allOf").and_then(|v| v.as_array()) {
            return Some((
                CompositeType::AllOf,
                schemas.iter().map(|s| OpenApiSchema::new(s.clone())).collect(),
            ));
        }
        None
    }

    /// Get array item schema
    pub fn get_array_items(&self) -> Option<OpenApiSchema> {
        self.schema
            .get("items")
            .map(|items| OpenApiSchema::new(items.clone()))
    }

    /// Get object properties
    pub fn get_properties(&self) -> HashMap<String, OpenApiSchema> {
        self.schema
            .get("properties")
            .and_then(|v| v.as_object())
            .map(|props| {
                props
                    .iter()
                    .map(|(k, v)| (k.clone(), OpenApiSchema::new(v.clone())))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get required property names
    pub fn get_required(&self) -> Vec<String> {
        self.schema
            .get("required")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get default value
    pub fn get_default(&self) -> Option<Value> {
        self.schema.get("default").cloned()
    }

    /// Get description
    pub fn get_description(&self) -> Option<String> {
        self.schema
            .get("description")
            .and_then(|v| v.as_str())
            .map(String::from)
    }

    /// Validate a value against this schema
    pub fn validate(&self, value: &Value) -> Result<(), ComposioError> {
        // Basic type validation
        let schema_type = self.get_type()?;
        
        match schema_type {
            OpenApiType::Null => {
                if !value.is_null() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected null value".to_string(),
                    ));
                }
            }
            OpenApiType::Boolean => {
                if !value.is_boolean() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected boolean value".to_string(),
                    ));
                }
            }
            OpenApiType::Integer => {
                if !value.is_i64() && !value.is_u64() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected integer value".to_string(),
                    ));
                }
            }
            OpenApiType::Number => {
                if !value.is_number() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected number value".to_string(),
                    ));
                }
            }
            OpenApiType::String => {
                if !value.is_string() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected string value".to_string(),
                    ));
                }
            }
            OpenApiType::Array => {
                if !value.is_array() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected array value".to_string(),
                    ));
                }
                // Validate array items if schema is provided
                if let Some(items_schema) = self.get_array_items() {
                    if let Some(arr) = value.as_array() {
                        for item in arr {
                            items_schema.validate(item)?;
                        }
                    }
                }
            }
            OpenApiType::Object => {
                if !value.is_object() {
                    return Err(ComposioError::InvalidSchema(
                        "Expected object value".to_string(),
                    ));
                }
            }
        }

        // Enum validation
        if self.is_enum() {
            if let Some(enum_values) = self.get_enum_values() {
                if !enum_values.contains(value) {
                    return Err(ComposioError::InvalidSchema(format!(
                        "Value {:?} not in enum: {:?}",
                        value, enum_values
                    )));
                }
            }
        }

        Ok(())
    }

    /// Convert to a Rust type description string
    pub fn to_rust_type_string(&self) -> String {
        if self.is_enum() {
            if let Some(values) = self.get_enum_values() {
                let variants: Vec<String> = values
                    .iter()
                    .filter_map(|v| {
                        if let Some(s) = v.as_str() {
                            Some(format!("\"{}\"", s))
                        } else {
                            Some(v.to_string())
                        }
                    })
                    .collect();
                return format!("Enum({})", variants.join(" | "));
            }
        }

        if let Some((composite_type, schemas)) = self.get_composite_schemas() {
            let type_strings: Vec<String> = schemas
                .iter()
                .map(|s| s.to_rust_type_string())
                .collect();
            return match composite_type {
                CompositeType::OneOf | CompositeType::AnyOf => {
                    format!("Union({})", type_strings.join(" | "))
                }
                CompositeType::AllOf => {
                    format!("Intersection({})", type_strings.join(" & "))
                }
            };
        }

        match self.get_type() {
            Ok(t) => t.to_rust_type().to_string(),
            Err(_) => "Value".to_string(),
        }
    }
}

/// Composite schema type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeType {
    OneOf,
    AnyOf,
    AllOf,
}

/// Parameter definition extracted from OpenAPI schema
#[derive(Debug, Clone)]
pub struct ParameterDefinition {
    /// Parameter name
    pub name: String,

    /// Parameter type description
    pub type_description: String,

    /// Whether this parameter is required
    pub required: bool,

    /// Default value if any
    pub default: Option<Value>,

    /// Parameter description
    pub description: Option<String>,

    /// Original schema
    pub schema: OpenApiSchema,
}

/// Extract parameter definitions from an OpenAPI schema
///
/// This function analyzes an OpenAPI schema (typically from a tool's input_parameters)
/// and extracts structured parameter information.
///
/// # Arguments
///
/// * `schema` - The OpenAPI schema (should be an object schema with properties)
///
/// # Returns
///
/// A vector of parameter definitions
pub fn extract_parameters(schema: &Value) -> Vec<ParameterDefinition> {
    let schema = OpenApiSchema::new(schema.clone());
    let properties = schema.get_properties();
    let required = schema.get_required();
    let required_set: std::collections::HashSet<_> = required.iter().collect();

    properties
        .into_iter()
        .map(|(name, prop_schema)| ParameterDefinition {
            type_description: prop_schema.to_rust_type_string(),
            required: required_set.contains(&name),
            default: prop_schema.get_default(),
            description: prop_schema.get_description(),
            schema: prop_schema.clone(),
            name,
        })
        .collect()
}

/// Merge multiple schemas (for allOf)
pub fn merge_schemas(schemas: &[Value]) -> Value {
    let mut merged = json!({
        "type": "object",
        "properties": {},
        "required": []
    });

    for schema in schemas {
        if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
            if let Some(merged_props) = merged.get_mut("properties").and_then(|v| v.as_object_mut()) {
                for (key, value) in props {
                    merged_props.insert(key.clone(), value.clone());
                }
            }
        }

        if let Some(req) = schema.get("required").and_then(|v| v.as_array()) {
            if let Some(merged_req) = merged.get_mut("required").and_then(|v| v.as_array_mut()) {
                for item in req {
                    if !merged_req.contains(item) {
                        merged_req.push(item.clone());
                    }
                }
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_type_from_str() {
        assert_eq!(OpenApiType::from_str("string").unwrap(), OpenApiType::String);
        assert_eq!(OpenApiType::from_str("integer").unwrap(), OpenApiType::Integer);
        assert_eq!(OpenApiType::from_str("boolean").unwrap(), OpenApiType::Boolean);
        assert!(OpenApiType::from_str("invalid").is_err());
    }

    #[test]
    fn test_openapi_type_to_rust() {
        assert_eq!(OpenApiType::String.to_rust_type(), "String");
        assert_eq!(OpenApiType::Integer.to_rust_type(), "i64");
        assert_eq!(OpenApiType::Boolean.to_rust_type(), "bool");
    }

    #[test]
    fn test_schema_get_type() {
        let schema = json!({"type": "string"});
        let openapi_schema = OpenApiSchema::new(schema);
        assert_eq!(openapi_schema.get_type().unwrap(), OpenApiType::String);
    }

    #[test]
    fn test_schema_is_enum() {
        let schema = json!({
            "type": "string",
            "enum": ["option1", "option2"]
        });
        let openapi_schema = OpenApiSchema::new(schema);
        assert!(openapi_schema.is_enum());
        assert_eq!(openapi_schema.get_enum_values().unwrap().len(), 2);
    }

    #[test]
    fn test_schema_validate_string() {
        let schema = json!({"type": "string"});
        let openapi_schema = OpenApiSchema::new(schema);
        
        assert!(openapi_schema.validate(&json!("hello")).is_ok());
        assert!(openapi_schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_schema_validate_enum() {
        let schema = json!({
            "type": "string",
            "enum": ["red", "green", "blue"]
        });
        let openapi_schema = OpenApiSchema::new(schema);
        
        assert!(openapi_schema.validate(&json!("red")).is_ok());
        assert!(openapi_schema.validate(&json!("yellow")).is_err());
    }

    #[test]
    fn test_schema_get_properties() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            }
        });
        let openapi_schema = OpenApiSchema::new(schema);
        let props = openapi_schema.get_properties();
        
        assert_eq!(props.len(), 2);
        assert!(props.contains_key("name"));
        assert!(props.contains_key("age"));
    }

    #[test]
    fn test_schema_get_required() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        });
        let openapi_schema = OpenApiSchema::new(schema);
        let required = openapi_schema.get_required();
        
        assert_eq!(required.len(), 1);
        assert_eq!(required[0], "name");
    }

    #[test]
    fn test_extract_parameters() {
        let schema = json!({
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "Issue title"
                },
                "body": {
                    "type": "string",
                    "default": ""
                },
                "priority": {
                    "type": "string",
                    "enum": ["low", "medium", "high"]
                }
            },
            "required": ["title"]
        });

        let params = extract_parameters(&schema);
        assert_eq!(params.len(), 3);
        
        let title_param = params.iter().find(|p| p.name == "title").unwrap();
        assert!(title_param.required);
        assert_eq!(title_param.description, Some("Issue title".to_string()));
        
        let body_param = params.iter().find(|p| p.name == "body").unwrap();
        assert!(!body_param.required);
        assert_eq!(body_param.default, Some(json!("")));
    }

    #[test]
    fn test_composite_schema_oneof() {
        let schema = json!({
            "oneOf": [
                {"type": "string"},
                {"type": "integer"}
            ]
        });
        let openapi_schema = OpenApiSchema::new(schema);
        
        assert!(openapi_schema.is_composite());
        let (composite_type, schemas) = openapi_schema.get_composite_schemas().unwrap();
        assert_eq!(composite_type, CompositeType::OneOf);
        assert_eq!(schemas.len(), 2);
    }

    #[test]
    fn test_merge_schemas() {
        let schema1 = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });
        
        let schema2 = json!({
            "type": "object",
            "properties": {
                "age": {"type": "integer"}
            },
            "required": ["age"]
        });

        let merged = merge_schemas(&[schema1, schema2]);
        let props = merged.get("properties").unwrap().as_object().unwrap();
        let required = merged.get("required").unwrap().as_array().unwrap();
        
        assert_eq!(props.len(), 2);
        assert_eq!(required.len(), 2);
    }

    #[test]
    fn test_array_schema() {
        let schema = json!({
            "type": "array",
            "items": {"type": "string"}
        });
        let openapi_schema = OpenApiSchema::new(schema);
        
        assert_eq!(openapi_schema.get_type().unwrap(), OpenApiType::Array);
        assert!(openapi_schema.get_array_items().is_some());
        
        // Validate array with correct items
        assert!(openapi_schema.validate(&json!(["a", "b", "c"])).is_ok());
        // Validate array with incorrect items
        assert!(openapi_schema.validate(&json!([1, 2, 3])).is_err());
    }
}
