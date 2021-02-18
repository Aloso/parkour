//! Actions are used to store the parsed command-line arguments in local
//! variables. Actions can make sure that arguments are specified at most once.
//!
//! The structs [SetOnce], [Set], [Unset], [Reset], [Inc], [Dec], [Append],
//! [SetPositional] and [SetSubcommand] implement the [Action] trait. Each
//! struct has a different strategy of updating the local variable, and is
//! implemented for different types. For example, [Inc] and [Dec] are only
//! implemented for integer types, whereas [Set] is implemented for all types.
//!
//! ## Usage
//!
//! Make sure the [Action] trait is in scope, e.g.
//! ```rust
//! use parkour::actions::Action;
//! // or
//! use parkour::actions::Action as _;
//! // or
//! use parkour::prelude::*;
//! ```

use crate::{Error, FromInput, FromInputValue, Parse};

mod bool;
mod option;

/// The result of [`Action::apply`]
pub type ApplyResult = Result<bool, Error>;

/// The trait for _actions_. Actions are used to store the parsed command-line
/// arguments in local variables. Actions can make sure that arguments are
/// specified at most once.
pub trait Action<C> {
    /// Perform the action.
    fn apply<P: Parse>(self, input: &mut P, context: &C) -> ApplyResult;
}

/// Set the parsed value, ensuring that it is set at most once. When the action
/// is performed and the value is not in its initial state (e.g. `None`), an
/// error is returned.
pub struct SetOnce<'a, T>(pub &'a mut T);

/// Set the value to it's initial state, e.g. `None`. This returns an error if
/// the value is still in its initial state.
pub struct Unset<'a, T>(pub &'a mut T);

/// Set the parsed value. When this action is performed multiple times, only the
/// last value is preserved.
pub struct Set<'a, T>(pub &'a mut T);

/// Reset the value to it's initial state, e.g. `None`. If it is already in its
/// initial state, nothing happens.
pub struct Reset<'a, T>(pub &'a mut T);

/// Increments the value.
pub struct Inc<'a, T>(pub &'a mut T);

/// Decrements the value.
pub struct Dec<'a, T>(pub &'a mut T);

/// Appends the parsed value(s) to the existing ones.
pub struct Append<'a, T>(pub &'a mut T);

/// Like [`Set`], but works for positional arguments.
pub struct SetPositional<'a, T>(pub &'a mut T);

/// Like [`Set`], but works for subcommands.
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
