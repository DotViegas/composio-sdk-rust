//! Example demonstrating telemetry usage
//!
//! This example shows how to use the telemetry system to track
//! function invocations and errors.
//!
//! Run with:
//! ```bash
//! cargo run --example telemetry_usage
//! ```

use composio_sdk::models::telemetry::{
    create_event, push_event, ErrorData, LanguageType, Metadata, ServiceType, SourceData,
    TelemetryData,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    println!("Composio SDK - Telemetry Usage Example\n");

    // Example 1: Simple metric event
    println!("1. Sending simple metric event...");
    let simple_event = create_event(
        "metric",
        TelemetryData {
            function_name: "execute_tool".to_string(),
            duration_ms: Some(150.5),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ),
            ..Default::default()
        },
    );
    push_event(simple_event);
    println!("   ✓ Simple metric sent\n");

    // Example 2: Metric with source information
    println!("2. Sending metric with source information...");
    let source_event = create_event(
        "metric",
        TelemetryData {
            function_name: "create_session".to_string(),
            duration_ms: Some(75.2),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ),
            source: Some(SourceData {
                host: Some("example-host".to_string()),
                service: Some(ServiceType::Sdk),
                language: Some(LanguageType::Rust),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
                platform: Some(std::env::consts::OS.to_string()),
                environment: None,
            }),
            ..Default::default()
        },
    );
    push_event(source_event);
    println!("   ✓ Metric with source sent\n");

    // Example 3: Metric with metadata
    println!("3. Sending metric with metadata...");
    let metadata_event = create_event(
        "metric",
        TelemetryData {
            function_name: "list_toolkits".to_string(),
            duration_ms: Some(200.0),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ),
            metadata: Some(Metadata {
                project_id: Some("proj_123456".to_string()),
                provider: Some("openai".to_string()),
            }),
            ..Default::default()
        },
    );
    push_event(metadata_event);
    println!("   ✓ Metric with metadata sent\n");

    // Example 4: Error event
    println!("4. Sending error event...");
    let error_event = create_event(
        "error",
        TelemetryData {
            function_name: "execute_tool".to_string(),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ),
            error: Some(ErrorData {
                name: "ToolExecutionError".to_string(),
                code: Some("TOOL_EXEC_FAILED".to_string()),
                error_id: Some("err_abc123".to_string()),
                message: Some("Failed to execute tool: connection timeout".to_string()),
                stack: Some("at execute_tool (tool_executor.rs:42)".to_string()),
            }),
            ..Default::default()
        },
    );
    push_event(error_event);
    println!("   ✓ Error event sent\n");

    // Example 5: Complete event with all fields
    println!("5. Sending complete event with all fields...");
    let mut props = std::collections::HashMap::new();
    props.insert(
        "toolkit".to_string(),
        serde_json::Value::String("github".to_string()),
    );
    props.insert(
        "tool".to_string(),
        serde_json::Value::String("GITHUB_CREATE_ISSUE".to_string()),
    );

    let complete_event = create_event(
        "metric",
        TelemetryData {
            function_name: "execute_tool_with_retry".to_string(),
            duration_ms: Some(350.8),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ),
            props: Some(props),
            source: Some(SourceData {
                host: Some("production-server".to_string()),
                service: Some(ServiceType::Sdk),
                language: Some(LanguageType::Rust),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
                platform: Some(std::env::consts::OS.to_string()),
                environment: None,
            }),
            metadata: Some(Metadata {
                project_id: Some("proj_prod_123".to_string()),
                provider: Some("anthropic".to_string()),
            }),
            error: None,
        },
    );
    push_event(complete_event);
    println!("   ✓ Complete event sent\n");

    // Give some time for events to be sent
    println!("Waiting for events to be sent...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("\n✓ All telemetry events sent successfully!");
    println!("\nNote: Telemetry is non-blocking and fails silently.");
    println!("Check the Composio telemetry dashboard to verify events were received.");
}
