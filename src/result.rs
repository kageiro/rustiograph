//! Result types used throughout Rustiograph.
//!
//! This module provides [`Result`], a convenience alias for
//! [`std::result::Result`] that uses [`RustiographError`] as its default
//! error type.

use crate::error::RustiographError;

/// A convenience [`Result`] type for Rustiograph operations.
///
/// The error type defaults to [`RustiographError`], while the success type
/// remains generic.
///
/// The error type can be overridden when needed:
///
/// ```
/// use rustiograph::result::Result;
///
/// let value: Result<u32, std::io::Error> = Ok(42);
/// ```
///
/// Most public Rustiograph operations use this alias without specifying
/// the error type:
///
/// ```
/// use rustiograph::result::Result;
///
/// fn example() -> Result<u32> {
///     Ok(42)
/// }
/// ```
pub type Result<T, E = RustiographError> = std::result::Result<T, E>;
