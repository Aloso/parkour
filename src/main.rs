use std::time::Instant;

use palex::{Input, StringInput};
use palr::{Error, Parse};

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
            Error::NoValue | Error::EarlyExit => {}
            e => eprintln!("{}", anyhow::Error::new(e)),
        },
    }
}

fn main_() -> Result<Command, Error> {
    let mut input = StringInput::new(std::env::args());
    Command::parse_value_of_option(&mut input, "no arguments provided")
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

impl Parse for Output {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
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
            word => return Err(Error::Unexpected { word: word.to_string() }),
        })
    }
}

/// `-h/--help` flag
#[derive(Debug)]
struct Help;

impl Parse for Help {
    fn parse_from_value(value: &str) -> Result<Self, Error> {
        if bool::parse_from_value(value)? {
            Ok(Help)
        } else {
            Err(Error::NoValue)
        }
    }

    fn parse<I: Input>(input: &mut I) -> Result<Self, Error> {
        if input.eat_one_dash("h").is_some() || input.eat_two_dashes("help").is_some() {
            Ok(Help)
        } else {
            Err(Error::NoValue)
        }
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

impl Parse for Show {
    fn parse_from_value(_: &str) -> Result<Self, Error> {
        panic!("`Show` doesn't support parsing from a string")
    }

    fn parse<I: Input>(input: &mut I) -> Result<Self, Error> {
        if input.eat_no_dash("s").is_some() || input.eat_no_dash("show").is_some() {
            let mut pos1 = None;
            let mut out = None;
            let mut size = None;
            let mut color = None;

            while !input.is_empty() {
                if Help::try_parse(input)?.is_some() {
                    println!("Help for `show` subcommand");
                    return Err(Error::EarlyExit);
                }

                if input.eat_one_dash("o").is_some()
                    || input.eat_two_dashes("out").is_some()
                {
                    if out.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "option `--out`".to_string(),
                            max: 1,
                        });
                    }
                    out = Some(Output::parse_value_of_option(input, "option `--out`")?);
                    continue;
                }

                if input.eat_one_dash("s").is_some()
                    || input.eat_two_dashes("size").is_some()
                {
                    if size.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "option `--size`".to_string(),
                            max: 1,
                        });
                    }
                    size = Some(u8::parse_value_of_option(input, "option `--size`")?);
                    continue;
                }

                if input.eat_one_dash("c").is_some()
                    || input.eat_two_dashes("color").is_some()
                {
                    if color.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "option `--color`".to_string(),
                            max: 1,
                        });
                    }
                    color = Some(bool::parse_value_of_option(input, "option `--color`")?);
                    continue;
                }

                if input.can_parse_dash_option() {
                    return Err(Error::Unexpected {
                        word: input.bump_argument().unwrap().to_string(),
                    });
                }

                if let Some(value) = input.value_allows_leading_dashes() {
                    if pos1.is_some() {
                        return Err(Error::TooManyOptionOccurrences {
                            option: "positional argument".to_string(),
                            max: 1,
                        });
                    }
                    pos1 = Some(value.eat().to_string());
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

impl Parse for Command {
    fn parse_from_value(_: &str) -> Result<Self, Error> {
        panic!("`Command` doesn't support parsing from a string")
    }

    fn parse<I: Input>(input: &mut I) -> Result<Self, Error> {
        input.bump_argument().unwrap();
        let mut show = None;

        while !input.is_empty() {
            if Help::try_parse(input)?.is_some() {
                println!("Help");
                return Err(Error::EarlyExit);
            }

            if let Some(s) = Show::try_parse(input)? {
                if show.is_some() {
                    return Err(Error::TooManyOptionOccurrences {
                        option: "subcommand `show`".to_string(),
                        max: 1,
                    });
                }
                show = Some(s);
                continue;
            }

            if let Some(value) = input.value_allows_leading_dashes() {
                return Err(Error::Unexpected { word: value.eat().to_string() });
            }
        }
        Ok(Command { show })
    }
}
