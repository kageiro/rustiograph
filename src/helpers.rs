//! Internal helper functions used throughout Rustiograph.
//!
//! This module contains small reusable utilities that simplify common
//! operations such as decoding API responses and converting values between
//! representations.

use serde_json::Value;

use crate::error::RustiographError;
use crate::result::Result;

/// Deserializes a JSON value into the specified type.
///
/// This is a small wrapper around [`serde_json::from_value`] that converts
/// deserialization errors into [`RustiographError::Decode`]. The name of the
/// target type is included in the error to make decoding failures easier to
/// diagnose.
///
/// # Errors
///
/// Returns [`RustiographError::Decode`] if `raw` cannot be deserialized into
/// `T`.
pub fn decode<T: serde::de::DeserializeOwned>(raw: Value) -> Result<T> {
    serde_json::from_value(raw).map_err(|source| RustiographError::Decode {
        type_name: std::any::type_name::<T>(),
        source,
    })
}
