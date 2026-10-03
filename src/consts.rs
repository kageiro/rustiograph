//! Constants used throughout Rustiograph.
//!
//! This module contains the supported HTML elements and attributes used when
//! converting HTML to Telegra.ph nodes, as well as the default client
//! configuration.

use std::time::Duration;

/// HTML tags supported by the Telegra.ph content format.
///
/// Tags not included in this list are rejected when HTML is converted into
/// [`crate::types::node::Node`] values.
pub const ALLOWED_TAGS: &[&str] = &[
    "a",
    "aside",
    "b",
    "blockquote",
    "br",
    "code",
    "em",
    "figcaption",
    "figure",
    "h3",
    "h4",
    "hr",
    "i",
    "iframe",
    "img",
    "li",
    "ol",
    "p",
    "pre",
    "s",
    "strong",
    "u",
    "ul",
    "video",
];

/// HTML void elements that do not have closing tags.
///
/// These elements are serialized without a closing tag when converting
/// [`crate::types::node::Node`] values back to HTML.
///
/// The list follows the HTML void-element set rather than
/// [`ALLOWED_TAGS`], so it may contain elements that are not currently
/// accepted as Telegra.ph content.
pub const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "menuitem",
    "meta", "param", "source", "track", "wbr",
];

/// HTML attributes supported by the Telegra.ph content format.
///
/// Attributes not included in this list are ignored when converting HTML
/// into [`crate::types::node::Node`] values.
pub const ALLOWED_ATTRS: &[&str] = &["href", "src"];

/// Default Telegra.ph service hostname.
pub const SERVICE_URL: &str = "telegra.ph";

/// Default timeout for HTTP requests made by Rustiograph.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// Default `User-Agent` sent with HTTP requests.
pub const DEFAULT_USER_AGENT: &str = "RustioGraph";
