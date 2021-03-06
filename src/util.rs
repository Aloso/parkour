//! Several utility types.

use std::fmt;
use std::fmt::Write as _;

use palex::ArgsInput;

use crate::actions::ApplyResult;
use crate::Parse;

/// The parsing context for a flag.
///
/// A flag is either short (i.e. it starts with a single dash) or long (i.e. it
/// starts with two dashes). Note that the dashes should **not** be written in
/// the string, i.e. use `Flag::Long("version")`, not `Flag::Long("--version")`.
///
/// Arguments can often be specified with a long and a short flag (e.g. `--help`
/// and `-h`); Use `Flag::LongShort("help", "h")` in this case. If an argument
/// has more than 2 flags, use `Flag::Many(vec![...])`.
#[derive(Debug, Clone)]
pub enum Flag<'a> {
    /// A short flag, like `-h`
    Short(&'a str),
    /// A long flag, like `--help`
    Long(&'a str),
    /// A flag with a long and a short alias, e.g. `-h,--help`.
    LongShort(&'a str, &'a str),
    /// A flag with multiple aliases
    Many(Vec<Flag<'a>>),
}

impl Flag<'_> {
    /// Returns the first alias of the flag as a [String].
    pub fn first_to_string(&self) -> String {
        match self {
            &Flag::Short(s) => format!("-{}", s),
            &Flag::Long(l) => format!("--{}", l),
            &Flag::LongShort(l, _) => format!("--{}", l),
            Flag::Many(v) => v[0].first_to_string(),
        }
    }

    /// Parses a flag from a [`Parse`] instance.
    pub fn from_input<'a>(input: &mut ArgsInput, context: &Flag<'a>) -> ApplyResult {
        Ok(match context {
            &Flag::Short(f) => input.parse_short_flag(f),
            &Flag::Long(f) => input.parse_long_flag(f),
            &Flag::LongShort(l, s) => {
                input.parse_long_flag(l) || input.parse_short_flag(s)
            }
            Flag::Many(flags) => {
                flags.iter().any(|flag| matches!(Self::from_input(input, flag), Ok(true)))
            }
        })
    }
}

impl fmt::Display for Flag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Flag::Short(s) => write!(f, "-{}", s),
            Flag::Long(l) => write!(f, "--{}", l),
            Flag::LongShort(l, s) => write!(f, "--{},-{}", l, s),
            Flag::Many(v) => {
                for (i, flag) in v.iter().enumerate() {
                    if i > 0 {
                        f.write_char(',')?;
                    }
                    fmt::Display::fmt(flag, f)?;
                }
                Ok(())
            }
        }
    }
}

/// The parsing context for a named argument, e.g. `--help=config`.
#[derive(Debug, Clone)]
pub struct ArgCtx<'a, C> {
    /// The flag before the argument value(s)
    pub flag: Flag<'a>,
    /// The context for the argument value(s)
    pub inner: C,
}

impl<'a, C> ArgCtx<'a, C> {
    /// Creates a new `ArgCtx` instance
    pub fn new(flag: Flag<'a>, inner: C) -> Self {
        Self { flag, inner }
    }
}

impl<'a, C: Default> From<Flag<'a>> for ArgCtx<'a, C> {
    fn from(flag: Flag<'a>) -> Self {
        ArgCtx { flag, inner: C::default() }
    }
}

/// The parsing context for a positional argument.
#[derive(Debug, Clone)]
pub struct PosCtx<'a, C> {
    /// The name of the positional argument, used in error messages
    pub name: &'a str,
    /// The context for the argument value
    pub inner: C,
}

impl<'a, C> PosCtx<'a, C> {
    /// Creates a new `PosCtx` instance
    pub fn new(name: &'a str, inner: C) -> Self {
        Self { name, inner }
    }
}

impl<'a, C: Default> From<&'a str> for PosCtx<'a, C> {
    fn from(name: &'a str) -> Self {
        PosCtx { name, inner: C::default() }
    }
}
