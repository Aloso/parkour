use crate::{Error, Parse};

pub struct StringCtx {
    pub min_length: usize,
    pub max_length: usize,
}

impl Default for StringCtx {
    fn default() -> Self {
        StringCtx { min_length: 0, max_length: usize::MAX }
    }
}

impl StringCtx {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        StringCtx { min_length, max_length }
    }
}

impl Parse for String {
    type Context = StringCtx;

    fn parse_from_value(value: &str, context: StringCtx) -> Result<Self, Error> {
        if value.len() < context.min_length || value.len() > context.max_length {
            Err(Error::Unexpected {
                word: format!(
                    "string with length {}, expected length in range [{}, {}]",
                    value.len(),
                    context.min_length,
                    context.max_length
                ),
            })
        } else {
            Ok(value.to_string())
        }
    }
}
