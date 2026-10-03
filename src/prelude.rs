//! Convenient re-exports of commonly used `RustioGraph` types.
//!
//! This module provides a small set of commonly used types so applications
//! can import the most frequently needed parts of `RustioGraph` with a single
//! `use` statement.
//!
//! The prelude is intentionally limited to commonly used public types.
//! Less frequently used types can be imported from their respective modules.

pub use crate::{api::Rustiograph, api::RustiographBuilder, content::ContentInput};

pub use crate::error::{ApiError, RustiographError};

pub use crate::methods::{
    CreateAccount, CreatePage, EditAccountInfo, EditPage, GetAccountInfo, GetPage, GetPageList,
    GetViews, RevokeToken,
};

pub use crate::methods::Method;
pub use crate::result::Result;

pub use crate::bounded::{AuthorName, AuthorUrl, ShortName, Title};
pub use crate::types::{Account, Node, Page, PageList, PageViews};
