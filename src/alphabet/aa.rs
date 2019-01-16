/// Definitions for the Protein alphabet

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
