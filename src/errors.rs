//! Error types for the library.


#[derive(Debug, Fail)]
pub enum SeqError {
    #[fail(display = "Encountered unknown character in alphabet {}", base)]
    AlphabetReadError { base: char },
    #[fail(display = "Encountered iterator with size {} for codon, which should have size 3.", n)]
    CodonLengthError { n: usize },
}
