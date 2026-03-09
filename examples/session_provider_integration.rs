//! Example demonstrating Session + Provider integration
//!
//! This example shows how to use the provider system with sessions to get
//! meta tools formatted for different AI frameworks (OpenAI, Anthropic).
//!
//! # Usage
//!
//! ```bash
//! export COMPOSIO_API_KEY="your_api_key"
//! cargo run --example session_provider_integration
//! ```

use composio_sdk::{ComposioClient, providers::{OpenAIProvider, AnthropicProvider}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = ComposioClient::builder()
        .api_key(std::env::var("COMPOSIO_API_KEY")?)
        .build()?;

    println!("🚀 Session + Provider Integration Demo\n");

    // Create a session for a user
    println!("📝 Creating session for user_123...");
    let session = client
        .create_session("user_123")
        .toolkits(vec!["github", "gmail"])
        .send()
        .await?;

    println!("✅ Session created: {}", session.session_id());
    println!("   MCP URL: {}\n", session.mcp_url());

    // ========================================
    // Example 1: Get tools in universal format
    // ========================================
    println!("📦 Example 1: Universal Format (ToolSchema)");
    println!("─────────────────────────────────────────");
    
    let universal_tools = session.get_meta_tools().await?;
    println!("Found {} meta tools in universal format:", universal_tools.len());
    
    for tool in &universal_tools {
        println!("  • {} - {}", tool.slug, tool.name);
    }
    println!();

    // ========================================
    // Example 2: Get tools in OpenAI format
    // ========================================
    println!("🤖 Example 2: OpenAI Format (ChatCompletionToolParam)");
    println!("─────────────────────────────────────────────────────");
    
    let openai_provider = OpenAIProvider::new();
    let openai_tools = session.get_provider_tools(&openai_provider).await?;
    
    println!("Found {} meta tools in OpenAI format:", openai_tools.len());
    
    for tool in &openai_tools {
        println!("  • Type: {}", tool.r#type);
        println!("    Function: {}", tool.function.name);
        println!("    Description: {}", tool.function.description);
        if let Some(strict) = tool.function.strict {
            println!("    Strict: {}", strict);
        }
    }
    println!();

    // ========================================
    // Example 3: Get tools in OpenAI format with strict validation
    // ========================================
    println!("🔒 Example 3: OpenAI Format with Strict Validation");
    println!("──────────────────────────────────────────────────");
    
    let openai_strict_provider = OpenAIProvider::new().with_strict(true);
    let openai_strict_tools = session.get_provider_tools(&openai_strict_provider).await?;
    
    println!("Found {} meta tools with strict validation:", openai_strict_tools.len());
    
    for tool in &openai_strict_tools {
        println!("  • {}", tool.function.name);
        if let Some(strict) = tool.function.strict {
            println!("    Strict validation: {}", strict);
        }
    }
    println!();

    // ========================================
    // Example 4: Get tools in Anthropic format
    // ========================================
    println!("🧠 Example 4: Anthropic Format (AnthropicTool)");
    println!("──────────────────────────────────────────");
    
    let anthropic_provider = AnthropicProvider::new();
    let anthropic_tools = session.get_provider_tools(&anthropic_provider).await?;
    
    println!("Found {} meta tools in Anthropic format:", anthropic_tools.len());
    
    for tool in &anthropic_tools {
        println!("  • Name: {}", tool.name);
        println!("    Description: {}", tool.description);
        println!("    Input schema type: {}", tool.input_schema.get("type").unwrap_or(&serde_json::Value::Null));
    }
    println!();

    // ========================================
    // Example 5: Serialize to JSON (for API calls)
    // ========================================
    println!("📄 Example 5: Serialized JSON (ready for API calls)");
    println!("───────────────────────────────────────────────────");
    
    // OpenAI format
    let openai_json = serde_json::to_string_pretty(&openai_tools)?;
    println!("OpenAI format (first 500 chars):");
    println!("{}", &openai_json[..openai_json.len().min(500)]);
    println!("...\n");
    
    // Anthropic format
    let anthropic_json = serde_json::to_string_pretty(&anthropic_tools)?;
    println!("Anthropic format (first 500 chars):");
    println!("{}", &anthropic_json[..anthropic_json.len().min(500)]);
    println!("...\n");

    // ========================================
    // Example 6: Practical usage pattern
    // ========================================
    println!("💡 Example 6: Practical Usage Pattern");
    println!("─────────────────────────────────────");
    println!("
// In a real application, you would use these tools like this:

// For OpenAI:
let openai_tools = session.get_provider_tools(&OpenAIProvider::new()).await?;
// let response = openai_client.chat().completions().create(
//     ChatCompletionRequest {{
//         model: \"gpt-4\",
//         messages: vec![...],
//         tools: Some(openai_tools),
//         ...
//     }}
// ).await?;

// For Anthropic:
let anthropic_tools = session.get_provider_tools(&AnthropicProvider::new()).await?;
// let response = anthropic_client.messages().create(
//     MessageRequest {{
//         model: \"claude-3-5-sonnet-20241022\",
//         messages: vec![...],
//         tools: Some(anthropic_tools),
//         ...
//     }}
// ).await?;
");

    println!("✅ Demo completed successfully!");
    
    Ok(())
}
