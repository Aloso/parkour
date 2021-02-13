use std::fmt::Debug;

use crate::{input::PalrInput, ArgTraitResult, Error, ValueResultTrait};

#[derive(Debug)]
pub enum Argument {
    Command(Command),
    NamedArg(NamedArg),
    Other(Box<dyn ArgumentTrait>),
}

#[derive(Debug)]
pub struct Command {
    pub names: Vec<String>,
    pub positional_args: Vec<PositionalArg>,
    pub args: Vec<Argument>,
}

#[derive(Debug)]
pub struct NamedArg {
    pub names: Vec<ParamName>,
    pub value: Value,
    pub value_count: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamName {
    Long(String),
    Short(String),
}

impl From<String> for ParamName {
    fn from(s: String) -> Self {
        if s.chars().nth(1).is_some() {
            ParamName::Long(s)
        } else {
            ParamName::Short(s)
        }
    }
}

impl From<&'_ str> for ParamName {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

#[derive(Debug)]
pub struct PositionalArg {
    pub value: Value,
    pub value_count: (usize, usize),
}

#[derive(Debug)]
pub enum Number {
    I128 { min: i128, max: i128 },
    U128 { min: u128, max: u128 },
    I64 { min: i64, max: i64 },
    U64 { min: u64, max: u64 },
    I32 { min: i32, max: i32 },
    U32 { min: u32, max: u32 },
    I16 { min: i16, max: i16 },
    U16 { min: u16, max: u16 },
    I8 { min: i8, max: i8 },
    U8 { min: u8, max: u8 },
    F64 { min: f64, max: f64 },
    F32 { min: f32, max: f32 },
}

#[derive(Debug)]
pub enum Value {
    String,
    Num(Number),
    Enum(Vec<String>),
    List {
        inner: Box<Value>,
        value_count: (usize, usize),
    },
    Other(Box<dyn ValueTrait>),
}

pub trait ArgumentTrait: Debug {
    fn parse_argument(
        &self,
        input: &mut PalrInput,
    ) -> Result<Option<Box<dyn ArgTraitResult>>, Error>;
}

pub trait ValueTrait: Debug {
    fn parse_value(
        &self,
        input: &mut PalrInput,
    ) -> Result<Option<Box<dyn ValueResultTrait>>, Error>;
}
