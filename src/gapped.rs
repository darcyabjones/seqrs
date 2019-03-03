//! Generalised gapped alphabets.
//!
//! Gapped alphabets extend regular alphabets with a sum type and wrapping
//! the base in a tuple struct.
//!

use std::convert::TryFrom;
use std::fmt;

/// A gapped alphabet essentially adds a new [`Gap`] variant to the wrapped type.
/// [`Base`] is used to contain the wrapped alphabet variants (apologies to
/// Amino Acid fans out there :) ). [`Gap`] represents a gap in the sequence.
///
/// It is very similar to the [`Option`] type, but with a few extra methods
/// that were difficult to implement on top of option.
///
/// See the [module](../gapped/index.html) level documentation for more
/// practical description of how to use it.
///
/// [`Gap`]: #variant.Gap
/// [`Base`]: #variant.Base
/// [`Option`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Gapped<T> {
    Gap,
    Base(T),
}

impl<T> Gapped<T> {
    /// Returns true if is [`Base`] option.
    ///
    /// [`Base`]: #variant.Base
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let x = Base('a');
    /// assert_eq!(x.is_base(), true);
    ///
    /// let x: Gapped<char> = Gap;
    /// assert_eq!(x.is_base(), false);
    /// ```
    #[inline]
    pub fn is_base(&self) -> bool {
        match *self {
            Gapped::Base(_) => true,
            Gapped::Gap => false,
        }
    }

    /// Returns true if is [`Gap`] option.
    ///
    /// [`Gap`]: #variant.Gap
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let x = Base('a');
    /// assert_eq!(x.is_gap(), false);
    ///
    /// let x: Gapped<char> = Gap;
    /// assert_eq!(x.is_gap(), true);
    /// ```
    #[inline]
    pub fn is_gap(&self) -> bool {
        !self.is_base()
    }

    /// Converts from [`Gapped<T>`] to [`Gapped<&T>`].
    ///
    /// The [`map`] method takes the `self` argument by value, consuming the
    /// original, so this technique uses [`as_ref`] to first take a [`Gapped`]
    /// to a reference to the value inside the original.
    ///
    /// [`map`]: #method.map
    /// [`as_ref`]: #method.as_ref
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Gapped<&T>`]: enum.Gapped.html
    /// [`Gapped`]: enum.Gapped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let base = Base('a');
    ///
    /// // First, cast `Gapped<char>` to `Gapped<&char>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `base` on the stack.
    /// let upper = base.as_ref().map(|s| s.to_ascii_uppercase());
    ///
    /// assert_eq!(upper, Base('A'));
    /// println!("still can print: {:?} into {:?}", base, upper);
    /// ```
    #[inline]
    pub fn as_ref(&self) -> Gapped<&T> {
        match *self {
            Gapped::Base(ref x) => Gapped::Base(x),
            Gapped::Gap => Gapped::Gap,
        }
    }

    /// Converts from [`Gapped<T>`] to [`Gapped<&mut T>`].
    ///
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Gapped<&mut T>`]: enum.Gapped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let mut x = Base('a');
    /// match x.as_mut() {
    ///     Base(v) => *v = 't',
    ///     Gap => {},
    /// }
    /// assert_eq!(x, Base('t'));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Gapped<&mut T> {
        match *self {
            Gapped::Base(ref mut x) => Gapped::Base(x),
            Gapped::Gap => Gapped::Gap,
        }
    }

    /// Unwraps a [`Gapped`], yielding the content of a [`Base`].
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Gap`] with a custom panic message provided by
    /// `msg`.
    ///
    /// [`Gapped`]: enum.Gapped.html
    /// [`Base`]: #variant.Base
    /// [`Gap`]: #variant.Gap
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Base('a');
    /// assert_eq!(x.expect("the world is ending"), 'a');
    /// ```
    ///
    /// ```{.should_panic}
    /// use seqrs::gapped::Gapped;
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

    /// Moves the value `v` out of the [`Gapped<T>`] if it is [`Base(v)`].
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Gap`]
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Gap`].
    ///
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Base(v)`]: #variant.Base
    /// [`Gap`]: #variant.Gap
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    ///
    /// let x: Gapped<char> = Gapped::Base('a');
    /// assert_eq!(x.unwrap(), 'a');
    /// ```
    ///
    /// ```{.should_panic}
    /// use seqrs::gapped::Gapped;
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
    /// Arguments passed to [`unwrap_or`] are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or`]: #method.unwrap_or
    /// [`unwrap_or_else`]: #method.unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped::*;
    ///
    /// assert_eq!(Base('A').unwrap_or('T'), 'A');
    /// assert_eq!(Gap.unwrap_or('T'), 'T');
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
    /// use seqrs::gapped::Gapped::*;
    ///
    /// assert_eq!(Base('A').unwrap_or_else(|| '-'), 'A');
    /// assert_eq!(Gap.unwrap_or_else(|| '-'), '-');
    /// ```
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Gapped::Base(x) => x,
            Gapped::Gap => f(),
        }
    }

    /// Maps a [`Gapped<T>`] to [`Gapped<U>`] by applying a function to
    /// a contained value.
    ///
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Gapped<U>`]: enum.Gapped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let base = Base('a');
    /// let maybe_base = base.map(|s| s.to_ascii_uppercase());
    /// assert_eq!(maybe_base, Base('A'));
    ///
    /// let base: Gapped<char> = Gap;
    /// let maybe_base = base.map(|s| s.to_ascii_uppercase());
    /// assert_eq!(maybe_base, Gap);
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
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let base = Base('A');
    /// assert_eq!(base.map_or('T', |v| v.to_ascii_lowercase()), 'a');
    ///
    /// let base: Gapped<char> = Gap;
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
    /// use seqrs::gapped::Gapped;
    ///
    /// let k = 21;
    ///
    /// let x = Gapped::Base('C');
    /// assert_eq!(x.map_or_else(|| 2 * k, |_| 4), 4);
    /// ```
    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Gapped::Base(t) => f(t),
            Gapped::Gap => default(),
        }
    }

    /// Applies a function that returns a [`Gapped`] value to the wrapped value.
    ///
    /// [`Gapped`]: enum.Gapped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// // Applies functions over wrapped types.
    /// let x = Base('C');
    /// assert_eq!(x.flat_map(|c| Base(c == 'C')), Base(true));
    ///
    /// // Can change the enum variant.
    /// let x = Base('-').flat_map(|c| {
    ///     if c == '-' {
    ///         Gap
    ///     } else {
    ///         Base(c)
    ///     }
    /// });
    /// assert_eq!(x, Gap);
    ///
    /// // Will not apply the function to the `Gap` variant.
    /// let x: Gapped<char> = Gap;
    /// assert_eq!(x.flat_map(|_| Base('x')), Gap);
    /// ```
    #[inline]
    pub fn flat_map<U, F: FnOnce(T) -> Gapped<U>>(self, f: F) -> Gapped<U> {
        self.map_or(Gapped::Gap, f)
    }

    /// Transforms the [`Gapped<T>`] into a [`Result<T, E>`], mapping [`Base(v)`] to
    /// [`Ok(v)`] and [`Gap`] to [`Err(err)`].
    ///
    /// Arguments passed to [`base_or`] are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`base_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Result<T, E>`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Ok(v)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err(err)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Gap`]: #variant.Gap
    /// [`Base(v)`]: #variant.Base
    /// [`base_or`]: #method.base_or
    /// [`base_or_else`]: #method.base_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let x = Base('A');
    /// assert_eq!(x.base_or(0), Ok('A'));
    ///
    /// let x: Gapped<char> = Gap;
    /// assert_eq!(x.base_or(0), Err(0));
    /// ```
    #[inline]
    pub fn base_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Gapped::Base(v) => Ok(v),
            Gapped::Gap => Err(err),
        }
    }

    /// Transforms the [`Gapped<T>`] into a [`Result<T, E>`], mapping
    /// [`Base(v)`] to [`Ok(v)`] and [`Gap`] to [`Err(err())`].
    ///
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Result<T, E>`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Ok(v)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err(err())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Gap`]: #variant.Gap
    /// [`Base(v)`]: #variant.Base
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let x = Base('A');
    /// assert_eq!(x.base_or_else(|| 0), Ok('A'));
    ///
    /// let x: Gapped<char> = Gap;
    /// assert_eq!(x.base_or_else(|| 0), Err(0));
    /// ```
    #[inline]
    pub fn base_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Gapped::Base(v) => Ok(v),
            Gapped::Gap => Err(err()),
        }
    }

    /// Transforms the [`Gapped<T>`] into an [`Option<T>`], mapping [`Base(v)`]
    /// into [`Some(v)`] and [`Gap`] into [`None`].
    ///
    /// [`Gapped<T>`]: enum.Gapped.html
    /// [`Option<T>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
    /// [`None`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#variant.None
    /// [`Some(v)`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#variant.Some
    /// [`Gap`]: #variant.Gap
    /// [`Base(v)`]: #variant.Base
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::gapped::Gapped;
    /// use seqrs::gapped::Gapped::*;
    ///
    /// let x = Base('x');
    /// assert_eq!(x.into_option(), Some('x'));
    ///
    /// let x: Gapped<char> = Gap;
    /// assert_eq!(x.into_option(), None);
    /// ```
    pub fn into_option(self) -> Option<T> {
        match self {
            Gapped::Base(v) => Some(v),
            Gapped::Gap => None,
        }
    }
}

impl<T> Default for Gapped<T> {
    /// Returns [`Gap`][Gapped::Gap].
    #[inline]
    fn default() -> Gapped<T> {
        Gapped::Gap
    }
}

impl<T> From<Option<T>> for Gapped<T> {
    fn from(t: Option<T>) -> Gapped<T> {
        match t {
            Some(base) => Gapped::Base(base),
            None => Gapped::Gap,
        }
    }
}

impl<T> Into<Option<T>> for Gapped<T> {
    fn into(self) -> Option<T> {
        match self {
            Gapped::Base(base) => Some(base),
            Gapped::Gap => None,
        }
    }
}

impl<'a, T> TryFrom<&'a u8> for Gapped<T>
where
    T: TryFrom<&'a u8>,
{
    type Error = T::Error;

    /// Parse a byte as a gap, and pass non-gap to wrapped type.
    ///
    /// # Examples:
    ///
    /// ```rust,ignore
    /// use seqrs::alphabet::DNA;
    /// use seqrs::gapped::Gapped;
    /// use std::convert::{TryFrom, TryInto};
    ///
    /// let base = Gapped::<DNA>::try_from(b'a').unwrap();
    /// assert_eq!(base, Gapped::Base(DNA::A));
    ///
    /// let base = Gapped::<DNA>::try_from(b'-').unwrap();
    /// assert_eq!(base, Gapped::Gap);
    /// ```
    fn try_from(base: &'a u8) -> Result<Self, Self::Error> {
        match base {
            b'-' => Ok(Gapped::Gap),
            a => T::try_from(a).map(Gapped::Base),
        }
    }
}

impl<T: TryFrom<u8>> TryFrom<u8> for Gapped<T> {
    type Error = T::Error;

    /// Parse an owned byte as a gap, and pass non-gap to wrapped type.
    fn try_from(base: u8) -> Result<Self, Self::Error> {
        match base {
            b'-' => Ok(Gapped::Gap),
            a => T::try_from(a).map(Gapped::Base),
        }
    }
}

impl<'a, T: TryFrom<&'a char>> TryFrom<&'a char> for Gapped<T> {
    type Error = T::Error;

    /// Parse a character as a gap, and pass non-gap to wrapped type.
    ///
    /// # Examples:
    ///
    /// WARNING: try_from is currently unstable, so this example cannot be
    /// tested.
    ///
    /// ```rust,ignore
    /// use seqrs::alphabet::DNA;
    /// use seqrs::gapped::Gapped;
    /// use std::convert::{TryFrom, TryInto};
    ///
    /// let base = Gapped::<DNA>::try_from('a').unwrap();
    /// assert_eq!(base, Gapped::Base(DNA::A));
    ///
    /// let base = Gapped::<DNA>::try_from('-').unwrap();
    /// assert_eq!(base, Gapped::Gap);
    /// ```
    fn try_from(base: &'a char) -> Result<Self, Self::Error> {
        match base {
            '-' => Ok(Gapped::Gap),
            a => T::try_from(a).map(Gapped::Base),
        }
    }
}

impl<T: TryFrom<char>> TryFrom<char> for Gapped<T> {
    type Error = T::Error;

    /// Parse an owned byte as a gap, and pass non-gap to wrapped type.
    fn try_from(base: char) -> Result<Self, Self::Error> {
        match base {
            '-' => Ok(Gapped::Gap),
            a => T::try_from(a).map(Gapped::Base),
        }
    }
}

impl<T: Into<u8> + Copy> From<&Gapped<T>> for u8 {
    /// Convert gapped alphabet to byte representation.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabet::DNA;
    /// use seqrs::gapped::Gapped;
    /// use std::convert::{From, Into};
    ///
    /// assert_eq!(u8::from(Gapped::Base(DNA::A)), b'A');
    ///
    /// let gap: Gapped<DNA> = Gapped::Gap;
    /// assert_eq!(u8::from(gap), b'-');
    ///
    /// // Into is also implicitly defined.
    /// let base: u8 = Gapped::Base(DNA::A).into();
    /// assert_eq!(base, b'A');
    /// ```
    fn from(base: &Gapped<T>) -> Self {
        match base {
            Gapped::Base(x) => (*x).into(),
            Gapped::Gap => b'-',
        }
    }
}

impl<T: Into<u8> + Copy> From<Gapped<T>> for u8 {
    /// Convert gapped alphabet to byte representation.
    fn from(base: Gapped<T>) -> Self {
        match base {
            Gapped::Base(x) => x.into(),
            Gapped::Gap => b'-',
        }
    }
}

impl<T: Into<char> + Copy> From<&Gapped<T>> for char {
    /// Convert gapped alphabet to char representation.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabet::DNA;
    /// use seqrs::gapped::Gapped;
    /// use std::convert::{From, Into};
    ///
    /// assert_eq!(char::from(&Gapped::Base(DNA::A)), 'A');
    ///
    /// let gap: Gapped<DNA> = Gapped::Gap;
    /// assert_eq!(char::from(&gap), '-');
    ///
    /// // Into is also implicitly defined.
    /// let base: char = (&Gapped::Base(DNA::A)).into();
    /// assert_eq!(base, 'A');
    /// ```
    fn from(base: &Gapped<T>) -> Self {
        match base {
            Gapped::Base(x) => (*x).into(),
            Gapped::Gap => '-',
        }
    }
}

impl<T: Into<char> + Copy> From<Gapped<T>> for char {
    /// Convert gapped alphabet to char representation.
    fn from(base: Gapped<T>) -> Self {
        match base {
            Gapped::Base(x) => x.into(),
            Gapped::Gap => '-',
        }
    }
}

impl<T> fmt::Display for Gapped<T>
where
    Gapped<T>: Into<char>,
    T: Into<char> + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

#[cfg(test)]
mod tests {
    //! Testing gaps.
    //! There are a couple of odd type hints required here that I'd like to
    //! get rid of.

    use super::Gapped::*;
    use super::*;
    use crate::alphabet::DNA;

    use std::convert::TryInto;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<Gapped<DNA>>(), 1);
    }

    #[test]
    fn test_from() {
        assert_eq!(Gapped::<DNA>::try_from('-').unwrap(), Gapped::Gap);
        assert_eq!(Gapped::<DNA>::try_from('A').unwrap(), Gapped::Base(DNA::A));
        assert_eq!(Gapped::<DNA>::try_from('T').unwrap(), Gapped::Base(DNA::T));
        assert_eq!(Gapped::<DNA>::try_from(&'-').unwrap(), Gapped::Gap);
        assert_eq!(Gapped::<DNA>::try_from(&'A').unwrap(), Gapped::Base(DNA::A));
        assert_eq!(Gapped::<DNA>::try_from(&'T').unwrap(), Gapped::Base(DNA::T));

        assert_eq!(Gapped::<DNA>::try_from(b'T').unwrap(), Gapped::Base(DNA::T));
        assert_eq!(Gapped::<DNA>::try_from(b'-').unwrap(), Gapped::Gap);
        assert_eq!(
            Gapped::<DNA>::try_from(&b'T').unwrap(),
            Gapped::Base(DNA::T)
        );
        assert_eq!(Gapped::<DNA>::try_from(&b'-').unwrap(), Gapped::Gap);

        assert_eq!(u8::from(Gapped::Base(DNA::T)), b'T');
        assert_eq!(u8::from(Gapped::Gap::<DNA>), b'-');
        assert_eq!(u8::from(&Gapped::Base(DNA::T)), b'T');
        assert_eq!(u8::from(&Gapped::Gap::<DNA>), b'-');

        assert_eq!(char::from(Gapped::Base(DNA::T)), 'T');
        assert_eq!(char::from(Gapped::Gap::<DNA>), '-');
        assert_eq!(char::from(&Gapped::Base(DNA::T)), 'T');
        assert_eq!(char::from(&Gapped::Gap::<DNA>), '-');
    }

    #[test]
    fn test_from_iter() {
        let seq: Result<Vec<Gapped<DNA>>, _> = "ATG-N".bytes().map(|b| b.try_into()).collect();

        assert_eq!(
            seq.unwrap(),
            vec![Base(DNA::A), Base(DNA::T), Base(DNA::G), Gap, Base(DNA::N)]
        );
    }

}
