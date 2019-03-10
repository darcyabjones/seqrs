//! A standard non-redundant DNA alphabet.
use std::convert::TryFrom;

use crate::alphabet::Alphabet;
use crate::alphabet::DNA;
use crate::complement::Complement;
use crate::errors::{SeqError, SeqErrorKind};
use crate::matcher::Match;

/// A Non-redundant four letter DNA alphabet.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum DNA4 {
    A = 0b00,
    C = 0b01,
    G = 0b10,
    T = 0b11,
}

impl Alphabet for DNA4 {
    /// The number of letters in this alphabet.
    fn cardinality() -> u8 {
        4
    }

    fn rank(&self) -> u8 {
        *self as u8
    }

    unsafe fn from_rank_unsafe(r: u8) -> Self {
        debug_assert!(r < Self::cardinality());
        std::mem::transmute::<u8, Self>(r)
    }

    /// Returns a Vec of all of the Enum variants.
    fn variants() -> Vec<Self> {
        vec![DNA4::A, DNA4::C, DNA4::G, DNA4::T]
    }
}

try_from_borrowed! {
    impl TryFrom<&u8> for DNA4 {
        type Error = SeqError;
        fn try_from(base: &u8) -> Result<Self, Self::Error> {
            match base.to_ascii_uppercase() {
                b'A' => Ok(DNA4::A),
                b'C' => Ok(DNA4::C),
                b'G' => Ok(DNA4::G),
                b'T' => Ok(DNA4::T),
                b => Err(SeqErrorKind::AlphabetReadError { base: b as char }.into()),
            }
        }
    }
}

try_from_borrowed! {
    impl TryFrom<&char> for DNA4 {
        type Error = SeqError;
        fn try_from(base: &char) -> Result<Self, Self::Error> {
            crate::utils::char_to_byte(base).and_then(|b| Self::try_from(&b))
        }
    }
}

from_borrowed! {
    impl From<&DNA4> for u8 {
        fn from(base: &DNA4) -> Self {
            match base {
                DNA4::A => b'A',
                DNA4::C => b'C',
                DNA4::G => b'G',
                DNA4::T => b'T',
            }
        }
    }
}

from_borrowed! {
    impl From<&DNA4> for char {
        fn from(base: &DNA4) -> Self {
            u8::from(base) as char
        }
    }
}

impl std::fmt::Display for DNA4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl Complement for DNA4 {
    type Compl = DNA4;
    fn complement(&self) -> Self::Compl {
        let a = *self as u8;
        let comp = a ^ 0b11;

        unsafe { std::mem::transmute::<u8, Self::Compl>(comp) }
    }
}

impl Match<DNA4> for DNA4 {
    fn matches(&self, other: &DNA4) -> bool {
        self == other
    }
}

try_from_borrowed! {
    impl TryFrom<&DNA> for DNA4 {
        type Error = SeqError;
        fn try_from(base: &DNA) -> Result<Self, Self::Error> {
            let a = *base as u8;
            // If DNA has more than 1 bit set it's redundant, so not representable.
            if a.count_ones() > 1 {
                Err(SeqErrorKind::RedundantAlphabetConversionError {
                    base: char::from(base),
                }
                .into())
            } else {
                // This is effectively the log2.
                // DNA and DNA4 are arranged so that they have this exponent relationship.
                let b = a.trailing_zeros() as u8;
                unsafe { Ok(std::mem::transmute::<u8, DNA4>(b)) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complement::IntoReverseComplement;
    use crate::gapped::Gapped;
    use proptest::prelude::any;
    use proptest::sample::select;
    use proptest::{proptest, proptest_helper};

    #[test]
    fn test_complement_vec() {
        use super::DNA4::*;
        let d = vec![A, T, G, C];
        let c: Vec<DNA4> = d.into_iter().reverse_complement().collect();
        assert_eq!(c, vec![G, C, A, T]);

        let d = vec![A, T, G, C];
        let c: Vec<DNA4> = d.iter().reverse_complement().collect();
        assert_eq!(c, vec![G, C, A, T]);
    }

    #[test]
    fn test_gapped_complement() {
        use crate::gapped::Gapped::{Base, Gap};

        assert_eq!(Gap::<DNA4>.complement(), Gap);
        assert_eq!(Base(DNA4::T).complement(), Base(DNA4::A));

        assert_eq!(Base(&DNA4::T).complement(), Base(DNA4::A));
        assert_eq!((&Base(DNA4::T)).complement(), Base(DNA4::A));

        assert_eq!((&Base(&DNA4::T)).complement(), Base(DNA4::A));
    }

    #[test]
    fn test_gapped_complement_iter() {
        use super::DNA4::*;
        use crate::gapped::Gapped::{Base, Gap};

        let seq = vec![Base(A), Base(T), Gap, Base(G)];
        let comp: Vec<Gapped<DNA4>> = seq.into_iter().reverse_complement().collect();
        assert_eq!(comp, vec![Base(C), Gap, Base(A), Base(T)]);

        let seq = vec![Base(A), Base(T), Gap, Base(G)];
        let comp: Vec<Gapped<DNA4>> = seq.iter().reverse_complement().collect();
        assert_eq!(comp, vec![Base(C), Gap, Base(A), Base(T)]);
    }

    #[test]
    fn test_match() {
        assert!(DNA4::A.matches(&DNA4::A));
        assert!(DNA4::T.matches(&DNA4::T));
        assert!(DNA4::G.matches(&DNA4::G));
        assert!(DNA4::C.matches(&DNA4::C));
        assert!(DNA4::A.doesnt_match(&DNA4::T));
        assert!(DNA4::A.doesnt_match(&DNA4::C));
        assert!(DNA4::A.doesnt_match(&DNA4::G));
    }

    #[test]
    fn test_from_dna() {
        assert_eq!(DNA4::try_from(DNA::A).unwrap(), DNA4::A);
        assert_eq!(DNA4::try_from(DNA::T).unwrap(), DNA4::T);
        assert_eq!(DNA4::try_from(DNA::G).unwrap(), DNA4::G);
        assert_eq!(DNA4::try_from(DNA::C).unwrap(), DNA4::C);

        assert!(DNA4::try_from(DNA::N).is_err());
        assert!(DNA4::try_from(DNA::M).is_err());
        assert!(DNA4::try_from(DNA::W).is_err());
    }

    #[test]
    fn test_cardinality() {
        assert_eq!(DNA4::cardinality(), 4);
    }

    proptest! {
        // Basic parsing properties.
        #[test]
        fn test_from_u8_doesnt_crash(c in any::<u8>()) {
            let _dummy = DNA4::try_from(c);
        }

        #[test]
        fn test_from_char_doesnt_crash(c in any::<char>()) {
            let _dummy = DNA4::try_from(c);
        }

        #[test]
        fn test_to_u8_doesnt_crash(b in select(DNA4::variants())) {
            let _dummy = u8::from(b);
        }

        #[test]
        fn test_to_char_doesnt_crash(b in select(DNA4::variants())) {
            let _dummy = char::from(b);
        }

        // converting from DNA to u8 and back to DNA should recover same base.
        #[test]
        fn test_from_to_u8_recovers_original(b in select(DNA4::variants())) {
            assert_eq!(DNA4::try_from(u8::from(b)).unwrap(), b);
            assert_eq!(DNA4::try_from(&u8::from(b)).unwrap(), b);
            assert_eq!(
                DNA4::try_from(u8::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
            assert_eq!(
                DNA4::try_from(&u8::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
        }

        #[test]
        fn test_from_to_char_recovers_original(b in select(DNA4::variants())) {
            assert_eq!(DNA4::try_from(char::from(b)).unwrap(), b);
            assert_eq!(DNA4::try_from(&char::from(b)).unwrap(), b);
            assert_eq!(
                DNA4::try_from(char::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
            assert_eq!(
                DNA4::try_from(&char::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
        }

    } // End proptest!
}
