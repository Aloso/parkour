use crate::{Error, Parse};

pub trait FromInput: Sized {
    type Context;

    fn from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Self, Error>;

    fn try_from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Option<Self>, Error> {
        match Self::from_input(input, context) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub trait FromInputValue: Sized {
    type Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error>;

    fn allow_leading_dashes(_context: &Self::Context) -> bool {
        false
    }
}
