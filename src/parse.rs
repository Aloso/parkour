use palex::Input;

use crate::{Error, FromInput, FromInputValue};

pub trait Parse: Input + Sized {
    #[inline]
    fn parse<F: FromInput>(&mut self, context: &F::Context) -> Result<F, Error> {
        F::from_input(self, context)
    }

    #[inline]
    fn try_parse<F: FromInput>(
        &mut self,
        context: &F::Context,
    ) -> Result<Option<F>, Error> {
        match self.parse(context) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn parse_value<V: FromInputValue>(
        &mut self,
        context: &V::Context,
    ) -> Result<V, Error>;

    #[inline]
    fn try_parse_value<V: FromInputValue>(
        &mut self,
        context: &V::Context,
    ) -> Result<Option<V>, Error> {
        match self.parse_value(context) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }


    fn parse_short_flag(&mut self, flag: &str) -> bool;

    fn parse_long_flag(&mut self, flag: &str) -> bool;

    fn parse_command(&mut self, command: &str) -> bool;

    fn expect_empty(&mut self) -> Result<(), Error> {
        if !self.is_empty() {
            return Err(Error::UnexpectedArgument {
                arg: self.bump_argument().unwrap().to_string(),
            });
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
            let value = self.value_allows_leading_dashes().ok_or(Error::NoValue)?;
            let result = V::from_input_value(value.as_str(), context)?;
            value.eat();
            Ok(result)
        } else {
            let value = self.value().ok_or(Error::NoValue)?;
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
