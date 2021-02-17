use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

/// The error type when parsing command-line arguments
#[derive(Debug)]
pub enum Error {
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
        Error::ParseIntError(e)
    }
}
impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::ParseFloatError(e)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoValue => write!(f, "no value"),
            Error::MissingValue => write!(f, "missing value"),
            Error::IncompleteValue(part) => {
                write!(f, "missing part {} of value", part)
            }
            Error::EarlyExit => write!(f, "early exit"),
            Error::UnexpectedValue { expected, got } => {
                write!(
                    f,
                    "unexpected value `{}`, expected {}",
                    got.escape_debug(),
                    expected.escape_debug()
                )
            }
            Error::UnexpectedArgument { arg } => {
                write!(f, "unexpected argument `{}`", arg.escape_debug())
            }
            Error::TooManyValues { max, count } => {
                write!(f, "too many values, expected at most {}, got {}", max, count)
            }
            Error::WrongNumberOfValues { expected, got } => {
                write!(f, "wrong number of values, expected {}, got {}", expected, got)
            }
            Error::MissingArgument { arg: option } => {
                write!(f, "required {} was not provided", option)
            }
            Error::TooManyArgOccurrences { option, max } => {
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

            Error::ParseIntError(e) => write!(f, "{}", e),
            Error::ParseFloatError(e) => write!(f, "{}", e),
        }
    }
}
