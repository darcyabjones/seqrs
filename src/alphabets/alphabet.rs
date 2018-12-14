///
///
///

use errors::SeqError;
use alphabets::Complement;
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};

/// Safely casts char as byte, raising AlphabetReadError if overflow.
fn char_to_byte(c: &char) -> Result<u8, SeqError> {
    let int = *c as u32;

    if int <= (u8::max_value() as u32) {
        Ok(int as u8)
    } else {
        Err(SeqError::AlphabetReadError { base: *c })
    }
}

#[macro_export]
macro_rules! nucl_alphabet {
    (
        enum $name:ident {
            $( $variant:ident = {bits: $bits:expr, byte: $byte:expr, compl: $compl:ident, redundant: [$($red:ident),*]}; )+
        }
    ) => {
        #[repr(u8)]
        #[derive(Debug, Hash, Copy, Clone)]
        pub enum $name {
            $($variant = $bits),*
        }

        impl TryFrom<&u8> for $name {
            type Error = SeqError;

            fn try_from(base: &u8) -> Result<Self, Self::Error> {
                match base.to_ascii_uppercase() {
                    $($byte => Ok($name::$variant),)*
                    b => Err(SeqError::AlphabetReadError { base: b as char }),
                }
            }
        }

        impl TryFrom<u8> for $name {
            type Error = SeqError;

            fn try_from(base: u8) -> Result<Self, Self::Error> {
                Self::try_from(&(base))
            }
        }

        impl TryFrom<&char> for $name {
            type Error = SeqError;

            fn try_from(base: &char) -> Result<Self, Self::Error> {
                char_to_byte(base).and_then(|b| Self::try_from(&b))
            }
        }

        impl TryFrom<char> for $name {
            type Error = SeqError;

            fn try_from(base: char) -> Result<Self, Self::Error> {
                char_to_byte(&base).and_then(|b| Self::try_from(&b))
            }
        }

        impl From<&$name> for u8 {
            fn from(base: &$name) -> Self {
                match base {
                    $($name::$variant => $byte,)*
                }
            }
        }

        impl From<$name> for u8 {
            fn from(base: $name) -> Self {
                (&base).into()
            }
        }

        impl From<&$name> for char {
            fn from(base: &$name) -> Self {
                u8::from(base) as char
            }
        }

        impl From<$name> for char {
            fn from(base: $name) -> Self {
                u8::from(&base) as char
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        ($name::$variant, $name::$variant) => true,
                        $(
                            ($name::$variant, $name::$red) => true,
                        )*
                    )*
                    _ => false
                }
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                (*self as u8).cmp(&(*other as u8))
            }
        }

        impl Complement for $name {
            fn complement(&self) -> Self {
                match self {
                    $(
                        $name::$variant => $name::$compl,
                    )*
                }
            }
        }
    }
}

nucl_alphabet!{
    enum DNAnr {
        A = {bits: 0b00, byte: b'A', compl: T, redundant: []};
        T = {bits: 0b01, byte: b'T', compl: A, redundant: []};
        G = {bits: 0b10, byte: b'G', compl: C, redundant: []};
        C = {bits: 0b11, byte: b'C', compl: G, redundant: []};
    }
}

nucl_alphabet!{
    enum DNA {
        A = {bits: 0b0001, byte: b'A', compl: T, redundant: [M, R, V, W, H, D, N]};
        C = {bits: 0b0010, byte: b'C', compl: G, redundant: [M, S, V, Y, H, B, N]};
        M = {bits: 0b0011, byte: b'M', compl: K, redundant: [A, C, R, S, V, W, Y, H, D, B, N]};
        G = {bits: 0b0100, byte: b'G', compl: C, redundant: [R, S, V, K, D, B, N]};
        R = {bits: 0b0101, byte: b'R', compl: Y, redundant: [A, M, G, S, V, W, H, K, D, B, N]};
        S = {bits: 0b0110, byte: b'S', compl: S, redundant: [C, M, G, R, V, Y, H, K, D, B, N]};
        V = {bits: 0b0111, byte: b'V', compl: B, redundant: [A, C, M, G, R, S, W, Y, H, K, D, B, N]};
        T = {bits: 0b1000, byte: b'T', compl: A, redundant: [W, Y, H, K, D, B, N]};
        W = {bits: 0b1001, byte: b'W', compl: W, redundant: [A, M, R, V, T, Y, H, K, D, B, N]};
        Y = {bits: 0b1010, byte: b'Y', compl: R, redundant: [C, M, S, V, T, W, H, K, D, B, N]};
        H = {bits: 0b1011, byte: b'H', compl: D, redundant: [A, C, M, R, S, V, T, W, Y, K, D, B, N]};
        K = {bits: 0b1100, byte: b'K', compl: M, redundant: [G, R, S, V, T, W, Y, H, D, B, N]};
        D = {bits: 0b1101, byte: b'D', compl: H, redundant: [A, M, G, R, S, V, T, W, Y, H, K, B, N]};
        B = {bits: 0b1110, byte: b'B', compl: V, redundant: [C, M, G, R, S, V, T, W, Y, H, K, D, N]};
        N = {bits: 0b1111, byte: b'N', compl: N, redundant: [A, C, M, G, R, S, V, T, W, Y, H, K, D, B]};
    }
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(DNAnr::try_from(  'A').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from( &'A').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from( b'A').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from(&b'A').unwrap(), DNAnr::A);

        assert_eq!(DNAnr::try_from( 'a').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from( 'T').unwrap(), DNAnr::T);
        assert_eq!(DNAnr::try_from(&'A').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from(&'a').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from(&'T').unwrap(), DNAnr::T);
        assert_eq!(DNAnr::try_from(&'c').unwrap(), DNAnr::C);
        assert_eq!(DNAnr::try_from(&'G').unwrap(), DNAnr::G);

        assert_eq!(  u8::from( DNAnr::A), b'A');
        assert_eq!(  u8::from(&DNAnr::A), b'A');
        assert_eq!(char::from( DNAnr::A),  'A');
        assert_eq!(char::from(&DNAnr::A),  'A');
    }

    #[test]
    fn test_eq() {
        assert_eq!(DNAnr::A, DNAnr::A);
        assert_eq!(DNAnr::T, DNAnr::T);
        assert_eq!(DNAnr::G, DNAnr::G);
        assert_eq!(DNAnr::C, DNAnr::C);
    }
}
