# parkour

A fast, extensible, command-line arguments parser.

[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue?style=flat-square)](https://docs.rs/parkour)
[![Crates.io](https://img.shields.io/crates/v/parkour?style=flat-square)](https://crates.io/crates/parkour)
[![Downloads](https://img.shields.io/crates/d/parkour?style=flat-square)](https://crates.io/crates/parkour)
[![License-MIT](<https://img.shields.io/badge/license-MIT-blue?style=flat-square>)](./LICENSE-MIT)
[![License-Apache](<https://img.shields.io/badge/license-Apache 2.0-blue?style=flat-square>)](./LICENSE-MIT)
[![Checks](https://flat.badgen.net/github/checks/Aloso/parkour/main)](https://github.com/Aloso/parkour/actions)

## Introduction 📚

The most popular argument parser, `clap`, allows you list all the possible arguments and their constraints, and then gives you a dynamic, stringly-typed object containing all the values. Usually these values are then manually extracted into structs and enums to access the values more conveniently and get the advantages of a static type system ([example](https://github.com/rust-lang/cargo/blob/master/src/bin/cargo/cli.rs)).

Parkour uses a different approach: Instead of parsing the arguments into an intermediate, stringly-typed object, it parses them directly into the types you want, so there's no cumbersome conversion. For types outside the standard library, you need to implement a trait, but in most cases this can be done with a simple derive macro.

This has several advantages:

* It is very flexible: Every aspect of argument parsing can be tailored to your needs.
* It is strongly typed: Many errors can be caught at compile time, so you waste less time debugging.
* It is zero-cost: If you don't need a feature, you don't have to use it. Parkour should also be pretty fast, but don't take my word for it, benchmark it 😉

## Status

Parkour started as an experiment and is very new (about 1 week old at the time of writing). Expect frequent breaking changes. If you like what you see, consider supporting this work by

* Reading the [docs](https://docs.rs/parkour)
* Trying it out
* Giving feedback in [this issue](https://github.com/Aloso/parkour/issues/1)
* Opening issues or sending PRs

Right now, parkour lacks some important features, which I intend to implement:

* Auto-generated help messages
* A DSL to write (sub)commands more ergonomically
* More powerful derive macros
* Error messages with ANSI colors

## Example

```rust
use parkour::prelude::*;

#[derive(FromInputValue)]
enum ColorMode {
    Always,
    Auto,
    Never,
}

struct Command {
    color_mode: ColorMode,
    file: String,
}

impl FromInput for Command {
    type Context = ();

    fn from_input<P: Parse>(input: &mut P, _: &Self::Context)
        -> Result<Self, parkour::Error> {
        // discard the first argument
        input.bump_argument().unwrap();

        let mut file = None;
        let mut color_mode = None;

        while !input.is_empty() {
            if input.parse_long_flag("help") || input.parse_short_flag("h") {
                println!("Usage: run [-h,--help] [--color,-c auto|always|never] FILE");
                return Err(parkour::Error::early_exit());
            }
            if SetOnce(&mut color_mode)
                .apply(input, &Flag::LongShort("color", "c").into())? {
                continue;
            }
            if SetPositional(&mut file).apply(input, &"FILE")? {
                continue;
            }
            input.expect_empty()?;
        }

        Ok(Command {
            color_mode: color_mode.unwrap_or(ColorMode::Auto),
            file: file.ok_or_else(|| parkour::Error::missing_argument("FILE"))?,
        })
    }
}
```

## Code of Conduct 🤝

Please be friendly and respectful to others. This should be a place where everyone can feel safe, therefore I intend to enforce the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.
