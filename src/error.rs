use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

use crate::util::Flag;

/// The error type when parsing command-line arguments
#[derive(Debug)]
pub struct Error {
    inner: ErrorInner,
    source: Option<Box<dyn std::error::Error + Sync + Send + 'static>>,
}

impl Error {
    pub fn with_source(
        self,
        source: impl std::error::Error + Sync + Send + 'static,
    ) -> Self {
        Error { source: Some(Box::new(source)), ..self }
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

    /// Returns `true` if this is a `MissingValue` error
    pub fn is_missing_value(&self) -> bool {
        self.inner == ErrorInner::MissingValue
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
    pub fn unexpected_value(got: impl ToString, expected: impl ToString) -> Self {
        ErrorInner::UnexpectedValue {
            got: got.to_string(),
            expected: expected.to_string(),
        }
        .into()
    }

    /// Create a `MissingArgument` error
    pub fn missing_argument(arg: impl ToString) -> Self {
        ErrorInner::MissingArgument { arg: arg.to_string() }.into()
    }

    /// Create a `InOption` error
    pub fn in_option(flag: &Flag) -> Self {
        ErrorInner::InOption(flag.first_to_string()).into()
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
    /// Similarly to Option::None, this indicates that the argument you tried to
    /// parse wasn't present at the current position
    NoValue,

    /// This indicates that the argument you tried to parse wasn't present at
    /// the current position, but was required
    MissingValue,

    /// This indicates that the argument you tried to parse was only partly
    /// present
    IncompleteValue(usize),

    /// Used when an option or flag should abort argument parsing, like --help
    EarlyExit,

    InOption(String),

    UnexpectedValue {
        got: String,
        expected: String,
    },
    TooManyValues {
        max: usize,
        count: usize,
    },
    WrongNumberOfValues {
        expected: usize,
        got: usize,
    },
    MissingArgument {
        arg: String,
    },
    UnexpectedArgument {
        arg: String,
    },
    TooManyArgOccurrences {
        option: String,
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
            ErrorInner::InOption(opt) => write!(f, "in `{}`", opt.escape_debug()),
            ErrorInner::UnexpectedValue { expected, got } => {
                write!(
                    f,
                    "unexpected value `{}`, expected {}",
                    got.escape_debug(),
                    expected.escape_debug()
                )
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
            ErrorInner::MissingArgument { arg: option } => {
                write!(f, "required {} was not provided", option)
            }
            ErrorInner::TooManyArgOccurrences { option, max } => {
                if let Some(max) = max {
                    write!(
                        f,
                        "{} was used too often, it can be used at most {} times",
                        option, max
                    )
                } else {
                    write!(f, "{} was used too often", option)
                }
            }

            ErrorInner::ParseIntError(e) => write!(f, "{}", e),
            ErrorInner::ParseFloatError(e) => write!(f, "{}", e),
        }
    }
}
