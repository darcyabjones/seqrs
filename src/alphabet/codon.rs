/// A generalised codon alphabet.

use ::Translate;
use alphabet::AA;
use alphabet::DNA;
use errors::{SeqError, SeqErrorKind};

use std::convert::{TryFrom, TryInto};
use std::str::FromStr;


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


pub trait Codons<I> {
    type Item;
    type Iter;

    fn codons(self) -> CodonsIterator<I>;
}

impl<I, T, U> Codons<I> for U
    where U: IntoIterator<Item=T, IntoIter=I>,
          I: Iterator<Item=T>,
{
    type Item = T;
    type Iter = I;

    fn codons(self) -> CodonsIterator<I> {
        CodonsIterator{ iter: self.into_iter() }
    }
}

#[derive(Debug, Clone)]
pub struct CodonsIterator<I> {
    iter: I,
}

impl<I, T> Iterator for CodonsIterator<I>
    where I: Iterator<Item=T>,
{
    type Item = Codon<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let one = self.iter.next()?;
        let two = self.iter.next()?;
        let three = self.iter.next()?;

        Some(Codon(one, two, three))
    }

    /// Calculate the length of codon iterable.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabet::DNA;
    /// use seqrs::alphabet::DNA::*;
    /// use seqrs::alphabet::Codon;
    /// use seqrs::alphabet::Codons;
    ///
    /// let seq = vec![A, A].codons();
    /// assert_eq!((0, Some(0)), seq.size_hint());
    ///
    /// let seq = vec![A, A, A].codons();
    /// assert_eq!((1, Some(1)), seq.size_hint());
    ///
    /// let seq = vec![A, A, A, A].codons();
    /// assert_eq!((1, Some(1)), seq.size_hint());
    ///
    /// let seq = vec![A, A, A, A, A].codons();
    /// assert_eq!((1, Some(1)), seq.size_hint());
    ///
    /// let seq = vec![A, A, A, A, A, A].codons();
    /// assert_eq!((2, Some(2)), seq.size_hint());
    /// ```
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (length, _) = self.iter.size_hint();
        let length = length / 3;
        (length, Some(length))
    }
}


impl<I, T> DoubleEndedIterator for CodonsIterator<I>
    where I: DoubleEndedIterator<Item=T> + ExactSizeIterator<Item=T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let n = self.iter.len() % 3;
        for _ in 0..n {
            self.iter.next_back()?;
        }

        let three = self.iter.next_back()?;
        let two = self.iter.next_back()?;
        let one = self.iter.next_back()?;

        Some(Codon(one, two, three))
    }
}


impl<I, T> ExactSizeIterator for CodonsIterator<I>
    where I: ExactSizeIterator<Item=T>,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len() / 3
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.iter.len() < 3
    }
}


impl<T> FromStr for Codon<T> 
    where T: TryFrom<char, Error=SeqError>,
{
    type Err = SeqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().map(|c| c.try_into());
        let err = || SeqError::from(SeqErrorKind::CodonFromStrTooShort);
        let one = chars.next().ok_or_else(err)??;
        let two = chars.next().ok_or_else(err)??;
        let three = chars.next().ok_or_else(err)??;

        Ok(Codon(one, two, three))
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use alphabet::DNA;
    use alphabet::DNA::*;
    use alphabet::AA;

    /*
    #[test]
    fn test_codon_collect() {
        let c: Result<Codon<DNA>, _>= vec![A, T, C].iter().collect();
        assert_eq!(c.unwrap(), Codon(A, T, G));
    }
    */

    #[test]
    fn test_codon_from_str() {
        let c = "ATG".parse::<Codon<DNA>>().unwrap();
        assert_eq!(c, Codon(A, T, G));
    }

    #[test]
    fn test_codon_iter() {
        let mut arr = vec![A, T, G, C].codons();
        assert_eq!(Some(Codon(A, T, G)), arr.next());
        assert_eq!(None, arr.next());
    }

    #[test]
    fn test_codon_rev_iter() {
        let mut arr = vec![A, T, G, C].codons();
        assert_eq!(Some(Codon(A, T, G)), arr.next_back());
        assert_eq!(None, arr.next_back());

        let mut arr = vec![A, T, G, C, A, A, G, A].codons();
        assert_eq!(Some(Codon(C, A, A)), arr.next_back());
        assert_eq!(Some(Codon(A, T, G)), arr.next_back());
        assert_eq!(None, arr.next_back());
    }

    #[test]
    fn test_codon_len() {
        let mut arr = vec![A, T, G, C].codons();
        assert_eq!(1, arr.len());
        assert_eq!(false, arr.is_empty());
        let _ = arr.next();
        assert_eq!(true, arr.is_empty());
    }

    #[test]
    fn test_eq() {
        assert_eq!(Codon::new(A, T, G), Codon(A, T, G));
    }

    #[test]
    fn test_access() {
        let met = Codon(A, T, G);
        assert_eq!(met.0, A);
        assert_eq!(met.first(), A);
    }

    #[test]
    fn test_translate() {
        let met = Codon(A, T, G);
        assert_eq!(met.translate(), AA::M);
    }

    #[test]
    fn test_translate_arr() {
        let arr = vec![Codon(A, T, G), Codon(C, T, C)];
        let mapped: Vec<AA> = arr.iter().map(|c| c.translate()).collect();
        assert_eq!(mapped, vec![AA::M, AA::L]);
    }
}

