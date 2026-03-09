//! Example demonstrating file upload and download functionality
//!
//! This example shows how to:
//! - Upload local files to S3
//! - Upload files from public URLs
//! - Download files from S3
//! - Process tool schemas with file parameters
//!
//! # Usage
//!
//! ```bash
//! export COMPOSIO_API_KEY="your-api-key"
//! cargo run --example file_handling
//! ```

use composio_sdk::{
    client::ComposioClient,
    models::files::{FileUploadable, FileDownloadable, FileHelper},
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let api_key = std::env::var("COMPOSIO_API_KEY")
        .expect("COMPOSIO_API_KEY environment variable not set");
    
    let client = ComposioClient::builder()
        .api_key(&api_key)
        .build()?;
    
    println!("=== Composio File Handling Example ===\n");
    
    // Example 1: Upload a local file
    println!("1. Uploading local file...");
    if Path::new("README.md").exists() {
        match FileUploadable::from_path(
            &client,
            Path::new("README.md"),
            "GMAIL_SEND_EMAIL",
            "gmail"
        ).await {
            Ok(uploadable) => {
                println!("   ✓ File uploaded successfully!");
                println!("   - Name: {}", uploadable.name);
                println!("   - MIME type: {}", uploadable.mimetype);
                println!("   - S3 key: {}", uploadable.s3key);
            }
            Err(e) => {
                println!("   ✗ Upload failed: {}", e);
            }
        }
    } else {
        println!("   ⚠ README.md not found, skipping local file upload");
    }
    
    println!();
    
    // Example 2: Upload from URL
    println!("2. Uploading file from URL...");
    let test_url = "https://raw.githubusercontent.com/rust-lang/rust/master/README.md";
    
    match FileUploadable::from_url(
        &client,
        test_url,
        "SLACK_SEND_MESSAGE",
        "slack"
    ).await {
        Ok(uploadable) => {
            println!("   ✓ File uploaded from URL successfully!");
            println!("   - Name: {}", uploadable.name);
            println!("   - MIME type: {}", uploadable.mimetype);
            println!("   - S3 key: {}", uploadable.s3key);
        }
        Err(e) => {
            println!("   ✗ Upload from URL failed: {}", e);
        }
    }
    
    println!();
    
    // Example 3: Download a file
    println!("3. Downloading file from S3...");
    let downloadable = FileDownloadable {
        name: "example.txt".to_string(),
        mimetype: "text/plain".to_string(),
        s3url: "https://example.com/file.txt".to_string(), // Replace with actual S3 URL
    };
    
    match downloadable.download(Path::new("/tmp/composio")).await {
        Ok(path) => {
            println!("   ✓ File downloaded successfully!");
            println!("   - Path: {}", path.display());
        }
        Err(e) => {
            println!("   ✗ Download failed: {}", e);
        }
    }
    
    println!();
    
    // Example 4: Process tool schema with file parameters
    println!("4. Processing tool schema with file parameters...");
    let file_helper = FileHelper::new(None);
    
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "attachment": {
                "type": "object",
                "file_uploadable": true,
                "description": "File to attach"
            },
            "message": {
                "type": "string",
                "description": "Message text"
            }
        },
        "required": ["message"]
    });
    
    println!("   Original schema:");
    println!("   {}", serde_json::to_string_pretty(&schema)?);
    
    let processed = file_helper.process_schema_recursively(schema);
    
    println!("\n   Processed schema:");
    println!("   {}", serde_json::to_string_pretty(&processed)?);
    
    println!("\n=== Example completed ===");
    
    Ok(())
}
