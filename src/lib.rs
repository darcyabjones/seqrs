#![feature(test)]
#![feature(try_from)]
extern crate test;

#[macro_use]
extern crate failure;

pub mod errors;
pub mod dna;
pub mod dna_iupac;
pub mod prot;
pub mod complement;
