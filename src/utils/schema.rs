//! JSON Schema utilities for Composio SDK
//!
//! This module provides utilities for working with JSON schemas, including:
//! - Type mapping between JSON Schema types and Rust types
//! - Schema validation and conversion
//! - Default value coercion
//! - Reserved keyword handling
//!
//! # Example
//!
//! ```rust
//! use composio_sdk::utils::schema::{JsonSchemaType, coerce_default_value};
//! use serde_json::json;
//!
//! let schema = json!({
//!     "type": "boolean",
//!     "default": "true"
//! });
//!
//! let coerced = coerce_default_value(&json!("true"), &schema);
//! assert_eq!(coerced, json!(true));
//! ```

use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

/// JSON Schema primitive types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonSchemaType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
    Null,
}

impl JsonSchemaType {
    /// Parse a JSON Schema type string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "string" => Some(Self::String),
            "integer" => Some(Self::Integer),
            "number" => Some(Self::Number),
            "boolean" => Some(Self::Boolean),
            "array" => Some(Self::Array),
            "object" => Some(Self::Object),
            "null" => Some(Self::Null),
            _ => None,
        }
    }

    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Array => "array",
            Self::Object => "object",
            Self::Null => "null",
        }
    }

    /// Check if this is a container type (array or object)
    pub fn is_container(&self) -> bool {
        matches!(self, Self::Array | Self::Object)
    }

    /// Get the Rust type name for this JSON Schema type
    pub fn rust_type_name(&self) -> &'static str {
        match self {
            Self::String => "String",
            Self::Integer => "i64",
            Self::Number => "f64",
            Self::Boolean => "bool",
            Self::Array => "Vec<Value>",
            Self::Object => "HashMap<String, Value>",
            Self::Null => "Option<Value>",
        }
    }

    /// Get the default/fallback value for this type
    pub fn fallback_value(&self) -> Value {
        match self {
            Self::String => json!(""),
            Self::Integer => json!(0),
            Self::Number => json!(0.0),
            Self::Boolean => json!(false),
            Self::Array => json!([]),
            Self::Object => json!({}),
            Self::Null => Value::Null,
        }
    }
}

/// Rust reserved keywords that need special handling
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    "try",
];

/// Reserved Pydantic field names that need special handling
const RESERVED_FIELD_NAMES: &[&str] = &["validate"];

/// Marker for nested object keyword substitutions
const OBJ_MARKER: &str = "-_object_-";

/// Make a field name safe by appending a suffix
///
/// For Rust keywords, we append `_field` to make them valid identifiers.
/// For Pydantic reserved names, we append `_` for compatibility.
pub fn make_safe_field_name(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{}_field", name)
    } else if RESERVED_FIELD_NAMES.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

/// Check if a name is a Rust keyword
pub fn is_rust_keyword(name: &str) -> bool {
    RUST_KEYWORDS.contains(&name)
}

/// Check if a name is a reserved field name
pub fn is_reserved_field_name(name: &str) -> bool {
    RESERVED_FIELD_NAMES.contains(&name)
}

/// Substitute reserved keywords in a JSON schema
///
/// Returns a tuple of (modified_schema, keyword_mappings) where:
/// - modified_schema has safe property names
/// - keyword_mappings maps safe names back to original names
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::schema::substitute_reserved_keywords;
/// use serde_json::json;
///
/// let schema = json!({
///     "properties": {
///         "type": {"type": "string"},
///         "match": {"type": "integer"}
///     },
///     "required": ["type"]
/// });
///
/// let (safe_schema, mappings) = substitute_reserved_keywords(&schema);
/// // safe_schema will have "type_field" and "match_field" as property names
/// ```
pub fn substitute_reserved_keywords(schema: &Value) -> (Value, HashMap<String, String>) {
    let mut mappings = HashMap::new();
    
    if !schema.is_object() {
        return (schema.clone(), mappings);
    }

    let mut result = schema.clone();
    
    if let Some(properties) = result.get("properties").and_then(|p| p.as_object()) {
        let mut new_properties = serde_json::Map::new();
        
        for (prop_name, prop_value) in properties {
            if is_rust_keyword(prop_name) || is_reserved_field_name(prop_name) {
                let safe_name = make_safe_field_name(prop_name);
                
                // Recursively handle nested objects
                let safe_value = if prop_value.get("type") == Some(&json!("object")) {
                    let (nested_schema, nested_map) = substitute_reserved_keywords(prop_value);
                    if !nested_map.is_empty() {
                        mappings.insert(format!("{}{}", safe_name, OBJ_MARKER), 
                                      serde_json::to_string(&nested_map).unwrap_or_default());
                    }
                    nested_schema
                } else {
                    prop_value.clone()
                };
                
                new_properties.insert(safe_name.clone(), safe_value);
                mappings.insert(safe_name, prop_name.clone());
            } else {
                new_properties.insert(prop_name.clone(), prop_value.clone());
            }
        }
        
        result["properties"] = Value::Object(new_properties);
        
        // Update required array
        if let Some(required) = result.get("required").and_then(|r| r.as_array()) {
            let reverse_map: HashMap<_, _> = mappings.iter()
                .filter(|(k, _)| !k.ends_with(OBJ_MARKER))
                .map(|(k, v)| (v.as_str(), k.as_str()))
                .collect();
            
            let new_required: Vec<Value> = required.iter()
                .map(|r| {
                    if let Some(s) = r.as_str() {
                        if let Some(safe) = reverse_map.get(s) {
                            json!(safe)
                        } else {
                            r.clone()
                        }
                    } else {
                        r.clone()
                    }
                })
                .collect();
            
            result["required"] = json!(new_required);
        }
    }
    
    (result, mappings)
}

/// Reinstate reserved keywords in a request object
///
/// Reverses the substitution performed by `substitute_reserved_keywords`.
/// Modifies the request in-place and returns it.
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::schema::reinstate_reserved_keywords;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// let mut request = json!({
///     "type_field": "example",
///     "match_field": 42
/// });
///
/// let mut mappings = HashMap::new();
/// mappings.insert("type_field".to_string(), "type".to_string());
/// mappings.insert("match_field".to_string(), "match".to_string());
///
/// let result = reinstate_reserved_keywords(&mut request, &mappings);
/// // result will have "type" and "match" as keys
/// ```
pub fn reinstate_reserved_keywords(
    request: &mut Value,
    mappings: &HashMap<String, String>,
) -> Value {
    if !request.is_object() {
        return request.clone();
    }

    let mut sorted_keys: Vec<_> = mappings.keys().collect();
    sorted_keys.sort_by(|a, b| b.cmp(a)); // Reverse order

    let obj = request.as_object_mut().unwrap();
    let mut updates = Vec::new();

    for clean_key in sorted_keys {
        let mut subkeys = None;
        let actual_key = if clean_key.ends_with(OBJ_MARKER) {
            if let Some(nested_json) = mappings.get(clean_key.as_str()) {
                if let Ok(nested_map) = serde_json::from_str::<HashMap<String, String>>(nested_json) {
                    subkeys = Some(nested_map);
                }
            }
            clean_key.trim_end_matches(OBJ_MARKER)
        } else {
            clean_key.as_str()
        };

        if let Some(mut value) = obj.remove(actual_key) {
            if let Some(nested_mappings) = subkeys {
                value = reinstate_reserved_keywords(&mut value, &nested_mappings);
            }
            
            if let Some(original_key) = mappings.get(clean_key.as_str()) {
                updates.push((original_key.clone(), value));
            }
        }
    }

    for (key, value) in updates {
        obj.insert(key, value);
    }

    Value::Object(obj.clone())
}

/// Coerce a default value to match the expected type from JSON schema
///
/// Handles common mismatches where string defaults should be boolean/int/float.
/// This fixes issues where API returns stringified defaults like "true" instead of true.
///
/// Coercion precedence: boolean > integer > float
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::schema::coerce_default_value;
/// use serde_json::json;
///
/// let schema = json!({"type": "boolean"});
/// let default = json!("true");
/// let coerced = coerce_default_value(&default, &schema);
/// assert_eq!(coerced, json!(true));
/// ```
pub fn coerce_default_value(default: &Value, schema: &Value) -> Value {
    // Only coerce string values
    let default_str = match default.as_str() {
        Some(s) => s,
        None => return default.clone(),
    };

    // Collect expected types from schema
    let mut expected_types = HashSet::new();

    // Direct type
    if let Some(type_str) = schema.get("type").and_then(|t| t.as_str()) {
        if let Some(schema_type) = JsonSchemaType::from_str(type_str) {
            expected_types.insert(schema_type);
        }
    }

    // Types from combiners (anyOf, oneOf, allOf)
    for combiner in &["anyOf", "oneOf", "allOf"] {
        if let Some(options) = schema.get(combiner).and_then(|o| o.as_array()) {
            for option in options {
                if let Some(type_str) = option.get("type").and_then(|t| t.as_str()) {
                    if let Some(schema_type) = JsonSchemaType::from_str(type_str) {
                        expected_types.insert(schema_type);
                    }
                }
            }
        }
    }

    // If string is expected, no coercion needed
    if expected_types.contains(&JsonSchemaType::String) {
        return default.clone();
    }

    // Boolean coercion (takes precedence)
    if expected_types.contains(&JsonSchemaType::Boolean) {
        let lower = default_str.to_lowercase();
        if matches!(lower.as_str(), "true" | "yes" | "1") {
            return json!(true);
        }
        if matches!(lower.as_str(), "false" | "no" | "0") {
            return json!(false);
        }
    }

    // Integer coercion
    if expected_types.contains(&JsonSchemaType::Integer) {
        if let Ok(i) = default_str.parse::<i64>() {
            return json!(i);
        }
    }

    // Float coercion
    if expected_types.contains(&JsonSchemaType::Number) {
        if let Ok(f) = default_str.parse::<f64>() {
            return json!(f);
        }
    }

    default.clone()
}

/// Generate a unique request ID
///
/// Uses UUID v4 for generating unique identifiers.
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::schema::generate_request_id;
///
/// let request_id = generate_request_id();
/// assert_eq!(request_id.len(), 36); // UUID format
/// ```
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a UUID v4 string
///
/// Alias for `generate_request_id()` for compatibility with Python SDK.
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::schema::generate_uuid;
///
/// let uuid = generate_uuid();
/// assert_eq!(uuid.len(), 36);
/// ```
pub fn generate_uuid() -> String {
    generate_request_id()
}

/// Generate a short ID (8 characters) from a UUID
///
/// Returns the first 8 characters of a UUID with dashes removed.
/// Useful for generating compact identifiers.
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::schema::generate_short_id;
///
/// let short_id = generate_short_id();
/// assert_eq!(short_id.len(), 8);
/// assert!(!short_id.contains('-'));
/// ```
pub fn generate_short_id() -> String {
    generate_uuid()
        .chars()
        .filter(|c| *c != '-')
        .take(8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_schema_type_from_str() {
        assert_eq!(JsonSchemaType::from_str("string"), Some(JsonSchemaType::String));
        assert_eq!(JsonSchemaType::from_str("integer"), Some(JsonSchemaType::Integer));
        assert_eq!(JsonSchemaType::from_str("boolean"), Some(JsonSchemaType::Boolean));
        assert_eq!(JsonSchemaType::from_str("invalid"), None);
    }

    #[test]
    fn test_json_schema_type_as_str() {
        assert_eq!(JsonSchemaType::String.as_str(), "string");
        assert_eq!(JsonSchemaType::Integer.as_str(), "integer");
        assert_eq!(JsonSchemaType::Boolean.as_str(), "boolean");
    }

    #[test]
    fn test_json_schema_type_is_container() {
        assert!(JsonSchemaType::Array.is_container());
        assert!(JsonSchemaType::Object.is_container());
        assert!(!JsonSchemaType::String.is_container());
        assert!(!JsonSchemaType::Integer.is_container());
    }

    #[test]
    fn test_json_schema_type_fallback_value() {
        assert_eq!(JsonSchemaType::String.fallback_value(), json!(""));
        assert_eq!(JsonSchemaType::Integer.fallback_value(), json!(0));
        assert_eq!(JsonSchemaType::Boolean.fallback_value(), json!(false));
        assert_eq!(JsonSchemaType::Array.fallback_value(), json!([]));
        assert_eq!(JsonSchemaType::Object.fallback_value(), json!({}));
    }

    #[test]
    fn test_is_rust_keyword() {
        assert!(is_rust_keyword("type"));
        assert!(is_rust_keyword("match"));
        assert!(is_rust_keyword("impl"));
        assert!(!is_rust_keyword("name"));
        assert!(!is_rust_keyword("value"));
    }

    #[test]
    fn test_make_safe_field_name() {
        assert_eq!(make_safe_field_name("type"), "type_field");
        assert_eq!(make_safe_field_name("match"), "match_field");
        assert_eq!(make_safe_field_name("validate"), "validate_");
        assert_eq!(make_safe_field_name("normal"), "normal");
    }

    #[test]
    fn test_substitute_reserved_keywords() {
        let schema = json!({
            "properties": {
                "type": {"type": "string"},
                "match": {"type": "integer"},
                "normal": {"type": "boolean"}
            },
            "required": ["type"]
        });

        let (safe_schema, mappings) = substitute_reserved_keywords(&schema);
        
        assert!(safe_schema["properties"].get("type_field").is_some());
        assert!(safe_schema["properties"].get("match_field").is_some());
        assert!(safe_schema["properties"].get("normal").is_some());
        
        assert_eq!(mappings.get("type_field"), Some(&"type".to_string()));
        assert_eq!(mappings.get("match_field"), Some(&"match".to_string()));
        
        let required = safe_schema["required"].as_array().unwrap();
        assert!(required.contains(&json!("type_field")));
    }

    #[test]
    fn test_reinstate_reserved_keywords() {
        let mut request = json!({
            "type_field": "example",
            "match_field": 42,
            "normal": true
        });

        let mut mappings = HashMap::new();
        mappings.insert("type_field".to_string(), "type".to_string());
        mappings.insert("match_field".to_string(), "match".to_string());

        let result = reinstate_reserved_keywords(&mut request, &mappings);
        
        assert_eq!(result.get("type"), Some(&json!("example")));
        assert_eq!(result.get("match"), Some(&json!(42)));
        assert_eq!(result.get("normal"), Some(&json!(true)));
        assert!(result.get("type_field").is_none());
        assert!(result.get("match_field").is_none());
    }

    #[test]
    fn test_coerce_default_value_boolean() {
        let schema = json!({"type": "boolean"});
        
        assert_eq!(coerce_default_value(&json!("true"), &schema), json!(true));
        assert_eq!(coerce_default_value(&json!("false"), &schema), json!(false));
        assert_eq!(coerce_default_value(&json!("yes"), &schema), json!(true));
        assert_eq!(coerce_default_value(&json!("no"), &schema), json!(false));
        assert_eq!(coerce_default_value(&json!("1"), &schema), json!(true));
        assert_eq!(coerce_default_value(&json!("0"), &schema), json!(false));
    }

    #[test]
    fn test_coerce_default_value_integer() {
        let schema = json!({"type": "integer"});
        
        assert_eq!(coerce_default_value(&json!("42"), &schema), json!(42));
        assert_eq!(coerce_default_value(&json!("-10"), &schema), json!(-10));
        assert_eq!(coerce_default_value(&json!("0"), &schema), json!(0));
    }

    #[test]
    fn test_coerce_default_value_number() {
        let schema = json!({"type": "number"});
        
        assert_eq!(coerce_default_value(&json!("3.14"), &schema), json!(3.14));
        assert_eq!(coerce_default_value(&json!("-2.5"), &schema), json!(-2.5));
    }

    #[test]
    fn test_coerce_default_value_string_no_coercion() {
        let schema = json!({"type": "string"});
        
        assert_eq!(coerce_default_value(&json!("true"), &schema), json!("true"));
        assert_eq!(coerce_default_value(&json!("42"), &schema), json!("42"));
    }

    #[test]
    fn test_coerce_default_value_with_combiners() {
        let schema = json!({
            "anyOf": [
                {"type": "boolean"},
                {"type": "integer"}
            ]
        });
        
        // Boolean takes precedence over integer
        assert_eq!(coerce_default_value(&json!("true"), &schema), json!(true));
        assert_eq!(coerce_default_value(&json!("1"), &schema), json!(true));
        assert_eq!(coerce_default_value(&json!("42"), &schema), json!(42));
    }

    #[test]
    fn test_coerce_default_value_non_string() {
        let schema = json!({"type": "boolean"});
        
        // Non-string values are returned as-is
        assert_eq!(coerce_default_value(&json!(true), &schema), json!(true));
        assert_eq!(coerce_default_value(&json!(42), &schema), json!(42));
        assert_eq!(coerce_default_value(&Value::Null, &schema), Value::Null);
    }

    #[test]
    fn test_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        
        assert_eq!(id1.len(), 36); // UUID v4 format
        assert_eq!(id2.len(), 36);
        assert_ne!(id1, id2); // Should be unique
    }

    #[test]
    fn test_generate_uuid() {
        let uuid1 = generate_uuid();
        let uuid2 = generate_uuid();
        
        assert_eq!(uuid1.len(), 36);
        assert_eq!(uuid2.len(), 36);
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_generate_short_id() {
        let short1 = generate_short_id();
        let short2 = generate_short_id();
        
        assert_eq!(short1.len(), 8);
        assert_eq!(short2.len(), 8);
        assert!(!short1.contains('-'));
        assert!(!short2.contains('-'));
        assert_ne!(short1, short2);
    }
}
