//! Example demonstrating toolkit versioning
//!
//! This example shows how to use toolkit versioning to control which versions
//! of toolkits are used in your application.
//!
//! Run with:
//! ```bash
//! cargo run --example toolkit_versioning
//! ```

use composio_sdk::{ToolkitVersion, ToolkitVersionParam, ToolkitVersions};
use std::collections::HashMap;

fn main() {
    println!("=== Composio SDK: Toolkit Versioning Example ===\n");

    // Example 1: Use latest for all toolkits
    println!("1. Using 'latest' for all toolkits:");
    let config = ToolkitVersionParam::Latest;
    println!("   Config: {:?}", config);
    println!("   Serialized: {}\n", serde_json::to_string(&config).unwrap());

    // Example 2: Use specific versions for different toolkits
    println!("2. Using specific versions:");
    let mut versions: ToolkitVersions = HashMap::new();
    versions.insert(
        "github".to_string(),
        ToolkitVersion::Specific("20250906_01".to_string()),
    );
    versions.insert("gmail".to_string(), ToolkitVersion::Latest);
    versions.insert(
        "slack".to_string(),
        ToolkitVersion::Specific("20250801_01".to_string()),
    );

    let config = ToolkitVersionParam::Versions(versions);
    println!("   Config: {:?}", config);
    println!("   Serialized: {}\n", serde_json::to_string_pretty(&config).unwrap());

    // Example 3: Don't specify versions (use server default)
    println!("3. Not specifying versions (server default):");
    let config = ToolkitVersionParam::None;
    println!("   Config: {:?}", config);
    println!("   Serialized: {}\n", serde_json::to_string(&config).unwrap());

    // Example 4: Using environment variables
    println!("4. Using environment variables:");
    println!("   Set COMPOSIO_TOOLKIT_VERSION_GITHUB=20250906_01");
    std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");

    use composio_sdk::utils::toolkit_version::get_toolkit_version;
    let version = get_toolkit_version("github", None);
    println!("   Resolved version for github: {}", version.as_str());

    std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");

    // Example 5: Precedence order
    println!("\n5. Version resolution precedence:");
    println!("   1. COMPOSIO_TOOLKIT_VERSION_{{TOOLKIT}} (highest)");
    println!("   2. User-provided configuration");
    println!("   3. COMPOSIO_TOOLKIT_VERSION (global)");
    println!("   4. 'latest' (default)");

    // Example 6: Getting versions from environment
    println!("\n6. Extracting versions from environment:");
    std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");
    std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GMAIL", "latest");

    use composio_sdk::utils::toolkit_version::get_versions_from_env;
    let env_versions = get_versions_from_env();
    println!("   Found {} toolkit versions in environment:", env_versions.len());
    for (toolkit, version) in &env_versions {
        println!("     - {}: {}", toolkit, version.as_str());
    }

    std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
    std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GMAIL");

    println!("\n=== Example Complete ===");
}
