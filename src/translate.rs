use std::collections::HashMap;
use lazy_static::lazy_static;

/*
pub trait Translate<'a, K, V> {
    type Iter: Iterator<Item=V>;
    fn translate(&'a self, table: &TranslationTable<K, V>) -> Self::Iter;
}

impl<'a, K, V, I, U> Translate<'a, K, V> for U
    where I: Iterator<Item=K>,
          U: IntoIterator<Item=K, IntoIter=I>,
{

    type Iter = std::iter::Map<I, Box<FnOnce(K) -> V>>;

    fn translate(&'a self, table: &TranslationTable<K, V>) -> Self::Iter {
        self.into_iter().map(|a| table.get(&a))
    }
}

pub trait Translate<'a, I, K: 'a, V: 'a> {

    type Item;
    type Iter;

    fn translate(self, table: &'a TranslationTable<K, V>) -> TranslateIterator<'a, I, K, V>;
}

impl<'a, U, I, K, V> Translate<'a, I, K, V> for U
    where U: IntoIterator<Item=K, IntoIter=I>,
          I: Iterator<Item=K>,
          K: 'a,
          V: 'a,
{
    type Item = V;
    type Iter = I;

    fn translate(self, table: &'a TranslationTable<K, V>) -> TranslateIterator<'a, I, K, V> {
        TranslateIterator { iter: self.into_iter(), table: table }
    }
}
*/


#[derive(Clone)]
pub struct TranslateIterator<I, T> {
    iter: I,
    table: T,
}

impl<I, T, V> Iterator for TranslateIterator<I, T>
    where I: Iterator,
          T: TranslationTable<I::Item, V>
{
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.table.translation_table(&e))
    }
}

/// Abstraction trait around hashmap accession functions.
/// Used so we can auto impl Translate and to allow lookup from
/// things other than HashMaps (e.g trees, functions etc).
///
/// NOTE: Will panic if table is not complete, e.g. is missing keys.
/// I can't find a way to easily get compile time checks on stuff
pub trait TranslationTable<K, V>: Sized {
    fn get(&self, k: &K) -> V;
}
/*
impl<K, V> TranslationTable<K, V> for HashMap<K, V>
    where K: std::hash::Hash + Eq,
          V: Clone,
{
    fn translation_table(&self, k: &K) -> V {
        self.get(k).map(|ref v| v.clone()).unwrap().to_owned()
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use ::translation_tables::dna_aa::Standard;
    use ::alphabet::DNA;
    use ::alphabet::DNA::*;
    use ::alphabet::AA;
    use ::alphabet::Codon;

    #[test]
    fn test_translate() {
        let table = Standard;
        let x: AA = TranslationTable::get(&table, &Codon(A, T, G));
        assert_eq!(x, AA::M);
    }

    #[test]
    fn test_trans() {
        let x: &[Codon<DNA>] = &[Codon(A, T, G), Codon(A, T, G)];
        let table = Standard;
        let mut iter = x.into_iter().map(|e| table.get(e));
        assert_eq!(iter.next().unwrap(), AA::M);
    }

}
