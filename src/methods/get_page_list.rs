//! Parameters for the Telegra.ph `getPageList` API method.
//!
//! This module provides [`GetPageList`], the request parameters used to
//! retrieve a paginated list of pages associated with the current
//! Telegra.ph account.
//!
//! The [`GetPageList`] type is re-exported from the parent [`methods`] module
//! for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

/// Parameters for the Telegraph [`getPageList`] method.
///
/// Returns the pages that belong to the account the request is authorized
/// with. On success, Telegraph returns a `PageList` object with the total
/// number of pages and the requested slice of them, sorted by creation
/// date from newest to oldest.
///
/// Optional fields that are `None` are not serialized, so Telegraph uses
/// its default values for them.
///
/// [`getPageList`]: https://telegra.ph/api#getPageList
///
/// # Examples
///
/// ```ignore
/// // First 50 pages.
/// let params = GetPageList {
///     offset: None,
///     limit: Some(50),
/// };
///
/// // Next 50 pages.
/// let params = GetPageList {
///     offset: Some(50),
///     limit: Some(50),
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GetPageList {
    /// Sequential number of the first page to be returned.
    ///
    /// Optional. Telegraph's default is `0`, which is the newest page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Maximum number of pages to return.
    ///
    /// Optional. Telegraph accepts values from 0 to 200, and the default
    /// is `50`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
