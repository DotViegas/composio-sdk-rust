/// Enhanced error handling example demonstrating new error types
///
/// This example shows how to use the comprehensive error types
/// available in the Composio SDK, including:
/// - Enum validation with suggestions
/// - Version selection errors
/// - Connected account errors
/// - Webhook errors
/// - Tool discovery errors
/// - Timeout handling

use composio_sdk::error::ComposioError;

fn main() {
    println!("=== Enhanced Error Handling Examples ===\n");

    // Example 1: Invalid Enum with Suggestions
    println!("1. Invalid Enum Error (with fuzzy matching):");
    let valid_toolkits = vec![
        "github".to_string(),
        "gmail".to_string(),
        "slack".to_string(),
        "notion".to_string(),
    ];
    
    let error = ComposioError::invalid_enum("gihub", "Toolkit", valid_toolkits.clone());
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 2: Invalid Enum without close match
    println!("2. Invalid Enum Error (no close match):");
    let error = ComposioError::invalid_enum("xyz123", "Toolkit", valid_toolkits);
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 3: Multiple Connected Accounts
    println!("3. Multiple Connected Accounts Error:");
    let error = ComposioError::multiple_connected_accounts("user_123", "github", 3);
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 4: Version Selection Error
    println!("4. Version Selection Error:");
    let error = ComposioError::version_selection_error(
        "GITHUB_CREATE_ISSUE",
        "1.0.0",
        "2.0.0",
    );
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 5: Response Too Large
    println!("5. Response Too Large Error:");
    let error = ComposioError::response_too_large(10_000_000, 5_000_000);
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 6: Tool Not Found
    println!("6. Tool Not Found Error:");
    let error = ComposioError::tool_not_found("GITHUB_INVALID_ACTION");
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 7: Connected Account Not Found
    println!("7. Connected Account Not Found Error:");
    let error = ComposioError::connected_account_not_found("ca_123456");
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 8: API Key Not Provided
    println!("8. API Key Not Provided Error:");
    let error = ComposioError::ApiKeyNotProvided;
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 9: Tool Version Required
    println!("9. Tool Version Required Error:");
    let error = ComposioError::ToolVersionRequired;
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 10: Webhook Signature Verification
    println!("10. Webhook Signature Verification Error:");
    let error = ComposioError::WebhookSignatureVerificationError(
        "Invalid signature: expected 'abc123', got 'xyz789'".to_string(),
    );
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 11: Timeout Error (retryable)
    println!("11. Timeout Error (retryable):");
    let error = ComposioError::Timeout("Request timed out after 30 seconds".to_string());
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 12: Invalid Trigger Filters
    println!("12. Invalid Trigger Filters Error:");
    let error = ComposioError::InvalidTriggerFilters(
        "Unknown filter field: 'invalid_field'".to_string(),
    );
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 13: Invalid Modifier
    println!("13. Invalid Modifier Error:");
    let error = ComposioError::InvalidModifier(
        "Modifier 'before_execute' not found for tool 'GITHUB_CREATE_ISSUE'".to_string(),
    );
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 14: No Items Found
    println!("14. No Items Found Error:");
    let error = ComposioError::NoItemsFound(
        "No toolkits found matching query 'nonexistent'".to_string(),
    );
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    // Example 15: Toolkit Error
    println!("15. Toolkit Error:");
    let error = ComposioError::ToolkitError(
        "Failed to initialize toolkit 'github': missing auth config".to_string(),
    );
    println!("   Error: {}", error);
    println!("   Retryable: {}\n", error.is_retryable());

    println!("=== Error Handling Best Practices ===\n");
    println!("1. Always check is_retryable() before implementing retry logic");
    println!("2. Use specific error types for better error messages");
    println!("3. Leverage fuzzy matching for enum validation");
    println!("4. Handle version conflicts explicitly");
    println!("5. Verify webhook signatures for security");
    println!("6. Set appropriate timeouts for long-running operations");
}
