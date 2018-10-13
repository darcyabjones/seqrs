#![feature(test)]
extern crate test;

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
    Gap,
}

impl From<u8> for DNA {
    fn from(base: u8) -> Self {
        match base {
            b'a' | b'A' => DNA::A,
            b't' | b'T' => DNA::T,
            b'g' | b'G' => DNA::G,
            b'c' | b'C' => DNA::C,
            b'r' | b'R' => DNA::R,
            b'y' | b'Y' => DNA::Y,
            b's' | b'S' => DNA::S,
            b'w' | b'W' => DNA::W,
            b'k' | b'K' => DNA::K,
            b'm' | b'M' => DNA::M,
            b'b' | b'B' => DNA::B,
            b'd' | b'D' => DNA::D,
            b'h' | b'H' => DNA::H,
            b'v' | b'V' => DNA::V,
            b'n' | b'N' => DNA::N,
            b'-'        => DNA::Gap,
            b           => panic!("Encountered unknown base {}", b),
        }
    }
}

impl From<DNA> for u8 {
    fn from(base: DNA) -> Self {
        match base {
            DNA::A   => b'A',
            DNA::T   => b'T',
            DNA::G   => b'G',
            DNA::C   => b'C',
            DNA::R   => b'R',
            DNA::Y   => b'Y',
            DNA::S   => b'S',
            DNA::W   => b'W',
            DNA::K   => b'K',
            DNA::M   => b'M',
            DNA::B   => b'B',
            DNA::D   => b'D',
            DNA::H   => b'H',
            DNA::V   => b'V',
            DNA::N   => b'N',
            DNA::Gap => b'-',
        }
    }
}

impl PartialEq for DNA {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DNA::Gap, DNA::Gap)                => true,
            (DNA::Gap, _)    | (_, DNA::Gap)    => false,
            (DNA::N, _)      | (_, DNA::N)      => true,
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
            (DNA::B, DNA::A) | (DNA::A, DNA::B) => false,
            (DNA::D, DNA::C) | (DNA::C, DNA::D) => false,
            (DNA::H, DNA::G) | (DNA::G, DNA::H) => false,
            (DNA::V, DNA::T) | (DNA::T, DNA::V) => false,
            (DNA::B, _)      | (_, DNA::B)      => true,
            (DNA::D, _)      | (_, DNA::D)      => true,
            (DNA::H, _)      | (_, DNA::H)      => true,
            (DNA::V, _)      | (_, DNA::V)      => true,
            _ => false,
        }
    }
}

impl Eq for DNA {}

trait Complement {
    fn complement(&self) -> Self;
}

pub struct Seq<'a> {
    seq: &'a [DNA]
}

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
            DNA::Gap => DNA::Gap,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_from() {
        assert_eq!(DNA::from(b'A'), DNA::A);
        assert_eq!(DNA::from(b'a'), DNA::A);
        assert_eq!(DNA::from(b'-'), DNA::Gap);
        assert_eq!(DNA::from(b'T'), DNA::T);
        assert_eq!(DNA::from(b'c'), DNA::C);
        assert_eq!(DNA::from(b'G'), DNA::G);
        assert_eq!(DNA::from(b'w'), DNA::W);
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
        assert_eq!(DNA::A, DNA::N);
        assert_eq!(DNA::N, DNA::A);
        assert_eq!(DNA::N, DNA::N);
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
