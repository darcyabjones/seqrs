/// Definitions for the Protein alphabet
use std::convert::TryFrom;

use crate::alphabet::Alphabet;
use crate::alphabet::RedundantAlphabet;
use crate::matcher::Match;
use crate::errors::{SeqError, SeqErrorKind};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AA {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl AA {
    pub fn is_iupac(&self) -> bool {
        match &self {
            AA::A => true,
            AA::B => true,
            AA::C => true,
            AA::D => true,
            AA::E => true,
            AA::F => true,
            AA::G => true,
            AA::H => true,
            AA::I => true,
            AA::J => false,
            AA::K => true,
            AA::L => true,
            AA::M => true,
            AA::N => true,
            AA::O => false,
            AA::P => true,
            AA::Q => true,
            AA::R => true,
            AA::S => true,
            AA::T => true,
            AA::U => false,
            AA::V => true,
            AA::W => true,
            AA::X => true,
            AA::Y => true,
            AA::Z => true,
        }
    }

    /*
    pub fn name(&self) -> String {
        match &self {
            AA::A => String::from("Alanine"),
            AA::B => String::from("Aspartic acid or Asparagine"),
            AA::C => String::from("Cysteine"),
            AA::D => String::from("Aspartic acid"),
            AA::E => String::from("Glutamic acid"),
            AA::F => String::from("Phenylalanine"),
            AA::G => String::from("Glycine"),
            AA::H => String::from("Histidine"),
            AA::I => String::from("Isoleucine"),
            AA::J => String::from("Isoleucine or Leucine"),
            AA::K => String::from("Lysine"),
            AA::L => String::from("Leucine"),
            AA::M => String::from("Methionine"),
            AA::N => String::from("Asparagine"),
            AA::O => String::from("Pyroleucine"),
            AA::P => String::from("Proline"),
            AA::Q => String::from("Glutamine"),
            AA::R => String::from("Arginine"),
            AA::S => String::from("Serine"),
            AA::T => String::from("Threonine"),
            AA::U => String::from("Selenocysteine"),
            AA::V => String::from("Valine"),
            AA::W => String::from("Tryptophan"),
            AA::X => String::from("Any amino acid"),
            AA::Y => String::from("Tyrosine"),
            AA::Z => String::from("Glutamine or Glutamic acid"),
        }
    }
    */

    fn redundant_matches(&self) -> Vec<Self> {
        use super::AA::*;
        match self {
            B => vec![D, N],
            J => vec![I, L],
            X => vec![
                A, C, D, E, F, G, H, I, K, L, M, N, O, P, Q, R, S, T, U, V, W, Y,
            ],
            Z => vec![E, Q],
            _ => Vec::new(),
        }
    }
}

impl Alphabet for AA {
    /// The number of letters in this alphabet.
    fn cardinality() -> usize {
        26
    }

    fn rank(&self) -> usize {
        *self as usize
    }

    unsafe fn from_rank_unsafe(r: usize) -> Self {
        debug_assert!(r < Self::cardinality());
        std::mem::transmute::<u8, Self>(r as u8)
    }

    /// Returns a Vec of all of the Enum variants.
    fn variants() -> Vec<Self> {
        vec![
            AA::A,
            AA::B,
            AA::C,
            AA::D,
            AA::E,
            AA::F,
            AA::G,
            AA::H,
            AA::I,
            AA::J,
            AA::K,
            AA::L,
            AA::M,
            AA::N,
            AA::O,
            AA::P,
            AA::Q,
            AA::R,
            AA::S,
            AA::T,
            AA::U,
            AA::V,
            AA::W,
            AA::X,
            AA::Y,
            AA::Z,
        ]
    }
}

try_from_borrowed! {
    impl TryFrom<&u8> for AA {
        type Error = SeqError;
        fn try_from(base: &u8) -> Result<Self, Self::Error> {
            match base.to_ascii_uppercase() {
                b'A' => Ok(AA::A),
                b'B' => Ok(AA::B),
                b'C' => Ok(AA::C),
                b'D' => Ok(AA::D),
                b'E' => Ok(AA::E),
                b'F' => Ok(AA::F),
                b'G' => Ok(AA::G),
                b'H' => Ok(AA::H),
                b'I' => Ok(AA::I),
                b'J' => Ok(AA::J),
                b'K' => Ok(AA::K),
                b'L' => Ok(AA::L),
                b'M' => Ok(AA::M),
                b'N' => Ok(AA::N),
                b'O' => Ok(AA::O),
                b'P' => Ok(AA::P),
                b'Q' => Ok(AA::Q),
                b'R' => Ok(AA::R),
                b'S' => Ok(AA::S),
                b'T' => Ok(AA::T),
                b'U' => Ok(AA::U),
                b'V' => Ok(AA::V),
                b'W' => Ok(AA::W),
                b'X' => Ok(AA::X),
                b'Y' => Ok(AA::Y),
                b'Z' => Ok(AA::Z),
                b => Err(SeqErrorKind::AlphabetReadError { base: b as char }.into()),
            }
        }
    }
}

try_from_borrowed! {
    impl TryFrom<&char> for AA {
        type Error = SeqError;
        fn try_from(base: &char) -> Result<Self, Self::Error> {
            crate::utils::char_to_byte(base).and_then(|b| Self::try_from(&b))
        }
    }
}

from_borrowed! {
    impl From<&AA> for u8 {
        fn from(base: &AA) -> Self {
            match base {
                AA::A => b'A',
                AA::B => b'B',
                AA::C => b'C',
                AA::D => b'D',
                AA::E => b'E',
                AA::F => b'F',
                AA::G => b'G',
                AA::H => b'H',
                AA::I => b'I',
                AA::J => b'J',
                AA::K => b'K',
                AA::L => b'L',
                AA::M => b'M',
                AA::N => b'N',
                AA::O => b'O',
                AA::P => b'P',
                AA::Q => b'Q',
                AA::R => b'R',
                AA::S => b'S',
                AA::T => b'T',
                AA::U => b'U',
                AA::V => b'V',
                AA::W => b'W',
                AA::X => b'X',
                AA::Y => b'Y',
                AA::Z => b'Z',
            }
        }
    }
}

from_borrowed! {
    impl From<&AA> for char {
        fn from(base: &AA) -> Self {
            u8::from(base) as char
        }
    }
}

impl std::fmt::Display for AA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl Match<AA> for AA {
    fn matches(&self, other: &AA) -> bool {
        match (&self, &other) {
            (a, b) if a == b => true,
            (_, AA::X) => true,
            (AA::X, _) => true,
            (AA::B, AA::D) => true,
            (AA::B, AA::N) => true,
            (AA::D, AA::B) => true,
            (AA::N, AA::B) => true,
            (AA::J, AA::I) => true,
            (AA::J, AA::L) => true,
            (AA::I, AA::J) => true,
            (AA::L, AA::J) => true,
            (AA::Z, AA::E) => true,
            (AA::Z, AA::Q) => true,
            (AA::E, AA::Z) => true,
            (AA::Q, AA::Z) => true,
            _ => false,
        }
    }
}

impl RedundantAlphabet for AA {
    fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (a, b) if a == b => *a,
            (AA::N, AA::D) => AA::B,
            (AA::D, AA::N) => AA::B,
            (AA::L, AA::I) => AA::J,
            (AA::I, AA::L) => AA::J,
            (AA::Q, AA::E) => AA::Z,
            (AA::E, AA::Q) => AA::Z,
            _ => AA::X,
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (a, b) if a == b => Some(*a),
            (AA::X, b) => Some(*b),
            (a, AA::X) => Some(*a),
            (AA::B, AA::D) => Some(AA::D),
            (AA::B, AA::N) => Some(AA::N),
            (AA::D, AA::B) => Some(AA::D),
            (AA::N, AA::B) => Some(AA::N),
            (AA::J, AA::I) => Some(AA::I),
            (AA::J, AA::L) => Some(AA::L),
            (AA::I, AA::J) => Some(AA::I),
            (AA::L, AA::J) => Some(AA::L),
            (AA::Z, AA::E) => Some(AA::E),
            (AA::Z, AA::Q) => Some(AA::Q),
            (AA::E, AA::Z) => Some(AA::E),
            (AA::Q, AA::Z) => Some(AA::Q),
            _ => None,
        }
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (a, b) if a == b => None,
            (_, AA::X) => None,
            (AA::X, _) => Some(AA::X),
            (AA::B, AA::D) => Some(AA::N),
            (AA::B, AA::N) => Some(AA::D),
            (AA::D, AA::B) => None,
            (AA::N, AA::B) => None,
            (AA::J, AA::I) => Some(AA::L),
            (AA::J, AA::L) => Some(AA::I),
            (AA::I, AA::J) => None,
            (AA::L, AA::J) => None,
            (AA::Z, AA::E) => Some(AA::Q),
            (AA::Z, AA::Q) => Some(AA::E),
            (AA::E, AA::Z) => None,
            (AA::Q, AA::Z) => None,
            (a, _) => Some(*a),
        }
    }

    fn is_redundant(&self) -> bool {
        match self {
            AA::B | AA::J | AA::X | AA::Z => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::any;
    use proptest::sample::select;
    use proptest::{proptest, proptest_helper};

    #[test]
    fn test_cardinality() {
        assert_eq!(AA::cardinality() as usize, AA::variants().len());
    }

    #[test]
    fn test_redundant_matches() {
        for aa in AA::variants() {
            assert!(aa.matches(&aa));
            for red_aa in aa.redundant_matches() {
                assert!(aa.matches(&red_aa));
            }
        }
    }

    proptest! {
        // Basic parsing properties.
        #[test]
        fn test_from_u8_doesnt_crash(c in any::<u8>()) {
            let _dummy = AA::try_from(c);
        }

        #[test]
        fn test_from_char_doesnt_crash(c in any::<char>()) {
            let _dummy = AA::try_from(c);
        }

        #[test]
        fn test_to_u8_doesnt_crash(b in select(AA::variants())) {
            let _dummy = u8::from(b);
        }

        #[test]
        fn test_to_char_doesnt_crash(b in select(AA::variants())) {
            let _dummy = char::from(b);
        }

        // converting from AA to u8 and back to AA should recover same aa.
        #[test]
        fn test_from_to_u8_recovers_original(b in select(AA::variants())) {
            assert_eq!(AA::try_from(u8::from(b)).unwrap(), b);
            assert_eq!(AA::try_from(&u8::from(b)).unwrap(), b);
            assert_eq!(
                AA::try_from(u8::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
            assert_eq!(
                AA::try_from(&u8::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
        }

        #[test]
        fn test_from_to_char_recovers_original(b in select(AA::variants())) {
            assert_eq!(AA::try_from(char::from(b)).unwrap(), b);
            assert_eq!(AA::try_from(&char::from(b)).unwrap(), b);
            assert_eq!(
                AA::try_from(char::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
            assert_eq!(
                AA::try_from(&char::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
        }

        // Test some properties of the redundant set-like operations.
        // Because we don't have every possible set permutation encoded in the
        // AA definition, we can't test set operations properly.
        #[test]
        fn test_union_is_reciprocal(
            base1 in select(AA::variants()),
            base2 in select(AA::variants()),
        ) {
            assert_eq!(base1.union(&base2), base2.union(&base1));
        }

        #[test]
        fn test_intersection_is_reciprocal(
            base1 in select(AA::variants()),
            base2 in select(AA::variants()),
        ) {
            assert_eq!(base1.intersection(&base2), base2.intersection(&base1));
        }

    }
}
