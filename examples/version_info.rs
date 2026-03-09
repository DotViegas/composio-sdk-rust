//! Example demonstrating SDK version information
//!
//! This example shows how to access SDK version metadata.
//!
//! Run with:
//! ```bash
//! cargo run --example version_info
//! ```

use composio_sdk;

fn main() {
    println!("=== Composio SDK Version Information ===");
    println!();
    println!("SDK Name:    {}", composio_sdk::NAME);
    println!("SDK Version: {}", composio_sdk::VERSION);
    println!();
    println!("Using version() function: {}", composio_sdk::version());
    println!();
    println!("This version includes:");
    println!("  ✓ Session Management");
    println!("  ✓ Tool Execution");
    println!("  ✓ Meta Tools (5 core tools)");
    println!("  ✓ Webhook Events");
    println!("  ✓ Tool Modifiers");
    println!("  ✓ Toolkit Versioning");
    println!("  ✓ Skills Integration");
    println!("  ✓ Wizard Instructions");
}
