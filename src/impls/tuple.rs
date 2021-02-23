use crate::help::PossibleValues;
use crate::{Error, ErrorInner, FromInputValue};

#[derive(Debug)]
pub struct TupleCtx<C> {
    pub delimiter: char,
    pub inner: C,
}

impl<C> TupleCtx<C> {
    pub fn new(delimiter: char, inner: C) -> Self {
        Self { delimiter, inner }
    }
}

impl<C: Default> Default for TupleCtx<C> {
    fn default() -> Self {
        TupleCtx { delimiter: ',', inner: C::default() }
    }
}

macro_rules! impl_tuple {
    ($( $t:ident $v:ident $i:tt ),* $(,)?) => {
        impl<$( $t: FromInputValue ),*> FromInputValue for ($( $t ),* ,) {
            type Context = TupleCtx<($( $t::Context ),* ,)>;

            fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error> {
                let mut iter = value.split(context.delimiter);

                $(
                    let $v = $t::from_input_value(
                        iter.next().ok_or_else(|| ErrorInner::IncompleteValue($i + 1))?,
                        &context.inner.$i,
                    )?;
                )*

                Ok(($( $v ),* ,))
            }

            fn allow_leading_dashes(context: &Self::Context) -> bool {
                T1::allow_leading_dashes(&context.inner.0)
            }

            fn possible_values(context: &Self::Context) -> Option<PossibleValues> {
                T1::possible_values(&context.inner.0)
            }
        }
    };
}

impl_tuple!(
    T1 v1 0,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
    T7 v7 6,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
    T7 v7 6,
    T8 v8 7,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
    T7 v7 6,
    T8 v8 7,
    T9 v9 8,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
    T7 v7 6,
    T8 v8 7,
    T9 v9 8,
    T10 v10 9,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
    T7 v7 6,
    T8 v8 7,
    T9 v9 8,
    T10 v10 9,
    T11 v11 10,
);
impl_tuple!(
    T1 v1 0,
    T2 v2 1,
    T3 v3 2,
    T4 v4 3,
    T5 v5 4,
    T6 v6 5,
    T7 v7 6,
    T8 v8 7,
    T9 v9 8,
    T10 v10 9,
    T11 v11 10,
    T12 v12 11,
);
