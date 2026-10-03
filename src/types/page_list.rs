//! The list of an account's pages.
//!
//! [`PageList`] is returned by `getPageList`. It contains one slice of
//! the account's pages and their total count, which is used for
//! pagination.

use crate::types::page::Page;
use serde::{Deserialize, Serialize};

/// A list of pages that belong to an account.
///
/// Returned by the [`getPageList`] method. It holds one slice of the
/// account's pages, selected with `offset` and `limit`, together with the
/// total number of pages, so you can request the remaining slices.
///
/// Pages are sorted by creation date, from newest to oldest. Pages in
/// the list are returned without `content`, so [`Page::content`] is
/// empty for every item. To get the content, fetch the page with
/// `getPage` and `return_content: true`.
///
/// [`getPageList`]: https://telegra.ph/api#getPageList
///
/// # Examples
///
/// Checking whether there are more pages to load:
///
/// ```ignore
/// let list: PageList = client.get_page_list(Some(0), Some(50)).await?;
/// let has_more = (list.pages.len() as i64) < list.total_count;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageList {
    /// Total number of pages in the account.
    ///
    /// This is the number of all pages, not the size of the current
    /// slice. Defaults to `0` if the field is missing.
    #[serde(default)]
    pub total_count: i64,

    /// Pages of the requested slice, from newest to oldest.
    ///
    /// Its length is at most the requested `limit`. Defaults to an empty
    /// list if the field is missing.
    #[serde(default)]
    pub pages: Vec<Page>,
}
