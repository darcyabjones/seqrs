#![feature(test)]
#![feature(try_from)]
extern crate test;

#[macro_use]
extern crate failure;

pub mod errors;
pub mod alphabets;
pub mod charcase;
pub mod seq;
pub mod convert;
