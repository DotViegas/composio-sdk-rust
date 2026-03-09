use serde::Deserialize;
use thiserror::Error;

/// Main error type for the Composio SDK
#[derive(Debug, Error)]
pub enum ComposioError {
    /// API error returned from Composio backend
    #[error("API error: {message} (status: {status})")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message
        message: String,
        /// Error code (optional)
        code: Option<String>,
        /// Error slug identifier (optional)
        slug: Option<String>,
        /// Request ID for debugging (optional)
        request_id: Option<String>,
        /// Suggested fix for the error (optional)
        suggested_fix: Option<String>,
        /// Detailed field-level errors (optional)
        errors: Option<Vec<ErrorDetail>>,
    },

    /// Network error from HTTP client
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Execution error
    #[error("Execution error: {0}")]
    ExecutionError(String),

    /// Invalid input provided by user
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// File not found error
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// Invalid file error
    #[error("Invalid file: {0}")]
    InvalidFile(String),

    /// File upload failed
    #[error("File upload failed: {0}")]
    UploadFailed(String),

    /// File download failed
    #[error("File download failed: {0}")]
    DownloadFailed(String),

    /// File too large error
    #[error("File too large: {0}")]
    FileTooLarge(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid schema error
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    // ============================================
    // Resource & Not Found Errors
    // ============================================
    
    /// Resource not found error
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Tool not found error
    #[error("Tool not found: {tool_slug}. Use COMPOSIO_SEARCH_TOOLS to discover available tools")]
    ToolNotFound {
        /// Tool slug that was not found
        tool_slug: String,
    },

    /// Connected account not found error
    #[error("Connected account not found: {account_id}")]
    ConnectedAccountNotFound {
        /// Account ID that was not found
        account_id: String,
    },

    /// Invalid connected account error
    #[error("Invalid connected account: {reason}")]
    InvalidConnectedAccount {
        /// Reason for invalidity
        reason: String,
    },

    /// Multiple connected accounts found when one was expected
    #[error("Multiple connected accounts found for user '{user_id}' and toolkit '{toolkit}'. Please specify connected_account_id explicitly")]
    MultipleConnectedAccounts {
        /// User ID
        user_id: String,
        /// Toolkit slug
        toolkit: String,
        /// Number of accounts found
        count: usize,
    },

    /// No items found in collection
    #[error("No items found: {0}")]
    NoItemsFound(String),

    // ============================================
    // Authentication & API Key Errors
    // ============================================
    
    /// API key not provided error
    #[error("API Key not provided. Either provide API key or export it as 'COMPOSIO_API_KEY' environment variable")]
    ApiKeyNotProvided,

    /// Invalid API key error
    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    // ============================================
    // Enum & Validation Errors
    // ============================================
    
    /// Invalid enum value with suggestions
    #[error("Invalid value '{value}' for enum '{enum_name}'{suggestion}")]
    InvalidEnum {
        /// The invalid value provided
        value: String,
        /// Name of the enum type
        enum_name: String,
        /// Suggested correct value (if found)
        suggestion: String,
        /// List of valid values
        valid_values: Vec<String>,
    },

    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    // ============================================
    // Versioning Errors
    // ============================================
    
    /// Toolkit version required error
    #[error("Toolkit version not specified. For manual execution of the tool please pass a specific toolkit version.\n\nPossible fixes:\n1. Pass the toolkit version as a parameter to the execute function ('latest' is not supported in manual execution)\n2. Set the toolkit versions in the Composio config (toolkit_versions={{'<toolkit-slug>': '<toolkit-version>'}})\n3. Set the toolkit version in the environment variable (COMPOSIO_TOOLKIT_VERSION_<TOOLKIT_SLUG>)\n4. Set dangerously_skip_version_check to True (this might cause unexpected behavior when new versions of the tools are released)")]
    ToolVersionRequired,

    /// Version selection error
    #[error("Error selecting version for tool '{tool}': requested '{requested}', locked '{locked}'")]
    VersionSelectionError {
        /// Tool name
        tool: String,
        /// Requested version
        requested: String,
        /// Locked version
        locked: String,
    },

    /// Invalid version string
    #[error("Invalid version string: {0}")]
    InvalidVersionString(String),

    /// Lock file error
    #[error("Lock file error: {0}")]
    LockFileError(String),

    /// Invalid lock file
    #[error("Invalid lock file: {0}")]
    InvalidLockFile(String),

    // ============================================
    // Trigger & Webhook Errors
    // ============================================
    
    /// Trigger error
    #[error("Trigger error: {0}")]
    TriggerError(String),

    /// Webhook signature verification failed
    #[error("Webhook signature verification failed: {0}")]
    WebhookSignatureVerificationError(String),

    /// Invalid webhook payload
    #[error("Invalid webhook payload: {0}")]
    WebhookPayloadError(String),

    /// Trigger subscription error
    #[error("Trigger subscription error: {0}")]
    TriggerSubscriptionError(String),

    /// Invalid trigger filters
    #[error("Invalid trigger filters: {0}")]
    InvalidTriggerFilters(String),

    // ============================================
    // Tool Execution & Modifier Errors
    // ============================================
    
    /// Invalid modifier error
    #[error("Invalid modifier: {0}")]
    InvalidModifier(String),

    /// Tool execution function not set
    #[error("Tool execution function not set: {0}")]
    ExecuteToolFnNotSet(String),

    /// Error processing tool execution request
    #[error("Error processing tool execution request: {0}")]
    ErrorProcessingToolExecution(String),

    // ============================================
    // Timeout & Size Errors
    // ============================================
    
    /// Request timeout error
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// Response too large error
    #[error("Response too large: {size} bytes exceeds maximum of {max_size} bytes")]
    ResponseTooLarge {
        /// Actual size in bytes
        size: usize,
        /// Maximum allowed size in bytes
        max_size: usize,
    },

    // ============================================
    // Usage & SDK Errors
    // ============================================
    
    /// SDK usage error
    #[error("Usage error: {0}")]
    UsageError(String),

    /// Toolkit error
    #[error("Toolkit error: {0}")]
    ToolkitError(String),
}

/// Detailed error information for individual field errors
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ErrorDetail {
    /// Field name that caused the error (optional)
    pub field: Option<String>,
    /// Error message for this field
    pub message: String,
}

/// Error response structure from Composio API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub message: String,
    /// Error code (optional)
    pub code: Option<String>,
    /// Error slug identifier (optional)
    pub slug: Option<String>,
    /// HTTP status code
    pub status: u16,
    /// Request ID for debugging (optional)
    pub request_id: Option<String>,
    /// Suggested fix for the error (optional)
    pub suggested_fix: Option<String>,
    /// Detailed field-level errors (optional)
    pub errors: Option<Vec<ErrorDetail>>,
}

impl ComposioError {
    /// Create an ApiError from an HTTP response
    ///
    /// This method attempts to parse the response body as an ErrorResponse.
    /// If parsing fails, it creates a generic ApiError with the status code.
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();

        match response.json::<ErrorResponse>().await {
            Ok(err_resp) => ComposioError::ApiError {
                status,
                message: err_resp.message,
                code: err_resp.code,
                slug: err_resp.slug,
                request_id: err_resp.request_id,
                suggested_fix: err_resp.suggested_fix,
                errors: err_resp.errors,
            },
            Err(_) => ComposioError::ApiError {
                status,
                message: format!("HTTP error {}", status),
                code: None,
                slug: None,
                request_id: None,
                suggested_fix: None,
                errors: None,
            },
        }
    }

    /// Create an InvalidEnum error with suggestions
    ///
    /// Uses fuzzy matching to suggest the closest valid value.
    /// Similar to Python's EnumStringNotFound with difflib.
    ///
    /// # Example
    ///
    /// ```rust
    /// use composio_sdk::error::ComposioError;
    ///
    /// let valid_values = vec!["github".to_string(), "gmail".to_string(), "slack".to_string()];
    /// let error = ComposioError::invalid_enum("gihub", "Toolkit", valid_values);
    /// // Error message: "Invalid value 'gihub' for enum 'Toolkit'. Did you mean 'github'?"
    /// ```
    pub fn invalid_enum(value: impl Into<String>, enum_name: impl Into<String>, valid_values: Vec<String>) -> Self {
        let value = value.into();
        let enum_name = enum_name.into();
        
        // Find closest match using simple string distance
        let suggestion = find_closest_match(&value, &valid_values)
            .map(|m| format!(". Did you mean '{}'?", m))
            .unwrap_or_default();

        ComposioError::InvalidEnum {
            value,
            enum_name,
            suggestion,
            valid_values,
        }
    }

    /// Create a MultipleConnectedAccounts error
    pub fn multiple_connected_accounts(
        user_id: impl Into<String>,
        toolkit: impl Into<String>,
        count: usize,
    ) -> Self {
        ComposioError::MultipleConnectedAccounts {
            user_id: user_id.into(),
            toolkit: toolkit.into(),
            count,
        }
    }

    /// Create a VersionSelectionError
    pub fn version_selection_error(
        tool: impl Into<String>,
        requested: impl Into<String>,
        locked: impl Into<String>,
    ) -> Self {
        ComposioError::VersionSelectionError {
            tool: tool.into(),
            requested: requested.into(),
            locked: locked.into(),
        }
    }

    /// Create a ResponseTooLarge error
    pub fn response_too_large(size: usize, max_size: usize) -> Self {
        ComposioError::ResponseTooLarge { size, max_size }
    }

    /// Create a ToolNotFound error
    pub fn tool_not_found(tool_slug: impl Into<String>) -> Self {
        ComposioError::ToolNotFound {
            tool_slug: tool_slug.into(),
        }
    }

    /// Create a ConnectedAccountNotFound error
    pub fn connected_account_not_found(account_id: impl Into<String>) -> Self {
        ComposioError::ConnectedAccountNotFound {
            account_id: account_id.into(),
        }
    }

    /// Format validation error with detailed field information
    ///
    /// Provides a developer-friendly error message highlighting:
    /// - Missing required fields
    /// - Invalid field values
    /// - Field-specific error messages
    ///
    /// This is particularly useful for debugging API validation errors
    /// and improving monitoring/logging output.
    ///
    /// # Returns
    ///
    /// A formatted string with:
    /// - Base error message
    /// - List of missing fields (if any)
    /// - Detailed field-level errors with parameter names
    ///
    /// # Example
    ///
    /// ```rust
    /// use composio_sdk::error::{ComposioError, ErrorDetail};
    ///
    /// let error = ComposioError::ApiError {
    ///     status: 400,
    ///     message: "Validation failed".to_string(),
    ///     code: Some("VALIDATION_ERROR".to_string()),
    ///     slug: None,
    ///     request_id: Some("req_123".to_string()),
    ///     suggested_fix: None,
    ///     errors: Some(vec![
    ///         ErrorDetail {
    ///             field: Some("user_id".to_string()),
    ///             message: "Field required".to_string(),
    ///         },
    ///         ErrorDetail {
    ///             field: Some("toolkit".to_string()),
    ///             message: "Invalid toolkit name".to_string(),
    ///         }
    ///     ]),
    /// };
    /// 
    /// let formatted = error.format_validation_error();
    /// println!("{}", formatted);
    /// // Output:
    /// // Validation failed
    /// // - Following fields are missing: ["user_id"]
    /// // - Invalid toolkit name on parameter `toolkit`
    /// ```
    pub fn format_validation_error(&self) -> String {
        match self {
            ComposioError::ApiError { 
                message, 
                errors, 
                request_id,
                suggested_fix,
                .. 
            } => {
                let mut output = message.clone();
                
                // Add request ID for tracking
                if let Some(req_id) = request_id {
                    output.push_str(&format!(" (request_id: {})", req_id));
                }
                
                if let Some(error_details) = errors {
                    let mut missing_fields = Vec::new();
                    let mut other_errors = Vec::new();
                    
                    for detail in error_details {
                        let field_name = detail.field.as_deref().unwrap_or("unknown");
                        let msg_lower = detail.message.to_lowercase();
                        
                        // Detect missing/required field errors
                        if msg_lower.contains("required") 
                            || msg_lower.contains("missing")
                            || msg_lower.contains("field is required") {
                            missing_fields.push(field_name);
                        } else {
                            other_errors.push(format!(
                                "{} on parameter `{}`",
                                detail.message, field_name
                            ));
                        }
                    }
                    
                    // Format missing fields
                    if !missing_fields.is_empty() {
                        output.push_str(&format!(
                            "\n- Following fields are missing: {:?}",
                            missing_fields
                        ));
                    }
                    
                    // Format other errors
                    if !other_errors.is_empty() {
                        output.push_str("\n- ");
                        output.push_str(&other_errors.join("\n- "));
                    }
                }
                
                // Add suggested fix if available
                if let Some(fix) = suggested_fix {
                    output.push_str(&format!("\n\nSuggested fix: {}", fix));
                }
                
                output
            }
            ComposioError::ValidationError(msg) => {
                format!("Validation error: {}", msg)
            }
            _ => format!("{}", self),
        }
    }

    /// Check if this error should be retried
    ///
    /// Returns true for transient errors that may succeed on retry:
    /// - 429 (Rate Limited)
    /// - 500 (Internal Server Error)
    /// - 502 (Bad Gateway)
    /// - 503 (Service Unavailable)
    /// - 504 (Gateway Timeout)
    /// - Network errors
    /// - Timeout errors
    ///
    /// Returns false for client errors (4xx except 429) that won't succeed on retry.
    pub fn is_retryable(&self) -> bool {
        match self {
            ComposioError::ApiError { status, .. } => {
                matches!(status, 429 | 500 | 502 | 503 | 504)
            }
            ComposioError::NetworkError(_) => true,
            ComposioError::Timeout(_) => true,
            _ => false,
        }
    }
}

/// Find the closest matching string using Levenshtein distance
///
/// Returns the closest match if the distance is within a reasonable threshold.
fn find_closest_match(target: &str, candidates: &[String]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    let target_lower = target.to_lowercase();
    let mut best_match: Option<(String, usize)> = None;

    for candidate in candidates {
        let candidate_lower = candidate.to_lowercase();
        let distance = levenshtein_distance(&target_lower, &candidate_lower);
        
        match &best_match {
            None => best_match = Some((candidate.clone(), distance)),
            Some((_, best_distance)) if distance < *best_distance => {
                best_match = Some((candidate.clone(), distance));
            }
            _ => {}
        }
    }

    // Only suggest if distance is reasonable (less than half the length)
    best_match.and_then(|(matched, distance)| {
        let threshold = target.len() / 2;
        if distance <= threshold {
            Some(matched)
        } else {
            None
        }
    })
}

/// Calculate Levenshtein distance between two strings
///
/// This is a simple implementation for fuzzy string matching.
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1,      // insertion
                ),
                matrix[i - 1][j - 1] + cost,   // substitution
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let error = ComposioError::ApiError {
            status: 404,
            message: "Resource not found".to_string(),
            code: Some("NOT_FOUND".to_string()),
            slug: Some("resource-not-found".to_string()),
            request_id: Some("req_123".to_string()),
            suggested_fix: Some("Check the resource ID".to_string()),
            errors: None,
        };

        let display = format!("{}", error);
        assert!(display.contains("API error"));
        assert!(display.contains("Resource not found"));
        assert!(display.contains("404"));
    }

    #[test]
    fn test_invalid_input_error() {
        let error = ComposioError::InvalidInput("Invalid API key".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Invalid input"));
        assert!(display.contains("Invalid API key"));
    }

    #[test]
    fn test_config_error() {
        let error = ComposioError::ConfigError("Invalid base URL".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Configuration error"));
        assert!(display.contains("Invalid base URL"));
    }

    #[test]
    fn test_serialization_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
            .unwrap_err();
        let error: ComposioError = json_error.into();

        match error {
            ComposioError::SerializationError(_) => (),
            _ => panic!("Expected SerializationError"),
        }
    }

    #[test]
    fn test_is_retryable_for_rate_limit() {
        let error = ComposioError::ApiError {
            status: 429,
            message: "Rate limited".to_string(),
            code: None,
            slug: None,
            request_id: None,
            suggested_fix: None,
            errors: None,
        };

        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_for_server_errors() {
        for status in [500, 502, 503, 504] {
            let error = ComposioError::ApiError {
                status,
                message: "Server error".to_string(),
                code: None,
                slug: None,
                request_id: None,
                suggested_fix: None,
                errors: None,
            };

            assert!(
                error.is_retryable(),
                "Status {} should be retryable",
                status
            );
        }
    }

    #[test]
    fn test_is_not_retryable_for_client_errors() {
        for status in [400, 401, 403, 404] {
            let error = ComposioError::ApiError {
                status,
                message: "Client error".to_string(),
                code: None,
                slug: None,
                request_id: None,
                suggested_fix: None,
                errors: None,
            };

            assert!(
                !error.is_retryable(),
                "Status {} should not be retryable",
                status
            );
        }
    }

    #[test]
    fn test_serialization_error_is_not_retryable() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
            .unwrap_err();
        let error: ComposioError = json_error.into();

        assert!(!error.is_retryable());
    }

    #[test]
    fn test_invalid_input_not_retryable() {
        let error = ComposioError::InvalidInput("Invalid API key".to_string());
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_config_error_not_retryable() {
        let error = ComposioError::ConfigError("Invalid base URL".to_string());
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_detail_deserialization() {
        let json = r#"{
            "field": "email",
            "message": "Invalid email format"
        }"#;

        let detail: ErrorDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.field, Some("email".to_string()));
        assert_eq!(detail.message, "Invalid email format");
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{
            "message": "Validation failed",
            "code": "VALIDATION_ERROR",
            "slug": "validation-failed",
            "status": 400,
            "request_id": "req_abc123",
            "suggested_fix": "Check your input parameters",
            "errors": [
                {
                    "field": "user_id",
                    "message": "User ID is required"
                }
            ]
        }"#;

        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.message, "Validation failed");
        assert_eq!(response.code, Some("VALIDATION_ERROR".to_string()));
        assert_eq!(response.status, 400);
        assert!(response.errors.is_some());
        assert_eq!(response.errors.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_error_response_minimal_deserialization() {
        let json = r#"{
            "message": "Internal server error",
            "status": 500
        }"#;

        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.message, "Internal server error");
        assert_eq!(response.status, 500);
        assert!(response.code.is_none());
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_format_validation_error_with_missing_fields() {
        let error = ComposioError::ApiError {
            status: 400,
            message: "Validation failed".to_string(),
            code: Some("VALIDATION_ERROR".to_string()),
            slug: None,
            request_id: Some("req_abc123".to_string()),
            suggested_fix: Some("Provide all required fields".to_string()),
            errors: Some(vec![
                ErrorDetail {
                    field: Some("user_id".to_string()),
                    message: "Field required".to_string(),
                },
                ErrorDetail {
                    field: Some("toolkit".to_string()),
                    message: "This field is required".to_string(),
                },
            ]),
        };

        let formatted = error.format_validation_error();
        
        assert!(formatted.contains("Validation failed"));
        assert!(formatted.contains("request_id: req_abc123"));
        assert!(formatted.contains("Following fields are missing"));
        assert!(formatted.contains("user_id"));
        assert!(formatted.contains("toolkit"));
        assert!(formatted.contains("Suggested fix: Provide all required fields"));
    }

    #[test]
    fn test_format_validation_error_with_invalid_values() {
        let error = ComposioError::ApiError {
            status: 400,
            message: "Invalid request data".to_string(),
            code: Some("VALIDATION_ERROR".to_string()),
            slug: None,
            request_id: None,
            suggested_fix: None,
            errors: Some(vec![
                ErrorDetail {
                    field: Some("email".to_string()),
                    message: "Invalid email format".to_string(),
                },
                ErrorDetail {
                    field: Some("age".to_string()),
                    message: "Must be a positive integer".to_string(),
                },
            ]),
        };

        let formatted = error.format_validation_error();
        
        assert!(formatted.contains("Invalid request data"));
        assert!(formatted.contains("Invalid email format on parameter `email`"));
        assert!(formatted.contains("Must be a positive integer on parameter `age`"));
        assert!(!formatted.contains("Following fields are missing"));
    }

    #[test]
    fn test_format_validation_error_mixed_errors() {
        let error = ComposioError::ApiError {
            status: 400,
            message: "Validation failed".to_string(),
            code: Some("VALIDATION_ERROR".to_string()),
            slug: None,
            request_id: Some("req_xyz789".to_string()),
            suggested_fix: Some("Check your input parameters".to_string()),
            errors: Some(vec![
                ErrorDetail {
                    field: Some("user_id".to_string()),
                    message: "Field required".to_string(),
                },
                ErrorDetail {
                    field: Some("toolkit".to_string()),
                    message: "Invalid toolkit name".to_string(),
                },
                ErrorDetail {
                    field: Some("auth_config".to_string()),
                    message: "Missing required field".to_string(),
                },
            ]),
        };

        let formatted = error.format_validation_error();
        
        // Should have both missing fields and other errors
        assert!(formatted.contains("Following fields are missing"));
        assert!(formatted.contains("user_id"));
        assert!(formatted.contains("auth_config"));
        assert!(formatted.contains("Invalid toolkit name on parameter `toolkit`"));
        assert!(formatted.contains("Suggested fix: Check your input parameters"));
    }

    #[test]
    fn test_format_validation_error_no_field_details() {
        let error = ComposioError::ApiError {
            status: 400,
            message: "Bad request".to_string(),
            code: None,
            slug: None,
            request_id: None,
            suggested_fix: None,
            errors: None,
        };

        let formatted = error.format_validation_error();
        
        assert_eq!(formatted, "Bad request");
    }

    #[test]
    fn test_format_validation_error_for_validation_error_type() {
        let error = ComposioError::ValidationError("Invalid session configuration".to_string());
        
        let formatted = error.format_validation_error();
        
        assert_eq!(formatted, "Validation error: Invalid session configuration");
    }

    #[test]
    fn test_format_validation_error_for_other_error_types() {
        let error = ComposioError::ConfigError("Invalid base URL".to_string());
        
        let formatted = error.format_validation_error();
        
        assert!(formatted.contains("Configuration error"));
        assert!(formatted.contains("Invalid base URL"));
    }

    #[test]
    fn test_format_validation_error_with_unknown_field() {
        let error = ComposioError::ApiError {
            status: 400,
            message: "Validation failed".to_string(),
            code: None,
            slug: None,
            request_id: None,
            suggested_fix: None,
            errors: Some(vec![
                ErrorDetail {
                    field: None,
                    message: "Unknown validation error".to_string(),
                },
            ]),
        };

        let formatted = error.format_validation_error();
        
        assert!(formatted.contains("Unknown validation error on parameter `unknown`"));
    }

    // ============================================
    // Tests for new error types
    // ============================================

    #[test]
    fn test_invalid_enum_with_suggestion() {
        let valid_values = vec!["github".to_string(), "gmail".to_string(), "slack".to_string()];
        let error = ComposioError::invalid_enum("gihub", "Toolkit", valid_values);

        let display = format!("{}", error);
        assert!(display.contains("Invalid value 'gihub' for enum 'Toolkit'"));
        assert!(display.contains("Did you mean 'github'?"));
    }

    #[test]
    fn test_invalid_enum_without_close_match() {
        let valid_values = vec!["github".to_string(), "gmail".to_string(), "slack".to_string()];
        let error = ComposioError::invalid_enum("xyz123", "Toolkit", valid_values);

        let display = format!("{}", error);
        assert!(display.contains("Invalid value 'xyz123' for enum 'Toolkit'"));
        assert!(!display.contains("Did you mean"));
    }

    #[test]
    fn test_multiple_connected_accounts_error() {
        let error = ComposioError::multiple_connected_accounts("user_123", "github", 3);

        let display = format!("{}", error);
        assert!(display.contains("Multiple connected accounts found"));
        assert!(display.contains("user_123"));
        assert!(display.contains("github"));
        assert!(display.contains("Please specify connected_account_id explicitly"));
    }

    #[test]
    fn test_version_selection_error() {
        let error = ComposioError::version_selection_error("GITHUB_CREATE_ISSUE", "1.0.0", "2.0.0");

        let display = format!("{}", error);
        assert!(display.contains("Error selecting version"));
        assert!(display.contains("GITHUB_CREATE_ISSUE"));
        assert!(display.contains("requested '1.0.0'"));
        assert!(display.contains("locked '2.0.0'"));
    }

    #[test]
    fn test_response_too_large_error() {
        let error = ComposioError::response_too_large(10_000_000, 5_000_000);

        let display = format!("{}", error);
        assert!(display.contains("Response too large"));
        assert!(display.contains("10000000 bytes"));
        assert!(display.contains("5000000 bytes"));
    }

    #[test]
    fn test_tool_not_found_error() {
        let error = ComposioError::tool_not_found("GITHUB_INVALID_ACTION");

        let display = format!("{}", error);
        assert!(display.contains("Tool not found: GITHUB_INVALID_ACTION"));
        assert!(display.contains("COMPOSIO_SEARCH_TOOLS"));
    }

    #[test]
    fn test_connected_account_not_found_error() {
        let error = ComposioError::connected_account_not_found("ca_123456");

        let display = format!("{}", error);
        assert!(display.contains("Connected account not found"));
        assert!(display.contains("ca_123456"));
    }

    #[test]
    fn test_api_key_not_provided_error() {
        let error = ComposioError::ApiKeyNotProvided;

        let display = format!("{}", error);
        assert!(display.contains("API Key not provided"));
        assert!(display.contains("COMPOSIO_API_KEY"));
    }

    #[test]
    fn test_tool_version_required_error() {
        let error = ComposioError::ToolVersionRequired;

        let display = format!("{}", error);
        assert!(display.contains("Toolkit version not specified"));
        assert!(display.contains("Possible fixes"));
        assert!(display.contains("dangerously_skip_version_check"));
    }

    #[test]
    fn test_webhook_signature_verification_error() {
        let error = ComposioError::WebhookSignatureVerificationError(
            "Invalid signature".to_string()
        );

        let display = format!("{}", error);
        assert!(display.contains("Webhook signature verification failed"));
        assert!(display.contains("Invalid signature"));
    }

    #[test]
    fn test_timeout_is_retryable() {
        let error = ComposioError::Timeout("Request timed out after 30s".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_tool_not_found_not_retryable() {
        let error = ComposioError::tool_not_found("INVALID_TOOL");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_invalid_enum_not_retryable() {
        let error = ComposioError::invalid_enum(
            "invalid",
            "TestEnum",
            vec!["valid1".to_string(), "valid2".to_string()]
        );
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_levenshtein_distance() {
        use super::levenshtein_distance;

        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "ab"), 1);
        assert_eq!(levenshtein_distance("abc", "abcd"), 1);
        assert_eq!(levenshtein_distance("github", "gihub"), 1);
        assert_eq!(levenshtein_distance("slack", "slak"), 1);
    }

    #[test]
    fn test_find_closest_match() {
        use super::find_closest_match;

        let candidates = vec![
            "github".to_string(),
            "gmail".to_string(),
            "slack".to_string(),
        ];

        assert_eq!(
            find_closest_match("gihub", &candidates),
            Some("github".to_string())
        );
        assert_eq!(
            find_closest_match("gmial", &candidates),
            Some("gmail".to_string())
        );
        assert_eq!(
            find_closest_match("slak", &candidates),
            Some("slack".to_string())
        );
        
        // No close match
        assert_eq!(find_closest_match("xyz123", &candidates), None);
        
        // Empty candidates
        assert_eq!(find_closest_match("test", &[]), None);
    }

    #[test]
    fn test_no_items_found_error() {
        let error = ComposioError::NoItemsFound("No toolkits found for query".to_string());

        let display = format!("{}", error);
        assert!(display.contains("No items found"));
        assert!(display.contains("No toolkits found"));
    }

    #[test]
    fn test_invalid_trigger_filters_error() {
        let error = ComposioError::InvalidTriggerFilters(
            "Invalid filter: unknown_field".to_string()
        );

        let display = format!("{}", error);
        assert!(display.contains("Invalid trigger filters"));
        assert!(display.contains("unknown_field"));
    }

    #[test]
    fn test_invalid_modifier_error() {
        let error = ComposioError::InvalidModifier(
            "Modifier 'before_execute' not found".to_string()
        );

        let display = format!("{}", error);
        assert!(display.contains("Invalid modifier"));
        assert!(display.contains("before_execute"));
    }
}
