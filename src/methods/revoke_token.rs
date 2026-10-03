//! Parameters for the Telegra.ph `revokeAccessToken` API method.
//!
//! This module provides [`RevokeToken`], the client-side parameters used when
//! revoking the current Telegra.ph access token.
//!
//! The API method itself does not accept request parameters other than the
//! access token supplied by the client. [`RevokeToken`] therefore contains
//! only client-side configuration and is not serialized into the request
//! body.
//!
//! The [`RevokeToken`] type is re-exported from the parent [`methods`] module
//! for convenient access.
//!
//! [`methods`]: crate::methods

/// Parameters for the Telegraph [`revokeAccessToken`] method.
///
/// Revokes the current access token of the account and generates a new
/// one. Use it to reset all connected sessions, or if you have reason to
/// believe the token was compromised. On success, Telegraph returns the
/// `Account` object with the new `access_token` and `auth_url` fields.
///
/// After a successful call the old token stops working, so the new one
/// must be saved and used for all following requests.
///
/// This method has no parameters other than the access token, which is
/// supplied by the client, so nothing from this struct is sent as a
/// request parameter. That is why the struct does not derive `Serialize`.
///
/// [`revokeAccessToken`]: https://telegra.ph/api#revokeAccessToken
///
/// # Examples
///
/// ```ignore
/// let params = RevokeToken { auth: false };
/// ```
#[derive(Debug, Clone)]
pub struct RevokeToken {
    /// Whether to store the newly generated access token in the client.
    ///
    /// If `true`, the token returned by the API replaces the current token
    /// stored in [`Rustiograph`](crate::api::Rustiograph). If `false`, the returned token is available
    /// through the returned [`Account`](crate::types::Account) but is not stored automatically.
    pub auth: bool,
}
