//! Example demonstrating tools management functionality
//!
//! This example shows how to:
//! - List available tools with filtering
//! - Get detailed information about specific tools
//! - Execute tools with arguments
//! - Use proxy execution for custom API calls
//! - Generate tool inputs from natural language
//!
//! Run with: cargo run --example tools_usage

use composio_sdk::client::ComposioClient;
use composio_sdk::models::tools::{
    ToolListParams, ToolExecuteParams, ToolProxyParams, HttpMethod,
    ToolInputGenerationParams,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let api_key = std::env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");

    let client = ComposioClient::builder()
        .api_key(api_key)
        .build()?;

    println!("=== Composio Tools Management Example ===\n");

    // Example 1: List tools from a specific toolkit
    println!("--- Example 1: List GitHub Tools ---");
    let params = ToolListParams {
        toolkit_slug: Some("github".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    let tools = client.list_tools(params).await?;
    println!("Found {} GitHub tools (showing first 5):", tools.items.len());
    for tool in &tools.items {
        println!("  - {} ({})", tool.name, tool.slug);
        println!("    Description: {}", tool.description);
        println!("    Version: {}", tool.version);
        println!("    Scopes: {:?}", tool.scopes);
        println!("    Tags: {:?}", tool.tags);
        println!();
    }

    // Example 2: Search for tools
    println!("--- Example 2: Search Tools ---");
    let params = ToolListParams {
        search: Some("create issue".to_string()),
        limit: Some(3),
        ..Default::default()
    };

    let search_results = client.list_tools(params).await?;
    println!("Tools matching 'create issue':");
    for tool in &search_results.items {
        println!("  - {} ({})", tool.name, tool.slug);
        println!("    Toolkit: {}", tool.toolkit.slug);
    }
    println!();

    // Example 3: Get specific tool details
    println!("--- Example 3: Get Tool Details ---");
    let tool = client.get_tool("GITHUB_GET_REPOS").await?;
    println!("Tool: {}", tool.name);
    println!("Slug: {}", tool.slug);
    println!("Description: {}", tool.description);
    println!("Toolkit: {} ({})", tool.toolkit.name.unwrap_or_default(), tool.toolkit.slug);
    println!("Version: {}", tool.version);
    println!("Available versions: {:?}", tool.available_versions);
    println!("Deprecated: {}", tool.is_deprecated);
    println!("Requires auth: {}", !tool.no_auth);
    println!("Input schema: {}", serde_json::to_string_pretty(&tool.input_parameters)?);
    println!();

    // Example 4: Filter tools by tags
    println!("--- Example 4: Filter by Tags ---");
    let params = ToolListParams {
        toolkit_slug: Some("github".to_string()),
        tags: Some(vec!["write".to_string()]),
        limit: Some(3),
        ..Default::default()
    };

    let write_tools = client.list_tools(params).await?;
    println!("GitHub write tools:");
    for tool in &write_tools.items {
        println!("  - {} ({})", tool.name, tool.slug);
    }
    println!();

    // Example 5: Filter tools by scopes
    println!("--- Example 5: Filter by Scopes ---");
    let params = ToolListParams {
        toolkit_slug: Some("github".to_string()),
        scopes: Some(vec!["repo".to_string()]),
        limit: Some(3),
        ..Default::default()
    };

    let scoped_tools = client.list_tools(params).await?;
    println!("GitHub tools requiring 'repo' scope:");
    for tool in &scoped_tools.items {
        println!("  - {} ({})", tool.name, tool.slug);
        println!("    Scopes: {:?}", tool.scopes);
    }
    println!();

    // Example 6: Execute a tool (commented out - requires authentication)
    println!("--- Example 6: Execute Tool ---");
    println!("Note: This requires a connected account");
    
    // Uncomment to actually execute:
    // let mut arguments = HashMap::new();
    // arguments.insert("owner".to_string(), serde_json::json!("composio"));
    // arguments.insert("repo".to_string(), serde_json::json!("composio"));
    //
    // let params = ToolExecuteParams {
    //     slug: "GITHUB_GET_REPOS".to_string(),
    //     arguments,
    //     user_id: Some("user_123".to_string()),
    //     version: Some("1.0.0".to_string()),
    //     ..Default::default()
    // };
    //
    // let result = client.execute_tool(params).await?;
    // println!("Execution successful: {}", result.successful);
    // if let Some(error) = result.error {
    //     println!("Error: {}", error);
    // } else {
    //     println!("Result: {:?}", result.data);
    // }
    
    println!("(Skipped - uncomment code to test)");
    println!();

    // Example 7: Proxy execution (commented out - requires authentication)
    println!("--- Example 7: Proxy Execution ---");
    println!("Note: This makes a direct API call using connected account credentials");
    
    // Uncomment to actually execute:
    // let params = ToolProxyParams {
    //     endpoint: "/repos/composio/composio".to_string(),
    //     method: HttpMethod::Get,
    //     connected_account_id: Some("ca_123".to_string()),
    //     body: None,
    //     parameters: None,
    //     custom_connection_data: None,
    // };
    //
    // let result = client.proxy_tool(params).await?;
    // println!("Status: {}", result.status);
    // println!("Successful: {}", result.successful);
    // println!("Data: {:?}", result.data);
    
    println!("(Skipped - uncomment code to test)");
    println!();

    // Example 8: Generate tool inputs from natural language
    println!("--- Example 8: Generate Tool Inputs ---");
    println!("Note: This uses AI to convert natural language to structured arguments");
    
    // Uncomment to actually execute:
    // let params = ToolInputGenerationParams {
    //     tool_slug: "GITHUB_CREATE_ISSUE".to_string(),
    //     text: "Create an issue about fixing the login bug in the composio repo".to_string(),
    //     custom_tool_description: None,
    //     custom_system_prompt: None,
    // };
    //
    // let result = client.generate_tool_inputs(params).await?;
    // if result.successful {
    //     if let Some(arguments) = result.arguments {
    //         println!("Generated arguments:");
    //         for (key, value) in arguments {
    //             println!("  {}: {}", key, value);
    //         }
    //     }
    // } else if let Some(error) = result.error {
    //     println!("Error: {}", error);
    // }
    
    println!("(Skipped - uncomment code to test)");
    println!();

    // Example 9: List tools with specific slugs
    println!("--- Example 9: Get Multiple Tools by Slug ---");
    let params = ToolListParams {
        tool_slugs: Some(vec![
            "GITHUB_CREATE_ISSUE".to_string(),
            "GITHUB_GET_REPOS".to_string(),
            "GITHUB_CREATE_PR".to_string(),
        ]),
        ..Default::default()
    };

    let specific_tools = client.list_tools(params).await?;
    println!("Requested tools:");
    for tool in &specific_tools.items {
        println!("  - {} ({})", tool.name, tool.slug);
    }
    println!();

    // Example 10: Pagination
    println!("--- Example 10: Pagination ---");
    let params = ToolListParams {
        toolkit_slug: Some("github".to_string()),
        limit: Some(10),
        ..Default::default()
    };

    let page1 = client.list_tools(params.clone()).await?;
    println!("Page 1: {} tools", page1.items.len());
    
    if let Some(next_cursor) = page1.next_cursor {
        let params = ToolListParams {
            cursor: Some(next_cursor),
            ..params
        };
        let page2 = client.list_tools(params).await?;
        println!("Page 2: {} tools", page2.items.len());
    }
    println!();

    println!("=== Example Complete ===");
    println!("\nKey Takeaways:");
    println!("1. Tools are individual actions within toolkits");
    println!("2. You can filter by toolkit, search, tags, scopes, and more");
    println!("3. Each tool has detailed input/output schemas");
    println!("4. execute_tool() runs tools with user authentication");
    println!("5. proxy_tool() allows direct API calls to any endpoint");
    println!("6. generate_tool_inputs() converts natural language to arguments");
    println!("7. Pagination is supported for large result sets");

    Ok(())
}
