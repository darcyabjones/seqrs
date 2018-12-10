//! Wrapper to contain biological alphabets.

use std::convert::TryFrom;
use std::convert::TryInto;
use std::convert::Into;
use errors::SeqError;
use alphabets::DNA;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seq<T> {
    con: Vec<T>
}


impl<'a, T> Seq<T> {

    pub fn new() -> Self {
        Seq { con: Vec::new() }
    }
}


impl<'a, T: TryFrom<char>> TryFrom<&'a str> for Seq<T> {
    type Error = T::Error;

    /// Converts string to Seq object.
    ///
    /// # Examples:
    ///
    /// ```
    /// #![feature(try_from)]
    /// use seqrs::alphabets::DNA;
    /// use seqrs::seq::Seq;
    ///
    /// use std::convert::TryFrom;
    ///
    /// let seq: Seq<DNA> = Seq::try_from("ATG").unwrap();
    /// ```
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let seq: Result<Vec<T>, Self::Error> = string
            .chars()
            .map(|b| T::try_from(b))
            .collect();

        seq.map(|s| Self { con: s})
    }
}

/*
impl<T: Complement> ReverseComplement for Seq<T> {
}

impl IntoCodons<T> for Seq<T> {}


impl<T: Translate> Translate for Seq<T> {}

*/
