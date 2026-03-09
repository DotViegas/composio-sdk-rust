//! OpenAPI schema utilities usage example
//!
//! This example demonstrates how to work with OpenAPI schemas in Rust,
//! including type conversion, validation, and parameter extraction.

use composio_sdk::utils::openapi::{extract_parameters, merge_schemas, OpenApiSchema};
use serde_json::json;

fn main() {
    println!("=== OpenAPI Schema Utilities Example ===\n");

    // Example 1: Basic type validation
    println!("1. Basic Type Validation");
    let string_schema = json!({"type": "string"});
    let schema = OpenApiSchema::new(string_schema);

    match schema.validate(&json!("hello")) {
        Ok(_) => println!("   ✓ Valid string value"),
        Err(e) => println!("   ✗ Invalid: {}", e),
    }

    match schema.validate(&json!(123)) {
        Ok(_) => println!("   ✓ Valid value"),
        Err(e) => println!("   ✗ Invalid: {} (expected)", e),
    }

    // Example 2: Enum validation
    println!("\n2. Enum Validation");
    let enum_schema = json!({
        "type": "string",
        "enum": ["low", "medium", "high"]
    });
    let schema = OpenApiSchema::new(enum_schema);

    println!("   Enum values: {:?}", schema.get_enum_values());
    match schema.validate(&json!("medium")) {
        Ok(_) => println!("   ✓ Valid enum value: 'medium'"),
        Err(e) => println!("   ✗ Invalid: {}", e),
    }

    match schema.validate(&json!("critical")) {
        Ok(_) => println!("   ✓ Valid value"),
        Err(e) => println!("   ✗ Invalid: {} (expected)", e),
    }

    // Example 3: Array validation
    println!("\n3. Array Validation");
    let array_schema = json!({
        "type": "array",
        "items": {"type": "string"}
    });
    let schema = OpenApiSchema::new(array_schema);

    match schema.validate(&json!(["tag1", "tag2", "tag3"])) {
        Ok(_) => println!("   ✓ Valid array of strings"),
        Err(e) => println!("   ✗ Invalid: {}", e),
    }

    match schema.validate(&json!([1, 2, 3])) {
        Ok(_) => println!("   ✓ Valid array"),
        Err(e) => println!("   ✗ Invalid: {} (expected)", e),
    }

    // Example 4: Object schema with properties
    println!("\n4. Object Schema Properties");
    let object_schema = json!({
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
            },
            "labels": {
                "type": "array",
                "items": {"type": "string"}
            }
        },
        "required": ["title"]
    });

    let schema = OpenApiSchema::new(object_schema.clone());
    let properties = schema.get_properties();
    let required = schema.get_required();

    println!("   Properties: {}", properties.len());
    for (name, prop_schema) in properties {
        println!(
            "     - {}: {} {}",
            name,
            prop_schema.to_rust_type_string(),
            if required.contains(&name) {
                "(required)"
            } else {
                "(optional)"
            }
        );
    }

    // Example 5: Extract parameters
    println!("\n5. Parameter Extraction");
    let params = extract_parameters(&object_schema);
    println!("   Extracted {} parameters:", params.len());
    for param in params {
        println!("     - {}", param.name);
        println!("       Type: {}", param.type_description);
        println!("       Required: {}", param.required);
        if let Some(desc) = param.description {
            println!("       Description: {}", desc);
        }
        if let Some(default) = param.default {
            println!("       Default: {}", default);
        }
    }

    // Example 6: Composite schemas (oneOf)
    println!("\n6. Composite Schemas (oneOf)");
    let oneof_schema = json!({
        "oneOf": [
            {"type": "string"},
            {"type": "integer"},
            {"type": "boolean"}
        ]
    });
    let schema = OpenApiSchema::new(oneof_schema);

    if let Some((composite_type, schemas)) = schema.get_composite_schemas() {
        println!("   Composite type: {:?}", composite_type);
        println!("   Options: {}", schemas.len());
        for (i, s) in schemas.iter().enumerate() {
            println!("     {}. {}", i + 1, s.to_rust_type_string());
        }
    }

    println!("   Rust type: {}", schema.to_rust_type_string());

    // Example 7: Merge schemas (allOf)
    println!("\n7. Merge Schemas (allOf)");
    let schema1 = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "email": {"type": "string"}
        },
        "required": ["name"]
    });

    let schema2 = json!({
        "type": "object",
        "properties": {
            "age": {"type": "integer"},
            "city": {"type": "string"}
        },
        "required": ["age"]
    });

    let merged = merge_schemas(&[schema1, schema2]);
    println!("   Merged schema:");
    println!("{}", serde_json::to_string_pretty(&merged).unwrap());

    // Example 8: Real-world tool schema
    println!("\n8. Real-World Tool Schema (GitHub Create Issue)");
    let github_issue_schema = json!({
        "type": "object",
        "properties": {
            "owner": {
                "type": "string",
                "description": "Repository owner"
            },
            "repo": {
                "type": "string",
                "description": "Repository name"
            },
            "title": {
                "type": "string",
                "description": "Issue title"
            },
            "body": {
                "type": "string",
                "description": "Issue body",
                "default": ""
            },
            "assignees": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Usernames to assign"
            },
            "labels": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Labels to add"
            },
            "milestone": {
                "type": "integer",
                "description": "Milestone number"
            }
        },
        "required": ["owner", "repo", "title"]
    });

    let params = extract_parameters(&github_issue_schema);
    println!("   GitHub Create Issue Parameters:");
    for param in params {
        let required_marker = if param.required { "*" } else { " " };
        println!(
            "   {} {}: {}",
            required_marker, param.name, param.type_description
        );
        if let Some(desc) = param.description {
            println!("       {}", desc);
        }
    }

    // Example 9: Type conversion examples
    println!("\n9. Type Conversion Examples");
    let examples = vec![
        ("string", json!({"type": "string"})),
        ("integer", json!({"type": "integer"})),
        ("boolean", json!({"type": "boolean"})),
        ("array", json!({"type": "array", "items": {"type": "string"}})),
        ("object", json!({"type": "object"})),
        (
            "enum",
            json!({"type": "string", "enum": ["a", "b", "c"]}),
        ),
    ];

    for (name, schema_json) in examples {
        let schema = OpenApiSchema::new(schema_json);
        println!("   {} → {}", name, schema.to_rust_type_string());
    }

    println!("\n=== Example Complete ===");
}
