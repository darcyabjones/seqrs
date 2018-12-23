//! Definitions for the DNA alphabet

use errors::SeqError;
use ::{Complement, Match};
use ::alphabets::alphabet::char_to_byte;

use std::convert::TryFrom;

alphabet! {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum DNA {
        A = {
            bits: 0b0001,
            chr: b'A',
            compl: T,
            redundant: [M, R, V, W, H, D, N],
        };
        C = {
            bits: 0b0010,
            chr: b'C',
            compl: G,
            redundant: [M, S, V, Y, H, B, N],
        };
        M = {
            bits: 0b0011,
            chr: b'M',
            compl: K,
            redundant: [A, C, R, S, V, W, Y, H, D, B, N],
        };
        G = {
            bits: 0b0100,
            chr: b'G',
            compl: C,
            redundant: [R, S, V, K, D, B, N],
        };
        R = {
            bits: 0b0101,
            chr: b'R',
            compl: Y,
            redundant: [A, M, G, S, V, W, H, K, D, B, N],
        };
        S = {
            bits: 0b0110,
            chr: b'S',
            compl: S,
            redundant: [C, M, G, R, V, Y, H, K, D, B, N],
        };
        V = {
            bits: 0b0111,
            chr: b'V',
            compl: B,
            redundant: [A, C, M, G, R, S, W, Y, H, K, D, B, N],
        };
        T = {
            bits: 0b1000,
            chr: b'T',
            compl: A,
            redundant: [W, Y, H, K, D, B, N],
        };
        W = {
            bits: 0b1001,
            chr: b'W',
            compl: W,
            redundant: [A, M, R, V, T, Y, H, K, D, B, N],
        };
        Y = {
            bits: 0b1010,
            chr: b'Y',
            compl: R,
            redundant: [C, M, S, V, T, W, H, K, D, B, N],
        };
        H = {
            bits: 0b1011,
            chr: b'H',
            compl: D,
            redundant: [A, C, M, R, S, V, T, W, Y, K, D, B, N],
        };
        K = {
            bits: 0b1100,
            chr: b'K',
            compl: M,
            redundant: [G, R, S, V, T, W, Y, H, D, B, N],
        };
        D = {
            bits: 0b1101,
            chr: b'D',
            compl: H,
            redundant: [A, M, G, R, S, V, T, W, Y, H, K, B, N],
        };
        B = {
            bits: 0b1110,
            chr: b'B',
            compl: V,
            redundant: [C, M, G, R, S, V, T, W, Y, H, K, D, N],
        };
        N = {
            bits: 0b1111,
            chr: b'N',
            compl: N,
            redundant: [A, C, M, G, R, S, V, T, W, Y, H, K, D, B],
        };
    }
}

impl Default for DNA {
    /// Returns [`N`][DNA::N].
    #[inline]
    fn default() -> Self { DNA::N }
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
