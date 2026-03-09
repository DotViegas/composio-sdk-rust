//! Integration tests for tool modifiers

use composio_sdk::models::{
    AfterExecute, BeforeExecute, Modifier, SchemaModifier, ToolExecuteParams,
    ToolExecutionResponse, ToolSchema,
    apply_before_execute_modifiers, apply_after_execute_modifiers, apply_schema_modifiers,
};
use std::collections::HashMap;

// Test modifier implementations
struct TestBeforeExecute;
impl BeforeExecute for TestBeforeExecute {
    fn modify(&self, _tool: &str, _toolkit: &str, mut params: ToolExecuteParams) -> ToolExecuteParams {
        params.arguments.insert("test_key".to_string(), serde_json::json!("test_value"));
        params
    }
}

struct TestAfterExecute;
impl AfterExecute for TestAfterExecute {
    fn modify(
        &self,
        _tool: &str,
        _toolkit: &str,
        mut response: ToolExecutionResponse,
    ) -> ToolExecutionResponse {
        response.successful = true;
        response
    }
}

struct TestSchemaModifier;
impl SchemaModifier for TestSchemaModifier {
    fn modify(&self, _tool: &str, _toolkit: &str, mut schema: ToolSchema) -> ToolSchema {
        schema.description = format!("Modified: {}", schema.description);
        schema
    }
}

#[test]
fn test_before_execute_modifier() {
    let modifier = Modifier::before_execute(
        vec!["TEST_TOOL".to_string()],
        vec![],
        TestBeforeExecute,
    );

    let params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    let result = modifier.apply_to_params("TEST_TOOL", "test", params).unwrap();
    assert!(result.arguments.contains_key("test_key"));
    assert_eq!(result.arguments.get("test_key").unwrap(), &serde_json::json!("test_value"));
}

#[test]
fn test_after_execute_modifier() {
    let modifier = Modifier::after_execute(
        vec!["TEST_TOOL".to_string()],
        vec![],
        TestAfterExecute,
    );

    let response = ToolExecutionResponse {
        successful: false,
        data: serde_json::json!({}),
        error: None,
        log_id: "test_log".to_string(),
    };

    let result = modifier.apply_to_response("TEST_TOOL", "test", response).unwrap();
    assert!(result.successful);
}

#[test]
fn test_schema_modifier() {
    let modifier = Modifier::schema(
        vec!["TEST_TOOL".to_string()],
        vec![],
        TestSchemaModifier,
    );

    let schema = ToolSchema {
        slug: "TEST_TOOL".to_string(),
        name: "Test Tool".to_string(),
        description: "Original description".to_string(),
        toolkit: "test".to_string(),
        input_parameters: serde_json::json!({}),
        output_parameters: serde_json::json!({}),
        scopes: vec![],
        tags: vec![],
        version: "1.0.0".to_string(),
        available_versions: vec![],
        is_deprecated: false,
        no_auth: false,
    };

    let result = modifier.apply_to_schema("TEST_TOOL", "test", schema).unwrap();
    assert_eq!(result.description, "Modified: Original description");
}

#[test]
fn test_modifier_filtering_by_tool() {
    let modifier = Modifier::before_execute(
        vec!["SPECIFIC_TOOL".to_string()],
        vec![],
        TestBeforeExecute,
    );

    let params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    // Should apply to SPECIFIC_TOOL
    let result = modifier.apply_to_params("SPECIFIC_TOOL", "test", params.clone()).unwrap();
    assert!(result.arguments.contains_key("test_key"));

    // Should NOT apply to OTHER_TOOL
    let result = modifier.apply_to_params("OTHER_TOOL", "test", params).unwrap();
    assert!(!result.arguments.contains_key("test_key"));
}

#[test]
fn test_modifier_filtering_by_toolkit() {
    let modifier = Modifier::before_execute(
        vec![],
        vec!["specific_toolkit".to_string()],
        TestBeforeExecute,
    );

    let params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    // Should apply to specific_toolkit
    let result = modifier.apply_to_params("ANY_TOOL", "specific_toolkit", params.clone()).unwrap();
    assert!(result.arguments.contains_key("test_key"));

    // Should NOT apply to other_toolkit
    let result = modifier.apply_to_params("ANY_TOOL", "other_toolkit", params).unwrap();
    assert!(!result.arguments.contains_key("test_key"));
}

#[test]
fn test_modifier_apply_to_all() {
    let modifier = Modifier::before_execute(
        vec![],  // Empty = all tools
        vec![],  // Empty = all toolkits
        TestBeforeExecute,
    );

    let params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    // Should apply to any tool/toolkit combination
    let result = modifier.apply_to_params("ANY_TOOL", "any_toolkit", params).unwrap();
    assert!(result.arguments.contains_key("test_key"));
}

#[test]
fn test_multiple_modifiers() {
    struct AddFieldModifier(String, String);
    impl BeforeExecute for AddFieldModifier {
        fn modify(&self, _tool: &str, _toolkit: &str, mut params: ToolExecuteParams) -> ToolExecuteParams {
            params.arguments.insert(self.0.clone(), serde_json::json!(self.1.clone()));
            params
        }
    }

    let modifiers = vec![
        Modifier::before_execute(
            vec![],
            vec![],
            AddFieldModifier("field1".to_string(), "value1".to_string()),
        ),
        Modifier::before_execute(
            vec![],
            vec![],
            AddFieldModifier("field2".to_string(), "value2".to_string()),
        ),
    ];

    let params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };

    let result = apply_before_execute_modifiers(&modifiers, "TEST_TOOL", "test", params).unwrap();
    
    assert!(result.arguments.contains_key("field1"));
    assert!(result.arguments.contains_key("field2"));
    assert_eq!(result.arguments.get("field1").unwrap(), &serde_json::json!("value1"));
    assert_eq!(result.arguments.get("field2").unwrap(), &serde_json::json!("value2"));
}

#[test]
fn test_modifier_type_mismatch() {
    let modifier = Modifier::before_execute(
        vec!["TEST_TOOL".to_string()],
        vec![],
        TestBeforeExecute,
    );

    let response = ToolExecutionResponse {
        successful: false,
        data: serde_json::json!({}),
        error: None,
        log_id: "test_log".to_string(),
    };

    // Should return error when applying wrong modifier type
    let result = modifier.apply_to_response("TEST_TOOL", "test", response);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("type mismatch"));
}

#[test]
fn test_apply_helper_functions() {
    let modifiers = vec![
        Modifier::before_execute(vec![], vec![], TestBeforeExecute),
        Modifier::after_execute(vec![], vec![], TestAfterExecute),
        Modifier::schema(vec![], vec![], TestSchemaModifier),
    ];

    // Test before execute helper
    let params = ToolExecuteParams {
        allow_tracing: None,
        arguments: HashMap::new(),
        connected_account_id: None,
        custom_auth_params: None,
        custom_connection_data: None,
        entity_id: None,
        text: None,
        user_id: Some("user_123".to_string()),
        version: None,
        dangerously_skip_version_check: None,
    };
    let result = apply_before_execute_modifiers(&modifiers, "TEST", "test", params).unwrap();
    assert!(result.arguments.contains_key("test_key"));

    // Test after execute helper
    let response = ToolExecutionResponse {
        successful: false,
        data: serde_json::json!({}),
        error: None,
        log_id: "test".to_string(),
    };
    let result = apply_after_execute_modifiers(&modifiers, "TEST", "test", response).unwrap();
    assert!(result.successful);

    // Test schema helper
    let schema = ToolSchema {
        slug: "TEST".to_string(),
        name: "Test".to_string(),
        description: "Original".to_string(),
        toolkit: "test".to_string(),
        input_parameters: serde_json::json!({}),
        output_parameters: serde_json::json!({}),
        scopes: vec![],
        tags: vec![],
        version: "1.0.0".to_string(),
        available_versions: vec![],
        is_deprecated: false,
        no_auth: false,
    };
    let result = apply_schema_modifiers(&modifiers, "TEST", "test", schema).unwrap();
    assert_eq!(result.description, "Modified: Original");
}
