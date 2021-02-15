use palex::Input;

use crate::{Error, FromInput, FromInputValue, Parse};

pub struct VecCtx<C> {
    pub min_items: usize,
    pub max_items: usize,
    pub delimiter: Option<char>,
    pub inner: C,
}

impl<C: Default> Default for VecCtx<C> {
    fn default() -> Self {
        VecCtx {
            min_items: 0,
            max_items: usize::MAX,
            delimiter: Some(','),
            inner: C::default(),
        }
    }
}

impl<T: FromInputValue<Context = C>, C: Clone> FromInput for Vec<T> {
    type Context = VecCtx<C>;

    fn from_input<I: Input>(
        input: &mut I,
        context: Self::Context,
    ) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            let value = input.value_allows_leading_dashes().ok_or(Error::NoValue)?;
            let result = Self::from_input_value(value.as_str(), context)?;
            value.eat();
            Ok(result)
        } else {
            let mut values = Vec::new();
            let mut count = 0;

            for _ in 0..context.max_items {
                match input.parse_value(context.inner.clone()) {
                    Err(Error::NoValue) => break,
                    Err(e) => return Err(e),
                    Ok(value) => {
                        values.push(value);
                        count += 1;
                    }
                }
            }

            if count < context.min_items {
                return Err(Error::WrongNumberOfValues {
                    min: context.min_items,
                    max: context.max_items,
                    count,
                });
            }

            Ok(values)
        }
    }
}

impl<T: FromInputValue<Context = C>, C: Clone> FromInputValue for Vec<T> {
    type Context = VecCtx<C>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: Vec<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, context.inner.clone()))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if (context.min_items..=context.max_items).contains(&count) {
                Ok(values)
            } else {
                Err(Error::WrongNumberOfValues {
                    min: context.min_items,
                    max: context.max_items,
                    count,
                })
            }
        } else {
            Ok(vec![T::from_input_value(value, context.inner)?])
        }
    }
}
