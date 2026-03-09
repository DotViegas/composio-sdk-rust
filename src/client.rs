//! HTTP client for Composio API
//!
//! This module provides the main HTTP client for interacting with the Composio API.
//! It uses the builder pattern for flexible configuration and includes automatic
//! retry logic for transient failures.
//!
//! # Example
//!
//! ```no_run
//! use composio_sdk::client::ComposioClient;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ComposioClient::builder()
//!     .api_key("your_api_key")
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(5)
//!     .build()?;
//! # Ok(())
//! # }
//! ```

use crate::config::ComposioConfig;
use crate::error::ComposioError;
use crate::retry::RetryPolicy;
use serde::Deserialize;
use std::time::Duration;

/// Main client for interacting with Composio API
///
/// The client manages HTTP connections and configuration for all API requests.
/// It includes automatic retry logic for transient failures and proper error handling.
///
/// # Example
///
/// ```no_run
/// use composio_sdk::client::ComposioClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ComposioClient::builder()
///     .api_key("your_api_key")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ComposioClient {
    http_client: reqwest::Client,
    config: ComposioConfig,
}

/// Builder for ComposioClient
///
/// Provides a fluent API for configuring the Composio client with custom settings.
/// All configuration options are optional and will use sensible defaults if not specified.
///
/// # Example
///
/// ```no_run
/// use composio_sdk::client::ComposioClient;
/// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
/// use std::time::Duration;
/// use std::collections::HashMap;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut versions = HashMap::new();
/// versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
///
/// let client = ComposioClient::builder()
///     .api_key("your_api_key")
///     .base_url("https://custom.api.com")
///     .timeout(Duration::from_secs(60))
///     .max_retries(5)
///     .initial_retry_delay(Duration::from_secs(2))
///     .max_retry_delay(Duration::from_secs(30))
///     .toolkit_versions(ToolkitVersionParam::Versions(versions))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct ComposioClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    initial_retry_delay: Option<Duration>,
    max_retry_delay: Option<Duration>,
    toolkit_versions: Option<crate::models::versioning::ToolkitVersionParam>,
    file_download_dir: Option<std::path::PathBuf>,
    auto_upload_download_files: Option<bool>,
    telemetry_enabled: Option<bool>,
}

impl ComposioClient {
    /// Create a new client builder
    ///
    /// Returns a `ComposioClientBuilder` that can be used to configure and build
    /// a `ComposioClient` instance.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> ComposioClientBuilder {
        ComposioClientBuilder::default()
    }

    /// Get a reference to the HTTP client
    ///
    /// This is useful for advanced use cases where you need direct access to the
    /// underlying reqwest client.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Get a reference to the configuration
    ///
    /// Returns the configuration used by this client.
    pub fn config(&self) -> &ComposioConfig {
        &self.config
    }

    /// Create a new session for a user
    ///
    /// Returns a `SessionBuilder` that can be used to configure and create
    /// a Tool Router session for the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User identifier for session isolation
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let session = client
    ///     .create_session("user_123")
    ///     .toolkits(vec!["github", "gmail"])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_session(&self, user_id: impl Into<String>) -> crate::session::SessionBuilder<'_> {
        crate::session::SessionBuilder::new(self, user_id.into())
    }

    /// Get an existing session by ID
    ///
    /// Retrieves session details for a previously created Tool Router session.
    /// This is useful for inspecting session configuration and available tools.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session ID to retrieve
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Session not found (404)
    /// - Network error occurs
    /// - API returns an error response
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let session = client.get_session("sess_abc123").await?;
    /// println!("Session ID: {}", session.session_id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_session(
        &self,
        session_id: impl Into<String>,
    ) -> Result<crate::session::Session, ComposioError> {
        let session_id = session_id.into();
        let url = format!(
            "{}/tool_router/session/{}",
            self.config.base_url, session_id
        );

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        let session_response: crate::models::SessionResponse = response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?;

        // Convert to Session
        Ok(crate::session::Session::from_response(
            self.clone(),
            session_response,
        ))
    }

    /// List connected accounts
    ///
    /// Retrieves a list of connected accounts based on the provided filters.
    ///
    /// # Arguments
    ///
    /// * `params` - Filter parameters for the query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::connected_accounts::ConnectedAccountListParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = ConnectedAccountListParams {
    ///     user_ids: Some(vec!["user_123".to_string()]),
    ///     toolkit_slugs: Some(vec!["github".to_string()]),
    ///     ..Default::default()
    /// };
    ///
    /// let accounts = client.list_connected_accounts(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_connected_accounts(
        &self,
        params: crate::models::connected_accounts::ConnectedAccountListParams,
    ) -> Result<crate::models::connected_accounts::ConnectedAccountListResponse, ComposioError> {
        let mut url = format!("{}/api/v3/connected_accounts", self.config.base_url);
        
        // Build query parameters
        let mut query_params = vec![];
        
        if let Some(user_ids) = &params.user_ids {
            query_params.push(format!("user_ids={}", user_ids.join(",")));
        }
        if let Some(auth_config_ids) = &params.auth_config_ids {
            query_params.push(format!("auth_config_ids={}", auth_config_ids.join(",")));
        }
        if let Some(toolkit_slugs) = &params.toolkit_slugs {
            query_params.push(format!("toolkit_slugs={}", toolkit_slugs.join(",")));
        }
        if let Some(connected_account_ids) = &params.connected_account_ids {
            query_params.push(format!("connected_account_ids={}", connected_account_ids.join(",")));
        }
        if let Some(statuses) = &params.statuses {
            let status_strings: Vec<String> = statuses.iter()
                .map(|s| serde_json::to_string(s).unwrap_or_default().trim_matches('"').to_string())
                .collect();
            query_params.push(format!("statuses={}", status_strings.join(",")));
        }
        if let Some(show_disabled) = params.show_disabled {
            query_params.push(format!("show_disabled={}", show_disabled));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &params.cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        if let Some(order_by) = &params.order_by {
            query_params.push(format!("order_by={}", order_by));
        }
        if let Some(order_direction) = &params.order_direction {
            query_params.push(format!("order_direction={}", order_direction));
        }
        
        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Get a specific connected account by ID
    ///
    /// # Arguments
    ///
    /// * `account_id` - The connected account ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let account = client.get_connected_account("ca_abc123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_connected_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<crate::models::connected_accounts::ConnectedAccountInfo, ComposioError> {
        let account_id = account_id.into();
        let url = format!("{}/api/v3/connected_accounts/{}", self.config.base_url, account_id);

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    // ========================================================================
    // Toolkits Methods
    // ========================================================================

    /// List all toolkits
    ///
    /// Retrieves a list of available toolkits based on the provided filters.
    /// Toolkits are collections of tools that can be used to perform various tasks.
    ///
    /// # Arguments
    ///
    /// * `params` - Filter parameters for the query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::toolkits::ToolkitListParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = ToolkitListParams {
    ///     category: Some("communication".to_string()),
    ///     limit: Some(20),
    ///     ..Default::default()
    /// };
    ///
    /// let toolkits = client.list_toolkits(params).await?;
    /// for toolkit in toolkits.items {
    ///     println!("Toolkit: {} ({})", toolkit.name, toolkit.slug);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_toolkits(
        &self,
        params: crate::models::toolkits::ToolkitListParams,
    ) -> Result<crate::models::toolkits::ToolkitListResponse, ComposioError> {
        let mut url = format!("{}/api/v3/toolkits", self.config.base_url);
        
        // Build query parameters
        let mut query_params = vec![];
        
        if let Some(category) = &params.category {
            query_params.push(format!("category={}", category));
        }
        if let Some(cursor) = &params.cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(sort_by) = &params.sort_by {
            let sort_str = match sort_by {
                crate::models::toolkits::SortBy::Usage => "usage",
                crate::models::toolkits::SortBy::Alphabetically => "alphabetically",
            };
            query_params.push(format!("sort_by={}", sort_str));
        }
        if let Some(managed_by) = &params.managed_by {
            let managed_str = match managed_by {
                crate::models::toolkits::ManagedBy::Composio => "composio",
                crate::models::toolkits::ManagedBy::All => "all",
                crate::models::toolkits::ManagedBy::Project => "project",
            };
            query_params.push(format!("managed_by={}", managed_str));
        }
        if let Some(search) = &params.search {
            query_params.push(format!("search={}", search));
        }
        if let Some(show_deprecated) = params.show_deprecated {
            query_params.push(format!("show_deprecated={}", show_deprecated));
        }
        
        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Get a specific toolkit by slug
    ///
    /// Retrieves detailed information about a specific toolkit including
    /// authentication schemes, available tools, and configuration details.
    ///
    /// # Arguments
    ///
    /// * `slug` - The toolkit slug (e.g., "github", "gmail", "slack")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let toolkit = client.get_toolkit("github").await?;
    /// println!("Toolkit: {}", toolkit.name);
    /// println!("Auth schemes: {:?}", toolkit.auth_schemes);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_toolkit(
        &self,
        slug: impl Into<String>,
    ) -> Result<crate::models::toolkits::ToolkitRetrieveResponse, ComposioError> {
        let slug = slug.into();
        let url = format!("{}/api/v3/toolkits/{}", self.config.base_url, slug);

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// List all toolkit categories
    ///
    /// Retrieves a list of all available toolkit categories.
    /// Categories help organize toolkits by functionality or industry.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let categories = client.list_toolkit_categories().await?;
    /// for category in categories.items {
    ///     println!("Category: {}", category.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_toolkit_categories(
        &self,
    ) -> Result<crate::models::toolkits::ToolkitCategoriesResponse, ComposioError> {
        let url = format!("{}/api/v3/toolkits/categories", self.config.base_url);

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Authorize a user to a toolkit
    ///
    /// Creates an authentication link for a user to connect to a specific toolkit.
    /// If an auth config is not found, it will be created using Composio managed auth.
    ///
    /// This is a convenience method that:
    /// 1. Gets or creates an auth config for the toolkit
    /// 2. Initiates a connection for the user
    /// 3. Returns the connection request with redirect URL
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to authorize
    /// * `toolkit` - The slug of the toolkit to authorize (e.g., "github", "gmail")
    ///
    /// # Returns
    ///
    /// Returns a connection request with a `redirect_url` that the user should visit
    /// to complete the authentication flow.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let connection = client.authorize_toolkit("user_123", "github").await?;
    /// println!("Visit this URL to authenticate: {}", connection.redirect_url.unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authorize_toolkit(
        &self,
        user_id: impl Into<String>,
        toolkit: impl Into<String>,
    ) -> Result<crate::models::connected_accounts::ConnectionRequest, ComposioError> {
        let user_id = user_id.into();
        let toolkit = toolkit.into();
        
        // Get or create auth config
        let auth_config_id = self.get_or_create_auth_config(&toolkit).await?;
        
        // Initiate connection
        self.initiate_connection(user_id, auth_config_id, None).await
    }

    /// Get or create an auth config for a toolkit (internal helper)
    ///
    /// This method checks if an auth config exists for the toolkit.
    /// If found, returns the most recent one. If not found, creates a new one
    /// using Composio managed auth.
    async fn get_or_create_auth_config(
        &self,
        toolkit: &str,
    ) -> Result<String, ComposioError> {
        use crate::models::auth_configs::{AuthConfigListParams, AuthConfigCreateParams, AuthConfigOptions};
        
        // List existing auth configs for this toolkit
        let params = AuthConfigListParams {
            toolkit_slug: Some(toolkit.to_string()),
            ..Default::default()
        };
        
        let auth_configs = self.list_auth_configs(params).await?;
        
        // If we have existing configs, return the most recent one
        if !auth_configs.items.is_empty() {
            let mut configs = auth_configs.items;
            configs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            return Ok(configs[0].id.clone());
        }
        
        // Create new auth config using Composio managed auth
        let create_request = AuthConfigCreateParams {
            toolkit: toolkit.to_string(),
            options: AuthConfigOptions::Default {
                scopes: None,
                user_scopes: None,
                restrict_to_following_tools: Some(vec![]),
            },
        };
        
        let created = self.create_auth_config(create_request).await?;
        Ok(created.auth_config.id)
    }

    /// Initiate a connection for a user (internal helper)
    ///
    /// Creates a new connected account initiation request.
    async fn initiate_connection(
        &self,
        user_id: String,
        auth_config_id: String,
        callback_url: Option<String>,
    ) -> Result<crate::models::connected_accounts::ConnectionRequest, ComposioError> {
        use crate::models::connected_accounts::InitiateConnectionParams;
        
        let url = format!("{}/api/v3/connected_accounts", self.config.base_url);
        
        let request_body = InitiateConnectionParams {
            user_id,
            auth_config_id,
            callback_url,
            allow_multiple: None,
            config: None,
        };

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&request_body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response - API returns connection info with redirect_url
        #[derive(Deserialize)]
        struct ConnectionResponse {
            id: String,
            status: Option<crate::models::connected_accounts::ConnectionStatus>,
            redirect_url: Option<String>,
        }
        
        let conn_response: ConnectionResponse = response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?;
        
        Ok(crate::models::connected_accounts::ConnectionRequest::new(
            conn_response.id,
            conn_response.status.unwrap_or(crate::models::connected_accounts::ConnectionStatus::Initiated),
            conn_response.redirect_url,
        ))
    }

    /// List auth configs (internal helper)
    async fn list_auth_configs(
        &self,
        params: crate::models::auth_configs::AuthConfigListParams,
    ) -> Result<crate::models::auth_configs::AuthConfigListResponse, ComposioError> {
        let mut url = format!("{}/api/v3/auth_configs", self.config.base_url);
        
        // Build query parameters
        let mut query_params = vec![];
        
        if let Some(toolkit_slug) = &params.toolkit_slug {
            query_params.push(format!("toolkit_slug={}", toolkit_slug));
        }
        if let Some(is_composio_managed) = params.is_composio_managed {
            query_params.push(format!("is_composio_managed={}", is_composio_managed));
        }
        if let Some(show_disabled) = params.show_disabled {
            query_params.push(format!("show_disabled={}", show_disabled));
        }
        if let Some(search) = &params.search {
            query_params.push(format!("search={}", search));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &params.cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        
        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Create auth config (internal helper)
    async fn create_auth_config(
        &self,
        request: crate::models::auth_configs::AuthConfigCreateParams,
    ) -> Result<crate::models::auth_configs::AuthConfigCreateResponse, ComposioError> {
        let url = format!("{}/api/v3/auth_configs", self.config.base_url);

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&request)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Get connected account initiation fields for a toolkit
    ///
    /// Retrieves the required and optional fields needed to initiate a connection
    /// for a specific toolkit and authentication scheme.
    ///
    /// # Arguments
    ///
    /// * `toolkit` - The toolkit slug (e.g., "github", "gmail")
    /// * `auth_scheme` - The authentication scheme (e.g., "OAUTH2", "API_KEY")
    /// * `required_only` - If true, returns only required fields; if false, returns both required and optional
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let fields = client.get_connected_account_initiation_fields(
    ///     "github",
    ///     "OAUTH2",
    ///     false
    /// ).await?;
    ///
    /// for field in fields {
    ///     println!("Field: {} (required: {})", field.name, field.required);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_connected_account_initiation_fields(
        &self,
        toolkit: impl Into<String>,
        auth_scheme: impl Into<String>,
        required_only: bool,
    ) -> Result<Vec<crate::models::toolkits::AuthField>, ComposioError> {
        let toolkit = toolkit.into();
        let auth_scheme = auth_scheme.into();
        
        let toolkit_info = self.get_toolkit(&toolkit).await?;
        
        let details = toolkit_info.auth_config_details.ok_or_else(|| {
            ComposioError::InvalidInput(format!(
                "No auth config details found for toolkit: {}",
                toolkit
            ))
        })?;
        
        for auth_detail in details {
            if auth_detail.mode == auth_scheme {
                if required_only {
                    return Ok(auth_detail.fields.connected_account_initiation.required);
                } else {
                    let mut fields = auth_detail.fields.connected_account_initiation.required;
                    fields.extend(auth_detail.fields.connected_account_initiation.optional);
                    return Ok(fields);
                }
            }
        }
        
        Err(ComposioError::InvalidInput(format!(
            "Auth config details not found with toolkit={} and auth_scheme={}",
            toolkit, auth_scheme
        )))
    }

    /// Get auth config creation fields for a toolkit
    ///
    /// Retrieves the required and optional fields needed to create an auth config
    /// for a specific toolkit and authentication scheme.
    ///
    /// # Arguments
    ///
    /// * `toolkit` - The toolkit slug (e.g., "github", "gmail")
    /// * `auth_scheme` - The authentication scheme (e.g., "OAUTH2", "API_KEY")
    /// * `required_only` - If true, returns only required fields; if false, returns both required and optional
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let fields = client.get_auth_config_creation_fields(
    ///     "github",
    ///     "OAUTH2",
    ///     true
    /// ).await?;
    ///
    /// for field in fields {
    ///     println!("Required field: {}", field.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_auth_config_creation_fields(
        &self,
        toolkit: impl Into<String>,
        auth_scheme: impl Into<String>,
        required_only: bool,
    ) -> Result<Vec<crate::models::toolkits::AuthField>, ComposioError> {
        let toolkit = toolkit.into();
        let auth_scheme = auth_scheme.into();
        
        let toolkit_info = self.get_toolkit(&toolkit).await?;
        
        let details = toolkit_info.auth_config_details.ok_or_else(|| {
            ComposioError::InvalidInput(format!(
                "No auth config details found for toolkit: {}",
                toolkit
            ))
        })?;
        
        for auth_detail in details {
            if auth_detail.mode == auth_scheme {
                if required_only {
                    return Ok(auth_detail.fields.auth_config_creation.required);
                } else {
                    let mut fields = auth_detail.fields.auth_config_creation.required;
                    fields.extend(auth_detail.fields.auth_config_creation.optional);
                    return Ok(fields);
                }
            }
        }
        
        Err(ComposioError::InvalidInput(format!(
            "Auth config details not found with toolkit={} and auth_scheme={}",
            toolkit, auth_scheme
        )))
    }

    // ========================================================================
    // Tools Methods
    // ========================================================================

    /// List tools with filtering options
    ///
    /// Retrieves a list of available tools based on the provided filters.
    /// Tools are individual actions that can be performed on external services.
    ///
    /// # Arguments
    ///
    /// * `params` - Filter parameters for the query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::tools::ToolListParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = ToolListParams {
    ///     toolkit_slug: Some("github".to_string()),
    ///     limit: Some(20),
    ///     ..Default::default()
    /// };
    ///
    /// let tools = client.list_tools(params).await?;
    /// for tool in tools.items {
    ///     println!("Tool: {} ({})", tool.name, tool.slug);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_tools(
        &self,
        params: crate::models::tools::ToolListParams,
    ) -> Result<crate::models::tools::ToolListResponse, ComposioError> {
        let mut url = format!("{}/api/v3/tools", self.config.base_url);
        
        // Build query parameters
        let mut query_params = vec![];
        
        if let Some(tool_slugs) = &params.tool_slugs {
            query_params.push(format!("tool_slugs={}", tool_slugs.join(",")));
        }
        if let Some(toolkit_slug) = &params.toolkit_slug {
            query_params.push(format!("toolkit_slug={}", toolkit_slug));
        }
        if let Some(search) = &params.search {
            query_params.push(format!("search={}", search));
        }
        if let Some(scopes) = &params.scopes {
            query_params.push(format!("scopes={}", scopes.join(",")));
        }
        if let Some(tags) = &params.tags {
            query_params.push(format!("tags={}", tags.join(",")));
        }
        if let Some(importance) = &params.importance {
            query_params.push(format!("importance={}", importance));
        }
        if let Some(show_deprecated) = params.show_deprecated {
            query_params.push(format!("show_deprecated={}", show_deprecated));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &params.cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        if let Some(toolkit_versions) = &params.toolkit_versions {
            query_params.push(format!("toolkit_versions={}", toolkit_versions));
        }
        
        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Get a specific tool by slug
    ///
    /// Retrieves detailed information about a specific tool including
    /// input/output schemas, scopes, and version information.
    ///
    /// # Arguments
    ///
    /// * `slug` - The tool slug (e.g., "GITHUB_CREATE_ISSUE", "GMAIL_SEND_EMAIL")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let tool = client.get_tool("GITHUB_CREATE_ISSUE").await?;
    /// println!("Tool: {}", tool.name);
    /// println!("Description: {}", tool.description);
    /// println!("Version: {}", tool.version);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_tool(
        &self,
        slug: impl Into<String>,
    ) -> Result<crate::models::tools::ToolInfo, ComposioError> {
        let slug = slug.into();
        let url = format!("{}/api/v3/tools/{}", self.config.base_url, slug);

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Execute a tool
    ///
    /// Executes a specific tool with the provided arguments. The tool must be
    /// available and the user must have appropriate authentication.
    ///
    /// # Arguments
    ///
    /// * `params` - Tool execution parameters including slug, arguments, and auth info
    ///
    /// # Returns
    ///
    /// Returns a `ToolExecutionResponse` containing:
    /// - `data`: The tool's output data
    /// - `error`: Optional error message if execution failed
    /// - `successful`: Whether the execution was successful
    /// - `log_id`: Unique identifier for this execution (for debugging)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The tool is not found
    /// - The user doesn't have a connected account for the toolkit
    /// - The arguments are invalid or missing required fields
    /// - Network error occurs
    /// - API returns an error response
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::tools::ToolExecuteParams;
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let mut arguments = HashMap::new();
    /// arguments.insert("owner".to_string(), serde_json::json!("composio"));
    /// arguments.insert("repo".to_string(), serde_json::json!("composio"));
    /// arguments.insert("title".to_string(), serde_json::json!("Test issue"));
    ///
    /// let params = ToolExecuteParams {
    ///     slug: "GITHUB_CREATE_ISSUE".to_string(),
    ///     arguments,
    ///     user_id: Some("user_123".to_string()),
    ///     version: Some("1.0.0".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let result = client.execute_tool(params).await?;
    /// println!("Result: {:?}", result.data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_tool(
        &self,
        params: crate::models::tools::ToolExecuteParams,
    ) -> Result<crate::models::tools::ToolExecutionResponse, ComposioError> {
        use crate::utils::toolkit_version::get_toolkit_version;

        let url = format!("{}/api/v3/tools/execute/{}", self.config.base_url, params.slug());

        // Resolve version if not provided
        let version = if let Some(v) = params.version {
            v
        } else {
            // Extract toolkit from slug (e.g., "GITHUB_CREATE_ISSUE" -> "github")
            let toolkit = params.slug()
                .split('_')
                .next()
                .unwrap_or(params.slug())
                .to_lowercase();
            
            get_toolkit_version(&toolkit, self.config.toolkit_versions.as_ref()).as_str().to_string()
        };

        // Check if version is 'latest' and skip check is not enabled
        if version == "latest" && !params.dangerously_skip_version_check.unwrap_or(false) {
            return Err(ComposioError::InvalidInput(
                "Tool version 'latest' requires dangerously_skip_version_check=true. \
                 Please specify an explicit version or enable the skip check.".to_string()
            ));
        }

        // Build request body
        let mut body = serde_json::json!({
            "arguments": params.arguments,
            "version": version,
        });

        if let Some(connected_account_id) = params.connected_account_id {
            body["connected_account_id"] = serde_json::json!(connected_account_id);
        }
        if let Some(custom_auth_params) = params.custom_auth_params {
            body["custom_auth_params"] = serde_json::to_value(custom_auth_params)
                .map_err(|e| ComposioError::InvalidInput(e.to_string()))?;
        }
        if let Some(custom_connection_data) = params.custom_connection_data {
            body["custom_connection_data"] = serde_json::to_value(custom_connection_data)
                .map_err(|e| ComposioError::InvalidInput(e.to_string()))?;
        }
        if let Some(user_id) = params.user_id {
            body["user_id"] = serde_json::json!(user_id);
        }
        if let Some(text) = params.text {
            body["text"] = serde_json::json!(text);
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Execute a proxy request to a third-party API
    ///
    /// Makes an authenticated HTTP request to a third-party API using
    /// a connected account's credentials. This is useful for calling
    /// endpoints that don't have predefined tools.
    ///
    /// # Arguments
    ///
    /// * `params` - Proxy request parameters including endpoint, method, and auth
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::tools::{ToolProxyParams, HttpMethod};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = ToolProxyParams {
    ///     endpoint: "/repos/composio/composio".to_string(),
    ///     method: HttpMethod::Get,
    ///     connected_account_id: Some("ca_123".to_string()),
    ///     body: None,
    ///     parameters: None,
    ///     custom_connection_data: None,
    /// };
    ///
    /// let result = client.proxy_tool(params).await?;
    /// println!("Status: {}", result.status);
    /// println!("Data: {:?}", result.data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn proxy_tool(
        &self,
        params: crate::models::tools::ToolProxyParams,
    ) -> Result<crate::models::tools::ToolProxyResponse, ComposioError> {
        let url = format!("{}/api/v3/tools/execute/proxy", self.config.base_url);

        // Build request body
        let mut body = serde_json::json!({
            "endpoint": params.endpoint,
            "method": params.method,
        });

        if let Some(request_body) = params.body {
            body["body"] = request_body;
        }
        if let Some(connected_account_id) = params.connected_account_id {
            body["connected_account_id"] = serde_json::json!(connected_account_id);
        }
        if let Some(parameters) = params.parameters {
            body["parameters"] = serde_json::to_value(parameters)
                .map_err(|e| ComposioError::InvalidInput(e.to_string()))?;
        }
        if let Some(custom_connection_data) = params.custom_connection_data {
            body["custom_connection_data"] = serde_json::to_value(custom_connection_data)
                .map_err(|e| ComposioError::InvalidInput(e.to_string()))?;
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Generate tool inputs from natural language
    ///
    /// Uses AI to convert a natural language description into structured
    /// tool arguments. This is useful for allowing users to describe what
    /// they want to do in plain language.
    ///
    /// # Arguments
    ///
    /// * `params` - Input generation parameters including tool slug and text
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::tools::ToolInputGenerationParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = ToolInputGenerationParams {
    ///     tool_slug: "GITHUB_CREATE_ISSUE".to_string(),
    ///     text: "Create an issue about fixing the login bug in the composio repo".to_string(),
    ///     custom_tool_description: None,
    ///     custom_system_prompt: None,
    /// };
    ///
    /// let result = client.generate_tool_inputs(params).await?;
    /// if let Some(arguments) = result.arguments {
    ///     println!("Generated arguments: {:?}", arguments);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_tool_inputs(
        &self,
        params: crate::models::tools::ToolInputGenerationParams,
    ) -> Result<crate::models::tools::ToolInputGenerationResponse, ComposioError> {
        let url = format!(
            "{}/api/v3/tools/execute/{}/input",
            self.config.base_url,
            params.tool_slug
        );

        // Build request body
        let mut body = serde_json::json!({
            "text": params.text,
        });

        if let Some(custom_tool_description) = params.custom_tool_description {
            body["custom_tool_description"] = serde_json::json!(custom_tool_description);
        }
        if let Some(custom_system_prompt) = params.custom_system_prompt {
            body["custom_system_prompt"] = serde_json::json!(custom_system_prompt);
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    // ========================================================================
    // Triggers Methods
    // ========================================================================

    /// List trigger types
    ///
    /// Retrieves a list of available trigger types (templates) based on filters.
    /// Trigger types define what events can be listened for.
    ///
    /// # Arguments
    ///
    /// * `params` - Filter parameters for the query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::triggers::TriggerTypeListParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = TriggerTypeListParams {
    ///     toolkit_slugs: Some(vec!["github".to_string()]),
    ///     limit: Some(20),
    ///     ..Default::default()
    /// };
    ///
    /// let triggers = client.list_trigger_types(params).await?;
    /// for trigger in triggers.items {
    ///     println!("Trigger: {} ({})", trigger.name, trigger.slug);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_trigger_types(
        &self,
        params: crate::models::triggers::TriggerTypeListParams,
    ) -> Result<crate::models::triggers::TriggerTypeListResponse, ComposioError> {
        let mut url = format!("{}/api/v3/triggers_types", self.config.base_url);
        
        // Build query parameters
        let mut query_params = vec![];
        
        if let Some(cursor) = &params.cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(toolkit_slugs) = &params.toolkit_slugs {
            query_params.push(format!("toolkit_slugs={}", toolkit_slugs.join(",")));
        }
        if let Some(toolkit_versions) = &params.toolkit_versions {
            query_params.push(format!("toolkit_versions={}", toolkit_versions));
        }
        
        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Get a specific trigger type by slug
    ///
    /// Retrieves detailed information about a trigger type including
    /// configuration schema and payload schema.
    ///
    /// # Arguments
    ///
    /// * `slug` - The trigger type slug (e.g., "GITHUB_COMMIT_EVENT")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let trigger = client.get_trigger_type("GITHUB_COMMIT_EVENT").await?;
    /// println!("Trigger: {}", trigger.name);
    /// println!("Type: {}", trigger.trigger_type);
    /// println!("Config schema: {}", trigger.config);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_trigger_type(
        &self,
        slug: impl Into<String>,
    ) -> Result<crate::models::triggers::TriggerType, ComposioError> {
        let slug = slug.into();
        let url = format!("{}/api/v3/triggers_types/{}", self.config.base_url, slug);

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// List active trigger instances
    ///
    /// Retrieves a list of active trigger instances (listeners) based on filters.
    /// Trigger instances are active event listeners for specific users.
    ///
    /// # Arguments
    ///
    /// * `params` - Filter parameters for the query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::triggers::TriggerInstanceListParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let params = TriggerInstanceListParams {
    ///     trigger_names: Some(vec!["GITHUB_COMMIT_EVENT".to_string()]),
    ///     show_disabled: Some(false),
    ///     ..Default::default()
    /// };
    ///
    /// let instances = client.list_active_triggers(params).await?;
    /// for instance in instances.items {
    ///     println!("Instance: {} for user {}", instance.trigger_name, instance.user_id.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_active_triggers(
        &self,
        params: crate::models::triggers::TriggerInstanceListParams,
    ) -> Result<crate::models::triggers::TriggerInstanceListResponse, ComposioError> {
        let mut url = format!("{}/api/v3/trigger_instances/active", self.config.base_url);
        
        // Build query parameters
        let mut query_params = vec![];
        
        if let Some(trigger_ids) = &params.trigger_ids {
            query_params.push(format!("trigger_ids={}", trigger_ids.join(",")));
        }
        if let Some(trigger_names) = &params.trigger_names {
            query_params.push(format!("trigger_names={}", trigger_names.join(",")));
        }
        if let Some(auth_config_ids) = &params.auth_config_ids {
            query_params.push(format!("auth_config_ids={}", auth_config_ids.join(",")));
        }
        if let Some(connected_account_ids) = &params.connected_account_ids {
            query_params.push(format!("connected_account_ids={}", connected_account_ids.join(",")));
        }
        if let Some(show_disabled) = params.show_disabled {
            query_params.push(format!("show_disabled={}", show_disabled));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &params.cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        
        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Create a trigger instance
    ///
    /// Creates a new trigger instance (event listener) for a user.
    /// Either `connected_account_id` or `user_id` must be provided.
    /// If `user_id` is provided, the most recent connected account will be used.
    ///
    /// # Arguments
    ///
    /// * `params` - Trigger creation parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::triggers::TriggerCreateParams;
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// let mut config = HashMap::new();
    /// config.insert("repo".to_string(), serde_json::json!("composio"));
    /// config.insert("owner".to_string(), serde_json::json!("composio"));
    ///
    /// let params = TriggerCreateParams {
    ///     slug: "GITHUB_COMMIT_EVENT".to_string(),
    ///     user_id: Some("user_123".to_string()),
    ///     connected_account_id: None,
    ///     trigger_config: Some(config),
    ///     toolkit_versions: None,
    /// };
    ///
    /// let trigger = client.create_trigger(params).await?;
    /// println!("Created trigger: {}", trigger.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_trigger(
        &self,
        mut params: crate::models::triggers::TriggerCreateParams,
    ) -> Result<crate::models::triggers::TriggerCreateResponse, ComposioError> {
        // If user_id is provided but not connected_account_id, find the connected account
        if params.user_id.is_some() && params.connected_account_id.is_none() {
            let user_id = params.user_id.as_ref().unwrap();
            
            // Get trigger type to find toolkit
            let trigger_type = self.get_trigger_type(&params.slug).await?;
            let toolkit = trigger_type.toolkit.slug;
            
            // Find connected account for this user and toolkit
            let account_params = crate::models::connected_accounts::ConnectedAccountListParams {
                user_ids: Some(vec![user_id.clone()]),
                toolkit_slugs: Some(vec![toolkit]),
                ..Default::default()
            };
            
            let accounts = self.list_connected_accounts(account_params).await?;
            
            if accounts.items.is_empty() {
                return Err(ComposioError::InvalidInput(format!(
                    "No connected accounts found for trigger {} and user {}",
                    params.slug, user_id
                )));
            }
            
            // Use the most recent account
            let mut sorted_accounts = accounts.items;
            sorted_accounts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            params.connected_account_id = Some(sorted_accounts[0].id.clone());
        }
        
        if params.connected_account_id.is_none() {
            return Err(ComposioError::InvalidInput(
                "Either connected_account_id or user_id must be provided".to_string()
            ));
        }

        let url = format!("{}/api/v3/trigger_instances/{}/upsert", self.config.base_url, params.slug);

        // Build request body
        let mut body = serde_json::json!({
            "connected_account_id": params.connected_account_id.unwrap(),
        });

        if let Some(trigger_config) = params.trigger_config {
            body["trigger_config"] = serde_json::to_value(trigger_config)
                .map_err(|e| ComposioError::InvalidInput(e.to_string()))?;
        }
        if let Some(toolkit_versions) = params.toolkit_versions {
            body["toolkit_versions"] = serde_json::json!(toolkit_versions);
        }

        // Execute request with retry logic
        let response = crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        // Parse response
        Ok(response
            .json()
            .await
            .map_err(ComposioError::NetworkError)?)
    }

    /// Delete a trigger instance
    ///
    /// Permanently deletes a trigger instance. This cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `trigger_id` - The trigger instance ID to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// client.delete_trigger("ti_abc123").await?;
    /// println!("Trigger deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_trigger(
        &self,
        trigger_id: impl Into<String>,
    ) -> Result<(), ComposioError> {
        let trigger_id = trigger_id.into();
        let url = format!("{}/api/v3/trigger_instances/manage/{}", self.config.base_url, trigger_id);

        // Execute request with retry logic
        crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .delete(&url)
                .header("x-api-key", &self.config.api_key)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        Ok(())
    }

    /// Enable a trigger instance
    ///
    /// Enables a previously disabled trigger instance.
    ///
    /// # Arguments
    ///
    /// * `trigger_id` - The trigger instance ID to enable
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// client.enable_trigger("ti_abc123").await?;
    /// println!("Trigger enabled");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn enable_trigger(
        &self,
        trigger_id: impl Into<String>,
    ) -> Result<(), ComposioError> {
        let trigger_id = trigger_id.into();
        let url = format!("{}/api/v3/trigger_instances/manage/{}", self.config.base_url, trigger_id);

        let body = serde_json::json!({
            "status": "enable"
        });

        // Execute request with retry logic
        crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .patch(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        Ok(())
    }

    /// Disable a trigger instance
    ///
    /// Temporarily disables a trigger instance without deleting it.
    ///
    /// # Arguments
    ///
    /// * `trigger_id` - The trigger instance ID to disable
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// client.disable_trigger("ti_abc123").await?;
    /// println!("Trigger disabled");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn disable_trigger(
        &self,
        trigger_id: impl Into<String>,
    ) -> Result<(), ComposioError> {
        let trigger_id = trigger_id.into();
        let url = format!("{}/api/v3/trigger_instances/manage/{}", self.config.base_url, trigger_id);

        let body = serde_json::json!({
            "status": "disable"
        });

        // Execute request with retry logic
        crate::retry::with_retry(&self.config.retry_policy, || async {
            let response = self
                .http_client
                .patch(&url)
                .header("x-api-key", &self.config.api_key)
                .json(&body)
                .send()
                .await
                .map_err(ComposioError::NetworkError)?;

            // Check for errors
            if !response.status().is_success() {
                return Err(ComposioError::from_response(response).await);
            }

            Ok(response)
        })
        .await?;

        Ok(())
    }

    /// Verify an incoming webhook payload and signature
    ///
    /// This method validates that the webhook request is authentic by:
    /// 1. Validating the webhook timestamp is within the tolerance window
    /// 2. Verifying the HMAC-SHA256 signature using the correct algorithm
    /// 3. Parsing the payload and detecting the webhook version (V1, V2, or V3)
    ///
    /// # Arguments
    ///
    /// * `params` - Webhook verification parameters including id, payload, signature, timestamp, and secret
    ///
    /// # Returns
    ///
    /// Returns a `VerifyWebhookResult` containing:
    /// - `version`: Detected webhook version (V1, V2, or V3)
    /// - `payload`: Normalized trigger event
    /// - `raw_payload`: Original parsed payload
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Signature verification fails
    /// - Timestamp is outside tolerance window
    /// - Payload cannot be parsed
    /// - Required headers are missing
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::triggers::WebhookVerifyParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    ///
    /// // In a webhook handler (e.g., Actix-web, Axum, etc.)
    /// let params = WebhookVerifyParams {
    ///     id: "msg_abc123".to_string(),
    ///     payload: r#"{"type":"composio.trigger.message","data":{}}"#.to_string(),
    ///     signature: "v1,base64signature".to_string(),
    ///     timestamp: "1234567890".to_string(),
    ///     secret: "whsec_abc123".to_string(),
    ///     tolerance: Some(300),
    /// };
    ///
    /// let result = client.verify_webhook(params)?;
    /// println!("Webhook version: {:?}", result.version);
    /// println!("Trigger slug: {}", result.payload.trigger_slug);
    /// # Ok(())
    /// # }
    /// ```
    pub fn verify_webhook(
        &self,
        params: crate::models::triggers::WebhookVerifyParams,
    ) -> Result<crate::models::triggers::VerifyWebhookResult, ComposioError> {
        use base64::{Engine as _, engine::general_purpose};
        use std::time::{SystemTime, UNIX_EPOCH};

        let tolerance = params.tolerance.unwrap_or(300);

        // Validate timestamp if tolerance is set
        if tolerance > 0 {
            let timestamp_seconds: i64 = params.timestamp.parse()
                .map_err(|_| ComposioError::InvalidInput(
                    format!("Invalid webhook timestamp: {}. Expected Unix timestamp in seconds.", params.timestamp)
                ))?;

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| ComposioError::InvalidInput(format!("System time error: {}", e)))?
                .as_secs() as i64;

            let time_difference = (current_time - timestamp_seconds).abs();

            if time_difference > tolerance as i64 {
                return Err(ComposioError::InvalidInput(
                    format!(
                        "The webhook timestamp is outside the allowed tolerance. \
                        The webhook was sent {} seconds ago, but the maximum allowed age is {} seconds.",
                        time_difference, tolerance
                    )
                ));
            }
        }

        // Verify signature
        if params.payload.is_empty() {
            return Err(ComposioError::InvalidInput("No webhook payload was provided.".to_string()));
        }

        if params.signature.is_empty() {
            return Err(ComposioError::InvalidInput(
                "No signature header value was provided. \
                Please pass the value of the webhook signature header.".to_string()
            ));
        }

        if params.secret.is_empty() {
            return Err(ComposioError::InvalidInput(
                "No webhook secret was provided. \
                You can find your webhook secret in your Composio dashboard.".to_string()
            ));
        }

        if params.id.is_empty() {
            return Err(ComposioError::InvalidInput(
                "No webhook ID was provided. \
                Please pass the value of the 'webhook-id' header.".to_string()
            ));
        }

        if params.timestamp.is_empty() {
            return Err(ComposioError::InvalidInput(
                "No webhook timestamp was provided. \
                Please pass the value of the 'webhook-timestamp' header.".to_string()
            ));
        }

        // Parse signature header - format is "v1,base64Sig" or "v1,sig1 v1,sig2"
        let signature_parts: Vec<&str> = params.signature.split(' ').collect();
        let mut v1_signatures: Vec<&str> = Vec::new();

        for part in signature_parts {
            if part.starts_with("v1,") {
                v1_signatures.push(&part[3..]); // Remove "v1," prefix
            }
        }

        if v1_signatures.is_empty() {
            return Err(ComposioError::InvalidInput(
                "No valid v1 signature found in the signature header. \
                Expected format: 'v1,base64EncodedSignature'".to_string()
            ));
        }

        // Construct the string to sign: webhookId.webhookTimestamp.payload
        let to_sign = format!("{}.{}.{}", params.id, params.timestamp, params.payload);

        // Compute expected signature using HMAC-SHA256
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(params.secret.as_bytes())
            .map_err(|e| ComposioError::InvalidInput(format!("Invalid secret key: {}", e)))?;
        mac.update(to_sign.as_bytes());
        let expected_signature_bytes = mac.finalize().into_bytes();
        let expected_signature_b64 = general_purpose::STANDARD.encode(&expected_signature_bytes);

        // Check if any of the provided signatures match (timing-safe)
        let mut signature_valid = false;
        for provided_sig in v1_signatures {
            // Use constant-time comparison
            if expected_signature_b64.len() == provided_sig.len() {
                let mut matches = true;
                for (a, b) in expected_signature_b64.bytes().zip(provided_sig.bytes()) {
                    if a != b {
                        matches = false;
                    }
                }
                if matches {
                    signature_valid = true;
                    break;
                }
            }
        }

        if !signature_valid {
            return Err(ComposioError::InvalidInput("The signature provided is invalid.".to_string()));
        }

        // Parse and detect version
        let raw_payload: serde_json::Value = serde_json::from_str(&params.payload)
            .map_err(|e| ComposioError::InvalidInput(format!("Failed to parse webhook payload as JSON: {}", e)))?;

        // Detect version and normalize payload
        let (version, normalized_payload) = self.parse_webhook_payload(&raw_payload)?;

        Ok(crate::models::triggers::VerifyWebhookResult {
            version,
            payload: normalized_payload,
            raw_payload,
        })
    }

    /// Parse webhook payload and detect version (internal helper)
    fn parse_webhook_payload(
        &self,
        data: &serde_json::Value,
    ) -> Result<(crate::models::triggers::WebhookVersion, crate::models::triggers::TriggerEvent), ComposioError> {
        use crate::models::triggers::WebhookVersion;

        // Try V3 first (has 'type' starting with 'composio.' and 'metadata' as dict)
        if let Some(obj) = data.as_object() {
            if let Some(event_type) = obj.get("type").and_then(|v| v.as_str()) {
                if event_type.starts_with("composio.") 
                    && obj.contains_key("metadata") 
                    && obj.get("metadata").and_then(|v| v.as_object()).is_some()
                    && obj.contains_key("id")
                    && obj.contains_key("data") {
                    return Ok((WebhookVersion::V3, self.normalize_v3_payload(data)?));
                }
            }

            // Try V2 (has 'type', 'timestamp', 'data' with nested fields)
            if obj.contains_key("type")
                && obj.contains_key("timestamp")
                && obj.contains_key("data") {
                if let Some(data_obj) = obj.get("data").and_then(|v| v.as_object()) {
                    if data_obj.contains_key("connection_id") {
                        return Ok((WebhookVersion::V2, self.normalize_v2_payload(data)?));
                    }
                }
            }

            // Try V1 (has 'trigger_name', 'connection_id', 'trigger_id', 'payload')
            if obj.contains_key("trigger_name")
                && obj.contains_key("connection_id")
                && obj.contains_key("trigger_id")
                && obj.contains_key("payload") {
                return Ok((WebhookVersion::V1, self.normalize_v1_payload(data)?));
            }
        }

        Err(ComposioError::InvalidInput(
            "Webhook payload does not match any known version (V1, V2, or V3). \
            Please ensure the payload structure is correct.".to_string()
        ))
    }

    /// Normalize V1 payload to TriggerEvent format (internal helper)
    fn normalize_v1_payload(
        &self,
        data: &serde_json::Value,
    ) -> Result<crate::models::triggers::TriggerEvent, ComposioError> {
        use crate::models::triggers::{TriggerEvent, TriggerMetadata, TriggerConnectedAccount};

        let obj = data.as_object().ok_or_else(|| 
            ComposioError::InvalidInput("V1 payload must be an object".to_string())
        )?;

        let trigger_id = obj.get("trigger_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let trigger_name = obj.get("trigger_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let connection_id = obj.get("connection_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let payload = obj.get("payload").cloned();

        Ok(TriggerEvent {
            id: trigger_id.clone(),
            uuid: trigger_id.clone(),
            user_id: String::new(), // V1 doesn't have user_id
            toolkit_slug: String::new(), // V1 doesn't have toolkit_slug
            trigger_slug: trigger_name.clone(),
            metadata: TriggerMetadata {
                id: trigger_id.clone(),
                uuid: trigger_id.clone(),
                toolkit_slug: String::new(),
                trigger_slug: trigger_name,
                trigger_data: None,
                trigger_config: serde_json::json!({}),
                connected_account: TriggerConnectedAccount {
                    id: connection_id.clone(),
                    uuid: connection_id,
                    auth_config_id: String::new(),
                    auth_config_uuid: String::new(),
                    user_id: String::new(),
                    status: "ACTIVE".to_string(),
                },
            },
            payload,
            original_payload: None,
        })
    }

    /// Normalize V2 payload to TriggerEvent format (internal helper)
    fn normalize_v2_payload(
        &self,
        data: &serde_json::Value,
    ) -> Result<crate::models::triggers::TriggerEvent, ComposioError> {
        use crate::models::triggers::{TriggerEvent, TriggerMetadata, TriggerConnectedAccount};

        let obj = data.as_object().ok_or_else(|| 
            ComposioError::InvalidInput("V2 payload must be an object".to_string())
        )?;

        let event_type = obj.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_uppercase();

        let payload_data = obj.get("data")
            .and_then(|v| v.as_object())
            .ok_or_else(|| ComposioError::InvalidInput("V2 payload missing 'data' object".to_string()))?;

        let trigger_id = payload_data.get("trigger_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let trigger_nano_id = payload_data.get("trigger_nano_id")
            .and_then(|v| v.as_str())
            .unwrap_or(&trigger_id)
            .to_string();
        let user_id = payload_data.get("user_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let connection_id = payload_data.get("connection_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let connection_nano_id = payload_data.get("connection_nano_id")
            .and_then(|v| v.as_str())
            .unwrap_or(&connection_id)
            .to_string();

        // Extract payload fields, excluding metadata fields
        let excluded_keys = ["connection_id", "connection_nano_id", "trigger_nano_id", "trigger_id", "user_id"];
        let mut filtered_payload = serde_json::Map::new();
        for (k, v) in payload_data.iter() {
            if !excluded_keys.contains(&k.as_str()) {
                filtered_payload.insert(k.clone(), v.clone());
            }
        }

        Ok(TriggerEvent {
            id: trigger_nano_id.clone(),
            uuid: trigger_id.clone(),
            user_id: user_id.clone(),
            toolkit_slug: event_type.clone(),
            trigger_slug: event_type.clone(),
            metadata: TriggerMetadata {
                id: trigger_nano_id,
                uuid: trigger_id,
                toolkit_slug: event_type.clone(),
                trigger_slug: event_type,
                trigger_data: None,
                trigger_config: serde_json::json!({}),
                connected_account: TriggerConnectedAccount {
                    id: connection_nano_id,
                    uuid: connection_id,
                    auth_config_id: String::new(),
                    auth_config_uuid: String::new(),
                    user_id,
                    status: "ACTIVE".to_string(),
                },
            },
            payload: Some(serde_json::Value::Object(filtered_payload)),
            original_payload: None,
        })
    }

    /// Normalize V3 payload to TriggerEvent format (internal helper)
    fn normalize_v3_payload(
        &self,
        data: &serde_json::Value,
    ) -> Result<crate::models::triggers::TriggerEvent, ComposioError> {
        use crate::models::triggers::{TriggerEvent, TriggerMetadata, TriggerConnectedAccount};

        let obj = data.as_object().ok_or_else(|| 
            ComposioError::InvalidInput("V3 payload must be an object".to_string())
        )?;

        let metadata = obj.get("metadata")
            .and_then(|v| v.as_object())
            .ok_or_else(|| ComposioError::InvalidInput("V3 payload missing 'metadata' object".to_string()))?;

        // Check if this is a trigger event (has trigger-specific metadata fields)
        let is_trigger_event = metadata.contains_key("trigger_id")
            && metadata.contains_key("trigger_slug")
            && metadata.contains_key("user_id")
            && metadata.contains_key("connected_account_id")
            && metadata.contains_key("auth_config_id")
            && metadata.contains_key("log_id");

        if is_trigger_event {
            let trigger_id = metadata.get("trigger_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let trigger_slug = metadata.get("trigger_slug")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let user_id = metadata.get("user_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let connected_account_id = metadata.get("connected_account_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let auth_config_id = metadata.get("auth_config_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Extract toolkit slug from trigger slug (e.g., "GITHUB_COMMIT_EVENT" -> "GITHUB")
            let toolkit_slug = if trigger_slug.contains('_') {
                trigger_slug.split('_').next().unwrap_or("UNKNOWN").to_uppercase()
            } else {
                "UNKNOWN".to_string()
            };

            let event_data = obj.get("data").cloned();

            Ok(TriggerEvent {
                id: trigger_id.clone(),
                uuid: trigger_id.clone(),
                user_id: user_id.clone(),
                toolkit_slug: toolkit_slug.clone(),
                trigger_slug: trigger_slug.clone(),
                metadata: TriggerMetadata {
                    id: trigger_id.clone(),
                    uuid: trigger_id,
                    toolkit_slug,
                    trigger_slug,
                    trigger_data: None,
                    trigger_config: serde_json::json!({}),
                    connected_account: TriggerConnectedAccount {
                        id: connected_account_id.clone(),
                        uuid: connected_account_id.clone(),
                        auth_config_id: auth_config_id.clone(),
                        auth_config_uuid: auth_config_id,
                        user_id,
                        status: "ACTIVE".to_string(),
                    },
                },
                payload: event_data,
                original_payload: None,
            })
        } else {
            // Non-trigger V3 event (e.g., connection expired)
            let event_type = obj.get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let event_id = obj.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            Ok(TriggerEvent {
                id: event_id.clone(),
                uuid: event_id.clone(),
                user_id: String::new(),
                toolkit_slug: "COMPOSIO".to_string(),
                trigger_slug: event_type.clone(),
                metadata: TriggerMetadata {
                    id: event_id.clone(),
                    uuid: event_id,
                    toolkit_slug: "COMPOSIO".to_string(),
                    trigger_slug: event_type,
                    trigger_data: None,
                    trigger_config: serde_json::json!({}),
                    connected_account: TriggerConnectedAccount {
                        id: String::new(),
                        uuid: String::new(),
                        auth_config_id: String::new(),
                        auth_config_uuid: String::new(),
                        user_id: String::new(),
                        status: "ACTIVE".to_string(),
                    },
                },
                payload: obj.get("data").cloned(),
                original_payload: None,
            })
        }
    }
}

impl ComposioClientBuilder {
    /// Set the API key
    ///
    /// The API key is required for authenticating with the Composio API.
    /// You can obtain your API key from the Composio dashboard.
    ///
    /// # Arguments
    ///
    /// * `key` - The Composio API key (can be `String` or `&str`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the base URL
    ///
    /// Override the default Composio API base URL. This is useful for testing
    /// or when using a custom Composio deployment.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL (must start with http:// or https://)
    ///
    /// # Default
    ///
    /// `https://backend.composio.dev/api/v3`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .base_url("https://custom.api.com")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout
    ///
    /// Configure how long to wait for API requests to complete before timing out.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration
    ///
    /// # Default
    ///
    /// 30 seconds
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .timeout(Duration::from_secs(60))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    ///
    /// Configure how many times to retry failed requests for transient errors
    /// (rate limits, server errors, network issues).
    ///
    /// # Arguments
    ///
    /// * `retries` - Maximum number of retry attempts
    ///
    /// # Default
    ///
    /// 3 retries
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .max_retries(5)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Set the initial retry delay
    ///
    /// Configure the delay before the first retry attempt. Subsequent retries
    /// use exponential backoff based on this initial delay.
    ///
    /// # Arguments
    ///
    /// * `delay` - Initial delay duration
    ///
    /// # Default
    ///
    /// 1 second
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .initial_retry_delay(Duration::from_secs(2))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn initial_retry_delay(mut self, delay: Duration) -> Self {
        self.initial_retry_delay = Some(delay);
        self
    }

    /// Set the maximum retry delay
    ///
    /// Configure the maximum delay between retry attempts. This caps the
    /// exponential backoff to prevent excessively long waits.
    ///
    /// # Arguments
    ///
    /// * `delay` - Maximum delay duration
    ///
    /// # Default
    ///
    /// 10 seconds
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .max_retry_delay(Duration::from_secs(30))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = Some(delay);
        self
    }

    /// Set toolkit version configuration
    ///
    /// Configure which versions of toolkits to use. This allows you to:
    /// - Use "latest" for all toolkits (default behavior)
    /// - Specify different versions for different toolkits
    /// - Pin specific toolkits to specific versions for stability
    ///
    /// Version resolution follows this precedence:
    /// 1. `COMPOSIO_TOOLKIT_VERSION_{TOOLKIT}` environment variable (highest priority)
    /// 2. User-provided configuration (this method)
    /// 3. `COMPOSIO_TOOLKIT_VERSION` global environment variable
    /// 4. Default to "latest"
    ///
    /// # Arguments
    ///
    /// * `versions` - Toolkit version configuration
    ///
    /// # Default
    ///
    /// None (uses "latest" for all toolkits)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    /// use composio_sdk::models::versioning::{ToolkitVersion, ToolkitVersionParam};
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Use latest for all toolkits
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .toolkit_versions(ToolkitVersionParam::Latest)
    ///     .build()?;
    ///
    /// // Use specific versions for specific toolkits
    /// let mut versions = HashMap::new();
    /// versions.insert("github".to_string(), ToolkitVersion::Specific("20250906_01".to_string()));
    /// versions.insert("gmail".to_string(), ToolkitVersion::Latest);
    ///
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .toolkit_versions(ToolkitVersionParam::Versions(versions))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn toolkit_versions(mut self, versions: crate::models::versioning::ToolkitVersionParam) -> Self {
        self.toolkit_versions = Some(versions);
        self
    }

    /// Set the file download directory
    ///
    /// Configure the directory where downloaded files will be saved.
    /// If not set, files will be downloaded to the current working directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - Path to the download directory
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::ComposioClient;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .file_download_dir(PathBuf::from("./downloads"))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn file_download_dir(mut self, dir: impl Into<std::path::PathBuf>) -> Self {
        self.file_download_dir = Some(dir.into());
        self
    }

    /// Enable or disable automatic file upload/download
    ///
    /// When enabled (default), the SDK will automatically:
    /// - Upload local file paths to S3 before tool execution
    /// - Download file URLs returned by tools to local paths
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable automatic file handling
    ///
    /// # Default
    ///
    /// true
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .auto_upload_download_files(false)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn auto_upload_download_files(mut self, enabled: bool) -> Self {
        self.auto_upload_download_files = Some(enabled);
        self
    }

    /// Enable or disable telemetry tracking
    ///
    /// When enabled, the SDK will send anonymous usage telemetry to Composio.
    /// This helps improve the SDK but is disabled by default for privacy.
    ///
    /// Telemetry is sent asynchronously and does not block operations.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable telemetry
    ///
    /// # Default
    ///
    /// false (opt-in for privacy)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .telemetry_enabled(true)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn telemetry_enabled(mut self, enabled: bool) -> Self {
        self.telemetry_enabled = Some(enabled);
        self
    }

    /// Build the client
    ///
    /// Validates the configuration and constructs a `ComposioClient` instance.
    /// The reqwest HTTP client is configured with the specified timeout and
    /// default headers (including the API key).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - API key is not provided or is empty
    /// - Base URL is invalid (doesn't start with http:// or https://)
    /// - HTTP client construction fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use composio_sdk::client::ComposioClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ComposioClient::builder()
    ///     .api_key("your_api_key")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<ComposioClient, ComposioError> {
        // Try explicit API key first, then environment variable
        let api_key = self
            .api_key
            .or_else(|| std::env::var("COMPOSIO_API_KEY").ok())
            .ok_or_else(|| {
                ComposioError::ConfigError(
                    "API key not provided. Set COMPOSIO_API_KEY environment variable or use .api_key()".to_string()
                )
            })?;

        // Build configuration with defaults
        let mut config = ComposioConfig::new(api_key);

        if let Some(base_url) = self.base_url {
            config.base_url = base_url;
        }

        if let Some(timeout) = self.timeout {
            config.timeout = timeout;
        }

        // Build retry policy
        let mut retry_policy = RetryPolicy::default();
        if let Some(max_retries) = self.max_retries {
            retry_policy.max_retries = max_retries;
        }
        if let Some(initial_delay) = self.initial_retry_delay {
            retry_policy.initial_delay = initial_delay;
        }
        if let Some(max_delay) = self.max_retry_delay {
            retry_policy.max_delay = max_delay;
        }
        config.retry_policy = retry_policy;

        // Set toolkit versions if provided
        if let Some(toolkit_versions) = self.toolkit_versions {
            config.toolkit_versions = Some(toolkit_versions);
        }

        // Set file management options
        if let Some(file_download_dir) = self.file_download_dir {
            config.file_download_dir = Some(file_download_dir);
        }
        if let Some(auto_upload_download_files) = self.auto_upload_download_files {
            config.auto_upload_download_files = auto_upload_download_files;
        }

        // Set telemetry option (opt-in, disabled by default)
        if let Some(telemetry_enabled) = self.telemetry_enabled {
            config.telemetry_enabled = telemetry_enabled;
        }

        // Validate configuration
        config.validate()?;

        // Build HTTP client with timeout and default headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            reqwest::header::HeaderValue::from_str(&config.api_key)
                .map_err(|_| ComposioError::InvalidInput("Invalid API key format".to_string()))?,
        );

        let http_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()
            .map_err(|e| ComposioError::NetworkError(e))?;

        Ok(ComposioClient {
            http_client,
            config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_api_key_only() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        assert_eq!(client.config().api_key, "test_key");
        assert_eq!(
            client.config().base_url,
            "https://backend.composio.dev/api/v3"
        );
        assert_eq!(client.config().timeout, Duration::from_secs(30));
        assert_eq!(client.config().retry_policy.max_retries, 3);
    }

    #[test]
    fn test_builder_with_all_options() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .base_url("https://custom.api.com")
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .initial_retry_delay(Duration::from_secs(2))
            .max_retry_delay(Duration::from_secs(30))
            .build()
            .unwrap();

        assert_eq!(client.config().api_key, "test_key");
        assert_eq!(client.config().base_url, "https://custom.api.com");
        assert_eq!(client.config().timeout, Duration::from_secs(60));
        assert_eq!(client.config().retry_policy.max_retries, 5);
        assert_eq!(
            client.config().retry_policy.initial_delay,
            Duration::from_secs(2)
        );
        assert_eq!(
            client.config().retry_policy.max_delay,
            Duration::from_secs(30)
        );
    }

    #[test]
    fn test_builder_without_api_key_fails() {
        let result = ComposioClient::builder().build();

        assert!(result.is_err());
        match result {
            Err(ComposioError::InvalidInput(msg)) => {
                assert_eq!(msg, "API key is required");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_builder_with_empty_api_key_fails() {
        let result = ComposioClient::builder().api_key("").build();

        assert!(result.is_err());
        match result {
            Err(ComposioError::InvalidInput(msg)) => {
                assert_eq!(msg, "API key cannot be empty");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_builder_with_invalid_base_url_fails() {
        let result = ComposioClient::builder()
            .api_key("test_key")
            .base_url("invalid-url")
            .build();

        assert!(result.is_err());
        match result {
            Err(ComposioError::ConfigError(msg)) => {
                assert_eq!(msg, "Base URL must start with http:// or https://");
            }
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_builder_accepts_string_api_key() {
        let client = ComposioClient::builder()
            .api_key("test_key".to_string())
            .build()
            .unwrap();

        assert_eq!(client.config().api_key, "test_key");
    }

    #[test]
    fn test_builder_accepts_str_api_key() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        assert_eq!(client.config().api_key, "test_key");
    }

    #[test]
    fn test_client_is_cloneable() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        let cloned = client.clone();
        assert_eq!(client.config().api_key, cloned.config().api_key);
    }

    #[test]
    fn test_client_is_debuggable() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("ComposioClient"));
    }

    #[test]
    fn test_builder_is_debuggable() {
        let builder = ComposioClient::builder().api_key("test_key");

        let debug_str = format!("{:?}", builder);
        assert!(debug_str.contains("ComposioClientBuilder"));
    }

    #[test]
    fn test_http_client_has_correct_timeout() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .timeout(Duration::from_secs(45))
            .build()
            .unwrap();

        assert_eq!(client.config().timeout, Duration::from_secs(45));
    }

    #[test]
    fn test_config_accessor() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        let config = client.config();
        assert_eq!(config.api_key, "test_key");
    }

    #[test]
    fn test_http_client_accessor() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        let _http_client = client.http_client();
        // Just verify we can access it without panic
    }

    #[test]
    fn test_builder_method_chaining() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .base_url("https://test.com")
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .initial_retry_delay(Duration::from_secs(2))
            .max_retry_delay(Duration::from_secs(30))
            .build()
            .unwrap();

        assert_eq!(client.config().api_key, "test_key");
        assert_eq!(client.config().base_url, "https://test.com");
    }

    #[test]
    fn test_default_retry_policy() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();

        assert_eq!(client.config().retry_policy.max_retries, 3);
        assert_eq!(
            client.config().retry_policy.initial_delay,
            Duration::from_secs(1)
        );
        assert_eq!(
            client.config().retry_policy.max_delay,
            Duration::from_secs(10)
        );
    }

    #[test]
    fn test_custom_retry_policy() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .max_retries(7)
            .initial_retry_delay(Duration::from_millis(500))
            .max_retry_delay(Duration::from_secs(20))
            .build()
            .unwrap();

        assert_eq!(client.config().retry_policy.max_retries, 7);
        assert_eq!(
            client.config().retry_policy.initial_delay,
            Duration::from_millis(500)
        );
        assert_eq!(
            client.config().retry_policy.max_delay,
            Duration::from_secs(20)
        );
    }

    #[test]
    fn test_partial_retry_policy_customization() {
        let client = ComposioClient::builder()
            .api_key("test_key")
            .max_retries(5)
            .build()
            .unwrap();

        assert_eq!(client.config().retry_policy.max_retries, 5);
        assert_eq!(
            client.config().retry_policy.initial_delay,
            Duration::from_secs(1)
        );
        assert_eq!(
            client.config().retry_policy.max_delay,
            Duration::from_secs(10)
        );
    }
}
