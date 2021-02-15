use crate::{Error, Parse};

impl Parse for bool {
    type Context = ();

    fn parse_from_value(value: &str, _: ()) -> Result<Self, Error> {
        Ok(match value {
            "1" => true,
            "0" => false,
            s if s.eq_ignore_ascii_case("y") => true,
            s if s.eq_ignore_ascii_case("n") => false,
            s if s.eq_ignore_ascii_case("yes") => true,
            s if s.eq_ignore_ascii_case("no") => false,
            s if s.eq_ignore_ascii_case("true") => true,
            s if s.eq_ignore_ascii_case("false") => false,
            _ => return Err(Error::Unexpected { word: value.to_string() }),
        })
    }
}
