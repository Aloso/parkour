use crate::{Error, FromInputValue};

impl FromInputValue for char {
    type Context = ();

    fn from_input_value(value: &str, _: &()) -> Result<Self, Error> {
        let mut chars = value.chars();
        let next =
            chars.next().ok_or_else(|| Error::unexpected_value("", "character"))?;

        if chars.next().is_some() {
            return Err(Error::unexpected_value(value, "single character"));
        }
        Ok(next)
    }
}
