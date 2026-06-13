use bluetape_rs_codec::{Base58DecodeError, Base64DecodeError, TextDecodeError};
use bluetape_rs_core::ValidationError;
use thiserror::Error;

/// Errors returned by the invitation codec example.
#[derive(Debug, Error)]
pub enum InvitationError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("base64url decoding failed: {0}")]
    Base64(#[from] Base64DecodeError),
    #[error("base58 decoding failed: {0}")]
    Base58(#[from] Base58DecodeError),
    #[error("text decoding failed: {0}")]
    Text(#[from] TextDecodeError),
    #[error("{field} contains reserved delimiter `{delimiter}`")]
    ReservedDelimiter {
        field: &'static str,
        delimiter: char,
    },
    #[error("{artifact} has {actual_parts} parts, expected {expected_parts}")]
    InvalidShape {
        artifact: &'static str,
        expected_parts: usize,
        actual_parts: usize,
    },
}
