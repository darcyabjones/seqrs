use std::collections::HashMap;
use lazy_static::lazy_static;

pub trait Translate<K, V, T> where T: TranslationTable<K, V> {
    fn translate(&self, table: T) -> V;
}

impl<K, V, T> Translate<K, V, T> for K
    where T: TranslationTable<K, V>,
{
    fn translate(&self, table: T) -> V {
        table.translation_table(&self)
    }
}


#[derive(Debug, Clone)]
struct TranslateIterator<I, K, V> {
    table: TranslationTable<K, V>,
    iter: I,
}

impl<I, T, K, V> Iterator for TranslateIterator<I, K, V>
    where I: Iterator,
          T: TranslationTable<K, V>,
{
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.table.translation_table(e))
    }
}


/// Abstraction trait around hashmap accession functions.
/// Used so we can auto impl Translate and to allow lookup from
/// things other than HashMaps (e.g trees, functions etc).
///
/// NOTE: Will panic if table is not complete, e.g. is missing keys.
/// I can't find a way to easily get compile time checks on stuff
pub trait TranslationTable<K, V> {
    fn translation_table(&self, k: &K) -> V;
}

impl<K, V> TranslationTable<K, V> for HashMap<K, V>
    where K: std::hash::Hash + Eq,
          V: Clone,
{
    fn translation_table(&self, k: &K) -> V {
        self.get(k).map(|ref v| v.clone()).unwrap().to_owned()
    }
}

lazy_static! {
    static ref COUNT: HashMap<u8, char> = {
        let mut m = HashMap::with_capacity(5);

        m.insert(0, 'a');
        m.insert(1, 'b');
        m.insert(2, 'c');
        m.insert(3, 'd');
        m.insert(4, 'e');
        m
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let x: char = COUNT.translation_table(&1);
        assert_eq!(x, 'b');
    }

}
