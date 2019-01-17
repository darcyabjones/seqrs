/// Definitions for the Protein alphabet

use ::RedundantAlphabet;
use errors::SeqError;
use crate::Match;
use std::convert::TryFrom;

alphabet! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum AA {
        A = {
            chr: b'A',
            name: "Alanine",
            matches: [X],
            is_iupac: true,
        };
        B = {
            chr: b'B',
            name: "Aspartic acid or Asparagine",
            matches: [D, N, X],
            is_iupac: true,
        };
        C = {
            chr: b'C',
            name: "Cysteine",
            matches: [X],
            is_iupac: true,
        };
        D = {
            chr: b'D',
            name: "Aspartic acid",
            matches: [X],
            is_iupac: true,
        };
        E = {
            chr: b'E',
            name: "Glutamic acid",
            matches: [X],
            is_iupac: true,
        };
        F = {
            chr: b'F',
            name: "Phenylalanine",
            matches: [X],
            is_iupac: true,
        };
        G = {
            chr: b'G',
            name: "Glycine",
            matches: [X],
            is_iupac: true,
        };
        H = {
            chr: b'H',
            name: "Histidine",
            matches: [X],
            is_iupac: true,
        };
        I = {
            chr: b'I',
            name: "Isoleucine",
            matches: [X],
            is_iupac: true,
        };
        J = { // Used by mascot
            chr: b'J',
            name: "Isoleucine or Leucine",
            matches: [I, L, X],
            is_iupac: false,
        };
        K = {
            chr: b'K',
            name: "Lysine",
            matches: [X],
            is_iupac: true,
        };
        L = {
            chr: b'L',
            name: "Leucine",
            matches: [X],
            is_iupac: true,
        };
        M = {
            chr: b'M',
            name: "Methionine",
            matches: [X],
            is_iupac: true,
        };
        N = {
            chr: b'N',
            name: "Asparagine",
            matches: [X],
            is_iupac: true,
        };
        O = {
            chr: b'O',
            name: "Pyroleucine",
            matches: [X],
            is_iupac: false,
        };
        P = {
            chr: b'P',
            name: "Proline",
            matches: [X],
            is_iupac: true,
        };
        Q = {
            chr: b'Q',
            name: "Glutamine",
            matches: [X],
            is_iupac: true,
        };
        R = {
            chr: b'R',
            name: "Arginine",
            matches: [X],
            is_iupac: true,
        };
        S = {
            chr: b'S',
            name: "Serine",
            matches: [X],
            is_iupac: true,
        };
        T = {
            chr: b'T',
            name: "Threonine",
            matches: [X],
            is_iupac: true,
        };
        U = {
            chr: b'U',
            name: "Selenocysteine",
            matches: [X],
            is_iupac: false,
        };
        V = {
            chr: b'V',
            name: "Valine",
            matches: [X],
            is_iupac: true,
        };
        W = {
            chr: b'W',
            name: "Tryptophan",
            matches: [X],
            is_iupac: true,
        };
        X = {
            chr: b'X',
            name: "Any amino acid",
            matches: [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, Y, Z],
            is_iupac: true,
        };
        Y = {
            chr: b'Y',
            name: "Tyrosine",
            matches: [X],
            is_iupac: true,
        };
        Z = {
            chr: b'Z',
            name: "Glutamine or Glutamic acid",
            matches: [E, Q, X],
            is_iupac: true,
        };
        Stop = {
            chr: b'*',
            name: "Stop",
            matches: [],
            is_iupac: true,
        };
    }
}

impl Default for AA {
    /// Returns [`X`][AA::X].
    #[inline]
    fn default() -> Self { AA::X }
}

impl From<&AA> for AA {
    fn from(d: &AA) -> Self {
        *d
    }
}

impl RedundantAlphabet for AA {
    fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (   AA::A,    AA::A) => AA::A,
            (   AA::B,    AA::B) => AA::B,
            (   AA::B,    AA::D) => AA::B,
            (   AA::B,    AA::N) => AA::B,
            (   AA::C,    AA::C) => AA::C,
            (   AA::D,    AA::B) => AA::B,
            (   AA::D,    AA::D) => AA::D,
            (   AA::D,    AA::N) => AA::B,
            (   AA::E,    AA::E) => AA::E,
            (   AA::E,    AA::Q) => AA::Z,
            (   AA::E,    AA::Z) => AA::Z,
            (   AA::F,    AA::F) => AA::F,
            (   AA::G,    AA::G) => AA::G,
            (   AA::H,    AA::H) => AA::H,
            (   AA::I,    AA::I) => AA::I,
            (   AA::I,    AA::J) => AA::J,
            (   AA::I,    AA::L) => AA::J,
            (   AA::J,    AA::I) => AA::J,
            (   AA::J,    AA::J) => AA::J,
            (   AA::J,    AA::L) => AA::J,
            (   AA::K,    AA::K) => AA::K,
            (   AA::L,    AA::I) => AA::J,
            (   AA::L,    AA::J) => AA::J,
            (   AA::L,    AA::L) => AA::L,
            (   AA::M,    AA::M) => AA::M,
            (   AA::N,    AA::B) => AA::B,
            (   AA::N,    AA::D) => AA::B,
            (   AA::N,    AA::N) => AA::N,
            (   AA::O,    AA::O) => AA::O,
            (   AA::P,    AA::P) => AA::P,
            (   AA::Q,    AA::E) => AA::Z,
            (   AA::Q,    AA::Q) => AA::Q,
            (   AA::Q,    AA::Z) => AA::Z,
            (   AA::R,    AA::R) => AA::R,
            (   AA::S,    AA::S) => AA::S,
            (   AA::T,    AA::T) => AA::T,
            (   AA::U,    AA::U) => AA::U,
            (   AA::V,    AA::V) => AA::V,
            (   AA::W,    AA::W) => AA::W,
            (   AA::Y,    AA::Y) => AA::Y,
            (   AA::Z,    AA::E) => AA::Z,
            (   AA::Z,    AA::Q) => AA::Z,
            (   AA::Z,    AA::Z) => AA::Z,
            (AA::Stop, AA::Stop) => AA::Stop,
            (       _,        _) => AA::X,
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (   AA::A,    AA::A) => Some(AA::A),
            (   AA::B,    AA::B) => Some(AA::B),
            (   AA::B,    AA::D) => Some(AA::D),
            (   AA::B,    AA::N) => Some(AA::N),
            (   AA::C,    AA::C) => Some(AA::C),
            (   AA::D,    AA::B) => Some(AA::D),
            (   AA::D,    AA::D) => Some(AA::D),
            (   AA::D,    AA::N) => None,
            (   AA::E,    AA::E) => Some(AA::E),
            (   AA::E,    AA::Q) => None,
            (   AA::E,    AA::Z) => Some(AA::E),
            (   AA::F,    AA::F) => Some(AA::F),
            (   AA::G,    AA::G) => Some(AA::G),
            (   AA::H,    AA::H) => Some(AA::H),
            (   AA::I,    AA::I) => Some(AA::I),
            (   AA::I,    AA::J) => Some(AA::I),
            (   AA::I,    AA::L) => None,
            (   AA::J,    AA::I) => Some(AA::I),
            (   AA::J,    AA::J) => Some(AA::J),
            (   AA::J,    AA::L) => Some(AA::L),
            (   AA::K,    AA::K) => Some(AA::K),
            (   AA::L,    AA::I) => None,
            (   AA::L,    AA::J) => Some(AA::L),
            (   AA::L,    AA::L) => Some(AA::L),
            (   AA::M,    AA::M) => Some(AA::M),
            (   AA::N,    AA::B) => Some(AA::N),
            (   AA::N,    AA::D) => None,
            (   AA::N,    AA::N) => Some(AA::N),
            (   AA::O,    AA::O) => Some(AA::O),
            (   AA::P,    AA::P) => Some(AA::P),
            (   AA::Q,    AA::E) => None,
            (   AA::Q,    AA::Q) => Some(AA::Q),
            (   AA::Q,    AA::Z) => Some(AA::Q),
            (   AA::R,    AA::R) => Some(AA::R),
            (   AA::S,    AA::S) => Some(AA::S),
            (   AA::T,    AA::T) => Some(AA::T),
            (   AA::U,    AA::U) => Some(AA::U),
            (   AA::V,    AA::V) => Some(AA::V),
            (   AA::W,    AA::W) => Some(AA::W),
            (   AA::X,        a) => Some(*a),
            (       a,    AA::X) => Some(*a),
            (   AA::Y,    AA::Y) => Some(AA::Y),
            (   AA::Z,    AA::E) => Some(AA::E),
            (   AA::Z,    AA::Q) => Some(AA::Q),
            (   AA::Z,    AA::Z) => Some(AA::Z),
            (AA::Stop, AA::Stop) => Some(AA::Stop),
            (       _,        _) => None,
        }
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (       _,    AA::X) => None,
            (   AA::A,    AA::A) => None,
            (   AA::B,    AA::B) => None,
            (   AA::B,    AA::D) => Some(AA::N),
            (   AA::B,    AA::N) => Some(AA::D),
            (   AA::C,    AA::C) => None,
            (   AA::D,    AA::B) => None,
            (   AA::D,    AA::D) => None,
            (   AA::E,    AA::E) => None,
            (   AA::E,    AA::Z) => None,
            (   AA::F,    AA::F) => None,
            (   AA::G,    AA::G) => None,
            (   AA::H,    AA::H) => None,
            (   AA::I,    AA::I) => None,
            (   AA::I,    AA::J) => None,
            (   AA::J,    AA::I) => Some(AA::L),
            (   AA::J,    AA::J) => None,
            (   AA::J,    AA::L) => Some(AA::I),
            (   AA::K,    AA::K) => None,
            (   AA::L,    AA::J) => None,
            (   AA::L,    AA::L) => None,
            (   AA::M,    AA::M) => None,
            (   AA::N,    AA::B) => None,
            (   AA::N,    AA::N) => None,
            (   AA::O,    AA::O) => None,
            (   AA::P,    AA::P) => None,
            (   AA::Q,    AA::Q) => None,
            (   AA::Q,    AA::Z) => None,
            (   AA::R,    AA::R) => None,
            (   AA::S,    AA::S) => None,
            (   AA::T,    AA::T) => None,
            (   AA::U,    AA::U) => None,
            (   AA::V,    AA::V) => None,
            (   AA::W,    AA::W) => None,
            (   AA::X,        _) => Some(AA::X),
            (   AA::Y,    AA::Y) => None,
            (   AA::Z,    AA::E) => Some(AA::Q),
            (   AA::Z,    AA::Q) => Some(AA::E),
            (   AA::Z,    AA::Z) => None,
            (AA::Stop, AA::Stop) => None,
            (       a,        _) => Some(*a),
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

    #[test]
    fn test_from() {
        assert_eq!(AA::try_from('A').unwrap(),  AA::A);
        assert_eq!(AA::try_from('C').unwrap(),  AA::C);
        assert_eq!(AA::try_from('Z').unwrap(),  AA::Z);
        assert_eq!(AA::try_from('G').unwrap(),  AA::G);
        assert_eq!(AA::try_from(b'G').unwrap(), AA::G);
    }
}
