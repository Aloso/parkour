use std::marker::PhantomData;

use palex::Input;

pub use error::Error;

pub mod args;
mod error;
mod std_impls;

pub trait Parse: Sized {
    fn parse<I: Input>(input: &mut I) -> Result<Self, Error> {
        let value = input.value().ok_or(Error::NoValue)?;
        let result = Self::parse_from_value(value.as_str())?;
        value.eat();
        Ok(result)
    }

    fn try_parse<I: Input>(input: &mut I) -> Result<Option<Self>, Error> {
        match Self::parse(input) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn parse_value_of_option<I: Input>(input: &mut I, name: &str) -> Result<Self, Error> {
        match Self::parse(input) {
            Ok(value) => Ok(value),
            Err(Error::NoValue) => Err(Error::MissingValue { option: name.to_string() }),
            Err(e) => Err(e),
        }
    }

    fn parse_from_value(value: &str) -> Result<Self, Error>;
}


pub struct List<T: Parse, const MIN: usize, const MAX: usize> {
    pub inner: Vec<T>,
    phantom: PhantomData<([(); MIN], [(); MAX])>,
}

impl<T: Parse, const MIN: usize, const MAX: usize> List<T, MIN, MAX> {
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner, phantom: PhantomData }
    }
}

impl<T: Parse, const MIN: usize, const MAX: usize> Parse for List<T, MIN, MAX> {
    fn parse<I: Input>(input: &mut I) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() || (MIN == 1 && MAX == 1) {
            if MIN > 1 || MAX < 1 {
                return Err(Error::WrongNumberOfValues { min: MIN, max: MAX, count: 1 });
            }
            let value = T::parse(input)?;
            Ok(List::new(vec![value]))
        } else {
            let mut values = Vec::new();
            let mut count = 0;

            for _ in 0..MAX {
                match T::parse(input) {
                    Err(Error::NoValue) => break,
                    Err(e) => return Err(e),
                    Ok(value) => {
                        values.push(value);
                        count += 1;
                    }
                }
            }

            if count < MIN {
                return Err(Error::WrongNumberOfValues { min: MAX, max: MAX, count });
            }

            Ok(List::new(values))
        }
    }

    fn parse_from_value(value: &str) -> Result<Self, Error> {
        Ok(List::new(vec![T::parse_from_value(value)?]))
    }
}


pub struct ListDelimited<T: Parse, const MIN: usize, const MAX: usize, const DELIM: char>
{
    pub inner: Vec<T>,
    phantom: PhantomData<([(); MIN], [(); MAX])>,
}

impl<T: Parse, const MIN: usize, const MAX: usize, const DELIM: char>
    ListDelimited<T, MIN, MAX, DELIM>
{
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner, phantom: PhantomData }
    }
}

impl<T: Parse, const MIN: usize, const MAX: usize, const DELIM: char> Parse
    for ListDelimited<T, MIN, MAX, DELIM>
{
    fn parse<I: Input>(input: &mut I) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            let value = input.value_allows_leading_dashes().ok_or(Error::NoValue)?;
            let result = Self::parse_from_value(value.as_str())?;
            value.eat();
            Ok(result)
        } else {
            let mut values = Vec::new();
            let mut count = 0;

            for _ in 0..MAX {
                match T::parse(input) {
                    Err(Error::NoValue) => break,
                    Err(e) => return Err(e),
                    Ok(value) => {
                        values.push(value);
                        count += 1;
                    }
                }
            }

            if count < MIN {
                return Err(Error::WrongNumberOfValues { min: MAX, max: MAX, count });
            }

            Ok(ListDelimited::new(values))
        }
    }

    fn parse_from_value(value: &str) -> Result<Self, Error> {
        let values: Vec<T> =
            value.split(DELIM).map(T::parse_from_value).collect::<Result<_, _>>()?;

        let count = values.len();
        if !(MIN..=MAX).contains(&count) {
            return Err(Error::WrongNumberOfValues { min: MAX, max: MAX, count });
        }

        Ok(ListDelimited::new(values))
    }
}
