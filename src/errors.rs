//! Error types for the library.

use std::fmt;
use std::fmt::Display;
use failure::{Context, Fail, Backtrace};

#[derive(Debug)]
pub struct SeqError {
    inner: Context<SeqErrorKind>
}

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum SeqErrorKind {
    #[fail(display = "Encountered unknown character in alphabet {}", base)]
    AlphabetReadError { base: char },
    #[fail(display = "Encountered iterator with size {} for codon, which should have size 3.", n)]
    CodonLengthError { n: usize },
    #[fail(display = "A proper error type for auto try_into impls.")]
    Dummy,
}


impl Fail for SeqError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for SeqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl SeqError {
    pub fn kind(&self) -> &SeqErrorKind {
        self.inner.get_context()
    }
}

impl From<SeqErrorKind> for SeqError {
    fn from(kind: SeqErrorKind) -> SeqError {
        SeqError { inner: Context::new(kind) }
    }
}

impl From<Context<SeqErrorKind>> for SeqError {
    fn from(inner: Context<SeqErrorKind>) -> SeqError {
        SeqError { inner: inner }
    }
}
