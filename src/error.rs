use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum Error {
    NoValue,
    EarlyExit,

    MissingValue { option: String },
    Unexpected { word: String },
    WrongNumberOfValues { min: usize, max: usize, count: usize },
    MissingOption { option: String },
    TooManyOptionOccurrences { option: String, max: usize },

    ParseIntError(ParseIntError),
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
            Error::MissingValue { option } => write!(f, "missing value for {}", option),
            Error::EarlyExit => write!(f, "early exit"),
            Error::Unexpected { word } => write!(f, "unexpected word {:?}", word),
            Error::WrongNumberOfValues { min, max, count } => {
                if count < min {
                    write!(f, "too few values, expected at least {}, got {}", min, count)
                } else {
                    write!(f, "too many values, expected at most {}, got {}", max, count)
                }
            }
            Error::MissingOption { option } => {
                write!(f, "required {} was not provided", option)
            }
            Error::TooManyOptionOccurrences { option, max } => {
                write!(
                    f,
                    "{} was used too often, it can be used at most {} times",
                    option, max
                )
            }

            Error::ParseIntError(e) => write!(f, "{}", e),
            Error::ParseFloatError(e) => write!(f, "{}", e),
        }
    }
}