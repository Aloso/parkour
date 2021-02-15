use std::fmt;

use crate::Error;

pub trait MapNoValue<T, E> {
    fn map_no_value<F: FnOnce() -> E>(self, map: F) -> Result<T, Error>;
}

impl<T> MapNoValue<T, Error> for Result<T, Error> {
    fn map_no_value<F: FnOnce() -> Error>(self, map: F) -> Result<T, Error> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NoValue) => Err(map()),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag<'a> {
    Long(&'a str),
    Short(&'a str),
}

impl fmt::Display for Flag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Flag::Long(flag) => write!(f, "--{}", flag.escape_debug()),
            Flag::Short(flag) => write!(f, "-{}", flag.escape_debug()),
        }
    }
}
