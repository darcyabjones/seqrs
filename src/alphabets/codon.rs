/// A generalised codon alphabet.

use alphabets::Complement;
use std::convert::TryFrom;


/// Codons represented as tuple struct.
/// The tuple struct with public fields is used to make pattern matching
/// easier. I think it's a reasonable choice, given that these should
/// essentially be immutable and the order of codon elements has an
/// unambiguous interpretation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Codon<T>(pub T, pub T, pub T);


/// Codon constructor and accessors.
impl<T> Codon<T> {

    pub fn new(first: T, second: T, third: T) -> Self {
        Codon(first, second, third)
    }

    pub fn first(self) -> T {
        self.0
    }

    pub fn second(self) -> T {
        self.1
    }

    pub fn third(self) -> T {
        self.2
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alphabets::DNA;

    #[test]
    fn test_eq() {
        assert_eq!(Codon::new(DNA::A, DNA::T, DNA::G), Codon(DNA::A, DNA::T, DNA::G));
    }

    #[test]
    fn test_access() {
        let met = Codon(DNA::A, DNA::T, DNA::G);
        assert_eq!(met.0, DNA::A);
        assert_eq!(met.first(), DNA::A);
    }
}

