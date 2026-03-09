//! Base resource structures for Composio client
//!
//! This module provides base structures and traits for resource management,
//! including telemetry tracking and method tracing.

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;

use crate::client::ComposioClient;
use crate::models::telemetry::{
    push_event, Event, EventType, SourceData, Metadata, TelemetryData, ErrorData,
    ServiceType, LanguageType, EnvironmentType,
};

/// Environment type for telemetry
#[derive(Debug, Clone)]
pub enum Environment {
    Development,
    Production,
    Staging,
}

impl Environment {
    /// Get environment from environment variable or default to Development
    pub fn from_env() -> Self {
        match std::env::var("ENVIRONMENT").as_deref() {
            Ok("production") => Environment::Production,
            Ok("staging") => Environment::Staging,
            _ => Environment::Development,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Staging => "staging",
        }
    }
}

/// Context for tracking telemetry
#[derive(Debug, Clone)]
pub struct TelemetryContext {
    /// Whether tracking is allowed
    pub allow_tracking: bool,
    /// Current environment
    pub environment: Environment,
}

impl Default for TelemetryContext {
    fn default() -> Self {
        Self {
            allow_tracking: true,
            environment: Environment::from_env(),
        }
    }
}

impl TelemetryContext {
    /// Create a new telemetry context
    pub fn new(allow_tracking: bool) -> Self {
        Self {
            allow_tracking,
            environment: Environment::from_env(),
        }
    }

    /// Disable tracking
    pub fn disable_tracking(&mut self) {
        self.allow_tracking = false;
    }

    /// Enable tracking
    pub fn enable_tracking(&mut self) {
        self.allow_tracking = true;
    }
}

/// Base resource trait for Composio resources
/// 
/// This trait provides the foundation for all Composio resource types,
/// including telemetry tracking, method tracing, and payload sanitization.
pub trait Resource {
    /// Get the client reference
    fn client(&self) -> &ComposioClient;

    /// Get telemetry context
    fn telemetry_context(&self) -> &TelemetryContext;

    /// Sanitize payload by removing sensitive information
    /// 
    /// This method filters out sensitive fields from payloads before logging.
    /// Override this method in specific resource implementations to customize
    /// which fields should be sanitized.
    fn sanitize_payload<T>(&self, payload: T) -> T
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        // Default implementation returns payload as-is
        // Specific resources can override to filter sensitive fields
        payload
    }

    /// Get the provider name for telemetry
    /// 
    /// Returns the provider name from the client configuration if available.
    fn provider(&self) -> Option<String> {
        // This can be overridden by specific resources that have provider info
        None
    }

    /// Create a telemetry event for a method call
    /// 
    /// This method creates a telemetry event with metadata about the current
    /// execution context, including timestamp, environment, and provider information.
    fn create_method_event(
        &self,
        function_name: &str,
        provider: Option<&str>,
    ) -> Option<TelemetryData> {
        if !self.telemetry_context().allow_tracking {
            return None;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // Use provider from method parameter or fall back to resource's provider
        let provider_name = provider
            .map(|s| s.to_string())
            .or_else(|| self.provider());

        Some(TelemetryData {
            function_name: function_name.to_string(),
            duration_ms: None,
            timestamp: Some(timestamp),
            props: None,
            source: Some(SourceData {
                host: None,
                service: Some(ServiceType::Sdk),
                language: Some(LanguageType::Rust),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
                platform: None,
                environment: Some(match self.telemetry_context().environment {
                    Environment::Development => EnvironmentType::Development,
                    Environment::Production => EnvironmentType::Production,
                    Environment::Staging => EnvironmentType::Staging,
                }),
            }),
            metadata: Some(Metadata {
                project_id: None,
                provider: provider_name,
            }),
            error: None,
        })
    }

    /// Push a telemetry event
    fn push_telemetry_event(&self, event: Event) {
        if self.telemetry_context().allow_tracking {
            push_event(event);
        }
    }

    /// Trace a method execution with telemetry
    fn trace_method<F, R>(&self, function_name: &str, provider: Option<&str>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let mut telemetry_data = self.create_method_event(function_name, provider);
        let start_time = SystemTime::now();

        let result = f();

        if let Some(ref mut data) = telemetry_data {
            let duration_ms = SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_millis() as f64;

            // Update telemetry data with duration
            data.duration_ms = Some(duration_ms);

            self.push_telemetry_event((EventType::Metric, data.clone()));
        }

        result
    }

    /// Trace a method execution with error handling
    fn trace_method_with_error<F, R, E>(
        &self,
        function_name: &str,
        provider: Option<&str>,
        f: F,
    ) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
        E: std::fmt::Display,
    {
        let mut telemetry_data = self.create_method_event(function_name, provider);
        let start_time = SystemTime::now();

        let result = f();

        if let Some(ref mut data) = telemetry_data {
            let duration_ms = SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_millis() as f64;

            // Update telemetry data with duration
            data.duration_ms = Some(duration_ms);

            // Add error information if result is an error
            let event_type = if let Err(ref e) = result {
                data.error = Some(ErrorData {
                    name: std::any::type_name::<E>().to_string(),
                    code: None,
                    error_id: None,
                    message: Some(e.to_string()),
                    stack: None,
                });
                EventType::Error
            } else {
                EventType::Metric
            };

            self.push_telemetry_event((event_type, data.clone()));
        }

        result
    }
}

/// Base resource structure that implements the Resource trait
#[derive(Clone)]
pub struct BaseResource {
    /// Reference to the Composio client
    pub client: Arc<ComposioClient>,
    /// Telemetry context
    pub telemetry_context: TelemetryContext,
}

impl BaseResource {
    /// Create a new base resource
    pub fn new(client: Arc<ComposioClient>) -> Self {
        Self {
            client,
            telemetry_context: TelemetryContext::default(),
        }
    }

    /// Create a new base resource with custom telemetry context
    pub fn with_telemetry_context(
        client: Arc<ComposioClient>,
        telemetry_context: TelemetryContext,
    ) -> Self {
        Self {
            client,
            telemetry_context,
        }
    }
}

impl Resource for BaseResource {
    fn client(&self) -> &ComposioClient {
        &self.client
    }

    fn telemetry_context(&self) -> &TelemetryContext {
        &self.telemetry_context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_env() {
        // Test default (development)
        let env = Environment::from_env();
        assert_eq!(env.as_str(), "development");
    }

    #[test]
    fn test_environment_as_str() {
        assert_eq!(Environment::Development.as_str(), "development");
        assert_eq!(Environment::Production.as_str(), "production");
        assert_eq!(Environment::Staging.as_str(), "staging");
    }

    #[test]
    fn test_telemetry_context_default() {
        let ctx = TelemetryContext::default();
        assert!(ctx.allow_tracking);
    }

    #[test]
    fn test_telemetry_context_disable_enable() {
        let mut ctx = TelemetryContext::default();
        assert!(ctx.allow_tracking);

        ctx.disable_tracking();
        assert!(!ctx.allow_tracking);

        ctx.enable_tracking();
        assert!(ctx.allow_tracking);
    }

    #[test]
    fn test_base_resource_creation() {
        let client = Arc::new(
            ComposioClient::builder()
                .api_key("test_key")
                .build()
                .unwrap(),
        );
        let resource = BaseResource::new(client);

        assert!(resource.telemetry_context.allow_tracking);
    }
}

// Example usage of the Resource trait:
// 
// ```rust,no_run
// use composio_sdk::models::base::{Resource, BaseResource, TelemetryContext};
// use composio_sdk::client::ComposioClient;
// use composio_sdk::config::ComposioConfig;
// use std::sync::Arc;
// 
// // Create a custom resource that implements the Resource trait
// struct MyCustomResource {
//     base: BaseResource,
// }
// 
// impl MyCustomResource {
//     pub fn new(client: Arc<ComposioClient>) -> Self {
//         Self {
//             base: BaseResource::new(client),
//         }
//     }
//     
//     // Example method that uses telemetry tracing
//     pub fn do_something(&self) -> Result<String, String> {
//         self.trace_method_with_error("MyCustomResource.do_something", None, || {
//             // Your business logic here
//             Ok("Success!".to_string())
//         })
//     }
// }
// 
// impl Resource for MyCustomResource {
//     fn client(&self) -> &ComposioClient {
//         self.base.client()
//     }
//     
//     fn telemetry_context(&self) -> &TelemetryContext {
//         self.base.telemetry_context()
//     }
// }
// ```
