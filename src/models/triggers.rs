//! Triggers management
//!
//! This module provides functionality to manage triggers in Composio.
//! Triggers are event listeners that notify your application when specific
//! events occur in connected services.
//!
//! # Overview
//!
//! Triggers can be:
//! - Webhook-based (real-time notifications)
//! - Poll-based (periodic checks)
//!
//! # Trigger Types vs Trigger Instances
//!
//! - **Trigger Type**: Template defining what event to listen for (e.g., "GITHUB_COMMIT_EVENT")
//! - **Trigger Instance**: Active listener for a specific user and connected account

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Webhook payload version
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookVersion {
    /// Version 1 (legacy)
    V1,
    /// Version 2 (legacy)
    V2,
    /// Version 3 (current)
    V3,
}

/// Trigger event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerEvent {
    /// Trigger instance ID
    pub id: String,

    /// Trigger instance UUID
    pub uuid: String,

    /// User ID
    pub user_id: String,

    /// Toolkit slug
    pub toolkit_slug: String,

    /// Trigger slug
    pub trigger_slug: String,

    /// Event metadata
    pub metadata: TriggerMetadata,

    /// Event payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,

    /// Original payload from the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_payload: Option<serde_json::Value>,
}

/// Trigger event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerMetadata {
    /// Trigger instance ID
    pub id: String,

    /// Trigger instance UUID
    pub uuid: String,

    /// Toolkit slug
    pub toolkit_slug: String,

    /// Trigger slug
    pub trigger_slug: String,

    /// Trigger data (JSON string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_data: Option<String>,

    /// Trigger configuration
    pub trigger_config: serde_json::Value,

    /// Connected account information
    pub connected_account: TriggerConnectedAccount,
}

/// Connected account information in trigger metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConnectedAccount {
    /// Connected account ID
    pub id: String,

    /// Connected account UUID
    pub uuid: String,

    /// Auth config ID
    pub auth_config_id: String,

    /// Auth config UUID
    pub auth_config_uuid: String,

    /// User ID
    pub user_id: String,

    /// Connection status
    pub status: String,
}

/// Webhook verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyWebhookResult {
    /// Detected webhook version
    pub version: WebhookVersion,

    /// Normalized trigger event
    pub payload: TriggerEvent,

    /// Raw webhook payload
    pub raw_payload: serde_json::Value,
}

/// Trigger type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerType {
    /// Trigger slug
    pub slug: String,

    /// Trigger name
    pub name: String,

    /// Trigger description
    pub description: String,

    /// Trigger instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Trigger type (webhook or poll)
    #[serde(rename = "type")]
    pub trigger_type: String,

    /// Toolkit information
    pub toolkit: TriggerToolkitRef,

    /// Configuration schema
    pub config: serde_json::Value,

    /// Payload schema
    pub payload: serde_json::Value,

    /// Trigger version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Toolkit reference in trigger type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerToolkitRef {
    /// Toolkit slug
    pub slug: String,

    /// Toolkit name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Toolkit logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}

/// Trigger instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerInstance {
    /// Trigger instance ID
    pub id: String,

    /// Trigger instance UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    /// Trigger name/slug
    pub trigger_name: String,

    /// Connected account ID
    pub connected_account_id: String,

    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Trigger configuration
    pub trigger_config: serde_json::Value,

    /// Trigger state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// Disabled timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<String>,
}

/// Parameters for retrieving a trigger type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TriggerTypeRetrieveParams {
    /// Toolkit version specification (for example: "latest" or bracket notation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<String>,
}

/// Trigger type enum response
///
/// Contains all available trigger type slug enumeration values.
pub type TriggerTypeRetrieveEnumResponse = Vec<String>;

/// Parameters for listing trigger types
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TriggerTypeListParams {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Filter by toolkit slugs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slugs: Option<Vec<String>>,

    /// Toolkit versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<String>,
}

/// Response from listing trigger types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerTypeListResponse {
    /// List of trigger types
    pub items: Vec<TriggerType>,

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

/// Parameters for listing active trigger instances
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TriggerInstanceListParams {
    /// Filter by trigger IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_ids: Option<Vec<String>>,

    /// Filter by trigger names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_names: Option<Vec<String>>,

    /// Filter by auth config IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_config_ids: Option<Vec<String>>,

    /// Filter by connected account IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_ids: Option<Vec<String>>,

    /// Show disabled triggers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_disabled: Option<bool>,

    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response from listing active trigger instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerInstanceListResponse {
    /// List of trigger instances
    pub items: Vec<TriggerInstance>,

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

/// Parameters for creating a trigger instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCreateParams {
    /// Trigger slug
    pub slug: String,

    /// Connected account ID (required if user_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,

    /// User ID (will auto-find connected account)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Trigger configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<HashMap<String, serde_json::Value>>,

    /// Toolkit versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_versions: Option<String>,
}

/// Response from creating or updating a trigger instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCreateResponse {
    /// Trigger instance ID
    pub id: String,

    /// Trigger name/slug
    pub trigger_name: String,

    /// Connected account ID
    pub connected_account_id: String,

    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Trigger configuration
    pub trigger_config: serde_json::Value,

    /// Trigger state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Parameters for webhook verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVerifyParams {
    /// Webhook message ID from 'webhook-id' header
    pub id: String,

    /// Raw webhook payload as string
    pub payload: String,

    /// Webhook secret from Composio dashboard
    pub secret: String,

    /// Signature from 'webhook-signature' header
    pub signature: String,

    /// Timestamp from 'webhook-timestamp' header
    pub timestamp: String,

    /// Maximum allowed age in seconds (default: 300)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_version_serialization() {
        let v1 = WebhookVersion::V1;
        let json = serde_json::to_string(&v1).unwrap();
        assert_eq!(json, "\"V1\"");

        let v3 = WebhookVersion::V3;
        let json = serde_json::to_string(&v3).unwrap();
        assert_eq!(json, "\"V3\"");
    }

    #[test]
    fn test_trigger_type_retrieve_params_serialization() {
        let params = TriggerTypeRetrieveParams {
            toolkit_versions: Some("latest".to_string()),
        };

        let value = serde_json::to_value(&params).unwrap();
        assert_eq!(value["toolkit_versions"], "latest");
    }

    #[test]
    fn test_trigger_type_retrieve_enum_response_deserialization() {
        let payload = serde_json::json!(["GITHUB_COMMIT_EVENT", "SLACK_NEW_MESSAGE"]);

        let response: TriggerTypeRetrieveEnumResponse = serde_json::from_value(payload).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], "GITHUB_COMMIT_EVENT");
    }

    #[test]
    fn test_trigger_type_list_params_default() {
        let params = TriggerTypeListParams::default();
        assert!(params.cursor.is_none());
        assert!(params.limit.is_none());
        assert!(params.toolkit_slugs.is_none());
    }

    #[test]
    fn test_trigger_instance_list_params_default() {
        let params = TriggerInstanceListParams::default();
        assert!(params.trigger_ids.is_none());
        assert!(params.trigger_names.is_none());
        assert!(params.show_disabled.is_none());
    }

    #[test]
    fn test_trigger_event_deserialization() {
        let json = r#"{
            "id": "ti_123",
            "uuid": "uuid_123",
            "user_id": "user_456",
            "toolkit_slug": "github",
            "trigger_slug": "GITHUB_COMMIT_EVENT",
            "metadata": {
                "id": "ti_123",
                "uuid": "uuid_123",
                "toolkit_slug": "github",
                "trigger_slug": "GITHUB_COMMIT_EVENT",
                "trigger_data": null,
                "trigger_config": {},
                "connected_account": {
                    "id": "ca_789",
                    "uuid": "ca_uuid_789",
                    "auth_config_id": "ac_101",
                    "auth_config_uuid": "ac_uuid_101",
                    "user_id": "user_456",
                    "status": "ACTIVE"
                }
            },
            "payload": {"test": "data"},
            "original_payload": null
        }"#;

        let event: TriggerEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.id, "ti_123");
        assert_eq!(event.user_id, "user_456");
        assert_eq!(event.trigger_slug, "GITHUB_COMMIT_EVENT");
        assert_eq!(event.metadata.connected_account.id, "ca_789");
    }

    #[test]
    fn test_trigger_create_params() {
        let params = TriggerCreateParams {
            slug: "GITHUB_COMMIT_EVENT".to_string(),
            connected_account_id: Some("ca_123".to_string()),
            user_id: None,
            trigger_config: Some(HashMap::from([
                ("repo".to_string(), serde_json::json!("composio")),
                ("owner".to_string(), serde_json::json!("composio")),
            ])),
            toolkit_versions: None,
        };

        assert_eq!(params.slug, "GITHUB_COMMIT_EVENT");
        assert!(params.connected_account_id.is_some());
        assert!(params.trigger_config.is_some());
    }
}
