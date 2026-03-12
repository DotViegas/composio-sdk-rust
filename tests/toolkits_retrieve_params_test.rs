use composio_sdk::models::{ToolkitRetrieveParams, ToolkitRetrieveResponse};

#[test]
fn test_toolkit_retrieve_params_query_serialization() {
    let params = ToolkitRetrieveParams {
        version: Some("20250906_01".to_string()),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["version"], "20250906_01");
}

#[test]
fn test_toolkit_retrieve_response_deserialization() {
    let payload = serde_json::json!({
        "slug": "github",
        "name": "GitHub",
        "auth_schemes": ["OAUTH2"],
        "composio_managed_auth_schemes": ["OAUTH2"],
        "no_auth": false
    });

    let response: ToolkitRetrieveResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.slug, "github");
    assert_eq!(response.name, "GitHub");
    assert_eq!(response.auth_schemes.len(), 1);
}
