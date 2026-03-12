use composio_sdk::models::{TriggerTypeRetrieveEnumResponse, TriggerTypeRetrieveParams};

#[test]
fn test_trigger_type_retrieve_params_query_serialization() {
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
