//! Toolkit versioning types and utilities
//!
//! This module provides types for managing toolkit versions in the Composio SDK.
//! It allows you to specify which version of a toolkit to use, either "latest"
//! or a specific version string like "20250906_01".
//!
//! # Examples
//!
//! ```rust
//! use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
//! use std::collections::HashMap;
//!
//! // Use latest version for all toolkits
//! let config = ToolkitVersionParam::Latest;
//!
//! // Use specific versions
//! let mut versions = HashMap::new();
//! versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
//! versions.insert("gmail".to_string(), ToolkitVersion::Latest);
//! let config = ToolkitVersionParam::Versions(versions);
//!
//! // Don't specify version (use server default)
//! let config = ToolkitVersionParam::None;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The "latest" version constant
pub const TOOLKIT_LATEST_VERSION: &str = "latest";

/// Version of a toolkit
///
/// A toolkit version can be either "latest" (always use the most recent version)
/// or a specific version string like "20250906_01".
///
/// # Examples
///
/// ```rust
/// use composio_sdk::models::versioning::ToolkitVersion;
///
/// // Use latest version
/// let version = ToolkitVersion::Latest;
/// assert_eq!(version.as_str(), "latest");
///
/// // Use specific version
/// let version = ToolkitVersion::Specific("20250906_01".to_string());
/// assert_eq!(version.as_str(), "20250906_01");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolkitVersion {
    /// Always use the latest version
    Latest,
    /// Use a specific version (e.g., "20250906_01")
    Specific(String),
}

impl Serialize for ToolkitVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ToolkitVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(if s == TOOLKIT_LATEST_VERSION {
            Self::Latest
        } else {
            Self::Specific(s)
        })
    }
}

impl ToolkitVersion {
    /// Convert the version to a string slice for API calls
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composio_sdk::models::versioning::ToolkitVersion;
    ///
    /// let latest = ToolkitVersion::Latest;
    /// assert_eq!(latest.as_str(), "latest");
    ///
    /// let specific = ToolkitVersion::Specific("20250906_01".to_string());
    /// assert_eq!(specific.as_str(), "20250906_01");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Latest => TOOLKIT_LATEST_VERSION,
            Self::Specific(version) => version.as_str(),
        }
    }

    /// Check if this is the latest version
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composio_sdk::models::versioning::ToolkitVersion;
    ///
    /// assert!(ToolkitVersion::Latest.is_latest());
    /// assert!(!ToolkitVersion::Specific("20250906_01".to_string()).is_latest());
    /// ```
    pub fn is_latest(&self) -> bool {
        matches!(self, Self::Latest)
    }

    /// Check if this is a specific version
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composio_sdk::models::versioning::ToolkitVersion;
    ///
    /// assert!(!ToolkitVersion::Latest.is_specific());
    /// assert!(ToolkitVersion::Specific("20250906_01".to_string()).is_specific());
    /// ```
    pub fn is_specific(&self) -> bool {
        matches!(self, Self::Specific(_))
    }
}

impl Default for ToolkitVersion {
    fn default() -> Self {
        Self::Latest
    }
}

impl From<&str> for ToolkitVersion {
    fn from(s: &str) -> Self {
        if s == TOOLKIT_LATEST_VERSION {
            Self::Latest
        } else {
            Self::Specific(s.to_string())
        }
    }
}

impl From<String> for ToolkitVersion {
    fn from(s: String) -> Self {
        if s == TOOLKIT_LATEST_VERSION {
            Self::Latest
        } else {
            Self::Specific(s)
        }
    }
}

/// Map of toolkit slugs to their versions
///
/// This allows you to specify different versions for different toolkits.
///
/// # Examples
///
/// ```rust
/// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersions};
/// use std::collections::HashMap;
///
/// let mut versions: ToolkitVersions = HashMap::new();
/// versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
/// versions.insert("gmail".to_string(), ToolkitVersion::Latest);
/// ```
pub type ToolkitVersions = HashMap<String, ToolkitVersion>;

/// Parameter for specifying toolkit versions
///
/// This enum allows you to specify toolkit versions in three ways:
/// - `Latest`: Use "latest" for all toolkits
/// - `Versions`: Specify different versions for different toolkits
/// - `None`: Don't specify versions (use server default)
///
/// # Examples
///
/// ```rust
/// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
/// use std::collections::HashMap;
///
/// // Use latest for all toolkits
/// let config = ToolkitVersionParam::Latest;
///
/// // Use specific versions
/// let mut versions = HashMap::new();
/// versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
/// let config = ToolkitVersionParam::Versions(versions);
///
/// // Don't specify (use default)
/// let config = ToolkitVersionParam::None;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ToolkitVersionParam {
    /// Use a map of toolkit-specific versions
    Versions(ToolkitVersions),
    /// Use "latest" for all toolkits
    Latest,
    /// Don't specify versions (use server default)
    None,
}

impl Serialize for ToolkitVersionParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Versions(map) => map.serialize(serializer),
            Self::Latest => serializer.serialize_str(TOOLKIT_LATEST_VERSION),
            Self::None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for ToolkitVersionParam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct ToolkitVersionParamVisitor;

        impl<'de> Visitor<'de> for ToolkitVersionParamVisitor {
            type Value = ToolkitVersionParam;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string 'latest', a map of versions, or null")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == TOOLKIT_LATEST_VERSION {
                    Ok(ToolkitVersionParam::Latest)
                } else {
                    Err(de::Error::custom(format!(
                        "expected 'latest' but got '{}'",
                        value
                    )))
                }
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let versions = ToolkitVersions::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(ToolkitVersionParam::Versions(versions))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ToolkitVersionParam::None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ToolkitVersionParam::None)
            }
        }

        deserializer.deserialize_any(ToolkitVersionParamVisitor)
    }
}

impl Default for ToolkitVersionParam {
    fn default() -> Self {
        Self::None
    }
}

impl ToolkitVersionParam {
    /// Check if this is the Latest variant
    pub fn is_latest(&self) -> bool {
        matches!(self, Self::Latest)
    }

    /// Check if this is the None variant
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Check if this is the Versions variant
    pub fn is_versions(&self) -> bool {
        matches!(self, Self::Versions(_))
    }

    /// Get the version for a specific toolkit
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
    /// use std::collections::HashMap;
    ///
    /// let mut versions = HashMap::new();
    /// versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
    /// let config = ToolkitVersionParam::Versions(versions);
    ///
    /// assert_eq!(
    ///     config.get_version("github").map(|v| v.as_str()),
    ///     Some("20250906_01")
    /// );
    /// assert_eq!(config.get_version("gmail"), None);
    /// ```
    pub fn get_version(&self, toolkit_slug: &str) -> Option<&ToolkitVersion> {
        match self {
            Self::Versions(map) => map.get(toolkit_slug),
            Self::Latest | Self::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolkit_version_latest() {
        let version = ToolkitVersion::Latest;
        assert_eq!(version.as_str(), "latest");
        assert!(version.is_latest());
        assert!(!version.is_specific());
    }

    #[test]
    fn test_toolkit_version_specific() {
        let version = ToolkitVersion::Specific("20250906_01".to_string());
        assert_eq!(version.as_str(), "20250906_01");
        assert!(!version.is_latest());
        assert!(version.is_specific());
    }

    #[test]
    fn test_toolkit_version_default() {
        let version = ToolkitVersion::default();
        assert!(version.is_latest());
    }

    #[test]
    fn test_toolkit_version_from_str() {
        let latest: ToolkitVersion = "latest".into();
        assert!(latest.is_latest());

        let specific: ToolkitVersion = "20250906_01".into();
        assert!(specific.is_specific());
        assert_eq!(specific.as_str(), "20250906_01");
    }

    #[test]
    fn test_toolkit_version_from_string() {
        let latest: ToolkitVersion = "latest".to_string().into();
        assert!(latest.is_latest());

        let specific: ToolkitVersion = "20250906_01".to_string().into();
        assert!(specific.is_specific());
    }

    #[test]
    fn test_toolkit_version_serialization() {
        let latest = ToolkitVersion::Latest;
        let json = serde_json::to_string(&latest).unwrap();
        // Latest serializes as the string "latest"
        assert_eq!(json, "\"latest\"");

        let specific = ToolkitVersion::Specific("20250906_01".to_string());
        let json = serde_json::to_string(&specific).unwrap();
        assert_eq!(json, "\"20250906_01\"");
    }

    #[test]
    fn test_toolkit_version_deserialization() {
        let json = "\"latest\"";
        let version: ToolkitVersion = serde_json::from_str(json).unwrap();
        assert!(version.is_latest());

        let json = "\"20250906_01\"";
        let version: ToolkitVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version.as_str(), "20250906_01");
    }

    #[test]
    fn test_toolkit_versions_map() {
        let mut versions: ToolkitVersions = HashMap::new();
        versions.insert("github".to_string(), ToolkitVersion::Latest);
        versions.insert(
            "gmail".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );

        assert_eq!(versions.len(), 2);
        assert!(versions.get("github").unwrap().is_latest());
        assert_eq!(
            versions.get("gmail").unwrap().as_str(),
            "20250906_01"
        );
    }

    #[test]
    fn test_toolkit_version_param_latest() {
        let param = ToolkitVersionParam::Latest;
        assert!(param.is_latest());
        assert!(!param.is_none());
        assert!(!param.is_versions());
        assert_eq!(param.get_version("github"), None);
    }

    #[test]
    fn test_toolkit_version_param_none() {
        let param = ToolkitVersionParam::None;
        assert!(!param.is_latest());
        assert!(param.is_none());
        assert!(!param.is_versions());
        assert_eq!(param.get_version("github"), None);
    }

    #[test]
    fn test_toolkit_version_param_versions() {
        let mut versions = HashMap::new();
        versions.insert("github".to_string(), ToolkitVersion::Latest);
        versions.insert(
            "gmail".to_string(),
            ToolkitVersion::Specific("20250906_01".to_string()),
        );

        let param = ToolkitVersionParam::Versions(versions);
        assert!(!param.is_latest());
        assert!(!param.is_none());
        assert!(param.is_versions());

        assert!(param.get_version("github").unwrap().is_latest());
        assert_eq!(
            param.get_version("gmail").unwrap().as_str(),
            "20250906_01"
        );
        assert_eq!(param.get_version("slack"), None);
    }

    #[test]
    fn test_toolkit_version_param_default() {
        let param = ToolkitVersionParam::default();
        assert!(param.is_none());
    }

    #[test]
    fn test_toolkit_version_param_serialization() {
        let latest = ToolkitVersionParam::Latest;
        let json = serde_json::to_string(&latest).unwrap();
        // Latest serializes as the string "latest"
        assert_eq!(json, "\"latest\"");

        let none = ToolkitVersionParam::None;
        let json = serde_json::to_string(&none).unwrap();
        // None serializes as null
        assert_eq!(json, "null");

        let mut versions = HashMap::new();
        versions.insert("github".to_string(), ToolkitVersion::Latest);
        let param = ToolkitVersionParam::Versions(versions);
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains("github"));
        assert!(json.contains("latest"));
    }

    #[test]
    fn test_toolkit_version_equality() {
        let v1 = ToolkitVersion::Latest;
        let v2 = ToolkitVersion::Latest;
        assert_eq!(v1, v2);

        let v3 = ToolkitVersion::Specific("20250906_01".to_string());
        let v4 = ToolkitVersion::Specific("20250906_01".to_string());
        assert_eq!(v3, v4);

        assert_ne!(v1, v3);
    }

    #[test]
    fn test_toolkit_version_clone() {
        let v1 = ToolkitVersion::Specific("20250906_01".to_string());
        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }
}
