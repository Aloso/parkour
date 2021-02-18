use crate::util::{ArgCtx, Flag};
use crate::{Error, Parse};

/// Trait for extracting information from the command-line input. This is
/// implemented for flags, positional and named arguments, subcommands, etc.
///
/// ### Implementation
///
/// ```
/// # use parkour::prelude::*;
/// // The struct we want to crate from a positional number argument
/// struct Foo(usize);
///
/// // Information that is available while parsing. When `even` is true,
/// // we only accept even numbers. Otherwise we only accept odd numbers.
/// struct FooCtx {
///     even: bool,
/// }
///
/// impl FromInput for Foo {
///     type Context = FooCtx;
///
///     fn from_input<P: Parse>(input: &mut P, context: &FooCtx) -> Result<Self, parkour::Error> {
///         let num: usize = input.parse_value(&Default::default())?;
///
///         if context.even && num % 2 != 0 {
///             Err(parkour::Error::unexpected_value(num, "even number"))
///         } else if !context.even && num % 2 == 0 {
///             Err(parkour::Error::unexpected_value(num, "odd number"))
///         } else {
///             Ok(Foo(num))
///         }
///     }
/// }
/// ```
pub trait FromInput: Sized {
    /// Information that is available while parsing
    type Context;

    /// Extract information from the command-line input.
    fn from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Self, Error>;

    /// Extract information from the command-line input, but convert
    /// [`Error::no_value`] to [`Option::None`]. This is useful when you want to
    /// bubble up all errors except for [`Error::no_value`]:
    ///
    /// ```no_run
    /// # use parkour::prelude::*;
    /// # let input: &mut parkour::StringInput = todo!();
    /// if let Some(value) = bool::try_from_input(input, &Flag::Short("b").into())? {
    ///     // do something with value
    /// }
    /// # Ok::<(), parkour::Error>(())
    /// ```
    fn try_from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Option<Self>, Error> {
        match Self::from_input(input, context) {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.is_no_value() => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// Trait for parsing a _value_. A value can be
/// - a positional argument
/// - a string following a flag; e.g in `--foo bar` or `--foo=bar`, the `bar`
///   part is a value. A flag can be followed by multiple values, e.g. `--foo
///   bar baz` or `--foo=bar,baz`
///
/// To parse values, they are first converted to a string slice. By default, an
/// argument that starts with a dash can't be parsed as a value, unless you
/// implement the `allow_leading_dashes` method.
pub trait FromInputValue: Sized {
    /// Information that is available while parsing
    type Context;

    /// The function that parses the string. This function is usually not
    /// invoked directly. Instead you can use [`Parse::parse_value`] and
    /// [`Parse::try_parse_value`]:
    ///
    /// ```no_run
    /// # use parkour::prelude::*;
    /// let mut input = parkour::parser();
    /// let n: i32 = input.parse_value(&NumberCtx { min: -1000, max: 1000 })?;
    /// # Ok::<(), parkour::Error>(())
    /// ```
    fn from_input_value(value: &str, context: &Self::Context) -> Result<Self, Error>;

    /// This function specifies whether this argument may start with leading
    /// dashes. For example, this returns `true` for numbers that can be
    /// negative. The default is `false`.
    fn allow_leading_dashes(_context: &Self::Context) -> bool {
        false
    }
}


impl<T: FromInputValue> FromInput for T {
    type Context = ArgCtx<'static, T::Context>;

    fn from_input<P: Parse>(
        input: &mut P,
        context: &Self::Context,
    ) -> Result<Self, Error> {
        if Flag::from_input(input, &context.flag)? {
            match input.parse_value(&context.inner) {
                Ok(value) => Ok(value),
                Err(e) if e.is_no_value() => {
                    Err(Error::missing_value()
                        .with_source(Error::in_argument(&context.flag)))
                }
                Err(e) => Err(e),
            }
        } else {
            Err(Error::no_value())
        }
    }
}
