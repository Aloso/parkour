use crate::help::PossibleValues;
use crate::{Error, FromInputValue};

impl FromInputValue for char {
    type Context = ();

    fn from_input_value(value: &str, context: &()) -> Result<Self, Error> {
        let mut chars = value.chars();
        let next = chars
            .next()
            .ok_or_else(|| Error::unexpected_value("", Self::possible_values(context)))?;

        if chars.next().is_some() {
            return Err(Error::unexpected_value(value, Self::possible_values(context)));
        }
        Ok(next)
    }

    fn possible_values(_: &Self::Context) -> Option<PossibleValues> {
        Some(PossibleValues::Other("character".into()))
    }
}
