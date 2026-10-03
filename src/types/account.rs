//! Account types.
//!
//! [`Account`] is the object that Telegraph returns from the account
//! methods: `createAccount`, `editAccountInfo`, `getAccountInfo` and
//! `revokeAccessToken`. Telegraph fills in only the fields relevant to
//! the method, so most fields are optional.
//!
//! [`AccountField`] lists the account fields that can be requested from
//! `getAccountInfo`.

use serde::{Deserialize, Serialize};

use crate::bounded::{AuthorName, AuthorUrl};

/// A Telegraph account.
///
/// Returned by the [`createAccount`], [`editAccountInfo`], [`getAccountInfo`]
/// and [`revokeAccessToken`] methods.
///
/// Most fields are optional: Telegraph fills in only those that are
/// relevant for the method, and for `getAccountInfo` only those listed
/// in the `fields` parameter.
///
/// [`createAccount`]: https://telegra.ph/api#createAccount
/// [`editAccountInfo`]: https://telegra.ph/api#editAccountInfo
/// [`getAccountInfo`]: https://telegra.ph/api#getAccountInfo
/// [`revokeAccessToken`]: https://telegra.ph/api#revokeAccessToken
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    /// Account name, shown to the user only inside the Telegraph
    /// interface. Telegraph does not show it to readers of your pages.
    pub short_name: String,

    /// Default author name used when creating new pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<AuthorName>,

    /// Default profile link, opened when readers click the author name
    /// under the page title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<AuthorUrl>,

    /// Access token of the account.
    ///
    /// Only returned by `createAccount` and `revokeAccessToken`. Keep it
    /// secret: anyone who has it can manage the account and its pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

    /// URL for authorizing a browser on telegra.ph and connecting it to
    /// this account.
    ///
    /// The link is valid for a single use and for a short time only
    /// (5 minutes according to the Telegraph docs). Telegraph returns it
    /// from `createAccount` and `revokeAccessToken`, and from
    /// `getAccountInfo` when `auth_url` is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,

    /// Number of pages that belong to the account.
    ///
    /// Only present if requested through [`AccountField::PageCount`].
    pub page_count: Option<i64>,
}

/// Account fields that can be requested from `getAccountInfo`.
///
/// Each variant corresponds to a field of [`Account`] and to the name
/// that Telegraph expects in the `fields` parameter: `short_name`,
/// `author_name`, `author_url`, `auth_url` and `page_count`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountField {
    /// The `short_name` field.
    ShortName,
    /// The `author_name` field.
    AuthorName,
    /// The `author_url` field.
    AuthorUrl,
    /// The `auth_url` field (a short-lived, single-use login link).
    AuthUrl,
    /// The `page_count` field.
    PageCount,
}
