//! Triggers usage examples
//!
//! This example demonstrates how to use the Composio SDK to manage triggers.
//! Triggers are event listeners that notify your application when specific
//! events occur in connected services.
//!
//! Run with: cargo run --example triggers_usage

use composio_sdk::client::ComposioClient;
use composio_sdk::models::triggers::{
    TriggerCreateParams, TriggerInstanceListParams, TriggerTypeListParams, WebhookVerifyParams,
};
use std::collections::HashMap;
use std::env;
use base64::Engine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");

    // Create client
    let client = ComposioClient::builder()
        .api_key(api_key)
        .build()?;

    println!("=== Composio Triggers Examples ===\n");

    // Example 1: List trigger types
    println!("1. Listing trigger types for GitHub...");
    list_trigger_types(&client).await?;

    // Example 2: Get specific trigger type
    println!("\n2. Getting specific trigger type details...");
    get_trigger_type(&client).await?;

    // Example 3: Create a trigger instance
    println!("\n3. Creating a trigger instance...");
    // create_trigger(&client).await?;

    // Example 4: List active triggers
    println!("\n4. Listing active triggers...");
    list_active_triggers(&client).await?;

    // Example 5: Enable/disable triggers
    println!("\n5. Managing trigger state...");
    // manage_trigger_state(&client).await?;

    // Example 6: Verify webhook
    println!("\n6. Verifying webhook signature...");
    verify_webhook_example(&client)?;

    println!("\n=== All examples completed successfully! ===");
    Ok(())
}

/// Example 1: List trigger types
async fn list_trigger_types(client: &ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
    let params = TriggerTypeListParams {
        toolkit_slugs: Some(vec!["github".to_string()]),
        limit: Some(5),
        ..Default::default()
    };

    let response = client.list_trigger_types(params).await?;

    println!("Found {} trigger types:", response.items.len());
    for trigger_type in response.items {
        println!("  - {} ({})", trigger_type.name, trigger_type.slug);
        println!("    Type: {}", trigger_type.trigger_type);
        println!("    Description: {}", trigger_type.description);
    }

    if let Some(cursor) = response.next_cursor {
        println!("  Next cursor: {}", cursor);
    }

    Ok(())
}

/// Example 2: Get specific trigger type
async fn get_trigger_type(client: &ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
    let trigger_slug = "GITHUB_COMMIT_EVENT";

    match client.get_trigger_type(trigger_slug).await {
        Ok(trigger_type) => {
            println!("Trigger: {} ({})", trigger_type.name, trigger_type.slug);
            println!("Type: {}", trigger_type.trigger_type);
            println!("Toolkit: {}", trigger_type.toolkit.slug);
            println!("Description: {}", trigger_type.description);

            if let Some(instructions) = &trigger_type.instructions {
                println!("Instructions: {}", instructions);
            }

            println!("\nConfiguration schema:");
            println!("{}", serde_json::to_string_pretty(&trigger_type.config)?);

            println!("\nPayload schema:");
            println!("{}", serde_json::to_string_pretty(&trigger_type.payload)?);
        }
        Err(e) => {
            println!("Error getting trigger type: {}", e);
        }
    }

    Ok(())
}

/// Example 3: Create a trigger instance
#[allow(dead_code)]
async fn create_trigger(client: &ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
    // Note: This requires a valid connected account
    // Uncomment and modify with your actual values

    let mut trigger_config = HashMap::new();
    trigger_config.insert("repo".to_string(), serde_json::json!("composio"));
    trigger_config.insert("owner".to_string(), serde_json::json!("composio"));

    let params = TriggerCreateParams {
        slug: "GITHUB_COMMIT_EVENT".to_string(),
        user_id: Some("user_123".to_string()), // Will auto-find connected account
        connected_account_id: None,
        trigger_config: Some(trigger_config),
        toolkit_versions: None,
    };

    match client.create_trigger(params).await {
        Ok(response) => {
            println!("Trigger created successfully!");
            println!("  ID: {}", response.id);
            println!("  Trigger name: {}", response.trigger_name);
            println!("  Connected account: {}", response.connected_account_id);
            if let Some(user_id) = response.user_id {
                println!("  User ID: {}", user_id);
            }
        }
        Err(e) => {
            println!("Error creating trigger: {}", e);
            println!("Note: This requires a valid connected account for the user");
        }
    }

    Ok(())
}

/// Example 4: List active triggers
async fn list_active_triggers(client: &ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
    let params = TriggerInstanceListParams {
        trigger_names: Some(vec!["GITHUB_COMMIT_EVENT".to_string()]),
        show_disabled: Some(false),
        limit: Some(10),
        ..Default::default()
    };

    match client.list_active_triggers(params).await {
        Ok(response) => {
            println!("Found {} active triggers:", response.items.len());
            for trigger in response.items {
                println!("  - ID: {}", trigger.id);
                println!("    Trigger: {}", trigger.trigger_name);
                println!("    Connected account: {}", trigger.connected_account_id);
                if let Some(user_id) = &trigger.user_id {
                    println!("    User ID: {}", user_id);
                }
                if let Some(state) = &trigger.state {
                    println!("    State: {}", state);
                }
                if let Some(created_at) = &trigger.created_at {
                    println!("    Created: {}", created_at);
                }
                println!();
            }

            if let Some(cursor) = response.next_cursor {
                println!("  Next cursor: {}", cursor);
            }
        }
        Err(e) => {
            println!("Error listing triggers: {}", e);
        }
    }

    Ok(())
}

/// Example 5: Manage trigger state (enable/disable)
#[allow(dead_code)]
async fn manage_trigger_state(client: &ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
    // Note: Replace with actual trigger ID
    let trigger_id = "ti_abc123";

    // Disable trigger
    println!("Disabling trigger {}...", trigger_id);
    match client.disable_trigger(trigger_id).await {
        Ok(_) => println!("  Trigger disabled successfully"),
        Err(e) => println!("  Error disabling trigger: {}", e),
    }

    // Wait a bit
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Enable trigger
    println!("Enabling trigger {}...", trigger_id);
    match client.enable_trigger(trigger_id).await {
        Ok(_) => println!("  Trigger enabled successfully"),
        Err(e) => println!("  Error enabling trigger: {}", e),
    }

    // Delete trigger (permanent)
    // println!("Deleting trigger {}...", trigger_id);
    // match client.delete_trigger(trigger_id).await {
    //     Ok(_) => println!("  Trigger deleted successfully"),
    //     Err(e) => println!("  Error deleting trigger: {}", e),
    // }

    Ok(())
}

/// Example 6: Verify webhook signature
fn verify_webhook_example(client: &ComposioClient) -> Result<(), Box<dyn std::error::Error>> {
    // Example webhook data (this would come from your webhook handler)
    let webhook_id = "msg_2mXqH9P8KQJKLdJKLdJKLd";
    let webhook_timestamp = "1234567890";
    let webhook_secret = "whsec_test_secret_key";

    // Example V3 webhook payload
    let payload = r#"{
        "id": "evt_123",
        "type": "composio.trigger.message",
        "timestamp": "2024-01-01T00:00:00Z",
        "metadata": {
            "trigger_id": "ti_abc123",
            "trigger_slug": "GITHUB_COMMIT_EVENT",
            "user_id": "user_456",
            "connected_account_id": "ca_789",
            "auth_config_id": "ac_101",
            "log_id": "log_202"
        },
        "data": {
            "author": "john_doe",
            "message": "Fix bug in authentication",
            "sha": "abc123def456"
        }
    }"#;

    // Compute signature (in real scenario, this comes from webhook headers)
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let to_sign = format!("{}.{}.{}", webhook_id, webhook_timestamp, payload);
    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())?;
    mac.update(to_sign.as_bytes());
    let signature_bytes = mac.finalize().into_bytes();
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(&signature_bytes);
    let webhook_signature = format!("v1,{}", signature_b64);

    println!("Webhook verification example:");
    println!("  ID: {}", webhook_id);
    println!("  Timestamp: {}", webhook_timestamp);
    println!("  Signature: {}...", &webhook_signature[..30]);

    let params = WebhookVerifyParams {
        id: webhook_id.to_string(),
        payload: payload.to_string(),
        signature: webhook_signature,
        timestamp: webhook_timestamp.to_string(),
        secret: webhook_secret.to_string(),
        tolerance: Some(999999), // Large tolerance for example
    };

    match client.verify_webhook(params) {
        Ok(result) => {
            println!("\n✓ Webhook verified successfully!");
            println!("  Version: {:?}", result.version);
            println!("  Trigger slug: {}", result.payload.trigger_slug);
            println!("  User ID: {}", result.payload.user_id);
            println!("  Toolkit: {}", result.payload.toolkit_slug);

            if let Some(payload_data) = &result.payload.payload {
                println!("\n  Event data:");
                println!("{}", serde_json::to_string_pretty(payload_data)?);
            }
        }
        Err(e) => {
            println!("✗ Webhook verification failed: {}", e);
        }
    }

    Ok(())
}
