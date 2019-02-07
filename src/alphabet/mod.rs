mod dna;
mod gapped;
mod aa;
mod codon;

pub use self::dna::DNA;
pub use self::aa::AA;
pub use self::gapped::Gapped;
pub use self::codon::{Codon, Codons};
