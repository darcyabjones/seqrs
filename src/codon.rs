/// A generalised codon alphabet.
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use crate::alphabet::Alphabet;
use crate::errors::{SeqError, SeqErrorKind};
use crate::translate::CodonTagTable;
use crate::translate::TranslationTable;

/// Codons represented as tuple struct.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Codon<T>(pub T, pub T, pub T);

impl<T> Codon<T> {
    /// Constructor function.
    pub fn new(first: T, second: T, third: T) -> Self {
        Codon(first, second, third)
    }

    /// Returns the first codon base.
    pub fn first<'a>(&'a self) -> &'a T {
        &self.0
    }

    /// Returns the second codon base.
    pub fn second<'a>(&'a self) -> &'a T {
        &self.1
    }

    /// Returns the third codon base.
    pub fn third<'a>(&'a self) -> &'a T {
        &self.2
    }

    /// Translates a [`Codon`] into an amino acid, using the mapping defined in
    /// some type implementing [`TranslationTable`].
    ///
    /// Note that this is equivalent to calling the [`get`] method on the
    /// object implementing [`TranslationTable`].
    ///
    /// ['Codon']: struct.Codon.html
    /// [`TranslationTable`]: ../translate/trait.TranslationTable.html
    /// [`get`]: ../translate/trait.TranslationTable.html#method.get
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    /// use seqrs::alphabet::DNA::*;
    /// use seqrs::alphabet::AA;
    /// use seqrs::translate::TranslationTable;
    /// use seqrs::translate::NCBITransTable;
    /// use seqrs::stopped::Stopped::{Res, StopOr, Stop};
    ///
    /// let codon = Codon(A, T, G);
    /// assert_eq!(codon.translate(&NCBITransTable::Standard), Res(AA::M));
    ///
    /// // This is equivalent to
    /// assert_eq!(NCBITransTable::Standard.get(&Codon(A, T, G)), Res(AA::M));
    ///
    /// let codon = Codon(T, A, G);
    /// assert_eq!(codon.translate(&NCBITransTable::Standard), Stop);
    ///
    /// // Stop or can represent cases of redundant ambiguity.
    /// let codon = Codon(N, N, N);
    /// assert_eq!(codon.translate(&NCBITransTable::Standard), StopOr(AA::X));
    ///
    /// // Stop or also represents edge cases of some translation tables.
    /// let codon = Codon(T, G, A);
    /// assert_eq!(codon.translate(&NCBITransTable::Karyorelict), StopOr(AA::W));
    /// ```
    #[inline]
    pub fn translate<'a, U, V>(&'a self, table: &U) -> V
    where
        U: TranslationTable<&'a Codon<T>, V>,
    {
        table.get(&self)
    }

    /// Gets the translation codon tag using the mapping defined in some type
    /// implementing [`CodonTagTable`]. The tags are related to the translation
    /// tables, telling you whether a codon could encode a Start, or Stop, or
    /// some other translation state.
    ///
    /// Note that this is equivalent to calling the [`get_tag`] method on the
    /// object implementing [`CodonTagTable`]
    ///
    /// [`CodonTagTable`]: ../translate/trait.CodonTagTable.html
    /// [`get_tag`]: ../translate/trait.CodonTagTable.html#method.get_tag
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    /// use seqrs::alphabet::DNA::*;
    /// use seqrs::alphabet::CodonTag;
    /// use seqrs::translate::CodonTagTable;
    /// use seqrs::translate::NCBITransTable;
    ///
    /// let codon = Codon(A, T, G);
    /// assert_eq!(codon.tag(&NCBITransTable::Standard), CodonTag::Start);
    ///
    /// // Equivalent to this.
    /// assert_eq!(NCBITransTable::Standard.get_tag(&Codon(A, T, G)), CodonTag::Start);
    ///
    /// let codon = Codon(T, A, G);
    /// assert_eq!(codon.tag(&NCBITransTable::Standard), CodonTag::Stop);
    ///
    /// let codon = Codon(N, N, N);
    /// assert_eq!(codon.tag(&NCBITransTable::Standard), CodonTag::Any);
    /// ```
    #[inline]
    pub fn tag<'a, U, V>(&'a self, table: &U) -> V
    where
        U: CodonTagTable<&'a Codon<T>, V>,
    {
        table.get_tag(&self)
    }

    /// Converts from [`Codon<T>`] to [`Codon<&T>`].
    ///
    /// The [`map`] methods (and relatives) takes the `self` argument by
    /// value, consuming the original. Use [`as_ref`] to first take a reference
    /// to the value inside the original [`Codon`].
    ///
    /// [`Codon`]: struct.Codon.html
    /// [`Codon<T>`]: struct.Codon.html
    /// [`Codon<&T>`]: struct.Codon.html
    /// [`map`]: #method.map
    /// [`as_ref`]: #method.as_ref
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    ///
    /// // Reimplement an alphabet without the Copy trait.
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum NC { A, T, G, C }
    ///
    /// let codon = Codon(NC::A, NC::T, NC::G);
    /// let mutated = codon.as_ref().flat_map(|one, _, three| {
    ///     Codon(one, &NC::C, three)
    /// });
    ///
    /// assert_eq!(mutated, Codon(&NC::A, &NC::C, &NC::G));
    /// println!("Can still print: {:?} into {:?}", codon, mutated);
    /// ```
    pub fn as_ref(&self) -> Codon<&T> {
        let Codon(ref one, ref two, ref three) = self;
        Codon(one, two, three)
    }

    /// Converts from [`Codon<T>`] to [`Codon<&mut T>`].
    ///
    /// [`Codon<T>`]: struct.Codon.html
    /// [`Codon<&mut T>`]: struct.Codon.html
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    ///
    /// let mut codon = Codon('A', 'T', 'G');
    /// {
    ///     let Codon(one, two, three) = codon.as_mut();
    ///     *one = 'Z';
    /// } // Mutable ref goes out of scope here
    ///
    /// assert_eq!(codon, Codon('Z', 'T', 'G'));
    /// ```
    pub fn as_mut(&mut self) -> Codon<&mut T> {
        let Codon(ref mut one, ref mut two, ref mut three) = *self;
        Codon(one, two, three)
    }

    /// Applies a function taking and returning a 3-tuple to the elements of
    /// the [`Codon`] object.
    ///
    /// Input and output types don't have to be the same.
    ///
    /// [`Codon`]: struct.Codon.html
    ///
    /// Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    ///
    /// let codon = Codon('A', 'T', 'G');
    /// let mapped = codon.map(|one, _, _| (one as u8, b'A', b'A'));
    ///
    /// assert_eq!(mapped, Codon(b'A', b'A', b'A'));
    /// ```
    pub fn map<U, F: FnOnce(T, T, T) -> (U, U, U)>(self, f: F) -> Codon<U> {
        let Codon(one, two, three) = self;
        let (one, two, three) = f(one, two, three);
        Codon(one, two, three)
    }

    /// Applies a function taking a 3-tuple and returning a [`Codon`] object
    /// over the [`Codon`] object.
    ///
    /// [`Codon`]: struct.Codon.html
    ///
    /// Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    ///
    /// let codon = Codon(b'A', b'T', b'G');
    /// let mapped = codon.flat_map(|one, two, three| {
    ///     Codon(one, two - 1, three - 2)
    /// });
    ///
    /// assert_eq!(mapped, Codon(b'A', b'S', b'E'));
    /// ```
    pub fn flat_map<U, F: FnOnce(T, T, T) -> Codon<U>>(self, f: F) -> Codon<U> {
        let Codon(one, two, three) = self;
        f(one, two, three)
    }

    /// Applies a function over each of the [`Codon`] bases individually.
    ///
    /// [`Codon`]: struct.Codon.html
    ///
    /// Examples:
    ///
    /// ```
    /// use seqrs::codon::Codon;
    ///
    /// let codon = Codon('A', 'T', 'G');
    /// let mapped: Codon<u8> = codon.map_each(|b| b.to_ascii_lowercase() as u8);
    ///
    /// assert_eq!(mapped, Codon(b'a', b't', b'g'));
    /// ```
    pub fn map_each<U, F: Fn(T) -> U>(self, f: F) -> Codon<U> {
        let Codon(one, two, three) = self;
        Codon(f(one), f(two), f(three))
    }
}

impl<T: Alphabet + Clone> Codon<T> {
    /// The number of possible codons given the alphabet.
    pub fn cardinality() -> usize {
        let size = T::cardinality() as u32;
        3_u32.pow(size) as usize
    }

    /// The unique numeric rank for this 3-mer.
    pub fn rank(&self) -> usize {
        let size = T::cardinality() as usize;
        let b1 = (self.first().rank() as usize) * size.pow(2);
        let b2 = (self.second().rank() as usize) * size;
        let b3 = self.third().rank() as usize;
        return b1 + b2 + b3;
    }

    pub fn variants() -> Vec<Self> {
        let mut output = Vec::with_capacity(Self::cardinality());
        for b1 in T::variants() {
            for b2 in T::variants() {
                for b3 in T::variants() {
                    output.push(Codon(b1.clone(), b2.clone(), b3.clone()))
                }
            }
        }

        output
    }
}

impl<T: Default> Default for Codon<T> {
    fn default() -> Self {
        Self(T::default(), T::default(), T::default())
    }
}

impl<T> FromStr for Codon<T>
where
    T: TryFrom<char, Error = SeqError>,
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

// Iterator for codons.

/// Yields a iterator over elements from another iterator wrapped in [`Codon`].
///
/// This trait is automatically implemented for any iterator.
pub trait IntoCodons: Sized {
    fn codons(self) -> Codons<Self>;
}

impl<I: Iterator> IntoCodons for I {
    fn codons(self) -> Codons<I> {
        Codons { iter: self }
    }
}

/// An iterator adapter over codons in another iterator.
#[derive(Debug, Clone)]
pub struct Codons<I> {
    iter: I,
}

impl<I, T> Iterator for Codons<I>
where
    I: Iterator<Item = T>,
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
    /// use seqrs::codon::Codon;
    /// use seqrs::codon::IntoCodons;
    ///
    /// let seq = vec!['A', 'A'].into_iter().codons();
    /// assert_eq!((0, Some(0)), seq.size_hint());
    ///
    /// let seq = vec!['A', 'A', 'A'].into_iter().codons();
    /// assert_eq!((1, Some(1)), seq.size_hint());
    ///
    /// let seq = vec!['A', 'A', 'A', 'A'].into_iter().codons();
    /// assert_eq!((1, Some(1)), seq.size_hint());
    ///
    /// let seq = vec!['A', 'A', 'A', 'A', 'A'].into_iter().codons();
    /// assert_eq!((1, Some(1)), seq.size_hint());
    ///
    /// let seq = vec!['A', 'A', 'A', 'A', 'A', 'A'].into_iter().codons();
    /// assert_eq!((2, Some(2)), seq.size_hint());
    /// ```
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (length, _) = self.iter.size_hint();
        let length = length / 3;
        (length, Some(length))
    }
}

impl<I, T> DoubleEndedIterator for Codons<I>
where
    I: DoubleEndedIterator<Item = T> + ExactSizeIterator<Item = T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // Because the last codon might not end at the final position
        // Skip the last n % 3 bases.
        let n = self.iter.len() % 3;
        for _ in 0..n {
            self.iter.next_back()?;
        }

        // Codon is in reverse order.
        let three = self.iter.next_back()?;
        let two = self.iter.next_back()?;
        let one = self.iter.next_back()?;

        Some(Codon(one, two, three))
    }
}

impl<I, T> ExactSizeIterator for Codons<I>
where
    I: ExactSizeIterator<Item = T>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::AA;
    use crate::alphabet::DNA;
    use crate::alphabet::DNA::*;
    use crate::stopped::Stopped;
    use crate::stopped::Stopped::{Res, Stop};
    use crate::translate::NCBITransTable;

    #[test]
    fn test_codon_from_str() {
        let c = "ATG".parse::<Codon<DNA>>().unwrap();
        assert_eq!(c, Codon(A, T, G));
    }

    #[test]
    fn test_codon_iter() {
        let mut arr = vec![A, T, G, C].into_iter().codons();
        assert_eq!(Some(Codon(A, T, G)), arr.next());
        assert_eq!(None, arr.next());

        let v = vec![A, T, G, C];
        let mut arr = v.iter().codons();
        assert_eq!(Some(Codon(&A, &T, &G)), arr.next());
        assert_eq!(None, arr.next());
    }

    #[test]
    fn test_codon_rev_iter() {
        let mut arr = vec![A, T, G, C].into_iter().codons();
        assert_eq!(Some(Codon(A, T, G)), arr.next_back());
        assert_eq!(None, arr.next_back());

        let mut arr = vec![A, T, G, C, A, A, G, A].into_iter().codons();
        assert_eq!(Some(Codon(C, A, A)), arr.next_back());
        assert_eq!(Some(Codon(A, T, G)), arr.next_back());
        assert_eq!(None, arr.next_back());
    }

    #[test]
    fn test_codon_len() {
        let mut arr = vec![A, T, G, C].into_iter().codons();
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
        assert_eq!(met.first(), &A);
    }

    #[test]
    fn test_translate() {
        let met = Codon(A, T, G);
        assert_eq!(met.translate(&NCBITransTable::Standard), Res(AA::M));
    }

    #[test]
    fn test_translate_arr() {
        let arr = vec![Codon(A, T, G), Codon(C, T, C), Codon(T, A, G)];
        let mapped: Vec<Stopped<AA>> = arr
            .iter()
            .map(|c| c.translate(&NCBITransTable::Standard))
            .collect();
        assert_eq!(mapped, vec![Res(AA::M), Res(AA::L), Stop]);
    }
}
