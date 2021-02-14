#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! A fast, small, dependency-free crate for lexing command-line arguments. You
//! can use this crate if you want to build your own argument parsing library.
//!
//! This crate is almost zero-cost, since it parses arguments lazily and avoids
//! most heap allocations. There's no dynamic dispatch.
//!
//! Check the `examples` folder for examples.

pub use input::Input;
pub use string_input::StringInput;
pub use token_kind::TokenKind;

mod input;
mod string_input;
mod token_kind;

#[cfg(test)]
mod tests;

pub mod part;
