#![forbid(unsafe_code)]
// #![warn(missing_docs)]

pub use error::Error;
pub use from_input::{FromInput, FromInputValue};
pub use palex::Input;
pub use parse::Parse;

mod error;
mod from_input;
pub mod impls;
mod parse;
pub mod util;
