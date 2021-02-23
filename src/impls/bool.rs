use crate::help::PossibleValues;
use crate::{Error, FromInputValue};

impl FromInputValue for bool {
    type Context = ();

    fn from_input_value(value: &str, context: &()) -> Result<Self, Error> {
        match value {
            "1" => Ok(true),
            "0" => Ok(false),
            s if s.eq_ignore_ascii_case("y") => Ok(true),
            s if s.eq_ignore_ascii_case("n") => Ok(false),
            s if s.eq_ignore_ascii_case("yes") => Ok(true),
            s if s.eq_ignore_ascii_case("no") => Ok(false),
            s if s.eq_ignore_ascii_case("true") => Ok(true),
            s if s.eq_ignore_ascii_case("false") => Ok(false),
            _ => Err(Error::unexpected_value(value, Self::possible_values(context))),
        }
    }

    fn possible_values(_: &Self::Context) -> Option<PossibleValues> {
        Some(PossibleValues::OneOf(vec![
            PossibleValues::String("yes".into()),
            PossibleValues::String("no".into()),
        ]))
    }
}
