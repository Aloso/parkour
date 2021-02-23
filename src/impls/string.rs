use std::borrow::Cow;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::help::PossibleValues;
use crate::{Error, FromInputValue};

/// The parsing context for strings
pub struct StringCtx {
    /// The minimum length of the string in bytes
    pub min_length: usize,
    /// The maximum length of the string in bytes
    pub max_length: usize,
    /// Whether or not the string may start with dashes
    pub allow_leading_dashes: bool,
}

impl Default for StringCtx {
    fn default() -> Self {
        StringCtx { min_length: 0, max_length: usize::MAX, allow_leading_dashes: false }
    }
}

impl StringCtx {
    /// Create a new `StringCtx` that doesn't accept strings starting with
    /// leading dashes
    pub fn new(min_length: usize, max_length: usize) -> Self {
        StringCtx { min_length, max_length, allow_leading_dashes: false }
    }

    /// Sets `allow_leading_dashes` to true
    pub fn allow_leading_dashes(mut self) -> Self {
        self.allow_leading_dashes = true;
        self
    }
}

impl FromInputValue for String {
    type Context = StringCtx;

    fn from_input_value(value: &str, context: &StringCtx) -> Result<Self, Error> {
        if value.len() < context.min_length || value.len() > context.max_length {
            Err(Error::unexpected_value(
                format!("string with length {}", value.len()),
                Self::possible_values(context),
            ))
        } else {
            Ok(value.to_string())
        }
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.allow_leading_dashes
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        Some(PossibleValues::Other(match (context.min_length, context.max_length) {
            (0, usize::MAX) => "string".into(),
            (1, usize::MAX) => "non-empty string".into(),
            (min, usize::MAX) => format!("string with at least {} bytes", min),
            (0, max) => format!("string with at most {} bytes", max),
            (min, max) => format!("string with {} to {} bytes", min, max),
        }))
    }
}

impl FromInputValue for OsString {
    type Context = StringCtx;

    fn from_input_value(value: &str, context: &StringCtx) -> Result<Self, Error> {
        if value.len() < context.min_length || value.len() > context.max_length {
            Err(Error::unexpected_value(
                format!("string with length {}", value.len()),
                Self::possible_values(context),
            ))
        } else {
            Ok(value.into())
        }
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.allow_leading_dashes
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        Some(PossibleValues::Other(match (context.min_length, context.max_length) {
            (0, usize::MAX) => "string".into(),
            (1, usize::MAX) => "non-empty string".into(),
            (min, usize::MAX) => format!("string with at least {} bytes", min),
            (0, max) => format!("string with at most {} bytes", max),
            (min, max) => format!("string with {} to {} bytes", min, max),
        }))
    }
}

impl FromInputValue for PathBuf {
    type Context = StringCtx;

    fn from_input_value(value: &str, context: &StringCtx) -> Result<Self, Error> {
        if value.len() < context.min_length || value.len() > context.max_length {
            Err(Error::unexpected_value(
                format!("string with length {}", value.len()),
                Self::possible_values(context),
            ))
        } else {
            Ok(value.into())
        }
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.allow_leading_dashes
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        Some(PossibleValues::Other(match (context.min_length, context.max_length) {
            (0, usize::MAX) => "path".into(),
            (1, usize::MAX) => "non-empty path".into(),
            (min, usize::MAX) => format!("path with at least {} bytes", min),
            (0, max) => format!("path with at most {} bytes", max),
            (min, max) => format!("path with {} to {} bytes", min, max),
        }))
    }
}

impl FromInputValue for Cow<'static, str> {
    type Context = StringCtx;

    fn from_input_value(value: &str, context: &StringCtx) -> Result<Self, Error> {
        if value.len() < context.min_length || value.len() > context.max_length {
            Err(Error::unexpected_value(
                format!("string with length {}", value.len()),
                Self::possible_values(context),
            ))
        } else {
            Ok(Cow::Owned(value.into()))
        }
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.allow_leading_dashes
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        Some(PossibleValues::Other(match (context.min_length, context.max_length) {
            (0, usize::MAX) => "string".into(),
            (1, usize::MAX) => "non-empty string".into(),
            (min, usize::MAX) => format!("string with at least {} bytes", min),
            (0, max) => format!("string with at most {} bytes", max),
            (min, max) => format!("string with {} to {} bytes", min, max),
        }))
    }
}
