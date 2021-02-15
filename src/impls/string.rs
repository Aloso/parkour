use crate::{Error, FromInputValue};

pub struct StringCtx {
    pub min_length: usize,
    pub max_length: usize,
    pub allow_leading_dashes: bool,
}

impl Default for StringCtx {
    fn default() -> Self {
        StringCtx { min_length: 0, max_length: usize::MAX, allow_leading_dashes: false }
    }
}

impl StringCtx {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        StringCtx { min_length, max_length, allow_leading_dashes: false }
    }

    pub fn allow_leading_dashes(mut self) -> Self {
        self.allow_leading_dashes = true;
        self
    }
}

impl FromInputValue for String {
    type Context = StringCtx;

    fn from_input_value(value: &str, context: StringCtx) -> Result<Self, Error> {
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

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.allow_leading_dashes
    }
}
