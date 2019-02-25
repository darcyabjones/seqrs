use crate::complement::Complement;
/// A fully redundant DNA alphabet.
use crate::errors::{SeqError, SeqErrorKind};
use crate::gapped::Gapped;
use crate::matcher::{Match, RedundantAlphabet};
use crate::alphabet::DNA4;

use std::convert::TryFrom;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum DNA {
    A = 0b0001,
    C = 0b0010,
    M = 0b0011,
    G = 0b0100,
    R = 0b0101,
    S = 0b0110,
    V = 0b0111,
    T = 0b1000,
    W = 0b1001,
    Y = 0b1010,
    H = 0b1011,
    K = 0b1100,
    D = 0b1101,
    B = 0b1110,
    N = 0b1111,
}

impl DNA {
    pub fn name(&self) -> String {
        match &self {
            DNA::A => String::from("Alanine"),
            DNA::C => String::from("Cytosine"),
            DNA::M => String::from("Alanine or Cytosine"),
            DNA::G => String::from("Guanine"),
            DNA::R => String::from("Alanine or Guanine"),
            DNA::S => String::from("Cytosine or Guanine"),
            DNA::V => String::from("Alanine, Cytosine, or Guanine"),
            DNA::T => String::from("Thymine"),
            DNA::W => String::from("Alanine or Thymine"),
            DNA::Y => String::from("Cytosine or Thymine"),
            DNA::H => String::from("Alanine, Cytosine, or Thymine"),
            DNA::K => String::from("Guanine or Thymine"),
            DNA::D => String::from("Alanine, Guanine, or Thymine"),
            DNA::B => String::from("Cytosine, Guanine, or Thymine"),
            DNA::N => String::from("Any nucleotide"),
        }
    }

    pub fn variants() -> Vec<Self> {
        vec![
            DNA::A,
            DNA::C,
            DNA::M,
            DNA::G,
            DNA::R,
            DNA::S,
            DNA::V,
            DNA::T,
            DNA::W,
            DNA::Y,
            DNA::H,
            DNA::K,
            DNA::D,
            DNA::B,
            DNA::N,
        ]
    }

    pub fn cardinality() -> usize {
        15
    }

    fn redundant_matches(&self) -> Vec<Self> {
        use super::DNA::*;
        match self {
            M => vec![A, C],
            R => vec![A, G],
            S => vec![C, G],
            V => vec![A, C, G],
            W => vec![A, T],
            Y => vec![C, T],
            H => vec![A, C, T],
            K => vec![G, T],
            D => vec![A, G, T],
            B => vec![C, G, T],
            N => vec![A, C, G, T],
            _ => Vec::new(),
        }
    }
}

impl From<&DNA> for DNA {
    fn from(b: &Self) -> Self {
        *b
    }
}

impl Default for DNA {
    /// Returns [`N`][DNA::N].
    #[inline]
    fn default() -> Self {
        DNA::N
    }
}

impl TryFrom<&u8> for DNA {
    type Error = SeqError;
    fn try_from(base: &u8) -> Result<Self, Self::Error> {
        match base.to_ascii_uppercase() {
            b'A' => Ok(DNA::A),
            b'C' => Ok(DNA::C),
            b'M' => Ok(DNA::M),
            b'G' => Ok(DNA::G),
            b'R' => Ok(DNA::R),
            b'S' => Ok(DNA::S),
            b'V' => Ok(DNA::V),
            b'T' => Ok(DNA::T),
            b'W' => Ok(DNA::W),
            b'Y' => Ok(DNA::Y),
            b'H' => Ok(DNA::H),
            b'K' => Ok(DNA::K),
            b'D' => Ok(DNA::D),
            b'B' => Ok(DNA::B),
            b'N' => Ok(DNA::N),
            b => Err(SeqErrorKind::AlphabetReadError { base: b as char }.into()),
        }
    }
}

impl TryFrom<u8> for DNA {
    type Error = SeqError;
    fn try_from(base: u8) -> Result<Self, Self::Error> {
        Self::try_from(&(base))
    }
}

impl TryFrom<&char> for DNA {
    type Error = SeqError;
    fn try_from(base: &char) -> Result<Self, Self::Error> {
        crate::utils::char_to_byte(base).and_then(|b| Self::try_from(&b))
    }
}

impl TryFrom<char> for DNA {
    type Error = SeqError;
    fn try_from(base: char) -> Result<Self, Self::Error> {
        crate::utils::char_to_byte(&base).and_then(|b| Self::try_from(&b))
    }
}

impl From<&DNA> for u8 {
    fn from(base: &DNA) -> Self {
        match base {
            DNA::A => b'A',
            DNA::C => b'C',
            DNA::M => b'M',
            DNA::G => b'G',
            DNA::R => b'R',
            DNA::S => b'S',
            DNA::V => b'V',
            DNA::T => b'T',
            DNA::W => b'W',
            DNA::Y => b'Y',
            DNA::H => b'H',
            DNA::K => b'K',
            DNA::D => b'D',
            DNA::B => b'B',
            DNA::N => b'N',
        }
    }
}

impl From<DNA> for u8 {
    fn from(base: DNA) -> Self {
        (&base).into()
    }
}

impl From<&DNA> for char {
    fn from(base: &DNA) -> Self {
        u8::from(base) as char
    }
}

impl From<DNA> for char {
    fn from(base: DNA) -> Self {
        u8::from(&base) as char
    }
}

impl std::fmt::Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl Complement for &DNA {
    type Compl = DNA;
    fn complement(self) -> Self::Compl {
        let a = *self as u8;
        let comp = (a & 0b0001) << 3 | (a & 0b0010) << 1 | (a & 0b0100) >> 1 | (a & 0b1000) >> 3;

        unsafe { std::mem::transmute::<u8, Self::Compl>(comp) }
    }
}

impl Complement for DNA {
    type Compl = DNA;
    fn complement(self) -> Self::Compl {
        (&self).complement()
    }
}

impl Complement for &Gapped<DNA> {
    type Compl = Gapped<DNA>;
    fn complement(self) -> Self::Compl {
        self.map(|a| a.complement())
    }
}

impl Complement for Gapped<DNA> {
    type Compl = Gapped<DNA>;
    fn complement(self) -> Self::Compl {
        (&self).complement()
    }
}

impl Complement for Gapped<&DNA> {
    type Compl = Gapped<DNA>;
    fn complement(self) -> Self::Compl {
        self.map(|a| a.complement())
    }
}

impl Match<DNA> for DNA {
    fn matches(&self, other: &DNA) -> bool {
        let a = *self as u8;
        let b = *other as u8;
        (a & b) > 0
    }
}

impl RedundantAlphabet for DNA {
    fn union(&self, other: &Self) -> Self {
        let a = *self as u8;
        let b = *other as u8;
        unsafe { std::mem::transmute::<u8, DNA>(a | b) }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let a = *self as u8;
        let b = *other as u8;
        let inter = a & b;
        if inter != 0 {
            unsafe { Some(std::mem::transmute::<u8, DNA>(inter)) }
        } else {
            None
        }
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        let a = *self as u8;
        let b = *other as u8;
        let inter = a & (!b);
        if inter != 0 {
            unsafe { Some(std::mem::transmute::<u8, DNA>(inter)) }
        } else {
            None
        }
    }

    fn is_redundant(&self) -> bool {
        match self {
            DNA::A | DNA::C | DNA::G | DNA::T => false,
            _ => true,
        }
    }
}


impl From<&DNA4> for DNA {
    fn from(base: &DNA4) -> Self {
        let a = *base as u8;
        let b: u8 = 0b0001 << a;

        unsafe { std::mem::transmute::<u8, DNA>(b) }
    }
}


impl From<DNA4> for DNA {
    fn from(base: DNA4) -> Self {
        (&base).into()
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
    use test::{black_box, Bencher};

    #[test]
    fn test_complement_vec() {
        let d = vec![DNA::A, DNA::T, DNA::G, DNA::C];
        let c: Vec<DNA> = d.into_iter().reverse_complement().collect();
        assert_eq!(c, vec![DNA::G, DNA::C, DNA::A, DNA::T]);

        let d = vec![DNA::A, DNA::T, DNA::G, DNA::C];
        let c: Vec<DNA> = d.iter().reverse_complement().collect();
        assert_eq!(c, vec![DNA::G, DNA::C, DNA::A, DNA::T]);
    }

    #[test]
    fn test_gapped_complement() {
        use crate::gapped::Gapped::{Base, Gap};

        assert_eq!(Gap::<DNA>.complement(), Gap);
        assert_eq!(Base(DNA::T).complement(), Base(DNA::A));

        assert_eq!(Base(&DNA::T).complement(), Base(DNA::A));
        assert_eq!((&Base(DNA::T)).complement(), Base(DNA::A));

        assert_eq!((&Base(&DNA::T)).complement(), Base(DNA::A));
    }

    #[test]
    fn test_gapped_complement_iter() {
        use crate::gapped::Gapped::{Base, Gap};

        let seq = vec![Base(DNA::A), Base(DNA::T), Gap, Base(DNA::G)];
        let comp: Vec<Gapped<DNA>> = seq.into_iter().reverse_complement().collect();
        assert_eq!(comp, vec![Base(DNA::C), Gap, Base(DNA::A), Base(DNA::T)]);

        let seq = vec![Base(DNA::A), Base(DNA::T), Gap, Base(DNA::G)];
        let comp: Vec<Gapped<DNA>> = seq.iter().reverse_complement().collect();
        assert_eq!(comp, vec![Base(DNA::C), Gap, Base(DNA::A), Base(DNA::T)]);
    }

    #[test]
    fn test_match() {
        assert!(DNA::A.matches(&DNA::A));
        assert!(DNA::T.matches(&DNA::T));
        assert!(DNA::G.matches(&DNA::G));
        assert!(DNA::C.matches(&DNA::C));
        assert!(DNA::A.matches(&DNA::N));
        assert!(DNA::N.matches(&DNA::A));
        assert!(DNA::N.matches(&DNA::N));
        assert!(DNA::B.matches(&DNA::D)); // CGT == AGT
        assert!(DNA::R.matches(&DNA::S)); // AG == GC
        assert!(DNA::S.matches(&DNA::M)); // GC != AC
        assert!(DNA::S.matches(&DNA::S)); // GC != AC
        assert!(DNA::A.doesnt_match(&DNA::T));
        assert!(DNA::A.doesnt_match(&DNA::C));
        assert!(DNA::A.doesnt_match(&DNA::G));
        assert!(DNA::T.doesnt_match(&DNA::A));
        assert!(DNA::T.doesnt_match(&DNA::C));
        assert!(DNA::T.doesnt_match(&DNA::G));
        assert!(DNA::C.doesnt_match(&DNA::G));
        assert!(DNA::G.doesnt_match(&DNA::C));
        assert!(DNA::V.doesnt_match(&DNA::T)); // ACG != T
        assert!(DNA::R.doesnt_match(&DNA::Y)); // AG != CT
        assert!(DNA::M.doesnt_match(&DNA::K)); // AC != GT
    }

    #[test]
    fn test_cardinality() {
        assert_eq!(DNA::cardinality(), 15);
    }


    #[test]
    fn test_from_dna4() {
        assert_eq!(DNA::from(DNA4::A), DNA::A);
        assert_eq!(DNA::from(DNA4::T), DNA::T);
        assert_eq!(DNA::from(DNA4::G), DNA::G);
        assert_eq!(DNA::from(DNA4::C), DNA::C);
    }

    proptest! {
        // Basic parsing properties.
        #[test]
        fn test_from_u8_doesnt_crash(c in any::<u8>()) {
            let _dummy = DNA::try_from(c);
        }

        #[test]
        fn test_from_char_doesnt_crash(c in any::<char>()) {
            let _dummy = DNA::try_from(c);
        }

        #[test]
        fn test_to_u8_doesnt_crash(b in select(DNA::variants())) {
            let _dummy = u8::from(b);
        }

        #[test]
        fn test_to_char_doesnt_crash(b in select(DNA::variants())) {
            let _dummy = char::from(b);
        }

        // converting from DNA to u8 and back to DNA should recover same base.
        #[test]
        fn test_from_to_u8_recovers_original(b in select(DNA::variants())) {
            assert_eq!(DNA::try_from(u8::from(b)).unwrap(), b);
            assert_eq!(DNA::try_from(&u8::from(b)).unwrap(), b);
            assert_eq!(
                DNA::try_from(u8::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
            assert_eq!(
                DNA::try_from(&u8::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
        }

        #[test]
        fn test_from_to_char_recovers_original(b in select(DNA::variants())) {
            assert_eq!(DNA::try_from(char::from(b)).unwrap(), b);
            assert_eq!(DNA::try_from(&char::from(b)).unwrap(), b);
            assert_eq!(
                DNA::try_from(char::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
            assert_eq!(
                DNA::try_from(&char::from(b).to_ascii_lowercase()).unwrap(),
                b
            );
        }

        // The complement of the complement of a base is just the base.
        #[test]
        fn test_complement_twice_recovers_original(b in select(DNA::variants())) {
            assert_eq!(b.complement().complement(), b);
        }

        // Test some properties of the redundant set-like operations.
        #[test]
        fn test_union_is_reciprocal(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
        ) {
            assert_eq!(base1.union(&base2), base2.union(&base1));
        }

        #[test]
        fn test_intersection_is_reciprocal(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
        ) {
            assert_eq!(base1.intersection(&base2), base2.intersection(&base1));
        }

        // A \ B == A intersect (B^complement^)
        #[test]
        fn test_intersection_gives_difference(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
        ) {
            // This will be None if base2 is N
            let compl: Option<DNA> = DNA::N.difference(&base2);

            // This will be None if base2 is N or no interection.
            let diff: Option<DNA> = compl
                .and_then(|c| base1.intersection(&c));

            assert_eq!(base1.difference(&base2), diff);
        }

        // A union (B union C) == (A union B) union C
        #[test]
        fn test_set_union_associative(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
            base3 in select(DNA::variants()),
        ) {
            let left = base1.union(&base2.union(&base3));
            let right = base1.union(&base2).union(&base3);
            assert_eq!(left, right);
        }

        // A inter (B inter C) == (A inter B) inter C
        #[test]
        fn test_set_intersection_associative(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
            base3 in select(DNA::variants()),
        ) {
            let left = base2
                .intersection(&base3)
                .and_then(|b| base1.intersection(&b));

            let right = base1
                .intersection(&base2)
                .and_then(|b| b.intersection(&base3));

            assert_eq!(left, right);
        }

        // A union (B intersection C) == (A union B) intersection (A union C)
        #[test]
        fn test_set_union_distributive(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
            base3 in select(DNA::variants()),
        ) {
            // map_or is used because empty sets are represented with None,
            // and we don't handle this in the intersection functions.
            let left = base2
                .intersection(&base3)
                .map_or(base1, |b| base1.union(&b));

            let right1 = base1.union(&base2);
            let right2 = base1.union(&base3);
            let right: Option<DNA> = right1.intersection(&right2);

            // Note that this will always at least be A (because union),
            // so unwrapping should be safe.
            assert_eq!(left, right.unwrap());
        }

        // A intersection (B union C) == (A inter B) union (A inter C)
        #[test]
        fn test_set_intersection_distributive(
            base1 in select(DNA::variants()),
            base2 in select(DNA::variants()),
            base3 in select(DNA::variants()),
        ) {
            let left: Option<DNA> = base1.intersection(&(base2.union(&base3)));

            let right1: Option<DNA> = base1.intersection(&base2);
            let right2: Option<DNA> = base1.intersection(&base3);

            // If right1 is none, just return right2.
            // If right1 is Some, map_or union over right2.
            // If right2 is None, returns right1.
            let right: Option<DNA> = right1.map_or(right2, |b| {
                right2.map_or(right1, |c| Some(b.union(&c)))
            });

            assert_eq!(left, right);
        }
    } // End proptest!

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
