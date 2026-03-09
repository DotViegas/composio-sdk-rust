//! Example demonstrating toolkit management functionality
//!
//! This example shows how to:
//! - List available toolkits with filtering
//! - Get detailed information about specific toolkits
//! - List toolkit categories
//! - Authorize users to toolkits
//! - Get authentication field requirements
//!
//! Run with: cargo run --example toolkits_usage

use composio_sdk::client::ComposioClient;
use composio_sdk::models::toolkits::{ToolkitListParams, SortBy, ManagedBy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let api_key = std::env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");

    let client = ComposioClient::builder()
        .api_key(api_key)
        .build()?;

    println!("=== Composio Toolkits Management Example ===\n");

    // Example 1: List all toolkits
    println!("--- Example 1: List All Toolkits ---");
    let params = ToolkitListParams {
        limit: Some(5),
        sort_by: Some(SortBy::Alphabetically),
        ..Default::default()
    };

    let toolkits = client.list_toolkits(params).await?;
    println!("Found {} toolkits (showing first 5):", toolkits.items.len());
    for toolkit in &toolkits.items {
        println!("  - {} ({})", toolkit.name, toolkit.slug);
        println!("    Auth schemes: {:?}", toolkit.auth_schemes);
        if let Some(meta) = &toolkit.meta {
            if let Some(tools_count) = meta.tools_count {
                println!("    Tools: {}", tools_count);
            }
            if let Some(triggers_count) = meta.triggers_count {
                println!("    Triggers: {}", triggers_count);
            }
        }
    }
    println!();

    // Example 2: Filter toolkits by category
    println!("--- Example 2: Filter by Category ---");
    let params = ToolkitListParams {
        category: Some("communication".to_string()),
        limit: Some(3),
        ..Default::default()
    };

    let communication_toolkits = client.list_toolkits(params).await?;
    println!("Communication toolkits:");
    for toolkit in &communication_toolkits.items {
        println!("  - {} ({})", toolkit.name, toolkit.slug);
    }
    println!();

    // Example 3: Filter by managed type
    println!("--- Example 3: Filter by Managed Type ---");
    let params = ToolkitListParams {
        managed_by: Some(ManagedBy::Composio),
        limit: Some(3),
        ..Default::default()
    };

    let managed_toolkits = client.list_toolkits(params).await?;
    println!("Composio-managed toolkits:");
    for toolkit in &managed_toolkits.items {
        println!("  - {} ({})", toolkit.name, toolkit.slug);
        println!("    Managed auth schemes: {:?}", toolkit.composio_managed_auth_schemes);
    }
    println!();

    // Example 4: Get specific toolkit details
    println!("--- Example 4: Get Toolkit Details ---");
    let github = client.get_toolkit("github").await?;
    println!("GitHub Toolkit:");
    println!("  Name: {}", github.name);
    println!("  Slug: {}", github.slug);
    println!("  Auth schemes: {:?}", github.auth_schemes);
    println!("  No auth required: {}", github.no_auth);
    
    if let Some(meta) = &github.meta {
        println!("  Description: {}", meta.description.as_ref().unwrap_or(&"N/A".to_string()));
        println!("  Categories: {:?}", meta.categories);
        println!("  Tools count: {}", meta.tools_count.unwrap_or(0));
        println!("  Triggers count: {}", meta.triggers_count.unwrap_or(0));
    }

    if let Some(auth_details) = &github.auth_config_details {
        println!("  Auth config details:");
        for detail in auth_details {
            println!("    - Mode: {}", detail.mode);
            println!("      Default: {}", detail.is_default);
            println!("      Required fields for connection: {}", 
                detail.fields.connected_account_initiation.required.len());
            println!("      Optional fields for connection: {}", 
                detail.fields.connected_account_initiation.optional.len());
        }
    }
    println!();

    // Example 5: List toolkit categories
    println!("--- Example 5: List Categories ---");
    let categories = client.list_toolkit_categories().await?;
    println!("Available categories ({}):", categories.items.len());
    for category in categories.items.iter().take(10) {
        println!("  - {}", category.name);
        if let Some(display_name) = &category.display_name {
            println!("    Display: {}", display_name);
        }
        if let Some(count) = category.count {
            println!("    Toolkits: {}", count);
        }
    }
    println!();

    // Example 6: Get connected account initiation fields
    println!("--- Example 6: Get Connection Fields ---");
    let fields = client.get_connected_account_initiation_fields(
        "github",
        "OAUTH2",
        false // Get both required and optional fields
    ).await?;
    
    println!("GitHub OAuth2 connection fields:");
    for field in &fields {
        println!("  - {} ({})", 
            field.name, 
            if field.required { "required" } else { "optional" }
        );
        if let Some(desc) = &field.description {
            println!("    Description: {}", desc);
        }
        if let Some(field_type) = &field.field_type {
            println!("    Type: {}", field_type);
        }
    }
    println!();

    // Example 7: Get auth config creation fields
    println!("--- Example 7: Get Auth Config Fields ---");
    let auth_fields = client.get_auth_config_creation_fields(
        "github",
        "OAUTH2",
        true // Get only required fields
    ).await?;
    
    println!("GitHub OAuth2 auth config required fields:");
    for field in &auth_fields {
        println!("  - {}", field.name);
        if let Some(desc) = &field.description {
            println!("    {}", desc);
        }
    }
    println!();

    // Example 8: Authorize a user to a toolkit
    println!("--- Example 8: Authorize User ---");
    println!("Note: This creates a connection request for a user");
    
    // Uncomment to actually create a connection:
    // let connection = client.authorize_toolkit("user_123", "github").await?;
    // if let Some(redirect_url) = connection.redirect_url {
    //     println!("Visit this URL to authenticate: {}", redirect_url);
    //     println!("Connection ID: {}", connection.id);
    //     println!("Status: {:?}", connection.status);
    // }
    
    println!("(Skipped - uncomment code to test)");
    println!();

    // Example 9: Search toolkits
    println!("--- Example 9: Search Toolkits ---");
    let params = ToolkitListParams {
        search: Some("git".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    let search_results = client.list_toolkits(params).await?;
    println!("Toolkits matching 'git':");
    for toolkit in &search_results.items {
        println!("  - {} ({})", toolkit.name, toolkit.slug);
    }
    println!();

    println!("=== Example Complete ===");
    println!("\nKey Takeaways:");
    println!("1. Toolkits are collections of tools for specific services");
    println!("2. You can filter by category, managed type, and search");
    println!("3. Each toolkit has detailed auth configuration information");
    println!("4. authorize_toolkit() simplifies the connection flow");
    println!("5. Field requirements help build custom auth UIs");

    Ok(())
}
