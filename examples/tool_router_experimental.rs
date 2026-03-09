//! Example demonstrating experimental Tool Router features
//!
//! This example shows how to use experimental features like assistive prompts
//! with timezone configuration.
//!
//! # Usage
//!
//! ```bash
//! COMPOSIO_API_KEY=your_key cargo run --example tool_router_experimental
//! ```

use composio_sdk::ComposioClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment
    let api_key = std::env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");

    // Create client
    let client = ComposioClient::builder()
        .api_key(&api_key)
        .build()?;

    println!("=== Tool Router Experimental Features Demo ===\n");

    // Example 1: Create session with experimental assistive prompt
    println!("1. Creating session with timezone-aware assistive prompt...");
    let session = client
        .create_session("user_experimental_demo")
        .toolkits(vec!["github", "gmail"])
        .experimental(Some("America/New_York".to_string()))
        .send()
        .await?;

    println!("   ✓ Session created: {}", session.session_id());
    println!("   ✓ MCP URL: {}", session.mcp_url());
    println!();

    // Example 2: Get meta tools to see available tools
    println!("2. Fetching meta tools...");
    let meta_tools = session.get_meta_tools().await?;
    println!("   ✓ Found {} meta tools:", meta_tools.len());
    for tool in &meta_tools {
        println!("     - {} ({})", tool.name, tool.slug);
    }
    println!();

    // Example 3: List toolkits with connection status
    println!("3. Listing toolkits with connection status...");
    let toolkits = session.list_toolkits().send().await?;
    println!("   ✓ Found {} toolkits:", toolkits.items.len());
    for toolkit in toolkits.items.iter().take(5) {
        let status = if toolkit.is_no_auth {
            "No auth required".to_string()
        } else if let Some(ref conn) = toolkit.connection {
            if conn.is_active {
                "Connected".to_string()
            } else {
                "Not connected".to_string()
            }
        } else {
            "Not connected".to_string()
        };
        println!("     - {} ({}): {}", toolkit.name, toolkit.slug, status);
    }
    println!();

    // Example 4: Create auth link for a toolkit
    println!("4. Creating authentication link for GitHub...");
    match session.create_auth_link("github", None).await {
        Ok(link) => {
            println!("   ✓ Auth link created:");
            println!("     - Link token: {}", link.link_token);
            println!("     - Redirect URL: {}", link.redirect_url);
            if let Some(account_id) = link.connected_account_id {
                println!("     - Connected account: {}", account_id);
            }
        }
        Err(e) => {
            println!("   ⚠ Could not create auth link: {}", e);
            println!("     (This is expected if already connected)");
        }
    }
    println!();

    // Example 5: Demonstrate workbench configuration
    println!("5. Creating session with workbench configuration...");
    let workbench_session = client
        .create_session("user_workbench_demo")
        .toolkits(vec!["github"])
        .workbench(Some(true), Some(1000))
        .send()
        .await?;

    println!("   ✓ Workbench session created: {}", workbench_session.session_id());
    println!();

    // Example 6: Demonstrate tag-based filtering
    println!("6. Creating session with tag-based tool filtering...");
    use composio_sdk::models::enums::TagType;
    
    let filtered_session = client
        .create_session("user_filtered_demo")
        .toolkits(vec!["github"])
        .tags(
            Some(vec![TagType::ReadOnlyHint]),  // Only read-only tools
            Some(vec![TagType::DestructiveHint]) // Exclude destructive tools
        )
        .send()
        .await?;

    println!("   ✓ Filtered session created: {}", filtered_session.session_id());
    println!("   ✓ Only read-only, non-destructive tools available");
    println!();

    println!("=== Demo Complete ===");
    println!("\nKey Features Demonstrated:");
    println!("  • Experimental assistive prompts with timezone");
    println!("  • Toolkit connection status checking");
    println!("  • Authentication link generation");
    println!("  • Workbench configuration");
    println!("  • Tag-based tool filtering");

    Ok(())
}
