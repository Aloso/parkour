use std::error::Error as _;

use parkour::prelude::*;

#[derive(FromInput, Debug, PartialEq)]
#[parkour(main)]
struct Command {
    #[arg(long, short)]
    dry_run: bool,
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
    ok!("$", Command { dry_run: false });
    ok!("$ --dry-run", Command { dry_run: true });
    ok!("$ -d", Command { dry_run: true });
}

#[test]
fn failures() {
    err!("$ -dYES", "unexpected value `YES`");
    err!("$ -d=yes", "unexpected value `yes`");
    err!("$ --dry-run=", "unexpected value ``");
    err!("$ --dry-run yes", "unexpected argument `yes`");
    err!("$ dry-run", "unexpected argument `dry-run`");
    err!(
        "$ --dry-run -d",
        "--dry-run was used too often, it can be used at most 1 times"
    );
}
