use std::num::*;

use crate::{Error, FromInputValue};

/// The parsing context for numeric types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberCtx<T> {
    /// The smallest accepted number
    pub min: T,
    /// The largest accepted number
    pub max: T,
}

impl<T: Copy + PartialOrd + std::fmt::Display> NumberCtx<T> {
    fn must_include(&self, n: T) -> Result<T, Error> {
        if n >= self.min && n <= self.max {
            Ok(n)
        } else {
            Err(Error::unexpected_value(
                format!("number {}", n),
                format!("number between {} and {}", self.min, self.max),
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

                #[allow(unused_comparisons)]
                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min.is_negative()
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

                #[allow(unused_comparisons)]
                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min.get().is_negative()
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

                #[allow(unused_comparisons)]
                fn allow_leading_dashes(_: &Self::Context) -> bool { false }
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

                #[allow(unused_comparisons)]
                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min.is_sign_negative()
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
