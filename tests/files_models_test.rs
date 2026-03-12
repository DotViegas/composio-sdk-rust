use composio_sdk::models::{
    FileCreatePresignedUrlParams, FileCreatePresignedUrlResponse, FileListParams, FileListResponse,
    FileStorageBackend,
};

#[test]
fn test_file_list_params_query_serialization_shape() {
    let params = FileListParams {
        cursor: Some("cursor_123".to_string()),
        limit: Some(25),
        tool_slug: Some("GMAIL_SEND_EMAIL".to_string()),
        toolkit_slug: Some("gmail".to_string()),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["cursor"], "cursor_123");
    assert_eq!(value["limit"], 25);
    assert_eq!(value["tool_slug"], "GMAIL_SEND_EMAIL");
    assert_eq!(value["toolkit_slug"], "gmail");
}

#[test]
fn test_file_upload_request_serialization() {
    let params = FileCreatePresignedUrlParams {
        filename: "invoice.pdf".to_string(),
        md5: "abc123".to_string(),
        mimetype: "application/pdf".to_string(),
        tool_slug: "GMAIL_SEND_EMAIL".to_string(),
        toolkit_slug: "gmail".to_string(),
    };

    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(value["filename"], "invoice.pdf");
    assert_eq!(value["md5"], "abc123");
    assert_eq!(value["mimetype"], "application/pdf");
    assert_eq!(value["tool_slug"], "GMAIL_SEND_EMAIL");
    assert_eq!(value["toolkit_slug"], "gmail");
}

#[test]
fn test_file_list_response_deserialization() {
    let payload = serde_json::json!({
        "current_page": 1,
        "total_items": 2,
        "total_pages": 1,
        "next_cursor": null,
        "items": [
            {
                "filename": "invoice.pdf",
                "md5": "md5-1",
                "mimetype": "application/pdf",
                "tool_slug": "GMAIL_SEND_EMAIL",
                "toolkit_slug": "gmail"
            },
            {
                "filename": "notes.txt",
                "md5": "md5-2",
                "mimetype": "text/plain",
                "tool_slug": "NOTION_APPEND_BLOCK",
                "toolkit_slug": "notion"
            }
        ]
    });

    let response: FileListResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.current_page, 1);
    assert_eq!(response.total_items, 2);
    assert_eq!(response.items.len(), 2);
    assert_eq!(response.items[0].filename, "invoice.pdf");
}

#[test]
fn test_file_upload_response_deserialization_with_alias_and_metadata() {
    let payload = serde_json::json!({
        "id": "file_123",
        "key": "uploads/file_123",
        "type": "file",
        "newPresignedUrl": "https://s3.example.com/upload",
        "metadata": {
            "storage_backend": "s3"
        }
    });

    let response: FileCreatePresignedUrlResponse = serde_json::from_value(payload).unwrap();
    assert_eq!(response.id, "file_123");
    assert_eq!(response.new_presigned_url, "https://s3.example.com/upload");
    assert_eq!(
        response.metadata.unwrap().storage_backend,
        FileStorageBackend::S3
    );
}
