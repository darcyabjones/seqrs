///
///

use errors::SeqError;
use std::borrow::Borrow;

/// Safely casts char as byte, raising AlphabetReadError if overflow.
fn char_to_byte(c: &char) -> Result<u8, char> {
    let int = *c as u32;

    if int <= (u8::max_value() as u32) {
        Ok(int as u8)
    } else {
        Err(*c)
    }
}

pub trait FromChar<T>: Sized {
    fn from_char(c: T) -> Result<Self, SeqError>;
}

pub trait IntoChar<T>: Sized {
    fn into_char(&self) -> T;
}


// Blanket implementations

impl<T> FromChar<char> for T
    where T: FromChar<u8>
{
    fn from_char(c: char) -> Result<Self, SeqError> {
        char_to_byte(&c)
            .map_err(|c| SeqError::AlphabetReadError { base: c } )
            .and_then(Self::from_char)
    }
}

impl<T> FromChar<&char> for T
    where T: FromChar<u8>
{
    fn from_char(c: &char) -> Result<Self, SeqError> {
        char_to_byte(c)
            .map_err(|c| SeqError::AlphabetReadError { base: c } )
            .and_then(Self::from_char)
    }
}

impl<T> FromChar<&u8> for T
    where T: FromChar<u8>
{
    fn from_char(c: &u8) -> Result<Self, SeqError> {
        Self::from_char(*c)
    }
}

impl<T> IntoChar<char> for T
    where T: IntoChar<u8>,
{
    fn into_char(&self) -> char {
        let byte: u8 = self.into_char();
        byte as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
    enum DNA {A, T, G, C}

    impl FromChar<u8> for DNA {
        fn from_char(c: u8) -> Result<Self, SeqError> {
            match c {
                b'A' => Ok(DNA::A),
                b'T' => Ok(DNA::T),
                b'C' => Ok(DNA::C),
                b'G' => Ok(DNA::A),
                b => Err(SeqError::AlphabetReadError { base: b as char})
            }
        }
    }

    impl IntoChar<u8> for DNA {
        fn into_char(&self) -> u8 {
            match self {
                DNA::A => b'A',
                DNA::T => b'T',
                DNA::C => b'C',
                DNA::G => b'G',
            }
        }
    }

    #[test]
    fn test_from_char() {
        assert_eq!(DNA::from_char(b'A').unwrap(), DNA::A);
        assert_eq!(DNA::from_char('A').unwrap(), DNA::A);
        assert_eq!(DNA::from_char(&b'A').unwrap(), DNA::A);
        assert_eq!(DNA::from_char(&'A').unwrap(), DNA::A);
    }

    #[test]
    fn test_into_char() {
        assert_eq!(IntoChar::<u8>::into_char(&DNA::A), b'A');
        let x: char = DNA::A.into_char();
        assert_eq!(x, 'A');
        let x: u8 = (&DNA::A).into_char();
        assert_eq!(x, b'A');
    }
}
