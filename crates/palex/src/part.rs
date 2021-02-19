//! Helper structs for checking if the next token matches your expectations and
//! consuming the token thereupon.

use crate::Input;

/// A helper struct for checking if the next token matches your expectations and
/// consuming the token thereupon. Instances of this type can be created with
/// the following methods:
///
/// - [`Input::no_dash`]
/// - [`Input::one_dash`]
/// - [`Input::two_dashes`]
/// - [`Input::value`]
pub struct InputPart<'a, P: Input> {
    input: &'a mut P,
    len: usize,
}

impl<'a, P: Input> InputPart<'a, P> {
    pub(super) fn new(len: usize, input: &'a mut P) -> Self {
        Self { input, len }
    }
}

impl<'a, P: Input> InputPart<'a, P> {
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
    pub fn take(self, len: usize) -> InputPart<'a, P> {
        InputPart { len, ..self }
    }

    /// Ignore everything but the first [char] of the token. The rest of the
    /// string is considered part of the next token.
    ///
    /// This returns `None` if the token is empty.
    pub fn take_char(self) -> Option<InputPart<'a, P>> {
        let len = self.as_str().chars().next()?.len_utf8();
        Some(InputPart { len, ..self })
    }

    /// If the token contains `c`, use only the part of the token before the
    /// first occurrence of `c`. The rest of the string is considered part
    /// of the next token.
    pub fn take_until(self, c: char) -> InputPart<'a, P> {
        let len = self.as_str().find(c).unwrap_or(self.len);
        InputPart { len, ..self }
    }

    /// Consumes and returns the token as string slice.
    pub fn eat(self) -> &'a str {
        self.input.bump(self.len)
    }
}

impl<'a, P: Input> AsRef<str> for InputPart<'a, P> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A helper struct for checking if the next token matches your expectations and
/// consuming the token thereupon. Instances of this type can be created with
/// the following method:
///
/// - [`Input::value_allows_leading_dashes`]
pub struct InputPartLd<'a, P: Input> {
    input: &'a mut P,
    len: usize,
}

impl<'a, P: Input> InputPartLd<'a, P> {
    pub(super) fn new(len: usize, input: &'a mut P) -> Self {
        Self { input, len }
    }
}

impl<'a, P: Input> InputPartLd<'a, P> {
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
    pub fn take(self, len: usize) -> InputPartLd<'a, P> {
        InputPartLd { len, ..self }
    }

    /// Ignore everything but the first [char] of the token. The rest of the
    /// string is considered part of the next token.
    ///
    /// This returns `None` if the token is empty.
    pub fn take_char(self) -> Option<InputPartLd<'a, P>> {
        let len = self.as_str().chars().next()?.len_utf8();
        Some(InputPartLd { len, ..self })
    }

    /// If the token contains `c`, use only the part of the token before the
    /// first occurrence of `c`. The rest of the string is considered part
    /// of the next token.
    pub fn take_until(self, c: char) -> InputPartLd<'a, P> {
        let len = self.as_str().find(c).unwrap_or(self.len);
        InputPartLd { len, ..self }
    }

    /// Consumes and returns the token as string slice.
    pub fn eat(self) -> &'a str {
        self.input.bump_with_leading_dashes(self.len)
    }
}

impl<'a, P: Input> AsRef<str> for InputPartLd<'a, P> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
