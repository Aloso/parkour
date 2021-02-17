use std::time::Instant;

use palr::actions::{Action, Flag, SetOnce, SetPositional, SetSubcommand};
use palr::util::MapNoValue;
use palr::StringInput;
use palr::{Error, FromInput, FromInputValue, Parse};

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

    match main_() {
        Ok(command) => {
            eprintln!("Took {:?}", start.elapsed());
            eprintln!("{:#?}", command);
        }
        Err(e) => match e {
            Error::NoValue | Error::EarlyExit => eprintln!("Took {:?}", start.elapsed()),
            e => eprintln!("{}", anyhow::Error::new(e)),
        },
    }
}

fn main_() -> Result<Command, Error> {
    let mut input = StringInput::new(std::env::args());
    Command::from_input(&mut input, &()).map_no_value(|| Error::MissingOption {
        option: "no arguments provided".to_string(),
    })
}

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

impl FromInputValue for Output {
    type Context = ();

    fn from_input_value(value: &str, _: &()) -> Result<Self, Error> {
        Ok(match value {
            "rgb" => Output::Rgb,
            "cmy" => Output::Cmy,
            "cmyk" => Output::Cmyk,
            "hsv" => Output::Hsv,
            "hsl" => Output::Hsl,
            "lch" => Output::Lch,
            "luv" => Output::Luv,
            "lab" => Output::Lab,
            "hunterlab" => Output::Hunterlab,
            "xyz" => Output::Xyz,
            "yxy" => Output::Yxy,
            "gry" => Output::Gry,
            "hex" => Output::Hex,
            "html" => Output::Html,
            word => {
                return Err(Error::Unexpected {
                    word: format!(
                        "value {:?}, expected one of: rgb cmy, cmyk, hsv, hsl, \
                        lch, luv, lab, hunterlab, xyz, yxy, gry, hex, html",
                        word
                    ),
                })
            }
        })
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
    /// `-c/--color` option
    color: Option<bool>,
}

impl FromInput for Show {
    type Context = ();

    fn from_input<P: Parse>(input: &mut P, _: &()) -> Result<Self, Error> {
        if input.parse_command("show") || input.parse_command("s") {
            let mut pos1 = None;
            let mut out = None;
            let mut size = None;
            let mut color = None;

            while !input.is_empty() {
                if input.parse_long_flag("") {
                    input.set_ignore_dashes(true);
                    continue;
                }

                if input.parse_long_flag("help") || input.parse_short_flag("h") {
                    println!("Help for `show` subcommand");
                    return Err(Error::EarlyExit);
                }

                if SetOnce(&mut out).apply(input, &Flag::LongShort("out", "o").into())? {
                    continue;
                }

                if SetOnce(&mut size)
                    .apply(input, &Flag::LongShort("size", "s").into())?
                {
                    continue;
                }

                if SetOnce(&mut color)
                    .apply(input, &Flag::LongShort("color", "c").into())?
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
                pos1: pos1.ok_or_else(|| Error::MissingOption {
                    option: "positional argument".into(),
                })?,
                out: out.ok_or_else(|| Error::MissingOption {
                    option: "option `--out`".into(),
                })?,
                size: size.unwrap_or(4),
                color,
            })
        } else {
            Err(Error::NoValue)
        }
    }
}

/// Main command
#[derive(Debug)]
struct Command {
    show: Option<Show>,
}

impl FromInput for Command {
    type Context = ();

    fn from_input<P: Parse>(input: &mut P, _: &()) -> Result<Self, Error> {
        input.bump_argument().unwrap();
        let mut show = None;

        while !input.is_empty() {
            if input.parse_long_flag("") {
                input.set_ignore_dashes(true);
                continue;
            }

            if input.parse_long_flag("help") || input.parse_short_flag("h") {
                println!("Help");
                return Err(Error::EarlyExit);
            }

            if SetSubcommand(&mut show).apply(input, &())? {
                continue;
            }

            input.expect_empty()?;
        }
        Ok(Command { show })
    }
}
