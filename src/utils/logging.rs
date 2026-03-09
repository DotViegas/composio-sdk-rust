//! Logging utilities for Composio SDK
//!
//! This module provides logging configuration with verbosity control,
//! message truncation, and environment-based setup.
//!
//! # Features
//!
//! - Environment-based log level configuration
//! - Verbosity levels (0-3) with automatic message truncation
//! - Global logger access via `get_logger()`
//! - Optional `local-debug` feature for detailed tracing
//!
//! # Examples
//!
//! ```no_run
//! use composio_sdk::logging::{setup, LogLevel};
//!
//! // Setup logging with INFO level
//! setup(LogLevel::Info);
//!
//! // Log with automatic truncation based on verbosity
//! log::info!("Processing large payload...");
//! ```

use std::sync::OnceLock;

/// Environment variable for log level configuration
pub const ENV_COMPOSIO_LOGGING_LEVEL: &str = "COMPOSIO_LOGGING_LEVEL";

/// Environment variable for log verbosity (0-3)
pub const ENV_COMPOSIO_LOG_VERBOSITY: &str = "COMPOSIO_LOG_VERBOSITY";

/// Default logger name
const DEFAULT_LOGGER_NAME: &str = "composio";

/// Global verbosity level
static VERBOSITY: OnceLock<u8> = OnceLock::new();

/// Log verbosity levels
///
/// Controls how much detail is included in log messages:
/// - Level 0: Minimal (256 chars)
/// - Level 1: Normal (512 chars)
/// - Level 2: Verbose (1024 chars)
/// - Level 3: Full (unlimited)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Minimal verbosity (256 chars max)
    Minimal = 0,
    /// Normal verbosity (512 chars max)
    Normal = 1,
    /// Verbose (1024 chars max)
    Verbose = 2,
    /// Full verbosity (no truncation)
    Full = 3,
}

impl Verbosity {
    /// Get max line size for this verbosity level
    pub fn max_line_size(self) -> Option<usize> {
        match self {
            Verbosity::Minimal => Some(256),
            Verbosity::Normal => Some(512),
            Verbosity::Verbose => Some(1024),
            Verbosity::Full => None,
        }
    }

    /// Parse verbosity from environment variable
    fn from_env() -> Self {
        std::env::var(ENV_COMPOSIO_LOG_VERBOSITY)
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .and_then(Self::from_u8)
            .unwrap_or(Verbosity::Minimal)
    }

    /// Convert u8 to Verbosity
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Verbosity::Minimal),
            1 => Some(Verbosity::Normal),
            2 => Some(Verbosity::Verbose),
            3 => Some(Verbosity::Full),
            _ => None,
        }
    }
}

/// Log levels supported by Composio SDK
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Critical errors
    Critical,
    /// Fatal errors (alias for Critical)
    Fatal,
    /// Errors
    Error,
    /// Warnings
    Warning,
    /// Warnings (alias)
    Warn,
    /// Informational messages
    Info,
    /// Debug messages
    Debug,
    /// Not set (inherit from parent)
    NotSet,
}

impl LogLevel {
    /// Parse log level from environment variable
    pub fn from_env() -> Option<Self> {
        std::env::var(ENV_COMPOSIO_LOGGING_LEVEL)
            .ok()
            .and_then(|s| Self::from_str(&s))
    }

    /// Parse log level from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "critical" => Some(LogLevel::Critical),
            "fatal" => Some(LogLevel::Fatal),
            "error" => Some(LogLevel::Error),
            "warning" => Some(LogLevel::Warning),
            "warn" => Some(LogLevel::Warn),
            "info" => Some(LogLevel::Info),
            "debug" => Some(LogLevel::Debug),
            "notset" => Some(LogLevel::NotSet),
            _ => None,
        }
    }

    /// Convert to log crate level
    #[cfg(feature = "local-debug")]
    pub fn to_tracing_level(self) -> tracing::Level {
        match self {
            LogLevel::Critical | LogLevel::Fatal => tracing::Level::ERROR,
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warning | LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::NotSet => tracing::Level::INFO,
        }
    }
}

/// Get current verbosity level
pub fn get_verbosity() -> Verbosity {
    let level = *VERBOSITY.get_or_init(|| Verbosity::from_env() as u8);
    Verbosity::from_u8(level).unwrap_or(Verbosity::Minimal)
}

/// Set verbosity level
pub fn set_verbosity(verbosity: Verbosity) {
    let _ = VERBOSITY.set(verbosity as u8);
}

/// Truncate message based on current verbosity level
pub fn truncate_message(msg: &str) -> String {
    let verbosity = get_verbosity();
    
    match verbosity.max_line_size() {
        None => msg.to_string(),
        Some(max_size) => {
            if msg.len() <= max_size {
                msg.to_string()
            } else {
                format!("{}...", &msg[..max_size])
            }
        }
    }
}

/// Setup logging with specified level
///
/// This function initializes the logging system with the given log level.
/// If the `local-debug` feature is enabled, it uses `tracing_subscriber`.
///
/// # Arguments
///
/// * `level` - The log level to use
///
/// # Examples
///
/// ```no_run
/// use composio_sdk::logging::{setup, LogLevel};
///
/// setup(LogLevel::Debug);
/// ```
pub fn setup(level: LogLevel) {
    #[cfg(feature = "local-debug")]
    {
        let tracing_level = level.to_tracing_level();
        
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing_level)
            .with_target(true)
            .with_thread_ids(false)
            .with_line_number(true)
            .with_file(true)
            .try_init();
    }

    #[cfg(not(feature = "local-debug"))]
    {
        // When local-debug is not enabled, logging is a no-op
        let _ = level;
    }
}

/// Setup logging from environment variables
///
/// Reads `COMPOSIO_LOGGING_LEVEL` and `COMPOSIO_LOG_VERBOSITY` from environment.
///
/// # Examples
///
/// ```no_run
/// use composio_sdk::logging::setup_from_env;
///
/// // Reads COMPOSIO_LOGGING_LEVEL=debug
/// setup_from_env();
/// ```
pub fn setup_from_env() {
    let level = LogLevel::from_env().unwrap_or(LogLevel::Info);
    setup(level);
}

/// Trait for types that can have logging capabilities
///
/// This trait provides a standard way to add logging to any type.
/// It's the Rust equivalent of Python's `WithLogger` mixin class.
///
/// # Examples
///
/// ```no_run
/// use composio_sdk::logging::WithLogger;
///
/// struct MyService {
///     logger_name: String,
/// }
///
/// impl WithLogger for MyService {
///     fn logger_name(&self) -> &str {
///         &self.logger_name
///     }
/// }
/// ```
pub trait WithLogger {
    /// Get the logger name for this type
    fn logger_name(&self) -> &str {
        DEFAULT_LOGGER_NAME
    }

    /// Log an info message with truncation
    fn log_info(&self, msg: &str) {
        #[cfg(feature = "local-debug")]
        {
            let truncated = truncate_message(msg);
            tracing::info!("[{}] {}", self.logger_name(), truncated);
        }
        #[cfg(not(feature = "local-debug"))]
        {
            let _ = msg;
        }
    }

    /// Log a debug message with truncation
    fn log_debug(&self, msg: &str) {
        #[cfg(feature = "local-debug")]
        {
            let truncated = truncate_message(msg);
            tracing::debug!("[{}] {}", self.logger_name(), truncated);
        }
        #[cfg(not(feature = "local-debug"))]
        {
            let _ = msg;
        }
    }

    /// Log a warning message (no truncation for warnings)
    fn log_warning(&self, msg: &str) {
        #[cfg(feature = "local-debug")]
        {
            tracing::warn!("[{}] {}", self.logger_name(), msg);
        }
        #[cfg(not(feature = "local-debug"))]
        {
            let _ = msg;
        }
    }

    /// Log an error message (no truncation for errors)
    fn log_error(&self, msg: &str) {
        #[cfg(feature = "local-debug")]
        {
            tracing::error!("[{}] {}", self.logger_name(), msg);
        }
        #[cfg(not(feature = "local-debug"))]
        {
            let _ = msg;
        }
    }
}

/// Log an error with appropriate verbosity level
///
/// This function logs errors with context-aware formatting:
/// - For validation errors (400), uses detailed formatting
/// - For other errors, uses standard display
/// - Respects the current verbosity level
///
/// # Arguments
///
/// * `error` - The error to log
/// * `context` - Optional context string (e.g., "Session creation", "Tool execution")
///
/// # Example
///
/// ```rust
/// use composio_sdk::utils::logging::log_error;
/// use composio_sdk::error::ComposioError;
///
/// let error = ComposioError::ValidationError("Invalid input".to_string());
/// log_error(&error, Some("Creating session"));
/// ```
pub fn log_error(error: &crate::error::ComposioError, context: Option<&str>) {
    use crate::error::ComposioError;
    
    let verbosity = get_verbosity();
    
    let prefix = if let Some(ctx) = context {
        format!("[{}] ", ctx)
    } else {
        String::new()
    };
    
    // Use detailed formatting for validation errors
    let message = match error {
        ComposioError::ApiError { status: 400, .. } => {
            format!("{}Validation Error:\n{}", prefix, error.format_validation_error())
        }
        ComposioError::ValidationError(_) => {
            format!("{}{}", prefix, error.format_validation_error())
        }
        _ => {
            format!("{}{}", prefix, error)
        }
    };
    
    // Log based on verbosity
    match verbosity {
        Verbosity::Minimal => {
            eprintln!("{}", truncate_message(&message));
        }
        Verbosity::Normal => {
            eprintln!("{}", message);
        }
        Verbosity::Verbose => {
            eprintln!("{}", message);
            if let ComposioError::ApiError { request_id: Some(req_id), .. } = error {
                eprintln!("Request ID: {}", req_id);
            }
        }
        Verbosity::Full => {
            eprintln!("{}", message);
            eprintln!("Error details: {:?}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbosity_max_line_size() {
        assert_eq!(Verbosity::Minimal.max_line_size(), Some(256));
        assert_eq!(Verbosity::Normal.max_line_size(), Some(512));
        assert_eq!(Verbosity::Verbose.max_line_size(), Some(1024));
        assert_eq!(Verbosity::Full.max_line_size(), None);
    }

    #[test]
    fn test_truncate_message() {
        // Note: We can't reliably test set_verbosity in unit tests because
        // OnceLock can only be set once per process. Instead, we test the
        // truncation logic directly.
        
        let short_msg = "Short message";
        let result = truncate_message(short_msg);
        // Short messages should never be truncated
        assert_eq!(result, short_msg);
        
        // Test with a very long message that should be truncated
        // regardless of verbosity level (unless Full)
        let long_msg = "a".repeat(2000);
        let result = truncate_message(&long_msg);
        
        // The result should either be the full message (if verbosity is Full)
        // or truncated (if any other verbosity level)
        if result.len() < long_msg.len() {
            // Message was truncated
            assert!(result.ends_with("..."), "Truncated message should end with ...");
        } else {
            // Message was not truncated (verbosity is Full)
            assert_eq!(result, long_msg);
        }
    }

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("info"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_verbosity_from_u8() {
        assert_eq!(Verbosity::from_u8(0), Some(Verbosity::Minimal));
        assert_eq!(Verbosity::from_u8(1), Some(Verbosity::Normal));
        assert_eq!(Verbosity::from_u8(2), Some(Verbosity::Verbose));
        assert_eq!(Verbosity::from_u8(3), Some(Verbosity::Full));
        assert_eq!(Verbosity::from_u8(4), None);
    }

    struct TestLogger {
        name: String,
    }

    impl WithLogger for TestLogger {
        fn logger_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_with_logger_trait() {
        let logger = TestLogger {
            name: "test_logger".to_string(),
        };
        
        assert_eq!(logger.logger_name(), "test_logger");
        
        // These should not panic
        logger.log_info("Test info message");
        logger.log_debug("Test debug message");
        logger.log_warning("Test warning message");
        logger.log_error("Test error message");
    }

    #[test]
    fn test_log_error_with_validation_error() {
        use crate::error::{ComposioError, ErrorDetail};
        
        let error = ComposioError::ApiError {
            status: 400,
            message: "Validation failed".to_string(),
            code: Some("VALIDATION_ERROR".to_string()),
            slug: None,
            request_id: Some("req_test123".to_string()),
            suggested_fix: Some("Check your input".to_string()),
            errors: Some(vec![
                ErrorDetail {
                    field: Some("user_id".to_string()),
                    message: "Field required".to_string(),
                },
            ]),
        };
        
        // This test just ensures the function doesn't panic
        // Actual output would go to stderr
        log_error(&error, Some("Test context"));
        log_error(&error, None);
    }

    #[test]
    fn test_log_error_with_other_errors() {
        use crate::error::ComposioError;
        
        let error1 = ComposioError::ConfigError("Invalid config".to_string());
        log_error(&error1, Some("Configuration"));
        
        let error2 = ComposioError::ValidationError("Invalid input".to_string());
        log_error(&error2, None);
    }
}
