use palex::Input;

use crate::Error;

pub trait Parse: Sized {
    type Context;

    fn parse<I: Input>(input: &mut I, context: Self::Context) -> Result<Self, Error> {
        let value = input.value().ok_or(Error::NoValue)?;
        let result = Self::parse_from_value(value.as_str(), context)?;
        value.eat();
        Ok(result)
    }

    fn try_parse<I: Input>(
        input: &mut I,
        context: Self::Context,
    ) -> Result<Option<Self>, Error> {
        match Self::parse(input, context) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn parse_or<I: Input>(
        input: &mut I,
        context: Self::Context,
        err: Error,
    ) -> Result<Self, Error> {
        match Self::parse(input, context) {
            Ok(value) => Ok(value),
            Err(Error::NoValue) => Err(err),
            Err(e) => Err(e),
        }
    }

    fn parse_or_else<I: Input, E: FnOnce() -> Error>(
        input: &mut I,
        context: Self::Context,
        err: E,
    ) -> Result<Self, Error> {
        match Self::parse(input, context) {
            Ok(value) => Ok(value),
            Err(Error::NoValue) => Err(err()),
            Err(e) => Err(e),
        }
    }

    fn parse_from_value(value: &str, context: Self::Context) -> Result<Self, Error>;
}
