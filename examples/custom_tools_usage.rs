//! Example demonstrating custom tools usage
//!
//! This example shows how to create and use custom tools:
//! - Simple tools without authentication
//! - Toolkit-based tools with authentication
//! - Registering and executing custom tools

use composio_sdk::ComposioClient;
use composio_sdk::models::CustomToolsRegistry;
use serde_json::json;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let api_key = env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");
    
    let client = ComposioClient::builder()
        .api_key(&api_key)
        .build()?;
    
    println!("=== Custom Tools Example ===\n");
    
    // Create custom tools registry
    let mut registry = CustomToolsRegistry::new(client.into());
    
    // Example 1: Simple custom tool (no authentication)
    println!("1. Registering simple custom tool...");
    registry.register_simple(
        "calculate_sum",
        "Calculate the sum of two numbers",
        json!({
            "type": "object",
            "properties": {
                "a": {
                    "type": "number",
                    "description": "First number"
                },
                "b": {
                    "type": "number",
                    "description": "Second number"
                }
            },
            "required": ["a", "b"]
        }),
        |request| {
            println!("  Executing calculate_sum with: {}", request);
            let a = request["a"].as_f64().unwrap_or(0.0);
            let b = request["b"].as_f64().unwrap_or(0.0);
            let result = a + b;
            println!("  Result: {}", result);
            Ok(json!({"result": result}))
        }
    );
    println!("  ✓ Registered CALCULATE_SUM\n");
    
    // Example 2: Another simple tool
    println!("2. Registering string manipulation tool...");
    registry.register_simple(
        "reverse_string",
        "Reverse a string",
        json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "Text to reverse"
                }
            },
            "required": ["text"]
        }),
        |request| {
            println!("  Executing reverse_string with: {}", request);
            let text = request["text"].as_str().unwrap_or("");
            let reversed: String = text.chars().rev().collect();
            println!("  Result: {}", reversed);
            Ok(json!({"reversed": reversed}))
        }
    );
    println!("  ✓ Registered REVERSE_STRING\n");
    
    // Example 3: Toolkit-based custom tool (with authentication)
    println!("3. Registering toolkit-based custom tool...");
    registry.register_with_auth(
        "create_custom_issue",
        "Create a custom GitHub issue with additional processing",
        "github",
        json!({
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "Issue title"
                },
                "body": {
                    "type": "string",
                    "description": "Issue body"
                },
                "labels": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Issue labels"
                }
            },
            "required": ["title"]
        }),
        |request, _execute_request, auth_credentials: &HashMap<String, serde_json::Value>| {
            println!("  Executing create_custom_issue with: {}", request);
            println!("  Auth credentials available: {:?}", auth_credentials.keys());
            
            // In a real implementation, you would use execute_request to make
            // authenticated API calls to GitHub
            // For this example, we'll just return a mock response
            
            // Example of how you would use execute_request:
            // let response = execute_request.execute(
            //     "/repos/owner/repo/issues",
            //     "POST",
            //     Some(request),
            //     None,
            //     None,
            // )?;
            
            Ok(json!({
                "id": 12345,
                "title": request["title"],
                "state": "open",
                "created_at": "2024-01-01T00:00:00Z"
            }))
        }
    );
    println!("  ✓ Registered GITHUB_CREATE_CUSTOM_ISSUE\n");
    
    // List all registered tools
    println!("4. Listing all registered custom tools:");
    let tools = registry.list();
    for tool in &tools {
        println!("  - {} ({})", tool.slug, tool.name);
        println!("    Description: {}", tool.description);
        println!("    Requires auth: {}", tool.requires_auth);
        if let Some(toolkit) = &tool.toolkit {
            println!("    Toolkit: {}", toolkit);
        }
        println!();
    }
    
    // Execute simple tools
    println!("5. Executing simple custom tools:");
    
    // Execute calculate_sum
    println!("  Executing CALCULATE_SUM...");
    let result = registry.execute(
        "CALCULATE_SUM",
        HashMap::from([
            ("a".to_string(), json!(10)),
            ("b".to_string(), json!(32)),
        ]),
        None,
    ).await?;
    println!("  Result: {}\n", result);
    
    // Execute reverse_string
    println!("  Executing REVERSE_STRING...");
    let result = registry.execute(
        "REVERSE_STRING",
        HashMap::from([
            ("text".to_string(), json!("Hello, Composio!")),
        ]),
        None,
    ).await?;
    println!("  Result: {}\n", result);
    
    // Get tool info (for API compatibility)
    println!("6. Converting custom tools to Tool format:");
    let tool_infos = registry.list_as_tools();
    for tool_info in &tool_infos {
        println!("  Tool: {}", tool_info.slug);
        println!("    Name: {}", tool_info.name);
        println!("    Toolkit: {}", tool_info.toolkit.slug);
        println!("    No auth: {}", tool_info.no_auth);
        println!();
    }
    
    // Note about authenticated tools
    println!("7. Note about authenticated tools:");
    println!("  To execute GITHUB_CREATE_CUSTOM_ISSUE, you would need:");
    println!("  - A connected GitHub account for the user");
    println!("  - Pass user_id when calling execute()");
    println!("  Example:");
    println!("    registry.execute(");
    println!("      \"GITHUB_CREATE_CUSTOM_ISSUE\",");
    println!("      HashMap::from([(\"title\".to_string(), json!(\"My Issue\"))]),");
    println!("      Some(\"user_123\"),");
    println!("    ).await?;");
    
    println!("\n=== Custom Tools Example Complete ===");
    
    Ok(())
}
