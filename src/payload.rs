//! Utilities for building and modifying JSON request payloads.
//!
//! This module provides [`Payload`], a small wrapper around
//! [`serde_json::Value`], together with helpers for serializing Rust values
//! into JSON payloads.

use crate::error::RustiographError;
use crate::result::Result;
use serde::Serialize;
use serde_json::Value;

/// A JSON value used as a request payload.
///
/// `Payload` provides a small API for constructing a JSON value from a
/// serializable Rust type and replacing individual fields before the payload
/// is sent to the API.
pub struct Payload(Value);

/// Serializes a value into a JSON request payload.
///
/// This is a convenience wrapper around [`serde_json::to_value`] that
/// converts serialization errors into [`RustiographError::Encode`].
///
/// # Errors
///
/// Returns [`RustiographError::Encode`] if `value` cannot be serialized.
pub fn to_payload<T: Serialize>(value: &T) -> Result<Value> {
    serde_json::to_value(value).map_err(RustiographError::Encode)
}

impl Payload {
    /// Creates a new payload from a serializable value.
    ///
    /// The value is converted into a [`serde_json::Value`] and stored inside
    /// the payload.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` cannot be serialized into JSON.
    pub fn new<T: Serialize>(value: &T) -> Result<Self> {
        Ok(Self(serde_json::to_value(value)?))
    }

    /// Replaces or inserts a field in the payload.
    ///
    /// If `field` already exists, its value is replaced. Otherwise, a new
    /// field is inserted.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` cannot be serialized into JSON.
    pub fn replace<T: Serialize>(&mut self, field: &str, value: T) -> Result<()> {
        self.0[field] = serde_json::to_value(value)?;
        Ok(())
    }

    /// Returns the underlying JSON value.
    ///
    /// This consumes the payload and transfers ownership of the contained
    /// [`serde_json::Value`].
    #[must_use]
    pub fn into_inner(self) -> Value {
        self.0
    }
}
