use crate::{Error, FromInput, FromInputValue, Parse};

mod bool;
mod option;

pub type ApplyResult = Result<bool, Error>;

pub trait Action<C> {
    fn apply<P: Parse>(self, input: &mut P, context: &C) -> ApplyResult;
}


pub struct SetOnce<'a, T>(pub &'a mut T);

pub struct Unset<'a, T>(pub &'a mut T);

pub struct Set<'a, T>(pub &'a mut T);

pub struct Reset<'a, T>(pub &'a mut T);

pub struct Inc<'a, T>(pub &'a mut T);

pub struct Dec<'a, T>(pub &'a mut T);

pub struct Append<'a, T>(pub &'a mut T);

pub struct SetPositional<'a, T>(pub &'a mut T);

pub struct SetSubcommand<'a, T>(pub &'a mut T);


impl<T: FromInputValue> Action<T::Context> for SetPositional<'_, T> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse_value(context)? {
            *self.0 = s;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<T: FromInput> Action<T::Context> for SetSubcommand<'_, T> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse(context)? {
            *self.0 = s;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<T: FromInput> Action<T::Context> for Set<'_, T> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = T::try_from_input(input, context)? {
            *self.0 = s;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
