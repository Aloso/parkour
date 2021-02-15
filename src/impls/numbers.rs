use crate::{Error, FromInputValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberCtx<T> {
    min: T,
    max: T,
}

impl<T: Copy + PartialOrd + std::fmt::Display> NumberCtx<T> {
    fn must_include(&self, n: T) -> Result<T, Error> {
        if n >= self.min && n <= self.max {
            Ok(n)
        } else {
            Err(Error::Unexpected {
                word: format!(
                    "number {}, expected number in range [{}, {}]",
                    n, self.min, self.max
                ),
            })
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

macro_rules! number_impl {
    ($( $t:ident ),*) => {
        $(
            impl FromInputValue for $t {
                type Context = NumberCtx<$t>;

                fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
                    context.must_include(value.parse()?)
                }

                #[allow(unused_comparisons)]
                fn allow_leading_dashes(context: &Self::Context) -> bool {
                    context.min < 0
                }
            }
        )*
    };
}

default_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
number_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl FromInputValue for f32 {
    type Context = NumberCtx<f32>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        context.must_include(value.parse()?)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.min < 0.0
    }
}

impl FromInputValue for f64 {
    type Context = NumberCtx<f64>;

    fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
        context.must_include(value.parse()?)
    }

    fn allow_leading_dashes(context: &Self::Context) -> bool {
        context.min < 0.0
    }
}
