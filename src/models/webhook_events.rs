//! Webhook event types for typed event handling.
//!
//! This module provides strongly-typed definitions for specific webhook event types,
//! enabling type-safe handling of events like connection expiration.
//!
//! # Overview
//!
//! Webhook events are delivered via HTTP POST to your configured webhook URL.
//! This module provides Rust types that match the webhook payload structure.
//!
//! # Event Types
//!
//! - `ConnectionExpiredEvent`: Emitted when a connected account expires
//! - `TriggerEvent`: Emitted when a trigger fires (defined in triggers.rs)
//!
//! # Example
//!
//! ```rust
//! use composio::models::webhook_events::{WebhookEvent, is_connection_expired_event};
//!
//! fn handle_webhook(payload: serde_json::Value) {
//!     if is_connection_expired_event(&payload) {
//!         // Handle connection expiration
//!         if let Ok(event) = serde_json::from_value::<ConnectionExpiredEvent>(payload) {
//!             println!("Connection {} expired for user {}", 
//!                 event.data.id, event.data.user_id);
//!         }
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Known webhook event types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// Connection expired event
    #[serde(rename = "composio.connected_account.expired")]
    ConnectionExpired,
    
    /// Trigger message event
    #[serde(rename = "composio.trigger.message")]
    TriggerMessage,
}

/// Connection status values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectionStatus {
    /// Connection is being initialized
    Initializing,
    
    /// OAuth flow has been initiated
    Initiated,
    
    /// Connection is active and working
    Active,
    
    /// Connection failed
    Failed,
    
    /// Connection has expired
    Expired,
    
    /// Connection is manually disabled
    Inactive,
}

// =============================================================================
// CONNECTED ACCOUNT DATA (matches GET /api/v3/connected_accounts/{id})
// =============================================================================

/// Toolkit information in connected account response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountToolkit {
    /// Toolkit slug (e.g., "github", "gmail")
    pub slug: String,
}

/// Deprecated auth config fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountAuthConfigDeprecated {
    /// Legacy UUID identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// Auth config details in connected account response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountAuthConfig {
    /// Auth config ID (nano ID format)
    pub id: String,
    
    /// Authentication scheme (OAUTH2, API_KEY, etc.)
    pub auth_scheme: String,
    
    /// Whether this uses Composio's managed authentication
    pub is_composio_managed: bool,
    
    /// Whether this auth config is disabled
    pub is_disabled: bool,
    
    /// Deprecated fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<ConnectedAccountAuthConfigDeprecated>,
}

/// Connection state value - varies by auth scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateVal {
    /// Connection status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    
    // OAuth2 fields
    /// OAuth2 access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    
    /// OAuth2 refresh token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    
    /// Token type (usually "Bearer")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    
    /// Token expiration time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<serde_json::Value>, // Can be int, string, or null
    
    /// OAuth2 scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<serde_json::Value>, // Can be string or array
    
    /// OpenID Connect ID token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    
    /// PKCE code verifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    
    /// OAuth callback URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    
    // OAuth1 fields
    /// OAuth1 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    
    /// OAuth1 token secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token_secret: Option<String>,
    
    // API Key fields
    /// API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    
    /// Generic API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_api_key: Option<String>,
    
    // Bearer Token fields
    /// Bearer token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    
    // Basic auth fields
    /// Username for basic auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    
    /// Password for basic auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Connection state data discriminated by auth scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    /// Authentication scheme identifier
    #[serde(rename = "authScheme")]
    pub auth_scheme: String,
    
    /// Connection state value (varies by auth scheme)
    pub val: ConnectionStateVal,
}

/// Deprecated connected account fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountDeprecated {
    /// Legacy labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    
    /// Legacy UUID identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// Connected account data matching GET /api/v3/connected_accounts/{id} response.
///
/// This is used in webhook payloads for connection lifecycle events.
/// It intentionally does NOT reuse the SDK client's response types because
/// webhook payloads arrive in raw snake_case format, while the SDK client
/// may transform responses to a different shape. This struct validates the
/// raw webhook payload directly without any transformation layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleConnectedAccountDetailedResponse {
    /// Toolkit information
    pub toolkit: ConnectedAccountToolkit,
    
    /// Auth config details
    pub auth_config: ConnectedAccountAuthConfig,
    
    /// Connected account ID (nano ID format)
    pub id: String,
    
    /// User ID this connection belongs to
    pub user_id: String,
    
    /// Connection status
    pub status: String, // ConnectionStatus value as string
    
    /// Creation timestamp (ISO-8601)
    pub created_at: String,
    
    /// Last update timestamp (ISO-8601)
    pub updated_at: String,
    
    /// Connection state with credentials
    pub state: ConnectionState,
    
    /// Additional connection data
    pub data: HashMap<String, serde_json::Value>,
    
    /// Connection parameters
    pub params: HashMap<String, serde_json::Value>,
    
    /// Reason for current status (if applicable)
    pub status_reason: Option<String>,
    
    /// Whether this connection is disabled
    pub is_disabled: bool,
    
    /// Test request endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_request_endpoint: Option<String>,
    
    /// Deprecated fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<ConnectedAccountDeprecated>,
}

// =============================================================================
// CONNECTION EXPIRED WEBHOOK EVENT
// =============================================================================

/// Webhook metadata for connection events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConnectionMetadata {
    /// Project ID
    pub project_id: String,
    
    /// Organization ID
    pub org_id: String,
}

/// Connection expired webhook event payload.
///
/// Emitted when a connected account expires due to authentication refresh failure.
///
/// # Example
///
/// ```rust
/// use composio::models::webhook_events::{ConnectionExpiredEvent, is_connection_expired_event};
///
/// fn handle_webhook(payload: serde_json::Value) {
///     if is_connection_expired_event(&payload) {
///         if let Ok(event) = serde_json::from_value::<ConnectionExpiredEvent>(payload) {
///             println!("Connection {} expired", event.data.id);
///             println!("Toolkit: {}", event.data.toolkit.slug);
///             println!("User: {}", event.data.user_id);
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionExpiredEvent {
    /// Unique message ID (e.g., "msg_847cdfcd-d219-4f18-a6dd-91acd42ca94a")
    pub id: String,
    
    /// ISO-8601 timestamp
    pub timestamp: String,
    
    /// Event type (always "composio.connected_account.expired")
    #[serde(rename = "type")]
    pub event_type: String,
    
    /// Connected account details
    pub data: SingleConnectedAccountDetailedResponse,
    
    /// Event metadata
    pub metadata: WebhookConnectionMetadata,
}

/// Type alias for non-trigger webhook events with specific typed schemas.
///
/// Trigger events (composio.trigger.message) are handled through the
/// TriggerEvent type in triggers.rs via the trigger subscription system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookEvent {
    /// Connection expired event
    ConnectionExpired(ConnectionExpiredEvent),
}

/// Check if a webhook payload is a connection expired event.
///
/// This function performs type narrowing to determine if the payload
/// represents a connection expiration event.
///
/// # Arguments
///
/// * `payload` - The webhook payload to check
///
/// # Returns
///
/// `true` if this is a connection expired event
///
/// # Example
///
/// ```rust
/// use composio::models::webhook_events::is_connection_expired_event;
/// use serde_json::json;
///
/// let payload = json!({
///     "type": "composio.connected_account.expired",
///     "id": "msg_123",
///     "timestamp": "2024-01-01T00:00:00Z",
///     "data": {},
///     "metadata": {}
/// });
///
/// if is_connection_expired_event(&payload) {
///     // Handle connection expired event
/// }
/// ```
pub fn is_connection_expired_event(payload: &serde_json::Value) -> bool {
    payload
        .get("type")
        .and_then(|t| t.as_str())
        .map(|t| t == "composio.connected_account.expired")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_webhook_event_type_serialization() {
        let event_type = WebhookEventType::ConnectionExpired;
        let json = serde_json::to_string(&event_type).unwrap();
        assert_eq!(json, "\"composio.connected_account.expired\"");

        let event_type = WebhookEventType::TriggerMessage;
        let json = serde_json::to_string(&event_type).unwrap();
        assert_eq!(json, "\"composio.trigger.message\"");
    }

    #[test]
    fn test_connection_status_serialization() {
        let status = ConnectionStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"ACTIVE\"");

        let status = ConnectionStatus::Expired;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"EXPIRED\"");
    }

    #[test]
    fn test_is_connection_expired_event() {
        let payload = json!({
            "type": "composio.connected_account.expired",
            "id": "msg_123",
            "timestamp": "2024-01-01T00:00:00Z"
        });

        assert!(is_connection_expired_event(&payload));

        let payload = json!({
            "type": "composio.trigger.message",
            "id": "msg_456"
        });

        assert!(!is_connection_expired_event(&payload));
    }

    #[test]
    fn test_connection_expired_event_deserialization() {
        let json = json!({
            "id": "msg_847cdfcd-d219-4f18-a6dd-91acd42ca94a",
            "timestamp": "2024-01-01T12:00:00Z",
            "type": "composio.connected_account.expired",
            "data": {
                "toolkit": {
                    "slug": "github"
                },
                "auth_config": {
                    "id": "ac_123",
                    "auth_scheme": "OAUTH2",
                    "is_composio_managed": true,
                    "is_disabled": false
                },
                "id": "ca_456",
                "user_id": "user_789",
                "status": "EXPIRED",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T12:00:00Z",
                "state": {
                    "authScheme": "OAUTH2",
                    "val": {
                        "status": "expired"
                    }
                },
                "data": {},
                "params": {},
                "status_reason": "Refresh token expired",
                "is_disabled": false
            },
            "metadata": {
                "project_id": "proj_123",
                "org_id": "org_456"
            }
        });

        let event: ConnectionExpiredEvent = serde_json::from_value(json).unwrap();
        assert_eq!(event.id, "msg_847cdfcd-d219-4f18-a6dd-91acd42ca94a");
        assert_eq!(event.event_type, "composio.connected_account.expired");
        assert_eq!(event.data.toolkit.slug, "github");
        assert_eq!(event.data.user_id, "user_789");
        assert_eq!(event.data.status, "EXPIRED");
        assert_eq!(event.metadata.project_id, "proj_123");
    }

    #[test]
    fn test_connection_state_val_oauth2() {
        let json = json!({
            "status": "active",
            "access_token": "token_123",
            "refresh_token": "refresh_456",
            "token_type": "Bearer",
            "expires_in": 3600,
            "scope": "repo user"
        });

        let state_val: ConnectionStateVal = serde_json::from_value(json).unwrap();
        assert_eq!(state_val.status, Some("active".to_string()));
        assert_eq!(state_val.access_token, Some("token_123".to_string()));
        assert_eq!(state_val.token_type, Some("Bearer".to_string()));
    }

    #[test]
    fn test_connection_state_val_api_key() {
        let json = json!({
            "status": "active",
            "api_key": "sk_test_123"
        });

        let state_val: ConnectionStateVal = serde_json::from_value(json).unwrap();
        assert_eq!(state_val.api_key, Some("sk_test_123".to_string()));
    }
}
