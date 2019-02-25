//! Error types for the library.

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct SeqError {
    inner: Context<SeqErrorKind>,
}

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum SeqErrorKind {
    #[fail(display = "Encountered unknown character in alphabet {}", base)]
    AlphabetReadError { base: char },
    #[fail(display = "String must contain 3 characters to be parsed into codon.")]
    CodonFromStrTooShort,
    #[fail(display = "Cannot convert redundant character {} into non-redundant alphabet.", base)]
    RedundantAlphabetConversionError { base: char },
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
        SeqError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<SeqErrorKind>> for SeqError {
    fn from(inner: Context<SeqErrorKind>) -> SeqError {
        SeqError { inner: inner }
    }
}
