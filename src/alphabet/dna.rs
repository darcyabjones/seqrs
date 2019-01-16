use errors::SeqError;
use ::{Complement, Match};

use std::convert::TryFrom;

alphabet! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum DNA {
        A = {
            bits: 0b0001,
            chr: b'A',
            name: "Alanine",
            compl: T,
            matches: [M, R, V, W, H, D, N],
        };
        C = {
            bits: 0b0010,
            chr: b'C',
            name: "Cytosine",
            compl: G,
            matches: [M, S, V, Y, H, B, N],
        };
        M = {
            bits: 0b0011,
            chr: b'M',
            name: "Alanine or Cytosine",
            compl: K,
            matches: [A, C, R, S, V, W, Y, H, D, B, N],
        };
        G = {
            bits: 0b0100,
            chr: b'G',
            name: "Guanine",
            compl: C,
            matches: [R, S, V, K, D, B, N],
        };
        R = {
            bits: 0b0101,
            chr: b'R',
            name: "Alanine or Guanine",
            compl: Y,
            matches: [A, M, G, S, V, W, H, K, D, B, N],
        };
        S = {
            bits: 0b0110,
            chr: b'S',
            name: "Cytosine or Guanine",
            compl: S,
            matches: [C, M, G, R, V, Y, H, K, D, B, N],
        };
        V = {
            bits: 0b0111,
            chr: b'V',
            name: "Alanine, Cytosine, or Guanine",
            compl: B,
            matches: [A, C, M, G, R, S, W, Y, H, K, D, B, N],
        };
        T = {
            bits: 0b1000,
            chr: b'T',
            name: "Thymine",
            compl: A,
            matches: [W, Y, H, K, D, B, N],
        };
        W = {
            bits: 0b1001,
            chr: b'W',
            name: "Alanine or Thymine",
            compl: W,
            matches: [A, M, R, V, T, Y, H, K, D, B, N],
        };
        Y = {
            bits: 0b1010,
            chr: b'Y',
            name: "Cytosine or Alanine",
            compl: R,
            matches: [C, M, S, V, T, W, H, K, D, B, N],
        };
        H = {
            bits: 0b1011,
            chr: b'H',
            name: "Alanine, Cytosine, or Thymine",
            compl: D,
            matches: [A, C, M, R, S, V, T, W, Y, K, D, B, N],
        };
        K = {
            bits: 0b1100,
            chr: b'K',
            name: "Guanine or Thymine",
            compl: M,
            matches: [G, R, S, V, T, W, Y, H, D, B, N],
        };
        D = {
            bits: 0b1101,
            chr: b'D',
            name: "Alanine, Guanine, or Thymine",
            compl: H,
            matches: [A, M, G, R, S, V, T, W, Y, H, K, B, N],
        };
        B = {
            bits: 0b1110,
            chr: b'B',
            name: "Cytosine, Guanine, or Thymine",
            compl: V,
            matches: [C, M, G, R, S, V, T, W, Y, H, K, D, N],
        };
        N = {
            bits: 0b1111,
            chr: b'N',
            name: "Any nucleotide",
            compl: N,
            matches: [A, C, M, G, R, S, V, T, W, Y, H, K, D, B],
        };
    }
}

impl Default for DNA {
    /// Returns [`N`][DNA::N].
    #[inline]
    fn default() -> Self { DNA::N }
}

impl From<&DNA> for DNA {
    fn from(d: &DNA) -> Self {
        *d
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

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
    fn test_to_from_u8() {
        for base in DNA::variants() {
            assert_eq!(DNA::try_from(u8::from(base)).unwrap(), base);
            assert_eq!(DNA::try_from(&u8::from(base)).unwrap(), base);
            assert_eq!(DNA::try_from(u8::from(base).to_ascii_lowercase()).unwrap(), base);
            assert_eq!(DNA::try_from(&u8::from(base).to_ascii_lowercase()).unwrap(), base);
        }
    }

    #[test]
    fn test_to_from_char() {
        for base in DNA::variants() {
            assert_eq!(DNA::try_from(char::from(base)).unwrap(), base);
            assert_eq!(DNA::try_from(&char::from(base)).unwrap(), base);
            assert_eq!(DNA::try_from(char::from(base).to_ascii_lowercase()).unwrap(), base);
            assert_eq!(DNA::try_from(&char::from(base).to_ascii_lowercase()).unwrap(), base);
        }
    }

    #[test]
    fn test_complement() {
        for base in DNA::variants() {
            assert_eq!(base.complement().complement(), base);
        }
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
