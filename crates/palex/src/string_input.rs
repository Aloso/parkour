use crate::{Input, TokenKind};

/// The default input type for argument parsing. This is generic over its
/// iterator type and can be used with [`std::env::args`]. See
/// [`StringInput::new()`] for more information.
///
/// Getting the current token and token kind is very cheap. Bumping the token is
/// a bit more expensive, since it involves more complicated logic and might
/// re-allocate.
pub struct StringInput<I: Iterator<Item = String> = std::env::Args> {
    current: Option<(usize, usize, TokenKind)>,
    iter: I,
    buf: String,
    ignore_dashes: bool,
}

impl<I: Iterator<Item = String>> StringInput<I> {
    /// Creates a new instance of this input.
    ///
    /// ### Example:
    ///
    /// ```
    /// # use palex::StringInput;
    /// let mut _input = StringInput::new(std::env::args());
    /// ```
    ///
    /// You probably want to discard the first argument in this case, which is
    /// just the path to the executable.
    pub fn new(mut iter: I) -> Self {
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
}

impl<I: Iterator<Item = String>> Input for StringInput<I> {
    fn current(&self) -> Option<(&str, TokenKind)> {
        self.current.map(|(i, _, kind)| (&self.buf[i..], kind))
    }

    fn current_str_with_leading_dashes(&self) -> Option<&str> {
        self.current.map(|(_, i, _)| &self.buf[i..])
    }

    fn bump(&mut self, len: usize) -> &str {
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

    fn bump_with_leading_dashes(&mut self, len: usize) -> &str {
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

    fn bump_argument(&mut self) -> Option<&str> {
        if let Some((i, _, _)) = self.current {
            let len = self.buf.len() - i;
            Some(self.bump(len))
        } else {
            None
        }
    }

    fn set_ignore_dashes(&mut self, ignore: bool) {
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

    fn ignore_dashes(&self) -> bool {
        self.ignore_dashes
    }
}

type ToStringMapping = for<'r> fn(&'r str) -> String;
type StringIter<'a> = std::iter::Map<std::str::Split<'a, char>, ToStringMapping>;

impl<'a> From<&'a str> for StringInput<StringIter<'a>> {
    fn from(s: &'a str) -> Self {
        StringInput::new(s.split(' ').map(ToString::to_string))
    }
}
