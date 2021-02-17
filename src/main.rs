use std::time::Instant;

use palr::util::{Flag, MapNoValue};
use palr::StringInput;
use palr::{Error, FromInput, FromInputValue, Parse};
use Flag::*;

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
    Command::from_input(&mut input, ()).map_no_value(|| Error::MissingOption {
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

    fn from_input_value(value: &str, _: ()) -> Result<Self, Error> {
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

    fn from_input<P: Parse>(input: &mut P, _: ()) -> Result<Self, Error> {
        if input.parse_command("show") || input.parse_command("s") {
            let mut pos1 = None;
            let mut out = None;
            let mut size = None;
            let mut color = None;

            while !input.is_empty() {
                if input.parse_long_flag("help") || input.parse_short_flag("h") {
                    println!("Help for `show` subcommand");
                    return Err(Error::EarlyExit);
                }

                if let Some(new_out) =
                    input.try_parse_flag_and_value(&[Long("out"), Short("o")], ())?
                {
                    if out.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "option `--out`".to_string(),
                            max: 1,
                        });
                    }
                    out = Some(new_out);
                    continue;
                }

                if let Some(new_size) = input.try_parse_flag_and_value(
                    &[Long("size"), Short("s")],
                    Default::default(),
                )? {
                    if size.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "option `--size`".to_string(),
                            max: 1,
                        });
                    }
                    size = Some(new_size);
                    continue;
                }

                if let Some(new_color) =
                    input.try_parse_flag_and_value(&[Long("color"), Short("c")], ())?
                {
                    if color.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "option `--color`".to_string(),
                            max: 1,
                        });
                    }
                    color = Some(new_color);
                    continue;
                }

                if let Some(value) = input.value_allows_leading_dashes() {
                    if pos1.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "positional argument".to_string(),
                            max: 1,
                        });
                    }
                    pos1 = Some(value.eat().to_string());
                    continue;
                }

                if !input.is_empty() {
                    return Err(Error::Unexpected {
                        word: input.bump_argument().unwrap().to_string(),
                    });
                }
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

    fn from_input<P: Parse>(input: &mut P, _: ()) -> Result<Self, Error> {
        input.bump_argument().unwrap();
        let mut show = None;

        while !input.is_empty() {
            if input.parse_long_flag("help") || input.parse_short_flag("h") {
                println!("Help");
                return Err(Error::EarlyExit);
            }

            if let Some(s) = input.try_parse(())? {
                if show.is_some() {
                    return Err(Error::TooManyOptionOccurrences {
                        option: "subcommand `show`".to_string(),
                        max: 1,
                    });
                }
                show = Some(s);
                continue;
            }

            if !input.is_empty() {
                return Err(Error::Unexpected {
                    word: input.bump_argument().unwrap().to_string(),
                });
            }
        }
        Ok(Command { show })
    }
}
