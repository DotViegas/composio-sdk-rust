//! Example demonstrating MIME type detection and validation
//!
//! This example shows how to use the mimetypes utility module to:
//! - Detect MIME types from file paths
//! - Guess MIME types from extensions
//! - Validate MIME type strings
//! - Normalize and extract MIME types
//! - Check MIME type categories (text, image, video, audio)

use composio_sdk::utils::mimetypes;
use std::path::Path;

fn main() {
    println!("=== MIME Type Detection Examples ===\n");

    // 1. Detect MIME type from file paths
    println!("1. Detecting MIME types from file paths:");
    let files = vec![
        "document.pdf",
        "photo.jpg",
        "data.json",
        "video.mp4",
        "audio.mp3",
        "archive.zip",
        "unknown.xyz123",
    ];

    for file in files {
        let mime = mimetypes::guess_mime_type(Path::new(file));
        println!("   {} → {}", file, mime);
    }

    // 2. Detect MIME type from extensions
    println!("\n2. Detecting MIME types from extensions:");
    let extensions = vec![
        "pdf", ".pdf", "json", "webp", "wasm", "heic", "opus",
    ];

    for ext in extensions {
        let mime = mimetypes::guess_from_extension(ext);
        println!("   {} → {}", ext, mime);
    }

    // 3. Validate MIME type strings
    println!("\n3. Validating MIME type strings:");
    let mime_types = vec![
        "image/png",
        "application/json",
        "text/plain",
        "invalid",
        "",
        "image/",
    ];

    for mime in mime_types {
        let is_valid = mimetypes::is_valid_mime_type(mime);
        println!("   '{}' → {}", mime, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }

    // 4. Normalize MIME types
    println!("\n4. Normalizing MIME types:");
    let mime_types = vec![
        "IMAGE/PNG",
        "text/html; charset=utf-8",
        "application/json",
        "invalid",
    ];

    for mime in mime_types {
        let normalized = mimetypes::normalize_mime_type(mime);
        println!("   '{}' → '{}'", mime, normalized);
    }

    // 5. Extract MIME type from Content-Type headers
    println!("\n5. Extracting MIME types from Content-Type headers:");
    let headers = vec![
        "text/html; charset=utf-8",
        "application/json",
        "multipart/form-data; boundary=----WebKitFormBoundary",
    ];

    for header in headers {
        let mime = mimetypes::extract_from_content_type(header);
        println!("   '{}' → '{}'", header, mime);
    }

    // 6. Check MIME type categories
    println!("\n6. Checking MIME type categories:");
    
    println!("\n   Text-based MIME types:");
    let text_types = vec![
        "text/plain",
        "text/html",
        "application/json",
        "application/xml",
        "image/png",
    ];
    for mime in text_types {
        let is_text = mimetypes::is_text_mime_type(mime);
        println!("      {} → {}", mime, if is_text { "✓ Text" } else { "✗ Not text" });
    }

    println!("\n   Image MIME types:");
    let image_types = vec![
        "image/png",
        "image/jpeg",
        "image/gif",
        "text/plain",
    ];
    for mime in image_types {
        let is_image = mimetypes::is_image_mime_type(mime);
        println!("      {} → {}", mime, if is_image { "✓ Image" } else { "✗ Not image" });
    }

    println!("\n   Video MIME types:");
    let video_types = vec![
        "video/mp4",
        "video/webm",
        "image/png",
    ];
    for mime in video_types {
        let is_video = mimetypes::is_video_mime_type(mime);
        println!("      {} → {}", mime, if is_video { "✓ Video" } else { "✗ Not video" });
    }

    println!("\n   Audio MIME types:");
    let audio_types = vec![
        "audio/mpeg",
        "audio/wav",
        "video/mp4",
    ];
    for mime in audio_types {
        let is_audio = mimetypes::is_audio_mime_type(mime);
        println!("      {} → {}", mime, if is_audio { "✓ Audio" } else { "✗ Not audio" });
    }

    // 7. Modern file formats
    println!("\n7. Modern file format support:");
    let modern_formats = vec![
        ("webp", "Modern image format"),
        ("wasm", "WebAssembly binary"),
        ("opus", "Modern audio codec"),
        ("webm", "Modern video format"),
        ("heic", "HEIF image format"),
        ("avif", "AV1 image format"),
    ];

    for (ext, description) in modern_formats {
        let mime = mimetypes::guess_from_extension(ext);
        println!("   .{} ({}) → {}", ext, description, mime);
    }

    // 8. Default fallback behavior
    println!("\n8. Default fallback for unknown types:");
    let unknown = vec!["unknown", "xyz123", "abc"];
    for ext in unknown {
        let mime = mimetypes::guess_from_extension(ext);
        println!("   .{} → {} (default)", ext, mime);
    }

    println!("\n=== Summary ===");
    println!("✓ MIME type detection works for 1000+ file extensions");
    println!("✓ Validation ensures MIME type strings are properly formatted");
    println!("✓ Normalization handles case-insensitive and parameterized types");
    println!("✓ Category checking helps with content-type specific logic");
    println!("✓ Default fallback ({}) for unknown types", mimetypes::DEFAULT_MIME_TYPE);
}
