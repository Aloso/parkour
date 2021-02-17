use crate::util::Flag;
use crate::{Error, Parse};

use super::{Action, ApplyResult, Reset, Set, SetOnce, Unset};

impl<'a> Action<Flag<'a>> for Set<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            *self.0 = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a> Action<Flag<'a>> for Reset<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            *self.0 = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a> Action<Flag<'a>> for SetOnce<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            if *self.0 {
                return Err(Error::TooManyArgOccurrences {
                    option: context.first_to_string(),
                    max: Some(1),
                });
            }
            *self.0 = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a> Action<Flag<'a>> for Unset<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if Flag::from_input(input, context)? {
            if !*self.0 {
                return Err(Error::TooManyArgOccurrences {
                    option: context.first_to_string(),
                    max: None,
                });
            }
            *self.0 = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
