use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

use crate::help::PossibleValues;
use crate::util::Flag;

/// The error type when parsing command-line arguments. You can create an
/// `Error` by creating an `ErrorInner` and converting it with `.into()`.
///
/// This error type supports an error source for attaching context to the error.
#[derive(Debug)]
pub struct Error {
    inner: ErrorInner,
    source: Option<Box<dyn std::error::Error + Sync + Send + 'static>>,
}

impl Error {
    /// Attach context to the error. Note that this overwrites the current
    /// source, if there is one.
    ///
    /// ### Usage
    ///
    /// ```
    /// use parkour::{Error, util::Flag};
    ///
    /// Error::missing_value()
    ///     .with_source(Error::in_subcommand("test"))
    /// # ;
    /// ```
    ///
    /// This could produce the following output:
    /// ```text
    /// missing value
    ///     source: in subcommand `test`
    /// ```
    pub fn with_source(
        self,
        source: impl std::error::Error + Sync + Send + 'static,
    ) -> Self {
        Error { source: Some(Box::new(source)), ..self }
    }

    /// Attach context to the error. This function ensures that an already
    /// attached source isn't discarded, but appended to the the new source. The
    /// sources therefore form a singly linked list.
    ///
    /// ### Usage
    ///
    /// ```
    /// use parkour::{Error, ErrorInner, util::Flag};
    ///
    /// Error::missing_value()
    ///     .chain(ErrorInner::IncompleteValue(1))
    /// # ;
    /// ```
    pub fn chain(mut self, source: ErrorInner) -> Self {
        let mut new = Self::from(source);
        new.source = self.source.take();
        Error { source: Some(Box::new(new)), ..self }
    }

    /// Create a `NoValue` error
    pub fn no_value() -> Self {
        ErrorInner::NoValue.into()
    }

    /// Returns `true` if this is a `NoValue` error
    pub fn is_no_value(&self) -> bool {
        self.inner == ErrorInner::NoValue
    }

    /// Create a `MissingValue` error
    pub fn missing_value() -> Self {
        ErrorInner::MissingValue.into()
    }

    /// Returns the [`ErrorInner`] of this error
    pub fn inner(&self) -> &ErrorInner {
        &self.inner
    }

    /// Create a `EarlyExit` error
    pub fn early_exit() -> Self {
        ErrorInner::EarlyExit.into()
    }

    /// Returns `true` if this is a `EarlyExit` error
    pub fn is_early_exit(&self) -> bool {
        self.inner == ErrorInner::EarlyExit
    }

    /// Create a `UnexpectedValue` error
    pub fn unexpected_value(
        got: impl ToString,
        expected: Option<PossibleValues>,
    ) -> Self {
        ErrorInner::UnexpectedValue { got: got.to_string(), expected }.into()
    }

    /// Create a `MissingArgument` error
    pub fn missing_argument(arg: impl ToString) -> Self {
        ErrorInner::MissingArgument { arg: arg.to_string() }.into()
    }

    /// Create a `InArgument` error
    pub fn in_argument(flag: &Flag) -> Self {
        ErrorInner::InArgument(flag.first_to_string()).into()
    }

    /// Create a `InSubcommand` error
    pub fn in_subcommand(cmd: impl ToString) -> Self {
        ErrorInner::InSubcommand(cmd.to_string()).into()
    }

    /// Create a `TooManyArgOccurrences` error
    pub fn too_many_arg_occurrences(arg: impl ToString, max: Option<u32>) -> Self {
        ErrorInner::TooManyArgOccurrences { arg: arg.to_string(), max }.into()
    }
}

impl From<ErrorInner> for Error {
    fn from(inner: ErrorInner) -> Self {
        Error { inner, source: None }
    }
}

/// The error type when parsing command-line arguments
#[derive(Debug, PartialEq)]
pub enum ErrorInner {
    /// The argument you tried to parse wasn't present at the current position.
    /// Has a similar purpose as `Option::None`
    NoValue,

    /// The argument you tried to parse wasn't present at the current position,
    /// but was required
    MissingValue,

    /// The argument you tried to parse was only partly present
    IncompleteValue(usize),

    /// Used when an argument should abort argument parsing, like --help
    EarlyExit,

    /// Indicates that the error originated in the specified argument. This
    /// should be used as the source for another error
    InArgument(String),

    /// Indicates that the error originated in the specified subcommand. This
    /// should be used as the source for another error
    InSubcommand(String),

    /// The parsed value doesn't meet our expectations
    UnexpectedValue {
        /// The value we tried to parse
        got: String,
        /// The expectation that was violated. For example, this string can
        /// contain a list of accepted values.
        expected: Option<PossibleValues>,
    },

    /// The parsed list contains more items than allowed
    TooManyValues {
        /// The maximum number of items
        max: usize,
        /// The number of items that was parsed
        count: usize,
    },

    /// The parsed array has the wrong length
    WrongNumberOfValues {
        /// The length of the array
        expected: usize,
        /// The number of items that was parsed
        got: usize,
    },

    /// A required argument was not provided
    MissingArgument {
        /// The name of the argument that is missing
        arg: String,
    },

    /// An unknown argument was provided
    UnexpectedArgument {
        /// The (full) argument that wasn't expected
        arg: String,
    },

    /// An argument was provided more often than allowed
    TooManyArgOccurrences {
        /// The name of the argument that was provided too many times
        arg: String,
        /// The maximum number of times the argument may be provided
        max: Option<u32>,
    },

    /// Parsing an integer failed
    ParseIntError(ParseIntError),

    /// Parsing a floating-point number failed
    ParseFloatError(ParseFloatError),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        ErrorInner::ParseIntError(e).into()
    }
}
impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        ErrorInner::ParseFloatError(e).into()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(source) => Some(&**source as &(dyn std::error::Error + 'static)),
            None => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            ErrorInner::NoValue => write!(f, "no value"),
            ErrorInner::MissingValue => write!(f, "missing value"),
            ErrorInner::IncompleteValue(part) => {
                write!(f, "missing part {} of value", part)
            }
            ErrorInner::EarlyExit => write!(f, "early exit"),
            ErrorInner::InArgument(opt) => write!(f, "in `{}`", opt.escape_debug()),
            ErrorInner::InSubcommand(cmd) => {
                write!(f, "in subcommand {}", cmd.escape_debug())
            }
            ErrorInner::UnexpectedValue { expected, got } => {
                if let Some(expected) = expected {
                    write!(
                        f,
                        "unexpected value `{}`, expected {}",
                        got.escape_debug(),
                        expected,
                    )
                } else {
                    write!(f, "unexpected value `{}`", got.escape_debug())
                }
            }
            ErrorInner::UnexpectedArgument { arg } => {
                write!(f, "unexpected argument `{}`", arg.escape_debug())
            }
            ErrorInner::TooManyValues { max, count } => {
                write!(f, "too many values, expected at most {}, got {}", max, count)
            }
            ErrorInner::WrongNumberOfValues { expected, got } => {
                write!(f, "wrong number of values, expected {}, got {}", expected, got)
            }
            ErrorInner::MissingArgument { arg } => {
                write!(f, "required {} was not provided", arg)
            }
            ErrorInner::TooManyArgOccurrences { arg, max } => {
                if let Some(max) = max {
                    write!(
                        f,
                        "{} was used too often, it can be used at most {} times",
                        arg, max
                    )
                } else {
                    write!(f, "{} was used too often", arg)
                }
            }

            ErrorInner::ParseIntError(e) => write!(f, "{}", e),
            ErrorInner::ParseFloatError(e) => write!(f, "{}", e),
        }
    }
}
