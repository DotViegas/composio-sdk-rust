//! Example: Enhanced Logging with Error Formatting
//!
//! This example demonstrates the enhanced logging capabilities including
//! validation error formatting and context-aware error logging.
//!
//! Run with different verbosity levels:
//! ```bash
//! # Minimal verbosity
//! COMPOSIO_LOG_VERBOSITY=0 cargo run --example logging_usage_enhanced
//!
//! # Verbose with request IDs
//! COMPOSIO_LOG_VERBOSITY=2 cargo run --example logging_usage_enhanced
//!
//! # Full verbosity with debug details
//! COMPOSIO_LOG_VERBOSITY=3 cargo run --example logging_usage_enhanced
//! ```

use composio_sdk::error::{ComposioError, ErrorDetail};
use composio_sdk::utils::logging::{log_error, set_verbosity, Verbosity};

fn main() {
    println!("=== Enhanced Logging with Error Formatting ===\n");

    // Example 1: Validation error with missing fields
    println!("Example 1: Validation Error - Missing Fields");
    println!("---------------------------------------------");
    
    let error1 = ComposioError::ApiError {
        status: 400,
        message: "Session creation failed".to_string(),
        code: Some("VALIDATION_ERROR".to_string()),
        slug: None,
        request_id: Some("req_example123".to_string()),
        suggested_fix: Some("Ensure user_id is provided and valid".to_string()),
        errors: Some(vec![
            ErrorDetail {
                field: Some("user_id".to_string()),
                message: "Field required".to_string(),
            },
            ErrorDetail {
                field: Some("auth_config_id".to_string()),
                message: "Missing required field".to_string(),
            },
        ]),
    };

    set_verbosity(Verbosity::Normal);
    log_error(&error1, Some("Session Creation"));

    // Example 2: Validation error with invalid values
    println!("\n\nExample 2: Validation Error - Invalid Values");
    println!("---------------------------------------------");
    
    let error2 = ComposioError::ApiError {
        status: 400,
        message: "Invalid request data".to_string(),
        code: Some("VALIDATION_ERROR".to_string()),
        slug: None,
        request_id: Some("req_xyz789".to_string()),
        suggested_fix: Some("Check the format of your input parameters".to_string()),
        errors: Some(vec![
            ErrorDetail {
                field: Some("toolkit".to_string()),
                message: "Invalid toolkit name 'github123'".to_string(),
            },
            ErrorDetail {
                field: Some("callback_url".to_string()),
                message: "Must be a valid HTTPS URL".to_string(),
            },
        ]),
    };

    log_error(&error2, Some("Tool Execution"));

    // Example 3: Mixed errors with verbose logging
    println!("\n\nExample 3: Mixed Errors - Verbose Logging");
    println!("------------------------------------------");
    
    let error3 = ComposioError::ApiError {
        status: 400,
        message: "Request validation failed".to_string(),
        code: Some("VALIDATION_ERROR".to_string()),
        slug: Some("validation-failed".to_string()),
        request_id: Some("req_mixed123".to_string()),
        suggested_fix: Some("Provide required fields and fix invalid values".to_string()),
        errors: Some(vec![
            ErrorDetail {
                field: Some("user_id".to_string()),
                message: "Field required".to_string(),
            },
            ErrorDetail {
                field: Some("toolkit".to_string()),
                message: "Invalid toolkit name".to_string(),
            },
        ]),
    };

    set_verbosity(Verbosity::Verbose);
    log_error(&error3, Some("API Call"));

    // Example 4: Simple validation error
    println!("\n\nExample 4: Simple Validation Error");
    println!("-----------------------------------");
    
    let error4 = ComposioError::ValidationError(
        "Invalid session configuration: toolkits must be an array".to_string(),
    );

    set_verbosity(Verbosity::Normal);
    log_error(&error4, Some("Configuration"));

    // Example 5: Full verbosity with debug details
    println!("\n\nExample 5: Full Verbosity with Debug Details");
    println!("---------------------------------------------");
    
    let error5 = ComposioError::ApiError {
        status: 400,
        message: "Complex validation error".to_string(),
        code: Some("VALIDATION_ERROR".to_string()),
        slug: None,
        request_id: Some("req_debug999".to_string()),
        suggested_fix: Some("Review all parameters".to_string()),
        errors: Some(vec![
            ErrorDetail {
                field: Some("limit".to_string()),
                message: "Must be between 1 and 100".to_string(),
            },
        ]),
    };

    set_verbosity(Verbosity::Full);
    log_error(&error5, Some("Pagination"));

    println!("\n\n=== Benefits of Enhanced Error Logging ===");
    println!("✓ Clear separation of missing vs invalid fields");
    println!("✓ Request ID tracking for debugging");
    println!("✓ Suggested fixes when available");
    println!("✓ Context-aware formatting");
    println!("✓ Verbosity-based detail levels");
    println!("✓ Better monitoring and debugging experience");
}
