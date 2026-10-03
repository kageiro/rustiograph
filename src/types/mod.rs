//! Core data types used throughout Rustiograph.
//!
//! This module contains the fundamental types required to represent
//! Telegra.ph API requests, responses, and resources.
//!
//! The types provided here are used by the API interface and other modules
//! throughout the crate. They include account information, pages, page
//! lists, page views, and Telegra.ph content nodes.
//!
//! The most commonly used types are re-exported from this module for
//! convenient access.

pub mod account;
pub mod node;
pub mod page;
pub mod page_list;
pub mod page_views;

pub use account::Account;
pub use node::{Node, NodeElement};
pub use page::{Page, PagePath};
pub use page_list::PageList;
pub use page_views::PageViews;
