//! Helper structs for checking if the next token matches your expectations and
//! consuming the token thereupon.

use crate::ArgsInput;

/// A helper struct for checking if the next token matches your expectations and
/// consuming the token thereupon. Instances of this type can be created with
/// the following methods:
///
/// - [`ArgsInput::no_dash`]
/// - [`ArgsInput::one_dash`]
/// - [`ArgsInput::two_dashes`]
/// - [`ArgsInput::value`]
pub struct InputPart<'a> {
    input: &'a mut ArgsInput,
    len: usize,
}

impl<'a> InputPart<'a> {
    pub(super) fn new(len: usize, input: &'a mut ArgsInput) -> Self {
        Self { input, len }
    }
}

impl<'a> InputPart<'a> {
    /// Returns the string slice of the token currently looked at
    pub fn as_str(&self) -> &str {
        &self.input.current().unwrap().0[..self.len]
    }

    /// Returns whether the token currently looked at is empty, i.e. has a
    /// length of 0
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length of the current token in bytes
    pub fn len(&self) -> usize {
        self.len
    }

    /// If the token is longer than `len` bytes, use only the first `len` bytes
    /// of this token. The rest of the string is considered part of the next
    /// token.
    pub fn take(self, len: usize) -> InputPart<'a> {
        InputPart { len, ..self }
    }

    /// Ignore everything but the first [char] of the token. The rest of the
    /// string is considered part of the next token.
    ///
    /// This returns `None` if the token is empty.
    pub fn take_char(self) -> Option<InputPart<'a>> {
        let len = self.as_str().chars().next()?.len_utf8();
        Some(InputPart { len, ..self })
    }

    /// If the token contains `c`, use only the part of the token before the
    /// first occurrence of `c`. The rest of the string is considered part
    /// of the next token.
    pub fn take_until(self, c: char) -> InputPart<'a> {
        let len = self.as_str().find(c).unwrap_or(self.len);
        InputPart { len, ..self }
    }

    /// Consumes and returns the token as string slice.
    pub fn eat(self) -> &'a str {
        self.input.bump(self.len)
    }
}

impl<'a> AsRef<str> for InputPart<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A helper struct for checking if the next token matches your expectations and
/// consuming the token thereupon. Instances of this type can be created with
/// the following method:
///
/// - [`ArgsInput::value_allows_leading_dashes`]
pub struct InputPartLd<'a> {
    input: &'a mut ArgsInput,
    len: usize,
}

impl<'a> InputPartLd<'a> {
    pub(super) fn new(len: usize, input: &'a mut ArgsInput) -> Self {
        Self { input, len }
    }
}

impl<'a> InputPartLd<'a> {
    /// Returns the string slice of the token currently looked at
    pub fn as_str(&self) -> &str {
        &self.input.current_str_with_leading_dashes().unwrap()[..self.len]
    }

    /// Returns whether the token currently looked at is empty, i.e. has a
    /// length of 0
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length of the current token in bytes
    pub fn len(&self) -> usize {
        self.len
    }

    /// If the token is longer than `len` bytes, use only the first `len` bytes
    /// of this token. The rest of the string is considered part of the next
    /// token.
    pub fn take(self, len: usize) -> InputPartLd<'a> {
        InputPartLd { len, ..self }
    }

    /// Ignore everything but the first [char] of the token. The rest of the
    /// string is considered part of the next token.
    ///
    /// This returns `None` if the token is empty.
    pub fn take_char(self) -> Option<InputPartLd<'a>> {
        let len = self.as_str().chars().next()?.len_utf8();
        Some(InputPartLd { len, ..self })
    }

    /// If the token contains `c`, use only the part of the token before the
    /// first occurrence of `c`. The rest of the string is considered part
    /// of the next token.
    pub fn take_until(self, c: char) -> InputPartLd<'a> {
        let len = self.as_str().find(c).unwrap_or(self.len);
        InputPartLd { len, ..self }
    }

    /// Consumes and returns the token as string slice.
    pub fn eat(self) -> &'a str {
        self.input.bump_with_leading_dashes(self.len)
    }
}

impl<'a> AsRef<str> for InputPartLd<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
