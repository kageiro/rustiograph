//! Parameters for the Telegra.ph `editAccountInfo` API method.
//!
//! This module provides [`EditAccountInfo`], the request parameters used to
//! update the information associated with the current Telegra.ph account.
//!
//! The [`EditAccountInfo`] type is re-exported from the parent [`methods`]
//! module for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

use crate::bounded::{AuthorName, AuthorUrl, ShortName};

/// Parameters for the Telegraph [`editAccountInfo`] method.
///
/// Updates the information of the account that the request is authorized
/// with. Only the fields you want to change need to be set. On success,
/// Telegraph returns the updated `Account` object.
///
/// Optional fields that are `None` are not serialized, so they are omitted
/// from the request and the corresponding values on the account stay
/// unchanged.
///
/// [`editAccountInfo`]: https://telegra.ph/api#editAccountInfo
///
/// # Examples
///
/// ```ignore
/// let params = EditAccountInfo {
///     short_name: ShortName::new("Sandbox")?,
///     author_name: Some(AuthorName::new("New author")?),
///     author_url: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct EditAccountInfo {
    /// New account name, shown to the user only inside the Telegraph
    /// interface. Telegraph does not show it to readers of your pages.
    ///
    /// Length is 1–32 characters (enforced by [`ShortName`]).
    pub short_name: ShortName,

    /// New default author name, used when creating new pages.
    ///
    /// Optional. Length is 0–128 characters (enforced by [`AuthorName`]).
    /// Pages that already exist are not affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,

    /// New default profile link, opened when readers click the author name
    /// under the page title. It can be any link, not necessarily a
    /// Telegram profile or channel.
    ///
    /// Optional. Length is 0–512 characters (enforced by [`AuthorUrl`]).
    /// Pages that already exist are not affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<AuthorUrl>,
}
