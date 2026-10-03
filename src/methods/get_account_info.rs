//! Parameters for the Telegra.ph `getAccountInfo` API method.
//!
//! This module provides [`GetAccountInfo`], the request parameters used to
//! retrieve information about the current Telegra.ph account.
//!
//! The [`GetAccountInfo`] type is re-exported from the parent [`methods`]
//! module for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

/// Parameters for the Telegraph [`getAccountInfo`] method.
///
/// Fetches information about the account that the request is authorized
/// with. On success, Telegraph returns the `Account` object, filled in
/// only with the requested fields.
///
/// [`getAccountInfo`]: https://telegra.ph/api#getAccountInfo
///
/// # Examples
///
/// ```ignore
/// let params = GetAccountInfo {
///     fields: &["short_name", "page_count"],
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountInfo<'a> {
    /// List of account fields to return.
    ///
    /// Available fields: `short_name`, `author_name`, `author_url`,
    /// `auth_url`, `page_count`. If the list is empty, Telegraph returns
    /// its default set (`short_name`, `author_name`, `author_url`).
    ///
    /// Serialized as a JSON array of strings, for example
    /// `["short_name","page_count"]`.
    pub fields: &'a [&'a str],
}
