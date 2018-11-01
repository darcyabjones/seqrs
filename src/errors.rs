//! Error types for the library.


#[derive(Debug, Fail)]
pub enum SeqError {
    #[fail(display = "Encountered unknown character in alphabet {}", base)]
    AlphabetReadError { base: char }
}
