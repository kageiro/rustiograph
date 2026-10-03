//! Types used to represent Telegra.ph page content.
//!
//! This module provides [`ContentInput`], which allows page content to be
//! supplied either as an HTML string or as a collection of [`Node`] values.
//!
//! [`ContentInput`] implements [`serde::Serialize`] by converting the input
//! into the JSON node representation expected by the Telegra.ph API.

use serde::ser::Error as _;
use serde::{Serialize, Serializer};

use crate::types::node::Node;
use crate::utils::html::{html_to_json, nodes_to_json};

/// Input content for a Telegra.ph page.
///
/// Content can be provided either as an HTML string or as a collection of
/// [`Node`] values. Both representations are converted into the JSON
/// structure expected by the Telegra.ph API when serialized.
///
/// # Examples
///
/// Using an HTML string:
///
/// ```
/// # use rustiograph::types::content::ContentInput;
///
/// let content = ContentInput::from("<p>Hello, world!</p>");
/// ```
///
/// Using [`Node`] values:
///
/// ```
/// # use rustiograph::types::content::ContentInput;
/// # use rustiograph::types::node::Node;
///
/// let content = ContentInput::from(vec![
///     Node::Text("Hello, world!".into()),
/// ]);
/// ```
///
/// [`ContentInput`] also implements [`From`] for `&str`, [`String`], and
/// `Vec<Node>`, allowing it to be passed directly to APIs accepting
/// `impl Into<ContentInput>`.
#[derive(Debug, Clone)]
pub enum ContentInput {
    /// HTML content that will be parsed and converted into Telegra.ph nodes.
    ///
    /// The HTML must contain only tags and attributes supported by
    /// Rustiograph. Unsupported tags result in a serialization error.
    Html(String),

    /// Content represented directly as Telegra.ph [`Node`] values.
    ///
    /// This variant skips HTML parsing and converts the supplied nodes
    /// directly into the JSON representation expected by the API.
    Nodes(Vec<Node>),
}

impl Serialize for ContentInput {
    /// Serializes the content into the JSON string format expected by
    /// Telegra.ph.
    ///
    /// [`ContentInput::Html`] is first parsed into nodes and then converted
    /// to JSON. [`ContentInput::Nodes`] is converted directly to JSON.
    ///
    /// The resulting JSON array is serialized as a string rather than as a
    /// nested JSON value because this is the format required by the
    /// Telegra.ph API for page content.
    ///
    /// # Errors
    ///
    /// Returns a serializer error if the HTML cannot be parsed, the supplied
    /// nodes cannot be converted, or the resulting JSON cannot be serialized.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self {
            ContentInput::Html(html) => html_to_json(html),
            ContentInput::Nodes(nodes) => nodes_to_json(nodes),
        }
        .map_err(S::Error::custom)?;

        let json = serde_json::to_string(&value).map_err(S::Error::custom)?;
        serializer.serialize_str(&json)
    }
}

impl From<String> for ContentInput {
    /// Converts an owned HTML string into [`ContentInput::Html`].
    fn from(s: String) -> Self {
        ContentInput::Html(s)
    }
}

impl From<&str> for ContentInput {
    /// Converts a string slice into [`ContentInput::Html`].
    fn from(s: &str) -> Self {
        ContentInput::Html(s.to_owned())
    }
}

impl From<Vec<Node>> for ContentInput {
    /// Converts a collection of [`Node`] values into [`ContentInput::Nodes`].
    fn from(nodes: Vec<Node>) -> Self {
        ContentInput::Nodes(nodes)
    }
}
