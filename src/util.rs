use std::fmt;
use std::fmt::Write as _;

use crate::actions::ApplyResult;
use crate::Parse;

#[derive(Debug, Clone)]
pub enum Flag<'a> {
    Short(&'a str),
    Long(&'a str),
    LongShort(&'a str, &'a str),
    Many(Box<[Flag<'a>]>),
}

impl Flag<'_> {
    pub fn first_to_string(&self) -> String {
        match self {
            &Flag::Short(s) => format!("-{}", s),
            &Flag::Long(l) => format!("--{}", l),
            &Flag::LongShort(l, _) => format!("--{}", l),
            Flag::Many(v) => v[0].first_to_string(),
        }
    }

    pub fn from_input<'a, P: Parse>(input: &mut P, context: &Flag<'a>) -> ApplyResult {
        Ok(match context {
            Flag::Short(f) => input.parse_short_flag(f),
            Flag::Long(f) => input.parse_long_flag(f),
            Flag::LongShort(l, s) => {
                input.parse_long_flag(l) || input.parse_short_flag(s)
            }
            Flag::Many(flags) => {
                flags.iter().any(|flag| Self::from_input(input, flag).is_ok())
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


#[derive(Debug, Clone)]
pub struct OptionCtx<'a, C> {
    pub flag: Flag<'a>,
    pub inner: C,
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


#[derive(Debug, Clone)]
pub struct PosCtx<'a, C> {
    pub name: &'a str,
    pub inner: C,
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
