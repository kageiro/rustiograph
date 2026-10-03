//! Parameters for the Telegra.ph `createAccount` API method.
//!
//! This module provides [`CreateAccount`], the request parameters used to
//! create a new Telegra.ph account.
//!
//! The [`CreateAccount`] type is re-exported from the parent [`methods`]
//! module for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

use crate::bounded::{AuthorName, AuthorUrl, ShortName};

/// Parameters for the Telegraph [`createAccount`] method.
///
/// Creates a new Telegraph account. The account is used to publish and
/// edit pages, and the `short_name` and author fields are used as default
/// values for pages created under it.
///
/// Optional fields that are `None` are not serialized, so they are omitted
/// from the request entirely.
///
/// [`createAccount`]: https://telegra.ph/api#createAccount
///
/// # Examples
///
/// ```ignore
/// let params = CreateAccount {
///     short_name: ShortName::new("Sandbox")?,
///     author_name: Some(AuthorName::new("Anonymous")?),
///     author_url: None,
///     auth: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct CreateAccount {
    /// Account name, shown to the user only inside the Telegraph interface.
    /// Telegraph does not show it to readers of your pages.
    ///
    /// Required. Length is 1–32 characters (enforced by [`ShortName`]).
    pub short_name: ShortName,

    /// Default author name for pages created with this account.
    /// It is displayed on the page under the title.
    ///
    /// Optional. Length is 0–128 characters (enforced by [`AuthorName`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,

    /// Default profile link for pages created with this account.
    /// It is opened when readers click the author name. It can be any
    /// link, not necessarily a Telegram profile or channel.
    ///
    /// Optional. Length is 0–512 characters (enforced by [`AuthorUrl`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<AuthorUrl>,

    /// Whether to store the newly generated access token in the client.
    ///
    /// If `true`, the token returned by the API replaces the current token
    /// stored in [`Rustiograph`](crate::api::Rustiograph). If `false`, the returned token is available
    /// through the returned [`Account`](crate::types::Account) but is not stored automatically.
    pub auth: bool,
}
