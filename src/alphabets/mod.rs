pub mod dna;
pub mod amino_acid;
pub mod gapped;
pub mod complement;
pub mod codon;
pub mod translate;

pub use self::dna::DNA;
pub use self::amino_acid::AminoAcid;
pub use self::gapped::Gapped;
pub use self::complement::Complement;
pub use self::codon::Codon;
pub use self::translate::Translate;
