use std::{fmt, ops::Deref};

/// An owned string that can be sliced in place from the start.
///
/// ### Example
///
/// ```
/// # use palr::OffsetString;
/// let mut s: OffsetString = "hello world!".into();
/// assert_eq!(&s, "hello world!");
///
/// s.inc_offset(2);
/// assert_eq!(&s, "llo world!");
///
/// s.inc_offset(4);
/// assert_eq!(&s, "world!");
///
/// assert_eq!(&s.original(), "hello world!");
/// ```
#[derive(Clone, Eq)]
pub struct OffsetString {
    inner: String,
    offset: usize,
}

impl OffsetString {
    /// Create a new OffsetString from a [`String`] and an offset
    pub fn new(inner: String, offset: usize) -> Self {
        Self { inner, offset }
    }

    /// Return the string as a [`str`] slice.
    /// This is equivalent to the [`Deref`] implementation
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.inner[self.offset..]
    }

    /// Increase the offset by `inc` bytes.
    pub fn inc_offset(&mut self, inc: usize) {
        self.offset += inc;
    }

    /// Return the original, owned [`String`]
    /// from which this `OffsetString` was created.
    pub fn original(self) -> String {
        self.inner
    }
}

impl Deref for OffsetString {
    type Target = str;

    fn deref(&self) -> &str {
        &self.inner[self.offset..]
    }
}

impl From<String> for OffsetString {
    fn from(inner: String) -> Self {
        Self { inner, offset: 0 }
    }
}

impl From<&str> for OffsetString {
    fn from(inner: &str) -> Self {
        Self {
            inner: inner.to_string(),
            offset: 0,
        }
    }
}

impl fmt::Debug for OffsetString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for OffsetString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl PartialEq for OffsetString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for OffsetString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialOrd for OffsetString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for OffsetString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}
