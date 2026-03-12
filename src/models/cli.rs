//! CLI session models for Composio API.

use serde::{Deserialize, Serialize};

/// CLI session status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CliSessionStatus {
    /// Session exists but has not been linked to an account.
    Pending,
    /// Session has been linked to an account.
    Linked,
}

/// Linked account payload returned for linked CLI sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliLinkedAccount {
    /// Linked account ID.
    pub id: String,
    /// Linked account email.
    pub email: String,
    /// Linked account display name.
    pub name: String,
}

/// Query parameters for retrieving a CLI session.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CliGetSessionParams {
    /// CLI session UUID or 6-character code.
    pub id: String,
}

/// Response for creating a CLI session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliCreateSessionResponse {
    /// Unique CLI session identifier.
    pub id: String,
    /// 6-character login code.
    pub code: String,
    /// Session expiry timestamp.
    #[serde(rename = "expiresAt", alias = "expires_at")]
    pub expires_at: String,
    /// Session status.
    pub status: CliSessionStatus,
}

/// Response for retrieving a CLI session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliGetSessionResponse {
    /// Unique CLI session identifier.
    pub id: String,
    /// Linked account details, when linked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CliLinkedAccount>,
    /// API key, when linked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// 6-character login code.
    pub code: String,
    /// Session expiry timestamp.
    #[serde(rename = "expiresAt", alias = "expires_at")]
    pub expires_at: String,
    /// Session status.
    pub status: CliSessionStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_get_session_params_serialization() {
        let params = CliGetSessionParams {
            id: "ABC123".to_string(),
        };

        let value = serde_json::to_value(&params).unwrap();
        assert_eq!(value["id"], "ABC123");
    }

    #[test]
    fn test_cli_create_session_response_deserialization_with_expires_alias() {
        let payload = r#"{
            "id": "2f3f9f6b-7d13-4d8e-9a2f-b5ebbd8b1832",
            "code": "ABC123",
            "expiresAt": "2026-01-01T00:00:00.000Z",
            "status": "pending"
        }"#;

        let response: CliCreateSessionResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(response.code, "ABC123");
        assert_eq!(response.status, CliSessionStatus::Pending);
        assert_eq!(response.expires_at, "2026-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_cli_get_session_response_deserialization_with_linked_account() {
        let payload = r#"{
            "id": "2f3f9f6b-7d13-4d8e-9a2f-b5ebbd8b1832",
            "code": "ABC123",
            "expiresAt": "2026-01-01T00:00:00.000Z",
            "status": "linked",
            "api_key": "cmp_test_key",
            "account": {
                "id": "user_123",
                "email": "demo@example.com",
                "name": "Demo User"
            }
        }"#;

        let response: CliGetSessionResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(response.status, CliSessionStatus::Linked);
        assert_eq!(response.api_key.as_deref(), Some("cmp_test_key"));
        assert_eq!(response.account.unwrap().email, "demo@example.com");
    }
}
