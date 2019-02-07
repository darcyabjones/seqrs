/// A generalised codon alphabet.

use crate::{Complement, Translate};
use alphabet::AA;
use alphabet::DNA;
use errors::{SeqError, SeqErrorKind};
use failure::ResultExt;

use std::convert::{TryFrom, TryInto};

/// Codons represented as tuple struct.
/// The tuple struct with public fields is used to make pattern matching
/// easier. I think it's a reasonable choice, given that these should
/// essentially be immutable and the order of codon elements has an
/// unambiguous interpretation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Codon<T>(pub T, pub T, pub T);


/// Codon constructor and accessors.
impl<T> Codon<T> {

    pub fn new(first: T, second: T, third: T) -> Self {
        Codon(first, second, third)
    }

    pub fn first(self) -> T {
        self.0
    }

    pub fn second(self) -> T {
        self.1
    }

    pub fn third(self) -> T {
        self.2
    }
}

impl<T> Codon<T> {

    /// Parse a 3 member array as a Codon.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabet::DNA;
    /// use seqrs::alphabet::DNA::*;
    /// use seqrs::alphabet::Codon;
    ///
    /// let codon = Codon::try_from_iter([A, A, A].iter()).unwrap();
    /// assert_eq!(codon, Codon(A, A, A));
    /// ```
    pub fn try_from_iter<U, I>(bases: I) -> Result<Self, SeqError> where
        U: Into<T>,
        I: IntoIterator<Item = U>
    {
        let mut bases = bases.into_iter().map(|b| b.into());

        let one = Self::item_to_type(bases.next(), 1)?;
        let two = Self::item_to_type(bases.next(), 2)?;
        let three = Self::item_to_type(bases.next(), 3)?;

        Ok(Codon(one, two, three))
    }

    fn item_to_type(item: Option<T>, pos: usize) -> Result<T, SeqError> {
        item.ok_or_else(|| SeqErrorKind::CodonLengthError { n: pos }.into())
    }
}


impl<T, U, I> TryFrom<I> for Codon<T> where
        U: Into<T>,
        I: IntoIterator<Item = U>,
{
    type Error = SeqError;

    fn try_from(bases: I) -> Result<Self, Self::Error> {
        Codon::<T>::try_from_iter(I)
    }
}


impl<T: Default> Default for Codon<T> {
    /// Returns codon with contained types defaults.
    ///
    /// Examples:
    ///
    /// ```
    /// use seqrs::alphabet::Codon;
    /// use seqrs::alphabet::DNA;
    ///
    /// let c = Codon::<DNA>::default();
    /// assert_eq!(c, Codon(DNA::default(), DNA::default(), DNA::default()));
    /// ```
    #[inline]
    fn default() -> Self {
        Self(T::default(), T::default(), T::default())
    }
}

impl Translate<AA> for Codon<DNA> {
    fn translate(&self) -> AA {
        match self {
            Codon(DNA::A, DNA::A, DNA::A) => AA::K,
            Codon(DNA::A, DNA::A, DNA::C) => AA::N,
            Codon(DNA::A, DNA::A, DNA::G) => AA::K,
            Codon(DNA::A, DNA::A, DNA::R) => AA::K,
            Codon(DNA::A, DNA::A, DNA::T) => AA::N,
            Codon(DNA::A, DNA::A, DNA::Y) => AA::N,
            Codon(DNA::A, DNA::C, _     ) => AA::T,
            Codon(DNA::A, DNA::G, DNA::A) => AA::R,
            Codon(DNA::A, DNA::G, DNA::G) => AA::R,
            Codon(DNA::A, DNA::G, DNA::T) => AA::S,
            Codon(DNA::A, DNA::G, DNA::C) => AA::S,
            Codon(DNA::A, DNA::G, DNA::Y) => AA::S,
            Codon(DNA::A, DNA::G, DNA::R) => AA::R,
            Codon(DNA::A, DNA::T, DNA::A) => AA::I,
            Codon(DNA::A, DNA::T, DNA::T) => AA::I,
            Codon(DNA::A, DNA::T, DNA::C) => AA::I,
            Codon(DNA::A, DNA::T, DNA::Y) => AA::I,
            Codon(DNA::A, DNA::T, DNA::W) => AA::I,
            Codon(DNA::A, DNA::T, DNA::H) => AA::I,
            Codon(DNA::A, DNA::T, DNA::G) => AA::M,
            Codon(DNA::C, DNA::A, DNA::A) => AA::Q,
            Codon(DNA::C, DNA::A, DNA::C) => AA::H,
            Codon(DNA::C, DNA::A, DNA::G) => AA::Q,
            Codon(DNA::C, DNA::A, DNA::R) => AA::Q,
            Codon(DNA::C, DNA::A, DNA::T) => AA::H,
            Codon(DNA::C, DNA::A, DNA::Y) => AA::H,
            Codon(DNA::C, DNA::C, _     ) => AA::P,
            Codon(DNA::C, DNA::G, _     ) => AA::R,
            Codon(DNA::C, DNA::T, _     ) => AA::L,
            Codon(DNA::M, DNA::G, DNA::A) => AA::R,
            Codon(DNA::M, DNA::G, DNA::G) => AA::R,
            Codon(DNA::M, DNA::G, DNA::R) => AA::R,
            Codon(DNA::M, DNA::T, DNA::A) => AA::J,
            Codon(DNA::M, DNA::T, DNA::C) => AA::J,
            Codon(DNA::M, DNA::T, DNA::M) => AA::J,
            Codon(DNA::M, DNA::T, DNA::T) => AA::J,
            Codon(DNA::M, DNA::T, DNA::W) => AA::J,
            Codon(DNA::M, DNA::T, DNA::Y) => AA::J,
            Codon(DNA::M, DNA::T, DNA::H) => AA::J,
            Codon(DNA::G, DNA::A, DNA::A) => AA::E,
            Codon(DNA::G, DNA::A, DNA::C) => AA::D,
            Codon(DNA::G, DNA::A, DNA::G) => AA::E,
            Codon(DNA::G, DNA::A, DNA::R) => AA::E,
            Codon(DNA::G, DNA::A, DNA::T) => AA::D,
            Codon(DNA::G, DNA::A, DNA::Y) => AA::D,
            Codon(DNA::G, DNA::C, _     ) => AA::A,
            Codon(DNA::G, DNA::G, _     ) => AA::G,
            Codon(DNA::G, DNA::T, _     ) => AA::V,
            Codon(DNA::R, DNA::A, DNA::C) => AA::B,
            Codon(DNA::R, DNA::A, DNA::T) => AA::B,
            Codon(DNA::R, DNA::A, DNA::Y) => AA::B,
            Codon(DNA::S, DNA::A, DNA::A) => AA::Z,
            Codon(DNA::S, DNA::A, DNA::G) => AA::Z,
            Codon(DNA::S, DNA::A, DNA::R) => AA::Z,
            Codon(DNA::T, DNA::A, DNA::A) => AA::Stop,
            Codon(DNA::T, DNA::A, DNA::C) => AA::Y,
            Codon(DNA::T, DNA::A, DNA::G) => AA::Stop,
            Codon(DNA::T, DNA::A, DNA::R) => AA::Stop,
            Codon(DNA::T, DNA::A, DNA::T) => AA::Y,
            Codon(DNA::T, DNA::A, DNA::Y) => AA::Y,
            Codon(DNA::T, DNA::C, _     ) => AA::S,
            Codon(DNA::T, DNA::G, DNA::A) => AA::Stop,
            Codon(DNA::T, DNA::G, DNA::C) => AA::C,
            Codon(DNA::T, DNA::G, DNA::G) => AA::W,
            Codon(DNA::T, DNA::G, DNA::R) => AA::Stop,
            Codon(DNA::T, DNA::G, DNA::T) => AA::C,
            Codon(DNA::T, DNA::G, DNA::Y) => AA::C,
            Codon(DNA::T, DNA::R, DNA::A) => AA::Stop,
            Codon(DNA::T, DNA::T, DNA::A) => AA::L,
            Codon(DNA::T, DNA::T, DNA::C) => AA::F,
            Codon(DNA::T, DNA::T, DNA::G) => AA::L,
            Codon(DNA::T, DNA::T, DNA::R) => AA::L,
            Codon(DNA::T, DNA::T, DNA::T) => AA::F,
            Codon(DNA::T, DNA::T, DNA::Y) => AA::F,
            Codon(DNA::W, DNA::T, DNA::A) => AA::J,
            Codon(DNA::Y, DNA::T, DNA::A) => AA::L,
            Codon(DNA::Y, DNA::T, DNA::G) => AA::L,
            Codon(DNA::Y, DNA::T, DNA::R) => AA::L,
            Codon(DNA::H, DNA::T, DNA::A) => AA::J,
            _                             => AA::X,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alphabet::DNA;
    use alphabet::AA;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from() {
        let codon = Codon::try_from_iter([DNA::A, DNA::A, DNA::A].iter()).unwrap();
        assert_eq!(codon, Codon(DNA::A, DNA::A, DNA::A));
    }

    #[test]
    fn test_eq() {
        assert_eq!(Codon::new(DNA::A, DNA::T, DNA::G), Codon(DNA::A, DNA::T, DNA::G));
    }

    #[test]
    fn test_access() {
        let met = Codon(DNA::A, DNA::T, DNA::G);
        assert_eq!(met.0, DNA::A);
        assert_eq!(met.first(), DNA::A);
    }

    #[test]
    fn test_translate() {
        let met = Codon(DNA::A, DNA::T, DNA::G);
        assert_eq!(met.translate(), AA::M);
    }

    #[test]
    fn test_translate_arr() {
        let arr = vec![Codon(DNA::A, DNA::T, DNA::G), Codon(DNA::C, DNA::T, DNA::C)];
        let mapped: Vec<AA> = arr.iter().map(|c| c.translate()).collect();
        assert_eq!(mapped, vec![AA::M, AA::L]);
    }
}

