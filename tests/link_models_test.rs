use composio_sdk::models::{ConnectedAccountLinkCreateParams, ConnectedAccountLinkCreateResponse};

#[test]
fn test_connected_account_link_create_params_serialization() {
    let params = ConnectedAccountLinkCreateParams {
        auth_config_id: "ac_123".to_string(),
        user_id: "user_123".to_string(),
        callback_url: Some("https://example.com/callback".to_string()),
        connection_data: Some(serde_json::json!({"region": "us"})),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["auth_config_id"], "ac_123");
    assert_eq!(value["user_id"], "user_123");
    assert_eq!(value["callback_url"], "https://example.com/callback");
    assert_eq!(value["connection_data"]["region"], "us");
}

#[test]
fn test_connected_account_link_create_response_deserialization_with_expires_at() {
    let payload = serde_json::json!({
        "connected_account_id": "ca_123",
        "link_token": "lt_123",
        "redirect_url": "https://connect.example.com",
        "expires_at": "2026-01-01T00:00:00Z"
    });

    let response: ConnectedAccountLinkCreateResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.connected_account_id, "ca_123");
    assert_eq!(response.link_token, "lt_123");
    assert_eq!(response.expires_at, "2026-01-01T00:00:00Z");
}
