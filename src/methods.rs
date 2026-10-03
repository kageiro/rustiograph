//! Telegra.ph API methods and request method abstractions.
//!
//! This module contains the request parameter types used by individual
//! Telegra.ph API methods and [`Method`], which represents the API methods
//! supported by Rustiograph.
//!
//! The [`IntoMethod`] trait provides a common interface for obtaining an API
//! method name from [`Method`] values and string-like values.

pub mod create_account;
pub mod create_page;
pub mod edit_account_info;
pub mod edit_page;
pub mod get_account_info;
pub mod get_page;
pub mod get_page_list;
pub mod get_views;
pub mod revoke_token;

pub use create_account::CreateAccount;
pub use create_page::CreatePage;
pub use edit_account_info::EditAccountInfo;
pub use edit_page::EditPage;
pub use get_account_info::GetAccountInfo;
pub use get_page::GetPage;
pub use get_page_list::GetPageList;
pub use get_views::GetViews;
pub use revoke_token::RevokeToken;

/// A Telegra.ph API method supported by Rustiograph.
///
/// Each variant corresponds to a method name defined by the Telegra.ph API.
/// Use [`Method::as_str`] to obtain the exact API method name.
///
/// # Examples
///
/// ```
/// use rustiograph::methods::Method;
///
/// assert_eq!(Method::CreatePage.as_str(), "createPage");
/// assert_eq!(Method::GetViews.as_str(), "getViews");
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Method {
    /// Creates a new Telegra.ph account.
    CreateAccount,

    /// Creates a new Telegra.ph page.
    CreatePage,

    /// Updates information for an existing account.
    EditAccountInfo,

    /// Updates an existing Telegra.ph page.
    EditPage,

    /// Retrieves information about an account.
    GetAccountInfo,

    /// Retrieves a Telegra.ph page.
    GetPage,

    /// Retrieves a list of pages associated with an account.
    GetPageList,

    /// Retrieves the number of views for a page.
    GetViews,

    /// Revokes an access token.
    RevokeAccessToken,
}

impl Method {
    /// Returns the method name expected by the Telegra.ph API.
    ///
    /// The returned string is a static string corresponding to the API
    /// method represented by this value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustiograph::methods::Method;
    ///
    /// assert_eq!(Method::CreateAccount.as_str(), "createAccount");
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Method::CreateAccount => "createAccount",
            Method::CreatePage => "createPage",
            Method::EditAccountInfo => "editAccountInfo",
            Method::EditPage => "editPage",
            Method::GetAccountInfo => "getAccountInfo",
            Method::GetPage => "getPage",
            Method::GetPageList => "getPageList",
            Method::GetViews => "getViews",
            Method::RevokeAccessToken => "revokeAccessToken",
        }
    }
}

/// Provides a common interface for values that can represent an API method.
///
/// This trait is implemented for [`Method`] and string types, allowing APIs
/// that accept a method name to work with either a predefined [`Method`] or a
/// custom string.
///
/// Implementations return the method name without allocating a new `String`.
pub trait IntoMethod {
    /// Returns the API method name represented by this value.
    ///
    /// The returned string is borrowed from the implementing value whenever
    /// applicable.
    fn method_name(&self) -> &str;
}

impl IntoMethod for Method {
    fn method_name(&self) -> &str {
        self.as_str()
    }
}

impl IntoMethod for str {
    fn method_name(&self) -> &str {
        self
    }
}

impl IntoMethod for String {
    fn method_name(&self) -> &str {
        self.as_str()
    }
}

impl<T: IntoMethod + ?Sized> IntoMethod for &T {
    fn method_name(&self) -> &str {
        (**self).method_name()
    }
}
