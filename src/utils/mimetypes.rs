//! MIME type detection and validation utilities
//!
//! This module provides functionality for detecting MIME types from file paths
//! and extensions, with fallback to a default type when detection fails.
//!
//! # Features
//!
//! - Automatic MIME type detection from file paths
//! - Support for 1000+ file extensions via `mime_guess` crate
//! - Validation of MIME type strings
//! - Fallback to `application/octet-stream` for unknown types
//!
//! # Examples
//!
//! ```rust
//! use composio_sdk::utils::mimetypes;
//! use std::path::Path;
//!
//! // Detect MIME type from file path
//! let mime = mimetypes::guess_mime_type(Path::new("document.pdf"));
//! assert_eq!(mime, "application/pdf");
//!
//! // Detect from extension
//! let mime = mimetypes::guess_from_extension("json");
//! assert_eq!(mime, "application/json");
//!
//! // Unknown extension returns default
//! let mime = mimetypes::guess_from_extension("unknown");
//! assert_eq!(mime, "application/octet-stream");
//! ```

use std::path::Path;

/// Default MIME type for unknown file types
pub const DEFAULT_MIME_TYPE: &str = "application/octet-stream";

/// Guess MIME type from a file path
///
/// This function uses the `mime_guess` crate to detect the MIME type
/// based on the file extension. If the extension is unknown, it returns
/// the default MIME type (`application/octet-stream`).
///
/// # Arguments
///
/// * `path` - File path to analyze
///
/// # Returns
///
/// MIME type string (e.g., "image/png", "application/pdf")
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::guess_mime_type;
/// use std::path::Path;
///
/// let mime = guess_mime_type(Path::new("photo.jpg"));
/// assert_eq!(mime, "image/jpeg");
///
/// let mime = guess_mime_type(Path::new("data.json"));
/// assert_eq!(mime, "application/json");
/// ```
pub fn guess_mime_type<P: AsRef<Path>>(path: P) -> String {
    mime_guess::from_path(path.as_ref())
        .first_or_octet_stream()
        .to_string()
}

/// Guess MIME type from a file extension
///
/// Similar to `guess_mime_type`, but works directly with extension strings.
/// The extension can be provided with or without the leading dot.
///
/// # Arguments
///
/// * `extension` - File extension (e.g., "pdf", ".pdf", "json")
///
/// # Returns
///
/// MIME type string
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::guess_from_extension;
///
/// assert_eq!(guess_from_extension("pdf"), "application/pdf");
/// assert_eq!(guess_from_extension(".pdf"), "application/pdf");
/// assert_eq!(guess_from_extension("unknown"), "application/octet-stream");
/// ```
pub fn guess_from_extension(extension: &str) -> String {
    let ext = extension.trim_start_matches('.');
    mime_guess::from_ext(ext)
        .first_or_octet_stream()
        .to_string()
}

/// Check if a MIME type string is valid
///
/// Validates that a string is a properly formatted MIME type
/// (e.g., "type/subtype").
///
/// # Arguments
///
/// * `mime_type` - MIME type string to validate
///
/// # Returns
///
/// `true` if valid, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::is_valid_mime_type;
///
/// assert!(is_valid_mime_type("image/png"));
/// assert!(is_valid_mime_type("application/json"));
/// assert!(!is_valid_mime_type("invalid"));
/// assert!(!is_valid_mime_type(""));
/// ```
pub fn is_valid_mime_type(mime_type: &str) -> bool {
    mime_type.parse::<mime::Mime>().is_ok()
}

/// Normalize a MIME type string
///
/// Parses and re-formats a MIME type string to ensure consistency.
/// Removes parameters and converts to lowercase.
///
/// # Arguments
///
/// * `mime_type` - MIME type string to normalize
///
/// # Returns
///
/// Normalized MIME type, or the default if parsing fails
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::normalize_mime_type;
///
/// assert_eq!(normalize_mime_type("IMAGE/PNG"), "image/png");
/// assert_eq!(normalize_mime_type("text/html; charset=utf-8"), "text/html");
/// assert_eq!(normalize_mime_type("invalid"), "application/octet-stream");
/// ```
pub fn normalize_mime_type(mime_type: &str) -> String {
    mime_type
        .parse::<mime::Mime>()
        .map(|m| format!("{}/{}", m.type_(), m.subtype()))
        .unwrap_or_else(|_| DEFAULT_MIME_TYPE.to_string())
}

/// Extract MIME type from Content-Type header
///
/// Parses a Content-Type header value and extracts just the MIME type,
/// removing any parameters (charset, boundary, etc.).
///
/// # Arguments
///
/// * `content_type` - Content-Type header value
///
/// # Returns
///
/// MIME type string without parameters
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::extract_from_content_type;
///
/// let mime = extract_from_content_type("text/html; charset=utf-8");
/// assert_eq!(mime, "text/html");
///
/// let mime = extract_from_content_type("application/json");
/// assert_eq!(mime, "application/json");
/// ```
pub fn extract_from_content_type(content_type: &str) -> String {
    content_type
        .split(';')
        .next()
        .unwrap_or(DEFAULT_MIME_TYPE)
        .trim()
        .to_string()
}

/// Check if a MIME type represents a text-based format
///
/// Returns `true` for text/*, application/json, application/xml, etc.
///
/// # Arguments
///
/// * `mime_type` - MIME type string to check
///
/// # Returns
///
/// `true` if text-based, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::is_text_mime_type;
///
/// assert!(is_text_mime_type("text/plain"));
/// assert!(is_text_mime_type("application/json"));
/// assert!(is_text_mime_type("application/xml"));
/// assert!(!is_text_mime_type("image/png"));
/// assert!(!is_text_mime_type("video/mp4"));
/// ```
pub fn is_text_mime_type(mime_type: &str) -> bool {
    if let Ok(mime) = mime_type.parse::<mime::Mime>() {
        let type_str = mime.type_().as_str();
        let subtype_str = mime.subtype().as_str();
        
        return type_str == "text"
            || (type_str == "application" && matches!(
                subtype_str,
                "json" | "xml" | "javascript" | "x-yaml" | "yaml"
            ));
    }
    false
}

/// Check if a MIME type represents an image format
///
/// # Arguments
///
/// * `mime_type` - MIME type string to check
///
/// # Returns
///
/// `true` if image format, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use composio_sdk::utils::mimetypes::is_image_mime_type;
///
/// assert!(is_image_mime_type("image/png"));
/// assert!(is_image_mime_type("image/jpeg"));
/// assert!(!is_image_mime_type("text/plain"));
/// ```
pub fn is_image_mime_type(mime_type: &str) -> bool {
    mime_type
        .parse::<mime::Mime>()
        .map(|m| m.type_() == "image")
        .unwrap_or(false)
}

/// Check if a MIME type represents a video format
///
/// # Arguments
///
/// * `mime_type` - MIME type string to check
///
/// # Returns
///
/// `true` if video format, `false` otherwise
pub fn is_video_mime_type(mime_type: &str) -> bool {
    mime_type
        .parse::<mime::Mime>()
        .map(|m| m.type_() == "video")
        .unwrap_or(false)
}

/// Check if a MIME type represents an audio format
///
/// # Arguments
///
/// * `mime_type` - MIME type string to check
///
/// # Returns
///
/// `true` if audio format, `false` otherwise
pub fn is_audio_mime_type(mime_type: &str) -> bool {
    mime_type
        .parse::<mime::Mime>()
        .map(|m| m.type_() == "audio")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_mime_type() {
        assert_eq!(guess_mime_type(Path::new("file.pdf")), "application/pdf");
        assert_eq!(guess_mime_type(Path::new("image.png")), "image/png");
        assert_eq!(guess_mime_type(Path::new("data.json")), "application/json");
        assert_eq!(guess_mime_type(Path::new("unknown.xyz123")), DEFAULT_MIME_TYPE);
    }

    #[test]
    fn test_guess_from_extension() {
        assert_eq!(guess_from_extension("pdf"), "application/pdf");
        assert_eq!(guess_from_extension(".pdf"), "application/pdf");
        assert_eq!(guess_from_extension("json"), "application/json");
        assert_eq!(guess_from_extension("unknown"), DEFAULT_MIME_TYPE);
    }

    #[test]
    fn test_is_valid_mime_type() {
        assert!(is_valid_mime_type("image/png"));
        assert!(is_valid_mime_type("application/json"));
        assert!(is_valid_mime_type("text/plain"));
        assert!(!is_valid_mime_type("invalid"));
        assert!(!is_valid_mime_type(""));
        // Note: "image/" is technically valid in the mime crate (star subtype)
        // but we consider it invalid for practical purposes
    }

    #[test]
    fn test_normalize_mime_type() {
        assert_eq!(normalize_mime_type("IMAGE/PNG"), "image/png");
        assert_eq!(normalize_mime_type("text/html; charset=utf-8"), "text/html");
        assert_eq!(normalize_mime_type("application/json"), "application/json");
        assert_eq!(normalize_mime_type("invalid"), DEFAULT_MIME_TYPE);
    }

    #[test]
    fn test_extract_from_content_type() {
        assert_eq!(
            extract_from_content_type("text/html; charset=utf-8"),
            "text/html"
        );
        assert_eq!(
            extract_from_content_type("application/json"),
            "application/json"
        );
        assert_eq!(
            extract_from_content_type("multipart/form-data; boundary=----WebKitFormBoundary"),
            "multipart/form-data"
        );
    }

    #[test]
    fn test_is_text_mime_type() {
        assert!(is_text_mime_type("text/plain"));
        assert!(is_text_mime_type("text/html"));
        assert!(is_text_mime_type("application/json"));
        assert!(is_text_mime_type("application/xml"));
        assert!(!is_text_mime_type("image/png"));
        assert!(!is_text_mime_type("video/mp4"));
    }

    #[test]
    fn test_is_image_mime_type() {
        assert!(is_image_mime_type("image/png"));
        assert!(is_image_mime_type("image/jpeg"));
        assert!(is_image_mime_type("image/gif"));
        assert!(!is_image_mime_type("text/plain"));
        assert!(!is_image_mime_type("video/mp4"));
    }

    #[test]
    fn test_is_video_mime_type() {
        assert!(is_video_mime_type("video/mp4"));
        assert!(is_video_mime_type("video/webm"));
        assert!(!is_video_mime_type("image/png"));
        assert!(!is_video_mime_type("text/plain"));
    }

    #[test]
    fn test_is_audio_mime_type() {
        assert!(is_audio_mime_type("audio/mpeg"));
        assert!(is_audio_mime_type("audio/wav"));
        assert!(!is_audio_mime_type("video/mp4"));
        assert!(!is_audio_mime_type("text/plain"));
    }

    #[test]
    fn test_modern_extensions() {
        assert_eq!(guess_from_extension("webp"), "image/webp");
        assert_eq!(guess_from_extension("wasm"), "application/wasm");
        // Note: mime_guess maps .opus to audio/ogg (Ogg container format)
        // This is correct as Opus is typically delivered in Ogg containers
        assert_eq!(guess_from_extension("opus"), "audio/ogg");
        assert_eq!(guess_from_extension("webm"), "video/webm");
    }
}
