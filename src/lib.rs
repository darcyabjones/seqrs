#![feature(test)]
#![feature(try_from)]
extern crate test;

#[macro_use]
extern crate failure;

pub mod errors;
pub mod alphabets;
mod traits;

pub use traits::{Match, RedundantAlphabet, Complement};
