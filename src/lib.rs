#![feature(exact_size_is_empty)]
#![feature(try_trait)]
#![feature(test)]
#![feature(try_from)]

extern crate test;


pub mod utils;

#[macro_use]
pub mod macros;

pub mod errors;
pub mod alphabet;

pub mod matcher;
pub mod complement;
pub mod translate;
pub mod codon;
pub mod gapped;
pub mod stopped;
