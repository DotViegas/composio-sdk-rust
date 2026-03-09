//! Example demonstrating SDK internal realtime credentials retrieval
//!
//! This example shows how to retrieve realtime credentials for establishing
//! WebSocket connections using Pusher. These credentials are used for receiving
//! real-time events and updates from the Composio platform.
//!
//! # Usage
//!
//! ```bash
//! export COMPOSIO_API_KEY="your-api-key"
//! cargo run --example internal_realtime
//! ```

use composio_sdk::client::ComposioClient;
use composio_sdk::config::ComposioConfig;
use composio_sdk::models::internal::Internal;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key = std::env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");

    println!("=== Composio SDK Internal Realtime Credentials Example ===\n");

    // Create client configuration
    let config = ComposioConfig {
        api_key,
        ..Default::default()
    };

    // Create client
    let client = Arc::new(ComposioClient::from_config(config)?);
    println!("✓ Client initialized\n");

    // Create Internal resource
    let internal = Internal::new(client);

    // Retrieve realtime credentials
    println!("Fetching SDK realtime credentials...");
    match internal.get_sdk_realtime_credentials().await {
        Ok(credentials) => {
            println!("✓ Successfully retrieved realtime credentials\n");
            println!("Credentials Details:");
            println!("  Pusher Key:     {}", credentials.pusher_key);
            println!("  Project ID:     {}", credentials.project_id);
            println!("  Pusher Cluster: {}", credentials.pusher_cluster);
            println!();
            println!("These credentials can be used to establish WebSocket connections");
            println!("for receiving real-time events from the Composio platform.");
        }
        Err(e) => {
            eprintln!("✗ Failed to retrieve realtime credentials: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example completed successfully ===");
    Ok(())
}
