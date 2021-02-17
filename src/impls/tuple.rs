use crate::{Error, FromInputValue};

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
    ($( $t:ident $v:ident $s:literal $i:tt ),* $(,)?) => {
        impl<$( $t: FromInputValue ),*> FromInputValue for ($( $t ),* ,) {
            type Context = TupleCtx<($( $t::Context ),* ,)>;

            fn from_input_value(value: &str, context: Self::Context) -> Result<Self, Error> {
                let mut iter = value.split(context.delimiter);

                $(
                    let $v = $t::from_input_value(
                        iter.next().ok_or_else(|| Error::IncompleteValue { part: $s.into() })?,
                        context.inner.$i,
                    )?;
                )*

                Ok(($( $v ),* ,))
            }
        }
    };
}

impl_tuple!(
    T1 v1 "1" 0,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
    T7 v7 "7" 6,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
    T7 v7 "7" 6,
    T8 v8 "8" 7,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
    T7 v7 "7" 6,
    T8 v8 "8" 7,
    T9 v9 "9" 8,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
    T7 v7 "7" 6,
    T8 v8 "8" 7,
    T9 v9 "9" 8,
    T10 v10 "10" 9,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
    T7 v7 "7" 6,
    T8 v8 "8" 7,
    T9 v9 "9" 8,
    T10 v10 "10" 9,
    T11 v11 "11" 10,
);
impl_tuple!(
    T1 v1 "1" 0,
    T2 v2 "2" 1,
    T3 v3 "3" 2,
    T4 v4 "4" 3,
    T5 v5 "5" 4,
    T6 v6 "6" 5,
    T7 v7 "7" 6,
    T8 v8 "8" 7,
    T9 v9 "9" 8,
    T10 v10 "10" 9,
    T11 v11 "11" 10,
    T12 v12 "12" 11,
);
