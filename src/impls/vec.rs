use palex::Input;

use crate::{Error, Parse};

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

impl<T: Parse<Context = C>, C: Clone> Parse for Vec<T> {
    type Context = VecCtx<C>;

    fn parse<I: Input>(input: &mut I, context: Self::Context) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            let value = input.value_allows_leading_dashes().ok_or(Error::NoValue)?;
            let result = Self::parse_from_value(value.as_str(), context)?;
            value.eat();
            Ok(result)
        } else {
            let mut values = Vec::new();
            let mut count = 0;

            for _ in 0..context.max_items {
                match T::parse(input, context.inner.clone()) {
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

    fn parse_from_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: Vec<T> = value
                .split(delim)
                .map(|s| T::parse_from_value(s, context.inner.clone()))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if !(context.min_items..=context.max_items).contains(&count) {
                return Err(Error::WrongNumberOfValues {
                    min: context.min_items,
                    max: context.max_items,
                    count,
                });
            }

            Ok(values)
        } else {
            Ok(vec![T::parse_from_value(value, context.inner)?])
        }
    }
}
