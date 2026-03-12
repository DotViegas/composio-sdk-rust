use composio_sdk::models::ToolRetrieveEnumResponse;

#[test]
fn test_tool_retrieve_enum_response_deserialization() {
    let payload = serde_json::json!([
        "GITHUB_CREATE_ISSUE",
        "SLACK_SEND_MESSAGE",
        "GMAIL_SEND_EMAIL"
    ]);

    let response: ToolRetrieveEnumResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.len(), 3);
    assert_eq!(response[0], "GITHUB_CREATE_ISSUE");
}
