//! Telemetry module for tracking SDK usage and errors
//!
//! This module provides functionality to collect and send telemetry data to Composio's
//! telemetry service. It runs in a background thread and batches events to minimize
//! performance impact.
//!
//! # Features
//!
//! - Non-blocking event collection
//! - Background thread for sending telemetry
//! - Automatic cleanup on shutdown
//! - Silent failure (telemetry errors don't affect application)
//!
//! # Examples
//!
//! ```rust
//! use composio_sdk::models::telemetry::{push_event, create_event, TelemetryData};
//!
//! // Create and push a metric event
//! let event = create_event(
//!     "metric",
//!     TelemetryData {
//!         function_name: "execute_tool".to_string(),
//!         duration_ms: Some(150.5),
//!         timestamp: Some(std::time::SystemTime::now()
//!             .duration_since(std::time::UNIX_EPOCH)
//!             .unwrap()
//!             .as_secs_f64()),
//!         ..Default::default()
//!     }
//! );
//! push_event(event);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use tokio::sync::mpsc;

/// Metrics endpoint
const METRIC_ENDPOINT: &str = "https://telemetry.composio.dev/v1/metrics/invocations";

/// Errors endpoint
const ERROR_ENDPOINT: &str = "https://telemetry.composio.dev/v1/errors";

/// Error data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorData {
    /// The name of the error
    pub name: String,

    /// The code of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The error ID of the error
    #[serde(rename = "errorId", skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,

    /// The message of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The stack trace of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

/// Source/host information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceData {
    /// The name of the source/host
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// The service of the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceType>,

    /// The language of the function that was invoked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageType>,

    /// The version of the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The platform of the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// The environment of the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentType>,
}

/// Service type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    Sdk,
    Apollo,
    Hermes,
    Thermos,
}

/// Language type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguageType {
    Python,
    Typescript,
    Go,
    Rust,
}

/// Environment type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnvironmentType {
    Development,
    Production,
    Ci,
    Staging,
    Test,
}

/// Runtime metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// The project ID of the source
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// The provider used in the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

/// Telemetry data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryData {
    /// The name of the function that was invoked
    #[serde(rename = "functionName")]
    pub function_name: String,

    /// The duration of the function invocation in milliseconds
    #[serde(rename = "durationMs", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,

    /// The timestamp of the function invocation in epoch seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,

    /// The properties of the function invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<HashMap<String, serde_json::Value>>,

    /// Source of the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceData>,

    /// Runtime metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorData>,
}

/// Event type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Metric,
    Error,
}

/// Event tuple containing type and data
pub type Event = (EventType, TelemetryData);

/// Global telemetry state
struct TelemetryState {
    sender: mpsc::UnboundedSender<Event>,
}

static TELEMETRY: OnceLock<Arc<Mutex<Option<TelemetryState>>>> = OnceLock::new();

/// Initialize the telemetry system
fn setup() -> Arc<Mutex<Option<TelemetryState>>> {
    TELEMETRY
        .get_or_init(|| {
            let (tx, mut rx) = mpsc::unbounded_channel::<Event>();

            // Spawn background task to process events
            tokio::spawn(async move {
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(2))
                    .build()
                    .unwrap_or_default();

                while let Some(event) = rx.recv().await {
                    push_to_server(&client, event).await;
                }
            });

            Arc::new(Mutex::new(Some(TelemetryState { sender: tx })))
        })
        .clone()
}

/// Push event to telemetry server
async fn push_to_server(client: &reqwest::Client, event: Event) {
    let (event_type, data) = event;

    let result = match event_type {
        EventType::Metric => {
            client
                .post(METRIC_ENDPOINT)
                .json(&vec![data])
                .send()
                .await
        }
        EventType::Error => {
            client
                .post(ERROR_ENDPOINT)
                .json(&data)
                .send()
                .await
        }
    };

    // Silently fail - telemetry shouldn't break the application
    if let Err(_e) = result {
        #[cfg(feature = "local-debug")]
        eprintln!("Telemetry error: {:?}", _e);
    }
}

/// Push an event to the telemetry queue
///
/// This function is non-blocking and will not fail if telemetry is unavailable.
///
/// # Examples
///
/// ```rust
/// use composio_sdk::models::telemetry::{push_event, create_event, TelemetryData};
///
/// let event = create_event("metric", TelemetryData {
///     function_name: "my_function".to_string(),
///     ..Default::default()
/// });
/// push_event(event);
/// ```
pub fn push_event(event: Event) {
    let state = setup();
    let guard = state.lock();
    if let Ok(guard) = guard {
        if let Some(telemetry) = guard.as_ref() {
            // Ignore send errors - telemetry is best-effort
            let _ = telemetry.sender.send(event);
        }
    }
}

/// Create a telemetry event
///
/// # Examples
///
/// ```rust
/// use composio_sdk::models::telemetry::{create_event, TelemetryData, EventType};
///
/// let event = create_event("metric", TelemetryData {
///     function_name: "execute_tool".to_string(),
///     duration_ms: Some(100.0),
///     ..Default::default()
/// });
/// ```
pub fn create_event(event_type: &str, data: TelemetryData) -> Event {
    let typ = match event_type {
        "error" => EventType::Error,
        _ => EventType::Metric,
    };
    (typ, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_metric_event() {
        let data = TelemetryData {
            function_name: "test_function".to_string(),
            duration_ms: Some(123.45),
            ..Default::default()
        };

        let event = create_event("metric", data.clone());
        assert_eq!(event.0, EventType::Metric);
        assert_eq!(event.1.function_name, "test_function");
        assert_eq!(event.1.duration_ms, Some(123.45));
    }

    #[test]
    fn test_create_error_event() {
        let data = TelemetryData {
            function_name: "test_function".to_string(),
            error: Some(ErrorData {
                name: "TestError".to_string(),
                message: Some("Test error message".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let event = create_event("error", data.clone());
        assert_eq!(event.0, EventType::Error);
        assert!(event.1.error.is_some());
    }

    #[test]
    fn test_push_event_does_not_panic() {
        let data = TelemetryData {
            function_name: "test".to_string(),
            ..Default::default()
        };
        let event = create_event("metric", data);

        // Should not panic even if telemetry system is not fully initialized
        push_event(event);
    }
}
