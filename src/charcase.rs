//! Some simple functions for converting ASCII character cases.


pub fn is_lower(c: u8) -> bool {
    c > 96 && c < 123
}

pub fn is_upper(c: u8) -> bool {
    c > 64 && c < 91
}

pub fn to_upper(c: u8) -> u8 {
    if is_lower(c) {
        c - 32
    } else {
        c
    }
}

pub fn to_lower(c: u8) -> u8 {
    if is_upper(c) {
        c + 32
    } else {
        c
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lower() {
        assert!(is_lower(b'a'));
        assert!(is_lower(b'z'));
        assert!(is_lower(b'j'));
        assert!(!is_lower(b'A'));
        assert!(!is_lower(b'J'));
        assert!(!is_lower(b'Z'));
    }

    #[test]
    fn test_is_upper() {
        assert!(!is_upper(b'a'));
        assert!(!is_upper(b'z'));
        assert!(!is_upper(b'j'));
        assert!(is_upper(b'A'));
        assert!(is_upper(b'J'));
        assert!(is_upper(b'Z'));
    }

    #[test]
    fn test_to_upper() {
        assert_eq!(to_upper(b'a'), b'A');
        assert_eq!(to_upper(b'A'), b'A');
        assert_eq!(to_upper(b'z'), b'Z');
        assert_eq!(to_upper(b'Z'), b'Z');
        assert_eq!(to_upper(b'j'), b'J');
        assert_eq!(to_upper(b'J'), b'J');
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(to_lower(b'a'), b'a');
        assert_eq!(to_lower(b'A'), b'a');
        assert_eq!(to_lower(b'z'), b'z');
        assert_eq!(to_lower(b'Z'), b'z');
        assert_eq!(to_lower(b'j'), b'j');
        assert_eq!(to_lower(b'J'), b'j');
    }
}
