use palex::Input;

use crate::{
    util::{Flag, MapNoValue},
    Error, FromInput, FromInputValue,
};

pub trait Parse: Input + Sized {
    #[inline]
    fn parse<F: FromInput>(&mut self, context: F::Context) -> Result<F, Error> {
        F::from_input(self, context)
    }

    #[inline]
    fn try_parse<F: FromInput>(
        &mut self,
        context: F::Context,
    ) -> Result<Option<F>, Error> {
        match self.parse(context) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn parse_value<V: FromInputValue>(&mut self, context: V::Context)
        -> Result<V, Error>;

    #[inline]
    fn try_parse_value<V: FromInputValue>(
        &mut self,
        context: V::Context,
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


    fn parse_flag_and_value<V: FromInputValue>(
        &mut self,
        flags: &[Flag],
        context: V::Context,
    ) -> Result<V, Error> {
        if flags.iter().any(|&flag| match flag {
            Flag::Long(flag) => self.parse_long_flag(flag),
            Flag::Short(flag) => self.parse_short_flag(flag),
        }) {
            self.parse_value(context)
                .map_no_value(|| Error::MissingValue { flag: flags[0].to_string() })
        } else {
            Err(Error::NoValue)
        }
    }

    fn try_parse_flag_and_value<V: FromInputValue>(
        &mut self,
        flags: &[Flag],
        context: V::Context,
    ) -> Result<Option<V>, Error> {
        match self.parse_flag_and_value(flags, context) {
            Ok(value) => Ok(Some(value)),
            Err(Error::NoValue) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl<I: Input> Parse for I {
    #[inline]
    fn parse_value<V: FromInputValue>(
        &mut self,
        context: V::Context,
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
