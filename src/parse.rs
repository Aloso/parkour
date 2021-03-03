use palex::ArgsInput;

use crate::{Error, ErrorInner, FromInput, FromInputValue};

/// An extension trait of [`palex::ArgsInput`], the trait for types that can
/// produce tokens from a list of command-line arguments.
///
/// This trait provides several convenience methods for parsing different
/// things.
pub trait Parse: Sized {
    /// Parse something using the [`FromInput`] trait
    fn parse<'a, F: FromInput<'a>>(&mut self, context: &F::Context) -> Result<F, Error>;

    /// Parse something using the [`FromInput`] trait, but convert
    /// [`Error::no_value`] to [`Option::None`]. This is useful when you want to
    /// bubble up all errors except for [`Error::no_value`]:
    ///
    /// ```no_run
    /// # use parkour::prelude::*;
    /// # let input: parkour::ArgsInput = todo!();
    /// if let Some(x) = input.try_parse(&Flag::Short("o").into())? {
    ///     // do something with x
    /// #  let _: usize = x;
    /// }
    /// # Ok::<(), parkour::Error>(())
    /// ```
    fn try_parse<'a, F: FromInput<'a>>(
        &mut self,
        context: &F::Context,
    ) -> Result<Option<F>, Error>;

    /// Parse a _value_ using the [`FromInputValue`] trait.
    fn parse_value<'a, V: FromInputValue<'a>>(
        &mut self,
        context: &V::Context,
    ) -> Result<V, Error>;

    /// Parse a _value_ using the [`FromInputValue`] trait, but convert
    /// [`Error::no_value`] to [`Option::None`]. This is useful when you want to
    /// bubble up all errors except for [`Error::no_value`]:
    ///
    /// ```no_run
    /// # use parkour::prelude::*;
    /// # let input: parkour::ArgsInput = todo!();
    /// if let Some(value) = input.try_parse_value(&Default::default())? {
    ///     // do something with value
    /// #  let _: usize = value;
    /// }
    /// # Ok::<(), parkour::Error>(())
    /// ```
    #[inline]
    fn try_parse_value<'a, V: FromInputValue<'a>>(
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
    fn expect_empty(&mut self) -> Result<(), Error>;

    /// Returns an error if the current argument is only partially consumed.
    fn expect_end_of_argument(&mut self) -> Result<(), Error>;
}

impl Parse for ArgsInput {
    #[inline]
    fn parse<'a, F: FromInput<'a>>(&mut self, context: &F::Context) -> Result<F, Error> {
        F::from_input(self, context)
    }

    #[inline]
    fn try_parse<'a, F: FromInput<'a>>(
        &mut self,
        context: &F::Context,
    ) -> Result<Option<F>, Error> {
        F::try_from_input(self, context)
    }

    #[inline]
    fn parse_value<'a, V: FromInputValue<'a>>(
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

    fn expect_empty(&mut self) -> Result<(), Error> {
        if !self.is_empty() {
            return Err(ErrorInner::UnexpectedArgument {
                arg: self.bump_argument().unwrap().to_string(),
            }
            .into());
        }
        Ok(())
    }

    fn expect_end_of_argument(&mut self) -> Result<(), Error> {
        if self.can_parse_value_no_whitespace() {
            return Err(ErrorInner::UnexpectedValue {
                value: self.bump_argument().unwrap().to_string(),
            }
            .into());
        }
        Ok(())
    }
}
