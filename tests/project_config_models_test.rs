use composio_sdk::models::{
    ProjectConfigResponse, ProjectConfigUpdateParams, ProjectLogVisibilitySetting,
};
use serde_json::json;

#[test]
fn test_project_config_update_params_serialization() {
    let params = ProjectConfigUpdateParams {
        display_name: Some("My Project".to_string()),
        is_2_fa_enabled: Some(true),
        is_composio_link_enabled_for_managed_auth: Some(false),
        log_visibility_setting: Some(ProjectLogVisibilitySetting::DontStoreData),
        logo_url: None,
        mask_secret_keys_in_connected_account: Some(true),
        require_mcp_api_key: Some(true),
        signed_url_file_expiry_in_seconds: Some(3600.0),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["display_name"], "My Project");
    assert_eq!(value["is_2FA_enabled"], true);
    assert_eq!(value["log_visibility_setting"], "dont_store_data");
    assert_eq!(value["mask_secret_keys_in_connected_account"], true);
    assert!(value.get("logo_url").is_none());
}

#[test]
fn test_project_config_response_deserialization() {
    let value = json!({
        "is_2FA_enabled": true,
        "log_visibility_setting": "show_all",
        "mask_secret_keys_in_connected_account": false,
        "display_name": "Acme",
        "is_composio_link_enabled_for_managed_auth": true,
        "logo_url": "https://example.com/logo.png",
        "require_mcp_api_key": false,
        "signed_url_file_expiry_in_seconds": 1200
    });

    let response: ProjectConfigResponse = serde_json::from_value(value).unwrap();
    assert!(response.is_2_fa_enabled);
    assert_eq!(
        response.log_visibility_setting,
        ProjectLogVisibilitySetting::ShowAll
    );
    assert_eq!(response.display_name.as_deref(), Some("Acme"));
    assert_eq!(response.require_mcp_api_key, Some(false));
}

#[test]
fn test_project_config_response_deserialization_accepts_snake_case_alias() {
    let value = json!({
        "is_2_fa_enabled": false,
        "log_visibility_setting": "dont_store_data",
        "mask_secret_keys_in_connected_account": true
    });

    let response: ProjectConfigResponse = serde_json::from_value(value).unwrap();
    assert!(!response.is_2_fa_enabled);
    assert_eq!(
        response.log_visibility_setting,
        ProjectLogVisibilitySetting::DontStoreData
    );
}
