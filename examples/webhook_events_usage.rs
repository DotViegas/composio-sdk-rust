//! Example demonstrating webhook event handling
//!
//! This example shows how to handle different types of webhook events
//! from Composio, including connection expiration events.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example webhook_events_usage
//! ```

use composio_sdk::models::{
    is_connection_expired_event, ConnectionExpiredEvent, WebhookEventType,
};
use serde_json::json;

fn main() {
    println!("=== Composio Webhook Events Example ===\n");

    // Example 1: Connection Expired Event
    println!("1. Handling Connection Expired Event");
    println!("=====================================");

    let connection_expired_payload = json!({
        "id": "msg_847cdfcd-d219-4f18-a6dd-91acd42ca94a",
        "timestamp": "2024-01-15T12:30:00Z",
        "type": "composio.connected_account.expired",
        "data": {
            "toolkit": {
                "slug": "github"
            },
            "auth_config": {
                "id": "ac_github_oauth",
                "auth_scheme": "OAUTH2",
                "is_composio_managed": true,
                "is_disabled": false
            },
            "id": "ca_user123_github",
            "user_id": "user_123",
            "status": "EXPIRED",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-15T12:30:00Z",
            "state": {
                "authScheme": "OAUTH2",
                "val": {
                    "status": "expired",
                    "access_token": "***masked***",
                    "token_type": "Bearer"
                }
            },
            "data": {},
            "params": {},
            "status_reason": "Refresh token expired - user needs to re-authenticate",
            "is_disabled": false
        },
        "metadata": {
            "project_id": "proj_abc123",
            "org_id": "org_xyz789"
        }
    });

    // Check if it's a connection expired event
    if is_connection_expired_event(&connection_expired_payload) {
        println!("✓ Detected connection expired event");

        // Deserialize to strongly-typed struct
        match serde_json::from_value::<ConnectionExpiredEvent>(connection_expired_payload.clone())
        {
            Ok(event) => {
                println!("\nEvent Details:");
                println!("  Message ID: {}", event.id);
                println!("  Timestamp: {}", event.timestamp);
                println!("  Event Type: {}", event.event_type);
                println!("\nConnection Details:");
                println!("  Connected Account ID: {}", event.data.id);
                println!("  User ID: {}", event.data.user_id);
                println!("  Toolkit: {}", event.data.toolkit.slug);
                println!("  Status: {}", event.data.status);
                println!(
                    "  Reason: {}",
                    event.data.status_reason.unwrap_or_default()
                );
                println!("\nAuth Config:");
                println!("  ID: {}", event.data.auth_config.id);
                println!("  Scheme: {}", event.data.auth_config.auth_scheme);
                println!(
                    "  Composio Managed: {}",
                    event.data.auth_config.is_composio_managed
                );
                println!("\nMetadata:");
                println!("  Project ID: {}", event.metadata.project_id);
                println!("  Organization ID: {}", event.metadata.org_id);

                // Example: Handle the expired connection
                println!("\n📋 Action Required:");
                println!("  → Generate new auth link for user: {}", event.data.user_id);
                println!("  → Notify user to re-authenticate with {}", event.data.toolkit.slug);
                println!("  → Use session.authorize('{}') to create Connect Link", event.data.toolkit.slug);
            }
            Err(e) => {
                eprintln!("✗ Failed to deserialize event: {}", e);
            }
        }
    }

    println!("\n");

    // Example 2: Generic Webhook Event Handling
    println!("2. Generic Webhook Event Handler");
    println!("=================================");

    let events = vec![
        (
            "Connection Expired",
            json!({
                "type": "composio.connected_account.expired",
                "id": "msg_001",
                "timestamp": "2024-01-15T10:00:00Z",
                "data": {
                    "toolkit": {"slug": "gmail"},
                    "auth_config": {
                        "id": "ac_gmail",
                        "auth_scheme": "OAUTH2",
                        "is_composio_managed": true,
                        "is_disabled": false
                    },
                    "id": "ca_gmail_001",
                    "user_id": "user_456",
                    "status": "EXPIRED",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-15T10:00:00Z",
                    "state": {
                        "authScheme": "OAUTH2",
                        "val": {"status": "expired"}
                    },
                    "data": {},
                    "params": {},
                    "status_reason": "Token refresh failed",
                    "is_disabled": false
                },
                "metadata": {
                    "project_id": "proj_123",
                    "org_id": "org_456"
                }
            }),
        ),
        (
            "Trigger Message",
            json!({
                "type": "composio.trigger.message",
                "id": "msg_002",
                "timestamp": "2024-01-15T11:00:00Z"
            }),
        ),
    ];

    for (name, payload) in events {
        println!("\nProcessing: {}", name);
        handle_webhook_event(payload);
    }

    println!("\n");

    // Example 3: Event Type Matching
    println!("3. Event Type Matching");
    println!("======================");

    let event_types = vec![
        WebhookEventType::ConnectionExpired,
        WebhookEventType::TriggerMessage,
    ];

    for event_type in event_types {
        let serialized = serde_json::to_string(&event_type).unwrap();
        println!("Event Type: {:?} → {}", event_type, serialized);
    }

    println!("\n=== Example Complete ===");
}

/// Generic webhook event handler
fn handle_webhook_event(payload: serde_json::Value) {
    let event_type = payload
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("unknown");

    match event_type {
        "composio.connected_account.expired" => {
            println!("  → Handling connection expired event");
            if let Ok(event) = serde_json::from_value::<ConnectionExpiredEvent>(payload) {
                println!("     User: {}", event.data.user_id);
                println!("     Toolkit: {}", event.data.toolkit.slug);
                println!("     Action: Send re-authentication notification");
            }
        }
        "composio.trigger.message" => {
            println!("  → Handling trigger message event");
            println!("     Action: Process trigger payload");
        }
        _ => {
            println!("  → Unknown event type: {}", event_type);
        }
    }
}
