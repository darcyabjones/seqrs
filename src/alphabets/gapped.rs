/// Gapped alphabets...

use alphabets::Complement;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gapped<T> {
    Gap,
    Occ(T),
}


impl<T: TryFrom<u8>> TryFrom<u8> for Gapped<T> {
    type Error = T::Error;

    fn try_from(base: u8) -> Result<Self, Self::Error> {
        match base {
            b'-' => Ok(Gapped::Gap),
            a    => T::try_from(a).map(Gapped::Occ),
        }
    }
}


impl<T: Complement> Complement for Gapped<T> {
    fn complement(&self) -> Self {
        match self {
            Gapped::Gap => Gapped::Gap,
            Gapped::Occ(a) => Gapped::Occ(a.complement()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alphabets::DNA;

    #[test]
    fn test_from() {
        assert_eq!(Gapped::<DNA>::try_from(b'-').unwrap(), Gapped::Gap);
        assert_eq!(Gapped::<DNA>::try_from(b'A').unwrap(), Gapped::Occ(DNA::A));
        assert_eq!(Gapped::<DNA>::try_from(b'T').unwrap(), Gapped::Occ(DNA::T));
    }

    #[test]
    fn test_complement() {
        assert_eq!(Gapped::Gap::<DNA>.complement(), Gapped::Gap);
        assert_eq!(Gapped::Occ(DNA::T).complement(), Gapped::Occ(DNA::A));
    }
}

