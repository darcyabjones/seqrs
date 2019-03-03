//! Generalised AA alphabet with stops.
//!
//! Extends Amino Acid alphabets to include stop codons.

use std::convert::TryFrom;
use std::fmt;

/// A stopped alphabet wraps an underlying alphabet in a new type.
/// Regular amino acids exist in [`Res`] wrappers, and Stop codons as [`Stop`]
/// variants. The [`StopOr`] variant handles cases of translating redundant
/// DNA alphabets, and cases of true biological variance where codon can
/// encode either a stop or a residue.
///
/// [`Res`]: #variant.Res
/// [`Stop`]: #variant.Stop
/// [`StopOr`]: #variant.StopOr
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Stopped<T> {
    Res(T),
    StopOr(T),
    Stop,
}

impl<T> Stopped<T> {
    /// Returns `true` if is the [`Res`] or [`StopOr`] variant.
    ///
    /// [`Res`]: #variant.Res
    /// [`StopOr`]: #variant.StopOr
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let x = Res('A');
    /// assert_eq!(x.is_residue(), true);
    ///
    /// let x = StopOr('A');
    /// assert_eq!(x.is_residue(), true);
    ///
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.is_residue(), false);
    /// ```
    pub fn is_residue(&self) -> bool {
        if let Stopped::Stop = self {
            false
        } else {
            true
        }
    }

    /// Returns `true` if is the [`Stop`] or [`StopOr`] variant.
    ///
    /// [`Stop`]: #variant.Stop
    /// [`StopOr`]: #variant.StopOr
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.is_stop(), true);
    ///
    /// let x = StopOr('A');
    /// assert_eq!(x.is_stop(), true);
    ///
    /// let x = Res('A');
    /// assert_eq!(x.is_stop(), false);
    /// ```
    pub fn is_stop(&self) -> bool {
        if let Stopped::Res(_) = self {
            false
        } else {
            true
        }
    }

    /// Converts from [`Stopped<T>`] to [`Stopped<&T>`].
    ///
    /// The [`map`] method takes the `self` argument by value, consuming the
    /// original, so this technique uses [`as_ref`] to first take a [`Stopped`]
    /// to a reference to the value inside the original.
    ///
    /// [`map`]: #method.map
    /// [`as_ref`]: #method.as_ref
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Stopped<&T>`]: enum.Stopped.html
    /// [`Stopped`]: enum.Stopped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let res = Res('a');
    ///
    /// // First, cast `Stopped<char>` to `Stopped<&char>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `res` on the stack.
    /// let upper = res.as_ref().map(|s| s.to_ascii_uppercase());
    ///
    /// assert_eq!(upper, Res('A'));
    /// println!("still can print: {:?} into {:?}", res, upper);
    /// ```
    pub fn as_ref(&self) -> Stopped<&T> {
        match *self {
            Stopped::Res(ref x) => Stopped::Res(x),
            Stopped::StopOr(ref x) => Stopped::StopOr(x),
            Stopped::Stop => Stopped::Stop,
        }
    }

    /// Converts from [`Stopped<T>`] to [`Stopped<&mut T>`].
    ///
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Stopped<&mut T>`]: enum.Stopped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let mut x = Res('a');
    /// match x.as_mut() {
    ///     Res(v) => *v = 't',
    ///     StopOr(v) => *v = 'X',
    ///     Stop => {},
    /// }
    /// assert_eq!(x, Res('t'));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Stopped<&mut T> {
        match *self {
            Stopped::Res(ref mut x) => Stopped::Res(x),
            Stopped::StopOr(ref mut x) => Stopped::StopOr(x),
            Stopped::Stop => Stopped::Stop,
        }
    }

    /// Unwraps a [`Stopped`], yielding the content of a [`Res`] or [`StopOr`].
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Stop`] with a custom panic message provided by
    /// `msg`.
    ///
    /// [`Stopped`]: enum.Stopped.html
    /// [`Res`]: #variant.Res
    /// [`StopOr`]: #variant.StopOr
    /// [`Stop`]: #variant.Stop
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    ///
    /// let x: Stopped<char> = Stopped::Res('a');
    /// assert_eq!(x.expect("the world is ending"), 'a');
    ///
    /// let x: Stopped<char> = Stopped::StopOr('a');
    /// assert_eq!(x.expect("the world is ending"), 'a');
    /// ```
    ///
    /// ```{.should_panic}
    /// use seqrs::stopped::Stopped;
    ///
    /// let x: Stopped<char> = Stopped::Stop;
    /// x.expect("the world is ending"); // panics with `the world is ending`
    /// ```
    pub fn expect(self, msg: &str) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x) => x,
            Stopped::Stop => panic!("{}", msg),
        }
    }

    /// Moves the value `v` out of the [`Stopped<T>`] if it is [`Res(v)`] or
    /// [`StopOr(v)`].
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Stop`]
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Stop`].
    ///
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Res(v)`]: #variant.Res
    /// [`StopOr(v)`]: #variant.StopOr
    /// [`Stop`]: #variant.Stop
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    ///
    /// let x: Stopped<char> = Stopped::Res('a');
    /// assert_eq!(x.unwrap(), 'a');
    ///
    /// let x: Stopped<char> = Stopped::StopOr('a');
    /// assert_eq!(x.unwrap(), 'a');
    /// ```
    ///
    /// ```{.should_panic}
    /// use seqrs::stopped::Stopped;
    ///
    /// let x: Stopped<char> = Stopped::Stop;
    /// assert_eq!(x.unwrap(), 'a'); // fails
    /// ```
    pub fn unwrap(self) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x) => x,
            Stopped::Stop => panic!("Called `Stopped::unwrap()` on a `Stop` value."),
        }
    }

    /// Returns the contained value or a default if variant is [`Stop`].
    ///
    /// Arguments passed to [`unwrap_or`] are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or`]: #method.unwrap_or
    /// [`unwrap_or_else`]: #method.unwrap_or_else
    /// [`Stop`]: #variant.Stop
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped::*;
    ///
    /// assert_eq!(Res('A').unwrap_or('T'), 'A');
    /// assert_eq!(StopOr('A').unwrap_or('T'), 'A');
    /// assert_eq!(Stop.unwrap_or('T'), 'T');
    /// ```
    pub fn unwrap_or(self, def: T) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x) => x,
            Stopped::Stop => def,
        }
    }

    /// Returns the contained value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped::*;
    ///
    /// assert_eq!(Res('A').unwrap_or_else(|| '*'), 'A');
    /// assert_eq!(StopOr('A').unwrap_or_else(|| '*'), 'A');
    /// assert_eq!(Stop.unwrap_or_else(|| '*'), '*');
    /// ```
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x) => x,
            Stopped::Stop => f(),
        }
    }

    /// Maps a [`Stopped<T>`] to [`Stopped<U>`] by applying a function to
    /// a contained value.
    ///
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Stopped<U>`]: enum.Stopped.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let res = Res('a');
    /// let maybe_res = res.map(|s| s.to_ascii_uppercase());
    /// assert_eq!(maybe_res, Res('A'));
    ///
    /// let res = StopOr('a');
    /// let maybe_res = res.map(|s| s.to_ascii_uppercase());
    /// assert_eq!(maybe_res, StopOr('A'));
    ///
    /// let res: Stopped<char> = Stop;
    /// let maybe_res = res.map(|s| s.to_ascii_uppercase());
    /// assert_eq!(maybe_res, Stop);
    /// ```
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Stopped<U> {
        match self {
            Stopped::Res(x) => Stopped::Res(f(x)),
            Stopped::StopOr(x) => Stopped::StopOr(f(x)),
            Stopped::Stop => Stopped::Stop,
        }
    }

    /// Applies a function to the contained value (if any),
    /// or returns the provided default (if not).
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let res = Res('M');
    /// assert_eq!(res.map_or('*', |v| v.to_ascii_lowercase()), 'm');
    ///
    /// let res = StopOr('M');
    /// assert_eq!(res.map_or('*', |v| v.to_ascii_lowercase()), 'm');
    ///
    /// let res: Stopped<char> = Stop;
    /// assert_eq!(res.map_or('*', |v| v.to_ascii_lowercase()), '*');
    /// ```
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Stopped::Res(x) => f(x),
            Stopped::StopOr(x) => f(x),
            Stopped::Stop => default,
        }
    }

    /// Applies a function to the contained value (if any),
    /// or computes a default (if not).
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let x = Res('C');
    /// assert_eq!(x.map_or_else(|| '*', |v| v.to_ascii_lowercase()), 'c');
    ///
    /// let x = StopOr('C');
    /// assert_eq!(x.map_or_else(|| '*', |v| v.to_ascii_lowercase()), 'c');
    ///
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.map_or_else(|| '*', |v| v.to_ascii_lowercase()), '*');
    /// ```
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        F: FnOnce(T) -> U,
        D: FnOnce() -> U,
    {
        match self {
            Stopped::Res(x) => f(x),
            Stopped::StopOr(x) => f(x),
            Stopped::Stop => default(),
        }
    }

    /// Applies a function that returns a [`Stopped`] value to the wrapped value.
    /// Similar to the [`and_then`] method on [`Option`] or [`Result`] types.
    ///
    /// [`Stopped`]: enum.Stopped.html
    /// [`and_then`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.and_then
    /// [`Option`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// // Applies functions over wrapped types.
    /// let x = Res('M');
    /// assert_eq!(x.flat_map(|c| Res(c == 'M')), Res(true));
    ///
    /// let x = StopOr('M');
    /// assert_eq!(x.flat_map(|c| StopOr(c == 'M')), StopOr(true));
    ///
    /// // Will not apply the function to the `Stop` variant.
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.flat_map(|c| StopOr(c == 'M')), Stop);
    ///
    /// // Can change the enum variant.
    /// let x = Res('*').flat_map(|c| {
    ///     if c == '*' {
    ///         Stop
    ///     } else {
    ///         Res(c)
    ///     }
    /// });
    /// assert_eq!(x, Stop);
    /// ```
    #[inline]
    pub fn flat_map<U, F: FnOnce(T) -> Stopped<U>>(self, f: F) -> Stopped<U> {
        self.map_or(Stopped::Stop, f)
    }

    /// Transforms the [`Stopped<T>`] into a [`Result<T, E>`], mapping
    /// [`Res(v)`] or [`StopOr(v)`] to [`Ok(v)`] and [`Stop`] to [`Err(err)`].
    ///
    /// Arguments passed to [`res_or`] are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`res_or_else`], which is lazily evaluated.
    ///
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Result<T, E>`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Ok(v)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err(err)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Stop`]: #variant.Stop
    /// [`StopOr(v)`]: #variant.StopOr
    /// [`Res(v)`]: #variant.Res
    /// [`stop_or`]: #method.stop_or
    /// [`stop_or_else`]: #method.stop_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let x = Res('M');
    /// assert_eq!(x.res_or(0), Ok('M'));
    ///
    /// let x = StopOr('M');
    /// assert_eq!(x.res_or(0), Ok('M'));
    ///
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.res_or(0), Err(0));
    /// ```
    pub fn res_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Stopped::Res(x) => Ok(x),
            Stopped::StopOr(x) => Ok(x),
            Stopped::Stop => Err(err),
        }
    }

    /// Transforms the [`Stopped<T>`] into a [`Result<T, E>`], mapping
    /// [`Res(v)`] or [`StopOr(v)`] to [`Ok(v)`] and [`Stop`] to [`Err(err())`].
    ///
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Result<T, E>`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Ok(v)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err(err())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Stop`]: #variant.Stop
    /// [`StopOr(v)`]: #variant.StopOr
    /// [`Res(v)`]: #variant.Res
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let x = Res('M');
    /// assert_eq!(x.res_or_else(|| 0), Ok('M'));
    ///
    /// let x = StopOr('M');
    /// assert_eq!(x.res_or_else(|| 0), Ok('M'));
    ///
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.res_or_else(|| 0), Err(0));
    /// ```
    pub fn res_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Stopped::Res(x) => Ok(x),
            Stopped::StopOr(x) => Ok(x),
            Stopped::Stop => Err(err()),
        }
    }

    /// Transforms the [`Stopped<T>`] into an [`Option<T>`], mapping [`Res(v)`]
    /// or [`StopOr(v)`] into [`Some(v)`] and [`Stop`] into [`None`].
    ///
    /// [`Stopped<T>`]: enum.Stopped.html
    /// [`Option<T>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
    /// [`None`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#variant.None
    /// [`Some(v)`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#variant.Some
    /// [`Stop`]: #variant.Stop
    /// [`StopOr(v)`]: #variant.StopOr
    /// [`Res(v)`]: #variant.Res
    ///
    /// # Examples
    ///
    /// ```
    /// use seqrs::stopped::Stopped;
    /// use seqrs::stopped::Stopped::*;
    ///
    /// let x = Res('x');
    /// assert_eq!(x.into_option(), Some('x'));
    ///
    /// let x = StopOr('x');
    /// assert_eq!(x.into_option(), Some('x'));
    ///
    /// let x: Stopped<char> = Stop;
    /// assert_eq!(x.into_option(), None);
    /// ```
    pub fn into_option(self) -> Option<T> {
        match self {
            Stopped::Res(x) => Some(x),
            Stopped::StopOr(x) => Some(x),
            Stopped::Stop => None,
        }
    }
}

impl<T> Default for Stopped<T> {
    #[inline]
    fn default() -> Stopped<T> {
        Stopped::Stop
    }
}

impl<T> From<Option<T>> for Stopped<T> {
    fn from(t: Option<T>) -> Stopped<T> {
        match t {
            Some(x) => Stopped::Res(x),
            None => Stopped::Stop,
        }
    }
}

impl<T> Into<Option<T>> for Stopped<T> {
    fn into(self) -> Option<T> {
        self.into_option()
    }
}

impl<'a, T> TryFrom<&'a u8> for Stopped<T>
where
    T: TryFrom<&'a u8>,
{
    type Error = T::Error;

    /// Parse a byte as a stop, and pass non-stop to wrapped type.
    ///
    /// # Examples:
    ///
    /// WARNING: try_from is currently unstable, so this example cannot be
    /// tested.
    ///
    /// ```rust,ignore
    /// use seqrs::alphabet::DNA;
    /// use seqrs::aastop::Stopped;
    /// use std::convert::{TryFrom, TryInto};
    ///
    /// let base = Stopped::<AA>::try_from(b'a').unwrap();
    /// assert_eq!(base, Stopped::Res(AA::A));
    ///
    /// let stop = Stopped::<AA>::try_from(b'*').unwrap();
    /// assert_eq!(stop, Stopped::Stop);
    /// ```
    fn try_from(base: &'a u8) -> Result<Self, Self::Error> {
        match base {
            b'*' => Ok(Stopped::Stop),
            a => T::try_from(a).map(Stopped::Res),
        }
    }
}

impl<T: TryFrom<u8>> TryFrom<u8> for Stopped<T> {
    type Error = T::Error;

    fn try_from(base: u8) -> Result<Self, Self::Error> {
        match base {
            b'*' => Ok(Stopped::Stop),
            a => T::try_from(a).map(Stopped::Res),
        }
    }
}

impl<'a, T> TryFrom<&'a char> for Stopped<T>
where
    T: TryFrom<&'a char>,
{
    type Error = T::Error;

    /// Parse a byte as a stop, and pass non-stop to wrapped type.
    ///
    /// # Examples:
    ///
    /// WARNING: try_from is currently unstable, so this example cannot be
    /// tested.
    ///
    /// ```rust,ignore
    /// use seqrs::alphabet::DNA;
    /// use seqrs::aastop::Stopped;
    /// use std::convert::{TryFrom, TryInto};
    ///
    /// let base = Stopped::<AA>::try_from('a').unwrap();
    /// assert_eq!(base, Stopped::Res(AA::A));
    ///
    /// let stop = Stopped::<AA>::try_from('*').unwrap();
    /// assert_eq!(stop, Stopped::Stop);
    /// ```
    fn try_from(base: &'a char) -> Result<Self, Self::Error> {
        match base {
            '*' => Ok(Stopped::Stop),
            a => T::try_from(a).map(Stopped::Res),
        }
    }
}

impl<T: TryFrom<char>> TryFrom<char> for Stopped<T> {
    type Error = T::Error;

    fn try_from(base: char) -> Result<Self, Self::Error> {
        match base {
            '*' => Ok(Stopped::Stop),
            a => T::try_from(a).map(Stopped::Res),
        }
    }
}

impl<T: Into<u8> + Copy> From<&Stopped<T>> for u8 {
    /// Convert Stopped alphabet to byte representation.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabet::AA;
    /// use seqrs::stopped::Stopped;
    /// use std::convert::{From, Into};
    ///
    /// assert_eq!(u8::from(Stopped::Res(AA::A)), b'A');
    ///
    /// let stop: Stopped<AA> = Stopped::Stop;
    /// assert_eq!(u8::from(stop), b'*');
    ///
    /// // Into is also implicitly defined.
    /// let res: u8 = Stopped::Res(AA::A).into();
    /// assert_eq!(res, b'A');
    /// ```
    fn from(base: &Stopped<T>) -> Self {
        match base {
            Stopped::Res(x) => (*x).into(),
            Stopped::StopOr(x) => (*x).into(),
            Stopped::Stop => b'*',
        }
    }
}

impl<T: Into<u8> + Copy> From<Stopped<T>> for u8 {
    /// Convert Stopped alphabet to byte representation.
    fn from(base: Stopped<T>) -> Self {
        match base {
            Stopped::Res(x) => x.into(),
            Stopped::StopOr(x) => x.into(),
            Stopped::Stop => b'*',
        }
    }
}

impl<T: Into<char> + Copy> From<&Stopped<T>> for char {
    /// Convert Stopped alphabet to byte representation.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabet::AA;
    /// use seqrs::stopped::Stopped;
    /// use std::convert::{From, Into};
    ///
    /// assert_eq!(char::from(Stopped::Res(AA::A)), 'A');
    ///
    /// let stop: Stopped<AA> = Stopped::Stop;
    /// assert_eq!(char::from(stop), '*');
    ///
    /// // Into is also implicitly defined.
    /// let res: char = Stopped::Res(AA::A).into();
    /// assert_eq!(res, 'A');
    /// ```
    fn from(base: &Stopped<T>) -> Self {
        match base {
            Stopped::Res(x) => (*x).into(),
            Stopped::StopOr(x) => (*x).into(),
            Stopped::Stop => '*',
        }
    }
}

impl<T: Into<char> + Copy> From<Stopped<T>> for char {
    /// Convert Stopped alphabet to byte representation.
    fn from(base: Stopped<T>) -> Self {
        match base {
            Stopped::Res(x) => x.into(),
            Stopped::StopOr(x) => x.into(),
            Stopped::Stop => '*',
        }
    }
}

impl<T> fmt::Display for Stopped<T>
where
    Stopped<T>: Into<char>,
    T: Into<char> + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::alphabet::AA;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<Stopped<AA>>(), 2);
    }

    #[test]
    fn test_from() {
        assert_eq!(Stopped::<AA>::try_from('*').unwrap(), Stopped::Stop);
        assert_eq!(Stopped::<AA>::try_from('A').unwrap(), Stopped::Res(AA::A));
        assert_eq!(Stopped::<AA>::try_from(&'*').unwrap(), Stopped::Stop);
        assert_eq!(Stopped::<AA>::try_from(&'A').unwrap(), Stopped::Res(AA::A));

        assert_eq!(Stopped::<AA>::try_from(b'*').unwrap(), Stopped::Stop);
        assert_eq!(Stopped::<AA>::try_from(b'A').unwrap(), Stopped::Res(AA::A));
        assert_eq!(Stopped::<AA>::try_from(&b'*').unwrap(), Stopped::Stop);
        assert_eq!(Stopped::<AA>::try_from(&b'A').unwrap(), Stopped::Res(AA::A));

        assert_eq!(u8::from(Stopped::Res(AA::A)), b'A');
        assert_eq!(u8::from(Stopped::Stop::<AA>), b'*');
        assert_eq!(u8::from(&Stopped::Res(AA::A)), b'A');
        assert_eq!(u8::from(&Stopped::Stop::<AA>), b'*');

        assert_eq!(char::from(Stopped::Res(AA::A)), 'A');
        assert_eq!(char::from(Stopped::Stop::<AA>), '*');
        assert_eq!(char::from(&Stopped::Res(AA::A)), 'A');
        assert_eq!(char::from(&Stopped::Stop::<AA>), '*');
    }
}
