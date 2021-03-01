use std::error::Error as _;

use parkour::prelude::*;

#[derive(FromInput, Debug, PartialEq)]
#[parkour(main)]
struct Command {
    #[arg(long = "color", long = "colour", short)]
    mode: ColorMode,
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
    ok!("$ -c always", Command { mode: ColorMode::Always });
    ok!("$ -c=always", Command { mode: ColorMode::Always });
    ok!("$ -cALwAyS", Command { mode: ColorMode::Always });
    ok!("$ --color always", Command { mode: ColorMode::Always });
    ok!("$ --color=always", Command { mode: ColorMode::Always });
    ok!("$ --colour=always", Command { mode: ColorMode::Always });
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
}
