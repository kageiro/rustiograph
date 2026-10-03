//! Interface for interacting with the Telegra.ph API.
//!
//! This module provides [`Rustiograph`], the main entry point for making
//! requests to the Telegra.ph API. It handles API configuration,
//! authentication, HTTP requests, response decoding, and content
//! preparation.
//!
//! In addition to high-level methods for supported Telegra.ph API
//! operations, [`Rustiograph`] provides lower-level utilities for constructing
//! API requests, formatting service URLs, and preparing page content.
//!
//! These utilities can be used when the high-level interface does not cover
//! a particular use case or when working with API methods that are not yet
//! represented by the library's typed interface.

use reqwest::Client;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use std::time::Duration;

use crate::methods::{
    CreateAccount, CreatePage, EditAccountInfo, EditPage, GetAccountInfo, GetPage, GetPageList,
    GetViews, RevokeToken,
};

use crate::consts::{DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, SERVICE_URL};
use crate::content::ContentInput;
use crate::error;
use crate::helpers::decode;
use crate::methods::IntoMethod;
use crate::methods::Method;
use crate::payload::to_payload;
use crate::result::Result;
use crate::types::account::Account;
use crate::types::page::Page;
use crate::types::page_list::PageList;
use crate::types::page_views::PageViews;
use crate::utils::html::{html_to_json, nodes_to_json};

use error::RustiographError;
use serde_json::{Value, json};

/// A client for interacting with the Telegra.ph API.
///
/// `Rustiograph` stores the HTTP client, authentication token, API endpoints,
/// and request-specific configuration used when communicating with the
/// Telegra.ph service.
///
/// A client can be created with [`Rustiograph::new`].
#[derive(Clone, Debug)]
pub struct Rustiograph {
    /// The underlying HTTP client used to perform API requests.
    pub client: Client,

    /// The access token used to authenticate requests.
    ///
    /// This is `None` when the client has not been authenticated with a
    /// Telegra.ph account.
    pub token: Option<String>,

    /// The name of the service being accessed.
    pub service: String,

    /// The base URL of the Telegra.ph API.
    pub api_url: String,

    /// The base URL of the Telegra.ph website.
    pub service_url: String,

    /// The `User-Agent` header sent with requests.
    ///
    /// When `None`, no custom `User-Agent` header is configured.
    pub request_user_agent: Option<HeaderValue>,

    /// The maximum amount of time allowed for an HTTP request.
    ///
    /// When `None`, the HTTP client's default timeout is used.
    pub request_timeout: Option<Duration>,
}

/// A builder for configuring and creating a [`Rustiograph`] client.
///
/// Fields that are not explicitly configured use their default values.
///
/// The builder can be created with [`Rustiograph::builder`] or
/// [`RustiographBuilder::default`].
#[derive(Default, Debug)]
#[must_use]
pub struct RustiographBuilder {
    /// The access token used to authenticate requests.
    ///
    /// If not provided, the resulting client is created without an
    /// authentication token.
    token: Option<String>,

    /// The name of the service being accessed.
    ///
    /// If not provided, the default service name is used.
    service: Option<String>,

    /// A custom HTTP client to use for API requests.
    ///
    /// If not provided, a client is created automatically.
    client: Option<Client>,

    /// A custom `User-Agent` value to include in API requests.
    ///
    /// If not provided, no custom value is configured.
    user_agent: Option<String>,

    /// The timeout applied to API requests.
    ///
    /// If not provided, the default request timeout is used.
    timeout: Option<Duration>,
}

#[derive(Deserialize)]
struct Envelope<T> {
    ok: bool,
    result: Option<T>,
    error: Option<String>,
}

#[derive(Serialize)]
struct Authed<'a, P: ?Sized> {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<&'a str>,
    #[serde(flatten)]
    params: &'a P,
}

impl RustiographBuilder {
    /// Sets the access token used for authenticated Telegra.ph API requests.
    ///
    /// The token is stored in the resulting [`Rustiograph`] instance.
    ///
    /// This option is optional. A client without an access token can still
    /// call API methods that do not require authentication.
    ///
    /// # Example
    ///
    /// ```text
    /// let Rustiograph = Rustiograph::builder()
    ///     .token("your-access-token")
    ///     .build()?;
    /// ```
    ///
    /// [`Rustiograph`]: crate::Rustiograph
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets the service domain used by the client.
    ///
    /// The value must be a domain name without a URL scheme. For example:
    ///
    /// ```text
    /// "telegra.ph"
    /// "example.com"
    /// "telegra.ph:8080"
    /// ```
    ///
    /// The service domain is used to construct the service and API URLs:
    ///
    /// ```text
    /// service URL: https://telegra.ph
    /// API URL:     https://api.telegra.ph
    /// ```
    ///
    /// The default value is [`SERVICE_URL`].
    ///
    /// Trailing `/` characters are removed when the client is built.
    ///
    /// This option can be useful when connecting to a Telegra.ph-compatible
    /// /// service hosted under another domain.
    ///
    /// # Errors
    ///
    /// An invalid service value causes [`RustiographBuilder::build`] to return
    /// [`RustiographError::InvalidServiceUrl`].
    ///
    /// [`SERVICE_URL`]: crate::consts::SERVICE_URL
    /// [`RustiographError::InvalidServiceUrl`]:
    ///     crate::RustiographError::InvalidServiceUrl
    pub fn service(mut self, service: impl Into<String>) -> Self {
        self.service = Some(service.into());
        self
    }

    /// Sets a custom HTTP client.
    ///
    /// When a custom [`reqwest::Client`] is provided, Rustiograph uses it
    /// instead of creating its own client.
    ///
    /// This is useful when the application needs [`reqwest`] configuration
    /// that is not exposed by [`RustiographBuilder`], such as a proxy, custom
    /// TLS configuration, connection settings, or other HTTP options.
    ///
    /// The supplied client is used as-is. Rustiograph does not modify its
    /// configuration.
    ///
    /// # Example
    ///
    /// ```text
    /// let client = reqwest::Client::builder()
    ///     .build()?;
    ///
    /// let Rustiograph = Rustiograph::builder()
    ///     .client(client)
    ///     .build()?;
    /// ```
    ///
    /// [`RustiographBuilder`]: crate::RustiographBuilder
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Sets the `User-Agent` value used by the client.
    ///
    /// When no custom HTTP client is supplied, this value is used as the
    /// default `User-Agent` when Rustiograph creates its internal
    /// [`reqwest::Client`].
    ///
    /// When a custom client is supplied with [`Self::client`], the value is
    /// stored separately and can be applied to requests by Rustiograph.
    ///
    /// If this option is not set, [`DEFAULT_USER_AGENT`] is used when
    /// creating the default HTTP client.
    ///
    /// # Errors
    ///
    /// An invalid HTTP header value causes [`RustiographBuilder::build`] to
    /// return [`RustiographError::InvalidUserAgent`].
    ///
    /// [`DEFAULT_USER_AGENT`]: crate::consts::DEFAULT_USER_AGENT
    /// [`RustiographError::InvalidUserAgent`]:
    ///     crate::RustiographError::InvalidUserAgent
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Sets the HTTP request timeout.
    ///
    /// When no custom HTTP client is supplied, this value is used when
    /// Rustiograph creates its internal [`reqwest::Client`].
    ///
    /// When a custom client is supplied with [`Self::client`], the timeout is
    /// stored separately and can be applied at the request level by
    /// Rustiograph.
    ///
    /// If this option is not set, [`DEFAULT_TIMEOUT`] is used when creating
    /// the default HTTP client.
    ///
    /// # Example
    ///
    /// ```text
    /// use std::time::Duration;
    /// ///
    /// let Rustiograph = Rustiograph::builder()
    ///     .timeout(Duration::from_secs(30))
    ///     .build()?;
    /// ```
    ///
    /// [`DEFAULT_TIMEOUT`]: crate::consts::DEFAULT_TIMEOUT
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Builds a [`Rustiograph`] client from the configured options.
    ///
    /// Options that were not explicitly configured use their default values.
    ///
    /// The default configuration is:
    ///
    /// ```text
    /// service    = "telegra.ph"
    /// user-agent = "RustioGraph"
    /// timeout    = 10 seconds
    /// token      = None
    /// ```
    ///
    /// If [`Self::client`] was not used, `build` creates a new
    /// [`reqwest::Client`] using the configured `User-Agent` and timeout.
    ///
    /// If a custom client was provided, that client is used instead and no
    /// additional HTTP client is created.
    ///
    /// # Service validation
    ///
    /// The service must contain a `.` and must not contain a URL scheme such
    /// as `http://` or `https://`.
    ///
    /// For example, these values are accepted:
    ///
    /// ```text
    /// "telegra.ph"
    /// "example.com"
    /// "telegra.ph:8080"
    /// ```
    ///
    /// These values are rejected:
    ///
    /// ```text
    /// "localhost"
    /// "https://telegra.ph"
    /// "http://example.com"
    /// ```
    ///
    /// Trailing `/` characters are removed before constructing the service
    /// and API URLs.
    ///
    /// # Errors
    ///
    /// Returns [`RustiographError::InvalidServiceUrl`] when the configured
    /// service does not satisfy the required format.
    ///
    /// Returns [`RustiographError::InvalidUserAgent`] when the configured
    /// `User-Agent` cannot be converted into a valid HTTP header value.
    ///
    /// Returns a [`reqwest::Error`] if Rustiograph creates an internal HTTP
    /// client and `reqwest` fails to build it.
    ///
    /// [`Rustiograph`]: crate::Rustiograph
    /// [`RustiographError::InvalidServiceUrl`]:
    ///     crate::RustiographError::InvalidServiceUrl
    /// [`RustiographError::InvalidUserAgent`]:
    ///     crate::RustiographError::InvalidUserAgent
    /// [`DEFAULT_TIMEOUT`]: crate::consts::DEFAULT_TIMEOUT
    /// [`DEFAULT_USER_AGENT`]: crate::consts::DEFAULT_USER_AGENT
    pub fn build(self) -> Result<Rustiograph> {
        let service = self.service.as_deref().unwrap_or(SERVICE_URL);

        if !service.contains('.') || service.contains("://") {
            return Err(RustiographError::InvalidServiceUrl);
        }
        let service = service.trim_end_matches('/').to_string();

        let ua = self
            .user_agent
            .as_deref()
            .map(HeaderValue::from_str)
            .transpose()
            .map_err(|_| RustiographError::InvalidUserAgent)?;

        let (client, request_user_agent, request_timeout) = if let Some(client) = self.client {
            (client, ua, self.timeout)
        } else {
            let client = Client::builder()
                .user_agent(ua.unwrap_or_else(|| HeaderValue::from_static(DEFAULT_USER_AGENT)))
                .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
                .build()?;
            (client, None, None)
        };

        Ok(Rustiograph {
            client,
            token: self.token,
            service_url: format!("https://{service}"),
            api_url: format!("https://api.{service}"),
            service,
            request_user_agent,
            request_timeout,
        })
    }
}

fn validate_method_name(name: &str) -> Result<()> {
    if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphanumeric()) {
        return Err(RustiographError::InvalidMethodName);
    }
    Ok(())
}

impl Rustiograph {
    /// Creates a new [`RustiographBuilder`].
    ///
    /// The builder can be used to configure the HTTP client, access token,
    /// service domain, `User-Agent`, and request timeout before creating a
    /// [`Rustiograph`] instance.
    ///
    /// All builder options are optional and use their default values when
    /// not explicitly configured.
    ///
    /// # Example
    ///
    /// ```text
    /// let Rustiograph = Rustiograph::builder()
    ///     .token("your-access-token")
    ///     .timeout(Duration::from_secs(30))
    ///     .build()?;
    /// ```
    ///
    /// For a client with default configuration, [`Rustiograph::new`] is a
    /// shorter alternative.
    ///
    /// [`RustiographBuilder`]: crate::RustiographBuilder
    pub fn builder() -> RustiographBuilder {
        RustiographBuilder::default()
    }

    /// Creates a [`Rustiograph`] client with the default configuration.
    ///
    /// The optional `token` is stored in the client and used for API methods
    /// that require authentication.
    ///
    /// The HTTP client, service URL, API URL, `User-Agent`, and request
    /// timeout use their default configuration.
    ///
    /// Pass `None` to create an unauthenticated client:
    ///
    /// ```text
    /// let Rustiograph = Rustiograph::new(None)?;
    /// ```
    ///
    /// An access token can also be supplied directly:
    ///
    /// ```text
    /// let Rustiograph = Rustiograph::new(Some("your-access-token".into()))?;
    /// ```
    ///
    /// For custom configuration, use [`Rustiograph::builder`] instead.
    ///
    /// # Errors
    ///
    /// Returns an error if the default client configuration cannot be built.
    ///
    /// [`Rustiograph`]: crate::Rustiograph
    pub fn new(token: Option<String>) -> Result<Self> {
        let mut b = Self::builder();
        b.token = token;
        b.build()
    }

    /// Creates a [`Rustiograph`] client using a custom HTTP client.
    ///
    /// The supplied [`reqwest::Client`] is used for all API requests. The
    /// optional `token` is stored in the resulting client and used for
    /// authenticated requests.
    ///
    /// This constructor is useful when the application already has a
    /// configured `reqwest::Client`, for example when it needs custom proxy,
    /// TLS, connection, or other HTTP configuration.
    ///
    /// ```text
    /// let client = reqwest::Client::builder()
    ///     .build()?;
    ///
    /// let Rustiograph = Rustiograph::with_client(
    ///     client,
    ///     Some("your-access-token".into()),
    /// )?;
    /// ```
    ///
    /// If more extensive Rustiograph-specific configuration is required,
    /// use [`Rustiograph::builder`] and [`RustiographBuilder::client`] instead.
    ///
    /// # Errors
    ///
    /// Returns an error if the client configuration cannot be completed.
    ///
    /// [`Rustiograph`]: crate::Rustiograph
    /// [`RustiographBuilder::client`]: crate::RustiographBuilder::client
    pub fn with_client(client: Client, token: Option<String>) -> Result<Self> {
        let mut b = Self::builder().client(client);
        b.token = token;
        b.build()
    }

    /// Returns the currently configured access token.
    ///
    /// The returned reference points to the token stored inside the
    /// [`Rustiograph`] instance and does not allocate a new string.
    ///
    /// Returns `None` when no access token is configured.
    ///
    /// # Example
    ///
    /// ```text
    /// if let Some(token) = Rustiograph.token() {
    ///     println!("Token: {token}");
    /// }
    /// ```
    ///
    /// [`Rustiograph`]: crate::Rustiograph
    #[must_use]
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Sets the access token used for authenticated API requests.
    ///
    /// The supplied value replaces the token currently stored in the client.
    ///
    /// This method does not perform any API request. It only changes the
    /// authentication token that will be used by subsequent requests.
    ///
    /// # Example
    ///
    /// ```text
    /// Rustiograph.set_token("new-access-token");
    /// ```
    ///
    /// To remove the current token, use [`Rustiograph::clear_token`].
    ///
    /// [`Rustiograph::clear_token`]: crate::Rustiograph::clear_token
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    /// Removes the currently configured access token.
    ///
    /// After this method returns, [`Rustiograph::token`] returns `None` and
    /// subsequent requests are made without the stored access token.
    ///
    /// This method does not perform any API request and does not revoke the
    /// token on the Telegra.ph server. It only removes the token from this
    /// [`Rustiograph`] instance.
    ///
    /// To revoke an access token through the Telegra.ph API, use the
    /// corresponding API method instead.
    ///
    /// [`Rustiograph::token`]: crate::Rustiograph::token
    /// [`Rustiograph`]: crate::Rustiograph
    pub fn clear_token(&mut self) {
        self.token = None;
    }
}

impl Rustiograph {
    /// Builds the URL for a Telegra.ph API method.
    ///
    /// The URL is constructed from the client's configured API URL, the
    /// method name, and an optional page path.
    ///
    /// Without a path, the resulting URL has the following form:
    ///
    /// ```text
    /// {api_url}/{method}
    /// ```
    ///
    /// When a path is provided, it is appended to the method:
    ///
    /// ```text
    /// {api_url}/{method}/{path}
    /// ```
    ///
    /// # Example
    ///
    /// ```text
    /// let url = Rustiograph.format_api_url("getPage", None);
    /// ```
    ///
    /// The result is equivalent to:
    ///
    /// ```text
    /// https://api.telegra.ph/getPage
    /// ```
    ///
    /// With a page path:
    ///
    /// ```text
    /// let url = Rustiograph.format_api_url(
    ///     "getPage",
    ///     Some("example-page-123"),
    /// );
    /// ```
    ///
    /// The result is:
    ///
    /// ```text
    /// https://api.telegra.ph/getPage/example-page-123
    /// ```
    ///
    /// The method does not validate or URL-encode `method` or `path`.
    /// Callers are expected to provide values suitable for use as URL path
    /// components.
    #[must_use]
    pub fn format_api_url(&self, method: &str, path: Option<&str>) -> String {
        let mut url = format!("{}/{}", self.api_url.trim_end_matches('/'), method);
        if let Some(p) = path {
            url.push('/');
            url.push_str(p);
        }
        url
    }

    /// Builds a URL for a page or other resource on the configured
    /// Telegra.ph service.
    ///
    /// The URL is constructed from the client's configured service URL and
    /// the supplied path:
    ///
    /// ```text
    /// {service_url}/{path}
    /// ```
    ///
    /// Trailing `/` characters are removed from `path` before constructing
    /// the URL.
    ///
    /// # Example
    ///
    /// ```text
    /// let url = Rustiograph.format_service_url("my-page-123");
    /// ```
    ///
    /// The result is:
    ///
    /// ```text
    /// https://telegra.ph/my-page-123
    /// ```
    ///
    /// This method only constructs the URL. It does not perform an HTTP
    /// request or validate whether the specified resource exists.
    #[must_use]
    pub fn format_service_url(&self, path: &str) -> String {
        format!("{}/{}", self.service_url, path.trim_end_matches('/'))
    }

    /// Converts page content into the JSON string format expected by the
    /// Telegra.ph API.
    ///
    /// [`ContentInput::Html`] values are parsed as HTML and converted into
    /// Telegra.ph's JSON node representation. [`ContentInput::Nodes`] values
    /// are converted directly from [`Node`] values into the same JSON
    /// representation.
    ///
    /// The resulting JSON array is then serialized into a string. This is
    /// necessary because Telegra.ph expects the `content` API parameter to be
    /// a JSON-encoded string rather than a JSON array value.
    ///
    /// # HTML input
    ///
    /// When [`ContentInput::Html`] is used, the HTML must contain only
    /// elements supported by Rustiograph's HTML parser and the Telegra.ph
    /// content format.
    ///
    /// For example:
    ///
    /// ```text
    /// let content = ContentInput::Html(
    ///     "<p>Hello <strong>world</strong>!</p>".into()
    /// );
    ///
    /// let prepared = Rustiograph.prepare_content(content)?;
    /// ```
    ///
    /// The resulting string contains the JSON representation of the
    /// corresponding Telegra.ph nodes.
    ///
    /// # Node input
    ///
    /// [`ContentInput::Nodes`] can be used when the caller already has
    /// structured [`Node`] values and does not need HTML parsing:
    ///
    /// ```text
    /// let content = ContentInput::Nodes(nodes);
    /// let prepared = Rustiograph.prepare_content(content)?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`RustiographError::InvalidContent`] when HTML or node content
    /// cannot be converted into the Telegra.ph node representation.
    ///
    /// Returns [`RustiographError::Encode`] when the resulting node structure
    /// cannot be serialized into JSON.
    ///
    /// [`ContentInput::Html`]: crate::content::ContentInput::Html
    /// [`ContentInput::Nodes`]: crate::content::ContentInput::Nodes
    /// [`Node`]: crate::types::Node
    /// [`RustiographError::InvalidContent`]:
    ///     crate::RustiographError::InvalidContent
    /// [`RustiographError::Encode`]: crate::RustiographError::Encode
    pub fn prepare_content(&self, content: ContentInput) -> Result<String> {
        let json_content = match content {
            ContentInput::Html(html) => {
                html_to_json(&html).map_err(RustiographError::InvalidContent)?
            }
            ContentInput::Nodes(nodes) => {
                nodes_to_json(&nodes).map_err(RustiographError::InvalidContent)?
            }
        };

        serde_json::to_string(&json_content).map_err(RustiographError::Encode)
    }
}

impl Rustiograph {
    fn maybe_set_token(&mut self, auth: bool, account: &Account) {
        if auth && let Some(token) = &account.access_token {
            self.set_token(token);
        }
    }

    async fn maybe_mix_author(&self, as_user: bool, payload: &mut Value) -> Result<()> {
        if as_user {
            self.mix_payload_author(payload).await?;
        }
        Ok(())
    }

    async fn mix_payload_author(&self, payload: &mut Value) -> Result<()> {
        let account = self
            .get_account_info(GetAccountInfo {
                fields: &["short_name", "author_name", "author_url"],
            })
            .await?;

        if let Some(obj) = payload.as_object_mut() {
            if let Some(name) = account.author_name {
                obj.entry("author_name").or_insert_with(|| json!(name));
            }
            if let Some(url) = account.author_url {
                obj.entry("author_url").or_insert_with(|| json!(url));
            }
        }

        Ok(())
    }

    async fn request<R, P>(
        &self,
        method: impl IntoMethod,
        path: Option<&str>,
        params: &P,
    ) -> Result<R>
    where
        R: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = self.format_api_url(method.method_name(), path);
        let body = Authed {
            access_token: self.token.as_deref(),
            params,
        };

        let mut req = self.client.post(url).json(&body);
        if let Some(t) = self.request_timeout {
            req = req.timeout(t);
        }
        if let Some(ua) = &self.request_user_agent {
            req = req.header(DEFAULT_USER_AGENT, ua);
        }

        let env: Envelope<R> = req.send().await?.json().await?;
        match env.result {
            Some(r) if env.ok => Ok(r),
            _ => Err(RustiographError::from_api(
                env.error.as_deref().unwrap_or("Unknown Error"),
            )),
        }
    }
}

impl Rustiograph {
    /// Creates a new Telegra.ph account.
    ///
    /// If `s.auth` is enabled, the returned access token is stored in this
    /// client for subsequent authenticated requests.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, sent, or decoded
    /// as an [`Account`].
    pub async fn create_account(&mut self, s: CreateAccount) -> Result<Account> {
        let payload = serde_json::to_value(&s)?;

        let account: Account = decode(self.request(Method::CreateAccount, None, &payload).await?)?;

        self.maybe_set_token(s.auth, &account);

        Ok(account)
    }

    /// Updates information associated with the current Telegra.ph account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, sent, or decoded
    /// as an [`Account`].
    pub async fn edit_account_info(&self, s: EditAccountInfo) -> Result<Account> {
        decode::<Account>(
            self.request(Method::EditAccountInfo, None, &serde_json::to_value(s)?)
                .await?,
        )
    }

    /// Retrieves information about the current Telegra.ph account.
    ///
    /// The requested fields are taken from [`GetAccountInfo::fields`].
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, sent, or decoded
    /// as an [`Account`].
    pub async fn get_account_info(&self, s: GetAccountInfo<'_>) -> Result<Account> {
        let mut payload = serde_json::to_value(&s)?;

        if !s.fields.is_empty() {
            payload["fields"] = json!(s.fields);
        }

        let raw: Value = self.request(Method::GetAccountInfo, None, &payload).await?;
        let value = if raw.is_null() { json!({}) } else { raw };
        decode(value)
    }

    /// Revokes the current access token.
    ///
    /// If `s.auth` is enabled, the access token returned by the API is stored
    /// in this client.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent or the response cannot
    /// be decoded as an [`Account`].
    pub async fn revoke_token(&mut self, s: RevokeToken) -> Result<Account> {
        let account: Account = decode(
            self.request(Method::RevokeAccessToken, None, &json!({}))
                .await?,
        )?;

        self.maybe_set_token(s.auth, &account);

        Ok(account)
    }

    /// Creates a new Telegra.ph page.
    ///
    /// The page content is converted into the JSON representation expected by
    /// the Telegra.ph API before the request is sent.
    ///
    /// If `s.as_user` is enabled, author information may be taken from the
    /// current account associated with this client.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, the content
    /// cannot be prepared, the request fails, or the response cannot be
    /// decoded as a [`Page`].
    pub async fn create_page(&self, s: CreatePage) -> Result<Page> {
        let mut payload = serde_json::to_value(&s)?;

        payload["content"] = Value::String(self.prepare_content(s.content)?);

        self.maybe_mix_author(s.as_user, &mut payload).await?;

        decode(self.request(Method::CreatePage, None, &payload).await?)
    }

    /// Updates an existing Telegra.ph page.
    ///
    /// The page is identified by the path contained in `s.path`.
    ///
    /// The page content is converted into the JSON representation expected by
    /// the Telegra.ph API before the request is sent.
    ///
    /// If `s.as_user` is enabled, author information may be taken from the
    /// current account associated with this client.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, the content
    /// cannot be prepared, the request fails, or the response cannot be
    /// decoded as a [`Page`].
    pub async fn edit_page(&self, s: EditPage<'_>) -> Result<Page> {
        let mut payload = serde_json::to_value(&s)?;

        payload["content"] = Value::String(self.prepare_content(s.content)?);

        self.maybe_mix_author(s.as_user, &mut payload).await?;

        decode(
            self.request(Method::EditPage, Some(s.path), &payload)
                .await?,
        )
    }

    /// Retrieves a Telegra.ph page by its path.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, sent, or decoded
    /// as a [`Page`].
    pub async fn get_page(&self, s: GetPage<'_>) -> Result<Page> {
        decode(
            self.request(Method::GetPage, None, &to_payload(&s)?)
                .await?,
        )
    }

    /// Retrieves a list of pages belonging to the current account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, sent, or decoded
    /// as a [`PageList`].
    pub async fn get_page_list(&self, s: GetPageList) -> Result<PageList> {
        decode(
            self.request(Method::GetPageList, None, &to_payload(&s)?)
                .await?,
        )
    }

    /// Retrieves the number of views for a Telegra.ph page.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized, sent, or decoded
    /// as page view information.
    pub async fn get_views(&self, s: GetViews<'_>) -> Result<i64> {
        Ok(decode::<PageViews>(
            self.request(Method::GetViews, None, &to_payload(&s)?)
                .await?,
        )?
        .views)
    }
}

impl Rustiograph {
    /// Calls a Telegra.ph API method directly.
    ///
    /// `call_raw` is the low-level interface for making API requests when the
    /// corresponding high-level Rustiograph method is not available or when
    /// direct control over the request and response types is required.
    ///
    /// Unlike the dedicated methods such as [`Rustiograph::create_page`] or
    /// [`Rustiograph::get_page`], this method does not require the API method to be
    /// explicitly implemented by Rustiograph. The method name is supplied by the
    /// caller, the request parameters are serialized from any type implementing
    /// [`serde::Serialize`], and the response is deserialized into the type
    /// specified by `R`.
    ///
    /// This makes `call_raw` useful for:
    ///
    /// - calling newly introduced Telegra.ph API methods before Rustiograph has
    ///   been updated to support them;
    /// - calling custom or experimental API methods;
    /// - providing a custom response type for an existing API method;
    /// - bypassing a high-level Rustiograph method when direct API access is
    ///   preferable.
    ///
    /// # Method names
    ///
    /// The `method` argument implements [`IntoMethod`]. This means that both
    /// [`Method`] values and arbitrary string values can be used.
    ///
    /// For methods already represented by Rustiograph, [`Method`] can be used:
    ///
    /// ```
    /// # use rustiograph::methods::Method;
    /// # let method = Method::GetPage;
    /// let method_name = method.as_str();
    /// ```
    ///
    /// For an API method that is not yet implemented by Rustiograph, pass its
    /// name directly as a string:
    ///
    /// ```text
    /// "newRustiographMethod"
    /// ```
    ///
    /// The string must contain a valid Telegra.ph API method name. The method name
    /// is validated before the request is sent.
    ///
    /// This allows applications to use newly introduced API methods without
    /// waiting for a new Rustiograph release.
    ///
    /// # Request parameters
    ///
    /// The `params` argument can be any value implementing [`serde::Serialize`].
    /// Rustiograph serializes this value into the JSON request payload before
    /// sending it to the API.
    ///
    /// For an implemented API method, Rustiograph normally provides a dedicated
    /// parameter structure. For an unsupported API method, the caller can define
    /// their own serializable structure matching the parameters expected by the
    /// API.
    ///
    /// For example:
    ///
    /// ```
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct CustomParams {
    ///     title: String,
    ///     limit: u32,
    /// }
    ///
    /// let params = CustomParams {
    ///     title: "My page".to_owned(),
    ///     limit: 10,
    /// };
    /// ```
    ///
    /// The resulting structure is serialized as a JSON object:
    ///
    /// ```json
    /// {
    ///     "title": "My page",
    ///     "limit": 10
    /// }
    /// ```
    ///
    /// A custom structure is not required, however. For simple requests,
    /// [`serde_json::Value`] or the [`serde_json::json!`] macro can be used
    /// directly:
    ///
    /// ```
    /// let params = serde_json::json!({
    ///     "title": "My page",
    ///     "limit": 10
    /// });
    /// ```
    ///
    /// This can be convenient when calling an API method only once or when
    /// experimenting with a method that is not yet represented by a Rustiograph
    /// type.
    ///
    /// Optional parameters can be represented using [`Option`] and omitted from
    /// the serialized payload with `skip_serializing_if`:
    ///
    /// ```
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct CustomParams {
    ///     title: String,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     limit: Option<u32>,
    /// }
    /// ```
    ///
    /// When `limit` is `None`, it will not be included in the JSON request.
    ///
    /// # Response type
    ///
    /// The return type `R` determines how the successful API response is
    /// deserialized.
    ///
    /// The caller must provide a type implementing [`serde::de::DeserializeOwned`]
    /// that matches the response returned by the API.
    ///
    /// For example, if an API method returns a response represented by
    /// `CustomResponse`, it can be called as follows:
    ///
    /// ```text
    /// let response: CustomResponse = Rustiograph
    ///     .call_raw("newRustiographMethod", None, &params)
    ///     .await?;
    /// ```
    ///
    /// Rustiograph does not need to know about `CustomResponse` in advance. The
    /// type only needs to implement [`serde::de::DeserializeOwned`].
    ///
    /// This also makes it possible to use existing Rustiograph response types
    /// when their structure matches the API response.
    ///
    /// # Path
    ///
    /// The `path` argument specifies an optional page path that is appended to the
    /// API endpoint for methods operating on a specific Telegra.ph page.
    ///
    /// For methods that do not use a page path, pass `None`:
    ///
    /// ```text
    /// Rustiograph.call_raw("someMethod", None, &params)
    /// ```
    ///
    /// For methods that operate on a page, pass the page path separately:
    ///
    /// ```text
    /// Rustiograph.call_raw("somePageMethod", Some("example-page-123"), &params)
    /// ```
    ///
    /// The page path is not included in the serialized request parameters. It is
    /// used as part of the request URL instead.
    ///
    /// This distinction is important for API methods whose endpoint has the form
    /// of:
    ///
    /// ```text
    /// https://telegra.ph/{path}/{method}
    /// ```
    ///
    /// The exact endpoint format is handled internally by Rustiograph.
    ///
    /// # Empty requests
    ///
    /// If an API method does not require parameters, [`Rustiograph::call_raw_empty`]
    /// can be used instead of constructing an empty parameter value.
    ///
    /// For example:
    ///
    /// ```text
    /// let result: CustomResponse = Rustiograph
    ///     .call_raw_empty("newRustiographMethod")
    ///     .await?;
    /// ```
    ///
    /// This is equivalent to calling `call_raw` with an empty parameter value.
    ///
    /// # `null` parameters
    ///
    /// If the serialized value of `params` is JSON `null`, Rustiograph converts
    /// it to an empty JSON object (`{}`) before sending the request.
    ///
    /// This means that a unit value such as `()` can be used when an API method
    /// expects an empty object:
    ///
    /// ```text
    /// let result: CustomResponse = Rustiograph
    ///     .call_raw("newRustiographMethod", None, &())
    ///     .await?;
    /// ```
    ///
    /// For methods that do not require parameters at all, prefer
    /// [`Rustiograph::call_raw_empty`] for clarity.
    ///
    /// # Complete raw request example
    ///
    /// A typical call to an API method that is not yet implemented by Rustiograph
    /// looks like this:
    ///
    /// ```text
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct NewMethodParams {
    ///     title: String,
    ///     limit: u32,
    /// }
    ///
    /// #[derive(serde::Deserialize)]
    /// struct NewMethodResponse {
    ///     success: bool,
    /// }
    ///
    /// let params = NewMethodParams {
    ///     title: "Hello".to_owned(),
    ///     limit: 10,
    /// };
    ///
    /// let response: NewMethodResponse = Rustiograph
    ///     .call_raw("newRustiographMethod", None, &params)
    ///     .await?;
    /// ```
    ///
    /// The important part is that neither `NewMethodParams` nor
    /// `NewMethodResponse` has to be provided by Rustiograph. They are defined by
    /// the application according to the API method being called.
    ///
    /// For a quick request, the parameter structure can instead be represented
    /// directly with JSON:
    ///
    /// ```text
    /// let params = serde_json::json!({
    ///     "title": "Hello",
    ///     "limit": 10,
    /// });
    ///
    /// let response: NewMethodResponse = Rustiograph
    ///     .call_raw("newRustiographMethod", None, &params)
    ///     .await?;
    /// ```
    ///
    /// # Forward compatibility
    ///
    /// `call_raw` is intentionally not limited to the methods currently known by
    /// Rustiograph.
    ///
    /// Telegra.ph may introduce a new API method before a new Rustiograph release
    /// is published. In that case, the new method can still be called by passing
    /// its name as a string and defining the required request and response types
    /// in the application.
    ///
    /// For example, an application can immediately use:
    ///
    /// ```text
    /// Rustiograph
    ///     .call_raw("futureRustiographMethod", None, &params)
    ///     .await?;
    /// ```
    ///
    /// Once Rustiograph adds a dedicated implementation for that method, the
    /// application can switch to the higher-level API if desired. Existing raw
    /// calls do not require Rustiograph to contain a corresponding [`Method`]
    /// variant.
    ///
    /// # Errors
    ///
    /// `call_raw` returns [`RustiographError`] if the request cannot be completed
    /// successfully.
    ///
    /// Errors can occur when:
    ///
    /// - the method name is invalid;
    /// - the request parameters cannot be serialized;
    /// - the HTTP request fails;
    /// - the Telegra.ph API returns an error;
    /// - the response does not contain the expected result;
    /// - the response cannot be deserialized into `R`.
    ///
    /// API errors are returned as [`RustiographError::Api`]. Other errors are
    /// returned directly and are not converted into API errors.
    ///
    /// # Choosing between high-level and raw methods
    ///
    /// Prefer the dedicated Rustiograph method when one exists. High-level
    /// methods provide typed request parameters, typed responses, and an API
    /// designed specifically for the corresponding Telegra.ph method.
    ///
    /// Use `call_raw` when:
    ///
    /// - the API method is not yet implemented by Rustiograph;
    /// - the API has changed and the library version does not yet expose the new
    ///   functionality;
    /// - a custom request or response type is required;
    /// - direct access to the underlying API is preferable.
    ///
    /// `call_raw` is therefore an escape hatch from the high-level API while
    /// retaining Rustiograph's HTTP handling, authentication, serialization, and
    /// error handling.
    ///
    /// [`IntoMethod`]: crate::methods::IntoMethod
    /// [`Method`]: crate::methods::Method
    /// [`Rustiograph`]: crate::Rustiograph
    /// [`RustiographError`]: crate::RustiographError
    /// [`RustiographError::Api`]: crate::RustiographError::Api
    /// [`serde::Serialize`]: serde::Serialize
    /// [`serde::de::DeserializeOwned`]: serde::de::DeserializeOwned
    /// [`serde_json::Value`]: serde_json::Value
    /// [`serde_json::json!`]: serde_json::json
    /// [`Option`]: std::option::Option
    /// [`Rustiograph::call_raw_empty`]: crate::Rustiograph::call_raw_empty
    pub async fn call_raw<M, P, R>(&self, method: M, path: Option<&str>, params: &P) -> Result<R>
    where
        M: IntoMethod,
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        validate_method_name(method.method_name())?;

        let payload = match serde_json::to_value(params).map_err(RustiographError::Encode)? {
            Value::Null => json!({}),
            other => other,
        };

        let raw: Value = self.request(method, path, &payload).await?;
        decode(raw)
    }

    /// Calls a Telegra.ph API method without request parameters.
    ///
    /// This is a convenience wrapper around [`Rustiograph::call_raw`] for
    /// methods that do not require a request payload.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Rustiograph::call_raw`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustiograph::{Account, Rustiograph};
    /// # use rustiograph::methods::Method;
    /// # async fn example(Rustiograph: &Rustiograph) -> rustiograph::Result<()> {
    /// let account: Account = Rustiograph
    ///     .call_raw_empty(Method::RevokeAccessToken)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_raw_empty<M, R>(&self, method: M) -> Result<R>
    where
        M: IntoMethod,
        R: DeserializeOwned,
    {
        self.call_raw(method, None, &()).await
    }

    /// Calls several Telegra.ph API methods until one succeeds.
    ///
    /// Methods are tried in the order they appear in `methods`.
    ///
    /// An [`RustiographError::Api`] error is treated as a method-specific
    /// failure and causes the next method to be tried. Other errors are
    /// returned immediately because they generally indicate a request,
    /// transport, rate-limit, serialization, or response-decoding failure
    /// that is unlikely to be resolved by changing the method name.
    ///
    /// If all methods return an API error, the error from the last attempted
    /// method is returned.
    ///
    /// # Errors
    ///
    /// Returns the first non-API error encountered, or the last API error if
    /// every method fails with an API error.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result: SomeResponse = Rustiograph
    ///     .call_raw_any(
    ///         &[Method::SomeMethod, Method::FallbackMethod],
    ///         &params,
    ///     )
    ///     .await?;
    /// ```
    pub async fn call_raw_any<M, P, R>(&self, methods: &[M], params: &P) -> Result<R>
    where
        M: IntoMethod,
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let mut last = RustiographError::InvalidMethodName;
        for method in methods {
            match self.call_raw(method, None, params).await {
                Ok(value) => return Ok(value),
                Err(err @ RustiographError::Api(_)) => last = err,
                Err(err) => return Err(err),
            }
        }
        Err(last)
    }
}
