use std::borrow::Cow;
use std::cell::{Cell, RefCell, UnsafeCell};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use crate::help::PossibleValues;
use crate::{Error, FromInputValue};

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for Box<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for Rc<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for Arc<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for Cell<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for RefCell<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for UnsafeCell<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for Mutex<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for RwLock<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: FromInputValue<'a>> FromInputValue<'a> for ManuallyDrop<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        T::possible_values(context)
    }
}

impl<'a, T: ToOwned> FromInputValue<'a> for Cow<'static, T>
where
    T::Owned: FromInputValue<'a>,
{
    type Context = <T::Owned as FromInputValue<'a>>::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        let v = <T::Owned as FromInputValue>::from_input_value(value, context)?;
        Ok(Cow::Owned(v))
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        <T::Owned as FromInputValue>::allow_leading_dashes(context)
    }

    fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
        <T::Owned as FromInputValue>::possible_values(context)
    }
}
