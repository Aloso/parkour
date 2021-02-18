use palex::Input;

use crate::{Error, ErrorInner, FromInput, FromInputValue};

/// An extension trait of [`palex::Input`], the trait for types that can produce
/// tokens from a list of command-line arguments.
///
/// This trait provides several convenience methods for parsing different
/// things.
///
/// Note that this trait is automatically implemented for all types that
/// implement `Input`.
pub trait Parse: Input + Sized {
    /// Parse something using the [`FromInput`] trait
    #[inline]
    fn parse<F: FromInput>(&mut self, context: &F::Context) -> Result<F, Error> {
        F::from_input(self, context)
    }

    /// Parse something using the [`FromInput`] trait, but convert
    /// [`Error::no_value`] to [`Option::None`]. This is useful when you want to
    /// bubble up all errors except for [`Error::no_value`]:
    ///
    /// ```rust,no_test
    /// if let Some(x) = input.try_parse(&())? {
    ///     // do something with x
    /// }
    /// ```
    #[inline]
    fn try_parse<F: FromInput>(
        &mut self,
        context: &F::Context,
    ) -> Result<Option<F>, Error> {
        F::try_from_input(self, context)
    }

    /// Parse a _value_ using the [`FromInputValue`] trait.
    fn parse_value<V: FromInputValue>(
        &mut self,
        context: &V::Context,
    ) -> Result<V, Error>;

    /// Parse a _value_ using the [`FromInputValue`] trait, but convert
    /// [`Error::no_value`] to [`Option::None`]. This is useful when you want to
    /// bubble up all errors except for [`Error::no_value`]:
    ///
    /// ```rust,no_test
    /// if let Some(value) = input.try_parse_value(&())? {
    ///     // do something with value
    /// }
    /// ```
    #[inline]
    fn try_parse_value<V: FromInputValue>(
        &mut self,
        context: &V::Context,
    ) -> Result<Option<V>, Error> {
        match self.parse_value(context) {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.is_no_value() => Ok(None),
            Err(e) => Err(e),
        }
    }


    /// Convenience function for parsing a flag with a single dash, like `-h` or
    /// `-foo`. Returns `true` if it succeeded.
    fn parse_short_flag(&mut self, flag: &str) -> bool;

    /// Convenience function for parsing a flag with two dashes, like `--h` or
    /// `--foo`. Returns `true` if it succeeded.
    fn parse_long_flag(&mut self, flag: &str) -> bool;

    /// Convenience function for parsing a (sub)command, i.e. an argument that
    /// doesn't start with a dash. Returns `true` if it succeeded.
    fn parse_command(&mut self, command: &str) -> bool;

    /// Returns an error if the input is not yet empty.
    fn expect_empty(&mut self) -> Result<(), Error> {
        if !self.is_empty() {
            return Err(ErrorInner::UnexpectedArgument {
                arg: self.bump_argument().unwrap().to_string(),
            }
            .into());
        }
        Ok(())
    }
}

impl<I: Input> Parse for I {
    #[inline]
    fn parse_value<V: FromInputValue>(
        &mut self,
        context: &V::Context,
    ) -> Result<V, Error> {
        if V::allow_leading_dashes(&context) {
            let value = self.value_allows_leading_dashes().ok_or_else(Error::no_value)?;
            let result = V::from_input_value(value.as_str(), context)?;
            value.eat();
            Ok(result)
        } else {
            let value = self.value().ok_or_else(Error::no_value)?;
            let result = V::from_input_value(value.as_str(), context)?;
            value.eat();
            Ok(result)
        }
    }

    #[inline]
    fn parse_short_flag(&mut self, flag: &str) -> bool {
        self.eat_one_dash(flag).is_some()
    }

    #[inline]
    fn parse_long_flag(&mut self, flag: &str) -> bool {
        self.eat_two_dashes(flag).is_some()
    }

    #[inline]
    fn parse_command(&mut self, command: &str) -> bool {
        self.eat_no_dash(command).is_some()
    }
}
