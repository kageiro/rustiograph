//! Parameters for the Telegra.ph `getPage` API method.
//!
//! This module provides [`GetPage`], the request parameters used to retrieve
//! a Telegra.ph page by its path.
//!
//! Unlike most Telegra.ph API methods, `getPage` does not require
//! authentication.
//!
//! The [`GetPage`] type is re-exported from the parent [`methods`] module
//! for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

/// Parameters for the Telegraph [`getPage`] method.
///
/// Fetches a page by its path. On success, Telegraph returns the `Page`
/// object. Unlike most other methods, this one does not require
/// authorization, so no access token is needed.
///
/// [`getPage`]: https://telegra.ph/api#getPage
///
/// # Examples
///
/// ```ignore
/// let params = GetPage {
///     path: "Sample-Page-12-15",
///     return_content: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GetPage<'a> {
    /// Path of the page to fetch, for example `Sample-Page-12-15`.
    ///
    /// Required. This is the part of the page URL after `telegra.ph/`.
    pub path: &'a str,

    /// Whether the `content` field should be included in the returned
    /// `Page` object.
    ///
    /// If `false`, the response contains only the page metadata
    /// (path, URL, title and so on). Telegraph's default is `false`, so
    /// set this to `true` if you need the page body.
    pub return_content: bool,
}
