use crate::{Error, FromInput, FromInputValue, Parse};

pub trait Action<C> {
    fn apply<P: Parse>(self, input: &mut P, context: &C) -> ApplyResult;
}

pub type ApplyResult = Result<bool, Error>;

pub struct SetOnce<'a, T>(pub &'a mut T);
pub struct Unset<'a, T>(pub &'a mut T);
pub struct Set<'a, T>(pub &'a mut T);
pub struct Reset<'a, T>(pub &'a mut T);
pub struct Inc<'a, T>(pub &'a mut T);
pub struct Dec<'a, T>(pub &'a mut T);
pub struct Append<'a, T>(pub &'a mut T);

pub struct SetPositional<'a, T>(pub &'a mut T);

pub struct SetSubcommand<'a, T>(pub &'a mut T);

pub enum Flag<'a> {
    Short(&'a str),
    Long(&'a str),
    LongShort(&'a str, &'a str),
    Many(Box<[Flag<'a>]>),
}

impl Flag<'_> {
    fn first_to_string(&self) -> String {
        match self {
            &Flag::Short(s) => format!("-{}", s),
            &Flag::Long(l) => format!("-{}", l),
            &Flag::LongShort(l, _) => format!("-{}", l),
            Flag::Many(v) => v[0].first_to_string(),
        }
    }
}

pub struct OptionCtx<'a, C> {
    flag: Flag<'a>,
    inner: C,
}

impl<'a, C> OptionCtx<'a, C> {
    pub fn new(flag: Flag<'a>, inner: C) -> Self {
        Self { flag, inner }
    }
}

impl<'a, C: Default> From<Flag<'a>> for OptionCtx<'a, C> {
    fn from(flag: Flag<'a>) -> Self {
        OptionCtx { flag, inner: C::default() }
    }
}

fn flag_from_input<'a, P: Parse>(input: &mut P, context: &Flag<'a>) -> ApplyResult {
    Ok(match context {
        Flag::Short(f) => input.parse_short_flag(f),
        Flag::Long(f) => input.parse_long_flag(f),
        Flag::LongShort(l, s) => input.parse_long_flag(l) || input.parse_short_flag(s),
        Flag::Many(flags) => {
            for flag in flags.iter() {
                if flag_from_input(input, flag).is_ok() {
                    return Ok(true);
                }
            }
            false
        }
    })
}

impl<T: FromInputValue> FromInput for T {
    type Context = OptionCtx<'static, T::Context>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Self, Error> {
        flag_from_input(input, &context.flag)?;
        match input.parse_value(&context.inner) {
            Ok(value) => Ok(value),
            Err(Error::NoValue) => Err(Error::MissingValue),
            Err(e) => Err(e),
        }
    }
}

impl<T: FromInputValue> Action<T::Context> for SetPositional<'_, T> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse_value(context)? {
            *self.0 = s;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<T: FromInput> Action<T::Context> for SetSubcommand<'_, T> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse(context)? {
            *self.0 = s;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<T: FromInput> Action<T::Context> for SetSubcommand<'_, Option<T>> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = input.try_parse(context)? {
            if self.0.is_some() {
                return Err(Error::TooManyOptionOccurrences {
                    option: "subcommand".to_string(),
                    max: 1,
                });
            }
            *self.0 = Some(s);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub struct PosCtx<'a, C> {
    name: &'a str,
    inner: C,
}

impl<'a, C> PosCtx<'a, C> {
    pub fn new(name: &'a str, inner: C) -> Self {
        Self { name, inner }
    }
}

impl<'a, C: Default> From<&'a str> for PosCtx<'a, C> {
    fn from(name: &'a str) -> Self {
        PosCtx { name, inner: C::default() }
    }
}

impl<'a, T: FromInputValue> Action<PosCtx<'a, T::Context>>
    for SetPositional<'_, Option<T>>
{
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &PosCtx<'a, T::Context>,
    ) -> ApplyResult {
        if let Some(s) = input.try_parse_value(&context.inner)? {
            if self.0.is_some() {
                return Err(Error::TooManyOptionOccurrences {
                    option: context.name.to_string(),
                    max: 1,
                });
            }
            *self.0 = Some(s);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<T: FromInput> Action<T::Context> for Set<'_, T> {
    fn apply<P: Parse>(self, input: &mut P, context: &T::Context) -> ApplyResult {
        if let Some(s) = T::try_from_input(input, context)? {
            *self.0 = s;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a> Action<Flag<'a>> for Set<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if flag_from_input(input, context)? {
            *self.0 = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a> Action<Flag<'a>> for Reset<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if flag_from_input(input, context)? {
            *self.0 = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a> Action<Flag<'a>> for SetOnce<'_, bool> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if flag_from_input(input, context)? {
            if *self.0 {
                return Err(Error::TooManyOptionOccurrences {
                    option: context.first_to_string(),
                    max: 1,
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
        if flag_from_input(input, context)? {
            if !*self.0 {
                return Err(Error::TooManyOptionOccurrences {
                    option: context.first_to_string(),
                    max: 1,
                });
            }
            *self.0 = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}


impl<V: FromInputValue> Action<OptionCtx<'static, V::Context>> for Set<'_, Option<V>> {
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &OptionCtx<'static, V::Context>,
    ) -> ApplyResult {
        match input.try_parse(context)? {
            Some(s) => {
                *self.0 = Some(s);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl<V: FromInputValue> Action<OptionCtx<'static, V::Context>>
    for SetOnce<'_, Option<V>>
{
    fn apply<P: Parse>(
        self,
        input: &mut P,
        context: &OptionCtx<'static, V::Context>,
    ) -> ApplyResult {
        match input.try_parse(context)? {
            Some(s) => {
                if self.0.is_some() {
                    return Err(Error::TooManyOptionOccurrences {
                        option: context.flag.first_to_string(),
                        max: 1,
                    });
                }
                *self.0 = Some(s);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl<'a, V: FromInputValue> Action<Flag<'a>> for Reset<'_, Option<V>> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if flag_from_input(input, context)? {
            *self.0 = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, V: FromInputValue> Action<Flag<'a>> for Unset<'_, Option<V>> {
    fn apply<P: Parse>(self, input: &mut P, context: &Flag<'a>) -> ApplyResult {
        if flag_from_input(input, context)? {
            if self.0.is_none() {
                return Err(Error::TooManyOptionOccurrences {
                    option: context.first_to_string(),
                    max: 1,
                });
            }
            *self.0 = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}


/*
impl<T> Action for Unset<'_, T> {}

impl<T> Action for Replace<'_, T> {}

impl<T> Action for Reset<'_, T> {}

impl<T> Action for Inc<'_, T> {}

impl<T> Action for Dec<'_, T> {}

impl<T> Action for Append<'_, T> {}
*/
