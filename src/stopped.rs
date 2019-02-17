//! Generalised AA alphabet with stops.

use std::convert::TryFrom;
use std::fmt;


#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Stopped<T> {
    Res(T),
    StopOr(T),
    Stop,
}


impl<T> Stopped<T> {

    pub fn is_residue(&self) -> bool {
        if let Stopped::Stop = self {
            false
        } else {
            true
        }
    }


    pub fn is_stop(&self) -> bool {
        match self {
            Stopped::Stop      => true,
            Stopped::StopOr(_) => true,
            _                  => false,
        }
    }


    pub fn as_ref(&self) -> Stopped<&T> {
        match *self {
            Stopped::Res(ref x)    => Stopped::Res(x),
            Stopped::StopOr(ref x) => Stopped::StopOr(x),
            Stopped::Stop          => Stopped::Stop,
        }
    }


    pub fn as_mut(&mut self) -> Stopped<&mut T> {
        match *self {
            Stopped::Res(ref mut x)     => Stopped::Res(x),
            Stopped::StopOr(ref mut x)  => Stopped::StopOr(x),
            Stopped::Stop               => Stopped::Stop,
        }
    }


    pub fn expect(self, msg: &str) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x)  => x,
            Stopped::Stop   => panic!("{}", msg),
        }
    }


    pub fn unwrap(self) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x)  => x,
            Stopped::Stop   => panic!("Called `Stopped::unwrap()` on a `Stop` value."),
        }
    }


    pub fn unwrap_or(self, def: T) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x)  => x,
            Stopped::Stop   => def,
        }
    }


    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Stopped::Res(x) => x,
            Stopped::StopOr(x)  => x,
            Stopped::Stop   => f(),
        }
    }


    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Stopped<U> {
        match self {
            Stopped::Res(x) => Stopped::Res(f(x)),
            Stopped::StopOr(x)  => Stopped::StopOr(f(x)),
            Stopped::Stop   => Stopped::Stop,
        }
    }


    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Stopped::Res(x) => f(x),
            Stopped::StopOr(x)  => f(x),
            Stopped::Stop   => default,
        }
    }


    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
        where F: FnOnce(T) -> U,
              D: FnOnce() -> U,
    {
        match self {
            Stopped::Res(x) => f(x),
            Stopped::StopOr(x)  => f(x),
            Stopped::Stop   => default(),
        }
    }


    pub fn res_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Stopped::Res(x) => Ok(x),
            Stopped::StopOr(x) => Ok(x),
            Stopped::Stop   => Err(err),
        }
    }


    pub fn res_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Stopped::Res(x)    => Ok(x),
            Stopped::StopOr(x) => Ok(x),
            Stopped::Stop      => Err(err()),
        }
    }


    pub fn into_option(self) -> Option<T> {
        match self {
            Stopped::Res(x)    => Some(x),
            Stopped::StopOr(x) => Some(x),
            Stopped::Stop      => None,
        }
    }
}


impl<T> Default for Stopped<T> {
    #[inline]
    fn default() -> Stopped<T> { Stopped::Stop }
}


impl<T> From<Option<T>> for Stopped<T> {
    fn from(t: Option<T>) -> Stopped<T> {
        match t {
            Some(x) => Stopped::Res(x),
            None    => Stopped::Stop,
        }
    }
}


impl<T> Into<Option<T>> for Stopped<T> {
    fn into(self) -> Option<T> {
        self.into_option()
    }
}


impl<'a, T> TryFrom<&'a u8> for Stopped<T>
    where T: TryFrom<&'a u8>
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
    /// let base = Stopped<AA>::try_from(b'a').unwrap();
    /// assert_eq!(base, Stopped::Res(AA::A));
    ///
    /// let stop = Stopped<AA>::try_from(b'*').unwrap();
    /// assert_eq!(stop, Stopped::Stop);
    /// ```
    fn try_from(base: &'a u8) -> Result<Self, Self::Error> {
        match base {
            b'*' => Ok(Stopped::Stop),
            a    => T::try_from(a).map(Stopped::Res),
        }
    }
}


impl<T: TryFrom<u8>> TryFrom<u8> for Stopped<T> {
    type Error = T::Error;

    fn try_from(base: u8) -> Result<Self, Self::Error> {
        match base {
            b'*' => Ok(Stopped::Stop),
            a    => T::try_from(a).map(Stopped::Res),
        }
    }
}


impl<'a, T> TryFrom<&'a char> for Stopped<T>
    where T: TryFrom<&'a char>
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
    /// let base = Stopped<AA>::try_from('a').unwrap();
    /// assert_eq!(base, Stopped::Res(AA::A));
    ///
    /// let stop = Stopped<AA>::try_from('*').unwrap();
    /// assert_eq!(stop, Stopped::Stop);
    /// ```
    fn try_from(base: &'a char) -> Result<Self, Self::Error> {
        match base {
            '*' => Ok(Stopped::Stop),
            a   => T::try_from(a).map(Stopped::Res),
        }
    }
}


impl<T: TryFrom<char>> TryFrom<char> for Stopped<T> {
    type Error = T::Error;

    fn try_from(base: char) -> Result<Self, Self::Error> {
        match base {
            '*' => Ok(Stopped::Stop),
            a   => T::try_from(a).map(Stopped::Res),
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
            Stopped::Res(x)    => (*x).into(),
            Stopped::StopOr(x) => (*x).into(),
            Stopped::Stop      => b'*',
        }
    }
}


impl<T: Into<u8> + Copy> From<Stopped<T>> for u8 {

    /// Convert Stopped alphabet to byte representation.
    fn from(base: Stopped<T>) -> Self {
        match base {
            Stopped::Res(x)    => x.into(),
            Stopped::StopOr(x) => x.into(),
            Stopped::Stop      => b'*',
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
            Stopped::Res(x)    => (*x).into(),
            Stopped::StopOr(x) => (*x).into(),
            Stopped::Stop      => '*',
        }
    }
}


impl<T: Into<char> + Copy> From<Stopped<T>> for char {

    /// Convert Stopped alphabet to byte representation.
    fn from(base: Stopped<T>) -> Self {
        match base {
            Stopped::Res(x)    => x.into(),
            Stopped::StopOr(x) => x.into(),
            Stopped::Stop      => '*',
        }
    }
}


impl<T> fmt::Display for Stopped<T>
    where Stopped<T>: Into<char>,
          T: Into<char> + Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use alphabet::AA;

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
