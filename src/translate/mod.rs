//! Common translation tables for converting codons to Amino Acids.
//! Names and mappings are derived from the NCBI translation tables,
//! available at: <https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi#SG1>.
//! The tables are implemented as empty structs that act as tokens for which
//! implementation of the `TranslationTable` trait to use.

mod tags;
mod trans;

pub use crate::translate::tags::{CodonTagTable, CodonTags};
pub use crate::translate::trans::{IntoTranslate, Translate, TranslationTable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            1 => Some(NCBITransTable::Standard),
            2 => Some(NCBITransTable::VertebrateMito),
            3 => Some(NCBITransTable::YeastMito),
            4 => Some(NCBITransTable::MoldProtozoanCoelenterateMito),
            5 => Some(NCBITransTable::InvertebrateMito),
            6 => Some(NCBITransTable::CiliateDasycladaceanHexamita),
            9 => Some(NCBITransTable::EchinodermFlatwormMito),
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
            _ => None,
        }
    }
}

