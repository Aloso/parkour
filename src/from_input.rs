use crate::util::{Flag, OptionCtx};
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
            Err(e) if e.is_no_value() => Ok(None),
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


impl<T: FromInputValue> FromInput for T {
    type Context = OptionCtx<'static, T::Context>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Self, Error> {
        if Flag::from_input(input, &context.flag)? {
            match input.parse_value(&context.inner) {
                Ok(value) => Ok(value),
                Err(e) if e.is_no_value() => {
                    Err(Error::missing_value()
                        .with_source(Error::in_option(&context.flag)))
                }
                Err(e) => Err(e),
            }
        } else {
            Err(Error::no_value())
        }
    }
}
