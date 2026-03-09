# Internal SDK Implementation

## Overview

This document describes the implementation of the `internal.py` Python module in Rust, which provides internal SDK functionality for realtime credentials and other internal operations.

## Python Source

**File:** `temp/composio/core/models/internal.py`

The Python implementation provides:
- `SDKRealtimeCredentialsResponse` - Response model for Pusher credentials
- `Internal` - Resource class for internal SDK operations
- `get_sdk_realtime_credentials()` - Method to retrieve realtime credentials

## Rust Implementation

**File:** `src/models/internal.rs`

### Key Components

#### 1. SDKRealtimeCredentialsResponse

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDKRealtimeCredentialsResponse {
    pub pusher_key: String,
    pub project_id: String,
    pub pusher_cluster: String,
}
```

Corresponds to the Python `SDKRealtimeCredentialsResponse` BaseModel.

#### 2. Internal Resource

```rust
#[derive(Clone)]
pub struct Internal {
    base: BaseResource,
}
```

Implements the `Resource` trait and provides access to internal SDK functionality.

#### 3. get_sdk_realtime_credentials()

```rust
pub async fn get_sdk_realtime_credentials(
    &self,
) -> Result<SDKRealtimeCredentialsResponse, ComposioError>
```

Async method that:
- Makes GET request to `/api/v3/internal/sdk/realtime/credentials`
- Includes automatic retry logic via `crate::retry::with_retry`
- Tracks telemetry (duration, errors)
- Returns Pusher credentials for WebSocket connections

### Implementation Details

#### HTTP Request Pattern

The implementation follows the standard pattern used throughout the SDK:

1. Build URL from base_url + endpoint
2. Execute request with retry logic using `crate::retry::with_retry`
3. Add authentication header (`x-api-key`)
4. Check response status
5. Parse JSON response
6. Track telemetry

#### Telemetry Integration

The method integrates with the SDK's telemetry system:
- Records method execution time
- Tracks success/failure metrics
- Logs errors with details
- Uses `EventType::Metric` for success, `EventType::Error` for failures

#### Error Handling

Proper error handling with `ComposioError`:
- Network errors (connection failures, timeouts)
- API errors (401 Unauthorized, 404 Not Found, etc.)
- JSON deserialization errors

### Testing

Three comprehensive tests:

1. **test_get_sdk_realtime_credentials_success**
   - Mocks successful API response
   - Verifies correct credential parsing
   - Validates all response fields

2. **test_get_sdk_realtime_credentials_unauthorized**
   - Mocks 401 Unauthorized response
   - Verifies error handling
   - Checks error type and status code

3. **test_sdk_realtime_credentials_response_deserialization**
   - Tests JSON deserialization
   - Validates struct field mapping

## Usage Example

```rust
use composio_sdk::client::ComposioClient;
use composio_sdk::models::internal::Internal;
use std::sync::Arc;

let client = Arc::new(
    ComposioClient::builder()
        .api_key("your-api-key")
        .build()?
);

let internal = Internal::new(client);
let credentials = internal.get_sdk_realtime_credentials().await?;

println!("Pusher Key: {}", credentials.pusher_key);
println!("Project ID: {}", credentials.project_id);
println!("Cluster: {}", credentials.pusher_cluster);
```

## Example File

**File:** `examples/internal_realtime.rs`

Demonstrates:
- Client initialization
- Internal resource creation
- Credential retrieval
- Error handling
- Output formatting

Run with:
```bash
export COMPOSIO_API_KEY="your-api-key"
cargo run --example internal_realtime
```

## Module Exports

Added to `src/models/mod.rs`:

```rust
pub mod internal;

// ...

/// Internal SDK resource
pub use internal::Internal;

/// SDK realtime credentials response
pub use internal::SDKRealtimeCredentialsResponse;
```

## Comparison with Python

| Feature | Python | Rust |
|---------|--------|------|
| Response Model | BaseModel (Pydantic) | Struct with Serialize/Deserialize |
| Resource Class | Inherits from Resource | Implements Resource trait |
| HTTP Client | self._client.get() | reqwest with retry logic |
| Error Handling | Exceptions | Result<T, ComposioError> |
| Async | Sync (blocking) | Async/await with Tokio |
| Telemetry | Not visible in code | Integrated with tracking |
| Type Safety | Runtime validation | Compile-time guarantees |

## Benefits of Rust Implementation

1. **Type Safety**: Compile-time guarantees for all data structures
2. **Performance**: Native async/await with zero-cost abstractions
3. **Error Handling**: Explicit Result types prevent silent failures
4. **Memory Safety**: No null pointer exceptions or memory leaks
5. **Telemetry**: Built-in tracking for observability
6. **Testing**: Comprehensive unit tests with mocking

## Future Enhancements

Potential additions to the Internal resource:

1. WebSocket connection helpers using Pusher credentials
2. Real-time event subscription management
3. Internal metrics and health checks
4. SDK version compatibility checks
5. Feature flag management

## Related Files

- `src/models/base.rs` - Base Resource trait and telemetry
- `src/client.rs` - ComposioClient HTTP methods
- `src/error.rs` - Error types and handling
- `src/retry.rs` - Retry logic with exponential backoff
- `src/models/telemetry.rs` - Telemetry data structures

## Status

✅ **Complete** - Fully implemented and tested

The `internal.py` module has been successfully translated to Rust with:
- Full feature parity
- Comprehensive tests (100% passing)
- Example usage
- Documentation
- Module exports
