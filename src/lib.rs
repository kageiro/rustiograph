//! # `RustioGraph`
//!
//! `RustioGraph` is a Rust crate that provides a convenient interface for
//! interacting with the [Telegra.ph API](https://telegra.ph/api).
//!
//! The crate is designed to simplify common Telegra.ph tasks such as
//! creating and editing pages, retrieving page information, managing
//! accounts, and automating publications.
//!
//! `RustioGraph` provides two levels of interaction with the Telegra.ph API:
//!
//! - a **high-level interface** with typed request and response types for
//!   common API operations;
//! - a **low-level interface** for making custom API requests that are not
//!   yet represented by the high-level API.
//!
//! This combination allows `RustioGraph` to remain convenient for common use
//! cases while still providing access to the underlying Telegra.ph API when
//! the library's API does not yet cover a newly introduced or otherwise
//! unsupported method.
//!
//! ## High-level API
//!
//! The high-level API provides typed methods for the Telegra.ph operations
//! supported by `RustioGraph`. Request parameters and API responses are
//! represented by dedicated Rust types, making common operations easier to
//! discover and use.
//!
//! For example, a page can be created using [`Rustiograph::create_page`]:
//!
//! ```no_run
//! # use rustiograph::{Rustiograph, methods::CreatePage};
//! #
//! # async fn example() -> rustiograph::Result<()> {
//! let mut telegraph = Rustiograph::new(None)?;
//!
//! let page = telegraph
//!     .create_page(CreatePage {
//!         title: "My page".parse()?,
//!         content: "<p>Hello, Telegra.ph!</p>".into(),
//!         author_name: None,
//!         author_url: None,
//!         return_content: false,
//!         as_user: false,
//!     })
//!     .await?;
//! #
//! # println!("{}", page.path);
//! # Ok(())
//! # }
//! ```
//!
//! The high-level interface is the recommended way to work with operations
//! that are already supported by the crate.
//!
//! ## Low-level API
//!
//! `RustioGraph` also provides a low-level interface for calling Telegra.ph
//! API methods directly. This makes it possible to use API functionality
//! that has not yet been added to the high-level interface.
//!
//! A method can be specified using one of the crate's [`Method`] variants
//! or by providing its API name as a string. Request parameters and response
//! types can be defined by the caller.
//!
//! This is particularly useful when Telegra.ph introduces a new API method
//! before `RustioGraph` has released an update supporting it. Applications do
//! not have to wait for a new version of the crate and can use the new API
//! functionality through the low-level interface.
//!
//! The low-level interface is also useful for custom integrations and API
//! operations that require request or response types specific to an
//! application.
//!
//! ## Content
//!
//! `RustioGraph` supports working with Telegra.ph page content through
//! [`ContentInput`]. Content can be provided as HTML or as a collection of
//! typed [`Node`] values.
//!
//! HTML content can be written directly when convenient, while nodes provide
//! a more structured way to construct page content programmatically.
//!
//! The crate handles conversion of supported content into the format expected
//! by the Telegra.ph API.
//!
//! ## Accounts and authentication
//!
//! `RustioGraph` provides types and methods for working with Telegra.ph
//! accounts and authentication tokens.
//!
//! Applications can create accounts, retrieve and edit account information,
//! and revoke access tokens. An authentication token can also be stored by
//! [`Rustiograph`] and used for authenticated API operations.
//!
//! ## Pages and publications
//!
//! The crate provides functionality for managing Telegra.ph pages, including
//! creating, editing, and retrieving pages.
//!
//! It can also retrieve page lists and view statistics, making it suitable
//! for applications that automate publishing workflows or need to interact
//! with existing Telegra.ph content programmatically.
//!
//! ## Error handling
//!
//! Operations return [`Result`], which uses [`RustiographError`] as its
//! default error type.
//!
//! Errors from different stages of an operation, including HTTP requests,
//! API responses, serialization, and content processing, are represented by
//! [`RustiographError`].
//!
//! ## API stability and compatibility
//!
//! `RustioGraph` aims to provide a convenient typed interface without limiting
//! access to the underlying Telegra.ph API.
//!
//! The high-level API may not immediately support every change made to the
//! official Telegra.ph API. The low-level interface is therefore intentionally
//! available as an escape hatch for methods and request formats that are not
//! yet represented by the crate.
//!
//! This allows applications to use newly available Telegra.ph functionality
//! while keeping the convenience of the typed interface for commonly used
//! operations.
//!
//! ## Main types
//!
//! The following types form the main public interface of `RustioGraph`:
//!
//! - [`Rustiograph`] — client used to interact with the Telegra.ph API.
//! - [`RustiographBuilder`] — builder for configuring a [`Rustiograph`] client.
//! - [`Method`] — supported Telegra.ph API methods.
//! - [`ContentInput`] — page content supplied as HTML or typed nodes.
//! - [`Node`] — representation of a Telegra.ph content node.
//! - [`RustiographError`] — error type returned by the crate.
//! - [`Result`] — convenience result type using [`RustiographError`] by
//!   default.
//!
//! ## Modules
//!
//! - [`api`] — main Telegra.ph API interface and client utilities.
//! - [`methods`] — API methods and their request parameter types.
//! - [`types`] — core data types used throughout the crate.
//! - [`utils`] — internal utility functionality.
//!
//! ## License
//!
//! `RustioGraph` is distributed under the terms of the license specified by
//! the project.

#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::invalid_rust_codeblocks)]
#![warn(rustdoc::bare_urls)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_must_use)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod api;
pub mod bounded;
pub mod consts;
pub mod content;
pub mod error;
pub mod helpers;
pub mod methods;
pub mod payload;
pub mod prelude;
pub mod result;
pub mod types;
pub mod utils;

pub use prelude::*;
