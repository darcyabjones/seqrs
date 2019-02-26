#![feature(exact_size_is_empty)]
#![feature(try_trait)]
#![feature(test)]
#![feature(try_from)]

extern crate test;

pub mod utils;

#[macro_use]
pub mod macros;

pub mod alphabet;
pub mod errors;

pub mod codon;
pub mod complement;
pub mod gapped;
pub mod matcher;
pub mod stopped;
pub mod translate;

pub mod packed;
