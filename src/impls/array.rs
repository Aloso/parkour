use std::convert::TryInto;

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

impl<T: FromInputValue, const N: usize> FromInputValue for [T; N] {
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
}

/*
impl<T: FromInputValue, const N: usize> FromInput for [T; N]
where
    T::Context: Clone,
{
    type Context = ArrayCtx<T::Context>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: Self::Context,
    ) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            input.parse_value(context)
        } else {
            let mut values = Vec::<T>::new();

            for _ in 0..N {
                match input.parse_value(context.inner.clone()) {
                    Err(Error::no_value) => break,
                    Err(e) => return Err(e),
                    Ok(value) => values.push(value),
                }
            }

            let len = values.len();
            match values.try_into() {
                Ok(values) => Ok(values),
                Err(_) => Err(Error::WrongNumberOfValues { expected: N, got: len }),
            }
        }
    }
}
*/
