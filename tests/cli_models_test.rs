use composio_sdk::models::{
    CliCreateSessionResponse, CliGetSessionParams, CliGetSessionResponse, CliSessionStatus,
};
use serde_json::json;

#[test]
fn test_cli_get_session_params_serialization() {
    let params = CliGetSessionParams {
        id: "ABC123".to_string(),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["id"], "ABC123");
}

#[test]
fn test_cli_create_session_response_deserialization_expires_alias() {
    let value = json!({
        "id": "2f3f9f6b-7d13-4d8e-9a2f-b5ebbd8b1832",
        "code": "ABC123",
        "expiresAt": "2026-01-01T00:00:00.000Z",
        "status": "pending"
    });

    let response: CliCreateSessionResponse = serde_json::from_value(value).unwrap();
    assert_eq!(response.status, CliSessionStatus::Pending);
    assert_eq!(response.expires_at, "2026-01-01T00:00:00.000Z");
}

#[test]
fn test_cli_get_session_response_deserialization_with_account_payload() {
    let value = json!({
        "id": "2f3f9f6b-7d13-4d8e-9a2f-b5ebbd8b1832",
        "code": "ABC123",
        "expiresAt": "2026-01-01T00:00:00.000Z",
        "status": "linked",
        "api_key": "cmp_live_xxx",
        "account": {
            "id": "user_123",
            "email": "demo@example.com",
            "name": "Demo User"
        }
    });

    let response: CliGetSessionResponse = serde_json::from_value(value).unwrap();
    assert_eq!(response.status, CliSessionStatus::Linked);
    assert_eq!(response.api_key.as_deref(), Some("cmp_live_xxx"));
    let account = response.account.expect("linked account should be present");
    assert_eq!(account.id, "user_123");
}
