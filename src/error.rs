//! Error types returned by the Rustiograph API.
//!
//! This module provides [`RustiographError`], the main error type used
//! throughout the crate, and [`ApiError`], which represents errors returned
//! directly by the Telegra.ph API.
//!
//! Most public operations return [`crate::result::Result`], whose error type
//! is [`RustiographError`].

use std::time::Duration;
use thiserror::Error;

/// The main error type returned by Rustiograph operations.
///
/// This enum covers errors originating from the Telegra.ph API, HTTP
/// requests, JSON serialization and deserialization, request validation,
/// and other failures encountered while using the library.
///
/// The enum is marked as non-exhaustive, so additional error variants may be
/// added in future releases without breaking exhaustive `match` expressions.
///
/// # Examples
///
/// ```
/// # use rustiograph::error::RustiographError;
///
/// fn handle_error(error: RustiographError) {
///     match error {
///         RustiographError::FloodWait { retry_after } => {
///             println!("Retry after {retry_after:?}");
///         }
///         RustiographError::Api(error) => {
///             println!("Telegra.ph API error: {error}");
///         }
///         _ => {}
///     }
/// }
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RustiographError {
    /// An error returned by the Telegra.ph API.
    #[error("telegraph api error: {0}")]
    Api(ApiError),

    /// The request was rate limited and should be retried after the
    /// specified duration.
    #[error("rate limited, retry after {retry_after:?}")]
    FloodWait {
        /// Time to wait before request can be repeated
        retry_after: Duration,
    },

    /// An error occurred while sending an HTTP request.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// The response body could not be decoded into the expected type.
    ///
    /// The `type_name` field contains the name of the type that was being
    /// decoded when the error occurred.
    #[error("failed to decode response as {type_name}")]
    Decode {
        /// The name of the type expected in the response.
        type_name: &'static str,
        /// The underlying JSON deserialization error.
        #[source]
        source: serde_json::Error,
    },

    /// The request could not be serialized into JSON.
    #[error("failed to encode request")]
    Encode(#[source] serde_json::Error),

    /// The API response does not contain the expected `result` field.
    #[error("response has no `result` field")]
    MissingResult,

    /// The supplied content is invalid.
    #[error("invalid content: {0}")]
    InvalidContent(String),

    /// A required content value was not provided.
    #[error("content is required")]
    ContentRequired,

    /// The supplied API method name is invalid.
    #[error("Invalid method name")]
    InvalidMethodName,

    /// The supplied service URL is invalid.
    #[error("Invalid service URL")]
    InvalidServiceUrl,

    /// The supplied `User-Agent` value is invalid.
    #[error("Invalid User Agent")]
    InvalidUserAgent,

    /// No files were supplied when a file upload was requested.
    #[cfg(feature = "upload")]
    #[error("No files passed")]
    NoFilesPassed,

    /// A JSON serialization or deserialization operation failed.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// The supplied content is shorter than the allowed minimum length.
    #[error("Content is too short")]
    ContentTooShort {
        /// The minimum allowed length.
        min: usize,
        /// The actual content length.
        actual: usize,
    },

    /// The supplied content exceeds the allowed maximum length.
    #[error("Content is too long")]
    ContentTooLong {
        /// The maximum allowed length.
        max: usize,
        /// The actual content length.
        actual: usize,
    },

    /// An unspecified error represented by a custom message.
    #[error("Other: {0}")]
    Other(String),
}

/// An error returned directly by the Telegra.ph API.
///
/// Known API error codes are represented by dedicated variants. Error codes
/// not recognized by the library are preserved by [`ApiError::Unknown`].
///
/// The enum is marked as non-exhaustive because the Telegra.ph API may
/// introduce additional error codes in the future.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ApiError {
    /// The supplied access token is invalid.
    AccessTokenInvalid,
    /// The requested page could not be found.
    PageNotFound,
    /// The supplied content exceeds the maximum size accepted by the API.
    ContentTooBig,
    /// An API error code that is not currently recognized by Rustiograph.
    ///
    /// The original error code returned by the API is preserved.
    Unknown(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessTokenInvalid => f.write_str("ACCESS_TOKEN_INVALID"),
            Self::PageNotFound => f.write_str("PAGE_NOT_FOUND"),
            Self::ContentTooBig => f.write_str("CONTENT_TOO_BIG"),
            Self::Unknown(s) => f.write_str(s),
        }
    }
}

impl RustiographError {
    /// Converts a Telegra.ph API error code into a [`RustiographError`].
    ///
    /// Known API error codes are converted to their corresponding
    /// [`ApiError`] variants. `FLOOD_WAIT_<seconds>` responses are converted
    /// into [`RustiographError::FloodWait`].
    ///
    /// Unknown error codes are preserved as [`ApiError::Unknown`].
    pub(crate) fn from_api(msg: &str) -> Self {
        if let Some(secs) = msg.strip_prefix("FLOOD_WAIT_").and_then(|s| s.parse().ok()) {
            return Self::FloodWait {
                retry_after: Duration::from_secs(secs),
            };
        }
        Self::Api(match msg {
            "ACCESS_TOKEN_INVALID" => ApiError::AccessTokenInvalid,
            "PAGE_NOT_FOUND" => ApiError::PageNotFound,
            "CONTENT_TOO_BIG" => ApiError::ContentTooBig,
            other => ApiError::Unknown(other.to_owned()),
        })
    }

    /// Returns the duration after which a rate-limited request may be retried.
    ///
    /// Returns `Some` only for [`RustiographError::FloodWait`] errors.
    /// Returns `None` for all other error variants.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use rustiograph::error::RustiographError;
    ///
    /// let error = RustiographError::FloodWait {
    ///     retry_after: Duration::from_secs(5),
    /// };
    ///
    /// assert_eq!(error.retry_after(), Some(Duration::from_secs(5)));
    /// ```
    #[must_use]
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::FloodWait { retry_after } => Some(*retry_after),
            _ => None,
        }
    }
}
