//! Project configuration models for Composio API.

use serde::{Deserialize, Serialize};

/// Log visibility policy for project logs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectLogVisibilitySetting {
    /// Store and show all logs.
    ShowAll,
    /// Do not store sensitive data in logs.
    DontStoreData,
}

/// Project configuration response model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfigResponse {
    /// Whether 2FA is enabled for the project.
    #[serde(rename = "is_2FA_enabled", alias = "is_2_fa_enabled")]
    pub is_2_fa_enabled: bool,

    /// Log visibility setting.
    pub log_visibility_setting: ProjectLogVisibilitySetting,

    /// Whether secret keys are masked in connected account outputs.
    pub mask_secret_keys_in_connected_account: bool,

    /// Optional display name for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Deprecated compatibility field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_composio_link_enabled_for_managed_auth: Option<bool>,

    /// Optional project logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    /// Whether MCP API key is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_mcp_api_key: Option<bool>,

    /// Signed URL file expiry in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url_file_expiry_in_seconds: Option<f64>,
}

/// Project configuration update request model.
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct ProjectConfigUpdateParams {
    /// Optional display name for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional 2FA enablement value.
    #[serde(
        rename = "is_2FA_enabled",
        alias = "is_2_fa_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_2_fa_enabled: Option<bool>,

    /// Deprecated compatibility field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_composio_link_enabled_for_managed_auth: Option<bool>,

    /// Optional log visibility setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_visibility_setting: Option<ProjectLogVisibilitySetting>,

    /// Optional project logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    /// Optional secret key masking setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_secret_keys_in_connected_account: Option<bool>,

    /// Optional MCP API key requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_mcp_api_key: Option<bool>,

    /// Optional signed URL file expiry in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url_file_expiry_in_seconds: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_update_serialization() {
        let params = ProjectConfigUpdateParams {
            display_name: Some("Engineering".to_string()),
            is_2_fa_enabled: Some(true),
            is_composio_link_enabled_for_managed_auth: None,
            log_visibility_setting: Some(ProjectLogVisibilitySetting::DontStoreData),
            logo_url: None,
            mask_secret_keys_in_connected_account: Some(true),
            require_mcp_api_key: Some(false),
            signed_url_file_expiry_in_seconds: Some(900.0),
        };

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["display_name"], "Engineering");
        assert_eq!(json["is_2FA_enabled"], true);
        assert_eq!(json["log_visibility_setting"], "dont_store_data");
        assert!(json.get("logo_url").is_none());
    }

    #[test]
    fn test_project_config_response_deserialization_with_2fa_alias() {
        let payload = r#"{
            "is_2FA_enabled": true,
            "log_visibility_setting": "show_all",
            "mask_secret_keys_in_connected_account": false,
            "display_name": "Acme",
            "require_mcp_api_key": true
        }"#;

        let response: ProjectConfigResponse = serde_json::from_str(payload).unwrap();
        assert!(response.is_2_fa_enabled);
        assert_eq!(
            response.log_visibility_setting,
            ProjectLogVisibilitySetting::ShowAll
        );
        assert_eq!(response.display_name.as_deref(), Some("Acme"));
        assert_eq!(response.require_mcp_api_key, Some(true));
    }

    #[test]
    fn test_project_config_response_deserialization_with_snake_case_alias() {
        let payload = r#"{
            "is_2_fa_enabled": false,
            "log_visibility_setting": "dont_store_data",
            "mask_secret_keys_in_connected_account": true
        }"#;

        let response: ProjectConfigResponse = serde_json::from_str(payload).unwrap();
        assert!(!response.is_2_fa_enabled);
        assert_eq!(
            response.log_visibility_setting,
            ProjectLogVisibilitySetting::DontStoreData
        );
        assert!(response.mask_secret_keys_in_connected_account);
    }
}
