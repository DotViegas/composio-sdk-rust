use composio_sdk::models::{
    MigrationGetNanoIdParams, MigrationGetNanoIdResponse, MigrationResourceType,
};

#[test]
fn test_migration_get_nanoid_params_serialization() {
    let params = MigrationGetNanoIdParams {
        resource_type: MigrationResourceType::ConnectedAccount,
        uuid: "550e8400-e29b-41d4-a716-446655440000".to_string(),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["type"], "CONNECTED_ACCOUNT");
    assert_eq!(value["uuid"], "550e8400-e29b-41d4-a716-446655440000");
}

#[test]
fn test_migration_get_nanoid_response_deserialization() {
    let payload = serde_json::json!({
        "nanoid": "ti_123abc"
    });

    let response: MigrationGetNanoIdResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.nanoid, "ti_123abc");
}
