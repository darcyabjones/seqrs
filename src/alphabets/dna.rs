//! Definitions for the DNA alphabet

use errors::SeqError;
use alphabets::Complement;
use charcase::to_upper;
use std::convert::TryFrom;

/// DNA is represented as an enum, including all IUPAC redundant bases.
#[derive(Debug, Clone, Copy, Hash)]
pub enum DNA {
    A,
    T,
    G,
    C,
    R, // A|G
    Y, // C|T
    S, // G|C
    W, // A|T
    K, // G|T
    M, // A|C
    B, // C|G|T
    D, // A|G|T
    H, // A|C|T
    V, // A|C|G
    N, // A|T|G|C
}

/// Read DNA from bytes.
/// Use try from because it's possible to receive invalid input.
impl TryFrom<char> for DNA {

    // Associated type for try from.
    type Error = SeqError;

    fn try_from(base: char) -> Result<Self, Self::Error> {
        match base.to_ascii_uppercase() {
            'A' => Ok(DNA::A),
            'T' => Ok(DNA::T),
            'G' => Ok(DNA::G),
            'C' => Ok(DNA::C),
            'R' => Ok(DNA::R),
            'Y' => Ok(DNA::Y),
            'S' => Ok(DNA::S),
            'W' => Ok(DNA::W),
            'K' => Ok(DNA::K),
            'M' => Ok(DNA::M),
            'B' => Ok(DNA::B),
            'D' => Ok(DNA::D),
            'H' => Ok(DNA::H),
            'V' => Ok(DNA::V),
            'N' => Ok(DNA::N),
            b   => Err(SeqError::AlphabetReadError { base: b as char }),
        }
    }
}

/// Convert DNA to byte representation.
impl From<DNA> for char {
    fn from(base: DNA) -> Self {
        match base {
            DNA::A => 'A',
            DNA::T => 'T',
            DNA::G => 'G',
            DNA::C => 'C',
            DNA::R => 'R',
            DNA::Y => 'Y',
            DNA::S => 'S',
            DNA::W => 'W',
            DNA::K => 'K',
            DNA::M => 'M',
            DNA::B => 'B',
            DNA::D => 'D',
            DNA::H => 'H',
            DNA::V => 'V',
            DNA::N => 'N',
        }
    }
}

/// Define equality for DNA bases.
/// For redundant bases, if it is possible to match evaluate as true.
impl PartialEq for DNA {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DNA::N, _     ) | (     _, DNA::N) => true,
            (DNA::A, DNA::A)                    => true,
            (DNA::T, DNA::T)                    => true,
            (DNA::G, DNA::G)                    => true,
            (DNA::C, DNA::C)                    => true,
            (DNA::R, DNA::R)                    => true,
            (DNA::Y, DNA::Y)                    => true,
            (DNA::S, DNA::S)                    => true,
            (DNA::W, DNA::W)                    => true,
            (DNA::K, DNA::K)                    => true,
            (DNA::M, DNA::M)                    => true,
            (DNA::R, DNA::A) | (DNA::R, DNA::G) => true,
            (DNA::Y, DNA::C) | (DNA::Y, DNA::T) => true,
            (DNA::S, DNA::G) | (DNA::S, DNA::C) => true,
            (DNA::W, DNA::A) | (DNA::W, DNA::T) => true,
            (DNA::K, DNA::G) | (DNA::K, DNA::T) => true,
            (DNA::M, DNA::A) | (DNA::M, DNA::C) => true,
            (DNA::A, DNA::R) | (DNA::G, DNA::R) => true,
            (DNA::C, DNA::Y) | (DNA::T, DNA::Y) => true,
            (DNA::G, DNA::S) | (DNA::C, DNA::S) => true,
            (DNA::A, DNA::W) | (DNA::T, DNA::W) => true,
            (DNA::G, DNA::K) | (DNA::T, DNA::K) => true,
            (DNA::A, DNA::M) | (DNA::C, DNA::M) => true,
            (DNA::R, DNA::S) | (DNA::S, DNA::R) => true, // G match
            (DNA::R, DNA::K) | (DNA::K, DNA::R) => true, // G match
            (DNA::R, DNA::W) | (DNA::W, DNA::R) => true, // A match
            (DNA::R, DNA::M) | (DNA::M, DNA::R) => true, // A match
            (DNA::Y, DNA::S) | (DNA::S, DNA::Y) => true, // C match
            (DNA::Y, DNA::M) | (DNA::M, DNA::Y) => true, // C match
            (DNA::Y, DNA::W) | (DNA::W, DNA::Y) => true, // T match
            (DNA::Y, DNA::K) | (DNA::K, DNA::Y) => true, // T match
            (DNA::S, DNA::K) | (DNA::K, DNA::S) => true, // G match
            (DNA::S, DNA::M) | (DNA::M, DNA::S) => true, // C match
            (DNA::W, DNA::M) | (DNA::M, DNA::W) => true, // A match
            (DNA::W, DNA::K) | (DNA::K, DNA::W) => true, // T match
            (DNA::B, DNA::A) | (DNA::A, DNA::B) => false,
            (DNA::D, DNA::C) | (DNA::C, DNA::D) => false,
            (DNA::H, DNA::G) | (DNA::G, DNA::H) => false,
            (DNA::V, DNA::T) | (DNA::T, DNA::V) => false,
            (DNA::B, _     ) | (     _, DNA::B) => true,
            (DNA::D, _     ) | (     _, DNA::D) => true,
            (DNA::H, _     ) | (     _, DNA::H) => true,
            (DNA::V, _     ) | (     _, DNA::V) => true,
            _                                   => false, // Everything else
        }
    }
}

/// Uses the partialeq definition.
impl Eq for DNA {}

/// How to complement the alphabet.
impl Complement for DNA {
    fn complement(&self) -> Self {
        match self {
            DNA::A => DNA::T,
            DNA::T => DNA::A,
            DNA::G => DNA::C,
            DNA::C => DNA::G,
            DNA::R => DNA::Y,
            DNA::Y => DNA::R,
            DNA::S => DNA::S,
            DNA::W => DNA::W,
            DNA::K => DNA::M,
            DNA::M => DNA::K,
            DNA::B => DNA::V,
            DNA::V => DNA::B,
            DNA::D => DNA::H,
            DNA::H => DNA::D,
            DNA::N => DNA::N,
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_from() {
        assert_eq!(DNA::try_from('A').unwrap(), DNA::A);
        assert_eq!(DNA::try_from('a').unwrap(), DNA::A);
        assert_eq!(DNA::try_from('T').unwrap(), DNA::T);
        assert_eq!(DNA::try_from('c').unwrap(), DNA::C);
        assert_eq!(DNA::try_from('G').unwrap(), DNA::G);
        assert_eq!(DNA::try_from('w').unwrap(), DNA::W);
    }

    #[test]
    fn test_complement() {
        assert_eq!(DNA::A.complement(), DNA::T);
        assert_eq!(DNA::T.complement(), DNA::A);
        assert_eq!(DNA::G.complement(), DNA::C);
        assert_eq!(DNA::C.complement(), DNA::G);
    }

    #[test]
    fn test_eq() {
        assert_eq!(DNA::A, DNA::A);
        assert_eq!(DNA::T, DNA::T);
        assert_eq!(DNA::G, DNA::G);
        assert_eq!(DNA::C, DNA::C);
        assert_ne!(DNA::A, DNA::T);
        assert_ne!(DNA::A, DNA::C);
        assert_ne!(DNA::A, DNA::G);
        assert_ne!(DNA::T, DNA::A);
        assert_ne!(DNA::T, DNA::C);
        assert_ne!(DNA::T, DNA::G);
        assert_ne!(DNA::C, DNA::G);
        assert_ne!(DNA::G, DNA::C);
        assert_eq!(DNA::A, DNA::N);
        assert_eq!(DNA::N, DNA::A);
        assert_eq!(DNA::N, DNA::N);
        assert_eq!(DNA::B, DNA::D); // CGT == AGT
        assert_eq!(DNA::R, DNA::S); // AG == GC
        assert_ne!(DNA::V, DNA::T); // ACG != T
        assert_ne!(DNA::R, DNA::Y); // AG != CT
        assert_ne!(DNA::M, DNA::K); // AC != GT
        assert_eq!(DNA::S, DNA::M); // GC != AC
        assert_eq!(DNA::S, DNA::S); // GC != AC
    }


    #[bench]
    fn bench_complement(b: &mut Bencher) {
        let seq = vec![DNA::A, DNA::T, DNA::C, DNA::G];
        b.iter(|| {
            for _i in 1..100 {
                for s in seq.iter() {
                    black_box(s.complement());
                }
            }
        })
    }

}
