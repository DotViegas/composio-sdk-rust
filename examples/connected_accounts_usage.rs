//! Example demonstrating connected accounts usage
//!
//! This example shows how to work with connected accounts:
//! - Creating connection requests
//! - Using different authentication schemes
//! - Managing connection states
//! - Waiting for connections to become active
//!
//! Run with:
//! ```bash
//! cargo run --example connected_accounts_usage
//! ```

use composio_sdk::models::{
    AuthScheme, AuthSchemeHelper, ConnectionRequest, ConnectionStatus, ConnectionState,
    InitiateConnectionParams, LinkConnectionParams, ConnectedAccountListParams,
};

fn main() {
    println!("=== Composio SDK - Connected Accounts Example ===\n");

    // Example 1: Connection Request
    println!("--- Example 1: Connection Request ---");
    let connection_request = ConnectionRequest::new(
        "ca_abc123".to_string(),
        ConnectionStatus::Initiated,
        Some("https://auth.composio.dev/oauth/github".to_string()),
    );

    println!("Connection ID: {}", connection_request.id);
    println!("Status: {:?}", connection_request.status);
    println!("Redirect URL: {:?}", connection_request.redirect_url);
    println!();

    // Example 2: Authentication Schemes
    println!("--- Example 2: Authentication Schemes ---");
    
    // OAuth2 connection state
    let oauth2_config = serde_json::json!({
        "client_id": "your_client_id",
        "client_secret": "your_client_secret",
        "scopes": ["repo", "user"]
    });
    let oauth2_state = AuthSchemeHelper::oauth2(oauth2_config);
    println!("OAuth2 State:");
    println!("  Scheme: {:?}", oauth2_state.auth_scheme);
    println!("  Status: {:?}", oauth2_state.status);
    println!();

    // API Key connection state
    let api_key_config = serde_json::json!({
        "api_key": "your_api_key_here"
    });
    let api_key_state = AuthSchemeHelper::api_key(api_key_config);
    println!("API Key State:");
    println!("  Scheme: {:?}", api_key_state.auth_scheme);
    println!("  Status: {:?}", api_key_state.status);
    println!();

    // Bearer Token connection state
    let bearer_config = serde_json::json!({
        "token": "your_bearer_token"
    });
    let bearer_state = AuthSchemeHelper::bearer_token(bearer_config);
    println!("Bearer Token State:");
    println!("  Scheme: {:?}", bearer_state.auth_scheme);
    println!("  Status: {:?}", bearer_state.status);
    println!();

    // Example 3: Using Global AUTH_SCHEME Helper
    println!("--- Example 3: Global AUTH_SCHEME Helper ---");
    let basic_config = serde_json::json!({
        "username": "user",
        "password": "pass"
    });
    let basic_state = AuthSchemeHelper::basic(basic_config);
    println!("Basic Auth State:");
    println!("  Scheme: {:?}", basic_state.auth_scheme);
    println!("  Status: {:?}", basic_state.status);
    println!();

    // Example 4: Initiate Connection Parameters
    println!("--- Example 4: Initiate Connection Parameters ---");
    let initiate_params = InitiateConnectionParams {
        user_id: "user_123".to_string(),
        auth_config_id: "ac_github_app".to_string(),
        callback_url: Some("https://myapp.com/callback".to_string()),
        allow_multiple: Some(false),
        config: None,
    };

    println!("Initiate Connection:");
    println!("  User ID: {}", initiate_params.user_id);
    println!("  Auth Config: {}", initiate_params.auth_config_id);
    println!("  Callback URL: {:?}", initiate_params.callback_url);
    println!("  Allow Multiple: {:?}", initiate_params.allow_multiple);
    println!();

    // Example 5: Link Connection Parameters
    println!("--- Example 5: Link Connection Parameters ---");
    let link_params = LinkConnectionParams {
        user_id: "user_456".to_string(),
        auth_config_id: "ac_gmail_oauth".to_string(),
        callback_url: Some("https://myapp.com/auth/complete".to_string()),
    };

    println!("Link Connection:");
    println!("  User ID: {}", link_params.user_id);
    println!("  Auth Config: {}", link_params.auth_config_id);
    println!("  Callback URL: {:?}", link_params.callback_url);
    println!();

    // Example 6: List Parameters
    println!("--- Example 6: Connected Account List Parameters ---");
    let list_params = ConnectedAccountListParams {
        user_ids: Some(vec!["user_123".to_string(), "user_456".to_string()]),
        auth_config_ids: Some(vec!["ac_github_app".to_string()]),
        statuses: Some(vec![ConnectionStatus::Active, ConnectionStatus::Initiated]),
        toolkit_slugs: Some(vec!["github".to_string(), "gmail".to_string()]),
        connected_account_ids: None,
        show_disabled: Some(false),
        limit: Some(20),
        cursor: None,
        order_by: Some("created_at".to_string()),
        order_direction: Some("desc".to_string()),
    };

    println!("List Parameters:");
    println!("  User IDs: {:?}", list_params.user_ids);
    println!("  Auth Config IDs: {:?}", list_params.auth_config_ids);
    println!("  Statuses: {:?}", list_params.statuses);
    println!("  Toolkit Slugs: {:?}", list_params.toolkit_slugs);
    println!("  Limit: {:?}", list_params.limit);
    println!();

    // Example 7: Connection Status Flow
    println!("--- Example 7: Connection Status Flow ---");
    let statuses = vec![
        ConnectionStatus::Initializing,
        ConnectionStatus::Initiated,
        ConnectionStatus::Active,
        ConnectionStatus::Expired,
        ConnectionStatus::Failed,
        ConnectionStatus::Inactive,
    ];

    println!("Connection Status Lifecycle:");
    for status in statuses {
        println!("  {:?}", status);
    }
    println!();

    // Example 8: All Authentication Schemes
    println!("--- Example 8: All Authentication Schemes ---");
    let schemes = vec![
        AuthScheme::Oauth1,
        AuthScheme::Oauth2,
        AuthScheme::ComposioLink,
        AuthScheme::ApiKey,
        AuthScheme::Basic,
        AuthScheme::BearerToken,
        AuthScheme::GoogleServiceAccount,
        AuthScheme::NoAuth,
        AuthScheme::CalcomAuth,
        AuthScheme::BillcomAuth,
        AuthScheme::BasicWithJwt,
    ];

    println!("Available Authentication Schemes:");
    for scheme in schemes {
        println!("  {:?}", scheme);
    }
    println!();

    // Example 9: Serialization
    println!("--- Example 9: Serialization ---");
    let connection_state = ConnectionState {
        auth_scheme: AuthScheme::Oauth2,
        status: ConnectionStatus::Active,
        config: serde_json::json!({
            "access_token": "token_abc123",
            "refresh_token": "refresh_xyz789",
            "expires_at": 1234567890
        }),
    };

    let json = serde_json::to_string_pretty(&connection_state).unwrap();
    println!("Connection State JSON:");
    println!("{}", json);
    println!();

    println!("=== Example Complete ===");
    println!("\nKey Takeaways:");
    println!("1. ConnectionRequest manages OAuth flow lifecycle");
    println!("2. AuthSchemeHelper provides convenient state builders");
    println!("3. Multiple authentication schemes supported");
    println!("4. Connection status tracks authentication progress");
    println!("5. List parameters enable flexible filtering");
    println!("6. All types are serializable for API communication");
}
