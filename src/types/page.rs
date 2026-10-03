//! The Telegraph page object.
//!
//! [`Page`] is returned by `createPage`, `editPage` and `getPage`, and
//! is an item of [`PageList`](super::page_list::PageList). It also
//! provides helpers for rendering the content as HTML and for parsing
//! the page path into a [`PagePath`].

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

use crate::bounded::{AuthorName, AuthorUrl};
use crate::types::node::Node;
use crate::utils::html::nodes_to_html;

static PATH_PATTERN: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^(?P<name>\S+)-(?P<month>\d{2})-(?P<day>\d{2})(?:-(?P<number>\d))?$")
        .expect("Invalid PATH_PATTERN")
});

/// A Telegraph page.
///
/// Returned by the [`createPage`], [`editPage`] and [`getPage`] methods,
/// and as an item of [`PageList`](crate::types::page_list::PageList)
/// returned by [`getPageList`].
///
/// Telegraph fills in only some fields depending on the method: for
/// example, `content` is present only if `return_content` was set to
/// `true`, and `can_edit` is returned only when the request is
/// authorized.
///
/// [`createPage`]: https://telegra.ph/api#createPage
/// [`editPage`]: https://telegra.ph/api#editPage
/// [`getPage`]: https://telegra.ph/api#getPage
/// [`getPageList`]: https://telegra.ph/api#getPageList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Path of the page, for example `Sample-Page-12-15`.
    ///
    /// This is the part of the page URL after `telegra.ph/`. Use it to
    /// identify the page in `editPage`, `getPage` and `getViews`.
    pub path: String,

    /// Full URL of the page, for example
    /// `https://telegra.ph/Sample-Page-12-15`.
    pub url: String,

    /// Title of the page.
    pub title: String,

    /// Description of the page, if Telegraph provides one.
    ///
    /// It is made from the beginning of the page text.
    pub description: Option<String>,

    /// Name of the author, displayed under the title of the page.
    pub author_name: Option<AuthorName>,

    /// Link opened when readers click the author name.
    pub author_url: Option<AuthorUrl>,

    /// URL of the first image on the page, if there is one.
    pub image_url: Option<String>,
    #[serde(default)]

    /// Content of the page as a list of nodes.
    ///
    /// Empty if the content was not requested (`return_content` is
    /// `false`), and also if the page itself is empty. The two cases
    /// cannot be told apart by this field.
    pub content: Vec<Node>,

    /// Number of views of the page.
    ///
    /// Counted for all time. Use `getViews` to get views for a
    /// specific period. Defaults to `0` if the field is missing.
    #[serde(default)]
    pub views: i64,

    /// Whether the account that made the request can edit the page.
    ///
    /// Returned only for authorized requests. Defaults to `false` if the
    /// field is missing, which also means "unknown" for unauthorized
    /// requests.
    #[serde(default)]
    pub can_edit: bool,
}

impl Page {
    /// Renders the page content as an HTML string.
    ///
    /// Converts the whole tree of [`content`](Page::content) nodes
    /// to HTML, one after another.
    ///
    /// # Errors
    ///
    /// Returns an error message if `content` is empty. This happens when
    /// the page was fetched without `return_content: true`, and also when
    /// the page itself has no content. The two cases cannot be told apart.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let page = client.get_page("Sample-Page-12-15", true).await?;
    /// let html = page.html_content()?;
    /// ```
    pub fn html_content(&self) -> Result<String, String> {
        if self.content.is_empty() {
            return Err("Content is not available".to_string());
        }
        Ok(nodes_to_html(&self.content))
    }

    /// Splits [`path`](Page::path) into its parts.
    ///
    /// Returns `None` if the path does not match `PATH_PATTERN` or if the
    /// month or day is not a number. A missing or unparsable numeric
    /// suffix is treated as `0`.
    fn parse_path(&self) -> Option<PagePath> {
        let path = self.path.as_ref();
        let caps = PATH_PATTERN.captures(path)?;

        Some(PagePath {
            name: caps.name("name")?.as_str().to_owned(),
            month: caps.name("month")?.as_str().parse().ok()?,
            day: caps.name("day")?.as_str().parse().ok()?,
            number: caps
                .name("number")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0),
        })
    }

    /// Parses the page path into a [`PagePath`].
    ///
    /// Telegraph builds the path from the title and the creation date:
    /// `Title-words-MM-DD`, optionally followed by `-N` when a page with
    /// the same title and date already exists.
    ///
    /// Returns `None` if the path does not have this form.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // path: "Sample-Page-12-15-2"
    /// let parsed = page.parsed_path().unwrap();
    /// assert_eq!(parsed.name, "Sample-Page");
    /// assert_eq!(parsed.month, 12);
    /// assert_eq!(parsed.day, 15);
    /// assert_eq!(parsed.number, 2);
    /// ```
    #[must_use]
    pub fn parsed_path(&self) -> Option<PagePath> {
        self.parse_path()
    }
}

/// Parts of a Telegraph page path.
///
/// A path looks like `Sample-Page-12-15` or `Sample-Page-12-15-2`: the
/// title words joined with hyphens, then the month and the day, then an
/// optional number. Get it from [`Page::parsed_path`] or build it by hand
/// and turn it back into a path with [`stringify`](PagePath::stringify).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagePath {
    /// Title part of the path, with words joined by hyphens, for example
    /// `Sample-Page`.
    pub name: String,

    /// Day of the month, as it appears in the path.
    ///
    /// Not checked to be a valid day.
    pub day: u32,

    /// Month, as it appears in the path. In the path itself the month
    /// comes before the day.
    ///
    /// Not checked to be a valid month.
    pub month: u32,

    /// Numeric suffix of the path.
    ///
    /// `0` means there is no suffix.
    pub number: u32,
}

impl PagePath {
    /// Restores a readable title from the name by replacing hyphens with
    /// spaces.
    ///
    /// The result is approximate: the path does not keep the original
    /// punctuation, so the title may differ from the real
    /// [`Page::title`], and hyphens that were part of the title also
    /// become spaces.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // name: "Sample-Page"
    /// assert_eq!(path.title(), "Sample Page");
    /// ```
    #[must_use]
    pub fn title(&self) -> String {
        self.name.replace('-', " ")
    }

    /// Builds a page path from the parts.
    ///
    /// Whitespace in `name` is replaced with hyphens. The month and day
    /// are written with two digits, and `number` is appended only when
    /// it is greater than `0`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let path = PagePath {
    ///     name: "Sample Page".into(),
    ///     day: 5,
    ///     month: 3,
    ///     number: 0,
    /// };
    /// assert_eq!(path.stringify(), "Sample-Page-03-05");
    /// ```
    #[must_use]
    pub fn stringify(&self) -> String {
        let mut result = self.name.split_whitespace().collect::<Vec<_>>().join("-");

        let _ = write!(result, "-{:02}-{:02}", self.month, self.day);

        if self.number > 0 {
            let _ = write!(result, "-{}", self.number);
        }

        result
    }
}
