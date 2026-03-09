//! Example demonstrating JSON Schema utilities
//!
//! This example shows how to use the schema utilities for:
//! - Type mapping and conversion
//! - Reserved keyword handling
//! - Default value coercion
//! - Request ID generation
//!
//! Run with:
//! ```bash
//! cargo run --example schema_utilities
//! ```

use composio_sdk::utils::schema::{
    JsonSchemaType, make_safe_field_name, substitute_reserved_keywords,
    reinstate_reserved_keywords, coerce_default_value, generate_request_id,
};
use serde_json::json;

fn main() {
    println!("=== JSON Schema Utilities Example ===\n");

    // 1. Type Mapping
    println!("1. Type Mapping:");
    let types = vec![
        JsonSchemaType::String,
        JsonSchemaType::Integer,
        JsonSchemaType::Boolean,
        JsonSchemaType::Array,
        JsonSchemaType::Object,
    ];

    for schema_type in types {
        println!(
            "  {} -> Rust type: {} (fallback: {})",
            schema_type.as_str(),
            schema_type.rust_type_name(),
            schema_type.fallback_value()
        );
    }

    // 2. Reserved Keyword Handling
    println!("\n2. Reserved Keyword Handling:");
    let field_names = vec!["type", "match", "impl", "normal", "validate"];
    
    for name in field_names {
        let safe_name = make_safe_field_name(name);
        if safe_name != name {
            println!("  '{}' -> '{}' (reserved)", name, safe_name);
        } else {
            println!("  '{}' -> '{}' (ok)", name, safe_name);
        }
    }

    // 3. Schema Keyword Substitution
    println!("\n3. Schema Keyword Substitution:");
    let schema = json!({
        "properties": {
            "type": {
                "type": "string",
                "description": "The type of resource"
            },
            "match": {
                "type": "integer",
                "description": "Match score"
            },
            "name": {
                "type": "string",
                "description": "Resource name"
            }
        },
        "required": ["type", "name"]
    });

    println!("  Original schema:");
    println!("    Properties: type, match, name");
    println!("    Required: [type, name]");

    let (safe_schema, mappings) = substitute_reserved_keywords(&schema);
    
    println!("\n  Safe schema:");
    if let Some(props) = safe_schema.get("properties").and_then(|p| p.as_object()) {
        let prop_names: Vec<_> = props.keys().map(|s| s.as_str()).collect();
        println!("    Properties: {}", prop_names.join(", "));
    }
    if let Some(required) = safe_schema.get("required").and_then(|r| r.as_array()) {
        let req_names: Vec<_> = required.iter()
            .filter_map(|v| v.as_str())
            .collect();
        println!("    Required: [{}]", req_names.join(", "));
    }

    println!("\n  Mappings:");
    for (safe, original) in &mappings {
        println!("    {} -> {}", safe, original);
    }

    // 4. Reinstate Keywords
    println!("\n4. Reinstate Keywords:");
    let mut request = json!({
        "type_field": "example",
        "match_field": 42,
        "name": "Test Resource"
    });

    println!("  Request with safe names:");
    println!("    {}", serde_json::to_string_pretty(&request).unwrap());

    let restored = reinstate_reserved_keywords(&mut request, &mappings);
    
    println!("\n  Request with original names:");
    println!("    {}", serde_json::to_string_pretty(&restored).unwrap());

    // 5. Default Value Coercion
    println!("\n5. Default Value Coercion:");
    
    let test_cases = vec![
        (json!("true"), json!({"type": "boolean"}), "String 'true' to boolean"),
        (json!("false"), json!({"type": "boolean"}), "String 'false' to boolean"),
        (json!("42"), json!({"type": "integer"}), "String '42' to integer"),
        (json!("3.14"), json!({"type": "number"}), "String '3.14' to float"),
        (json!("yes"), json!({"type": "boolean"}), "String 'yes' to boolean"),
        (json!("1"), json!({"type": "boolean"}), "String '1' to boolean (precedence)"),
        (json!("hello"), json!({"type": "string"}), "String stays string"),
    ];

    for (default, schema, description) in test_cases {
        let coerced = coerce_default_value(&default, &schema);
        println!("  {}: {} -> {}", description, default, coerced);
    }

    // 6. Coercion with Combiners
    println!("\n6. Coercion with Combiners (anyOf):");
    let combiner_schema = json!({
        "anyOf": [
            {"type": "boolean"},
            {"type": "integer"}
        ]
    });

    let test_values = vec![
        json!("true"),
        json!("false"),
        json!("1"),
        json!("0"),
        json!("42"),
    ];

    for value in test_values {
        let coerced = coerce_default_value(&value, &combiner_schema);
        println!("  {} -> {} (boolean takes precedence)", value, coerced);
    }

    // 7. Request ID Generation
    println!("\n7. Request ID Generation:");
    for i in 1..=3 {
        let request_id = generate_request_id();
        println!("  Request #{}: {}", i, request_id);
    }

    // 8. Practical Example: Tool Parameter Processing
    println!("\n8. Practical Example: Tool Parameter Processing");
    
    let tool_schema = json!({
        "properties": {
            "type": {
                "type": "string",
                "default": "issue"
            },
            "priority": {
                "type": "integer",
                "default": "1"
            },
            "is_urgent": {
                "type": "boolean",
                "default": "false"
            },
            "title": {
                "type": "string"
            }
        },
        "required": ["title"]
    });

    println!("\n  Original tool schema has reserved keyword 'type'");
    
    // Step 1: Make schema safe
    let (safe_tool_schema, tool_mappings) = substitute_reserved_keywords(&tool_schema);
    println!("  ✓ Schema made safe (type -> type_field)");

    // Step 2: Coerce default values
    if let Some(props) = safe_tool_schema.get("properties").and_then(|p| p.as_object()) {
        println!("\n  Coercing default values:");
        for (prop_name, prop_schema) in props {
            if let Some(default) = prop_schema.get("default") {
                let coerced = coerce_default_value(default, prop_schema);
                if default != &coerced {
                    println!("    {}: {} -> {}", prop_name, default, coerced);
                }
            }
        }
    }

    // Step 3: Create request with safe names
    let mut tool_request = json!({
        "type_field": "bug",
        "priority": 2,
        "is_urgent": true,
        "title": "Fix critical issue"
    });

    println!("\n  Request with safe names:");
    println!("    {}", serde_json::to_string(&tool_request).unwrap());

    // Step 4: Restore original names before sending to API
    let final_request = reinstate_reserved_keywords(&mut tool_request, &tool_mappings);
    println!("\n  Final request (ready for API):");
    println!("    {}", serde_json::to_string_pretty(&final_request).unwrap());

    println!("\n=== Example Complete ===");
}
