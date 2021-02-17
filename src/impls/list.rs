use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

use crate::{Error, ErrorInner, FromInputValue};

/// The parsing context for list-like types. This is used by the following types
/// from the standard library:
///
/// - [`Vec`]
/// - [`std::collections::VecDeque`]
/// - [`std::collections::HashSet`]
/// - [`std::collections::BTreeSet`]
/// - [`std::collections::LinkedList`]
///
/// This can parse argument lists like the following:
///
/// 1. `-f a b c d`
/// 2. `-f=a,b,c,d`
/// 3. `-f a -f b -f c -f d`
///
/// If you want to allow the third syntax, use [`crate::actions::Append`]
/// action, to make sure that all values are saved.
#[derive(Debug)]
pub struct ListCtx<C> {
    /// The maximum number of items that can be parsed at once. The default is
    /// `usize::MAX`.
    pub max_items: usize,
    /// The delimiter that is used when the `-f=a,b,c,d` syntax is used. The
    /// default is a comma.
    pub delimiter: Option<char>,
    /// The context of the values we want to parse
    pub inner: C,
    /// When `greedy` is set to true, the parser will greedily try to parse as
    /// many values as possible (up to `max_items`) at once, except when the
    /// 2nd syntax is used. This defaults to `false`, so the 1st syntax is
    /// unavailable by default.
    ///
    /// Note that setting `greedy` to `true` is less problematic if the values
    /// can't start with a dash, because then it will stop consuming arguments
    /// as soon as  it encounters an argument starting with a dash.
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

// TODO: Fix these implementations:
// Implement FromInput and not FromInputValue

/*
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
*/

impl<T: FromInputValue<Context = C>, C> FromInputValue for Vec<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: Vec<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, &context.inner))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(ErrorInner::TooManyValues { max: context.max_items, count }.into())
            }
        } else {
            Ok(vec![T::from_input_value(value, &context.inner)?])
        }
    }
}

/*
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
*/

impl<T: FromInputValue<Context = C>, C> FromInputValue for VecDeque<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: VecDeque<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, &context.inner))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(ErrorInner::TooManyValues { max: context.max_items, count }.into())
            }
        } else {
            let mut vec_deque = Self::with_capacity(1);
            vec_deque.push_back(T::from_input_value(value, &context.inner)?);
            Ok(vec_deque)
        }
    }
}

/*
impl<T: FromInputValue<Context = C>, C: Clone> FromInput for LinkedList<T> {
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
*/

impl<T: FromInputValue<Context = C>, C> FromInputValue for LinkedList<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: LinkedList<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, &context.inner))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(ErrorInner::TooManyValues { max: context.max_items, count }.into())
            }
        } else {
            let mut list = Self::new();
            list.push_back(T::from_input_value(value, &context.inner)?);
            Ok(list)
        }
    }
}

/*
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
*/

impl<T: FromInputValue<Context = C> + Ord, C> FromInputValue for BTreeSet<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: BTreeSet<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, &context.inner))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(ErrorInner::TooManyValues { max: context.max_items, count }.into())
            }
        } else {
            let mut vec_deque = Self::new();
            vec_deque.insert(T::from_input_value(value, &context.inner)?);
            Ok(vec_deque)
        }
    }
}

/*
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
*/

impl<T: FromInputValue<Context = C> + Hash + Eq, C> FromInputValue for HashSet<T> {
    type Context = ListCtx<C>;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        if let Some(delim) = context.delimiter {
            let values: HashSet<T> = value
                .split(delim)
                .map(|s| T::from_input_value(s, &context.inner))
                .collect::<Result<_, _>>()?;

            let count = values.len();
            if count <= context.max_items {
                Ok(values)
            } else {
                Err(ErrorInner::TooManyValues { max: context.max_items, count }.into())
            }
        } else {
            let mut vec_deque = Self::with_capacity(1);
            vec_deque.insert(T::from_input_value(value, &context.inner)?);
            Ok(vec_deque)
        }
    }
}
