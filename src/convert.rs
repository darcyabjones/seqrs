///
///

pub trait FromChar<T>: Sized {
    type CharError;

    fn from_char(c: T) -> Self;
}

pub trait 
