///
///
///

use errors::SeqError;

/// Safely casts char as byte, raising AlphabetReadError if overflow.
pub fn char_to_byte(c: &char) -> Result<u8, SeqError> {
    let int = *c as u32;

    if int <= (u8::max_value() as u32) {
        Ok(int as u8)
    } else {
        Err(SeqError::AlphabetReadError { base: *c })
    }
}


/// Creates a single letter biological alphabet.
///
#[macro_export]
macro_rules! alphabet {
    (   // Entry point handles non-public enum case.
        $(#[$($flag:tt)*])*
        enum $name:ident {
            $($variant:ident $(= {$($tail:tt)*})* );* $(;)*
        }
    ) => {
        alphabet!{
            $(#[$($flag)*])*
            ()
            @alph $name {
                $($variant: () = {$($($tail)*)*};)*
            }
        }
    };
    (   // Entry point handles public enum case.
        $(#[$($flag:tt)*])*
        pub enum $name:ident {
            $($variant:ident $(= {$($tail:tt)*})* );* $(;)*
        }
    ) => {
        alphabet!{
            $(#[$($flag)*])*
            (pub)
            @alph $name {
                $($variant: () = {$($($tail)*)*};)*
            }
        }
    };
    (   // Stop condition.
        // If there are no fields in the brackets, create the enum.
        // Optionally with bits as explicit enum variants.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {};)*
        }
    ) => {
        $(#[$($flag)*])*
        $($vis)* enum $name { $($variant $(= $bits)*),* }

        // Also impl some methods.
        impl $name {
            // Vec containing all of the enum variants.
            pub fn variants() -> Vec<Self> {
                vec![$($name::$variant),*]
            }

            // The number of different variants.
            pub fn cardinality() -> usize {
                Self::variants().len()
            }
        }
    };
    (
        // Move bits from record field to after enum variant.
        // This helps us keep it around to the end and handling cases where
        // it's not included.
        // Can get rid of requirement for trailing commas by using
        // $(, $($tail:tt)* )? when ? becomes available.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : () = {
                bits: $bits:expr,
                $($tail:tt)*
            };)*
        }
    ) => {
        alphabet!{
            $(#[$($flag)*])*
            ($($vis)*)
            @alph $name {
                $($variant: ($bits) = {$($tail)*};)*
            }
        }
    };
    (
        // Implement match to handle redundancy.
        // Can get rid of requirement for trailing commas by using
        // $(, $($tail:tt)* )? when ? becomes available.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                redundant: [$($red:ident),*],
                $($tail:tt)*
            };)*
        }
    ) => {
        impl Match<$name> for $name {
            fn matches(&self, other: &$name) -> bool {
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

        alphabet!{
            $(#[$($flag)*])*
            ($($vis)*)
            @alph $name {
                $($variant: ($($bits)*) = {$($tail)*};)*
            }
        }
    };
    (
        // Implement the Complement trait for the alphabet.
        // Can get rid of requirement for trailing commas by using
        // $(, $($tail:tt)* )? when ? becomes available.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                compl: $compl:ident,
                $($tail:tt)*
            };)*
        }
    ) => {
        impl Complement for $name {
            fn complement(&self) -> Self {
                match &self {
                    $($name::$variant => $name::$compl,)*
                }
            }
        }

        alphabet!{
            $(#[$($flag)*])*
            ($($vis)*)
            @alph $name {
                $($variant: ($($bits)*) = {$($tail)*};)*
            }
        }
    };
    (
        // Implement conversion traits to and from characters.
        // Can get rid of requirement for trailing commas by using
        // $(, $($tail:tt)* )? when ? becomes available.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                chr: $byte:expr,
                $($tail:tt)*
            };)*
        }
    ) => {
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

        alphabet!{
            $(#[$($flag)*])*
            ($($vis)*)
            @alph $name {
                $($variant : ($($bits)*) = {$($tail)*};)*
            }
        }
    };
    (
        // If we get an unsupported field in the variant structs, fail.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                $($unk:tt)+
            };)*
        }
    ) => {
        compile_error!("Encountered unsupported field in `alphabet!` enum variants.");
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::Match;
    use ::Complement;
    use std::convert::TryFrom;

    alphabet! {
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
        pub enum DNA {
            A = {chr: b'A', compl: T, redundant: [N],};
            T = {chr: b'T', compl: A, redundant: [N],};
            G = {chr: b'G', compl: C, redundant: [N],};
            C = {chr: b'C', compl: G, redundant: [N],};
            N = {chr: b'N', compl: N, redundant: [A, T, G, C],};
        }
    }


    #[test]
    fn test_match() {
        assert!(DNA::A.matches(&DNA::N));
        assert!(DNA::A.matches(&DNA::A));
        assert!(!DNA::A.matches(&DNA::C));
        assert!(DNA::A.doesnt_match(&DNA::C));
    }

    #[test]
    fn test_from() {
        assert_eq!(DNA::try_from(  'A').unwrap(), DNA::A);
        assert_eq!(DNA::try_from( &'A').unwrap(), DNA::A);
        assert_eq!(DNA::try_from( b'A').unwrap(), DNA::A);
        assert_eq!(DNA::try_from(&b'A').unwrap(), DNA::A);

        assert_eq!(DNA::try_from( 'a').unwrap(), DNA::A);
        assert_eq!(DNA::try_from( 'T').unwrap(), DNA::T);
        assert_eq!(DNA::try_from(&'A').unwrap(), DNA::A);
        assert_eq!(DNA::try_from(&'a').unwrap(), DNA::A);
        assert_eq!(DNA::try_from(&'T').unwrap(), DNA::T);
        assert_eq!(DNA::try_from(&'c').unwrap(), DNA::C);
        assert_eq!(DNA::try_from(&'G').unwrap(), DNA::G);

        assert_eq!(  u8::from( DNA::A), b'A');
        assert_eq!(  u8::from(&DNA::A), b'A');
        assert_eq!(char::from( DNA::A),  'A');
        assert_eq!(char::from(&DNA::A),  'A');
    }

    #[test]
    fn test_eq() {
        assert_eq!(DNA::A, DNA::A);
        assert_eq!(DNA::T, DNA::T);
        assert_eq!(DNA::G, DNA::G);
        assert_eq!(DNA::C, DNA::C);
    }
}
