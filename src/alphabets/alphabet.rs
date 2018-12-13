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

        impl TryFrom<char> for $name {
            type Error = SeqError;

            fn try_from(base: char) -> Result<Self, Self::Error> {
                (&base).try_into()
            }
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

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(DNAnr::try_from('A').unwrap(),  DNAnr::A);
        assert_eq!(DNAnr::try_from('a').unwrap(),  DNAnr::A);
        assert_eq!(DNAnr::try_from('T').unwrap(),  DNAnr::T);
        assert_eq!(DNAnr::try_from(&'A').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from(&'a').unwrap(), DNAnr::A);
        assert_eq!(DNAnr::try_from(&'T').unwrap(), DNAnr::T);
        assert_eq!(DNAnr::try_from(&'c').unwrap(), DNAnr::C);
        assert_eq!(DNAnr::try_from(&'G').unwrap(), DNAnr::G);
    }

    #[test]
    fn test_mac() {
        assert_eq!(DNAnr::A, DNAnr::A);
        assert_eq!(DNAnr::T, DNAnr::T);
        assert_eq!(DNAnr::G, DNAnr::G);
        assert_eq!(DNAnr::C, DNAnr::C);
    }
}
