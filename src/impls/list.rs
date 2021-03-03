use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;

use palex::ArgsInput;

use crate::actions::{Action, Set};
use crate::util::Flag;
use crate::{Error, ErrorInner, FromInput, FromInputValue, Parse, Result};

use super::StringCtx;

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
pub struct ListCtx<'a, C> {
    /// The flag after which the values should be parsed.
    pub flag: Flag<'a>,
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
    /// as soon as it encounters an argument starting with a dash.
    pub greedy: bool,
}

impl<'a, C: Default> From<Flag<'a>> for ListCtx<'a, C> {
    fn from(flag: Flag<'a>) -> Self {
        ListCtx {
            flag,
            max_items: usize::MAX,
            delimiter: Some(','),
            inner: C::default(),
            greedy: false,
        }
    }
}

impl<'a, T, C: 'a> FromInput<'a> for Vec<T>
where
    T: FromInputValue<'a, Context = C>,
{
    type Context = ListCtx<'a, C>;

    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self> {
        let mut flag_set = false;
        Set(&mut flag_set).apply(input, &context.flag)?;

        if flag_set {
            if input.can_parse_value_no_whitespace() || context.delimiter.is_some() {
                parse_list_no_ws(input, context)
            } else {
                parse_list_with_ws(input, context)
            }
        } else {
            Err(Error::no_value())
        }
    }
}

impl<'a, T, C: 'a> FromInput<'a> for VecDeque<T>
where
    T: FromInputValue<'a, Context = C>,
{
    type Context = ListCtx<'a, C>;

    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self> {
        let mut flag_set = false;
        Set(&mut flag_set).apply(input, &context.flag)?;

        if flag_set {
            if input.can_parse_value_no_whitespace() || context.delimiter.is_some() {
                parse_list_no_ws(input, context)
            } else {
                parse_list_with_ws(input, context)
            }
        } else {
            Err(Error::no_value())
        }
    }
}

impl<'a, T, C: 'a> FromInput<'a> for LinkedList<T>
where
    T: FromInputValue<'a, Context = C>,
{
    type Context = ListCtx<'a, C>;

    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self> {
        let mut flag_set = false;
        Set(&mut flag_set).apply(input, &context.flag)?;

        if flag_set {
            if input.can_parse_value_no_whitespace() || context.delimiter.is_some() {
                parse_list_no_ws(input, context)
            } else {
                parse_list_with_ws(input, context)
            }
        } else {
            Err(Error::no_value())
        }
    }
}

impl<'a, T, C: 'a> FromInput<'a> for BTreeSet<T>
where
    T: FromInputValue<'a, Context = C> + Ord,
{
    type Context = ListCtx<'a, C>;

    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self> {
        let mut flag_set = false;
        Set(&mut flag_set).apply(input, &context.flag)?;

        if flag_set {
            if input.can_parse_value_no_whitespace() || context.delimiter.is_some() {
                parse_list_no_ws(input, context)
            } else {
                parse_list_with_ws(input, context)
            }
        } else {
            Err(Error::no_value())
        }
    }
}

impl<'a, T, C: 'a> FromInput<'a> for HashSet<T>
where
    T: FromInputValue<'a, Context = C> + Hash + Eq,
{
    type Context = ListCtx<'a, C>;

    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self> {
        let mut flag_set = false;
        Set(&mut flag_set).apply(input, &context.flag)?;

        if flag_set {
            if input.can_parse_value_no_whitespace() || context.delimiter.is_some() {
                parse_list_no_ws(input, context)
            } else {
                parse_list_with_ws(input, context)
            }
        } else {
            Err(Error::no_value())
        }
    }
}

fn parse_list_no_ws<'a, L: List<T>, T: FromInputValue<'a>>(
    input: &mut ArgsInput,
    context: &ListCtx<'a, T::Context>,
) -> Result<L> {
    let inner = &context.inner;

    let value: String = input.parse_value(
        &StringCtx::default().allow_leading_dashes(T::allow_leading_dashes(inner)),
    )?;

    if let Some(delim) = context.delimiter {
        let values: L = value
            .split(delim)
            .map(|s| T::from_input_value(s, inner))
            .enumerate()
            .map(|(i, r)| r.map_err(|e| e.chain(ErrorInner::IncompleteValue(i))))
            .collect::<Result<_>>()?;

        let count = values.len();
        if count <= context.max_items {
            Ok(values)
        } else {
            Err(ErrorInner::TooManyValues { max: context.max_items, count }.into())
        }
    } else {
        let value = T::from_input_value(&value, inner)?;
        let mut list = L::default();
        list.add(value);
        Ok(list)
    }
}

fn parse_list_with_ws<'a, L: List<T>, T: FromInputValue<'a>>(
    input: &mut ArgsInput,
    context: &ListCtx<'a, T::Context>,
) -> Result<L> {
    let first = input
        .parse_value(&context.inner)
        .map_err(|e| e.chain(ErrorInner::IncompleteValue(0)))?;
    let mut list = L::default();
    list.add(first);

    for i in 1..context.max_items {
        if let Some(value) = input
            .try_parse_value(&context.inner)
            .map_err(|e| e.chain(ErrorInner::IncompleteValue(i)))?
        {
            list.add(value);
        } else {
            break;
        }
    }

    Ok(list)
}

trait List<T>: Default + FromIterator<T> {
    fn add(&mut self, value: T);
    fn len(&self) -> usize;
}

impl<T> List<T> for Vec<T> {
    fn add(&mut self, value: T) {
        self.push(value)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> List<T> for VecDeque<T> {
    fn add(&mut self, value: T) {
        self.push_back(value)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> List<T> for LinkedList<T> {
    fn add(&mut self, value: T) {
        self.push_back(value)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Ord> List<T> for BTreeSet<T> {
    fn add(&mut self, value: T) {
        self.insert(value);
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Hash + Eq> List<T> for HashSet<T> {
    fn add(&mut self, value: T) {
        self.insert(value);
    }

    fn len(&self) -> usize {
        self.len()
    }
}
