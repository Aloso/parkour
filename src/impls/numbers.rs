use std::num::*;

use crate::help::PossibleValues;
use crate::{Error, FromInputValue};

/// The parsing context for numeric types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberCtx<T> {
    /// The smallest accepted number
    pub min: T,
    /// The largest accepted number
    pub max: T,
}

impl<T: Copy + PartialOrd + FromInputValue<Context = Self> + std::fmt::Display>
    NumberCtx<T>
{
    fn must_include(&self, n: T) -> Result<T, Error> {
        if n >= self.min && n <= self.max {
            Ok(n)
        } else {
            Err(Error::unexpected_value(
                format!("number {}", n),
                T::possible_values(self),
            ))
        }
    }
}

macro_rules! default_impl {
    ($( $t:ident ),*) => {
        $(
            impl Default for NumberCtx<$t> {
                fn default() -> Self {
                    NumberCtx { min: $t::MIN, max: $t::MAX }
                }
            }
        )*
    };
}

macro_rules! from_input_value {
    (signed -> $( $t:ident ),*) => {
        $(
            impl FromInputValue for $t {
                type Context = NumberCtx<$t>;

                fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
                    context.must_include(value.parse()?)
                }

                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min.is_negative()
                }

                fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
                    Some(PossibleValues::Other(
                        match (context.min, context.max) {
                            ($t::MIN, $t::MAX) => "integer".into(),
                            ($t::MIN, max) => format!("integer at most {}", max),
                            (min, $t::MAX) => format!("integer at least {}", min),
                            (min, max) => format!("integer between {} and {}", min, max),
                        }
                    ))
                }
            }
        )*
    };
    (signed_nonzero -> $( $t:ident ),*) => {
        $(
            impl FromInputValue for $t {
                type Context = NumberCtx<$t>;

                fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
                    context.must_include(value.parse()?)
                }

                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min.get().is_negative()
                }

                fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
                    Some(PossibleValues::Other(
                        format!("integer between {} and {}", context.min, context.max),
                    ))
                }
            }
        )*
    };
    (unsigned -> $( $t:ident ),*) => {
        $(
            impl FromInputValue for $t {
                type Context = NumberCtx<$t>;

                fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
                    context.must_include(value.parse()?)
                }

                fn allow_leading_dashes(_: &Self::Context) -> bool { false }

                fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
                    Some(PossibleValues::Other(
                        format!("integer between {} and {}", context.min, context.max),
                    ))
                }
            }
        )*
    };
    (float -> $( $t:ident ),*) => {
        $(
            impl FromInputValue for $t {
                type Context = NumberCtx<$t>;

                fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
                    context.must_include(value.parse()?)
                }

                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min.is_sign_negative()
                }

                fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
                    Some(PossibleValues::Other(
                        match (context.min, context.max) {
                            (min, max) if min == $t::MIN && max == $t::MAX => "number".into(),
                            (min, max) if min == $t::MIN => format!("number at most {}", max),
                            (min, max) if max == $t::MAX => format!("number at least {}", min),
                            (min, max) => format!("number between {} and {}", min, max),
                        }
                    ))
                }
            }
        )*
    };
}

default_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

from_input_value! { signed -> i8, i16, i32, i64, i128, isize }
from_input_value! { signed_nonzero ->
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
}
from_input_value! { unsigned ->
    u8, u16, u32, u64, u128, usize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize
}
from_input_value! { float -> f32, f64 }
