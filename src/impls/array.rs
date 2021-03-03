use std::convert::TryInto;

use crate::help::PossibleValues;
use crate::{Error, ErrorInner, FromInputValue};

#[derive(Debug)]
pub struct ArrayCtx<C> {
    pub delimiter: Option<char>,
    pub inner: C,
}

impl<C> ArrayCtx<C> {
    pub fn new(delimiter: Option<char>, inner: C) -> Self {
        Self { delimiter, inner }
    }
}

impl<C: Default> Default for ArrayCtx<C> {
    fn default() -> Self {
        ArrayCtx { delimiter: Some(','), inner: C::default() }
    }
}

impl<'a, T: FromInputValue<'a>, const N: usize> FromInputValue<'a> for [T; N] {
    type Context = ArrayCtx<T::Context>;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values = value
                .split(delim)
                .map(|s| T::from_input_value(s, &context.inner))
                .collect::<Result<Vec<T>, _>>()?;

            let len = values.len();
            match values.try_into() {
                Ok(values) => Ok(values),
                Err(_) => {
                    Err(ErrorInner::WrongNumberOfValues { expected: N, got: len }.into())
                }
            }
        } else {
            Err(ErrorInner::WrongNumberOfValues { expected: N, got: 1 }.into())
        }
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(&context.inner)
    }
}
