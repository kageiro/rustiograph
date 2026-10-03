//! Page content as a tree of nodes.
//!
//! Telegraph describes the content of a page as a list of nodes. A
//! [`Node`] is either a piece of text or a [`NodeElement`], an element
//! with a tag, attributes and child nodes. Build elements with
//! [`NodeElement::new`], which accepts only the tags Telegraph allows,
//! and nest them with [`NodeElement::add`].
//!
//! The types serialize to the JSON format that Telegraph expects and
//! can be rendered to HTML.

use crate::{consts::ALLOWED_TAGS, utils::html::node_to_html};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single node of Telegraph page content.
///
/// Page content in Telegraph is a tree of nodes. Every node is either a
/// piece of plain text or an element with a tag, attributes and child
/// nodes. This enum models exactly that, like the [`Node`] type of the
/// Telegraph API.
///
/// A whole page is a list of such nodes (a JSON array). Nesting is
/// expressed through the `children` of a [`NodeElement`], and each child
/// is again a `Node`.
///
/// # Serialization
///
/// The enum is `#[serde(untagged)]`, so no variant name appears in the
/// JSON. A text node is a bare JSON string and an element is a JSON
/// object:
///
/// ```json
/// [
///   "Text at the top level, ",
///   {
///     "tag": "p",
///     "children": [
///       "Hello, ",
///       { "tag": "b", "children": ["world"] },
///       "!"
///     ]
///   }
/// ]
/// ```
///
/// When deserializing, serde tries the variants from top to bottom: a
/// JSON string becomes [`Node::Text`], an object becomes
/// [`Node::Element`]. Keep this order if you add variants.
///
/// Telegraph limits the serialized content of a page to 64 KB.
///
/// # Examples
///
/// Nodes are usually built from strings and [`NodeElement`]s through
/// `Into<Node>`:
///
/// ```ignore
/// let text: Node = "Hello".into();
/// let element: Node = NodeElement::new("br")?.into();
/// ```
///
/// [`Node`]: https://telegra.ph/api#Node
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
    /// A text node: a string of plain text without markup.
    ///
    /// Serialized as a bare JSON string. Formatting is not interpreted
    /// inside it. To make text bold, a link and so on, wrap it in a
    /// [`NodeElement`] with the matching tag.
    Text(String),

    /// An element node: a tag with attributes and child nodes.
    ///
    /// Serialized as a JSON object. See [`NodeElement`].
    Element(NodeElement),
}

/// An element node of Telegraph page content: a tag with attributes and
/// child nodes.
///
/// Create it with [`NodeElement::new`], which checks that the tag is one
/// of the tags Telegraph accepts, then fill it with children via
/// [`add`](NodeElement::add) and attributes via
/// [`set_attr`](NodeElement::set_attr).
///
/// # Allowed tags and attributes
///
/// Telegraph supports these tags: `a`, `aside`, `b`, `blockquote`, `br`,
/// `code`, `em`, `figcaption`, `figure`, `h3`, `h4`, `hr`, `i`,
/// `iframe`, `img`, `li`, `ol`, `p`, `pre`, `s`, `strong`, `u`, `ul`,
/// `video`. The only attributes it accepts are `href` (for `a`) and
/// `src` (for `img`, `iframe`, `video`).
///
/// # Examples
///
/// The paragraph `<p>Hello, <b>world</b>!</p>`:
///
/// ```ignore
/// let mut bold = NodeElement::new("b")?;
/// bold.add("world");
///
/// let mut p = NodeElement::new("p")?;
/// p.add("Hello, ").add(bold).add("!");
/// ```
///
/// A link:
///
/// ```ignore
/// let mut a = NodeElement::new("a")?;
/// a.set_attr("href", "https://example.com");
/// a.add("Example");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeElement {
    /// Name of the tag, for example `p` or `a`.
    ///
    /// [`NodeElement::new`] guarantees that it is an allowed tag. If you
    /// build the struct directly or deserialize it, the tag is not
    /// checked.
    pub tag: String,

    /// Attributes of the element, as name and value pairs.
    ///
    /// Telegraph accepts only `href` and `src`. Missing in JSON means
    /// empty.
    #[serde(default)]
    pub attrs: HashMap<String, String>,

    /// Child nodes of the element, in display order.
    ///
    /// Missing in JSON means empty.
    #[serde(default)]
    pub children: Vec<Node>,
}

impl NodeElement {
    /// Creates an empty element with the given tag.
    ///
    /// The new element has no attributes and no children.
    ///
    /// # Errors
    ///
    /// Returns an error message if `tag` is not in the list of tags
    /// allowed by Telegraph (see [`NodeElement`]). The check is
    /// case-sensitive.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert!(NodeElement::new("p").is_ok());
    /// assert!(NodeElement::new("div").is_err());
    /// ```
    pub fn new(tag: impl Into<String>) -> Result<Self, String> {
        let tag = tag.into();

        if !ALLOWED_TAGS.contains(&tag.as_str()) {
            return Err(format!("This tag name is not allowed '{tag}'"));
        }
        Ok(Self {
            tag,
            attrs: HashMap::new(),
            children: Vec::new(),
        })
    }

    /// Appends a child node to the end of the element.
    ///
    /// Accepts anything convertible into a [`Node`]: a `String`, a
    /// `&str` (both become text nodes) or a [`NodeElement`]. Returns
    /// `&mut Self`, so calls can be chained.
    ///
    /// The child is not checked against the tag: for example, adding
    /// children to `br` or `img` is not rejected here.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut p = NodeElement::new("p")?;
    /// p.add("Some text, ").add(NodeElement::new("br")?);
    /// ```
    pub fn add(&mut self, content: impl Into<Node>) -> &mut Self {
        self.children.push(content.into());
        self
    }

    /// Sets an attribute, replacing the previous value if it exists.
    ///
    /// Neither the name nor the value is validated. Telegraph accepts
    /// only `href` and `src`, and other attributes are rejected by the
    /// server.
    pub fn set_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attrs.insert(key.into(), value.into());
    }

    /// Returns the value of an attribute, or `None` if it is not set.
    #[must_use]
    pub fn get_attr(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }

    /// Removes an attribute and returns its previous value, or `None` if
    /// it was not set.
    pub fn remove_attr(&mut self, key: &str) -> Option<String> {
        self.attrs.remove(key)
    }

    /// Renders the element and its children as an HTML string.
    ///
    /// The element is converted together with its whole subtree: child
    /// elements are rendered recursively, and text nodes are inserted as
    /// text.
    ///
    /// The method clones the element before rendering, so for large trees
    /// it allocates a copy of the whole subtree. The original is not
    /// modified.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut bold = NodeElement::new("b")?;
    /// bold.add("world");
    ///
    /// let mut p = NodeElement::new("p")?;
    /// p.add("Hello, ").add(bold).add("!");
    ///
    /// assert_eq!(p.as_html(), "<p>Hello, <b>world</b>!</p>");
    /// ```
    #[must_use]
    pub fn as_html(&self) -> String {
        node_to_html(&Node::Element(self.clone()))
    }
}

/// Wraps a `String` into a text node.
impl From<String> for Node {
    fn from(s: String) -> Self {
        Node::Text(s)
    }
}

/// Copies a `&str` into a text node.
impl From<&str> for Node {
    fn from(s: &str) -> Self {
        Node::Text(s.to_owned())
    }
}

/// Wraps a [`NodeElement`] into an element node.
impl From<NodeElement> for Node {
    fn from(el: NodeElement) -> Self {
        Node::Element(el)
    }
}
