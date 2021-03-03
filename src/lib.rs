//! A fast, extensible, command-line arguments parser.
//!
//! This library is very new, so expect regular breaking changes. If you find a
//! bug or lacking documentation, don't hesitate to open an
//! [issue](https://github.com/Aloso/parkour/issues) or a pull request.
//!
//! This crate started as an experiment, so I'm not sure yet if I want to
//! maintain it long-term. See [here](https://github.com/Aloso/parkour/issues/1)
//! for more.
//!
//! ## Getting started
//!
//! Parkour requires const generics. The first rust version that supports them
//! is Rust 1.51 (`rustc 1.51.0-beta.2`). You can install it with `rustup
//! default beta`.
//!
//! It's recommended to import the [prelude](./prelude/index.html):
//!
//! ```
//! use parkour::prelude::*;
//! ```
//!
//! First, create a struct containing all the data you want to parse. For
//! example:
//!
//! ```
//! struct Command {
//!     color: Option<bool>,
//!     show: Option<Show>,
//! }
//!
//! struct Show {
//!     pos1: String,
//!     out: ColorSpace,
//!     size: u8,
//! }
//!
//! enum ColorSpace {
//!     Rgb,
//!     Cmy,
//!     Cmyk,
//!     Hsv,
//!     Hsl,
//!     CieLab,
//! }
//! ```
//!
//! `bool`, `u8` and `String` can all be parsed by default. To parse
//! `ColorSpace`, we have to implement the [`FromInputValue`] trait. This
//! easiest by using the derive macro:
//!
//! ```
//! # use parkour::prelude::*;
//! #[derive(FromInputValue)]
//! enum ColorSpace {
//!     Rgb,
//!     Cmy,
//!     Cmyk,
//!     Hsv,
//!     Hsl,
//!     CieLab,
//! }
//! ```
//!
//! This parses the names of the enum variants case-insensitively. When an
//! invalid value is provided, the error message will say something like:
//!
//! ```text
//! unexpected value, got `foo`, expected rgb, cmy, cmyk, hsv, hsl or cielab
//! ```
//!
//! Now let's implement `Show` as a subcommand. Unfortunately, there's no
//! convenient derive macro (yet):
//!
//! ```
//! # use parkour::prelude::*;
//! # #[derive(FromInputValue)]
//! # enum ColorSpace { Rgb, Cmy, Cmyk, Hsv, Hsl, CieLab }
//! #
//! struct Show {
//!     pos1: String,
//!     color_space: ColorSpace,
//!     size: u8,
//! }
//!
//! impl FromInput<'static> for Show {
//!     type Context = ();
//!
//!     fn from_input(input: &mut ArgsInput, _: &()) -> parkour::Result<Self> {
//!         if input.parse_command("show") {
//!             let mut pos1 = None;
//!             let mut color_space = None;
//!             let mut size = None;
//!
//!             while !input.is_empty() {
//!                 if SetOnce(&mut color_space)
//!                     .apply(input, &Flag::LongShort("color-space", "c").into())? {
//!                     continue;
//!                 }
//!
//!                 if SetOnce(&mut size)
//!                     .apply(input, &Flag::LongShort("size", "s").into())? {
//!                     continue;
//!                 }
//!
//!                 if pos1.is_none()
//!                     && SetPositional(&mut pos1).apply(input, &"pos1".into())? {
//!                     continue;
//!                 }
//!
//!                 input.expect_empty()?;
//!             }
//!
//!             Ok(Show {
//!                 pos1: pos1.ok_or_else(|| parkour::Error::missing_argument("pos1"))?,
//!                 color_space: color_space
//!                     .ok_or_else(|| parkour::Error::missing_argument("--color-space"))?,
//!                 size: size.unwrap_or(4),
//!             })
//!         } else {
//!             Err(parkour::Error::no_value())
//!         }
//!     }
//! }
//! ```
//!
//! To parse a subcommand, we implement the [`FromInput`] trait. We first check
//! if the next argument is the word `show`. If that's the case, we iterate over
//! the remaining input, until it is empty.
//!
//! In the subcommand, we expect two named arguments (`--color-space` and
//! `--size`) and a positional argument (`pos`). Therefore, in each iteration,
//! we first check if we can parse the named arguments, and then the positional
//! argument. If none of them succeeds and there is still input left, then
//! `input.expect_empty()?` throws an error.
//!
//! Producing the `Show` struct is rather straightforward (`pos` and
//! `--color-space` are required, `--size` defaults to `4`). However, parsing
//! the values involves some type system magic. `SetOnce` and `SetPositional`
//! are [actions], they check if the referenced types can be parsed, and if so,
//! assign the parsed value to the variable automatically. They also ensure that
//! each argument is parsed at most once.
//!
//! Whenever something is parsed, a _context_ is provided that can contain
//! information about _how_ the value should be parsed. In the above example,
//! `Flag::LongShort("color-space", "c").into()` is a context that instructs the
//! parser to parse the color space after the `--color-space` or the `-c` flag.
//!
//! The main command can be implemented similarly:
//!
//! ```
//! # use parkour::prelude::*;
//! # enum ColorSpace { Rgb, Cmy, Cmyk, Hsv, Hsl, CieLab }
//! # struct Show {
//! #     pos1: String,
//! #     color_space: ColorSpace,
//! #     size: u8,
//! # }
//! # impl FromInput<'static> for Show {
//! #     type Context = ();
//! #     fn from_input(input: &mut ArgsInput, _: &()) -> parkour::Result<Self> {
//! #         todo!()
//! #     }
//! # }
//! #
//! struct Command {
//!     color: Option<bool>,
//!     show: Option<Show>,
//! }
//!
//! impl FromInput<'static> for Command {
//!     type Context = ();
//!
//!     fn from_input(input: &mut ArgsInput, _: &()) -> parkour::Result<Self> {
//!         // discard the first argument, which is the path to the executable
//!         input.bump_argument().unwrap();
//!
//!         let mut show = None;
//!         let mut color = None;
//!
//!         while !input.is_empty() {
//!             if SetOnce(&mut color).apply(input, &Flag::LongShort("color", "c").into())? {
//!                 continue;
//!             }
//!
//!             if SetSubcommand(&mut show).apply(input, &())? {
//!                 continue;
//!             }
//!
//!             input.expect_empty()?;
//!         }
//!         Ok(Command { show, color })
//!     }
//! }
//! ```
//!
//! This is pretty self-explanatory. Now let's proceed to the main function:
//!
//! ```
//! # use parkour::prelude::*;
//! # struct Command {
//! #     color: Option<bool>,
//! #     show: Option<()>,
//! # }
//! # impl FromInput<'static> for Command {
//! #     type Context = ();
//! #     fn from_input(input: &mut ArgsInput, _: &()) -> parkour::Result<Self> {
//! #         Ok(Command { color: None, show: None })
//! #     }
//! # }
//! #
//! use std::error::Error;
//!
//! fn main() {
//!     match Command::from_input(&mut parkour::parser(), &()) {
//!         Ok(command) => {
//!             println!("parsed successfully");
//!         }
//!         Err(e) if e.is_early_exit() => {}
//!         Err(e) => {
//!             eprint!("{}", e);
//!             let mut source = e.source();
//!             while let Some(s) = source {
//!                 eprint!(": {}", s);
//!                 source = s.source();
//!             }
//!             eprintln!();
//!         }
//!     }
//! }
//! ```
//!
//! The [`parser`] function creates a new parser instance, which
//! implements [`Parse`]. This is used to parse the `Command`. If it fails, we
//! print the error with its sources. I will implement a more convenient method
//! for this, I just haven't gotten around to it yet. I also plan to implement
//! ANSI color support.
//!
//! What's with the `e.is_early_exit()`, you might wonder? This error is
//! returned when parsing was aborted and can be ignored. This error can be used
//! e.g. when the `--help` flag is encountered:
//!
//! ```no_run
//! # use parkour::prelude::*;
//! # struct Command {
//! #     color: Option<bool>,
//! #     show: Option<()>,
//! # }
//! impl FromInput<'static> for Command {
//!     type Context = ();
//!
//!     fn from_input(input: &mut ArgsInput, _: &()) -> Result<Self, parkour::Error> {
//! #       let color = None;
//! #       let show = None;
//!         // <snip>
//!         while !input.is_empty() {
//!             if input.parse_long_flag("help") || input.parse_short_flag("h") {
//!                 println!("Usage:\n\
//!                     my-program [-h,--help]\n\
//!                     my-program show POS1 -c,--color-space VALUE [-s,--size N]");
//!
//!                 return Err(parkour::Error::early_exit());
//!             }
//!
//!             // <snip>
//!         }
//!         Ok(Command { show, color })
//!     }
//! }
//! ```
//!
//! There is one special case that isn't handled yet: The argument `--` usually
//! causes the remaining tokens to be treated as positional arguments, even if
//! they start with a dash. This is easily implemented:
//!
//! ```no_run
//! # use parkour::prelude::*;
//! # struct Command {
//! #     color: Option<bool>,
//! #     show: Option<()>,
//! # }
//! impl FromInput<'static> for Command {
//!     type Context = ();
//!
//!     fn from_input(input: &mut ArgsInput, _: &()) -> Result<Self, parkour::Error> {
//! #       let color = None;
//! #       let show = None;
//!         // <snip>
//!         while !input.is_empty() {
//!             if input.parse_long_flag("") {
//!                 input.set_ignore_dashes(true);
//!                 continue;
//!             }
//!
//!             // <snip>
//!         }
//!         Ok(Command { show, color })
//!     }
//! }
//! ```
//!
//! Unfortunately, this must be repeated in every subcommand.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use error::{Error, ErrorInner};
pub use from_input::{FromInput, FromInputValue};
pub use parse::Parse;

pub use palex::ArgsInput;

#[cfg(feature = "derive")]
pub use parkour_derive::{FromInput, FromInputValue};

pub mod actions;
mod error;
mod from_input;
pub mod help;
pub mod impls;
mod parse;
pub mod util;

/// A parkour result.
pub type Result<T> = std::result::Result<T, Error>;

/// Create a new parser, which can be used to parse the
/// command-line arguments of the program.
pub fn parser() -> ArgsInput {
    ArgsInput::from_args()
}

/// A prelude to make it easier to import all the needed types and traits. Use
/// it like this:
///
/// ```
/// use parkour::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actions::{
        Action, Append, Dec, Inc, Reset, Set, SetOnce, SetPositional, SetSubcommand,
        Unset,
    };
    pub use crate::impls::{ListCtx, NumberCtx, StringCtx};
    pub use crate::util::{ArgCtx, Flag, PosCtx};
    pub use crate::{ArgsInput, FromInput, FromInputValue, Parse};
}
