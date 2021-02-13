use std::fmt::Debug;

use crate::{input::PalrInput, ArgTraitResult, Error, ValueResultTrait};

#[derive(Debug)]
pub enum Argument {
    Command(Command),
    NamedArg(NamedArg),
    Other(Box<dyn ArgumentTrait>),
}

impl From<Command> for Argument {
    fn from(c: Command) -> Self {
        Argument::Command(c)
    }
}

impl From<NamedArg> for Argument {
    fn from(n: NamedArg) -> Self {
        Argument::NamedArg(n)
    }
}

#[derive(Debug)]
pub struct Command {
    pub names: Vec<String>,
    pub positional_args: Vec<PositionalArg>,
    pub args: Vec<Argument>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            names: vec![name.into()],
            positional_args: vec![],
            args: vec![],
        }
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.names.push(alias.into());
        self
    }

    pub fn arg(mut self, arg: impl Into<Argument>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn positional_arg(mut self, arg: PositionalArg) -> Self {
        self.positional_args.push(arg);
        self
    }
}

#[derive(Debug)]
pub struct NamedArg {
    pub names: Vec<ParamName>,
    pub value: Value,
    pub value_count: (usize, usize),
}

impl NamedArg {
    pub fn flag(name: impl Into<ParamName>) -> Self {
        Self {
            names: vec![name.into()],
            value: Value::String,
            value_count: (0, 0),
        }
    }

    pub fn option(name: impl Into<ParamName>, value: Value) -> Self {
        Self {
            names: vec![name.into()],
            value,
            value_count: (1, 1),
        }
    }

    pub fn alias(mut self, alias: impl Into<ParamName>) -> Self {
        self.names.push(alias.into());
        self
    }

    pub fn value_count(mut self, min: usize, max: usize) -> Self {
        self.value_count = (min, max);
        self
    }
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

impl PositionalArg {
    pub fn single(value: Value) -> Self {
        Self {
            value,
            value_count: (1, 1),
        }
    }
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

impl Number {
    pub const FULL_I128: Number = Number::I128 {
        min: i128::MIN,
        max: i128::MAX,
    };
    pub const FULL_U128: Number = Number::U128 {
        min: u128::MIN,
        max: u128::MAX,
    };
    pub const FULL_I64: Number = Number::I64 {
        min: i64::MIN,
        max: i64::MAX,
    };
    pub const FULL_U64: Number = Number::U64 {
        min: u64::MIN,
        max: u64::MAX,
    };
    pub const FULL_I32: Number = Number::I32 {
        min: i32::MIN,
        max: i32::MAX,
    };
    pub const FULL_U32: Number = Number::U32 {
        min: u32::MIN,
        max: u32::MAX,
    };
    pub const FULL_I16: Number = Number::I16 {
        min: i16::MIN,
        max: i16::MAX,
    };
    pub const FULL_U16: Number = Number::U16 {
        min: u16::MIN,
        max: u16::MAX,
    };
    pub const FULL_I8: Number = Number::I8 {
        min: i8::MIN,
        max: i8::MAX,
    };
    pub const FULL_U8: Number = Number::U8 {
        min: u8::MIN,
        max: u8::MAX,
    };
    pub const FULL_F64: Number = Number::F64 {
        min: f64::MIN,
        max: f64::MAX,
    };
    pub const FULL_F32: Number = Number::F32 {
        min: f32::MIN,
        max: f32::MAX,
    };
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

impl Value {
    pub fn list(value: Value) -> Self {
        Value::List {
            inner: Box::new(value),
            value_count: (1, usize::MAX),
        }
    }

    pub fn other<T: ValueTrait + 'static>(value: T) -> Self {
        Value::Other(Box::new(value))
    }
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
