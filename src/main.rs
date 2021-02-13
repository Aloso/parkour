use std::time::Instant;

use palr::args::{Command, NamedArg, Number, PositionalArg, Value, ValueTrait};
use palr::input::PalrInput;
use palr::{PalrParse, ValueResultTrait};

fn main() {
    let start = Instant::now();

    let command = Command::new("colo")
        .arg(NamedArg::flag("help").alias("h"))
        .arg(
            Command::new("show")
                .alias("s")
                .positional_arg(PositionalArg::single(Value::list(Value::String)))
                .arg(NamedArg::flag("help").alias("h"))
                .arg(NamedArg::option("out", Value::other(OutputParser)).alias("o"))
                .arg(NamedArg::option("size", Value::Num(Number::FULL_U8)).alias("s")),
        );

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
