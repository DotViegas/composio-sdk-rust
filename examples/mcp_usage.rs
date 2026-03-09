//! Example demonstrating MCP (Model Context Protocol) operations
//!
//! This example shows how to:
//! - Create MCP server configurations
//! - List existing MCP servers
//! - Generate user-specific MCP URLs
//! - Update and delete MCP servers
//!
//! # Usage
//!
//! ```bash
//! export COMPOSIO_API_KEY="your-api-key"
//! cargo run --example mcp_usage
//! ```

use composio_sdk::ComposioError;

#[tokio::main]
async fn main() -> Result<(), ComposioError> {
    // Note: This example demonstrates MCP data structures and workflows
    // without requiring an actual API key or making API calls

    println!("=== MCP (Model Context Protocol) Operations Example ===\n");

    // Example 1: Create an MCP server with simple toolkit names
    println!("1. Creating MCP server with simple toolkit configuration...");
    
    // Note: This is a demonstration. In a real scenario, you would use the actual
    // MCP API endpoints through the client. The following shows the data structures:
    
    use composio_sdk::models::mcp::{
        MCPToolkitConfig, MCPServerInstance, MCPCreateParams, MCPListParams
    };

    // Create toolkit configurations
    let toolkit_configs = vec![
        MCPToolkitConfig::new("github"),
        MCPToolkitConfig::new("slack").with_auth_config("ac_slack_123"),
    ];

    println!("   Toolkit configs created:");
    for config in &toolkit_configs {
        println!("   - Toolkit: {}", config.toolkit);
        if let Some(auth_id) = &config.auth_config_id {
            println!("     Auth Config: {}", auth_id);
        }
    }

    // Example 2: Create MCP server parameters
    println!("\n2. Preparing MCP server creation parameters...");
    
    let create_params = MCPCreateParams {
        name: "my-mcp-server".to_string(),
        toolkits: vec!["github".to_string(), "slack".to_string()],
        auth_config_ids: Some(vec!["ac_slack_123".to_string()]),
        custom_tools: Some(vec![
            "GITHUB_CREATE_ISSUE".to_string(),
            "GITHUB_LIST_REPOS".to_string(),
            "SLACK_SEND_MESSAGE".to_string(),
        ]),
        managed_auth_via_composio: Some(true),
    };

    println!("   Server name: {}", create_params.name);
    println!("   Toolkits: {:?}", create_params.toolkits);
    println!("   Allowed tools: {:?}", create_params.custom_tools);
    println!("   Managed auth: {:?}", create_params.managed_auth_via_composio);

    // Example 3: List MCP servers with filters
    println!("\n3. Preparing MCP server list parameters...");
    
    let list_params = MCPListParams {
        page_no: Some(1),
        limit: Some(10),
        toolkits: Some("github".to_string()),
        auth_config_ids: None,
        name: Some("my-mcp".to_string()),
        order_by: Some("created_at".to_string()),
        order_direction: Some("desc".to_string()),
    };

    println!("   Page: {:?}", list_params.page_no);
    println!("   Limit: {:?}", list_params.limit);
    println!("   Filter by toolkit: {:?}", list_params.toolkits);
    println!("   Filter by name: {:?}", list_params.name);
    println!("   Order by: {:?} {:?}", list_params.order_by, list_params.order_direction);

    // Example 4: MCP Server Instance structure
    println!("\n4. Example MCP Server Instance structure:");
    
    let example_instance = MCPServerInstance {
        id: "mcp_abc123".to_string(),
        name: "My Personal MCP Server".to_string(),
        server_type: "streamable_http".to_string(),
        url: "https://mcp.composio.dev/user_123/mcp_abc123".to_string(),
        user_id: "user_123".to_string(),
        allowed_tools: vec![
            "GITHUB_CREATE_ISSUE".to_string(),
            "GITHUB_LIST_REPOS".to_string(),
            "SLACK_SEND_MESSAGE".to_string(),
        ],
        auth_configs: vec!["ac_github_456".to_string(), "ac_slack_123".to_string()],
    };

    println!("   Instance ID: {}", example_instance.id);
    println!("   Name: {}", example_instance.name);
    println!("   Type: {}", example_instance.server_type);
    println!("   URL: {}", example_instance.url);
    println!("   User ID: {}", example_instance.user_id);
    println!("   Allowed tools: {:?}", example_instance.allowed_tools);
    println!("   Auth configs: {:?}", example_instance.auth_configs);

    // Example 5: Serialization/Deserialization
    println!("\n5. Testing serialization and deserialization...");
    
    let json = serde_json::to_string_pretty(&example_instance)?;
    println!("   Serialized to JSON:");
    println!("{}", json);

    let deserialized: MCPServerInstance = serde_json::from_str(&json)?;
    println!("\n   Deserialized successfully!");
    println!("   Verified ID: {}", deserialized.id);

    // Example 6: Using From trait for toolkit configs
    println!("\n6. Creating toolkit configs using From trait...");
    
    let simple_config: MCPToolkitConfig = "gmail".into();
    println!("   Simple config: toolkit = {}", simple_config.toolkit);
    
    let string_config: MCPToolkitConfig = String::from("calendar").into();
    println!("   String config: toolkit = {}", string_config.toolkit);

    // Example 7: Common MCP workflows
    println!("\n7. Common MCP workflows:");
    
    println!("\n   Workflow 1: Create server for a specific user");
    println!("   - Create MCP server with toolkits");
    println!("   - Generate user-specific URL");
    println!("   - User connects AI assistant using the URL");
    
    println!("\n   Workflow 2: Update server configuration");
    println!("   - List existing servers");
    println!("   - Update allowed tools or auth configs");
    println!("   - Regenerate URLs for affected users");
    
    println!("\n   Workflow 3: Multi-user MCP server");
    println!("   - Create one MCP server configuration");
    println!("   - Generate unique URLs for each user");
    println!("   - Each user gets isolated access with their credentials");

    // Example 8: Integration with AI assistants
    println!("\n8. Integration examples:");
    
    println!("\n   Claude Desktop:");
    println!("   Add to claude_desktop_config.json:");
    println!(r#"   {{
     "mcpServers": {{
       "composio": {{
         "command": "npx",
         "args": ["-y", "@composio/mcp-server-js"],
         "env": {{
           "COMPOSIO_MCP_URL": "{}",
           "COMPOSIO_MCP_API_KEY": "your-api-key"
         }}
       }}
     }}
   }}"#, example_instance.url);

    println!("\n   OpenAI Agents SDK:");
    println!("   Use MCP tools in your agent:");
    println!(r#"   const agent = new Agent({{
     model: "gpt-4",
     mcp_servers: [{{
       url: "{}",
       headers: {{ "x-api-key": "your-api-key" }}
     }}]
   }});"#, example_instance.url);

    println!("\n=== Example completed successfully! ===");
    println!("\nNote: This example demonstrates the MCP data structures and workflows.");
    println!("To actually create and manage MCP servers, you would use the Composio client");
    println!("methods that interact with the MCP API endpoints.");

    Ok(())
}
