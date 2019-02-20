use std::marker::PhantomData;
use gapped::Gapped;


/// TranslationTable is a wrapper around some mapping function.
///
/// TranslationTable is implemented for 'tag' structs to allow different table
/// behaviour.
pub trait TranslationTable<K, V> {
    fn get(&self, k: &K) -> V;
}


impl<K, V, T> TranslationTable<Gapped<K>, Gapped<V>> for T
    where T: TranslationTable<K, V>
{
    fn get(&self, k: &Gapped<K>) -> Gapped<V> {
        k.as_ref().map(|c| self.get(c))
    }
}


pub trait Translate<O, T>: IntoIterator {
    fn translate(self, table: T) -> TranslationIterator<Self::IntoIter, T, O>;
}


impl<O, T, I> Translate<O, T> for I
    where T: TranslationTable<I::Item, O>,
          I: IntoIterator
{
    fn translate(self, table: T) -> TranslationIterator<Self::IntoIter, T, O> {
        TranslationIterator {
            iter: self.into_iter(),
            table: table,
            b: PhantomData
        }
    }
}


#[derive(Debug, Clone)]
pub struct TranslationIterator<I, T, O> {
    iter: I,
    table: T,
    b: PhantomData<O>,
}


impl<O, I, T> Iterator for TranslationIterator<I, T, O>
    where I: Iterator,
          T: TranslationTable<I::Item, O>
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


impl<O, I, T> DoubleEndedIterator for TranslationIterator<I, T, O>
    where I: DoubleEndedIterator,
          T: TranslationTable<I::Item, O>,
{
    #[inline]
    fn next_back(&mut self) -> Option<O> {
        self.iter.next_back().map(|codon| self.table.get(&codon))
    }
}


impl<O, I, T> ExactSizeIterator for TranslationIterator<I, T, O>
    where I: ExactSizeIterator,
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

    use translate::NCBITransTable;
    use alphabet::DNA::*;
    use alphabet::AA;
    use stopped::Stopped::{Res, Stop};
    use codon::Codon;
    use gapped::Gapped::{Gap, Base};

    #[test]
    fn test_translate_arr() {
        let arr = vec![
            Codon(A, T, G),
            Codon(T, A, A),
        ];

        let mut translated = arr.translate(NCBITransTable::Standard);

        assert_eq!(2, translated.len());
        assert_eq!(Some(Res(AA::M)), translated.next());
        assert_eq!(Some(Stop), translated.next());
        assert_eq!(None, translated.next());
    }

    #[test]
    fn test_translate_arr_rev() {
        let arr = vec![
            Codon(A, T, G),
            Codon(T, A, A),
        ];

        let mut translated = arr.translate(NCBITransTable::Standard).rev();
        assert_eq!(Some(Stop), translated.next());
        assert_eq!(Some(Res(AA::M)), translated.next());
        assert_eq!(None, translated.next());
    }

    #[test]
    fn test_translate_arr_gapped() {
        let arr = vec![
            Base(Codon(A, T, G)),
            Gap,
            Base(Codon(T, A, A)),
        ];

        let mut translated = arr.translate(NCBITransTable::Standard);
        assert_eq!(Some(Base(Res(AA::M))), translated.next());
        assert_eq!(Some(Gap), translated.next());
        assert_eq!(Some(Base(Stop)), translated.next());
        assert_eq!(None, translated.next());
    }
}
