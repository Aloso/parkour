use std::any::Any;
use std::fmt::Debug;

use args::{Argument, ArgumentTrait, Command, NamedArg, Number, ParamName, PositionalArg, Value};
use input::{Next, PalrInput};
pub use offset_string::OffsetString;

pub mod args;
pub mod input;
mod offset_string;
pub mod vec_input;

pub trait PalrParse {
    type Output;

    fn parse(&self, input: &mut PalrInput, allow_ws: bool) -> Result<Option<Self::Output>, Error>;
}

impl PalrParse for Argument {
    type Output = ArgResult;

    fn parse(&self, input: &mut PalrInput, _allow_ws: bool) -> Result<Option<ArgResult>, Error> {
        Ok(match self {
            Argument::Command(c) => c.parse(input, true)?.map(ArgResult::Command),
            Argument::NamedArg(a) => a.parse(input, true)?.map(ArgResult::Named),
            Argument::Other(o) => o.parse(input, true)?,
        })
    }
}

impl PalrParse for Command {
    type Output = CommandResult;

    fn parse(
        &self,
        input: &mut PalrInput,
        _allow_ws: bool,
    ) -> Result<Option<CommandResult>, Error> {
        if let Some(name) = input.eat_any_arg(&self.names) {
            let mut pa = self.positional_args.as_slice();
            let mut args = Vec::new();
            'looop: loop {
                if input.is_empty() {
                    break;
                }
                for arg in &self.args {
                    if let Some(arg_result) = arg.parse(input, true)? {
                        args.push(arg_result);
                        continue 'looop;
                    }
                }
                if let Some(arg) = pa.get(0) {
                    if let Some(arg_result) = arg.parse(input, true)? {
                        args.push(ArgResult::Positional(arg_result));
                        pa = &pa[1..];
                        continue 'looop;
                    }
                }
                if !input.is_empty() {
                    return Err(Error::Unexpected {
                        word: input.get_word().unwrap(),
                    });
                }
            }

            Ok(Some(CommandResult {
                name: name.clone(),
                args,
            }))
        } else {
            Ok(None)
        }
    }
}

impl PalrParse for PositionalArg {
    type Output = PositionalResult;

    fn parse(
        &self,
        input: &mut PalrInput,
        _allow_ws: bool,
    ) -> Result<Option<PositionalResult>, Error> {
        if self.value_count == (0, 0) {
            Ok(Some(PositionalResult { values: vec![] }))
        } else {
            let mut values = Vec::new();
            let mut count = 0;
            for _ in 0..self.value_count.1 {
                if input.is_empty() {
                    break;
                }
                if let Some(value) = self.value.parse(input, true)? {
                    values.push(value);
                    count += 1;
                } else {
                    break;
                }
            }

            if count < self.value_count.0 {
                return Err(Error::TooFewValues {
                    min: self.value_count.0,
                    count,
                });
            }

            Ok(Some(PositionalResult { values }))
        }
    }
}

impl PalrParse for NamedArg {
    type Output = NamedResult;

    fn parse(&self, input: &mut PalrInput, _allow_ws: bool) -> Result<Option<NamedResult>, Error> {
        if let Some(name) = input.eat_any_param(&self.names) {
            let allow_ws = matches!(input.peek(), Next::Arg(_) | Next::None);

            let mut values = Vec::new();
            let mut count = 0;
            for _ in 0..self.value_count.1 {
                if input.is_empty() {
                    break;
                }
                if let Some(value) = self.value.parse(input, allow_ws)? {
                    values.push(value);
                    count += 1;
                } else {
                    break;
                }
            }

            if count < self.value_count.0 {
                return Err(Error::TooFewValues {
                    min: self.value_count.0,
                    count,
                });
            }

            Ok(Some(NamedResult {
                name: name.clone(),
                values,
            }))
        } else {
            Ok(None)
        }
    }
}

impl PalrParse for Number {
    type Output = NumberResult;

    #[allow(unused_variables)]
    fn parse(&self, input: &mut PalrInput, allow_ws: bool) -> Result<Option<Self::Output>, Error> {
        let word = match input.peek_word() {
            Some(word) => word,
            None => return Ok(None),
        };
        if word.contains(|c: char| !c.is_ascii_digit()) {
            return Ok(None);
        }

        macro_rules! parse_num {
            ($word:ident, $min:ident, $max:ident, $variant:ident) => {{
                if let Ok(n) = $word.parse() {
                    if n < $min {
                        return Err(Error::NumberOutOfBounds { word: word.into() });
                    }
                    if n > $max {
                        return Err(Error::NumberOutOfBounds { word: word.into() });
                    }
                    Some(NumberResult::$variant(n))
                } else {
                    None
                }
            }};
        }

        let n = match *self {
            Number::I128 { min, max } => parse_num!(word, min, max, I128),
            Number::U128 { min, max } => parse_num!(word, min, max, U128),
            Number::I64 { min, max } => parse_num!(word, min, max, I64),
            Number::U64 { min, max } => parse_num!(word, min, max, U64),
            Number::I32 { min, max } => parse_num!(word, min, max, I32),
            Number::U32 { min, max } => parse_num!(word, min, max, U32),
            Number::I16 { min, max } => parse_num!(word, min, max, I16),
            Number::U16 { min, max } => parse_num!(word, min, max, U16),
            Number::I8 { min, max } => parse_num!(word, min, max, I8),
            Number::U8 { min, max } => parse_num!(word, min, max, U8),
            Number::F64 { min, max } => parse_num!(word, min, max, F64),
            Number::F32 { min, max } => parse_num!(word, min, max, F32),
        };
        if n.is_some() {
            input.get_word();
        }
        Ok(n)
    }
}

impl PalrParse for Value {
    type Output = ValueResult;

    fn parse(&self, input: &mut PalrInput, allow_ws: bool) -> Result<Option<Self::Output>, Error> {
        Ok(match self {
            Value::String => input.get_word().map(ValueResult::String),
            Value::Num(n) => n.parse(input, false)?.map(ValueResult::Num),
            Value::Other(o) => o.parse_value(input)?.map(ValueResult::Other),
            Value::Enum(e) => {
                if input.is_empty() {
                    None
                } else if let Some(name) = input.eat_any_word(e) {
                    // TODO: Allow consuming value until delimiter, like comma
                    Some(ValueResult::Enum(name.clone()))
                } else {
                    None
                }
            }
            Value::List { inner, value_count } => {
                // TODO: Ensure that no whitespace can be consume
                let mut values = Vec::new();
                let mut count = 0;
                for _ in 0..value_count.1 {
                    if input.is_empty() {
                        break;
                    }
                    if let Some(value) = inner.parse(input, allow_ws)? {
                        values.push(value);
                        count += 1;
                    } else {
                        break;
                    }
                }

                if count < value_count.0 {
                    return Err(Error::TooFewValues {
                        min: value_count.0,
                        count,
                    });
                }

                Some(ValueResult::List(values))
            }
        })
    }
}

impl PalrParse for Box<dyn ArgumentTrait> {
    type Output = ArgResult;

    fn parse(&self, input: &mut PalrInput, _allow_ws: bool) -> Result<Option<ArgResult>, Error> {
        Ok(self.parse_argument(input)?.map(ArgResult::Other))
    }
}

#[derive(Debug)]
pub enum ArgResult {
    Command(CommandResult),
    Named(NamedResult),
    Positional(PositionalResult),
    Other(Box<dyn ArgTraitResult>),
}

#[derive(Debug)]
pub struct CommandResult {
    pub name: String,
    pub args: Vec<ArgResult>,
}

#[derive(Debug)]
pub struct NamedResult {
    pub name: ParamName,
    pub values: Vec<ValueResult>,
}

#[derive(Debug)]
pub struct PositionalResult {
    pub values: Vec<ValueResult>,
}

#[derive(Debug)]
pub enum ValueResult {
    String(String),
    Num(NumberResult),
    Enum(String),
    List(Vec<ValueResult>),
    Other(Box<dyn ValueResultTrait>),
}

#[derive(Debug)]
pub enum NumberResult {
    I128(i128),
    U128(u128),
    I64(i64),
    U64(u64),
    I32(i32),
    U32(u32),
    I16(i16),
    U16(u16),
    I8(i8),
    U8(u8),
    F64(f64),
    F32(f32),
}

pub trait ArgTraitResult: Debug + Any {}

pub trait ValueResultTrait: Debug + Any {}

#[derive(Debug)]
pub enum Error {
    TooFewValues { min: usize, count: usize },
    InvalidNumber { word: String },
    NumberOutOfBounds { word: String },
    Unexpected { word: String },
}
