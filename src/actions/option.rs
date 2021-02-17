use crate::util::{Flag, OptionCtx, PosCtx};
use crate::{Error, FromInput, FromInputValue, Parse};

use super::{
    Action, ApplyResult, Reset, Set, SetOnce, SetPositional, SetSubcommand, Unset,
};

impl<V: FromInputValue> Action<OptionCtx<'static, V::Context>> for Set<'_, Option<V>> {
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &OptionCtx<'static, V::Context>,
    ) -> ApplyResult {
        match input.try_parse(context)? {
            Some(s) => {
                *self.0 = Some(s);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl<V: FromInputValue> Action<OptionCtx<'static, V::Context>>
    for SetOnce<'_, Option<V>>
{
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &OptionCtx<'static, V::Context>,
    ) -> ApplyResult {
        match input.try_parse(context)? {
            Some(s) => {
                if self.0.is_some() {
                    return Err(Error::TooManyArgOccurrences {
                        option: context.flag.first_to_string(),
                        max: Some(1),
                    });
                }
                *self.0 = Some(s);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl<'a, V: FromInputValue> Action<Flag<'a>> for Reset<'_, Option<V>> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            *self.0 = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, V: FromInputValue> Action<Flag<'a>> for Unset<'_, Option<V>> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            if self.0.is_none() {
                return Err(Error::TooManyArgOccurrences {
                    option: context.first_to_string(),
                    max: None,
                });
            }
            *self.0 = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, T: FromInputValue> Action<PosCtx<'a, T::Context>>
    for SetPositional<'_, Option<T>>
{
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &PosCtx<'a, T::Context>,
    ) -> ApplyResult {
        if let Some(s) = input.try_parse_value(&context.inner)? {
            if self.0.is_some() {
                return Err(Error::TooManyArgOccurrences {
                    option: context.name.to_string(),
                    max: None,
                });
            }
            *self.0 = Some(s);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<T: FromInput> Action<T::Context> for SetSubcommand<'_, Option<T>> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse(context)? {
            if self.0.is_some() {
                return Err(Error::TooManyArgOccurrences {
                    option: "subcommand".to_string(),
                    max: None,
                });
            }
            *self.0 = Some(s);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
