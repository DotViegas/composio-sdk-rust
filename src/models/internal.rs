//! Internal SDK models and resources
//!
//! This module provides internal SDK functionality for realtime credentials
//! and other internal operations.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::client::ComposioClient;
use crate::error::ComposioError;
use crate::models::base::{BaseResource, Resource, TelemetryContext};

/// Endpoint for SDK realtime credentials
const INTERNAL_SDK_REALTIME_CREDENTIALS_ENDPOINT: &str = "/api/v3/internal/sdk/realtime/credentials";

/// Response containing SDK realtime credentials for Pusher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDKRealtimeCredentialsResponse {
    /// Pusher API key
    pub pusher_key: String,
    /// Project ID
    pub project_id: String,
    /// Pusher cluster region
    pub pusher_cluster: String,
}

/// Internal resource for SDK operations
///
/// This resource provides access to internal SDK functionality,
/// including realtime credentials for WebSocket connections.
#[derive(Clone)]
pub struct Internal {
    base: BaseResource,
}

impl Internal {
    /// Create a new Internal resource
    pub fn new(client: Arc<ComposioClient>) -> Self {
        Self {
            base: BaseResource::new(client),
        }
    }

    /// Create a new Internal resource with custom telemetry context
    pub fn with_telemetry_context(
        client: Arc<ComposioClient>,
        telemetry_context: TelemetryContext,
    ) -> Self {
        Self {
            base: BaseResource::with_telemetry_context(client, telemetry_context),
        }
    }

    /// Get SDK realtime credentials
    ///
    /// Retrieves credentials for establishing realtime WebSocket connections
    /// using Pusher. These credentials are used for receiving real-time events
    /// and updates from the Composio platform.
    ///
    /// # Returns
    ///
    /// Returns `SDKRealtimeCredentialsResponse` containing:
    /// - `pusher_key`: The Pusher API key for authentication
    /// - `project_id`: The project identifier
    /// - `pusher_cluster`: The Pusher cluster region (e.g., "us2", "eu")
    ///
    /// # Errors
    ///
    /// Returns `ComposioError` if:
    /// - The API request fails
    /// - Authentication is invalid
    /// - The response cannot be deserialized
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::config::ComposioConfig;
    /// use composio_sdk::models::internal::Internal;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ComposioConfig {
    ///     api_key: "your-api-key".to_string(),
    ///     ..Default::default()
    /// };
    /// let client = Arc::new(ComposioClient::from_config(config)?);
    /// let internal = Internal::new(client);
    ///
    /// let credentials = internal.get_sdk_realtime_credentials().await?;
    /// println!("Pusher Key: {}", credentials.pusher_key);
    /// println!("Project ID: {}", credentials.project_id);
    /// println!("Cluster: {}", credentials.pusher_cluster);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sdk_realtime_credentials(
        &self,
    ) -> Result<SDKRealtimeCredentialsResponse, ComposioError> {
        let start_time = std::time::SystemTime::now();
        let mut telemetry_data = self.create_method_event(
            "Internal.get_sdk_realtime_credentials",
            None,
        );

        let url = format!(
            "{}{}",
            self.client().config().base_url,
            INTERNAL_SDK_REALTIME_CREDENTIALS_ENDPOINT
        );

        // Execute request with retry logic
        let result = crate::retry::with_retry(&self.client().config().retry_policy, || async {
            let response = self
                .client()
                .http_client()
                .get(&url)
                .header("x-api-key", &self.client().config().api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            // Parse response
            let credentials: SDKRealtimeCredentialsResponse = response
                .json()
                .await
                .map_err(ComposioError::NetworkError)?;

            Ok(credentials)
        })
        .await;

        // Update telemetry with duration and error info
        if let Some(ref mut data) = telemetry_data {
            let duration_ms = std::time::SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_millis() as f64;

            data.duration_ms = Some(duration_ms);

            let event_type = if let Err(ref e) = result {
                data.error = Some(crate::models::telemetry::ErrorData {
                    name: "ComposioError".to_string(),
                    code: None,
                    error_id: None,
                    message: Some(e.to_string()),
                    stack: None,
                });
                crate::models::telemetry::EventType::Error
            } else {
                crate::models::telemetry::EventType::Metric
            };

            self.push_telemetry_event((event_type, data.clone()));
        }

        result
    }
}

impl Resource for Internal {
    fn client(&self) -> &ComposioClient {
        self.base.client()
    }

    fn telemetry_context(&self) -> &TelemetryContext {
        self.base.telemetry_context()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ComposioClient;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn test_get_sdk_realtime_credentials_success() {
        let mock_server = MockServer::start().await;

        let response_body = serde_json::json!({
            "pusher_key": "test_pusher_key_123",
            "project_id": "proj_test_456",
            "pusher_cluster": "us2"
        });

        Mock::given(method("GET"))
            .and(path("/api/v3/internal/sdk/realtime/credentials"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = Arc::new(
            ComposioClient::builder()
                .api_key("test_key")
                .base_url(mock_server.uri())
                .build()
                .unwrap(),
        );
        let internal = Internal::new(client);

        let result = internal.get_sdk_realtime_credentials().await;
        assert!(result.is_ok());

        let credentials = result.unwrap();
        assert_eq!(credentials.pusher_key, "test_pusher_key_123");
        assert_eq!(credentials.project_id, "proj_test_456");
        assert_eq!(credentials.pusher_cluster, "us2");
    }

    #[tokio::test]
    async fn test_get_sdk_realtime_credentials_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v3/internal/sdk/realtime/credentials"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(serde_json::json!({
                    "message": "Invalid API key",
                    "code": "UNAUTHORIZED",
                    "status": 401
                })),
            )
            .mount(&mock_server)
            .await;

        let client = Arc::new(
            ComposioClient::builder()
                .api_key("invalid_key")
                .base_url(mock_server.uri())
                .build()
                .unwrap(),
        );
        let internal = Internal::new(client);

        let result = internal.get_sdk_realtime_credentials().await;
        assert!(result.is_err());

        if let Err(ComposioError::ApiError { status, .. }) = result {
            assert_eq!(status, 401);
        } else {
            panic!("Expected ApiError with status 401");
        }
    }

    #[tokio::test]
    async fn test_sdk_realtime_credentials_response_deserialization() {
        let json = r#"{
            "pusher_key": "abc123",
            "project_id": "proj_xyz",
            "pusher_cluster": "eu"
        }"#;

        let response: SDKRealtimeCredentialsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.pusher_key, "abc123");
        assert_eq!(response.project_id, "proj_xyz");
        assert_eq!(response.pusher_cluster, "eu");
    }
}
