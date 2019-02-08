#![feature(exact_size_is_empty)]
#![feature(try_trait)]
#![feature(test)]
#![feature(try_from)]
extern crate test;

extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate proptest;

extern crate lazy_static;

pub mod utils;

#[macro_use]
pub mod macros;

pub mod errors;
pub mod alphabet;

// Reexport traits to keep things succinct
mod matcher;
pub use matcher::{Match, RedundantAlphabet};

mod translate;
pub use translate::Translate;

mod complement;
pub use complement::{Complement, ReverseComplement};
