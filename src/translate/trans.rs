//! Traits and iterators for Codon translation into amino acids.

use std::marker::PhantomData;

/// TranslationTable is a wrapper around some mapping function.
///
/// TranslationTable is implemented for 'tag' structs or enums to allow different table types
/// behaviour.
pub trait TranslationTable<K, V> {
    fn get(&self, k: &K) -> V;
}

// Generic implementation for borrowed keys.
impl<K, V, T> TranslationTable<&K, V> for T
where
    T: TranslationTable<K, V>,
{
    fn get(&self, k: &&K) -> V {
        <T as TranslationTable<K, V>>::get(self, k)
    }
}

pub trait IntoTranslate<O, T>: Sized {
    fn translate(self, table: T) -> Translate<Self, T, O>;
}

impl<O, T, I> IntoTranslate<O, T> for I
where
    T: TranslationTable<I::Item, O>,
    I: Iterator,
{
    fn translate(self, table: T) -> Translate<Self, T, O> {
        Translate {
            iter: self,
            table: table,
            b: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Translate<I, T, O> {
    iter: I,
    table: T,
    b: PhantomData<O>,
}

impl<O, I, T> Iterator for Translate<I, T, O>
where
    I: Iterator,
    T: TranslationTable<I::Item, O>,
{
    type Item = O;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|codon| self.table.get(&codon))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<O, I, T> DoubleEndedIterator for Translate<I, T, O>
where
    I: DoubleEndedIterator,
    T: TranslationTable<I::Item, O>,
{
    #[inline]
    fn next_back(&mut self) -> Option<O> {
        self.iter.next_back().map(|codon| self.table.get(&codon))
    }
}

impl<O, I, T> ExactSizeIterator for Translate<I, T, O>
where
    I: ExactSizeIterator,
    T: TranslationTable<I::Item, O>,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::IntoTranslate;
    use crate::alphabet::AA;
    use crate::alphabet::DNA::*;
    use crate::codon::Codon;
    use crate::gapped::Gapped;
    use crate::gapped::Gapped::{Base, Gap};
    use crate::stopped::Stopped;
    use crate::stopped::Stopped::{Res, Stop};
    use crate::translate::NCBITransTable;

    #[test]
    fn test_trans_table_ownership() {
        assert_eq!(NCBITransTable::Standard.get(&Codon(A, T, G)), Res(AA::M));

        assert_eq!(NCBITransTable::Standard.get(&&Codon(A, T, G)), Res(AA::M));

        assert_eq!(
            NCBITransTable::Standard.get(&Base(Codon(A, T, G))),
            Base(Res(AA::M))
        );

        assert_eq!(
            NCBITransTable::Standard.get(&&Base(Codon(A, T, G))),
            Base(Res(AA::M))
        );
    }

    #[test]
    fn test_translate_iter() {
        let arr = vec![Codon(A, T, G), Codon(T, A, A)];

        let trans: Vec<Stopped<AA>> = arr.iter().translate(NCBITransTable::Standard).collect();

        assert_eq!(trans, vec![Res(AA::M), Stop]);
        println!("I can still use this {:?}", arr);
    }

    #[test]
    fn test_translate_into_iter() {
        let arr = vec![Codon(A, T, G), Codon(T, A, A)];

        let trans: Vec<Stopped<AA>> = arr
            .into_iter()
            .translate(NCBITransTable::Standard)
            .collect();

        assert_eq!(trans, vec![Res(AA::M), Stop]);
    }

    #[test]
    fn test_translate_iter_gapped() {
        let arr = vec![Base(Codon(A, T, G)), Gap, Base(Codon(T, A, A))];

        let trans: Vec<Gapped<Stopped<AA>>> =
            arr.iter().translate(NCBITransTable::Standard).collect();
        assert_eq!(trans, vec![Base(Res(AA::M)), Gap, Base(Stop)]);
        println!("I can still use this {:?}", arr);
    }

    #[test]
    fn test_translate_into_iter_gapped() {
        let arr = vec![Base(Codon(A, T, G)), Gap, Base(Codon(T, A, A))];

        let trans: Vec<Gapped<Stopped<AA>>> = arr
            .into_iter()
            .translate(NCBITransTable::Standard)
            .collect();
        assert_eq!(trans, vec![Base(Res(AA::M)), Gap, Base(Stop)]);
    }
}
