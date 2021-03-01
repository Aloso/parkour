use std::error::Error as _;

use parkour::prelude::*;

#[derive(FromInput, Debug, PartialEq)]
#[parkour(main)]
struct Command {
    #[arg(long = "color", long = "colour", short)] // --color,-c <always|auto|never>
    #[arg(long = "alias", short)] // --alias,-a <always|auto|never>
    mode: Option<ColorMode>,
}

#[derive(FromInputValue, Debug, PartialEq)]
enum ColorMode {
    Always,
    Auto,
    Never,
}

macro_rules! ok {
    ($s:literal, $v:expr) => {
        assert_parse!(Command, $s, $v)
    };
}
macro_rules! err {
    ($s:literal, $e:literal) => {
        assert_parse!(Command, $s, $e)
    };
}

#[test]
fn successes() {
    ok!("$", Command { mode: None });
    ok!("$ -c always", Command { mode: Some(ColorMode::Always) });
    ok!("$ -c=always", Command { mode: Some(ColorMode::Always) });
    ok!("$ -cALwAyS", Command { mode: Some(ColorMode::Always) });
    ok!("$ -a always", Command { mode: Some(ColorMode::Always) });
    ok!("$ --color always", Command { mode: Some(ColorMode::Always) });
    ok!("$ --color=always", Command { mode: Some(ColorMode::Always) });
    ok!("$ --colour=always", Command { mode: Some(ColorMode::Always) });
    ok!("$ --alias always", Command { mode: Some(ColorMode::Always) });
}

#[test]
fn failures() {
    err!("$ --color", "missing value: in `--color`: in `--color`");
    err!(
        "$ --color=",
        "unexpected value ``, expected `always`, `auto` or `never`: in `--color`"
    );
    err!(
        "$ --color a",
        "unexpected value `a`, expected `always`, `auto` or `never`: in `--color`"
    );
    err!(
        "$ -ca",
        "unexpected value `a`, expected `always`, `auto` or `never`: in `--color`"
    );
    err!("$ -bca", "unexpected argument `bca`");
    err!("$ --colorALWAYS", "unexpected argument `colorALWAYS`");
    err!("$ -cALWAYS d", "unexpected argument `d`");
    err!(
        "$ -cALWAYS=d",
        "unexpected value `ALWAYS=d`, expected `always`, `auto` or `never`: in `--color`"
    );
    err!(
        "$ -cALWAYS -aNEVER",
        "--alias was used too often, it can be used at most 1 times"
    );
}
