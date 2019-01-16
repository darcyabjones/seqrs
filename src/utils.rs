/// Small utility functions.
/// Yes yes I know that utils is a bad name. I couldn't think of anything.

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

