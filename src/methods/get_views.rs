//! Parameters for the Telegra.ph `getViews` API method.
//!
//! This module provides [`GetViews`], the request parameters used to retrieve
//! the number of views for a Telegra.ph page.
//!
//! The [`GetViews`] type is re-exported from the parent [`methods`] module
//! for convenient access.
//!
//! [`methods`]: crate::methods

use serde::Serialize;

/// Parameters for the Telegraph [`getViews`] method.
///
/// Returns the number of views for a page. On success, Telegraph returns
/// a `PageViews` object with a single `views` field.
///
/// The time fields narrow the period for which views are counted. If none
/// of them is set, the total number of views for all time is returned.
/// Fields must be given from the largest unit to the smallest: `month`
/// requires `year`, `day` requires `month`, and `hour` requires `day`.
///
/// Optional fields that are `None` are not serialized, so they are omitted
/// from the request entirely.
///
/// [`getViews`]: https://telegra.ph/api#getViews
///
/// # Examples
///
/// ```ignore
/// // Total views for all time.
/// let params = GetViews {
///     path: "Sample-Page-12-15",
///     year: None,
///     month: None,
///     day: None,
///     hour: None,
/// };
///
/// // Views for 15 December 2016, 13:00.
/// let params = GetViews {
///     path: "Sample-Page-12-15",
///     year: Some(2016),
///     month: Some(12),
///     day: Some(15),
///     hour: Some(13),
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GetViews<'a> {
    /// Path of the page, for example `Sample-Page-12-15`.
    ///
    /// Required. This is the part of the page URL after `telegra.ph/`.
    pub path: &'a str,

    /// Year to count views for.
    ///
    /// Optional. Required if `month` is set. Telegraph accepts values
    /// from 2000 to 2100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,

    /// Month to count views for.
    ///
    /// Optional. Required if `day` is set. Values are from 1 to 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u32>,

    /// Day to count views for.
    ///
    /// Optional. Required if `hour` is set. Values are from 1 to 31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u32>,

    /// Hour to count views for.
    ///
    /// Optional. Values are from 0 to 24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<u32>,
}
