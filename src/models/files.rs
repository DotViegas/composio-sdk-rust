//! File handling for Composio SDK
//!
//! This module provides functionality for uploading and downloading files
//! to/from S3, as well as processing tool schemas that involve file parameters.
//!
//! # Features
//!
//! - Upload files from local paths or public URLs
//! - Download files from S3 to local directory
//! - Transform tool schemas for file upload/download
//! - MD5 hash calculation for file integrity
//! - Security features: size limits, timeout handling, redirect protection
//!
//! # Examples
//!
//! ```rust,no_run
//! use composio_sdk::models::files::{FileUploadable, FileHelper};
//! use composio_sdk::client::ComposioClient;
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ComposioClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! // Upload a local file
//! let uploadable = FileUploadable::from_path(
//!     &client,
//!     Path::new("document.pdf"),
//!     "GMAIL_SEND_EMAIL",
//!     "gmail"
//! ).await?;
//!
//! println!("Uploaded to S3: {}", uploadable.s3key);
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ============================================================================
// Constants
// ============================================================================

/// Default chunk size for file operations (1 MB)
const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;

/// Maximum filename length to prevent issues with long URLs
const MAX_FILENAME_LENGTH: usize = 100;

/// Maximum response size when fetching files from URLs (100 MB)
const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024;

/// Connection timeout in seconds
const CONNECT_TIMEOUT_SECS: u64 = 5;

/// Read timeout in seconds
const READ_TIMEOUT_SECS: u64 = 60;

/// Local cache directory name
const LOCAL_CACHE_DIRECTORY_NAME: &str = ".composio";

/// Environment variable for cache directory
const ENV_LOCAL_CACHE_DIRECTORY: &str = "COMPOSIO_CACHE_DIR";

/// Local output file directory name
const LOCAL_OUTPUT_FILE_DIRECTORY: &str = "outputs";

// ============================================================================
// Helper Functions
// ============================================================================

/// Get the local cache directory path
pub fn get_local_cache_directory() -> PathBuf {
    if let Ok(cache_dir) = std::env::var(ENV_LOCAL_CACHE_DIRECTORY) {
        PathBuf::from(cache_dir)
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(LOCAL_CACHE_DIRECTORY_NAME)
    }
}

/// Get the local output file directory path
pub fn get_local_output_directory() -> PathBuf {
    get_local_cache_directory().join(LOCAL_OUTPUT_FILE_DIRECTORY)
}

/// Calculate MD5 hash of a file
///
/// Note: MD5 is used for file integrity checking and deduplication,
/// not for cryptographic security. The Composio API requires MD5 hashes.
pub fn calculate_md5(file_path: &Path) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path)?;
    let mut hasher = md5::Context::new();
    let mut buffer = vec![0; DEFAULT_CHUNK_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.consume(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.compute()))
}

/// Check if a string is a valid HTTP/HTTPS URL
pub fn is_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

// ============================================================================
// Data Structures
// ============================================================================

/// Query parameters for listing files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileListParams {
    /// Cursor token for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Page size limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Optional tool slug filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_slug: Option<String>,
    /// Optional toolkit slug filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_slug: Option<String>,
}

/// A file list item returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListItem {
    /// File name.
    pub filename: String,
    /// MD5 checksum.
    pub md5: String,
    /// MIME type.
    pub mimetype: String,
    /// Tool slug.
    pub tool_slug: String,
    /// Toolkit slug.
    pub toolkit_slug: String,
}

/// Paginated file listing response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListResponse {
    /// Current page number.
    pub current_page: u32,
    /// Number of total items.
    pub total_items: u32,
    /// Number of total pages.
    pub total_pages: u32,
    /// Cursor for next page, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Returned file items.
    pub items: Vec<FileListItem>,
}

/// Request payload for creating a file upload request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreatePresignedUrlParams {
    /// File name.
    pub filename: String,
    /// MD5 checksum.
    pub md5: String,
    /// MIME type.
    pub mimetype: String,
    /// Tool slug.
    pub tool_slug: String,
    /// Toolkit slug.
    pub toolkit_slug: String,
}

/// Storage backend used for uploaded file metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileStorageBackend {
    /// AWS S3 backend.
    S3,
    /// Azure Blob Storage backend.
    AzureBlobStorage,
}

/// Metadata included in presigned upload response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreatePresignedUrlMetadata {
    /// Backing storage provider.
    pub storage_backend: FileStorageBackend,
}

/// Response from file upload request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    /// File ID
    pub id: String,
    /// S3 key
    pub key: String,
    /// File type
    #[serde(rename = "type")]
    pub file_type: String,
    /// Presigned URL for upload
    #[serde(alias = "newPresignedUrl")]
    pub new_presigned_url: String,
    /// Additional metadata about the generated upload URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileCreatePresignedUrlMetadata>,
}

/// Alias for endpoint parity naming.
pub type FileCreatePresignedUrlResponse = FileUploadResponse;

/// File that can be uploaded to S3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadable {
    /// Filename
    pub name: String,
    /// MIME type
    pub mimetype: String,
    /// S3 key after upload
    pub s3key: String,
}

/// File that can be downloaded from S3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadable {
    /// Filename
    pub name: String,
    /// MIME type
    pub mimetype: String,
    /// S3 URL for download
    pub s3url: String,
}

impl FileUploadable {
    /// Create a FileUploadable from a local file path or public URL
    ///
    /// If the file parameter is a URL (starts with http:// or https://),
    /// it will fetch the file content from the URL and upload it to S3.
    /// Otherwise, it treats it as a local file path.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client for API calls
    /// * `file` - Local file path or public URL
    /// * `tool` - The tool slug
    /// * `toolkit` - The toolkit slug
    ///
    /// # Returns
    ///
    /// FileUploadable instance with S3 key
    ///
    /// # Errors
    ///
    /// Returns error if file doesn't exist, is not readable, or upload fails
    pub async fn from_path(
        client: &crate::client::ComposioClient,
        file: &Path,
        tool: &str,
        toolkit: &str,
    ) -> Result<Self, crate::error::ComposioError> {
        // Check if it's a URL
        if let Some(file_str) = file.to_str() {
            if is_url(file_str) {
                return Self::from_url(client, file_str, tool, toolkit).await;
            }
        }

        // Handle as local file path
        if !file.exists() {
            return Err(crate::error::ComposioError::FileNotFound(
                file.display().to_string(),
            ));
        }

        if !file.is_file() {
            return Err(crate::error::ComposioError::InvalidFile(format!(
                "Not a file: {}",
                file.display()
            )));
        }

        let filename = file.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
            crate::error::ComposioError::InvalidFile("Invalid filename".to_string())
        })?;

        let mimetype = crate::utils::mimetypes::guess_mime_type(file);

        let md5_hash = calculate_md5(file)?;

        // Request presigned URL from API
        let upload_response =
            Self::request_upload_url(client, &md5_hash, filename, &mimetype, tool, toolkit).await?;

        // Upload file to S3
        Self::upload_to_s3(&upload_response.new_presigned_url, file).await?;

        Ok(Self {
            name: filename.to_string(),
            mimetype,
            s3key: upload_response.key,
        })
    }

    /// Create a FileUploadable from a public URL
    pub async fn from_url(
        client: &crate::client::ComposioClient,
        url: &str,
        tool: &str,
        toolkit: &str,
    ) -> Result<Self, crate::error::ComposioError> {
        // Fetch file from URL
        let (filename, content, mimetype) = Self::fetch_from_url(url).await?;

        // Upload bytes to S3
        let s3key =
            Self::upload_bytes_to_s3(client, &filename, &content, &mimetype, tool, toolkit).await?;

        Ok(Self {
            name: filename,
            mimetype,
            s3key,
        })
    }

    /// Request presigned URL from Composio API
    async fn request_upload_url(
        client: &crate::client::ComposioClient,
        md5: &str,
        filename: &str,
        mimetype: &str,
        tool: &str,
        toolkit: &str,
    ) -> Result<FileUploadResponse, crate::error::ComposioError> {
        let params = FileCreatePresignedUrlParams {
            filename: filename.to_string(),
            md5: md5.to_string(),
            mimetype: mimetype.to_string(),
            tool_slug: tool.to_string(),
            toolkit_slug: toolkit.to_string(),
        };

        client.create_file_upload_request(params).await
    }

    /// Upload file to S3 using presigned URL
    async fn upload_to_s3(url: &str, file: &Path) -> Result<(), crate::error::ComposioError> {
        let file_content = tokio::fs::read(file).await?;

        let response = reqwest::Client::new()
            .put(url)
            .body(file_content)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(crate::error::ComposioError::UploadFailed(format!(
                "S3 upload failed with status: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Fetch file from URL with security protections
    async fn fetch_from_url(
        url: &str,
    ) -> Result<(String, Vec<u8>, String), crate::error::ComposioError> {
        use reqwest::redirect::Policy;

        let client = reqwest::Client::builder()
            .redirect(Policy::none()) // Disable redirects for security
            .connect_timeout(std::time::Duration::from_secs(CONNECT_TIMEOUT_SECS))
            .timeout(std::time::Duration::from_secs(READ_TIMEOUT_SECS))
            .build()?;

        let response = client.get(url).send().await?;

        // Reject redirects
        if response.status().is_redirection() {
            return Err(crate::error::ComposioError::UploadFailed(
                "URL returned redirect. Please provide a direct URL to the file.".to_string(),
            ));
        }

        if !response.status().is_success() {
            return Err(crate::error::ComposioError::UploadFailed(format!(
                "Failed to fetch file from URL. Status: {}",
                response.status()
            )));
        }

        // Check Content-Length header
        if let Some(content_length) = response.content_length() {
            if content_length as usize > MAX_RESPONSE_SIZE {
                return Err(crate::error::ComposioError::FileTooLarge(format!(
                    "File size ({} bytes) exceeds maximum ({} bytes)",
                    content_length, MAX_RESPONSE_SIZE
                )));
            }
        }

        // Extract mimetype before consuming response
        let mimetype = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|ct| crate::utils::mimetypes::extract_from_content_type(ct))
            .unwrap_or_else(|| crate::utils::mimetypes::DEFAULT_MIME_TYPE.to_string());

        // Download with size tracking
        let bytes = response.bytes().await?;
        if bytes.len() > MAX_RESPONSE_SIZE {
            return Err(crate::error::ComposioError::FileTooLarge(format!(
                "Response size exceeds maximum ({} bytes)",
                MAX_RESPONSE_SIZE
            )));
        }

        // Extract filename from URL
        let filename = Self::extract_filename_from_url(url, &mimetype);

        Ok((filename, bytes.to_vec(), mimetype))
    }

    /// Extract filename from URL or generate one
    fn extract_filename_from_url(url: &str, mimetype: &str) -> String {
        use url::Url;

        if let Ok(parsed) = Url::parse(url) {
            if let Some(segments) = parsed.path_segments() {
                if let Some(last) = segments.last() {
                    let decoded = urlencoding::decode(last).unwrap_or_default();
                    if !decoded.is_empty() {
                        return Self::truncate_filename(&decoded);
                    }
                }
            }
        }

        // Generate timestamped filename
        Self::generate_timestamped_filename(mimetype)
    }

    /// Truncate filename if too long
    fn truncate_filename(filename: &str) -> String {
        if filename.len() <= MAX_FILENAME_LENGTH {
            return filename.to_string();
        }

        // Extract extension
        let extension = if let Some(pos) = filename.rfind('.') {
            &filename[pos..]
        } else {
            ""
        };

        Self::generate_timestamped_filename(extension)
    }

    /// Generate a unique filename with timestamp
    fn generate_timestamped_filename(extension: &str) -> String {
        use chrono::Utc;
        use uuid::Uuid;

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let unique_id = &Uuid::new_v4().to_string()[..8];

        format!("file_{}_{}{}", timestamp, unique_id, extension)
    }

    /// Upload bytes to S3
    async fn upload_bytes_to_s3(
        client: &crate::client::ComposioClient,
        filename: &str,
        content: &[u8],
        mimetype: &str,
        tool: &str,
        toolkit: &str,
    ) -> Result<String, crate::error::ComposioError> {
        let md5_hash = format!("{:x}", md5::compute(content));

        let upload_response =
            Self::request_upload_url(client, &md5_hash, filename, mimetype, tool, toolkit).await?;

        // Upload to S3
        let response = reqwest::Client::new()
            .put(&upload_response.new_presigned_url)
            .header("Content-Type", mimetype)
            .body(content.to_vec())
            .send()
            .await?;

        if response.status() != 200 {
            return Err(crate::error::ComposioError::UploadFailed(format!(
                "S3 upload failed with status: {}",
                response.status()
            )));
        }

        Ok(upload_response.key)
    }
}

impl FileDownloadable {
    /// Download file from S3 to local directory
    ///
    /// # Arguments
    ///
    /// * `outdir` - Output directory path
    ///
    /// # Returns
    ///
    /// Path to the downloaded file
    pub async fn download(&self, outdir: &Path) -> Result<PathBuf, crate::error::ComposioError> {
        tokio::fs::create_dir_all(outdir).await?;

        let outfile = outdir.join(&self.name);

        let response = reqwest::get(&self.s3url).await?;

        if response.status() != 200 {
            return Err(crate::error::ComposioError::DownloadFailed(format!(
                "Failed to download file. Status: {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        tokio::fs::write(&outfile, bytes).await?;

        Ok(outfile)
    }
}

// ============================================================================
// FileHelper - Schema Processing and File Substitution
// ============================================================================

/// Helper for processing file uploads/downloads in tool schemas
pub struct FileHelper {
    _outdir: PathBuf,
}

impl FileHelper {
    /// Create a new FileHelper
    pub fn new(outdir: Option<PathBuf>) -> Self {
        let outdir = outdir.unwrap_or_else(get_local_output_directory);
        Self { _outdir: outdir }
    }

    /// Check if a schema has a specific file property
    fn has_file_property(&self, schema: &JsonValue, property_name: &str) -> bool {
        if let Some(obj) = schema.as_object() {
            // Direct property check
            if obj
                .get(property_name)
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                return true;
            }

            // Check anyOf variants
            if let Some(any_of) = obj.get("anyOf").and_then(|v| v.as_array()) {
                for variant in any_of {
                    if self.has_file_property(variant, property_name) {
                        return true;
                    }
                }
            }

            // Check oneOf variants
            if let Some(one_of) = obj.get("oneOf").and_then(|v| v.as_array()) {
                for variant in one_of {
                    if self.has_file_property(variant, property_name) {
                        return true;
                    }
                }
            }

            // Check allOf variants
            if let Some(all_of) = obj.get("allOf").and_then(|v| v.as_array()) {
                for variant in all_of {
                    if self.has_file_property(variant, property_name) {
                        return true;
                    }
                }
            }

            // Check nested properties
            if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
                for prop in properties.values() {
                    if self.has_file_property(prop, property_name) {
                        return true;
                    }
                }
            }

            // Check array items
            if let Some(items) = obj.get("items") {
                if self.has_file_property(items, property_name) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if schema has file_uploadable property
    pub fn is_file_uploadable(&self, schema: &JsonValue) -> bool {
        self.has_file_property(schema, "file_uploadable")
    }

    /// Check if schema has file_downloadable property
    pub fn is_file_downloadable(&self, schema: &JsonValue) -> bool {
        self.has_file_property(schema, "file_downloadable")
    }

    /// Transform schema for file upload (convert file_uploadable to path format)
    pub fn transform_schema_for_file_upload(&self, schema: JsonValue) -> JsonValue {
        if let Some(mut obj) = schema.as_object().cloned() {
            // Direct file_uploadable - transform it
            if obj
                .get("file_uploadable")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                return serde_json::json!({
                    "type": "string",
                    "format": "path",
                    "description": obj.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Path to file."),
                    "title": obj.get("title"),
                    "file_uploadable": true,
                });
            }

            // Transform anyOf variants
            if let Some(any_of) = obj.get("anyOf").and_then(|v| v.as_array()) {
                let transformed: Vec<JsonValue> = any_of
                    .iter()
                    .map(|v| self.transform_schema_for_file_upload(v.clone()))
                    .collect();
                obj.insert("anyOf".to_string(), JsonValue::Array(transformed));
            }

            // Transform oneOf variants
            if let Some(one_of) = obj.get("oneOf").and_then(|v| v.as_array()) {
                let transformed: Vec<JsonValue> = one_of
                    .iter()
                    .map(|v| self.transform_schema_for_file_upload(v.clone()))
                    .collect();
                obj.insert("oneOf".to_string(), JsonValue::Array(transformed));
            }

            // Transform allOf variants
            if let Some(all_of) = obj.get("allOf").and_then(|v| v.as_array()) {
                let transformed: Vec<JsonValue> = all_of
                    .iter()
                    .map(|v| self.transform_schema_for_file_upload(v.clone()))
                    .collect();
                obj.insert("allOf".to_string(), JsonValue::Array(transformed));
            }

            // Transform nested properties
            if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
                let transformed: HashMap<String, JsonValue> = properties
                    .iter()
                    .map(|(k, v)| (k.clone(), self.transform_schema_for_file_upload(v.clone())))
                    .collect();
                obj.insert(
                    "properties".to_string(),
                    serde_json::to_value(transformed).unwrap(),
                );
            }

            // Transform array items
            if let Some(items) = obj.get("items") {
                let transformed = self.transform_schema_for_file_upload(items.clone());
                obj.insert("items".to_string(), transformed);
            }

            return JsonValue::Object(obj);
        }

        schema
    }

    /// Process file_uploadable fields in schema
    pub fn process_file_uploadable_schema(&self, mut schema: JsonValue) -> JsonValue {
        if let Some(obj) = schema.as_object_mut() {
            if let Some(properties) = obj.get("properties").cloned() {
                let transformed = self.transform_schema_for_file_upload(properties);
                obj.insert("properties".to_string(), transformed);
            }
        }
        schema
    }

    /// Enhance schema descriptions with type hints and required notes
    pub fn enhance_schema_descriptions(&self, mut schema: JsonValue) -> JsonValue {
        if let Some(obj) = schema.as_object_mut() {
            let required = obj
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            if let Some(properties) = obj.get_mut("properties").and_then(|v| v.as_object_mut()) {
                for (param, prop_schema) in properties.iter_mut() {
                    if let Some(prop_obj) = prop_schema.as_object_mut() {
                        // Add type hint
                        if let Some(type_str) = prop_obj.get("type").and_then(|v| v.as_str()) {
                            if matches!(type_str, "string" | "integer" | "number" | "boolean") {
                                let desc = prop_obj
                                    .get("description")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .trim_end_matches('.');

                                let ext = format!("Please provide a value of type {}.", type_str);
                                let new_desc = if desc.is_empty() {
                                    ext
                                } else {
                                    format!("{}. {}", desc, ext)
                                };

                                prop_obj
                                    .insert("description".to_string(), JsonValue::String(new_desc));
                            }
                        }

                        // Add required note
                        if required.contains(param) {
                            let desc = prop_obj
                                .get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .trim_end_matches('.');

                            let new_desc = if desc.is_empty() {
                                "This parameter is required.".to_string()
                            } else {
                                format!("{}. This parameter is required.", desc)
                            };

                            prop_obj.insert("description".to_string(), JsonValue::String(new_desc));
                        }
                    }
                }
            }
        }
        schema
    }

    /// Process schema for both file handling and description enhancements
    pub fn process_schema_recursively(&self, schema: JsonValue) -> JsonValue {
        let schema = self.process_file_uploadable_schema(schema);
        self.enhance_schema_descriptions(schema)
    }
}
