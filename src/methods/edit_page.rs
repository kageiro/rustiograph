//! Parameters for the Telegra.ph `editPage` API method.
//!
//! This module provides [`EditPage`], the request parameters used to update
//! an existing Telegra.ph page.
//!
//! The page path is kept separately from the serialized request body because
//! it is used as part of the API endpoint rather than as a request field.
//!
//! The [`EditPage`] type is re-exported from the parent [`methods`] module
//! for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

use crate::bounded::{AuthorName, AuthorUrl, Title};
use crate::content::ContentInput;

/// Parameters for the Telegraph [`editPage`] method.
///
/// Edits an existing page of the account that the request is authorized
/// with. The page is replaced with the values passed here, so `title` and
/// `content` must always be provided, even if only one of them changes.
/// On success, Telegraph returns the updated `Page` object.
///
/// Optional fields that are `None` are not serialized, so they are omitted
/// from the request entirely.
///
/// [`editPage`]: https://telegra.ph/api#editPage
///
/// # Examples
///
/// ```ignore
/// let params = EditPage {
///     path: "Sample-Page-12-15",
///     title: Title::new("Updated title")?,
///     content: content,
///     author_name: None,
///     author_url: None,
///     return_content: false,
///     as_user: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct EditPage<'a> {
    /// Path of the page to edit, for example `Sample-Page-12-15`.
    ///
    /// Required. This is the part of the page URL after `telegra.ph/`.
    /// It is not serialized into the request body (`#[serde(skip)]`).
    #[serde(skip)]
    pub path: &'a str,

    /// New page title.
    ///
    /// Required. Length is 1–256 characters (enforced by [`Title`]).
    pub title: Title,

    /// New page content.
    ///
    /// Required. Telegraph accepts an array of content nodes, and the
    /// serialized content must not exceed 64 KB.
    pub content: ContentInput,

    /// New author name displayed on the page under the title.
    ///
    /// Optional. Length is 0–128 characters (enforced by [`AuthorName`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,

    /// New link opened when readers click the author name. It can be any
    /// link, not necessarily a Telegram profile or channel.
    ///
    /// Optional. Length is 0–512 characters (enforced by [`AuthorUrl`]).
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
    ///
    /// Client-side option: it is not serialized into the request
    /// (`#[serde(skip)]`).
    #[serde(skip)]
    pub as_user: bool,
}
