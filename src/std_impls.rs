use crate::{Error, Parse};

impl Parse for u8 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for u16 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for u32 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for u64 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for u128 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for usize {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for i8 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for i16 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for i32 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for i64 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for i128 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for isize {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for f32 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for f64 {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.parse()?)
    }
}

impl Parse for String {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(value.to_string())
    }
}

impl<P: Parse> Parse for Box<P> {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(Box::new(P::parse_from_value(value)?))
    }

    fn parse<I: palex::Input>(input: &mut I) -> Result<Self, Error> {
        Ok(Box::new(P::parse(input)?))
    }
}

impl Parse for bool {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
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
