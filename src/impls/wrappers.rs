use std::cell::{Cell, RefCell, UnsafeCell};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use crate::{Error, FromInputValue};

impl<T: FromInputValue> FromInputValue for Box<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for Rc<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for Arc<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for Cell<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for RefCell<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for UnsafeCell<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for Mutex<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for RwLock<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}

impl<T: FromInputValue> FromInputValue for ManuallyDrop<T> {
    type Context = T::Context;

    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
        T::from_input_value(value, context).map(Self::new)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        T::allow_leading_dashes(context)
    }
}