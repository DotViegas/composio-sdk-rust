//! Enums for Composio API

use serde::{Deserialize, Serialize};

/// Meta tool slugs for the 5 core Composio meta tools
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetaToolSlug {
    /// Search for relevant tools across 1000+ apps
    ComposioSearchTools,
    /// Execute up to 20 tools in parallel
    ComposioMultiExecuteTool,
    /// Handle OAuth and API key authentication
    ComposioManageConnections,
    /// Run Python code in persistent sandbox
    ComposioRemoteWorkbench,
    /// Execute bash commands for file/data processing
    ComposioRemoteBashTool,
}

/// Tag types for tool filtering by behavior hints
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TagType {
    /// Tool only reads data, doesn't modify anything
    ReadOnlyHint,
    /// Tool modifies or deletes data
    DestructiveHint,
    /// Tool can be safely retried (same result)
    IdempotentHint,
    /// Tool requires open world context
    OpenWorldHint,
}

/// Authentication schemes supported by toolkits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthScheme {
    /// OAuth 1.0 authentication
    Oauth1,
    /// OAuth 2.0 authentication
    Oauth2,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    BearerToken,
    /// HTTP Basic authentication
    Basic,
    /// No authentication required
    NoAuth,
    /// Snowflake authentication
    Snowflake,
    /// Cal.com authentication
    CalcomAuth,
    /// Bill.com authentication
    BillcomAuth,
    /// Composio Link authentication
    ComposioLink,
    /// Basic authentication with JWT
    BasicWithJwt,
    /// Google Service Account authentication
    GoogleServiceAccount,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_meta_tool_slug_serialization() {
        let slug = MetaToolSlug::ComposioSearchTools;
        let json = serde_json::to_string(&slug).unwrap();
        assert_eq!(json, "\"COMPOSIO_SEARCH_TOOLS\"");

        let slug = MetaToolSlug::ComposioMultiExecuteTool;
        let json = serde_json::to_string(&slug).unwrap();
        assert_eq!(json, "\"COMPOSIO_MULTI_EXECUTE_TOOL\"");

        let slug = MetaToolSlug::ComposioManageConnections;
        let json = serde_json::to_string(&slug).unwrap();
        assert_eq!(json, "\"COMPOSIO_MANAGE_CONNECTIONS\"");

        let slug = MetaToolSlug::ComposioRemoteWorkbench;
        let json = serde_json::to_string(&slug).unwrap();
        assert_eq!(json, "\"COMPOSIO_REMOTE_WORKBENCH\"");

        let slug = MetaToolSlug::ComposioRemoteBashTool;
        let json = serde_json::to_string(&slug).unwrap();
        assert_eq!(json, "\"COMPOSIO_REMOTE_BASH_TOOL\"");
    }

    #[test]
    fn test_meta_tool_slug_deserialization() {
        let json = "\"COMPOSIO_SEARCH_TOOLS\"";
        let slug: MetaToolSlug = serde_json::from_str(json).unwrap();
        assert!(matches!(slug, MetaToolSlug::ComposioSearchTools));

        let json = "\"COMPOSIO_MULTI_EXECUTE_TOOL\"";
        let slug: MetaToolSlug = serde_json::from_str(json).unwrap();
        assert!(matches!(slug, MetaToolSlug::ComposioMultiExecuteTool));

        let json = "\"COMPOSIO_MANAGE_CONNECTIONS\"";
        let slug: MetaToolSlug = serde_json::from_str(json).unwrap();
        assert!(matches!(slug, MetaToolSlug::ComposioManageConnections));

        let json = "\"COMPOSIO_REMOTE_WORKBENCH\"";
        let slug: MetaToolSlug = serde_json::from_str(json).unwrap();
        assert!(matches!(slug, MetaToolSlug::ComposioRemoteWorkbench));

        let json = "\"COMPOSIO_REMOTE_BASH_TOOL\"";
        let slug: MetaToolSlug = serde_json::from_str(json).unwrap();
        assert!(matches!(slug, MetaToolSlug::ComposioRemoteBashTool));
    }

    #[test]
    fn test_tag_type_serialization() {
        let tag = TagType::ReadOnlyHint;
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"READ_ONLY_HINT\"");

        let tag = TagType::DestructiveHint;
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"DESTRUCTIVE_HINT\"");

        let tag = TagType::IdempotentHint;
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"IDEMPOTENT_HINT\"");

        let tag = TagType::OpenWorldHint;
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"OPEN_WORLD_HINT\"");
    }

    #[test]
    fn test_tag_type_deserialization() {
        let json = "\"READ_ONLY_HINT\"";
        let tag: TagType = serde_json::from_str(json).unwrap();
        assert!(matches!(tag, TagType::ReadOnlyHint));

        let json = "\"DESTRUCTIVE_HINT\"";
        let tag: TagType = serde_json::from_str(json).unwrap();
        assert!(matches!(tag, TagType::DestructiveHint));

        let json = "\"IDEMPOTENT_HINT\"";
        let tag: TagType = serde_json::from_str(json).unwrap();
        assert!(matches!(tag, TagType::IdempotentHint));

        let json = "\"OPEN_WORLD_HINT\"";
        let tag: TagType = serde_json::from_str(json).unwrap();
        assert!(matches!(tag, TagType::OpenWorldHint));
    }

    #[test]
    fn test_auth_scheme_serialization() {
        let scheme = AuthScheme::Oauth2;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"OAUTH2\"");

        let scheme = AuthScheme::Oauth1;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"OAUTH1\"");

        let scheme = AuthScheme::ApiKey;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"API_KEY\"");

        let scheme = AuthScheme::BearerToken;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"BEARER_TOKEN\"");

        let scheme = AuthScheme::Basic;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"BASIC\"");

        let scheme = AuthScheme::NoAuth;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"NO_AUTH\"");

        let scheme = AuthScheme::Snowflake;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"SNOWFLAKE\"");

        let scheme = AuthScheme::CalcomAuth;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"CALCOM_AUTH\"");

        let scheme = AuthScheme::BillcomAuth;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"BILLCOM_AUTH\"");

        let scheme = AuthScheme::ComposioLink;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"COMPOSIO_LINK\"");

        let scheme = AuthScheme::BasicWithJwt;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"BASIC_WITH_JWT\"");

        let scheme = AuthScheme::GoogleServiceAccount;
        let json = serde_json::to_string(&scheme).unwrap();
        assert_eq!(json, "\"GOOGLE_SERVICE_ACCOUNT\"");
    }

    #[test]
    fn test_auth_scheme_deserialization() {
        let json = "\"OAUTH2\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::Oauth2));

        let json = "\"OAUTH1\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::Oauth1));

        let json = "\"API_KEY\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::ApiKey));

        let json = "\"BEARER_TOKEN\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::BearerToken));

        let json = "\"BASIC\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::Basic));

        let json = "\"NO_AUTH\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::NoAuth));

        let json = "\"SNOWFLAKE\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::Snowflake));

        let json = "\"CALCOM_AUTH\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::CalcomAuth));

        let json = "\"BILLCOM_AUTH\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::BillcomAuth));

        let json = "\"COMPOSIO_LINK\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::ComposioLink));

        let json = "\"BASIC_WITH_JWT\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::BasicWithJwt));

        let json = "\"GOOGLE_SERVICE_ACCOUNT\"";
        let scheme: AuthScheme = serde_json::from_str(json).unwrap();
        assert!(matches!(scheme, AuthScheme::GoogleServiceAccount));
    }

    #[test]
    fn test_enum_copy_trait() {
        // Test that Copy trait works for MetaToolSlug and TagType
        let slug1 = MetaToolSlug::ComposioSearchTools;
        let slug2 = slug1; // Copy
        let _slug3 = slug1; // Can still use slug1 after copy
        assert!(matches!(slug2, MetaToolSlug::ComposioSearchTools));

        let tag1 = TagType::ReadOnlyHint;
        let tag2 = tag1; // Copy
        let _tag3 = tag1; // Can still use tag1 after copy
        assert!(matches!(tag2, TagType::ReadOnlyHint));
    }

    #[test]
    fn test_enum_clone_trait() {
        // Test that Clone trait works for all enums
        let slug = MetaToolSlug::ComposioSearchTools;
        let slug_clone = slug.clone();
        assert!(matches!(slug_clone, MetaToolSlug::ComposioSearchTools));

        let tag = TagType::ReadOnlyHint;
        let tag_clone = tag.clone();
        assert!(matches!(tag_clone, TagType::ReadOnlyHint));

        let scheme = AuthScheme::Oauth2;
        let scheme_clone = scheme.clone();
        assert!(matches!(scheme_clone, AuthScheme::Oauth2));
    }

    #[test]
    fn test_enum_debug_trait() {
        // Test that Debug trait works for all enums
        let slug = MetaToolSlug::ComposioSearchTools;
        let debug_str = format!("{:?}", slug);
        assert!(debug_str.contains("ComposioSearchTools"));

        let tag = TagType::ReadOnlyHint;
        let debug_str = format!("{:?}", tag);
        assert!(debug_str.contains("ReadOnlyHint"));

        let scheme = AuthScheme::Oauth2;
        let debug_str = format!("{:?}", scheme);
        assert!(debug_str.contains("Oauth2"));
    }
}
