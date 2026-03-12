use composio_sdk::models::{
    AuthConfigDeleteResponse, AuthConfigStatus, AuthConfigStatusUpdateResponse,
    AuthConfigUpdateParams, AuthConfigUpdateResponse,
};

#[test]
fn test_auth_config_update_params_serialization() {
    let params = AuthConfigUpdateParams::Default {
        credentials: None,
        is_enabled_for_tool_router: Some(true),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["type"], "default");
    assert_eq!(value["is_enabled_for_tool_router"], true);
}

#[test]
fn test_auth_config_status_enum_serialization() {
    let status = AuthConfigStatus::Disabled;
    let value = serde_json::to_value(&status).unwrap();
    assert_eq!(value, serde_json::json!("DISABLED"));
    assert_eq!(status.as_str(), "DISABLED");
}

#[test]
fn test_auth_config_update_response_deserialization() {
    let payload = serde_json::json!({"success": true});
    let response: AuthConfigUpdateResponse = serde_json::from_value(payload).unwrap();
    assert!(response.success);
}

#[test]
fn test_auth_config_delete_response_deserialization() {
    let payload = serde_json::json!({"success": true, "message": "deleted"});
    let response: AuthConfigDeleteResponse = serde_json::from_value(payload).unwrap();
    assert!(response.success);
    assert_eq!(response.message.as_deref(), Some("deleted"));
}

#[test]
fn test_auth_config_status_update_response_deserialization() {
    let payload = serde_json::json!({"success": true, "status": "ENABLED"});
    let response: AuthConfigStatusUpdateResponse = serde_json::from_value(payload).unwrap();
    assert!(response.success);
    assert_eq!(response.status, "ENABLED");
}
