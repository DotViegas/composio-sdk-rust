//! Tests for Tool Router types and serialization
//!
//! This test suite validates that all Tool Router types can be properly
//! serialized and deserialized, matching the expected API format.

use composio_sdk::models::*;
use serde_json::json;

#[test]
fn test_experimental_config_serialization() {
    let config = ExperimentalConfig {
        assistive_prompt: Some(AssistivePromptConfig {
            user_timezone: Some("America/New_York".to_string()),
        }),
    };

    let json = serde_json::to_value(&config).unwrap();
    assert_eq!(
        json,
        json!({
            "assistive_prompt": {
                "user_timezone": "America/New_York"
            }
        })
    );
}

#[test]
fn test_experimental_config_deserialization() {
    let json = json!({
        "assistive_prompt": {
            "user_timezone": "Europe/London"
        }
    });

    let config: ExperimentalConfig = serde_json::from_value(json).unwrap();
    assert!(config.assistive_prompt.is_some());
    assert_eq!(
        config.assistive_prompt.unwrap().user_timezone,
        Some("Europe/London".to_string())
    );
}

#[test]
fn test_session_config_with_experimental() {
    let config = SessionConfig {
        user_id: "user_123".to_string(),
        toolkits: None,
        auth_configs: None,
        connected_accounts: None,
        manage_connections: None,
        tools: None,
        tags: None,
        workbench: None,
        experimental: Some(ExperimentalConfig {
            assistive_prompt: Some(AssistivePromptConfig {
                user_timezone: Some("Asia/Tokyo".to_string()),
            }),
        }),
        toolkit_versions: None,
    };

    let json = serde_json::to_value(&config).unwrap();
    assert_eq!(json["user_id"], "user_123");
    assert_eq!(
        json["experimental"]["assistive_prompt"]["user_timezone"],
        "Asia/Tokyo"
    );
}

#[test]
fn test_toolkit_connection_auth_config_deserialization() {
    let json = json!({
        "id": "ac_123",
        "mode": "OAUTH2",
        "is_composio_managed": true
    });

    let auth_config: ToolkitConnectionAuthConfig = serde_json::from_value(json).unwrap();
    assert_eq!(auth_config.id, "ac_123");
    assert_eq!(auth_config.mode, "OAUTH2");
    assert!(auth_config.is_composio_managed);
}

#[test]
fn test_toolkit_connected_account_deserialization() {
    let json = json!({
        "id": "ca_456",
        "status": "ACTIVE"
    });

    let account: ToolkitConnectedAccount = serde_json::from_value(json).unwrap();
    assert_eq!(account.id, "ca_456");
    assert_eq!(account.status, "ACTIVE");
}

#[test]
fn test_toolkit_connection_deserialization() {
    let json = json!({
        "is_active": true,
        "auth_config": {
            "id": "ac_123",
            "mode": "OAUTH2",
            "is_composio_managed": true
        },
        "connected_account": {
            "id": "ca_456",
            "status": "ACTIVE"
        }
    });

    let connection: ToolkitConnection = serde_json::from_value(json).unwrap();
    assert!(connection.is_active);
    assert!(connection.auth_config.is_some());
    assert!(connection.connected_account.is_some());

    let auth_config = connection.auth_config.unwrap();
    assert_eq!(auth_config.id, "ac_123");

    let account = connection.connected_account.unwrap();
    assert_eq!(account.id, "ca_456");
}

#[test]
fn test_toolkit_connection_state_deserialization() {
    let json = json!({
        "slug": "github",
        "name": "GitHub",
        "is_no_auth": false,
        "logo": "https://logo.url",
        "connection": {
            "is_active": true,
            "auth_config": {
                "id": "ac_123",
                "mode": "OAUTH2",
                "is_composio_managed": true
            },
            "connected_account": {
                "id": "ca_456",
                "status": "ACTIVE"
            }
        }
    });

    let state: ToolkitConnectionState = serde_json::from_value(json).unwrap();
    assert_eq!(state.slug, "github");
    assert_eq!(state.name, "GitHub");
    assert!(!state.is_no_auth);
    assert_eq!(state.logo, Some("https://logo.url".to_string()));
    assert!(state.connection.is_some());

    let connection = state.connection.unwrap();
    assert!(connection.is_active);
}

#[test]
fn test_toolkit_connection_state_no_auth() {
    let json = json!({
        "slug": "composio",
        "name": "Composio",
        "is_no_auth": true,
        "connection": null
    });

    let state: ToolkitConnectionState = serde_json::from_value(json).unwrap();
    assert_eq!(state.slug, "composio");
    assert!(state.is_no_auth);
    assert!(state.connection.is_none());
}

#[test]
fn test_toolkit_connections_details_deserialization() {
    let json = json!({
        "items": [
            {
                "slug": "github",
                "name": "GitHub",
                "is_no_auth": false,
                "connection": {
                    "is_active": true
                }
            },
            {
                "slug": "gmail",
                "name": "Gmail",
                "is_no_auth": false,
                "connection": {
                    "is_active": false
                }
            }
        ],
        "total_pages": 1,
        "next_cursor": null
    });

    let details: ToolkitConnectionsDetails = serde_json::from_value(json).unwrap();
    assert_eq!(details.items.len(), 2);
    assert_eq!(details.total_pages, 1);
    assert!(details.next_cursor.is_none());

    assert_eq!(details.items[0].slug, "github");
    assert_eq!(details.items[1].slug, "gmail");
}

#[test]
fn test_tool_router_mcp_server_type_deserialization() {
    let json_http = json!("http");
    let json_sse = json!("sse");

    let http: ToolRouterMcpServerType = serde_json::from_value(json_http).unwrap();
    let sse: ToolRouterMcpServerType = serde_json::from_value(json_sse).unwrap();

    assert_eq!(http, ToolRouterMcpServerType::Http);
    assert_eq!(sse, ToolRouterMcpServerType::Sse);
}

#[test]
fn test_tool_router_mcp_server_config_deserialization() {
    let json = json!({
        "type": "http",
        "url": "https://mcp.composio.dev/session_123",
        "headers": {
            "x-api-key": "test_key"
        }
    });

    let config: ToolRouterMcpServerConfig = serde_json::from_value(json).unwrap();
    assert_eq!(config.server_type, ToolRouterMcpServerType::Http);
    assert_eq!(config.url, "https://mcp.composio.dev/session_123");
    assert!(config.headers.is_some());

    let headers = config.headers.unwrap();
    assert_eq!(headers.get("x-api-key"), Some(&Some("test_key".to_string())));
}

#[test]
fn test_tool_router_session_experimental_deserialization() {
    let json = json!({
        "assistive_prompt": "Use COMPOSIO_SEARCH_TOOLS to discover tools"
    });

    let experimental: ToolRouterSessionExperimental = serde_json::from_value(json).unwrap();
    assert_eq!(
        experimental.assistive_prompt,
        Some("Use COMPOSIO_SEARCH_TOOLS to discover tools".to_string())
    );
}

#[test]
fn test_assistive_prompt_config_none() {
    let config = AssistivePromptConfig {
        user_timezone: None,
    };

    let json = serde_json::to_value(&config).unwrap();
    // Should serialize to empty object when timezone is None
    assert_eq!(json, json!({}));
}

#[test]
fn test_session_config_minimal() {
    let config = SessionConfig {
        user_id: "user_123".to_string(),
        toolkits: None,
        auth_configs: None,
        connected_accounts: None,
        manage_connections: None,
        tools: None,
        tags: None,
        workbench: None,
        experimental: None,
        toolkit_versions: None,
    };

    let json = serde_json::to_value(&config).unwrap();
    // Only user_id should be present
    assert_eq!(json, json!({"user_id": "user_123"}));
}

#[test]
fn test_toolkit_connections_details_with_pagination() {
    let json = json!({
        "items": [],
        "total_pages": 5,
        "next_cursor": "cursor_abc123"
    });

    let details: ToolkitConnectionsDetails = serde_json::from_value(json).unwrap();
    assert_eq!(details.items.len(), 0);
    assert_eq!(details.total_pages, 5);
    assert_eq!(details.next_cursor, Some("cursor_abc123".to_string()));
}

#[test]
fn test_toolkit_connection_partial_data() {
    let json = json!({
        "is_active": false,
        "auth_config": {
            "id": "ac_123",
            "mode": "API_KEY",
            "is_composio_managed": false
        }
    });

    let connection: ToolkitConnection = serde_json::from_value(json).unwrap();
    assert!(!connection.is_active);
    assert!(connection.auth_config.is_some());
    assert!(connection.connected_account.is_none());
}

#[test]
fn test_experimental_config_empty() {
    let config = ExperimentalConfig {
        assistive_prompt: None,
    };

    let json = serde_json::to_value(&config).unwrap();
    assert_eq!(json, json!({}));
}
