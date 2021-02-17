use crate::part::{InputPart, InputPartLD};
use crate::TokenKind;

/// The trait for types that can produce tokens from a list of command-line
/// arguments.
///
/// To implement this trait efficiently, accessing the current token and its
/// [`TokenKind`] should be cheap.
///
/// This trait is implemented for [crate::StringInput].
pub trait Input {
    /// Returns the current token as string slice and the [`TokenKind`] of the
    /// current token, or [None] if the input is empty.
    ///
    /// This function skips the leading dashes of arguments. If you don't want
    /// that, use [`Input::current_str_with_leading_dashes()`] instead.
    fn current(&self) -> Option<(&str, TokenKind)>;

    /// Returns the current token (including the leading dashes) as string
    /// slice, or [None] if the input is empty.
    fn current_str_with_leading_dashes(&self) -> Option<&str>;

    /// Bumps the current token by `len` bytes.
    ///
    /// Leading dashes are ignored, e.g. bumping the argument `--foo` by one
    /// byte returns `f`; the rest of the token is `oo`. If you don't want
    /// this, use [`Input::bump_with_leading_dashes()`] instead.
    ///
    /// If the bytes are followed by an equals sign and the current
    /// [`TokenKind`] is `OneDash`, `TwoDashes` or `AfterOneDash`, the
    /// equals sign is skipped.
    ///
    /// If afterwards the current argument is empty, a new argument is read and
    /// becomes the "current token"
    fn bump(&mut self, len: usize) -> &str;

    /// Bumps the current token (including leading dashes) by `len` bytes.
    ///
    /// If the bytes are followed by an equals sign and the current
    /// [`TokenKind`] is `OneDash`, `TwoDashes` or `AfterOneDash`, the
    /// equals sign is skipped.
    ///
    /// If afterwards the current argument is empty, a new argument is read and
    /// becomes the "current token"
    fn bump_with_leading_dashes(&mut self, len: usize) -> &str;

    /// Bumps the current argument (including leading dashes) completely.
    fn bump_argument(&mut self) -> Option<&str>;

    /// Sets the parsing mode. When `true`, all arguments are considered
    /// positional, i.e. leading dashes are ignored.
    fn set_ignore_dashes(&mut self, ignore: bool);

    /// Returns the parsing mode. When `true`, all arguments are considered
    /// positional, i.e. leading dashes are ignored.
    fn ignore_dashes(&self) -> bool;


    /// Returns `true` if the input is empty. This means that all arguments have
    /// been fully parsed.
    fn is_empty(&self) -> bool {
        self.current().is_none()
    }

    /// Returns `true` if the input is not empty. This means that all arguments
    /// have been fully parsed.
    fn is_not_empty(&self) -> bool {
        self.current().is_some()
    }

    /// Returns `true` if a value within the same argument is expected. Or in
    /// other words, if we just consumed a single-dash option or an equals sign
    /// and there are remaining bytes in the same argument.
    fn can_parse_value_no_whitespace(&self) -> bool {
        if let Some((_, current)) = self.current() {
            matches!(current, TokenKind::AfterOneDash | TokenKind::AfterEquals)
        } else {
            false
        }
    }

    /// Returns `true` if the current token can be parsed as a flag or option
    /// (e.g. `-h`, `--help`).
    fn can_parse_dash_option(&self) -> bool {
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
    fn eat_no_dash<'a>(&mut self, token: &'a str) -> Option<&str> {
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
    fn eat_one_dash<'a>(&mut self, token: &'a str) -> Option<&str> {
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
    fn eat_two_dashes<'a>(&mut self, token: &'a str) -> Option<&str> {
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
    fn eat_value<'a>(&mut self, token: &'a str) -> Option<&str> {
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
    fn eat_value_allows_leading_dashes<'a>(&mut self, token: &'a str) -> Option<&str> {
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
    fn no_dash(&mut self) -> Option<InputPart<'_, Self>>
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
    fn one_dash(&mut self) -> Option<InputPart<'_, Self>>
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
    fn two_dashes(&mut self) -> Option<InputPart<'_, Self>>
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
    fn value(&mut self) -> Option<InputPart<'_, Self>>
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
    fn value_allows_leading_dashes(&mut self) -> Option<InputPartLD<'_, Self>>
    where
        Self: Sized,
    {
        match self.current_str_with_leading_dashes() {
            Some(s) => Some(InputPartLD::new(s.len(), self)),
            None => None,
        }
    }
}
