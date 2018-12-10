//! Translate trait.
//!

pub trait Translate<T> {
    fn translate(&self) -> T;
}
