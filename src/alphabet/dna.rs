/// A fully redundant DNA alphabet.

use errors::{SeqError, SeqErrorKind};
use ::{Complement, Match, RedundantAlphabet};

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
            name: "Cytosine or Thymine",
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

impl RedundantAlphabet for DNA {

    fn union(&self, other: &Self) -> Self {
        // This implementation can be rewritten to use bit operations.
        // Currently favoring simpler approaches to see what the compiler does.
        match (&self, &other) {
            (DNA::A, DNA::A) => DNA::A,
            (DNA::A, DNA::C) => DNA::M,
            (DNA::A, DNA::M) => DNA::M,
            (DNA::A, DNA::G) => DNA::R,
            (DNA::A, DNA::R) => DNA::R,
            (DNA::A, DNA::S) => DNA::V,
            (DNA::A, DNA::V) => DNA::V,
            (DNA::A, DNA::T) => DNA::W,
            (DNA::A, DNA::W) => DNA::W,
            (DNA::A, DNA::Y) => DNA::H,
            (DNA::A, DNA::H) => DNA::H,
            (DNA::A, DNA::K) => DNA::D,
            (DNA::A, DNA::D) => DNA::D,
            (DNA::A, DNA::B) => DNA::N,
            (DNA::A, DNA::N) => DNA::N,
            (DNA::C, DNA::A) => DNA::M,
            (DNA::C, DNA::C) => DNA::C,
            (DNA::C, DNA::M) => DNA::M,
            (DNA::C, DNA::G) => DNA::S,
            (DNA::C, DNA::R) => DNA::V,
            (DNA::C, DNA::S) => DNA::S,
            (DNA::C, DNA::V) => DNA::V,
            (DNA::C, DNA::T) => DNA::Y,
            (DNA::C, DNA::W) => DNA::H,
            (DNA::C, DNA::Y) => DNA::Y,
            (DNA::C, DNA::H) => DNA::H,
            (DNA::C, DNA::K) => DNA::B,
            (DNA::C, DNA::D) => DNA::N,
            (DNA::C, DNA::B) => DNA::B,
            (DNA::C, DNA::N) => DNA::N,
            (DNA::M, DNA::A) => DNA::M,
            (DNA::M, DNA::C) => DNA::M,
            (DNA::M, DNA::M) => DNA::M,
            (DNA::M, DNA::G) => DNA::V,
            (DNA::M, DNA::R) => DNA::V,
            (DNA::M, DNA::S) => DNA::V,
            (DNA::M, DNA::V) => DNA::V,
            (DNA::M, DNA::T) => DNA::H,
            (DNA::M, DNA::W) => DNA::H,
            (DNA::M, DNA::Y) => DNA::H,
            (DNA::M, DNA::H) => DNA::H,
            (DNA::M, DNA::K) => DNA::N,
            (DNA::M, DNA::D) => DNA::N,
            (DNA::M, DNA::B) => DNA::N,
            (DNA::M, DNA::N) => DNA::N,
            (DNA::G, DNA::A) => DNA::R,
            (DNA::G, DNA::C) => DNA::S,
            (DNA::G, DNA::M) => DNA::V,
            (DNA::G, DNA::G) => DNA::G,
            (DNA::G, DNA::R) => DNA::R,
            (DNA::G, DNA::S) => DNA::S,
            (DNA::G, DNA::V) => DNA::V,
            (DNA::G, DNA::T) => DNA::K,
            (DNA::G, DNA::W) => DNA::D,
            (DNA::G, DNA::Y) => DNA::B,
            (DNA::G, DNA::H) => DNA::N,
            (DNA::G, DNA::K) => DNA::K,
            (DNA::G, DNA::D) => DNA::D,
            (DNA::G, DNA::B) => DNA::B,
            (DNA::G, DNA::N) => DNA::N,
            (DNA::R, DNA::A) => DNA::R,
            (DNA::R, DNA::C) => DNA::V,
            (DNA::R, DNA::M) => DNA::V,
            (DNA::R, DNA::G) => DNA::R,
            (DNA::R, DNA::R) => DNA::R,
            (DNA::R, DNA::S) => DNA::V,
            (DNA::R, DNA::V) => DNA::V,
            (DNA::R, DNA::T) => DNA::D,
            (DNA::R, DNA::W) => DNA::D,
            (DNA::R, DNA::Y) => DNA::N,
            (DNA::R, DNA::H) => DNA::N,
            (DNA::R, DNA::K) => DNA::D,
            (DNA::R, DNA::D) => DNA::D,
            (DNA::R, DNA::B) => DNA::N,
            (DNA::R, DNA::N) => DNA::N,
            (DNA::S, DNA::A) => DNA::V,
            (DNA::S, DNA::C) => DNA::S,
            (DNA::S, DNA::M) => DNA::V,
            (DNA::S, DNA::G) => DNA::S,
            (DNA::S, DNA::R) => DNA::V,
            (DNA::S, DNA::S) => DNA::S,
            (DNA::S, DNA::V) => DNA::V,
            (DNA::S, DNA::T) => DNA::B,
            (DNA::S, DNA::W) => DNA::N,
            (DNA::S, DNA::Y) => DNA::B,
            (DNA::S, DNA::H) => DNA::N,
            (DNA::S, DNA::K) => DNA::B,
            (DNA::S, DNA::D) => DNA::N,
            (DNA::S, DNA::B) => DNA::B,
            (DNA::S, DNA::N) => DNA::N,
            (DNA::V, DNA::A) => DNA::V,
            (DNA::V, DNA::C) => DNA::V,
            (DNA::V, DNA::M) => DNA::V,
            (DNA::V, DNA::G) => DNA::V,
            (DNA::V, DNA::R) => DNA::V,
            (DNA::V, DNA::S) => DNA::V,
            (DNA::V, DNA::V) => DNA::V,
            (DNA::V, DNA::T) => DNA::N,
            (DNA::V, DNA::W) => DNA::N,
            (DNA::V, DNA::Y) => DNA::N,
            (DNA::V, DNA::H) => DNA::N,
            (DNA::V, DNA::K) => DNA::N,
            (DNA::V, DNA::D) => DNA::N,
            (DNA::V, DNA::B) => DNA::N,
            (DNA::V, DNA::N) => DNA::N,
            (DNA::T, DNA::A) => DNA::W,
            (DNA::T, DNA::C) => DNA::Y,
            (DNA::T, DNA::M) => DNA::H,
            (DNA::T, DNA::G) => DNA::K,
            (DNA::T, DNA::R) => DNA::D,
            (DNA::T, DNA::S) => DNA::B,
            (DNA::T, DNA::V) => DNA::N,
            (DNA::T, DNA::T) => DNA::T,
            (DNA::T, DNA::W) => DNA::W,
            (DNA::T, DNA::Y) => DNA::Y,
            (DNA::T, DNA::H) => DNA::H,
            (DNA::T, DNA::K) => DNA::K,
            (DNA::T, DNA::D) => DNA::D,
            (DNA::T, DNA::B) => DNA::B,
            (DNA::T, DNA::N) => DNA::N,
            (DNA::W, DNA::A) => DNA::W,
            (DNA::W, DNA::C) => DNA::H,
            (DNA::W, DNA::M) => DNA::H,
            (DNA::W, DNA::G) => DNA::D,
            (DNA::W, DNA::R) => DNA::D,
            (DNA::W, DNA::S) => DNA::N,
            (DNA::W, DNA::V) => DNA::N,
            (DNA::W, DNA::T) => DNA::W,
            (DNA::W, DNA::W) => DNA::W,
            (DNA::W, DNA::Y) => DNA::H,
            (DNA::W, DNA::H) => DNA::H,
            (DNA::W, DNA::K) => DNA::D,
            (DNA::W, DNA::D) => DNA::D,
            (DNA::W, DNA::B) => DNA::N,
            (DNA::W, DNA::N) => DNA::N,
            (DNA::Y, DNA::A) => DNA::H,
            (DNA::Y, DNA::C) => DNA::Y,
            (DNA::Y, DNA::M) => DNA::H,
            (DNA::Y, DNA::G) => DNA::B,
            (DNA::Y, DNA::R) => DNA::N,
            (DNA::Y, DNA::S) => DNA::B,
            (DNA::Y, DNA::V) => DNA::N,
            (DNA::Y, DNA::T) => DNA::Y,
            (DNA::Y, DNA::W) => DNA::H,
            (DNA::Y, DNA::Y) => DNA::Y,
            (DNA::Y, DNA::H) => DNA::H,
            (DNA::Y, DNA::K) => DNA::B,
            (DNA::Y, DNA::D) => DNA::N,
            (DNA::Y, DNA::B) => DNA::B,
            (DNA::Y, DNA::N) => DNA::N,
            (DNA::H, DNA::A) => DNA::H,
            (DNA::H, DNA::C) => DNA::H,
            (DNA::H, DNA::M) => DNA::H,
            (DNA::H, DNA::G) => DNA::N,
            (DNA::H, DNA::R) => DNA::N,
            (DNA::H, DNA::S) => DNA::N,
            (DNA::H, DNA::V) => DNA::N,
            (DNA::H, DNA::T) => DNA::H,
            (DNA::H, DNA::W) => DNA::H,
            (DNA::H, DNA::Y) => DNA::H,
            (DNA::H, DNA::H) => DNA::H,
            (DNA::H, DNA::K) => DNA::N,
            (DNA::H, DNA::D) => DNA::N,
            (DNA::H, DNA::B) => DNA::N,
            (DNA::H, DNA::N) => DNA::N,
            (DNA::K, DNA::A) => DNA::D,
            (DNA::K, DNA::C) => DNA::B,
            (DNA::K, DNA::M) => DNA::N,
            (DNA::K, DNA::G) => DNA::K,
            (DNA::K, DNA::R) => DNA::D,
            (DNA::K, DNA::S) => DNA::B,
            (DNA::K, DNA::V) => DNA::N,
            (DNA::K, DNA::T) => DNA::K,
            (DNA::K, DNA::W) => DNA::D,
            (DNA::K, DNA::Y) => DNA::B,
            (DNA::K, DNA::H) => DNA::N,
            (DNA::K, DNA::K) => DNA::K,
            (DNA::K, DNA::D) => DNA::D,
            (DNA::K, DNA::B) => DNA::B,
            (DNA::K, DNA::N) => DNA::N,
            (DNA::D, DNA::A) => DNA::D,
            (DNA::D, DNA::C) => DNA::N,
            (DNA::D, DNA::M) => DNA::N,
            (DNA::D, DNA::G) => DNA::D,
            (DNA::D, DNA::R) => DNA::D,
            (DNA::D, DNA::S) => DNA::N,
            (DNA::D, DNA::V) => DNA::N,
            (DNA::D, DNA::T) => DNA::D,
            (DNA::D, DNA::W) => DNA::D,
            (DNA::D, DNA::Y) => DNA::N,
            (DNA::D, DNA::H) => DNA::N,
            (DNA::D, DNA::K) => DNA::D,
            (DNA::D, DNA::D) => DNA::D,
            (DNA::D, DNA::B) => DNA::N,
            (DNA::D, DNA::N) => DNA::N,
            (DNA::B, DNA::A) => DNA::N,
            (DNA::B, DNA::C) => DNA::B,
            (DNA::B, DNA::M) => DNA::N,
            (DNA::B, DNA::G) => DNA::B,
            (DNA::B, DNA::R) => DNA::N,
            (DNA::B, DNA::S) => DNA::B,
            (DNA::B, DNA::V) => DNA::N,
            (DNA::B, DNA::T) => DNA::B,
            (DNA::B, DNA::W) => DNA::N,
            (DNA::B, DNA::Y) => DNA::B,
            (DNA::B, DNA::H) => DNA::N,
            (DNA::B, DNA::K) => DNA::B,
            (DNA::B, DNA::D) => DNA::N,
            (DNA::B, DNA::B) => DNA::B,
            (DNA::B, DNA::N) => DNA::N,
            (DNA::N,      _) => DNA::N,
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        // This implementation can be rewritten to use bit operations.
        // Currently favoring simpler approaches to see what the compiler does.
        match (&self, &other) {
            (DNA::A, DNA::A) => Some(DNA::A),
            (DNA::A, DNA::C) => None,
            (DNA::A, DNA::M) => Some(DNA::A),
            (DNA::A, DNA::G) => None,
            (DNA::A, DNA::R) => Some(DNA::A),
            (DNA::A, DNA::S) => None,
            (DNA::A, DNA::V) => Some(DNA::A),
            (DNA::A, DNA::T) => None,
            (DNA::A, DNA::W) => Some(DNA::A),
            (DNA::A, DNA::Y) => None,
            (DNA::A, DNA::H) => Some(DNA::A),
            (DNA::A, DNA::K) => None,
            (DNA::A, DNA::D) => Some(DNA::A),
            (DNA::A, DNA::B) => None,
            (DNA::A, DNA::N) => Some(DNA::A),
            (DNA::C, DNA::A) => None,
            (DNA::C, DNA::C) => Some(DNA::C),
            (DNA::C, DNA::M) => Some(DNA::C),
            (DNA::C, DNA::G) => None,
            (DNA::C, DNA::R) => None,
            (DNA::C, DNA::S) => Some(DNA::C),
            (DNA::C, DNA::V) => Some(DNA::C),
            (DNA::C, DNA::T) => None,
            (DNA::C, DNA::W) => None,
            (DNA::C, DNA::Y) => Some(DNA::C),
            (DNA::C, DNA::H) => Some(DNA::C),
            (DNA::C, DNA::K) => None,
            (DNA::C, DNA::D) => None,
            (DNA::C, DNA::B) => Some(DNA::C),
            (DNA::C, DNA::N) => Some(DNA::C),
            (DNA::M, DNA::A) => Some(DNA::A),
            (DNA::M, DNA::C) => Some(DNA::C),
            (DNA::M, DNA::M) => Some(DNA::M),
            (DNA::M, DNA::G) => None,
            (DNA::M, DNA::R) => Some(DNA::A),
            (DNA::M, DNA::S) => Some(DNA::C),
            (DNA::M, DNA::V) => Some(DNA::M),
            (DNA::M, DNA::T) => None,
            (DNA::M, DNA::W) => Some(DNA::A),
            (DNA::M, DNA::Y) => Some(DNA::C),
            (DNA::M, DNA::H) => Some(DNA::M),
            (DNA::M, DNA::K) => None,
            (DNA::M, DNA::D) => Some(DNA::A),
            (DNA::M, DNA::B) => Some(DNA::C),
            (DNA::M, DNA::N) => Some(DNA::M),
            (DNA::G, DNA::A) => None,
            (DNA::G, DNA::C) => None,
            (DNA::G, DNA::M) => None,
            (DNA::G, DNA::G) => Some(DNA::G),
            (DNA::G, DNA::R) => Some(DNA::G),
            (DNA::G, DNA::S) => Some(DNA::G),
            (DNA::G, DNA::V) => Some(DNA::G),
            (DNA::G, DNA::T) => None,
            (DNA::G, DNA::W) => None,
            (DNA::G, DNA::Y) => None,
            (DNA::G, DNA::H) => None,
            (DNA::G, DNA::K) => Some(DNA::G),
            (DNA::G, DNA::D) => Some(DNA::G),
            (DNA::G, DNA::B) => Some(DNA::G),
            (DNA::G, DNA::N) => Some(DNA::G),
            (DNA::R, DNA::A) => Some(DNA::A),
            (DNA::R, DNA::C) => None,
            (DNA::R, DNA::M) => Some(DNA::A),
            (DNA::R, DNA::G) => Some(DNA::G),
            (DNA::R, DNA::R) => Some(DNA::R),
            (DNA::R, DNA::S) => Some(DNA::G),
            (DNA::R, DNA::V) => Some(DNA::R),
            (DNA::R, DNA::T) => None,
            (DNA::R, DNA::W) => Some(DNA::A),
            (DNA::R, DNA::Y) => None,
            (DNA::R, DNA::H) => Some(DNA::A),
            (DNA::R, DNA::K) => Some(DNA::G),
            (DNA::R, DNA::D) => Some(DNA::R),
            (DNA::R, DNA::B) => Some(DNA::G),
            (DNA::R, DNA::N) => Some(DNA::R),
            (DNA::S, DNA::A) => None,
            (DNA::S, DNA::C) => Some(DNA::C),
            (DNA::S, DNA::M) => Some(DNA::C),
            (DNA::S, DNA::G) => Some(DNA::G),
            (DNA::S, DNA::R) => Some(DNA::G),
            (DNA::S, DNA::S) => Some(DNA::S),
            (DNA::S, DNA::V) => Some(DNA::S),
            (DNA::S, DNA::T) => None,
            (DNA::S, DNA::W) => None,
            (DNA::S, DNA::Y) => Some(DNA::C),
            (DNA::S, DNA::H) => Some(DNA::C),
            (DNA::S, DNA::K) => Some(DNA::G),
            (DNA::S, DNA::D) => Some(DNA::G),
            (DNA::S, DNA::B) => Some(DNA::S),
            (DNA::S, DNA::N) => Some(DNA::S),
            (DNA::V, DNA::A) => Some(DNA::A),
            (DNA::V, DNA::C) => Some(DNA::C),
            (DNA::V, DNA::M) => Some(DNA::M),
            (DNA::V, DNA::G) => Some(DNA::G),
            (DNA::V, DNA::R) => Some(DNA::R),
            (DNA::V, DNA::S) => Some(DNA::S),
            (DNA::V, DNA::V) => Some(DNA::V),
            (DNA::V, DNA::T) => None,
            (DNA::V, DNA::W) => Some(DNA::A),
            (DNA::V, DNA::Y) => Some(DNA::C),
            (DNA::V, DNA::H) => Some(DNA::M),
            (DNA::V, DNA::K) => Some(DNA::G),
            (DNA::V, DNA::D) => Some(DNA::R),
            (DNA::V, DNA::B) => Some(DNA::S),
            (DNA::V, DNA::N) => Some(DNA::V),
            (DNA::T, DNA::A) => None,
            (DNA::T, DNA::C) => None,
            (DNA::T, DNA::M) => None,
            (DNA::T, DNA::G) => None,
            (DNA::T, DNA::R) => None,
            (DNA::T, DNA::S) => None,
            (DNA::T, DNA::V) => None,
            (DNA::T, DNA::T) => Some(DNA::T),
            (DNA::T, DNA::W) => Some(DNA::T),
            (DNA::T, DNA::Y) => Some(DNA::T),
            (DNA::T, DNA::H) => Some(DNA::T),
            (DNA::T, DNA::K) => Some(DNA::T),
            (DNA::T, DNA::D) => Some(DNA::T),
            (DNA::T, DNA::B) => Some(DNA::T),
            (DNA::T, DNA::N) => Some(DNA::T),
            (DNA::W, DNA::A) => Some(DNA::A),
            (DNA::W, DNA::C) => None,
            (DNA::W, DNA::M) => Some(DNA::A),
            (DNA::W, DNA::G) => None,
            (DNA::W, DNA::R) => Some(DNA::A),
            (DNA::W, DNA::S) => None,
            (DNA::W, DNA::V) => Some(DNA::A),
            (DNA::W, DNA::T) => Some(DNA::T),
            (DNA::W, DNA::W) => Some(DNA::W),
            (DNA::W, DNA::Y) => Some(DNA::T),
            (DNA::W, DNA::H) => Some(DNA::W),
            (DNA::W, DNA::K) => Some(DNA::T),
            (DNA::W, DNA::D) => Some(DNA::W),
            (DNA::W, DNA::B) => Some(DNA::T),
            (DNA::W, DNA::N) => Some(DNA::W),
            (DNA::Y, DNA::A) => None,
            (DNA::Y, DNA::C) => Some(DNA::C),
            (DNA::Y, DNA::M) => Some(DNA::C),
            (DNA::Y, DNA::G) => None,
            (DNA::Y, DNA::R) => None,
            (DNA::Y, DNA::S) => Some(DNA::C),
            (DNA::Y, DNA::V) => Some(DNA::C),
            (DNA::Y, DNA::T) => Some(DNA::T),
            (DNA::Y, DNA::W) => Some(DNA::T),
            (DNA::Y, DNA::Y) => Some(DNA::Y),
            (DNA::Y, DNA::H) => Some(DNA::Y),
            (DNA::Y, DNA::K) => Some(DNA::T),
            (DNA::Y, DNA::D) => Some(DNA::T),
            (DNA::Y, DNA::B) => Some(DNA::Y),
            (DNA::Y, DNA::N) => Some(DNA::Y),
            (DNA::H, DNA::A) => Some(DNA::A),
            (DNA::H, DNA::C) => Some(DNA::C),
            (DNA::H, DNA::M) => Some(DNA::M),
            (DNA::H, DNA::G) => None,
            (DNA::H, DNA::R) => Some(DNA::A),
            (DNA::H, DNA::S) => Some(DNA::C),
            (DNA::H, DNA::V) => Some(DNA::M),
            (DNA::H, DNA::T) => Some(DNA::T),
            (DNA::H, DNA::W) => Some(DNA::W),
            (DNA::H, DNA::Y) => Some(DNA::Y),
            (DNA::H, DNA::H) => Some(DNA::H),
            (DNA::H, DNA::K) => Some(DNA::T),
            (DNA::H, DNA::D) => Some(DNA::W),
            (DNA::H, DNA::B) => Some(DNA::Y),
            (DNA::H, DNA::N) => Some(DNA::H),
            (DNA::K, DNA::A) => None,
            (DNA::K, DNA::C) => None,
            (DNA::K, DNA::M) => None,
            (DNA::K, DNA::G) => Some(DNA::G),
            (DNA::K, DNA::R) => Some(DNA::G),
            (DNA::K, DNA::S) => Some(DNA::G),
            (DNA::K, DNA::V) => Some(DNA::G),
            (DNA::K, DNA::T) => Some(DNA::T),
            (DNA::K, DNA::W) => Some(DNA::T),
            (DNA::K, DNA::Y) => Some(DNA::T),
            (DNA::K, DNA::H) => Some(DNA::T),
            (DNA::K, DNA::K) => Some(DNA::K),
            (DNA::K, DNA::D) => Some(DNA::K),
            (DNA::K, DNA::B) => Some(DNA::K),
            (DNA::K, DNA::N) => Some(DNA::K),
            (DNA::D, DNA::A) => Some(DNA::A),
            (DNA::D, DNA::C) => None,
            (DNA::D, DNA::M) => Some(DNA::A),
            (DNA::D, DNA::G) => Some(DNA::G),
            (DNA::D, DNA::R) => Some(DNA::R),
            (DNA::D, DNA::S) => Some(DNA::G),
            (DNA::D, DNA::V) => Some(DNA::R),
            (DNA::D, DNA::T) => Some(DNA::T),
            (DNA::D, DNA::W) => Some(DNA::W),
            (DNA::D, DNA::Y) => Some(DNA::T),
            (DNA::D, DNA::H) => Some(DNA::W),
            (DNA::D, DNA::K) => Some(DNA::K),
            (DNA::D, DNA::D) => Some(DNA::D),
            (DNA::D, DNA::B) => Some(DNA::K),
            (DNA::D, DNA::N) => Some(DNA::D),
            (DNA::B, DNA::A) => None,
            (DNA::B, DNA::C) => Some(DNA::C),
            (DNA::B, DNA::M) => Some(DNA::C),
            (DNA::B, DNA::G) => Some(DNA::G),
            (DNA::B, DNA::R) => Some(DNA::G),
            (DNA::B, DNA::S) => Some(DNA::S),
            (DNA::B, DNA::V) => Some(DNA::S),
            (DNA::B, DNA::T) => Some(DNA::T),
            (DNA::B, DNA::W) => Some(DNA::T),
            (DNA::B, DNA::Y) => Some(DNA::Y),
            (DNA::B, DNA::H) => Some(DNA::Y),
            (DNA::B, DNA::K) => Some(DNA::K),
            (DNA::B, DNA::D) => Some(DNA::K),
            (DNA::B, DNA::B) => Some(DNA::B),
            (DNA::B, DNA::N) => Some(DNA::B),
            (DNA::N, &base  ) => Some(*base),
        }
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        // This implementation can be rewritten to use bit operations.
        // Currently favoring simpler approaches to see what the compiler does.
        match (&self, &other) {
            (DNA::A, DNA::A) => None,
            (DNA::A, DNA::C) => Some(DNA::A),
            (DNA::A, DNA::M) => None,
            (DNA::A, DNA::G) => Some(DNA::A),
            (DNA::A, DNA::R) => None,
            (DNA::A, DNA::S) => Some(DNA::A),
            (DNA::A, DNA::V) => None,
            (DNA::A, DNA::T) => Some(DNA::A),
            (DNA::A, DNA::W) => None,
            (DNA::A, DNA::Y) => Some(DNA::A),
            (DNA::A, DNA::H) => None,
            (DNA::A, DNA::K) => Some(DNA::A),
            (DNA::A, DNA::D) => None,
            (DNA::A, DNA::B) => Some(DNA::A),
            (DNA::A, DNA::N) => None,
            (DNA::C, DNA::A) => Some(DNA::C),
            (DNA::C, DNA::C) => None,
            (DNA::C, DNA::M) => None,
            (DNA::C, DNA::G) => Some(DNA::C),
            (DNA::C, DNA::R) => Some(DNA::C),
            (DNA::C, DNA::S) => None,
            (DNA::C, DNA::V) => None,
            (DNA::C, DNA::T) => Some(DNA::C),
            (DNA::C, DNA::W) => Some(DNA::C),
            (DNA::C, DNA::Y) => None,
            (DNA::C, DNA::H) => None,
            (DNA::C, DNA::K) => Some(DNA::C),
            (DNA::C, DNA::D) => Some(DNA::C),
            (DNA::C, DNA::B) => None,
            (DNA::C, DNA::N) => None,
            (DNA::M, DNA::A) => Some(DNA::C),
            (DNA::M, DNA::C) => Some(DNA::A),
            (DNA::M, DNA::M) => None,
            (DNA::M, DNA::G) => Some(DNA::M),
            (DNA::M, DNA::R) => Some(DNA::C),
            (DNA::M, DNA::S) => Some(DNA::A),
            (DNA::M, DNA::V) => None,
            (DNA::M, DNA::T) => Some(DNA::M),
            (DNA::M, DNA::W) => Some(DNA::C),
            (DNA::M, DNA::Y) => Some(DNA::A),
            (DNA::M, DNA::H) => None,
            (DNA::M, DNA::K) => Some(DNA::M),
            (DNA::M, DNA::D) => Some(DNA::C),
            (DNA::M, DNA::B) => Some(DNA::A),
            (DNA::M, DNA::N) => None,
            (DNA::G, DNA::A) => Some(DNA::G),
            (DNA::G, DNA::C) => Some(DNA::G),
            (DNA::G, DNA::M) => Some(DNA::G),
            (DNA::G, DNA::G) => None,
            (DNA::G, DNA::R) => None,
            (DNA::G, DNA::S) => None,
            (DNA::G, DNA::V) => None,
            (DNA::G, DNA::T) => Some(DNA::G),
            (DNA::G, DNA::W) => Some(DNA::G),
            (DNA::G, DNA::Y) => Some(DNA::G),
            (DNA::G, DNA::H) => Some(DNA::G),
            (DNA::G, DNA::K) => None,
            (DNA::G, DNA::D) => None,
            (DNA::G, DNA::B) => None,
            (DNA::G, DNA::N) => None,
            (DNA::R, DNA::A) => Some(DNA::G),
            (DNA::R, DNA::C) => Some(DNA::R),
            (DNA::R, DNA::M) => Some(DNA::G),
            (DNA::R, DNA::G) => Some(DNA::A),
            (DNA::R, DNA::R) => None,
            (DNA::R, DNA::S) => Some(DNA::A),
            (DNA::R, DNA::V) => None,
            (DNA::R, DNA::T) => Some(DNA::R),
            (DNA::R, DNA::W) => Some(DNA::G),
            (DNA::R, DNA::Y) => Some(DNA::R),
            (DNA::R, DNA::H) => Some(DNA::G),
            (DNA::R, DNA::K) => Some(DNA::A),
            (DNA::R, DNA::D) => None,
            (DNA::R, DNA::B) => Some(DNA::A),
            (DNA::R, DNA::N) => None,
            (DNA::S, DNA::A) => Some(DNA::S),
            (DNA::S, DNA::C) => Some(DNA::G),
            (DNA::S, DNA::M) => Some(DNA::G),
            (DNA::S, DNA::G) => Some(DNA::C),
            (DNA::S, DNA::R) => Some(DNA::C),
            (DNA::S, DNA::S) => None,
            (DNA::S, DNA::V) => None,
            (DNA::S, DNA::T) => Some(DNA::S),
            (DNA::S, DNA::W) => Some(DNA::S),
            (DNA::S, DNA::Y) => Some(DNA::G),
            (DNA::S, DNA::H) => Some(DNA::G),
            (DNA::S, DNA::K) => Some(DNA::C),
            (DNA::S, DNA::D) => Some(DNA::C),
            (DNA::S, DNA::B) => None,
            (DNA::S, DNA::N) => None,
            (DNA::V, DNA::A) => Some(DNA::S),
            (DNA::V, DNA::C) => Some(DNA::R),
            (DNA::V, DNA::M) => Some(DNA::G),
            (DNA::V, DNA::G) => Some(DNA::M),
            (DNA::V, DNA::R) => Some(DNA::C),
            (DNA::V, DNA::S) => Some(DNA::A),
            (DNA::V, DNA::V) => None,
            (DNA::V, DNA::T) => Some(DNA::V),
            (DNA::V, DNA::W) => Some(DNA::S),
            (DNA::V, DNA::Y) => Some(DNA::R),
            (DNA::V, DNA::H) => Some(DNA::G),
            (DNA::V, DNA::K) => Some(DNA::M),
            (DNA::V, DNA::D) => Some(DNA::C),
            (DNA::V, DNA::B) => Some(DNA::A),
            (DNA::V, DNA::N) => None,
            (DNA::T, DNA::A) => Some(DNA::T),
            (DNA::T, DNA::C) => Some(DNA::T),
            (DNA::T, DNA::M) => Some(DNA::T),
            (DNA::T, DNA::G) => Some(DNA::T),
            (DNA::T, DNA::R) => Some(DNA::T),
            (DNA::T, DNA::S) => Some(DNA::T),
            (DNA::T, DNA::V) => Some(DNA::T),
            (DNA::T, DNA::T) => None,
            (DNA::T, DNA::W) => None,
            (DNA::T, DNA::Y) => None,
            (DNA::T, DNA::H) => None,
            (DNA::T, DNA::K) => None,
            (DNA::T, DNA::D) => None,
            (DNA::T, DNA::B) => None,
            (DNA::T, DNA::N) => None,
            (DNA::W, DNA::A) => Some(DNA::T),
            (DNA::W, DNA::C) => Some(DNA::W),
            (DNA::W, DNA::M) => Some(DNA::T),
            (DNA::W, DNA::G) => Some(DNA::W),
            (DNA::W, DNA::R) => Some(DNA::T),
            (DNA::W, DNA::S) => Some(DNA::W),
            (DNA::W, DNA::V) => Some(DNA::T),
            (DNA::W, DNA::T) => Some(DNA::A),
            (DNA::W, DNA::W) => None,
            (DNA::W, DNA::Y) => Some(DNA::A),
            (DNA::W, DNA::H) => None,
            (DNA::W, DNA::K) => Some(DNA::A),
            (DNA::W, DNA::D) => None,
            (DNA::W, DNA::B) => Some(DNA::A),
            (DNA::W, DNA::N) => None,
            (DNA::Y, DNA::A) => Some(DNA::Y),
            (DNA::Y, DNA::C) => Some(DNA::T),
            (DNA::Y, DNA::M) => Some(DNA::T),
            (DNA::Y, DNA::G) => Some(DNA::Y),
            (DNA::Y, DNA::R) => Some(DNA::Y),
            (DNA::Y, DNA::S) => Some(DNA::T),
            (DNA::Y, DNA::V) => Some(DNA::T),
            (DNA::Y, DNA::T) => Some(DNA::C),
            (DNA::Y, DNA::W) => Some(DNA::C),
            (DNA::Y, DNA::Y) => None,
            (DNA::Y, DNA::H) => None,
            (DNA::Y, DNA::K) => Some(DNA::C),
            (DNA::Y, DNA::D) => Some(DNA::C),
            (DNA::Y, DNA::B) => None,
            (DNA::Y, DNA::N) => None,
            (DNA::H, DNA::A) => Some(DNA::Y),
            (DNA::H, DNA::C) => Some(DNA::W),
            (DNA::H, DNA::M) => Some(DNA::T),
            (DNA::H, DNA::G) => Some(DNA::H),
            (DNA::H, DNA::R) => Some(DNA::Y),
            (DNA::H, DNA::S) => Some(DNA::W),
            (DNA::H, DNA::V) => Some(DNA::T),
            (DNA::H, DNA::T) => Some(DNA::M),
            (DNA::H, DNA::W) => Some(DNA::C),
            (DNA::H, DNA::Y) => Some(DNA::A),
            (DNA::H, DNA::H) => None,
            (DNA::H, DNA::K) => Some(DNA::M),
            (DNA::H, DNA::D) => Some(DNA::C),
            (DNA::H, DNA::B) => Some(DNA::A),
            (DNA::H, DNA::N) => None,
            (DNA::K, DNA::A) => Some(DNA::K),
            (DNA::K, DNA::C) => Some(DNA::K),
            (DNA::K, DNA::M) => Some(DNA::K),
            (DNA::K, DNA::G) => Some(DNA::T),
            (DNA::K, DNA::R) => Some(DNA::T),
            (DNA::K, DNA::S) => Some(DNA::T),
            (DNA::K, DNA::V) => Some(DNA::T),
            (DNA::K, DNA::T) => Some(DNA::G),
            (DNA::K, DNA::W) => Some(DNA::G),
            (DNA::K, DNA::Y) => Some(DNA::G),
            (DNA::K, DNA::H) => Some(DNA::G),
            (DNA::K, DNA::K) => None,
            (DNA::K, DNA::D) => None,
            (DNA::K, DNA::B) => None,
            (DNA::K, DNA::N) => None,
            (DNA::D, DNA::A) => Some(DNA::K),
            (DNA::D, DNA::C) => Some(DNA::D),
            (DNA::D, DNA::M) => Some(DNA::K),
            (DNA::D, DNA::G) => Some(DNA::W),
            (DNA::D, DNA::R) => Some(DNA::T),
            (DNA::D, DNA::S) => Some(DNA::W),
            (DNA::D, DNA::V) => Some(DNA::T),
            (DNA::D, DNA::T) => Some(DNA::R),
            (DNA::D, DNA::W) => Some(DNA::G),
            (DNA::D, DNA::Y) => Some(DNA::R),
            (DNA::D, DNA::H) => Some(DNA::G),
            (DNA::D, DNA::K) => Some(DNA::A),
            (DNA::D, DNA::D) => None,
            (DNA::D, DNA::B) => Some(DNA::A),
            (DNA::D, DNA::N) => None,
            (DNA::B, DNA::A) => Some(DNA::B),
            (DNA::B, DNA::C) => Some(DNA::K),
            (DNA::B, DNA::M) => Some(DNA::K),
            (DNA::B, DNA::G) => Some(DNA::Y),
            (DNA::B, DNA::R) => Some(DNA::Y),
            (DNA::B, DNA::S) => Some(DNA::T),
            (DNA::B, DNA::V) => Some(DNA::T),
            (DNA::B, DNA::T) => Some(DNA::S),
            (DNA::B, DNA::W) => Some(DNA::S),
            (DNA::B, DNA::Y) => Some(DNA::G),
            (DNA::B, DNA::H) => Some(DNA::G),
            (DNA::B, DNA::K) => Some(DNA::C),
            (DNA::B, DNA::D) => Some(DNA::C),
            (DNA::B, DNA::B) => None,
            (DNA::B, DNA::N) => None,
            (DNA::N, DNA::A) => Some(DNA::B),
            (DNA::N, DNA::C) => Some(DNA::D),
            (DNA::N, DNA::M) => Some(DNA::K),
            (DNA::N, DNA::G) => Some(DNA::H),
            (DNA::N, DNA::R) => Some(DNA::Y),
            (DNA::N, DNA::S) => Some(DNA::W),
            (DNA::N, DNA::V) => Some(DNA::T),
            (DNA::N, DNA::T) => Some(DNA::V),
            (DNA::N, DNA::W) => Some(DNA::S),
            (DNA::N, DNA::Y) => Some(DNA::R),
            (DNA::N, DNA::H) => Some(DNA::G),
            (DNA::N, DNA::K) => Some(DNA::M),
            (DNA::N, DNA::D) => Some(DNA::C),
            (DNA::N, DNA::B) => Some(DNA::A),
            (DNA::N, DNA::N) => None,
        }
    }

    fn is_redundant(&self) -> bool {
        match self {
            DNA::A | DNA::C | DNA::G | DNA::T => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};
    use proptest::prelude::any;
    use proptest::sample::select;
    use ::ReverseComplement;

    #[test]
    fn test_complement_vec() {
        let d = vec![DNA::A, DNA::T, DNA::G, DNA::C];
        let c: Vec<DNA> = d.reverse_complement().rev().collect();
        assert_eq!(c, vec![DNA::T, DNA::A, DNA::C, DNA::G]);
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

    proptest!{
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
