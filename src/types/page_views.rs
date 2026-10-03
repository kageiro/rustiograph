//! Page view statistics.
//!
//! [`PageViews`] is returned by `getViews` and contains the number of
//! views of a page for the requested period.

use serde::{Deserialize, Serialize};

/// The number of views of a page.
///
/// Returned by the [`getViews`] method. Depending on the time fields
/// passed to the method, it holds the views for all time or for a
/// specific year, month, day or hour.
///
/// [`getViews`]: https://telegra.ph/api#getViews
///
/// # Examples
///
/// ```ignore
/// let stats: PageViews = client.get_views("Sample-Page-12-15", None).await?;
/// println!("views: {}", stats.views);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageViews {
    /// Number of views of the page for the requested period.
    ///
    /// If no time fields were passed, this is the total for all time.
    /// Defaults to `0` if the field is missing.
    #[serde(default)]
    pub views: i64,
}
