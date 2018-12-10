//! Generalised gapped alphabets.
//!
//! Gapped alphabets extend regular alphabets with a sum type and wrapping
//! the base in a struct.

use alphabets::Complement;
use alphabets::Translate;
use std::convert::TryFrom;


/// A gapped alphabet combines any type with a new enum.
/// `Occ` for occupied.
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Gapped<T> {
    Gap,
    Base(T),
}


impl<T> Gapped<T> {

    /// Returns true if is Base option.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Base('a');
    /// assert_eq!(x.is_base(), true);
    ///
    /// let x: Gapped<char> = Gapped::Gap;
    /// assert_eq!(x.is_base(), false);
    /// ```
    #[inline]
    pub fn is_base(&self) -> bool {
        match *self {
            Gapped::Base(_) => true,
            Gapped::Gap => false,
        }
    }

    /// Returns true if is Gap option.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    /// let x: Gapped<char> = Gapped::Base('a');
    /// assert_eq!(x.is_gap(), false);
    ///
    /// let x: Gapped<char> = Gapped::Gap;
    /// assert_eq!(x.is_gap(), true);
    /// ```
    #[inline]
    pub fn is_gap(&self) -> bool {
        !self.is_base()
    }

    /// Converts from `Gapped<T>` to `Gapped<&T>`.
    ///
    /// Based on impl of Option in stdlib.
    /// From https://doc.rust-lang.org/src/core/option.rs.html#223-250 :
    /// Convert an `Option<`[`String`]`>` into an `Option<`[`usize`]`>`, preserving the original.
    /// The [`map`] method takes the `self` argument by value, consuming the original,
    /// so this technique uses `as_ref` to first take an `Option` to a reference
    /// to the value inside the original.
    ///
    /// [`map`]: enum.Option.html#method.map
    /// [`String`]: ../../std/string/struct.String.html
    /// [`usize`]: ../../std/primitive.usize.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    /// let base: Gapped<char> = Gapped::Base('a');
    /// // First, cast `Gapped<char>` to `Gapped<&char>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `text` on the stack.
    /// let upper: Gapped<char> = base.as_ref().map(|s| s.to_ascii_uppercase());
    /// assert_eq!(upper, Gapped::Base('A'));
    /// println!("still can print: {:?} into {:?}", base, upper);
    /// ```
    #[inline]
    pub fn as_ref(&self) -> Gapped<&T> {
        match *self {
            Gapped::Base(ref x) => Gapped::Base(x),
            Gapped::Gap => Gapped::Gap,
        }
    }

    /// Converts from `Gapped<T>` to `Gapped<&mut T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let mut x = Gapped::Base('a');
    /// match x.as_mut() {
    ///     Gapped::Base(v) => *v = 't',
    ///     Gapped::Gap => {},
    /// }
    /// assert_eq!(x, Gapped::Base('t'));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Gapped<&mut T> {
        match *self {
            Gapped::Base(ref mut x) => Gapped::Base(x),
            Gapped::Gap => Gapped::Gap,
        }
    }

    /// Unwraps a Gapped, yielding the content of a [`Base`].
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Gap`] with a custom panic message provided by
    /// `msg`.
    ///
    /// [`Base`]: #variant.Base
    /// [`Gap`]: #variant.Gap
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Base('a');
    /// assert_eq!(x.expect("the world is ending"), 'a');
    /// ```
    ///
    /// ```{.should_panic}
    /// use seqrs::alphabets::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Gap;
    /// x.expect("the world is ending"); // panics with `the world is ending`
    /// ```
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Gapped::Base(val) => val,
            Gapped::Gap => panic!("{}", msg),
        }
    }

    /// Moves the value `v` out of the `Gapped<T>` if it is [`Base(v)`].
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Gap`]
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Gap`].
    ///
    /// [`Base(v)`]: #variant.Base
    /// [`Gap`]: #variant.Gap
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Base('a');
    /// assert_eq!(x.unwrap(), 'a');
    /// ```
    ///
    /// ```{.should_panic}
    /// use seqrs::alphabets::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Gap;
    /// assert_eq!(x.unwrap(), 'a'); // fails
    /// ```
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Gapped::Base(val) => val,
            Gapped::Gap => panic!("called `Gapped::unwrap()` on a `Gap` value"),
        }
    }

    /// Returns the contained value or a default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: #method.unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// assert_eq!(Gapped::Base('A').unwrap_or('T'), 'A');
    /// assert_eq!(Gapped::Gap.unwrap_or('T'), 'T');
    /// ```
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        match self {
            Gapped::Base(x) => x,
            Gapped::Gap => def,
        }
    }

    /// Returns the contained value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// assert_eq!(Gapped::Base('A').unwrap_or_else(|| '-'), 'A');
    /// assert_eq!(Gapped::Gap.unwrap_or_else(|| '-'), '-');
    /// ```
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Gapped::Base(x) => x,
            Gapped::Gap => f(),
        }
    }

    /// Maps an `Gapped<T>` to `Gapped<U>` by applying a function to a contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let base = Gapped::Base('a');
    /// let maybe_base = base.map(|s| s.to_ascii_uppercase());
    /// assert_eq!(maybe_base, Gapped::Base('A'));
    /// ```
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Gapped<U> {
        match self {
            Gapped::Base(x) => Gapped::Base(f(x)),
            Gapped::Gap => Gapped::Gap,
        }
    }

    /// Applies a function to the contained value (if any),
    /// or returns the provided default (if not).
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let base: Gapped<char> = Gapped::Base('A');
    /// assert_eq!(base.map_or('T', |v| v.to_ascii_lowercase()), 'a');
    ///
    /// let base: Gapped<char> = Gapped::Gap;
    /// assert_eq!(base.map_or('T', |v| v.to_ascii_lowercase()), 'T');
    /// ```
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Gapped::Base(t) => f(t),
            Gapped::Gap => default,
        }
    }

    /// Applies a function to the contained value (if any),
    /// or computes a default (if not).
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    ///
    /// let k = 21;
    ///
    /// let x: Gapped<char> = Gapped::Base('C');
    /// assert_eq!(x.map_or_else(|| 2 * k, |_| 4), 4);
    /// ```
    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Gapped::Base(t) => f(t),
            Gapped::Gap => default(),
        }
    }

    /// Transforms the `Gapped<T>` into a [`Result<T, E>`], mapping [`Base(v)`] to
    /// [`Ok(v)`] and [`Gap`] to [`Err(err)`].
    ///
    /// Arguments passed to `ok_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`ok_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`Result<T, E>`]: ../../std/result/enum.Result.html
    /// [`Ok(v)`]: ../../std/result/enum.Result.html#variant.Ok
    /// [`Err(err)`]: ../../std/result/enum.Result.html#variant.Err
    /// [`Gap`]: #variant.Gap
    /// [`Base(v)`]: #variant.Base
    /// [`ok_or_else`]: #method.ok_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    /// use seqrs::alphabets::DNA;
    ///
    /// let x = Gapped::Base(DNA::A);
    /// assert_eq!(x.ok_or(0), Ok(DNA::A));
    ///
    /// let x: Gapped<DNA> = Gapped::Gap;
    /// assert_eq!(x.ok_or(0), Err(0));
    /// ```
    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Gapped::Base(v) => Ok(v),
            Gapped::Gap => Err(err),
        }
    }

    /// Transforms the `Gapped<T>` into a [`Result<T, E>`], mapping [`Base(v)`] to
    /// [`Ok(v)`] and [`Gap`] to [`Err(err())`].
    ///
    /// [`Result<T, E>`]: ../../std/result/enum.Result.html
    /// [`Ok(v)`]: ../../std/result/enum.Result.html#variant.Ok
    /// [`Err(err())`]: ../../std/result/enum.Result.html#variant.Err
    /// [`Gap`]: #variant.Gap
    /// [`Base(v)`]: #variant.Base
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::alphabets::Gapped;
    /// use seqrs::alphabets::DNA;
    ///
    /// let x = Gapped::Base(DNA::A);
    /// assert_eq!(x.ok_or_else(|| 0), Ok(DNA::A));
    ///
    /// let x: Gapped<DNA> = Gapped::Gap;
    /// assert_eq!(x.ok_or_else(|| 0), Err(0));
    /// ```
    #[inline]
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Gapped::Base(v) => Ok(v),
            Gapped::Gap => Err(err()),
        }
    }

}


impl<T> Default for Gapped<T> {
    /// Returns [`Gap`][Gapped::Gap].
    #[inline]
    fn default() -> Gapped<T> { Gapped::Gap }
}


impl<T: TryFrom<char>> TryFrom<char> for Gapped<T> {
    type Error = T::Error;

    /// Parse a character as a gap, and pass non-gap to wrapped type.
    fn try_from(base: char) -> Result<Self, Self::Error> {
        match base {
            '-' => Ok(Gapped::Gap),
            a   => T::try_from(a).map(Gapped::Base),
        }
    }
}


/// Complement is implemented for any wrapped type that also implements
/// complement. A gap is always it's own complement.
impl<T: Complement> Complement for Gapped<T> {
    fn complement(&self) -> Self {
        match self {
            Gapped::Gap    => Gapped::Gap,
            Gapped::Base(a) => Gapped::Base(a.complement()),
        }
    }
}


/// Translate is implemented for any wrapped type that also implements
/// translate.
impl<A, T: Translate<A>> Translate<Gapped<A>> for Gapped<T> {
    fn translate(&self) -> Gapped<A> {
        match self {
            Gapped::Gap => Gapped::Gap,
            Gapped::Base(a) => Gapped::Base(a.translate()),
        }
    }
}



#[cfg(test)]
mod tests {
    //! Testing gaps.
    //! There are a couple of odd type hints required here that I'd like to
    //! get rid of.

    use super::*;
    use alphabets::DNA;

    #[test]
    fn test_from() {
        assert_eq!(Gapped::<DNA>::try_from('-').unwrap(), Gapped::Gap);
        assert_eq!(Gapped::<DNA>::try_from('A').unwrap(), Gapped::Base(DNA::A));
        assert_eq!(Gapped::<DNA>::try_from('T').unwrap(), Gapped::Base(DNA::T));
    }

    #[test]
    fn test_complement() {
        assert_eq!(Gapped::Gap::<DNA>.complement(), Gapped::Gap);
        assert_eq!(Gapped::Base(DNA::T).complement(), Gapped::Base(DNA::A));
    }
}

