//! Complete example demonstrating toolkit versioning integration
//!
//! This example shows how to:
//! 1. Configure toolkit versions at the client level
//! 2. Override versions at the session level
//! 3. Use environment variables for version control
//! 4. Execute tools with automatic version resolution
//!
//! Run with:
//! ```bash
//! cargo run --example toolkit_versioning_integration
//! ```

use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
use composio_sdk::ComposioClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Toolkit Versioning Integration Example\n");

    // Example 1: Use "latest" for all toolkits (default behavior)
    println!("📦 Example 1: Using 'latest' for all toolkits");
    let client1 = ComposioClient::builder()
        .api_key("test_api_key")
        .toolkit_versions(ToolkitVersionParam::Latest)
        .build()?;
    
    println!("✓ Client created with Latest version policy");
    println!("  Config: {:?}\n", client1.config().toolkit_versions);

    // Example 2: Specify different versions for different toolkits
    println!("📦 Example 2: Specific versions per toolkit");
    let mut versions = HashMap::new();
    versions.insert(
        "github".to_string(),
        ToolkitVersion::Specific("20250906_01".to_string()),
    );
    versions.insert("gmail".to_string(), ToolkitVersion::Latest);
    versions.insert(
        "slack".to_string(),
        ToolkitVersion::Specific("20250801_01".to_string()),
    );

    let client2 = ComposioClient::builder()
        .api_key("test_api_key")
        .toolkit_versions(ToolkitVersionParam::Versions(versions.clone()))
        .build()?;

    println!("✓ Client created with specific versions:");
    println!("  - github: 20250906_01");
    println!("  - gmail: latest");
    println!("  - slack: 20250801_01\n");

    // Example 3: Session inherits client configuration
    println!("📦 Example 3: Session inherits from client");
    let session = client2.create_session("user_123");
    println!("✓ Session created for user_123");
    println!("  Inherited toolkit_versions from client\n");

    // Example 4: Override versions at session level
    println!("📦 Example 4: Override versions in session");
    let mut session_versions = HashMap::new();
    session_versions.insert(
        "github".to_string(),
        ToolkitVersion::Specific("20250801_01".to_string()),
    );

    let session_with_override = client2
        .create_session("user_456")
        .toolkit_versions(ToolkitVersionParam::Versions(session_versions));

    println!("✓ Session created with overridden versions:");
    println!("  - github: 20250801_01 (overridden from 20250906_01)\n");

    // Example 5: Environment variable precedence
    println!("📦 Example 5: Environment variable precedence");
    println!("Set environment variables to override config:");
    println!("  export COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01");
    println!("  export COMPOSIO_TOOLKIT_VERSION=latest");
    println!("\nPrecedence order:");
    println!("  1. COMPOSIO_TOOLKIT_VERSION_{{TOOLKIT}} (highest)");
    println!("  2. User-provided configuration");
    println!("  3. COMPOSIO_TOOLKIT_VERSION (global)");
    println!("  4. Default: 'latest' (lowest)\n");

    // Example 6: Demonstrate version resolution
    println!("📦 Example 6: Version resolution in action");
    use composio_sdk::utils::toolkit_version::get_toolkit_version;

    let github_version = get_toolkit_version("github", Some(&ToolkitVersionParam::Versions(versions.clone())));
    println!("✓ Resolved version for 'github': {}", github_version.as_str());

    let gmail_version = get_toolkit_version("gmail", Some(&ToolkitVersionParam::Versions(versions.clone())));
    println!("✓ Resolved version for 'gmail': {}", gmail_version.as_str());

    let unknown_version = get_toolkit_version("unknown_toolkit", Some(&ToolkitVersionParam::Versions(versions)));
    println!("✓ Resolved version for 'unknown_toolkit': {} (default)", unknown_version.as_str());

    println!("\n🎉 All examples completed successfully!");
    println!("\n💡 Key Takeaways:");
    println!("  1. Toolkit versions can be configured at client or session level");
    println!("  2. Sessions inherit client configuration by default");
    println!("  3. Session configuration can override client configuration");
    println!("  4. Environment variables have highest precedence");
    println!("  5. Version resolution is automatic during tool execution");

    Ok(())
}
