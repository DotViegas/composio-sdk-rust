//! Direct connected-account link endpoint models.

use serde::{Deserialize, Serialize};

/// Request body for creating a direct connected account link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountLinkCreateParams {
    /// Auth config ID used for link generation.
    pub auth_config_id: String,
    /// User ID to create the link for.
    pub user_id: String,
    /// Optional callback URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// Optional connection data to prefill auth fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_data: Option<serde_json::Value>,
}

/// Response for direct connected account link creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccountLinkCreateResponse {
    /// Connected account ID that was created.
    pub connected_account_id: String,
    /// Link token for auth session.
    pub link_token: String,
    /// Redirect URL to complete authentication.
    pub redirect_url: String,
    /// Link expiration timestamp.
    #[serde(alias = "expiresAt")]
    pub expires_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected_account_link_create_params_serialization() {
        let params = ConnectedAccountLinkCreateParams {
            auth_config_id: "ac_123".to_string(),
            user_id: "user_123".to_string(),
            callback_url: Some("https://example.com/callback".to_string()),
            connection_data: Some(serde_json::json!({"region":"us"})),
        };

        let value = serde_json::to_value(&params).unwrap();
        assert_eq!(value["auth_config_id"], "ac_123");
        assert_eq!(value["user_id"], "user_123");
        assert_eq!(value["callback_url"], "https://example.com/callback");
        assert_eq!(value["connection_data"]["region"], "us");
    }

    #[test]
    fn test_connected_account_link_create_response_deserialization() {
        let payload = serde_json::json!({
            "connected_account_id": "ca_123",
            "link_token": "lt_123",
            "redirect_url": "https://connect.example.com",
            "expires_at": "2026-01-01T00:00:00Z"
        });

        let response: ConnectedAccountLinkCreateResponse = serde_json::from_value(payload).unwrap();
        assert_eq!(response.connected_account_id, "ca_123");
        assert_eq!(response.expires_at, "2026-01-01T00:00:00Z");
    }
}
