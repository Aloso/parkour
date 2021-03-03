#[cfg(not(any(test, feature = "dyn_iter")))]
use std::env::Args;

use crate::part::{InputPart, InputPartLd};
use crate::TokenKind;

/// The default input type for argument parsing. This is generic over its
/// iterator type and can be used with [`std::env::args`]. See
/// [`ArgsInput::new()`] for more information.
///
/// Getting the current token and token kind is very cheap. Bumping the token is
/// a bit more expensive, since it involves more complicated logic and might
/// re-allocate.
pub struct ArgsInput {
    current: Option<(usize, usize, TokenKind)>,

    #[cfg(any(test, feature = "dyn_iter"))]
    iter: Box<dyn Iterator<Item = String>>,
    #[cfg(not(any(test, feature = "dyn_iter")))]
    iter: Args,

    buf: String,
    ignore_dashes: bool,
}

#[cfg(any(test, feature = "dyn_iter"))]
impl ArgsInput {
    /// Creates a new instance of this input.
    ///
    /// ### Example:
    ///
    /// ```
    /// # use palex::ArgsInput;
    /// let mut _input = ArgsInput::new(std::env::args());
    /// ```
    ///
    /// You probably want to discard the first argument in this case, which is
    /// just the path to the executable.
    pub fn new<I: Iterator<Item = String> + 'static>(iter: I) -> Self {
        let mut iter = Box::new(iter);
        match iter.next() {
            Some(buf) => Self {
                current: Some(Self::trim_leading_dashes(false, &buf, 0)),
                iter,
                buf,
                ignore_dashes: false,
            },
            None => {
                Self { current: None, iter, buf: String::new(), ignore_dashes: false }
            }
        }
    }
}

#[cfg(any(test, feature = "dyn_iter"))]
impl From<&'static str> for ArgsInput {
    fn from(s: &'static str) -> Self {
        ArgsInput::new(s.split(' ').map(ToString::to_string))
    }
}

impl ArgsInput {
    /// Creates a new instance from the command-line arguments
    ///
    /// ### Example:
    ///
    /// ```
    /// # use palex::ArgsInput;
    /// let mut _input = ArgsInput::from_args();
    /// ```
    ///
    /// You probably want to discard the first argument in this case, which is
    /// just the path to the executable.
    pub fn from_args() -> Self {
        #[cfg(any(test, feature = "dyn_iter"))]
        let mut iter = Box::new(std::env::args());
        #[cfg(not(any(test, feature = "dyn_iter")))]
        let mut iter = std::env::args();

        match iter.next() {
            Some(buf) => Self {
                current: Some(Self::trim_leading_dashes(false, &buf, 0)),
                iter,
                buf,
                ignore_dashes: false,
            },
            None => {
                Self { current: None, iter, buf: String::new(), ignore_dashes: false }
            }
        }
    }

    fn trim_leading_dashes(
        ignore: bool,
        string: &str,
        current: usize,
    ) -> (usize, usize, TokenKind) {
        if ignore {
            (current, current, TokenKind::NoDash)
        } else if string.starts_with("--") {
            (current + 2, current, TokenKind::TwoDashes)
        } else if string.starts_with('-') {
            (current + 1, current, TokenKind::OneDash)
        } else {
            (current, current, TokenKind::NoDash)
        }
    }

    fn trim_equals(&self, current: usize, kind: TokenKind) -> (usize, usize, TokenKind) {
        match kind {
            TokenKind::NoDash => {}
            TokenKind::OneDash => {
                if self.buf[current..].starts_with('=') {
                    return (current + 1, current + 1, TokenKind::AfterEquals);
                } else {
                    return (current, current, TokenKind::AfterOneDash);
                }
            }
            TokenKind::TwoDashes => {
                if self.buf[current..].starts_with('=') {
                    return (current + 1, current + 1, TokenKind::AfterEquals);
                }
            }
            TokenKind::AfterOneDash => {
                if self.buf[current..].starts_with('=') {
                    return (current + 1, current + 1, TokenKind::AfterEquals);
                }
            }
            TokenKind::AfterEquals => {}
        }
        (current, current, kind)
    }

    /// Returns the current token as string slice and the [`TokenKind`] of the
    /// current token, or [None] if the input is empty.
    ///
    /// This function skips the leading dashes of arguments. If you don't want
    /// that, use [`ArgsInput::current_str_with_leading_dashes()`] instead.
    pub(crate) fn current(&self) -> Option<(&str, TokenKind)> {
        self.current.map(|(i, _, kind)| (&self.buf[i..], kind))
    }

    /// Returns the current token (including the leading dashes) as string
    /// slice, or [None] if the input is empty.
    pub(crate) fn current_str_with_leading_dashes(&self) -> Option<&str> {
        self.current.map(|(_, i, _)| &self.buf[i..])
    }

    /// Bumps the current token by `len` bytes.
    ///
    /// Leading dashes are ignored, e.g. bumping the argument `--foo` by one
    /// byte returns `f`; the rest of the token is `oo`. If you don't want
    /// this, use [`ArgsInput::bump_with_leading_dashes()`] instead.
    ///
    /// If the bytes are followed by an equals sign and the current
    /// [`TokenKind`] is `OneDash`, `TwoDashes` or `AfterOneDash`, the
    /// equals sign is skipped.
    ///
    /// If afterwards the current argument is empty, a new argument is read and
    /// becomes the "current token"
    pub(crate) fn bump(&mut self, len: usize) -> &str {
        if let Some((current, _, kind)) = &mut self.current {
            let current_len = self.buf.len() - *current;
            if len > current_len {
                panic!("index bumped out of bounds: {} > {}", len, current_len);
            }

            let prev_current = *current;
            *current += len;

            if current_len == len {
                match self.iter.next() {
                    Some(s) => {
                        self.buf.push_str(&s);
                        self.current = Some(Self::trim_leading_dashes(
                            self.ignore_dashes,
                            &s,
                            *current,
                        ));
                    }
                    None => self.current = None,
                }
            } else {
                let (current, kind) = (*current, *kind);
                self.current = Some(self.trim_equals(current, kind));
            }

            &self.buf[prev_current..prev_current + len]
        } else {
            panic!("tried to bump index on empty input by {}", len)
        }
    }

    /// Bumps the current token (including leading dashes) by `len` bytes.
    ///
    /// If the bytes are followed by an equals sign and the current
    /// [`TokenKind`] is `OneDash`, `TwoDashes` or `AfterOneDash`, the
    /// equals sign is skipped.
    ///
    /// If afterwards the current argument is empty, a new argument is read and
    /// becomes the "current token"
    pub(crate) fn bump_with_leading_dashes(&mut self, len: usize) -> &str {
        if let Some((current, cwd, kind)) = &mut self.current {
            let current_len = self.buf.len() - *cwd;
            if len > current_len {
                panic!("index bumped out of bounds: {} > {}", len, current_len);
            }

            let prev_current = *cwd;
            *current += len;
            *cwd += len;

            if current_len == len {
                match self.iter.next() {
                    Some(s) => {
                        self.buf.push_str(&s);
                        self.current =
                            Some(Self::trim_leading_dashes(self.ignore_dashes, &s, *cwd));
                    }
                    None => self.current = None,
                }
            } else {
                let (current, kind) = (*current, *kind);
                self.current = Some(self.trim_equals(current, kind));
            }

            &self.buf[prev_current..prev_current + len]
        } else {
            panic!("tried to bump index on empty input by {}", len)
        }
    }

    /// Bumps the current argument (including leading dashes) completely.
    pub fn bump_argument(&mut self) -> Option<&str> {
        if let Some((i, _, _)) = self.current {
            let len = self.buf.len() - i;
            Some(self.bump(len))
        } else {
            None
        }
    }

    /// Sets the parsing mode. When `true`, all arguments are considered
    /// positional, i.e. leading dashes are ignored.
    pub fn set_ignore_dashes(&mut self, ignore: bool) {
        self.ignore_dashes = ignore;
        if let Some((current, cwd, kind)) = &mut self.current {
            if ignore {
                *current = *cwd;
                *kind = TokenKind::NoDash;
            } else {
                self.current =
                    Some(Self::trim_leading_dashes(ignore, &self.buf[*current..], *cwd));
            }
        }
    }

    /// Returns the parsing mode. When `true`, all arguments are considered
    /// positional, i.e. leading dashes are ignored.
    pub fn ignore_dashes(&self) -> bool {
        self.ignore_dashes
    }

    /// Returns `true` if the input is empty. This means that all arguments have
    /// been fully parsed.
    pub fn is_empty(&self) -> bool {
        self.current().is_none()
    }

    /// Returns `true` if the input is not empty. This means that all arguments
    /// have been fully parsed.
    pub fn is_not_empty(&self) -> bool {
        self.current().is_some()
    }

    /// Returns `true` if a value within the same argument is expected. Or in
    /// other words, if we just consumed a single-dash flag or an equals sign
    /// and there are remaining bytes in the same argument.
    pub fn can_parse_value_no_whitespace(&self) -> bool {
        if let Some((_, current)) = self.current() {
            matches!(current, TokenKind::AfterOneDash | TokenKind::AfterEquals)
        } else {
            false
        }
    }

    /// Returns `true` if the current token can be parsed as a flag or named
    /// argument (e.g. `-h`, `--help=config`).
    pub fn can_parse_dash_argument(&self) -> bool {
        if let Some((_, current)) = self.current() {
            matches!(
                current,
                TokenKind::OneDash | TokenKind::TwoDashes | TokenKind::AfterOneDash
            )
        } else {
            false
        }
    }

    /// Eat the current token if the argument doesn't start with dashes and
    /// matches `token` exactly.
    pub fn eat_no_dash<'a>(&mut self, token: &'a str) -> Option<&str> {
        if let Some((s, TokenKind::NoDash)) = self.current() {
            if token == s {
                return Some(self.bump(token.len()));
            }
        }
        None
    }

    /// Eat the current token if the argument starts with a single dash, and the
    /// current token starts with `token`.
    ///
    /// Does not work if the token appears after an equals sign has already been
    /// parsed.
    pub fn eat_one_dash<'a>(&mut self, token: &'a str) -> Option<&str> {
        if let Some((s, TokenKind::OneDash)) | Some((s, TokenKind::AfterOneDash)) =
            self.current()
        {
            if s.starts_with(token) {
                return Some(self.bump(token.len()));
            }
        }
        None
    }

    /// Eat the current token if the argument starts with (at least) two dashes,
    /// and the current token either matches `token` exactly, or starts with
    /// `token` followed by an equals sign.
    ///
    /// Does not work if the token appears after an equals sign has already been
    /// parsed.
    pub fn eat_two_dashes<'a>(&mut self, token: &'a str) -> Option<&str> {
        if let Some((s, TokenKind::TwoDashes)) = self.current() {
            if let Some(rest) = s.strip_prefix(token) {
                if rest.is_empty() || rest.starts_with('=') {
                    return Some(self.bump(token.len()));
                }
            }
        }
        None
    }

    /// Eat the current token if it matches `token` exactly.
    ///
    /// This method only works if the current [`TokenKind`] is either `NoDash`,
    /// `AfterOneDash` or `AfterEquals`.
    pub fn eat_value<'a>(&mut self, token: &'a str) -> Option<&str> {
        if let Some((s, kind)) = self.current() {
            match kind {
                TokenKind::TwoDashes | TokenKind::OneDash => return None,

                | TokenKind::NoDash
                | TokenKind::AfterOneDash
                | TokenKind::AfterEquals => {
                    if let Some(rest) = s.strip_prefix(token) {
                        if rest.is_empty() {
                            return Some(self.bump(token.len()));
                        }
                    }
                }
            }
        }
        None
    }

    /// Eat the current token (including any leading dashes) if it matches
    /// `token` exactly.
    pub fn eat_value_allows_leading_dashes<'a>(
        &mut self,
        token: &'a str,
    ) -> Option<&str> {
        if let Some(s) = self.current_str_with_leading_dashes() {
            if let Some(rest) = s.strip_prefix(token) {
                if rest.is_empty() {
                    return Some(self.bump_with_leading_dashes(token.len()));
                }
            }
        }
        None
    }

    /// If the argument doesn't start with dashes, returns a helper struct for
    /// obtaining, validating and eating the next token.
    pub fn no_dash(&mut self) -> Option<InputPart<'_>>
    where
        Self: Sized,
    {
        match self.current() {
            Some((s, TokenKind::NoDash)) => Some(InputPart::new(s.len(), self)),
            _ => None,
        }
    }

    /// If the argument starts with a single dash, returns a helper struct for
    /// obtaining, validating and eating the next token.
    pub fn one_dash(&mut self) -> Option<InputPart<'_>>
    where
        Self: Sized,
    {
        match self.current() {
            Some((s, TokenKind::OneDash)) => Some(InputPart::new(s.len(), self)),
            _ => None,
        }
    }

    /// If the argument starts with two (or more) dashes, returns a helper
    /// struct for obtaining, validating and eating the next token.
    pub fn two_dashes(&mut self) -> Option<InputPart<'_>>
    where
        Self: Sized,
    {
        match self.current() {
            Some((s, TokenKind::TwoDashes)) => Some(InputPart::new(s.len(), self)),
            _ => None,
        }
    }

    /// Returns a helper struct for obtaining, validating and eating the next
    /// token. Works only if the current [`TokenKind`] is either `NoDash`,
    /// `AfterOneDash` or `AfterEquals`.
    ///
    /// The value is not allowed to start with a dash, unless the dash is not at
    /// the start of the current argument.
    pub fn value(&mut self) -> Option<InputPart<'_>>
    where
        Self: Sized,
    {
        match self.current() {
            | Some((s, TokenKind::NoDash))
            | Some((s, TokenKind::AfterOneDash))
            | Some((s, TokenKind::AfterEquals)) => Some(InputPart::new(s.len(), self)),
            _ => None,
        }
    }

    /// Returns a helper struct for obtaining, validating and eating the next
    /// token. The value is allowed to start with a dash.
    pub fn value_allows_leading_dashes(&mut self) -> Option<InputPartLd<'_>>
    where
        Self: Sized,
    {
        match self.current_str_with_leading_dashes() {
            Some(s) => Some(InputPartLd::new(s.len(), self)),
            None => None,
        }
    }
}
