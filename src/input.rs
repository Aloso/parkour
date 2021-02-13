use std::env::Args;

use crate::{args::ParamName, OffsetString};

/// The argument that is currently being processed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Next {
    Arg(String),
    ArgRest(OffsetString),
    ArgAfterEquals(OffsetString),
    None,
}

/// A wrapper for an iterator over [`String`]
#[derive(Debug)]
pub struct PalrInput<I = Args> {
    next: Next,
    rest: I,
}

impl<I: Iterator<Item = String>> PalrInput<I> {
    /// Create a new [`PalrInput`] instance.
    pub fn new(args: I) -> Self {
        let mut input = Self {
            next: Next::None,
            rest: args,
        };
        input.bump();
        input
    }

    pub fn is_empty(&self) -> bool {
        self.next == Next::None
    }

    /// Return the argument that is currently being processed _by value_
    /// and replace it with [`Next::None`].
    #[inline]
    fn take_next(&mut self) -> Next {
        std::mem::replace(&mut self.next, Next::None)
    }

    /// Return the argument that is currently being processed
    #[inline]
    pub fn peek(&self) -> &Next {
        &self.next
    }

    /// Advance the iterator, returning the current argument
    #[inline]
    pub fn bump(&mut self) -> Next {
        let prev = self.take_next();
        self.next = match self.rest.next() {
            Some(arg) => Next::Arg(arg),
            None => Next::None,
        };
        prev
    }

    /// Return `true` and advance the iterator, if the current argument matches `expected`.
    /// Note that this function respects partial arguments, e.g.
    ///
    /// ```
    /// # use palr::input::PalrInput;
    /// let mut input = PalrInput::new_from_vec(vec!["hello", "-world", "--hello=world"]);
    /// assert!(input.eat_word("hello"));
    ///
    /// assert!(input.eat_short_param("w"));
    /// assert!(input.eat_word("orld"));
    ///
    /// assert!(input.eat_long_param("hello"));
    /// assert!(input.eat_word("world"));
    /// ```
    pub fn eat_word(&mut self, expected: &str) -> bool {
        // TODO: Allow consuming until delimiter, like comma
        let found = match self.peek() {
            Next::Arg(a) if a == expected => true,
            Next::ArgRest(a) if a == expected => true,
            Next::ArgAfterEquals(a) if a == expected => true,
            _ => false,
        };
        if found {
            self.bump();
        }
        found
    }

    /// Return `true` and advance the iterator, if the current argument matches `expected`.
    /// Note that this function respects partial arguments, e.g.
    ///
    /// ```
    /// # use palr::input::PalrInput;
    /// let mut input = PalrInput::new_from_vec(vec!["hello", "-world", "--hello=world"]);
    /// assert!(input.eat_word("hello"));
    ///
    /// assert!(input.eat_short_param("w"));
    /// assert!(input.eat_word("orld"));
    ///
    /// assert!(input.eat_long_param("hello"));
    /// assert!(input.eat_word("world"));
    /// ```
    pub fn get_word(&mut self) -> Option<String> {
        Some(match self.bump() {
            Next::Arg(a) => a,
            Next::ArgRest(a) => a.to_string(),
            Next::ArgAfterEquals(a) => a.to_string(),
            _ => return None,
        })
    }

    /// Return the argument that is currently being processed
    #[inline]
    pub fn peek_word(&self) -> Option<&str> {
        Some(match self.peek() {
            Next::Arg(a) => a,
            Next::ArgRest(a) => a,
            Next::ArgAfterEquals(a) => a,
            _ => return None,
        })
    }

    /// Return `true` and advance the iterator, if the current argument matches `expected`.
    /// Note that this function does NOT respects partial arguments, e.g.
    ///
    /// ```
    /// # use palr::input::PalrInput;
    /// let mut input = PalrInput::new_from_vec(vec!["hello", "-world", "--hello=world"]);
    /// assert!(input.eat_arg("hello"));
    ///
    /// assert!(input.eat_short_param("w"));
    /// assert_eq!(input.eat_arg("orld"), false);
    /// ```
    pub fn eat_arg(&mut self, expected: &str) -> bool {
        match &self.next {
            Next::Arg(a) if a == expected => {
                self.bump();
                true
            }
            _ => false,
        }
    }

    /// Return `true` and advance the iterator, if the next short parameter matches `expected`.
    /// A parameter is considered "short" if it starts with a single dash (`-`). Short parameters
    /// can be concatenated, i.e. `-a -b -cd=foo` is equivalent to `-abcd=foo`.
    ///
    /// ```
    /// # use palr::input::PalrInput;
    /// let mut input = PalrInput::new_from_vec(vec!["-hello", "-w=orld", "--hello=world"]);
    /// assert!(input.eat_short_param("h"));
    /// assert!(input.eat_short_param("ello"));
    /// assert!(input.eat_short_param("w"));
    /// assert!(input.eat_word("orld"));
    ///
    /// assert_eq!(input.eat_arg("hello"), false);
    /// ```
    pub fn eat_short_param(&mut self, expected: &str) -> bool {
        match self.take_next() {
            Next::Arg(a) if a.starts_with('-') && a[1..].starts_with(expected) => {
                let mut a = OffsetString::from(a);
                a.inc_offset(expected.len() + 1);

                match a.chars().next() {
                    Some('=') => {
                        a.inc_offset(1);
                        if a.is_empty() {
                            panic!("Illegal");
                        }
                        self.next = Next::ArgAfterEquals(a);
                    }
                    Some(_) => self.next = Next::ArgRest(a),
                    None => {
                        self.bump();
                    }
                }
                true
            }
            Next::ArgRest(mut a) if a.starts_with(expected) => {
                a.inc_offset(expected.len());

                match a.chars().next() {
                    Some('=') => {
                        a.inc_offset(1);
                        if a.is_empty() {
                            panic!("Illegal");
                        }
                        self.next = Next::ArgAfterEquals(a);
                    }
                    Some(_) => self.next = Next::ArgRest(a),
                    None => {
                        self.bump();
                    }
                }
                true
            }
            o => {
                self.next = o;
                false
            }
        }
    }

    /// Return `true` and advance the iterator, if the next long parameter matches `expected`.
    /// A parameter is considered "long" if it starts with two dashes (`--`).
    ///
    /// ```
    /// # use palr::input::PalrInput;
    /// let mut input = PalrInput::new_from_vec(vec!["hello", "--world", "--hello=world"]);
    /// assert!(input.eat_arg("hello"));
    /// assert!(input.eat_long_param("world"));
    /// assert!(input.eat_long_param("hello"));
    /// assert!(input.eat_word("world"));
    /// ```
    pub fn eat_long_param(&mut self, expected: &str) -> bool {
        match self.take_next() {
            Next::Arg(a) if a.starts_with("--") && a[2..].starts_with(expected) => {
                let mut a = OffsetString::from(a);
                a.inc_offset(expected.len() + 2);

                match a.chars().next() {
                    Some('=') => {
                        a.inc_offset(1);
                        if a.is_empty() {
                            panic!("Illegal");
                        }
                        self.next = Next::ArgAfterEquals(a);
                    }
                    Some(_) => {
                        self.next = Next::Arg(a.original());
                        return false;
                    }
                    None => {
                        self.bump();
                    }
                }
                true
            }
            Next::ArgRest(mut a) if a.starts_with(expected) => {
                a.inc_offset(expected.len());

                match a.chars().next() {
                    Some('=') => {
                        a.inc_offset(1);
                        if a.is_empty() {
                            panic!("Illegal");
                        }
                        self.next = Next::ArgAfterEquals(a);
                    }
                    Some(_) => self.next = Next::ArgRest(a),
                    None => {
                        self.bump();
                    }
                }
                true
            }
            o => {
                self.next = o;
                false
            }
        }
    }

    /// Return `true` and advance the iterator, if the next parameter matches `expected`.
    /// A parameter is considered "long" if it starts with two dashes (`--`).
    ///
    /// ```
    /// # use palr::input::PalrInput;
    /// # use palr::ParamName;
    ///
    /// let mut input = PalrInput::new_from_vec(vec!["-hello", "-world", "--hello=world"]);
    /// assert!(input.eat_param(&"h".into()));
    /// assert!(input.eat_word("ello"));
    ///
    /// assert!(input.eat_param(&ParamName::Short("world".into())));
    /// assert!(input.eat_param(&"hello".into()));
    /// assert_eq!(input.eat_param(&"world".into()), false);
    /// ```
    #[inline]
    pub fn eat_param(&mut self, expected: &ParamName) -> bool {
        match expected {
            ParamName::Long(long) => self.eat_long_param(long),
            ParamName::Short(short) => self.eat_short_param(short),
        }
    }

    /// Return `true` and advance the iterator, if the next parameter matches any of the
    /// items in the `expected` collection. See [`PalrInput::eat_param`] for more info.
    #[inline]
    pub fn eat_any_param<'a>(
        &mut self,
        expected: impl IntoIterator<Item = &'a ParamName>,
    ) -> Option<&'a ParamName> {
        expected.into_iter().find(|e| self.eat_param(e))
    }

    /// Return `true` and advance the iterator, if the current argument matches any of the
    /// items in the `expected` collection. See [`PalrInput::eat_arg`] for more info.
    #[inline]
    pub fn eat_any_arg<A: AsRef<str>>(
        &mut self,
        expected: impl IntoIterator<Item = A>,
    ) -> Option<A> {
        expected.into_iter().find(|e| self.eat_arg(e.as_ref()))
    }

    /// Return `true` and advance the iterator, if the current argument matches any of the
    /// items in the `expected` collection. See [`PalrInput::eat_word`] for more info.
    #[inline]
    pub fn eat_any_word<A: AsRef<str>>(
        &mut self,
        expected: impl IntoIterator<Item = A>,
    ) -> Option<A> {
        expected.into_iter().find(|e| self.eat_word(e.as_ref()))
    }
}
