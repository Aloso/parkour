use std::error::Error as _;
use std::time::Instant;

use palr::actions::{Action, SetOnce, SetPositional, SetSubcommand};
use palr::util::Flag;
use palr::{Error, FromInput, FromInputValue, Parse, StringInput};

fn main() {
    // Command {
    //      Argument(-h/--help) { Help } -> throws Error::EarlyExit,
    //      Subcommand(s/show) {
    //          PositionalArg { String }
    //          Argument(-h/--help) { Help } -> throws Error::EarlyExit,
    //          Argument(-o/--out) { Output }
    //          Argument(-s/--size) { u8 } [default: 4]
    //      }
    // }

    let start = Instant::now();

    match main_inner() {
        Ok(command) => {
            eprintln!("Took {:?}", start.elapsed());
            eprintln!("{:#?}", command);
        }
        Err(e) if e.is_no_value() || e.is_early_exit() => {
            eprintln!("Took {:?}", start.elapsed());
        }
        Err(e) => {
            eprint!("{}", e);
            let mut source = e.source();
            while let Some(s) = source {
                eprint!(": {}", s);
                source = s.source();
            }
            eprintln!();
        }
    }
}

fn main_inner() -> Result<Command, Error> {
    let mut input = StringInput::new(std::env::args());
    Command::try_from_input(&mut input, &())?
        .ok_or_else(|| Error::missing_argument("arguments"))
}

/// Main command
#[derive(Debug)]
struct Command {
    /// `-c/--color` option
    color: Option<bool>,
    /// `s/show` subcommand
    show: Option<Show>,
}

impl FromInput for Command {
    type Context = ();

    fn from_input<P: Parse>(input: &mut P, _: &()) -> Result<Self, Error> {
        input.bump_argument().unwrap();

        let mut show = None;
        let mut color = None;

        while !input.is_empty() {
            if input.parse_long_flag("") {
                // handle `--`
                input.set_ignore_dashes(true);
                continue;
            }

            if input.parse_long_flag("help") || input.parse_short_flag("h") {
                println!("Help");
                return Err(Error::early_exit());
            }

            if SetOnce(&mut color).apply(input, &Flag::LongShort("color", "c").into())? {
                continue;
            }

            if SetSubcommand(&mut show).apply(input, &())? {
                continue;
            }

            input.expect_empty()?;
        }
        Ok(Command { show, color })
    }
}

/// `s/show` subcommand
#[derive(Debug)]
struct Show {
    /// first positional argument
    pos1: String,
    /// `-o/--out` option
    out: Output,
    /// `-s/--size` option, default: 4
    size: u8,
}

impl FromInput for Show {
    type Context = ();

    fn from_input<P: Parse>(input: &mut P, _: &()) -> Result<Self, Error> {
        if input.parse_command("show") || input.parse_command("s") {
            let mut pos1 = None;
            let mut out = None;
            let mut size = None;

            while !input.is_empty() {
                if input.parse_long_flag("") {
                    // handle `--`
                    input.set_ignore_dashes(true);
                    continue;
                }

                if input.parse_long_flag("help") || input.parse_short_flag("h") {
                    println!("Help for `show` subcommand");
                    return Err(Error::early_exit());
                }

                if SetOnce(&mut out).apply(input, &Flag::LongShort("out", "o").into())? {
                    continue;
                }

                if SetOnce(&mut size)
                    .apply(input, &Flag::LongShort("size", "s").into())?
                {
                    continue;
                }

                if pos1.is_none()
                    && SetPositional(&mut pos1).apply(input, &"pos1".into())?
                {
                    continue;
                }

                input.expect_empty()?;
            }

            Ok(Show {
                pos1: pos1.ok_or_else(|| Error::missing_argument("pos1"))?,
                out: out.ok_or_else(|| Error::missing_argument("--out"))?,
                size: size.unwrap_or(4),
            })
        } else {
            Err(Error::no_value())
        }
    }
}

#[derive(Debug)]
enum Output {
    Rgb,
    Cmy,
    Cmyk,
    Hsv,
    Hsl,
    CieLab,
}

impl FromInputValue for Output {
    type Context = ();

    fn from_input_value(value: &str, _: &()) -> Result<Self, Error> {
        match value {
            "rgb" => Ok(Output::Rgb),
            "cmy" => Ok(Output::Cmy),
            "cmyk" => Ok(Output::Cmyk),
            "hsv" => Ok(Output::Hsv),
            "hsl" => Ok(Output::Hsl),
            "cielab" => Ok(Output::CieLab),
            v => Err(Error::unexpected_value(v, "rgb, cmy, cmyk, hsv, hsl or cielab")),
        }
    }
}
