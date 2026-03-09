//! Provider System Demonstration
//!
//! This example demonstrates the provider system, showing how to use different
//! providers (OpenAI, Anthropic) with the Composio SDK.
//!
//! Run with:
//! ```bash
//! cargo run --example provider_system_demo
//! ```

use composio_sdk::providers::{OpenAIProvider, AnthropicProvider, Provider};
use composio_sdk::models::response::ToolSchema;
use serde_json::json;

fn create_sample_tool() -> ToolSchema {
    ToolSchema {
        slug: "GITHUB_CREATE_ISSUE".to_string(),
        name: "Create GitHub Issue".to_string(),
        description: "Create a new issue in a GitHub repository".to_string(),
        toolkit: "github".to_string(),
        input_parameters: json!({
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
                    "description": "Issue body"
                }
            },
            "required": ["owner", "repo", "title"]
        }),
        output_parameters: json!({
            "type": "object",
            "properties": {
                "number": {"type": "integer"},
                "url": {"type": "string"}
            }
        }),
        version: "1.0.0".to_string(),
        available_versions: vec!["1.0.0".to_string()],
        is_deprecated: false,
        no_auth: false,
        scopes: vec!["repo".to_string()],
        tags: vec!["github".to_string(), "issues".to_string()],
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Provider System Demonstration ===\n");

    let sample_tool = create_sample_tool();

    // ========================================================================
    // 1. OpenAI Provider (Default)
    // ========================================================================
    println!("1. OpenAI Provider (Default)");
    println!("   Converting Composio tool to OpenAI format...\n");

    let openai_provider = OpenAIProvider::new();
    println!("   Provider name: {}", openai_provider.name());
    
    let openai_tool = openai_provider.wrap_tool(&sample_tool);
    let openai_json = serde_json::to_string_pretty(&openai_tool)?;
    
    println!("   OpenAI Tool Format:");
    println!("{}", openai_json);
    println!();

    // ========================================================================
    // 2. OpenAI Provider with Strict Validation
    // ========================================================================
    println!("2. OpenAI Provider with Strict Validation");
    println!("   Enabling strict schema validation...\n");

    let openai_strict = OpenAIProvider::new().with_strict(true);
    let openai_strict_tool = openai_strict.wrap_tool(&sample_tool);
    let openai_strict_json = serde_json::to_string_pretty(&openai_strict_tool)?;
    
    println!("   OpenAI Tool Format (Strict):");
    println!("{}", openai_strict_json);
    println!();

    // ========================================================================
    // 3. Anthropic Provider
    // ========================================================================
    println!("3. Anthropic Provider");
    println!("   Converting Composio tool to Anthropic format...\n");

    let anthropic_provider = AnthropicProvider::new();
    println!("   Provider name: {}", anthropic_provider.name());
    
    let anthropic_tool = anthropic_provider.wrap_tool(&sample_tool);
    let anthropic_json = serde_json::to_string_pretty(&anthropic_tool)?;
    
    println!("   Anthropic Tool Format:");
    println!("{}", anthropic_json);
    println!();

    // ========================================================================
    // 4. Comparison
    // ========================================================================
    println!("4. Format Comparison");
    println!("   ┌─────────────────────────────────────────────────────────┐");
    println!("   │ OpenAI Format                                           │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    println!("   │ • type: \"function\"                                      │");
    println!("   │ • function.name: \"GITHUB_CREATE_ISSUE\"                  │");
    println!("   │ • function.description: \"...\"                           │");
    println!("   │ • function.parameters: {{...}}                           │");
    println!("   │ • function.strict: true/false (optional)                │");
    println!("   └─────────────────────────────────────────────────────────┘");
    println!();
    println!("   ┌─────────────────────────────────────────────────────────┐");
    println!("   │ Anthropic Format                                        │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    println!("   │ • name: \"GITHUB_CREATE_ISSUE\"                           │");
    println!("   │ • description: \"...\"                                    │");
    println!("   │ • input_schema: {{...}}                                  │");
    println!("   └─────────────────────────────────────────────────────────┘");
    println!();

    // ========================================================================
    // 5. Multiple Tools
    // ========================================================================
    println!("5. Converting Multiple Tools");
    
    let tools = vec![
        sample_tool.clone(),
        ToolSchema {
            slug: "GITHUB_GET_ISSUE".to_string(),
            name: "Get GitHub Issue".to_string(),
            description: "Get details of a GitHub issue".to_string(),
            toolkit: "github".to_string(),
            input_parameters: json!({"type": "object", "properties": {}}),
            output_parameters: json!({}),
            version: "1.0.0".to_string(),
            available_versions: vec![],
            is_deprecated: false,
            no_auth: false,
            scopes: vec![],
            tags: vec![],
        },
    ];

    let openai_tools = openai_provider.wrap_tools(tools.clone());
    let anthropic_tools = anthropic_provider.wrap_tools(tools);

    println!("   OpenAI: Converted {} tools", openai_tools.len());
    println!("   Anthropic: Converted {} tools", anthropic_tools.len());
    println!();

    // ========================================================================
    // 6. Usage with ComposioClient
    // ========================================================================
    println!("6. Usage with ComposioClient");
    println!();
    println!("   // Default OpenAI provider");
    println!("   let client = ComposioClient::builder()");
    println!("       .api_key(\"your_key\")");
    println!("       .build()?;");
    println!();
    println!("   // Explicit OpenAI provider");
    println!("   let client = ComposioClient::with_provider(OpenAIProvider::new())");
    println!("       .api_key(\"your_key\")");
    println!("       .build()?;");
    println!();
    println!("   // Anthropic provider");
    println!("   let client = ComposioClient::with_provider(AnthropicProvider::new())");
    println!("       .api_key(\"your_key\")");
    println!("       .build()?;");
    println!();
    println!("   // OpenAI with strict validation");
    println!("   let client = ComposioClient::with_provider(");
    println!("       OpenAIProvider::new().with_strict(true)");
    println!("   )");
    println!("       .api_key(\"your_key\")");
    println!("       .build()?;");
    println!();

    // ========================================================================
    // 7. Benefits
    // ========================================================================
    println!("7. Benefits of Provider System");
    println!("   ✓ Type Safety: Compile-time guarantees for tool formats");
    println!("   ✓ Zero-Cost: Trait resolution at compile-time (no runtime overhead)");
    println!("   ✓ Extensible: Easy to add new providers");
    println!("   ✓ Flexible: Switch providers without changing code");
    println!("   ✓ Framework Agnostic: Works with any AI framework");
    println!();

    println!("=== Demo Complete ===");

    Ok(())
}
