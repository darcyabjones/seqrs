/// A generalised codon alphabet.

use alphabets::Complement;
use alphabets::Translate;
use alphabets::AminoAcid;
use alphabets::AminoAcid::*;
use alphabets::DNA;
use std::convert::TryFrom;


/// Codons represented as tuple struct.
/// The tuple struct with public fields is used to make pattern matching
/// easier. I think it's a reasonable choice, given that these should
/// essentially be immutable and the order of codon elements has an
/// unambiguous interpretation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Codon<T>(pub T, pub T, pub T);


/// Codon constructor and accessors.
impl<T> Codon<T> {

    pub fn new(first: T, second: T, third: T) -> Self {
        Codon(first, second, third)
    }

    pub fn first(self) -> T {
        self.0
    }

    pub fn second(self) -> T {
        self.1
    }

    pub fn third(self) -> T {
        self.2
    }
}


impl Translate<AminoAcid> for Codon<DNA> {
    fn translate(&self) -> AminoAcid {
        match self {
            Codon(DNA::G, DNA::C, _     ) => Ala,
            Codon(DNA::A, DNA::G, DNA::A) => Arg,
            Codon(DNA::A, DNA::G, DNA::G) => Arg,
            Codon(DNA::A, DNA::G, DNA::R) => Arg,
            Codon(DNA::C, DNA::G, _     ) => Arg,
            Codon(DNA::M, DNA::G, DNA::A) => Arg,
            Codon(DNA::M, DNA::G, DNA::G) => Arg,
            Codon(DNA::M, DNA::G, DNA::R) => Arg,
            Codon(DNA::A, DNA::A, DNA::T) => Asn,
            Codon(DNA::A, DNA::A, DNA::C) => Asn,
            Codon(DNA::A, DNA::A, DNA::Y) => Asn,
            Codon(DNA::G, DNA::A, DNA::T) => Asp,
            Codon(DNA::G, DNA::A, DNA::C) => Asp,
            Codon(DNA::G, DNA::A, DNA::Y) => Asp,
            Codon(DNA::R, DNA::A, DNA::T) => Asx,
            Codon(DNA::R, DNA::A, DNA::C) => Asx,
            Codon(DNA::R, DNA::A, DNA::Y) => Asx,
            Codon(DNA::T, DNA::G, DNA::T) => Cys,
            Codon(DNA::T, DNA::G, DNA::C) => Cys,
            Codon(DNA::T, DNA::G, DNA::Y) => Cys,
            Codon(DNA::C, DNA::A, DNA::A) => Gln,
            Codon(DNA::C, DNA::A, DNA::G) => Gln,
            Codon(DNA::C, DNA::A, DNA::R) => Gln,
            Codon(DNA::G, DNA::A, DNA::A) => Glu,
            Codon(DNA::G, DNA::A, DNA::G) => Glu,
            Codon(DNA::G, DNA::A, DNA::R) => Glu,
            Codon(DNA::S, DNA::A, DNA::A) => Glx,
            Codon(DNA::S, DNA::A, DNA::G) => Glx,
            Codon(DNA::S, DNA::A, DNA::R) => Glx,
            Codon(DNA::G, DNA::G, _     ) => Gly,
            Codon(DNA::C, DNA::A, DNA::T) => His,
            Codon(DNA::C, DNA::A, DNA::C) => His,
            Codon(DNA::C, DNA::A, DNA::Y) => His,
            Codon(DNA::A, DNA::T, DNA::A) => Ile,
            Codon(DNA::A, DNA::T, DNA::T) => Ile,
            Codon(DNA::A, DNA::T, DNA::C) => Ile,
            Codon(DNA::A, DNA::T, DNA::Y) => Ile,
            Codon(DNA::A, DNA::T, DNA::W) => Ile,
            Codon(DNA::A, DNA::T, DNA::H) => Ile,
            Codon(DNA::T, DNA::T, DNA::A) => Leu,
            Codon(DNA::T, DNA::T, DNA::G) => Leu,
            Codon(DNA::T, DNA::T, DNA::R) => Leu,
            Codon(DNA::C, DNA::T, _     ) => Leu,
            Codon(DNA::Y, DNA::T, DNA::A) => Leu,
            Codon(DNA::Y, DNA::T, DNA::G) => Leu,
            Codon(DNA::Y, DNA::T, DNA::R) => Leu,
            Codon(DNA::A, DNA::A, DNA::A) => Lys,
            Codon(DNA::A, DNA::A, DNA::G) => Lys,
            Codon(DNA::A, DNA::A, DNA::R) => Lys,
            Codon(DNA::A, DNA::T, DNA::G) => Met,
            Codon(DNA::T, DNA::T, DNA::T) => Phe,
            Codon(DNA::T, DNA::T, DNA::C) => Phe,
            Codon(DNA::T, DNA::T, DNA::Y) => Phe,
            Codon(DNA::C, DNA::C, _     ) => Pro,
            Codon(DNA::A, DNA::G, DNA::T) => Ser,
            Codon(DNA::A, DNA::G, DNA::C) => Ser,
            Codon(DNA::A, DNA::G, DNA::Y) => Ser,
            Codon(DNA::T, DNA::C, _     ) => Ser,
            Codon(DNA::A, DNA::C, _     ) => Thr,
            Codon(DNA::T, DNA::G, DNA::G) => Trp,
            Codon(DNA::T, DNA::A, DNA::T) => Tyr,
            Codon(DNA::T, DNA::A, DNA::C) => Tyr,
            Codon(DNA::T, DNA::A, DNA::Y) => Tyr,
            Codon(DNA::G, DNA::T, _     ) => Val,
            Codon(DNA::T, DNA::A, DNA::A) => Stop,
            Codon(DNA::T, DNA::A, DNA::G) => Stop,
            Codon(DNA::T, DNA::A, DNA::R) => Stop,
            Codon(DNA::T, DNA::R, DNA::A) => Stop,
            Codon(DNA::T, DNA::G, DNA::A) => Stop,
            Codon(DNA::T, DNA::G, DNA::R) => Stop,
            _                             => Xaa,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alphabets::DNA;
    use alphabets::AminoAcid;

    #[test]
    fn test_eq() {
        assert_eq!(Codon::new(DNA::A, DNA::T, DNA::G), Codon(DNA::A, DNA::T, DNA::G));
    }

    #[test]
    fn test_access() {
        let met = Codon(DNA::A, DNA::T, DNA::G);
        assert_eq!(met.0, DNA::A);
        assert_eq!(met.first(), DNA::A);
    }

    #[test]
    fn test_translate() {
        let met = Codon(DNA::A, DNA::T, DNA::G);
        assert_eq!(met.translate(), AminoAcid::Met);
    }
}

