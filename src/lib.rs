//! A fast, extensible, command-line arguments parser.

#![forbid(unsafe_code)]
// #![warn(missing_docs)]

pub use error::{Error, ErrorInner};
pub use from_input::{FromInput, FromInputValue};
pub use parse::Parse;

pub use palex::{Input, StringInput};

pub mod actions;
mod error;
mod from_input;
pub mod impls;
mod parse;
pub mod util;

/// Create a new instance of the [Parser] trait, which can be used to parse the
/// command-line arguments of the program.
pub fn parser() -> impl Parse {
    StringInput::new(std::env::args())
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
    pub use crate::util::{Flag, OptionCtx, PosCtx};
    pub use crate::{FromInput, FromInputValue, Parse};
}
