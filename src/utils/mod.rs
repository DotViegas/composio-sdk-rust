//! Utility functions for the Composio SDK
//!
//! This module provides utility functions for common operations like
//! toolkit version management, file handling, logging, and more.

pub mod toolkit_version;
pub mod logging;
pub mod mimetypes;
pub mod openapi;
pub mod schema;

pub use toolkit_version::{
    get_toolkit_version, get_versions_from_env, merge_toolkit_versions,
};

pub use logging::{
    setup, setup_from_env, get_verbosity, set_verbosity, truncate_message,
    LogLevel, Verbosity, WithLogger,
    ENV_COMPOSIO_LOGGING_LEVEL, ENV_COMPOSIO_LOG_VERBOSITY,
};

pub use mimetypes::{
    guess_mime_type, guess_from_extension, is_valid_mime_type,
    normalize_mime_type, extract_from_content_type,
    is_text_mime_type, is_image_mime_type, is_video_mime_type, is_audio_mime_type,
    DEFAULT_MIME_TYPE,
};

pub use openapi::{
    OpenApiType, OpenApiSchema, CompositeType, ParameterDefinition,
    extract_parameters, merge_schemas,
};

pub use schema::{
    JsonSchemaType, make_safe_field_name, is_rust_keyword, is_reserved_field_name,
    substitute_reserved_keywords, reinstate_reserved_keywords,
    coerce_default_value, generate_request_id, generate_uuid, generate_short_id,
};
