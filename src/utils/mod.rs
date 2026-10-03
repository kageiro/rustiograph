//! Internal utility functions used by Rustiograph.
//!
//! This module contains reusable helpers for common operations that are not
//! part of the main public API.
//!
//! Currently, it provides [`html`], which contains functions for converting
//! between HTML, Rust node types, and the JSON representation used by the
//! Telegra.ph API.

pub mod html;
