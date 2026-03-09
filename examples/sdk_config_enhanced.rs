//! Enhanced SDK Configuration Example
//!
//! This example demonstrates the new configuration options added to match
//! the Python SDK functionality:
//! - Environment variable auto-detection for API key
//! - File management configuration
//! - Telemetry opt-in
//!
//! Run with:
//! ```bash
//! # Set API key via environment variable
//! export COMPOSIO_API_KEY=your_api_key
//! cargo run --example sdk_config_enhanced
//! ```

use composio_sdk::ComposioClient;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Enhanced SDK Configuration Example ===\n");

    // Example 1: Auto-detect API key from environment
    println!("1. Auto-detecting API key from COMPOSIO_API_KEY environment variable...");
    match ComposioClient::builder().build() {
        Ok(client) => {
            println!("   ✓ Client created successfully with environment API key");
            println!("   Base URL: {}", client.config().base_url);
        }
        Err(e) => {
            println!("   ✗ Failed: {}", e);
            println!("   Tip: Set COMPOSIO_API_KEY environment variable");
        }
    }

    println!();

    // Example 2: Explicit API key (overrides environment)
    println!("2. Using explicit API key (overrides environment)...");
    let client = ComposioClient::builder()
        .api_key("explicit_api_key")
        .build()?;
    println!("   ✓ Client created with explicit API key");

    println!();

    // Example 3: File management configuration
    println!("3. Configuring file management...");
    let client = ComposioClient::builder()
        .api_key("test_key")
        .file_download_dir(PathBuf::from("./downloads"))
        .auto_upload_download_files(true)
        .build()?;
    println!("   ✓ File download directory: {:?}", client.config().file_download_dir);
    println!("   ✓ Auto upload/download: {}", client.config().auto_upload_download_files);

    println!();

    // Example 4: Telemetry configuration (opt-in)
    println!("4. Configuring telemetry (opt-in for privacy)...");
    let client_with_telemetry = ComposioClient::builder()
        .api_key("test_key")
        .telemetry_enabled(true)
        .build()?;
    println!("   ✓ Telemetry enabled: {}", client_with_telemetry.config().telemetry_enabled);

    let client_without_telemetry = ComposioClient::builder()
        .api_key("test_key")
        .build()?;
    println!("   ✓ Telemetry disabled by default: {}", client_without_telemetry.config().telemetry_enabled);

    println!();

    // Example 5: Complete configuration
    println!("5. Complete configuration with all options...");
    let client = ComposioClient::builder()
        .api_key("test_key")
        .base_url("https://backend.composio.dev/api/v3")
        .timeout(std::time::Duration::from_secs(60))
        .max_retries(5)
        .file_download_dir(PathBuf::from("./downloads"))
        .auto_upload_download_files(true)
        .telemetry_enabled(false)
        .build()?;

    println!("   ✓ API key: {}", client.config().api_key);
    println!("   ✓ Base URL: {}", client.config().base_url);
    println!("   ✓ Timeout: {:?}", client.config().timeout);
    println!("   ✓ Max retries: {}", client.config().retry_policy.max_retries);
    println!("   ✓ File download dir: {:?}", client.config().file_download_dir);
    println!("   ✓ Auto file handling: {}", client.config().auto_upload_download_files);
    println!("   ✓ Telemetry: {}", client.config().telemetry_enabled);

    println!();
    println!("=== Comparison with Python SDK ===");
    println!();
    println!("Python SDK:");
    println!("  composio = Composio(");
    println!("      api_key=os.getenv('COMPOSIO_API_KEY'),  # Auto-detected");
    println!("      file_download_dir='./downloads',");
    println!("      auto_upload_download_files=True,");
    println!("      allow_tracking=False");
    println!("  )");
    println!();
    println!("Rust SDK (equivalent):");
    println!("  let client = ComposioClient::builder()");
    println!("      // .api_key() not needed - auto-detected from COMPOSIO_API_KEY");
    println!("      .file_download_dir(PathBuf::from(\"./downloads\"))");
    println!("      .auto_upload_download_files(true)");
    println!("      .telemetry_enabled(false)  // Privacy-first: opt-in");
    println!("      .build()?;");

    Ok(())
}
