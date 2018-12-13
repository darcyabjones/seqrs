///
///
///

use errors::SeqError;
use alphabets::Complement;
use std::convert::{TryFrom, TryInto};

#[macro_export]
macro_rules! nucl_alphabet {
    (
        enum $name:ident {
            $( $variant:ident => {bits: $b:expr, byte: $c:expr, compl: $compl:ident}; )+
        }
    ) => {
        #[repr(u8)]
        #[derive(Debug, Hash, Copy, Clone)]
        pub enum $name {
            $($variant = $b),*
        }

        impl TryFrom<&char> for $name {
            type Error = SeqError;

            fn try_from(base: &char) -> Result<Self, Self::Error> {
                match base.to_ascii_uppercase() {
                    $($c => Ok($name::$variant),)*
                    b => Err(SeqError::AlphabetReadError { base: b }),
                }
            }
        }

        impl TryFrom<char> for $name {
            type Error = SeqError;

            fn try_from(base: char) -> Result<Self, Self::Error> {
                (&base).try_into()
            }
        }

        impl TryFrom<&u8> for $name {
            type Error = SeqError;

            fn try_from(base: &u8) -> Result<Self, Self::Error> {
                Self::try_from(&(*base as char))
            }
        }

        impl TryFrom<u8> for $name {
            type Error = SeqError;

            fn try_from(base: u8) -> Result<Self, Self::Error> {
                Self::try_from(&(base as char))
            }
        }

        impl From<&$name> for char {
            fn from(base: &$name) -> Self {
                match base {
                    $($name::$variant => $c,)*
                }
            }
        }

        impl From<$name> for char {
            fn from(base: $name) -> Self {
                (&base).into()
            }
        }

        impl From<$name> for u8 {
            fn from(base: $name) -> Self {
                (char::from(&base) as u32) as u8
            }
        }

        impl From<&$name> for u8 {
            fn from(base: &$name) -> Self {
                (char::from(base) as u32) as u8
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(($name::$variant, $name::$variant) => true,)*
                    _ => false
                }
            }
        }

        impl Eq for $name {}

        impl Complement for $name {
            fn complement(&self) -> Self {
                match self {
                    $($name::$variant => $name::$compl,)*
                }
            }
        }
    }
}

nucl_alphabet!{
    enum DNAnr {
        A => {bits: 0b00, byte: 'A', compl: T};
        T => {bits: 0b01, byte: 'T', compl: A};
        G => {bits: 0b10, byte: 'G', compl: C};
        C => {bits: 0b11, byte: 'C', compl: G};
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DNA {
    A = b'A',
    T = b'T',
    G = b'G',
    C = b'C',
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    #[test]
    fn test_alpha() {
        assert_eq!(DNA::A as u8, b'A');
        assert_eq!(DNA::from(b'A'), DNA::A);
    }

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
