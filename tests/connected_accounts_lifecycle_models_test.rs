use composio_sdk::models::{
    ConnectedAccountDeleteResponse, ConnectedAccountRefreshParams, ConnectedAccountRefreshResponse,
    ConnectedAccountUpdateStatusParams, ConnectedAccountUpdateStatusResponse, ConnectionStatus,
};

#[test]
fn test_connected_account_refresh_params_serialization() {
    let params = ConnectedAccountRefreshParams {
        redirect_url: Some("https://example.com/callback".to_string()),
        validate_credentials: Some(true),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["redirect_url"], "https://example.com/callback");
    assert_eq!(value["validate_credentials"], true);
}

#[test]
fn test_connected_account_update_status_params_serialization() {
    let params = ConnectedAccountUpdateStatusParams { enabled: false };
    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["enabled"], false);
}

#[test]
fn test_connected_account_refresh_response_deserialization() {
    let payload = serde_json::json!({
        "id": "ca_123",
        "status": "ACTIVE",
        "redirect_url": null
    });

    let response: ConnectedAccountRefreshResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.id, "ca_123");
    assert_eq!(response.status, ConnectionStatus::Active);
    assert!(response.redirect_url.is_none());
}

#[test]
fn test_connected_account_status_update_response_deserialization() {
    let payload = serde_json::json!({"success": true});
    let response: ConnectedAccountUpdateStatusResponse = serde_json::from_value(payload).unwrap();
    assert!(response.success);
}

#[test]
fn test_connected_account_delete_response_deserialization() {
    let payload = serde_json::json!({"success": true});
    let response: ConnectedAccountDeleteResponse = serde_json::from_value(payload).unwrap();
    assert!(response.success);
}
