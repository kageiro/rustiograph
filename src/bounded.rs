//! String types with compile-time-defined length bounds.
//!
//! These types are used to represent string parameters whose length is
//! restricted by the Telegra.ph API.

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::error::RustiographError;
use crate::result::Result;

/// A string whose length is restricted to an inclusive range.
///
/// `MIN` and `MAX` define the minimum and maximum number of Unicode scalar
/// values (`char`) allowed in the string.
///
/// This type is used for Telegra.ph API parameters that have documented
/// length restrictions. A `BoundedStr` can only be constructed after its
/// length has been validated.
///
/// # Examples
///
/// ```
/// # use rustiograph::types::string::BoundedStr;
///
/// type Title = BoundedStr<1, 256>;
///
/// let title = Title::new("My page")?;
///
/// assert_eq!(title.as_str(), "My page");
/// # Ok::<(), rustiograph::error::RustiographError>(())
/// ```
///
/// # Errors
///
/// [`BoundedStr::new`] returns [`RustiographError::ContentTooShort`] when the
/// string is shorter than `MIN`, and [`RustiographError::ContentTooLong`] when
/// it is longer than `MAX`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedStr<const MIN: usize, const MAX: usize>(Arc<str>);

impl<const MIN: usize, const MAX: usize> BoundedStr<MIN, MAX> {
    /// Creates a bounded string after validating its length.
    ///
    /// The length is measured in Unicode scalar values (`char`), not bytes.
    ///
    /// # Errors
    ///
    /// Returns [`RustiographError::ContentTooShort`] if the string contains
    /// fewer than `MIN` characters.
    ///
    /// Returns [`RustiographError::ContentTooLong`] if the string contains
    /// more than `MAX` characters.
    pub fn new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        let actual = s.chars().count();

        if actual < MIN {
            return Err(RustiographError::ContentTooShort { min: MIN, actual });
        }
        if actual > MAX {
            return Err(RustiographError::ContentTooLong { max: MAX, actual });
        }
        Ok(Self(Arc::from(s)))
    }

    /// Returns the string as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the bounded string and returns its contents as an owned
    /// [`String`].
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0.to_string()
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<&str> for BoundedStr<MIN, MAX> {
    type Error = RustiographError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<String> for BoundedStr<MIN, MAX> {
    type Error = RustiographError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<str> for BoundedStr<MIN, MAX> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<const MIN: usize, const MAX: usize> fmt::Display for BoundedStr<MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const MIN: usize, const MAX: usize> Deref for BoundedStr<MIN, MAX> {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl<const MIN: usize, const MAX: usize> FromStr for BoundedStr<MIN, MAX> {
    type Err = RustiographError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// A Telegra.ph short name.
///
/// The value must contain between 1 and 32 Unicode characters.
pub type ShortName = BoundedStr<1, 32>;

/// A Telegra.ph page title.
///
/// The value must contain between 1 and 256 Unicode characters.
pub type Title = BoundedStr<1, 256>;

/// A Telegra.ph author name.
///
/// The value may contain between 0 and 128 Unicode characters.
pub type AuthorName = BoundedStr<0, 128>;

/// A Telegra.ph author URL.
///
/// The value may contain between 0 and 512 Unicode characters.
pub type AuthorUrl = BoundedStr<0, 512>;
