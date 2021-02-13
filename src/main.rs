use std::time::Instant;

use palr::args::{Argument, Command, NamedArg, Number, PositionalArg, Value, ValueTrait};
use palr::input::PalrInput;
use palr::{PalrParse, ValueResultTrait};

fn main() {
    let start = Instant::now();

    let command = Command {
        names: vec!["colo".into()],
        positional_args: vec![],
        args: vec![
            Argument::NamedArg(NamedArg {
                names: vec!["help".into(), "h".into()],
                value: Value::String,
                value_count: (0, 0),
            }),
            Argument::Command(Command {
                names: vec!["show".into(), "s".into()],
                positional_args: vec![PositionalArg {
                    value: Value::List {
                        inner: Box::new(Value::String),
                        value_count: (0, usize::MAX),
                    },
                    value_count: (1, 1),
                }],
                args: vec![
                    Argument::NamedArg(NamedArg {
                        names: vec!["help".into(), "h".into()],
                        value: Value::String,
                        value_count: (0, 0),
                    }),
                    Argument::NamedArg(NamedArg {
                        names: vec!["out".into(), "o".into()],
                        value: Value::Other(Box::new(OutputParser)),
                        value_count: (1, 1),
                    }),
                    Argument::NamedArg(NamedArg {
                        names: vec!["size".into(), "s".into()],
                        value: Value::Num(Number::U8 { min: 0, max: 255 }),
                        value_count: (1, 1),
                    }),
                ],
            }),
        ],
    };

    let mut args = std::env::args();
    let _ = args.next();
    let mut input = PalrInput::new(args);

    match command.parse(&mut input, true) {
        Ok(Some(res)) => {
            let time = start.elapsed();
            println!("{:#?}", res);
            println!("Parsing took {:?}", time);
        }
        e => {
            let time = start.elapsed();
            println!("{:#?}", e);
            println!("Parsing took {:?}", time);
        }
    }
}

#[derive(Debug)]
struct OutputParser;

#[derive(Debug)]
enum Output {
    Rgb,
    Cmy,
    Cmyk,
    Hsv,
    Hsl,
    Lch,
    Luv,
    Lab,
    Hunterlab,
    Xyz,
    Yxy,
    Gry,
    Hex,
    Html,
}

impl ValueTrait for OutputParser {
    fn parse_value(
        &self,
        input: &mut PalrInput,
    ) -> Result<Option<Box<dyn palr::ValueResultTrait>>, palr::Error> {
        let output = match input.peek_word() {
            Some("rgb") => Output::Rgb,
            Some("cmy") => Output::Cmy,
            Some("cmyk") => Output::Cmyk,
            Some("hsv") => Output::Hsv,
            Some("hsl") => Output::Hsl,
            Some("lch") => Output::Lch,
            Some("luv") => Output::Luv,
            Some("lab") => Output::Lab,
            Some("hunterlab") => Output::Hunterlab,
            Some("xyz") => Output::Xyz,
            Some("yxy") => Output::Yxy,
            Some("gry") => Output::Gry,
            Some("hex") => Output::Hex,
            Some("html") => Output::Html,
            _ => return Ok(None),
        };
        input.get_word();
        Ok(Some(Box::new(output)))
    }
}

impl ValueResultTrait for Output {}
