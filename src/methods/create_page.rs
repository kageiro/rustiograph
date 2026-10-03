//! Parameters for the Telegra.ph `createPage` API method.
//!
//! This module provides [`CreatePage`], the request parameters used to
//! create a new page under the account associated with the request.
//!
//! The [`CreatePage`] type is re-exported from the parent [`methods`]
//! module for convenient access.
//!
//! [`methods`]: crate::methods

use crate::{
    bounded::{AuthorName, AuthorUrl, Title},
    content::ContentInput,
};
use serde::Serialize;

/// Parameters for the Telegraph [`createPage`] method.
///
/// Creates a new page under the account that the request is authorized
/// with. On success, Telegraph returns the created `Page` object.
///
/// Optional fields that are `None` are not serialized, so they are omitted
/// from the request entirely. If an author field is omitted, Telegraph
/// uses the default value stored in the account (see `CreateAccount`).
///
/// [`createPage`]: https://telegra.ph/api#createPage
///
/// # Examples
///
/// ```ignore
/// let params = CreatePage {
///     title: Title::new("Sample page")?,
///     content: content,
///     author_name: None,
///     author_url: None,
///     return_content: false,
///     as_user: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct CreatePage {
    /// Page title.
    ///
    /// Required. Length is 1–256 characters (enforced by [`Title`]).
    pub title: Title,

    /// Page content.
    ///
    /// Required. Telegraph accepts an array of content nodes, and the
    /// serialized content must not exceed 64 KB.
    pub content: ContentInput,

    /// Author name displayed on the page under the title.
    ///
    /// Optional. Length is 0–128 characters (enforced by [`AuthorName`]).
    /// Overrides the account's default author name for this page only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,

    /// Link opened when readers click the author name. It can be any
    /// link, not necessarily a Telegram profile or channel.
    ///
    /// Optional. Length is 0–512 characters (enforced by [`AuthorUrl`]).
    /// Overrides the account's default author URL for this page only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<AuthorUrl>,

    /// Whether the `content` field should be included in the returned
    /// `Page` object.
    ///
    /// If `false`, the response contains only the page metadata
    /// (path, URL, title and so on), which makes it considerably smaller
    /// for large pages. Telegraph's default is `false`.
    pub return_content: bool,

    /// TODO
    pub as_user: bool,
}
