///
///
///

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

        impl From<&$name> for $name {
            fn from(b: &Self) -> Self {
                *b
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
        // is this a valid IUPAC character?
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                is_iupac: $isit:expr,
                $($tail:tt)*
            };)*
        }
    ) => {
        impl $name {
            pub fn is_iupac(&self) -> bool {
                match &self {
                    $($name::$variant => $isit,)*
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
        // More descriptive name
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                name: $basename:expr,
                $($tail:tt)*
            };)*
        }
    ) => {
        impl $name {
            pub fn name(&self) -> String {
                match &self {
                    $($name::$variant => String::from($basename),)*
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
        // Implement match to handle redundancy.
        // Can get rid of requirement for trailing commas by using
        // $(, $($tail:tt)* )? when ? becomes available.
        $(#[$($flag:tt)*])*
        ($($vis:tt)*)
        @alph $name:ident {
            $($variant:ident : ($($bits:expr)*) = {
                matches: [$($red:ident),*],
                $($tail:tt)*
            };)*
        }
    ) => {
        impl Match<$name> for $name {
            fn matches(&self, other: &$name) -> bool {
                match (&self, &other) {
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
        impl Complement for &$name {
            type Compl = $name;
            fn complement(self) -> Self::Compl {
                match self {
                    $($name::$variant => $name::$compl,)*
                }
            }
        }

        impl Complement for $name {
            type Compl = $name;
            fn complement(self) -> Self::Compl {
                (&self).complement()
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
                    b => Err(SeqErrorKind::AlphabetReadError { base: b as char }.into()),
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
                $crate::utils::char_to_byte(base).and_then(|b| Self::try_from(&b))
            }
        }

        impl TryFrom<char> for $name {
            type Error = SeqError;

            fn try_from(base: char) -> Result<Self, Self::Error> {
                $crate::utils::char_to_byte(&base).and_then(|b| Self::try_from(&b))
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

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", char::from(self))
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

/*
#[cfg(test)]
mod tests {
    use matcher::Match;
    use complement::Complement;
    use errors::SeqError;
    use errors::SeqErrorKind;
    use std::convert::TryFrom;

    alphabet! {
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
        pub enum DNA {
            A = {chr: b'A', compl: T, matches: [N],};
            T = {chr: b'T', compl: A, matches: [N],};
            G = {chr: b'G', compl: C, matches: [N],};
            C = {chr: b'C', compl: G, matches: [N],};
            N = {chr: b'N', compl: N, matches: [A, T, G, C],};
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
}*/
