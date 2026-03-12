//! Migration models for converting legacy UUIDs to NanoIds.

use serde::{Deserialize, Serialize};

/// Migration resource type for UUID-to-NanoId conversion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MigrationResourceType {
    /// Connected account resources.
    ConnectedAccount,
    /// Auth config resources.
    AuthConfig,
    /// Trigger instance resources.
    TriggerInstance,
}

/// Query parameters for retrieving a NanoId from a legacy UUID.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MigrationGetNanoIdParams {
    /// Resource type associated with the UUID.
    #[serde(rename = "type")]
    pub resource_type: MigrationResourceType,
    /// Legacy UUID to convert.
    pub uuid: String,
}

/// Response containing the converted NanoId.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationGetNanoIdResponse {
    /// NanoId corresponding to the provided legacy UUID.
    pub nanoid: String,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let payload = r#"{"nanoid":"ca_abc123xyz"}"#;
        let response: MigrationGetNanoIdResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(response.nanoid, "ca_abc123xyz");
    }
}
