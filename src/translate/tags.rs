use std::marker::PhantomData;

pub trait CodonTagTable<K, V> {
    fn get_tag(&self, k: &K) -> V;
}

// Generic implementation for borrowed keys.
impl<K, V, T> CodonTagTable<&K, V> for T
where
    T: CodonTagTable<K, V>,
{
    fn get_tag(&self, k: &&K) -> V {
        <T as CodonTagTable<K, V>>::get_tag(self, k)
    }
}

pub trait IntoCodonTags<O, T>: Sized {
    fn codon_tags(self, table: T) -> CodonTags<Self, T, O>;
}

impl<O, T, I> IntoCodonTags<O, T> for I
where
    T: CodonTagTable<I::Item, O>,
    I: Iterator,
{
    fn codon_tags(self, table: T) -> CodonTags<Self, T, O> {
        CodonTags {
            iter: self,
            table: table,
            b: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodonTags<I, T, O> {
    iter: I,
    table: T,
    b: PhantomData<O>,
}

impl<O, I, T> Iterator for CodonTags<I, T, O>
where
    I: Iterator,
    T: CodonTagTable<I::Item, O>,
{
    type Item = O;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|codon| self.table.get_tag(&codon))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<O, I, T> DoubleEndedIterator for CodonTags<I, T, O>
where
    I: DoubleEndedIterator,
    T: CodonTagTable<I::Item, O>,
{
    #[inline]
    fn next_back(&mut self) -> Option<O> {
        self.iter
            .next_back()
            .map(|codon| self.table.get_tag(&codon))
    }
}

impl<O, I, T> ExactSizeIterator for CodonTags<I, T, O>
where
    I: ExactSizeIterator,
    T: CodonTagTable<I::Item, O>,
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

    use crate::alphabet::CodonTag;
    use crate::alphabet::DNA::*;
    use crate::codon::Codon;
    use crate::gapped::Gapped;
    use crate::gapped::Gapped::{Base, Gap};
    use crate::translate::NCBITransTable;

    #[test]
    fn test_codon_tag_table_ownership() {
        assert_eq!(
            NCBITransTable::Standard.get_tag(&Codon(A, T, G)),
            CodonTag::Start
        );

        assert_eq!(
            NCBITransTable::Standard.get_tag(&&Codon(A, T, G)),
            CodonTag::Start
        );

        assert_eq!(
            NCBITransTable::Standard.get_tag(&Base(Codon(A, T, G))),
            Base(CodonTag::Start)
        );

        assert_eq!(
            NCBITransTable::Standard.get_tag(&&Base(Codon(A, T, G))),
            Base(CodonTag::Start)
        );
    }

    #[test]
    fn test_codon_tags_iter() {
        let arr = vec![Codon(A, T, G), Codon(T, A, A)];

        let trans: Vec<CodonTag> = arr.iter().codon_tags(NCBITransTable::Standard).collect();

        assert_eq!(trans, vec![CodonTag::Start, CodonTag::Stop]);
        println!("I can still use this {:?}", arr);
    }

    #[test]
    fn test_codon_tags_into_iter() {
        let arr = vec![Codon(A, T, G), Codon(T, A, A)];

        let trans: Vec<CodonTag> = arr
            .into_iter()
            .codon_tags(NCBITransTable::Standard)
            .collect();

        assert_eq!(trans, vec![CodonTag::Start, CodonTag::Stop]);
    }

    #[test]
    fn test_gapped_codon_tags_iter() {
        let arr = vec![Base(Codon(A, T, G)), Gap, Base(Codon(T, A, A))];

        let trans: Vec<Gapped<CodonTag>> =
            arr.iter().codon_tags(NCBITransTable::Standard).collect();

        assert_eq!(
            trans,
            vec![Base(CodonTag::Start), Gap, Base(CodonTag::Stop)]
        );
        println!("I can still use this {:?}", arr);
    }

    #[test]
    fn test_gapped_codon_tags_into_iter() {
        let arr = vec![Base(Codon(A, T, G)), Gap, Base(Codon(T, A, A))];

        let trans: Vec<Gapped<CodonTag>> = arr
            .into_iter()
            .codon_tags(NCBITransTable::Standard)
            .collect();

        assert_eq!(
            trans,
            vec![Base(CodonTag::Start), Gap, Base(CodonTag::Stop)]
        );
    }
}
