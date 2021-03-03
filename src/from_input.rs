use palex::ArgsInput;

use crate::help::PossibleValues;
use crate::util::{ArgCtx, Flag};
use crate::{Error, ErrorInner, Parse};

/// Trait for extracting information from the command-line input. This is
/// implemented for flags, positional and named arguments, subcommands, etc.
///
/// ### Implementation
///
/// * Return `Err(parkour::Error::no_value())` if the parsed value isn't present
/// * The lifetime parameter is for the context. If the context type doesn't
///   have a lifetime, use `'static`.
/// * Make sure that you consume exactly as much text as you need. Most methods
///   from [`Parse`] should take care of this automatically. Avoid using
///   lower-level functions, such as [`parkour::ArgsInput::current`] or
///   [`parkour::ArgsInput::bump`], which might not advance the input correctly.
///
/// ### Example
///
/// ```
/// # use parkour::prelude::*;
/// use parkour::help::PossibleValues;
///
/// // The struct we want to crate from a positional number argument
/// struct Foo(usize);
///
/// // Information that is available while parsing. When `even` is true,
/// // we only accept even numbers. Otherwise we only accept odd numbers.
/// struct FooCtx {
///     even: bool,
/// }
///
/// impl FromInput<'static> for Foo {
///     type Context = FooCtx;
///
///     fn from_input(input: &mut ArgsInput, context: &FooCtx) -> parkour::Result<Self> {
///         let num: usize = input.parse_value(&Default::default())?;
///
///         if context.even && num % 2 != 0 {
///             Err(parkour::Error::unexpected_value(
///                 num,
///                 Some(PossibleValues::Other("even number".into())),
///             ))
///         } else if !context.even && num % 2 == 0 {
///             Err(parkour::Error::unexpected_value(
///                 num,
///                 Some(PossibleValues::Other("odd number".into())),
///             ))
///         } else {
///             Ok(Foo(num))
///         }
///     }
/// }
/// ```
pub trait FromInput<'a>: Sized {
    /// Information that is available while parsing
    type Context: 'a;

    /// Extract information from the command-line input.
    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self, Error>;

    /// Extract information from the command-line input, but convert
    /// [`Error::no_value`] to [`Option::None`]. This is useful when you want to
    /// bubble up all errors except for [`Error::no_value`]:
    ///
    /// ```no_run
    /// # use parkour::prelude::*;
    /// # let input: &mut parkour::ArgsInput = todo!();
    /// if let Some(value) = bool::try_from_input(input, &Flag::Short("b").into())? {
    ///     // do something with value
    /// }
    /// # Ok::<(), parkour::Error>(())
    /// ```
    fn try_from_input(
        input: &mut ArgsInput,
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
///
/// The lifetime parameter is for the context. If the context type doesn't have
/// a lifetime, use `'static`.
pub trait FromInputValue<'a>: Sized {
    /// Information that is available while parsing
    type Context: 'a;

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
    fn allow_leading_dashes(_: &Self::Context) -> bool {
        false
    }

    /// Returns a list or short description of all the accepted values
    fn possible_values(context: &Self::Context) -> Option<PossibleValues>;
}

impl<'a, T: FromInputValue<'a>> FromInput<'a> for T
where
    T::Context: 'a,
{
    type Context = ArgCtx<'a, T::Context>;

    fn from_input(input: &mut ArgsInput, context: &Self::Context) -> Result<Self, Error> {
        if Flag::from_input(input, &context.flag)? {
            match input.parse_value(&context.inner) {
                Ok(value) => Ok(value),
                Err(e) if e.is_no_value() => Err(Error::missing_value()
                    .chain(ErrorInner::InArgument(context.flag.first_to_string()))),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::no_value())
        }
    }
}
