//! Example demonstrating tool modifiers usage
//!
//! This example shows how to use modifiers to customize tool behavior:
//! - BeforeExecute: Modify parameters before tool execution
//! - AfterExecute: Modify response after tool execution
//! - SchemaModifier: Modify tool schema before agent sees it
//!
//! Run with:
//! ```bash
//! cargo run --example modifiers_usage
//! ```

use composio_sdk::models::{
    AfterExecute, BeforeExecute, Modifier, SchemaModifier, ToolExecuteParams,
    ToolExecutionResponse, ToolSchema,
};
use std::collections::HashMap;

// Example BeforeExecute modifier that injects default labels
struct InjectLabelsModifier;

impl BeforeExecute for InjectLabelsModifier {
    fn modify(&self, tool: &str, toolkit: &str, mut params: ToolExecuteParams) -> ToolExecuteParams {
        println!("🔧 BeforeExecute: Injecting labels for {}/{}", toolkit, tool);
        
        // Add default labels to GitHub issue creation
        if tool == "GITHUB_CREATE_ISSUE" {
            params.arguments.insert(
                "labels".to_string(),
                serde_json::json!(["automated", "sdk-generated"]),
            );
        }
        
        params
    }
}

// Example AfterExecute modifier that logs results
struct LogResultModifier;

impl AfterExecute for LogResultModifier {
    fn modify(
        &self,
        tool: &str,
        toolkit: &str,
        response: ToolExecutionResponse,
    ) -> ToolExecutionResponse {
        println!("📊 AfterExecute: Tool {}/{} executed", toolkit, tool);
        println!("   Success: {}", response.successful);
        
        if !response.data.is_null() {
            println!("   Data: {}", serde_json::to_string_pretty(&response.data).unwrap_or_default());
        }
        
        response
    }
}

// Example SchemaModifier that updates tool descriptions
struct EnhanceDescriptionModifier;

impl SchemaModifier for EnhanceDescriptionModifier {
    fn modify(&self, tool: &str, toolkit: &str, mut schema: ToolSchema) -> ToolSchema {
        println!("📝 SchemaModifier: Enhancing description for {}/{}", toolkit, tool);
        
        // Add a prefix to the description
        schema.description = format!("[Enhanced] {}", schema.description);
        
        schema
    }
}

fn main() {
    println!("=== Composio SDK - Tool Modifiers Example ===\n");

    // Create modifiers
    let before_modifier = Modifier::before_execute(
        vec!["GITHUB_CREATE_ISSUE".to_string()],
        vec![],
        InjectLabelsModifier,
    );

    let after_modifier = Modifier::after_execute(
        vec![], // Apply to all tools
        vec!["github".to_string()],
        LogResultModifier,
    );

    let schema_modifier = Modifier::schema(
        vec!["GITHUB_CREATE_ISSUE".to_string()],
        vec![],
        EnhanceDescriptionModifier,
    );

    // Collect modifiers
    let modifiers = vec![before_modifier, after_modifier, schema_modifier];

    println!("✅ Created {} modifiers\n", modifiers.len());

    // Example 1: Apply BeforeExecute modifier
    println!("--- Example 1: BeforeExecute Modifier ---");
    let mut params = ToolExecuteParams {
        allow_tracing: None,
        arguments: {
            let mut args = HashMap::new();
            args.insert("title".to_string(), serde_json::json!("Test Issue"));
            args.insert("body".to_string(), serde_json::json!("This is a test"));
            args
        },
        connected_account_id: Some("ca_test123".to_string()),
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    println!("Before modification:");
    println!("  Arguments: {:?}", params.arguments.keys().collect::<Vec<_>>());

    // Apply before execute modifiers
    params = composio_sdk::models::apply_before_execute_modifiers(
        &modifiers,
        "GITHUB_CREATE_ISSUE",
        "github",
        params,
    )
    .expect("Failed to apply modifiers");

    println!("After modification:");
    println!("  Arguments: {:?}", params.arguments.keys().collect::<Vec<_>>());
    if let Some(labels) = params.arguments.get("labels") {
        println!("  Labels: {}", labels);
    }
    println!();

    // Example 2: Apply AfterExecute modifier
    println!("--- Example 2: AfterExecute Modifier ---");
    let response = ToolExecutionResponse {
        successful: true,
        data: serde_json::json!({
            "id": 12345,
            "number": 42,
            "title": "Test Issue",
            "state": "open"
        }),
        error: None,
        log_id: "log_abc123".to_string(),
    };

    let modified_response = composio_sdk::models::apply_after_execute_modifiers(
        &modifiers,
        "GITHUB_CREATE_ISSUE",
        "github",
        response,
    )
    .expect("Failed to apply modifiers");

    println!("Response processed: {}", modified_response.successful);
    println!();

    // Example 3: Apply SchemaModifier
    println!("--- Example 3: SchemaModifier ---");
    let schema = ToolSchema {
        slug: "GITHUB_CREATE_ISSUE".to_string(),
        name: "Create Issue".to_string(),
        description: "Create a new issue in a GitHub repository".to_string(),
        toolkit: "github".to_string(),
        input_parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "body": {"type": "string"}
            }
        }),
        output_parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "number"}
            }
        }),
        scopes: vec!["repo".to_string()],
        tags: vec!["write".to_string()],
        version: "1.0.0".to_string(),
        available_versions: vec!["1.0.0".to_string()],
        is_deprecated: false,
        no_auth: false,
    };

    println!("Original description: {}", schema.description);

    let modified_schema = composio_sdk::models::apply_schema_modifiers(
        &modifiers,
        "GITHUB_CREATE_ISSUE",
        "github",
        schema,
    )
    .expect("Failed to apply modifiers");

    println!("Modified description: {}", modified_schema.description);
    println!();

    // Example 4: Modifier filtering
    println!("--- Example 4: Modifier Filtering ---");
    println!("Modifiers only apply to specified tools/toolkits:");
    
    let other_params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    // This won't trigger the BeforeExecute modifier (different tool)
    let _result = composio_sdk::models::apply_before_execute_modifiers(
        &modifiers,
        "GITHUB_GET_REPO",
        "github",
        other_params,
    )
    .expect("Failed to apply modifiers");

    println!("✅ Modifiers correctly filtered for non-matching tools");
    println!();

    println!("=== Example Complete ===");
    println!("\nKey Takeaways:");
    println!("1. BeforeExecute modifiers can inject or modify parameters");
    println!("2. AfterExecute modifiers can process or transform responses");
    println!("3. SchemaModifier can customize tool schemas for agents");
    println!("4. Modifiers can target specific tools or toolkits");
    println!("5. Multiple modifiers are applied in sequence");
}
