use std::collections::{BTreeSet, HashSet, VecDeque};
use std::hash::Hash;

use crate::{Error, FromInput, FromInputValue, Parse};

#[derive(Debug)]
pub struct ListCtx<C> {
    pub max_items: usize,
    pub delimiter: Option<char>,
    pub inner: C,
    pub greedy: bool,
}

impl<C: Default> Default for ListCtx<C> {
    fn default() -> Self {
        ListCtx {
            max_items: usize::MAX,
            delimiter: Some(','),
            inner: C::default(),
            greedy: false,
        }
    }
}

impl<T: FromInputValue<Context = C>, C: Clone> FromInput for Vec<T> {
    type Context = ListCtx<C>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: Self::Context,
    ) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            input.parse_value(context)
        } else {
            let mut values = Self::new();

            if context.greedy {
                for _ in 0..context.max_items {
                    match input.parse_value(context.inner.clone()) {
                        Err(Error::NoValue) => break,
                        Err(e) => return Err(e),
                        Ok(value) => values.push(value),
                    }
                }
            } else {
                values.push(input.parse_value(context.inner)?);
            }

            Ok(values)
        }
    }
}

impl<T: FromInputValue<Context = C>, C: Clone> FromInputValue for Vec<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: Vec<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, context.inner.clone()))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(Error::TooManyValues { max: context.max_items, count })
            }
        } else {
            Ok(vec![T::from_input_value(value, context.inner)?])
        }
    }
}


impl<T: FromInputValue<Context = C>, C: Clone> FromInput for VecDeque<T> {
    type Context = ListCtx<C>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: Self::Context,
    ) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            input.parse_value(context)
        } else {
            let mut values = Self::new();

            if context.greedy {
                for _ in 0..context.max_items {
                    match input.parse_value(context.inner.clone()) {
                        Err(Error::NoValue) => break,
                        Err(e) => return Err(e),
                        Ok(value) => values.push_back(value),
                    }
                }
            } else {
                values.push_back(input.parse_value(context.inner)?);
            }

            Ok(values)
        }
    }
}

impl<T: FromInputValue<Context = C>, C: Clone> FromInputValue for VecDeque<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: VecDeque<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, context.inner.clone()))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(Error::TooManyValues { max: context.max_items, count })
            }
        } else {
            let mut vec_deque = Self::with_capacity(1);
            vec_deque.push_back(T::from_input_value(value, context.inner)?);
            Ok(vec_deque)
        }
    }
}


impl<T: FromInputValue<Context = C> + Ord, C: Clone> FromInput for BTreeSet<T> {
    type Context = ListCtx<C>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: Self::Context,
    ) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            input.parse_value(context)
        } else {
            let mut values = Self::new();

            if context.greedy {
                for _ in 0..context.max_items {
                    match input.parse_value(context.inner.clone()) {
                        Err(Error::NoValue) => break,
                        Err(e) => return Err(e),
                        Ok(value) => {
                            values.insert(value);
                        }
                    }
                }
            } else {
                values.insert(input.parse_value(context.inner)?);
            }

            Ok(values)
        }
    }
}

impl<T: FromInputValue<Context = C> + Ord, C: Clone> FromInputValue for BTreeSet<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: BTreeSet<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, context.inner.clone()))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(Error::TooManyValues { max: context.max_items, count })
            }
        } else {
            let mut vec_deque = Self::new();
            vec_deque.insert(T::from_input_value(value, context.inner)?);
            Ok(vec_deque)
        }
    }
}


impl<T: FromInputValue<Context = C> + Hash + Eq, C: Clone> FromInput for HashSet<T> {
    type Context = ListCtx<C>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: Self::Context,
    ) -> Result<Self, Error> {
        if input.can_parse_value_no_whitespace() {
            input.parse_value(context)
        } else {
            let mut values = Self::new();

            if context.greedy {
                for _ in 0..context.max_items {
                    match input.parse_value(context.inner.clone()) {
                        Err(Error::NoValue) => break,
                        Err(e) => return Err(e),
                        Ok(value) => {
                            values.insert(value);
                        }
                    }
                }
            } else {
                values.insert(input.parse_value(context.inner)?);
            }

            Ok(values)
        }
    }
}

impl<T: FromInputValue<Context = C> + Hash + Eq, C: Clone> FromInputValue for HashSet<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: HashSet<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, context.inner.clone()))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(Error::TooManyValues { max: context.max_items, count })
            }
        } else {
            let mut vec_deque = Self::with_capacity(1);
            vec_deque.insert(T::from_input_value(value, context.inner)?);
            Ok(vec_deque)
        }
    }
}
