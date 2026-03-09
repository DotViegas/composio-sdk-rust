//! Example: Validation Error Formatting
//!
//! This example demonstrates how to use the `format_validation_error()` method
//! to get detailed, developer-friendly error messages for validation failures.
//!
//! This is particularly useful for:
//! - Debugging API validation errors
//! - Improving logging and monitoring
//! - Providing clear error messages to developers
//!
//! Run with:
//! ```bash
//! cargo run --example validation_error_formatting
//! ```

use composio_sdk::error::{ComposioError, ErrorDetail};

fn main() {
    println!("=== Validation Error Formatting Examples ===\n");

    // Example 1: Missing required fields
    println!("Example 1: Missing Required Fields");
    println!("-----------------------------------");
    let error1 = ComposioError::ApiError {
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

    println!("Standard error display:");
    println!("{}\n", error1);
    
    println!("Formatted validation error:");
    println!("{}\n", error1.format_validation_error());

    // Example 2: Invalid field values
    println!("\nExample 2: Invalid Field Values");
    println!("--------------------------------");
    let error2 = ComposioError::ApiError {
        status: 400,
        message: "Invalid request data".to_string(),
        code: Some("VALIDATION_ERROR".to_string()),
        slug: None,
        request_id: Some("req_xyz789".to_string()),
        suggested_fix: Some("Check the format of your input parameters".to_string()),
        errors: Some(vec![
            ErrorDetail {
                field: Some("email".to_string()),
                message: "Invalid email format".to_string(),
            },
            ErrorDetail {
                field: Some("toolkit_slug".to_string()),
                message: "Toolkit 'invalid-toolkit' does not exist".to_string(),
            },
            ErrorDetail {
                field: Some("limit".to_string()),
                message: "Must be between 1 and 100".to_string(),
            },
        ]),
    };

    println!("Formatted validation error:");
    println!("{}\n", error2.format_validation_error());

    // Example 3: Mixed errors (missing + invalid)
    println!("\nExample 3: Mixed Errors");
    println!("-----------------------");
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
                field: Some("auth_config_id".to_string()),
                message: "Missing required field".to_string(),
            },
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

    println!("Formatted validation error:");
    println!("{}\n", error3.format_validation_error());

    // Example 4: Simple validation error without details
    println!("\nExample 4: Simple Validation Error");
    println!("-----------------------------------");
    let error4 = ComposioError::ValidationError(
        "Invalid session configuration: toolkits must be an array".to_string()
    );

    println!("Formatted validation error:");
    println!("{}\n", error4.format_validation_error());

    // Example 5: API error without field details
    println!("\nExample 5: Generic API Error");
    println!("----------------------------");
    let error5 = ComposioError::ApiError {
        status: 400,
        message: "Bad request".to_string(),
        code: None,
        slug: None,
        request_id: Some("req_generic456".to_string()),
        suggested_fix: Some("Check the API documentation".to_string()),
        errors: None,
    };

    println!("Formatted validation error:");
    println!("{}\n", error5.format_validation_error());

    // Example 6: Practical usage in error handling
    println!("\nExample 6: Practical Usage in Error Handling");
    println!("--------------------------------------------");
    
    // Simulate an API call that returns a validation error
    let result: Result<(), ComposioError> = Err(ComposioError::ApiError {
        status: 400,
        message: "Session creation failed".to_string(),
        code: Some("VALIDATION_ERROR".to_string()),
        slug: None,
        request_id: Some("req_session789".to_string()),
        suggested_fix: Some("Ensure user_id is provided and valid".to_string()),
        errors: Some(vec![
            ErrorDetail {
                field: Some("user_id".to_string()),
                message: "Field required".to_string(),
            },
        ]),
    });

    match result {
        Ok(_) => println!("Success!"),
        Err(e) => {
            // Use format_validation_error for better logging
            eprintln!("Error creating session:");
            eprintln!("{}", e.format_validation_error());
            
            // You can also log to a monitoring system
            // logger.error(&e.format_validation_error());
        }
    }

    println!("\n=== Benefits of format_validation_error() ===");
    println!("✓ Clear separation of missing vs invalid fields");
    println!("✓ Includes request_id for tracking");
    println!("✓ Shows suggested fixes when available");
    println!("✓ Developer-friendly parameter names");
    println!("✓ Easy to parse for monitoring systems");
}
