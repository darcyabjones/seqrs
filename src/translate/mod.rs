//! Common translation tables for converting codons to Amino Acids.
//! Names and mappings are derived from the NCBI translation tables,
//! available at: <https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi#SG1>.
//! The tables are implemented as empty structs that act as tokens for which
//! implementation of the `TranslationTable` trait to use.

use std::marker::PhantomData;
use gapped::Gapped;

pub mod dna_aa;


#[derive(Debug, Clone, Copy)]
pub enum NCBITransTable {
    Standard,
    VertebrateMito,
    YeastMito,
    MoldProtozoanCoelenterateMito,
    InvertebrateMito,
    CiliateDasycladaceanHexamita,
    EchinodermFlatwormMito,
    Euplotid,
    BacterialArchaealPlastid,
    AltYeast,
    AscidianMito,
    AltFlatwormMito,
    BlepharismaMacronuclear,
    ChlorophyceanMito,
    TrematodeMito,
    ScenedesmusMito,
    ThraustochytriumMito,
    PterobranchiaMito,
    SR1Gracilibacteria,
    Pachysolen,
    Karyorelict,
    Condylostoma,
    Mesodinium,
    Peritrich,
    Blastocrithidia,
    BalanophoraceaePlastid,
    CephalodiscidaeMito,
}


impl Default for NCBITransTable {
    fn default() -> NCBITransTable {
        NCBITransTable::Standard
    }
}


impl NCBITransTable {
    pub fn variants() -> Vec<Self> {
        vec![
            NCBITransTable::Standard,
            NCBITransTable::VertebrateMito,
            NCBITransTable::YeastMito,
            NCBITransTable::MoldProtozoanCoelenterateMito,
            NCBITransTable::InvertebrateMito,
            NCBITransTable::CiliateDasycladaceanHexamita,
            NCBITransTable::EchinodermFlatwormMito,
            NCBITransTable::Euplotid,
            NCBITransTable::BacterialArchaealPlastid,
            NCBITransTable::AltYeast,
            NCBITransTable::AscidianMito,
            NCBITransTable::AltFlatwormMito,
            NCBITransTable::BlepharismaMacronuclear,
            NCBITransTable::ChlorophyceanMito,
            NCBITransTable::TrematodeMito,
            NCBITransTable::ScenedesmusMito,
            NCBITransTable::ThraustochytriumMito,
            NCBITransTable::PterobranchiaMito,
            NCBITransTable::SR1Gracilibacteria,
            NCBITransTable::Pachysolen,
            NCBITransTable::Karyorelict,
            NCBITransTable::Condylostoma,
            NCBITransTable::Mesodinium,
            NCBITransTable::Peritrich,
            NCBITransTable::Blastocrithidia,
            NCBITransTable::BalanophoraceaePlastid,
            NCBITransTable::CephalodiscidaeMito,
        ]
    }

    pub fn id_to_table(id: usize) -> Option<Self> {
        match id {
            1  => Some(NCBITransTable::Standard),
            2  => Some(NCBITransTable::VertebrateMito),
            3  => Some(NCBITransTable::YeastMito),
            4  => Some(NCBITransTable::MoldProtozoanCoelenterateMito),
            5  => Some(NCBITransTable::InvertebrateMito),
            6  => Some(NCBITransTable::CiliateDasycladaceanHexamita),
            9  => Some(NCBITransTable::EchinodermFlatwormMito),
            10 => Some(NCBITransTable::Euplotid),
            11 => Some(NCBITransTable::BacterialArchaealPlastid),
            12 => Some(NCBITransTable::AltYeast),
            13 => Some(NCBITransTable::AscidianMito),
            14 => Some(NCBITransTable::AltFlatwormMito),
            15 => Some(NCBITransTable::BlepharismaMacronuclear),
            16 => Some(NCBITransTable::ChlorophyceanMito),
            21 => Some(NCBITransTable::TrematodeMito),
            22 => Some(NCBITransTable::ScenedesmusMito),
            23 => Some(NCBITransTable::ThraustochytriumMito),
            24 => Some(NCBITransTable::PterobranchiaMito),
            25 => Some(NCBITransTable::SR1Gracilibacteria),
            26 => Some(NCBITransTable::Pachysolen),
            27 => Some(NCBITransTable::Karyorelict),
            28 => Some(NCBITransTable::Condylostoma),
            29 => Some(NCBITransTable::Mesodinium),
            30 => Some(NCBITransTable::Peritrich),
            31 => Some(NCBITransTable::Blastocrithidia),
            32 => Some(NCBITransTable::BalanophoraceaePlastid),
            33 => Some(NCBITransTable::CephalodiscidaeMito),
            _  => None,
        }
    }
}


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
        TranslationIterator { iter: self.into_iter(), table: table, b: PhantomData }
    }
}


#[derive(Clone)]
pub struct TranslationIterator<I, T, O> {
    iter: I,
    table: T,
    b: PhantomData<O>,
}


impl<O, I: Iterator, T> Iterator for TranslationIterator<I, T, O>
    where T: TranslationTable<I::Item, O>
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
    where T: TranslationTable<I::Item, O>,
          I: DoubleEndedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<O> {
        self.iter.next_back().map(|codon| self.table.get(&codon))
    }
}


impl<O, I, T> ExactSizeIterator for TranslationIterator<I, T, O>
    where T: TranslationTable<I::Item, O>,
          I: ExactSizeIterator,
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

    use alphabet::DNA::*;
    use alphabet::AA;
    use stopped::Stopped::{Res, StopOr, Stop};
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
