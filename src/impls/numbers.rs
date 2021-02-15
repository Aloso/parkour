use crate::{Error, Parse};

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

macro_rules! number_impl {
    ($( $t:ident ),*) => {
        $(
            impl Default for NumberCtx<$t> {
                fn default() -> Self {
                    NumberCtx { min: $t::MIN, max: $t::MAX }
                }
            }

            impl Parse for $t {
                type Context = NumberCtx<$t>;

                fn parse_from_value(value: &str, context: Self::Context) -> Result<Self, Error> {
                    context.must_include(value.parse()?)
                }
            }
        )*
    }
}

number_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64);
