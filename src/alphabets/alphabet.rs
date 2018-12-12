///
///
///

use errors::SeqError;
use alphabets::Complement;
use std::convert::{TryFrom, TryInto};

#[macro_export]
macro_rules! alphabet {
    (
        enum $name:ident {
            $( $variant:ident => {char: $c:expr, str: $s:expr, compl: $compl:ident}; )+
        }
    ) => {
        #[derive(Debug, Hash, Copy, Clone)]
        pub enum $name {
            $($variant),*
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

alphabet!{
    enum DNAnr {
        A => {char: 'A', str: "A", compl: T};
        T => {char: 'T', str: "T", compl: A};
        G => {char: 'G', str: "G", compl: C};
        C => {char: 'C', str: "C", compl: G};
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
