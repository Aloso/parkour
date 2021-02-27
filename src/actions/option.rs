use crate::util::{ArgCtx, Flag, PosCtx};
use crate::{ErrorInner, FromInput, FromInputValue, Parse};

use super::{
    Action, ApplyResult, Reset, Set, SetOnce, SetPositional, SetSubcommand, Unset,
};

impl<'a, V: FromInputValue<'a>> Action<ArgCtx<'a, V::Context>> for Set<'_, Option<V>> {
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &ArgCtx<'a, V::Context>,
    ) -> ApplyResult {
        match input.try_parse(context).map_err(|e| {
            e.chain(ErrorInner::InArgument(context.flag.first_to_string()))
        })? {
            Some(s) => {
                *self.0 = Some(s);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl<'a, V: FromInputValue<'a>> Action<ArgCtx<'a, V::Context>>
    for SetOnce<'_, Option<V>>
{
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &ArgCtx<'a, V::Context>,
    ) -> ApplyResult {
        match input.try_parse(context).map_err(|e| {
            e.chain(ErrorInner::InArgument(context.flag.first_to_string()))
        })? {
            Some(s) => {
                if self.0.is_some() {
                    return Err(ErrorInner::TooManyArgOccurrences {
                        arg: context.flag.first_to_string(),
                        max: Some(1),
                    }
                    .into());
                }
                *self.0 = Some(s);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl<'a, V: FromInputValue<'a>> Action<Flag<'a>> for Reset<'_, Option<V>> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            *self.0 = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, V: FromInputValue<'a>> Action<Flag<'a>> for Unset<'_, Option<V>> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            if self.0.is_none() {
                return Err(ErrorInner::TooManyArgOccurrences {
                    arg: context.first_to_string(),
                    max: None,
                }
                .into());
            }
            *self.0 = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, T: FromInputValue<'a>> Action<PosCtx<'a, T::Context>>
    for SetPositional<'_, Option<T>>
{
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &PosCtx<'a, T::Context>,
    ) -> ApplyResult {
        if let Some(s) = input.try_parse_value(&context.inner)? {
            if self.0.is_some() {
                return Err(ErrorInner::TooManyArgOccurrences {
                    arg: context.name.to_string(),
                    max: None,
                }
                .into());
            }
            *self.0 = Some(s);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, T: FromInput<'a>> Action<T::Context> for SetSubcommand<'_, Option<T>> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse(context)? {
            if self.0.is_some() {
                return Err(ErrorInner::TooManyArgOccurrences {
                    arg: "subcommand".to_string(),
                    max: None,
                }
                .into());
            }
            *self.0 = Some(s);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
