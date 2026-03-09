//! Connected accounts management
//!
//! This module provides functionality to manage connected accounts,
//! which represent user connections to external services through Composio.
//!
//! # Overview
//!
//! Connected accounts are used to authenticate with third-party services.
//! They can be created through OAuth flows, API keys, or other authentication methods.
//!
//! # Connection Flow
//!
//! 1. Initiate a connection request
//! 2. User authenticates via redirect URL
//! 3. Wait for connection to become ACTIVE
//! 4. Use the connected account for tool execution

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Default timeout for waiting for connection (60 seconds)
pub const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(60);

/// Connection request representing an in-progress authentication
#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    /// Unique identifier for the connection
    pub id: String,

    /// Current status of the connection
    pub status: ConnectionStatus,

    /// Redirect URL for OAuth flows
    pub redirect_url: Option<String>,
}

/// Connection status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectionStatus {
    /// Connection is being initialized
    Initializing,

    /// OAuth flow has been initiated
    Initiated,

    /// Connection is active and ready to use
    Active,

    /// Connection credentials have expired
    Expired,

    /// Connection failed
    Failed,

    /// Connection has been manually disabled
    Inactive,
}

impl ConnectionRequest {
    /// Create a new connection request
    pub fn new(id: String, status: ConnectionStatus, redirect_url: Option<String>) -> Self {
        Self {
            id,
            status,
            redirect_url,
        }
    }

    /// Wait for the connection to become active
    ///
    /// This method polls the connection status until it becomes ACTIVE or the timeout is reached.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Optional timeout duration (defaults to 60 seconds)
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when connection is active, or `Err` on timeout or failure
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use composio_sdk::models::connected_accounts::ConnectionRequest;
    /// # use std::time::Duration;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut connection_request = ConnectionRequest::new(
    ///     "ca_abc123".to_string(),
    ///     composio_sdk::models::connected_accounts::ConnectionStatus::Initiated,
    ///     Some("https://auth.example.com".to_string()),
    /// );
    ///
    /// // Wait up to 60 seconds for connection
    /// connection_request.wait_for_connection(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_connection(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<(), ConnectionError> {
        let timeout = timeout.unwrap_or(DEFAULT_WAIT_TIMEOUT);
        let deadline = SystemTime::now() + timeout;

        while SystemTime::now() < deadline {
            // In a real implementation, this would poll the API
            // For now, we just check the status
            if self.status == ConnectionStatus::Active {
                return Ok(());
            }

            if self.status == ConnectionStatus::Failed {
                return Err(ConnectionError::Failed(format!(
                    "Connection {} failed",
                    self.id
                )));
            }

            // Sleep for 1 second before next poll
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Err(ConnectionError::Timeout(format!(
            "Timeout while waiting for connection {} to be active",
            self.id
        )))
    }
}

/// Connection error types
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    /// Connection timed out
    #[error("Connection timeout: {0}")]
    Timeout(String),

    /// Connection failed
    #[error("Connection failed: {0}")]
    Failed(String),

    /// Multiple connections found when only one expected
    #[error("Multiple connected accounts found: {0}")]
    MultipleAccounts(String),

    /// API error
    #[error("API error: {0}")]
    ApiError(String),
}

/// Authentication scheme enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthScheme {
    /// OAuth 1.0 authentication
    Oauth1,

    /// OAuth 2.0 authentication
    Oauth2,

    /// Composio Connect Link
    ComposioLink,

    /// API Key authentication
    ApiKey,

    /// Basic authentication
    Basic,

    /// Bearer token authentication
    BearerToken,

    /// Google Service Account
    GoogleServiceAccount,

    /// No authentication required
    NoAuth,

    /// Cal.com authentication
    CalcomAuth,

    /// Bill.com authentication
    BillcomAuth,

    /// Basic authentication with JWT
    BasicWithJwt,
}

/// Connection state for creating connected accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    /// Authentication scheme
    pub auth_scheme: AuthScheme,

    /// Connection status
    pub status: ConnectionStatus,

    /// Additional configuration (varies by auth scheme)
    #[serde(flatten)]
    pub config: serde_json::Value,
}

/// Helper functions for creating connection states with different auth schemes
pub struct AuthSchemeHelper;

impl AuthSchemeHelper {
    /// Create OAuth 1.0 connection state
    pub fn oauth1(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::Oauth1,
            status: ConnectionStatus::Initializing,
            config,
        }
    }

    /// Create OAuth 2.0 connection state
    pub fn oauth2(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::Oauth2,
            status: ConnectionStatus::Initializing,
            config,
        }
    }

    /// Create Composio Link connection state
    pub fn composio_link(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::ComposioLink,
            status: ConnectionStatus::Initializing,
            config,
        }
    }

    /// Create API Key connection state
    pub fn api_key(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::ApiKey,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create Basic auth connection state
    pub fn basic(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::Basic,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create Bearer token connection state
    pub fn bearer_token(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::BearerToken,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create Google Service Account connection state
    pub fn google_service_account(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::GoogleServiceAccount,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create No Auth connection state
    pub fn no_auth(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::NoAuth,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create Cal.com auth connection state
    pub fn calcom_auth(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::CalcomAuth,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create Bill.com auth connection state
    pub fn billcom_auth(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::BillcomAuth,
            status: ConnectionStatus::Active,
            config,
        }
    }

    /// Create Basic with JWT connection state
    pub fn basic_with_jwt(config: serde_json::Value) -> ConnectionState {
        ConnectionState {
            auth_scheme: AuthScheme::BasicWithJwt,
            status: ConnectionStatus::Active,
            config,
        }
    }
}

/// Global auth scheme helper instance
pub static AUTH_SCHEME: AuthSchemeHelper = AuthSchemeHelper;

/// Parameters for initiating a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateConnectionParams {
    /// User ID to create the connection for
    pub user_id: String,

    /// Auth config ID to use
    pub auth_config_id: String,

    /// Optional callback URL for OAuth flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,

    /// Whether to allow multiple connections for the same user and auth config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple: Option<bool>,

    /// Optional connection state configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ConnectionState>,
}

/// Parameters for creating a connection link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConnectionParams {
    /// User ID to create the connection for
    pub user_id: String,

    /// Auth config ID to use
    pub auth_config_id: String,

    /// Optional callback URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

/// Connected account list parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedAccountListParams {
    /// Filter by user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,

    /// Filter by auth config IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_ids: Option<Vec<String>>,

    /// Filter by connection statuses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<ConnectionStatus>>,

    /// Filter by toolkit slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slugs: Option<Vec<String>>,

    /// Filter by connected account IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_ids: Option<Vec<String>>,

    /// Show disabled accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_disabled: Option<bool>,

    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Order by field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Order direction (asc/desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<String>,
}

/// Connected account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountInfo {
    /// Unique identifier
    pub id: String,

    /// User ID
    pub user_id: String,

    /// Auth config ID
    pub auth_config_id: String,

    /// Toolkit slug
    pub toolkit: String,

    /// Connection status
    pub status: ConnectionStatus,

    /// Status reason (if failed or expired)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,

    /// Whether the account is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,

    /// Creation timestamp
    pub created_at: String,

    /// Last update timestamp
    pub updated_at: String,

    /// Connection state data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,

    /// Connection data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Connected account list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountListResponse {
    /// List of connected accounts
    pub items: Vec<ConnectedAccountInfo>,

    /// Next cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    /// Total number of pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u32>,

    /// Current page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<u32>,

    /// Total number of items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<u32>,
}

/// Connected account update status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountUpdateStatusResponse {
    /// Success status
    pub success: bool,

    /// Updated status
    pub status: ConnectionStatus,
}

// Note: The ConnectedAccounts resource implementation is pending full HTTP client integration.
// The data structures above are ready for use once the client supports generic HTTP methods.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_status_serialization() {
        let status = ConnectionStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"ACTIVE\"");
    }

    #[test]
    fn test_connection_status_deserialization() {
        let json = "\"ACTIVE\"";
        let status: ConnectionStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, ConnectionStatus::Active);
    }

    #[test]
    fn test_auth_scheme_serialization() {
        let scheme = AuthScheme::Oauth2;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"OAUTH2\"");
    }

    #[test]
    fn test_auth_scheme_helper_oauth2() {
        let config = serde_json::json!({"client_id": "test"});
        let state = AuthSchemeHelper::oauth2(config);
        assert_eq!(state.auth_scheme, AuthScheme::Oauth2);
        assert_eq!(state.status, ConnectionStatus::Initializing);
    }

    #[test]
    fn test_auth_scheme_helper_api_key() {
        let config = serde_json::json!({"api_key": "test_key"});
        let state = AuthSchemeHelper::api_key(config);
        assert_eq!(state.auth_scheme, AuthScheme::ApiKey);
        assert_eq!(state.status, ConnectionStatus::Active);
    }

    #[test]
    fn test_connection_request_new() {
        let request = ConnectionRequest::new(
            "ca_test123".to_string(),
            ConnectionStatus::Initiated,
            Some("https://auth.example.com".to_string()),
        );
        assert_eq!(request.id, "ca_test123");
        assert_eq!(request.status, ConnectionStatus::Initiated);
        assert!(request.redirect_url.is_some());
    }

    #[test]
    fn test_connected_account_list_params_default() {
        let params = ConnectedAccountListParams::default();
        assert!(params.user_ids.is_none());
        assert!(params.auth_config_ids.is_none());
        assert!(params.statuses.is_none());
    }

    #[test]
    fn test_initiate_connection_params_serialization() {
        let params = InitiateConnectionParams {
            user_id: "user_123".to_string(),
            auth_config_id: "ac_abc".to_string(),
            callback_url: Some("https://callback.example.com".to_string()),
            allow_multiple: Some(false),
            config: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("user_123"));
        assert!(json.contains("ac_abc"));
        assert!(json.contains("callback"));
    }
}
