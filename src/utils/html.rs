//! Utilities for converting between HTML, [`Node`] values, and the JSON
//! representation expected by the Telegra.ph API.
//!
//! The conversion pipeline is:
//!
//! - HTML → [`Node`] values with [`html_to_nodes`]
//! - [`Node`] values → HTML with [`node_to_html`] and [`nodes_to_html`]
//! - [`Node`] values → JSON with [`nodes_to_json`]
//! - HTML → JSON with [`html_to_json`]
//!
//! HTML input is restricted to the tags and attributes defined in
//! [`ALLOWED_TAGS`] and [`ALLOWED_ATTRS`]. Unsupported tags result in an
//! error, while HTML comments and whitespace-only text nodes are ignored.

use html_escape::{encode_quoted_attribute, encode_text};
use serde_json::Value;
use std::collections::HashMap;
use tl::{Node as TlNode, ParserOptions};

use crate::consts::{ALLOWED_ATTRS, ALLOWED_TAGS, VOID_ELEMENTS};
use crate::types::node::{Node, NodeElement};

/// Converts an HTML document into a vector of [`Node`] values.
///
/// Only tags listed in [`ALLOWED_TAGS`] and attributes listed in
/// [`ALLOWED_ATTRS`] are preserved. HTML comments and whitespace-only text
/// nodes are discarded.
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed or if it contains a tag
/// that is not allowed.
///
/// # Example
///
/// ```
/// # use rustiograph::utils::html::html_to_nodes;
///
/// let nodes = html_to_nodes("<p>Hello <b>world</b></p>")?;
///
/// # Ok::<(), String>(())
/// ```
pub fn html_to_nodes(html_content: &str) -> Result<Vec<Node>, String> {
    let dom = tl::parse(html_content, ParserOptions::default())
        .map_err(|e| format!("HTML Parse Error: {e:?}"))?;

    let parser = dom.parser();
    let mut result = Vec::new();

    for handle in dom.children() {
        if let Some(node) = convert_node(handle.get(parser), parser)? {
            result.push(node);
        }
    }

    Ok(result)
}

/// Converts a parsed [`tl::Node`] into a [`Node`].
///
/// Unsupported nodes, such as HTML comments, are ignored. Unsupported HTML
/// tags result in an error.
fn convert_node(node: Option<&tl::Node>, parser: &tl::Parser) -> Result<Option<Node>, String> {
    let Some(node) = node else { return Ok(None) };

    match node {
        TlNode::Raw(bytes) => {
            let text = String::from_utf8_lossy(bytes.as_bytes()).to_string();
            if text.trim().is_empty() {
                Ok(None)
            } else {
                Ok(Some(Node::Text(text)))
            }
        }
        TlNode::Tag(tag) => {
            let tag_name = tag.name().as_utf8_str().to_string();

            if !ALLOWED_TAGS.contains(&tag_name.as_str()) {
                return Err(format!("{tag_name} tag is not allowed"));
            }

            let mut attrs = HashMap::new();

            for (key, value) in tag.attributes().iter() {
                let key_str = key.as_ref();

                if ALLOWED_ATTRS.contains(&key_str)
                    && let Some(val) = value
                {
                    attrs.insert(key_str.to_owned(), val.as_ref().to_owned());
                }
            }

            let mut children = Vec::new();

            for child_handle in tag.children().top().iter() {
                if let Some(child_node) = convert_node(child_handle.get(parser), parser)? {
                    children.push(child_node);
                }
            }

            Ok(Some(Node::Element(NodeElement {
                tag: tag_name,
                attrs,
                children,
            })))
        }
        &TlNode::Comment(_) => Ok(None),
    }
}

/// Converts a [`Node`] into an HTML string.
///
/// Text content and attribute values are HTML-escaped before being inserted
/// into the resulting string. Void elements are emitted without a closing
/// tag.
///
/// # Example
///
/// ```
/// # use rustiograph::types::node::Node;
/// # use rustiograph::utils::html::node_to_html;
///
/// let node = Node::Text("Hello & world".into());
///
/// assert_eq!(node_to_html(&node), "Hello &amp; world");
/// ```
#[must_use]
pub fn node_to_html(node: &Node) -> String {
    match node {
        Node::Text(text) => encode_text(text).to_string(),

        Node::Element(el) => {
            let mut result = format!("<{}", el.tag);

            for (key, value) in &el.attrs {
                result.push(' ');
                result.push_str(key);
                result.push_str("=\"");
                result.push_str(&encode_quoted_attribute(value));
                result.push('"');
            }

            if VOID_ELEMENTS.contains(&el.tag.as_str()) {
                result.push_str("/>");
            } else {
                result.push('>');

                for child in &el.children {
                    result.push_str(&node_to_html(child));
                }

                result.push_str("</");
                result.push_str(&el.tag);
                result.push('>');
            }

            result
        }
    }
}

/// Converts a slice of [`Node`] values into an HTML string.
///
/// Nodes are serialized in their original order without adding any
/// additional separators between them.
///
/// # Example
///
/// ```
/// # use rustiograph::types::node::Node;
/// # use rustiograph::utils::html::nodes_to_html;
///
/// let nodes = vec![
/// Node::Text("Hello".into()),
/// Node::Text(" world".into()),
/// ];
///
/// assert_eq!(nodes_to_html(&nodes), "Hello world");
/// ```
pub fn nodes_to_html(nodes: &[Node]) -> String {
    nodes.iter().map(node_to_html).collect()
}

/// Converts a slice of [`Node`] values into the JSON representation used by
/// the Telegra.ph API.
///
/// Text nodes are represented as JSON strings. Element nodes are represented
/// as JSON objects containing a `tag` field and, when present, `attrs` and
/// `children` fields.
///
/// # Errors
///
/// Returns an error if a child node cannot be converted to JSON.
pub fn nodes_to_json(nodes: &[Node]) -> Result<Vec<Value>, String> {
    nodes
        .iter()
        .map(|node| match node {
            Node::Text(text) => Ok(Value::String(text.clone())),
            Node::Element(el) => {
                let mut map = serde_json::Map::new();
                map.insert("tag".to_string(), Value::String(el.tag.clone()));

                if !el.attrs.is_empty() {
                    let attrs: serde_json::Map<String, Value> = el
                        .attrs
                        .iter()
                        .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                        .collect();
                    map.insert("attrs".to_string(), Value::Object(attrs));
                }

                if !el.children.is_empty() {
                    let children = nodes_to_json(&el.children)?;
                    map.insert("children".to_string(), Value::Array(children));
                }

                Ok(Value::Object(map))
            }
        })
        .collect()
}
/// Converts HTML content directly into the JSON representation used by the
/// Telegra.ph API.
///
/// This is a convenience function equivalent to calling [`html_to_nodes`]
/// followed by [`nodes_to_json`].
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed or contains an unsupported
/// tag.
pub fn html_to_json(content: &str) -> Result<Vec<Value>, String> {
    let nodes = html_to_nodes(content)?;
    nodes_to_json(&nodes)
}
