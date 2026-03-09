//! Toolkit version management utilities
//!
//! This module provides functions for resolving toolkit versions from multiple sources:
//! 1. User-provided configuration (highest priority)
//! 2. Toolkit-specific environment variables (COMPOSIO_TOOLKIT_VERSION_{TOOLKIT})
//! 3. Default to "latest"
//!
//! The main workflow is:
//! 1. Use `get_toolkit_versions()` to build a complete configuration from env vars and user defaults
//! 2. Use `get_toolkit_version()` to look up specific toolkit versions from that configuration
//!
//! # Examples
//!
//! ```rust
//! use composio_sdk::utils::toolkit_version::{get_toolkit_versions, get_toolkit_version};
//! use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
//! use std::collections::HashMap;
//!
//! // Build configuration from environment and defaults
//! let config = get_toolkit_versions(None);
//!
//! // Get version for a specific toolkit
//! let version = get_toolkit_version("github", Some(&config));
//! assert_eq!(version.as_str(), "latest");
//!
//! // With user-provided versions
//! let mut versions = HashMap::new();
//! versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
//! let config = get_toolkit_versions(Some(ToolkitVersionParam::Versions(versions)));
//! let version = get_toolkit_version("github", Some(&config));
//! assert_eq!(version.as_str(), "20250906_01");
//! ```

use crate::models::versioning::{ToolkitVersion, ToolkitVersionParam, ToolkitVersions};
use std::collections::HashMap;
use std::env;

/// Get toolkit versions configuration by merging environment variables and user-provided defaults
///
/// This function merges toolkit versions from multiple sources with the following priority:
/// 1. User-provided toolkit version mappings (default_versions dict)
/// 2. Environment variables (COMPOSIO_TOOLKIT_VERSION_<TOOLKIT_NAME>)
/// 3. Fallback to 'latest' if no versions are configured
///
/// # Arguments
///
/// * `default_versions` - Optional default versions configuration
///
/// # Returns
///
/// A `ToolkitVersionParam` that can be:
/// - `Latest` if no specific versions are configured
/// - `Versions(map)` with merged environment and user-provided versions
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::toolkit_version::get_toolkit_versions;
/// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
/// use std::collections::HashMap;
///
/// // With no defaults, returns Latest
/// let versions = get_toolkit_versions(None);
/// assert!(versions.is_latest());
///
/// // With user defaults
/// let mut user_versions = HashMap::new();
/// user_versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
/// let versions = get_toolkit_versions(Some(ToolkitVersionParam::Versions(user_versions)));
/// assert!(versions.is_versions());
///
/// // Environment variables are merged
/// std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GMAIL", "20250801_01");
/// let versions = get_toolkit_versions(None);
/// if let ToolkitVersionParam::Versions(map) = versions {
///     assert!(map.contains_key("gmail"));
/// }
/// std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GMAIL");
/// ```
pub fn get_toolkit_versions(
    default_versions: Option<ToolkitVersionParam>,
) -> ToolkitVersionParam {
    // If already set by user as Latest, use it as global version for all toolkits
    if let Some(ToolkitVersionParam::Latest) = default_versions {
        return ToolkitVersionParam::Latest;
    }

    // Extract toolkit versions from environment variables
    let toolkit_versions_from_env = get_versions_from_env();

    // Extract user-provided toolkit versions (already normalized to lowercase)
    let user_provided_toolkit_versions: ToolkitVersions = match default_versions {
        Some(ToolkitVersionParam::Versions(map)) => map,
        _ => HashMap::new(),
    };

    // Merge: env vars first, then user-provided (user overrides env)
    let mut toolkit_versions = toolkit_versions_from_env;
    toolkit_versions.extend(user_provided_toolkit_versions);

    // If the toolkit_versions are empty, use 'latest'
    if toolkit_versions.is_empty() {
        return ToolkitVersionParam::Latest;
    }

    ToolkitVersionParam::Versions(toolkit_versions)
}

/// Get the version for a specific toolkit based on the provided configuration
///
/// This is a simplified version that looks up the toolkit version from the configuration.
/// For complete version resolution including environment variables, use `get_toolkit_versions()`
/// first to build the configuration.
///
/// # Arguments
///
/// * `toolkit_slug` - The slug of the toolkit (e.g., "github", "gmail")
/// * `toolkit_versions` - Optional toolkit versions configuration
///
/// # Returns
///
/// The toolkit version to use - either the specific version from config, or 'latest' as fallback
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::toolkit_version::get_toolkit_version;
/// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
/// use std::collections::HashMap;
///
/// // Default to latest
/// let version = get_toolkit_version("github", None);
/// assert_eq!(version.as_str(), "latest");
///
/// // From Latest configuration
/// let version = get_toolkit_version("github", Some(&ToolkitVersionParam::Latest));
/// assert_eq!(version.as_str(), "latest");
///
/// // From Versions configuration
/// let mut versions = HashMap::new();
/// versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
/// let config = ToolkitVersionParam::Versions(versions);
/// let version = get_toolkit_version("github", Some(&config));
/// assert_eq!(version.as_str(), "20250906_01");
/// ```
pub fn get_toolkit_version(
    toolkit_slug: &str,
    toolkit_versions: Option<&ToolkitVersionParam>,
) -> ToolkitVersion {
    // If toolkit_versions is Latest, use it as a global version for all toolkits
    if let Some(ToolkitVersionParam::Latest) = toolkit_versions {
        return ToolkitVersion::Latest;
    }

    // If toolkit_versions is a Versions map, look up the specific toolkit version
    if let Some(ToolkitVersionParam::Versions(map)) = toolkit_versions {
        if let Some(version) = map.get(toolkit_slug) {
            return version.clone();
        }
    }

    // Else use 'latest'
    ToolkitVersion::Latest
}

/// Merge toolkit version configurations with override precedence
///
/// This function merges two version configurations, with the override taking precedence.
///
/// # Arguments
///
/// * `default` - Default version configuration
/// * `override_versions` - Override version configuration (takes precedence)
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::toolkit_version::merge_toolkit_versions;
/// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
/// use std::collections::HashMap;
///
/// let mut default_versions = HashMap::new();
/// default_versions.insert("github".to_string(), ToolkitVersion::Latest);
/// let default = Some(ToolkitVersionParam::Versions(default_versions));
///
/// let mut override_versions = HashMap::new();
/// override_versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
/// let override_config = Some(ToolkitVersionParam::Versions(override_versions));
///
/// let merged = merge_toolkit_versions(default, override_config);
/// // Override takes precedence
/// assert!(merged.is_versions());
/// ```
pub fn merge_toolkit_versions(
    default: Option<ToolkitVersionParam>,
    override_versions: Option<ToolkitVersionParam>,
) -> ToolkitVersionParam {
    match (default, override_versions) {
        (_, Some(override_val)) => override_val,
        (Some(default_val), None) => default_val,
        (None, None) => ToolkitVersionParam::None,
    }
}

/// Extract toolkit versions from environment variables
///
/// This function scans all environment variables for toolkit-specific version settings
/// in the format `COMPOSIO_TOOLKIT_VERSION_{TOOLKIT}` and returns a map of toolkit
/// slugs to their versions.
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::toolkit_version::get_versions_from_env;
///
/// std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");
/// std::env::set_var("COMPOSIO_TOOLKIT_VERSION_GMAIL", "latest");
///
/// let versions = get_versions_from_env();
/// assert_eq!(versions.get("github").map(|v| v.as_str()), Some("20250906_01"));
/// assert_eq!(versions.get("gmail").map(|v| v.as_str()), Some("latest"));
///
/// std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
/// std::env::remove_var("COMPOSIO_TOOLKIT_VERSION_GMAIL");
/// ```
pub fn get_versions_from_env() -> ToolkitVersions {
    let mut versions = HashMap::new();

    for (key, value) in env::vars() {
        if let Some(toolkit) = key.strip_prefix("COMPOSIO_TOOLKIT_VERSION_") {
            versions.insert(toolkit.to_lowercase(), ToolkitVersion::from(value));
        }
    }

    versions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toolkit_version_default() {
        let version = get_toolkit_version("github", None);
        assert_eq!(version.as_str(), "latest");
    }

    #[test]
    fn test_get_toolkit_version_from_config_latest() {
        let config = Some(ToolkitVersionParam::Latest);
        let version = get_toolkit_version("github", config.as_ref());
        assert_eq!(version.as_str(), "latest");
    }

    #[test]
    fn test_get_toolkit_version_from_config_versions() {
        let mut versions = HashMap::new();
        versions.insert(
            "github".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );
        let config = Some(ToolkitVersionParam::Versions(versions));
        let version = get_toolkit_version("github", config.as_ref());
        assert_eq!(version.as_str(), "20250906_01");
    }

    #[test]
    fn test_get_toolkit_version_from_config_none() {
        let config = Some(ToolkitVersionParam::None);
        let version = get_toolkit_version("github", config.as_ref());
        assert_eq!(version.as_str(), "latest");
    }

    #[test]
    fn test_get_toolkit_version_not_in_config() {
        let mut versions = HashMap::new();
        versions.insert(
            "gmail".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );
        let config = Some(ToolkitVersionParam::Versions(versions));
        let version = get_toolkit_version("github", config.as_ref());
        assert_eq!(version.as_str(), "latest");
    }

    #[test]
    fn test_get_toolkit_versions_default() {
        let versions = get_toolkit_versions(None);
        assert!(versions.is_latest());
    }

    #[test]
    fn test_get_toolkit_versions_with_latest() {
        let versions = get_toolkit_versions(Some(ToolkitVersionParam::Latest));
        assert!(versions.is_latest());
    }

    #[test]
    fn test_get_toolkit_versions_with_user_versions() {
        let mut user_versions = HashMap::new();
        user_versions.insert(
            "github".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );
        let versions = get_toolkit_versions(Some(ToolkitVersionParam::Versions(user_versions)));
        
        assert!(versions.is_versions());
        if let ToolkitVersionParam::Versions(map) = versions {
            assert_eq!(map.get("github").unwrap().as_str(), "20250906_01");
        }
    }

    #[test]
    fn test_get_toolkit_versions_from_env() {
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GMAIL", "20250801_01");

        let versions = get_toolkit_versions(None);
        
        assert!(versions.is_versions());
        if let ToolkitVersionParam::Versions(map) = versions {
            assert_eq!(map.get("github").unwrap().as_str(), "20250906_01");
            assert_eq!(map.get("gmail").unwrap().as_str(), "20250801_01");
        }

        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GMAIL");
    }

    #[test]
    fn test_get_toolkit_versions_user_overrides_env() {
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250801_01");

        let mut user_versions = HashMap::new();
        user_versions.insert(
            "github".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );
        let versions = get_toolkit_versions(Some(ToolkitVersionParam::Versions(user_versions)));
        
        assert!(versions.is_versions());
        if let ToolkitVersionParam::Versions(map) = versions {
            // User version should override env
            assert_eq!(map.get("github").unwrap().as_str(), "20250906_01");
        }

        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
    }

    #[test]
    fn test_get_toolkit_versions_merge_env_and_user() {
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GMAIL", "20250801_01");

        let mut user_versions = HashMap::new();
        user_versions.insert(
            "github".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );
        let versions = get_toolkit_versions(Some(ToolkitVersionParam::Versions(user_versions)));
        
        assert!(versions.is_versions());
        if let ToolkitVersionParam::Versions(map) = versions {
            // Should have both env and user versions
            assert_eq!(map.get("github").unwrap().as_str(), "20250906_01");
            assert_eq!(map.get("gmail").unwrap().as_str(), "20250801_01");
        }

        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GMAIL");
    }

    #[test]
    fn test_merge_toolkit_versions_override_takes_precedence() {
        let default = Some(ToolkitVersionParam::Latest);
        let override_config = Some(ToolkitVersionParam::None);

        let merged = merge_toolkit_versions(default, override_config);
        assert!(merged.is_none());
    }

    #[test]
    fn test_merge_toolkit_versions_use_default() {
        let default = Some(ToolkitVersionParam::Latest);
        let override_config = None;

        let merged = merge_toolkit_versions(default, override_config);
        assert!(merged.is_latest());
    }

    #[test]
    fn test_merge_toolkit_versions_both_none() {
        let merged = merge_toolkit_versions(None, None);
        assert!(merged.is_none());
    }

    #[test]
    fn test_get_versions_from_env() {
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GMAIL", "latest");
        env::set_var("COMPOSIO_TOOLKIT_VERSION_SLACK", "20250801_01");

        let versions = get_versions_from_env();

        assert_eq!(versions.len(), 3);
        assert_eq!(
            versions.get("github").map(|v| v.as_str()),
            Some("20250906_01")
        );
        assert_eq!(versions.get("gmail").map(|v| v.as_str()), Some("latest"));
        assert_eq!(
            versions.get("slack").map(|v| v.as_str()),
            Some("20250801_01")
        );

        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GMAIL");
        env::remove_var("COMPOSIO_TOOLKIT_VERSION_SLACK");
    }

    #[test]
    fn test_get_versions_from_env_case_insensitive() {
        env::set_var("COMPOSIO_TOOLKIT_VERSION_GITHUB", "20250906_01");

        let versions = get_versions_from_env();

        // Should be lowercase
        assert!(versions.contains_key("github"));
        assert!(!versions.contains_key("GITHUB"));

        env::remove_var("COMPOSIO_TOOLKIT_VERSION_GITHUB");
    }
}
