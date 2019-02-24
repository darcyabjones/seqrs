//! Trait for nucleic acid complementation and a generalised reverse complement
//! iterator over sequences.
//!
//! # Examples:
//!
//! ```
//! use seqrs::complement::{Complement, IntoReverseComplement};
//!
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! pub enum DNA {A, T, G, C};
//!
//! impl Complement for &DNA {
//!     type Compl = DNA;
//!     fn complement(self) -> Self::Compl {
//!         match self {
//!             DNA::A => DNA::T,
//!             DNA::T => DNA::A,
//!             DNA::G => DNA::C,
//!             DNA::C => DNA::G,
//!         }
//!     }
//! }
//!
//! impl Complement for DNA {
//!     type Compl = DNA;
//!
//!     fn complement(self) -> Self::Compl {
//!         (&self).complement()
//!     }
//! }
//!
//! assert_eq!(DNA::A.complement(), DNA::T);
//!
//! // Any type that implements `IntoIterator` and `DoubleEndedIterator`
//! // automatically implements `ReverseComplement` if the type it contains
//! // implements `Complement`.
//! let seq = vec![DNA::A, DNA::T, DNA::G, DNA::C];
//! let rc: Vec<DNA> = seq.into_iter().reverse_complement().collect();
//!
//! assert_eq!(rc, vec![DNA::G, DNA::C, DNA::A, DNA::T]);
//! // Notice that collect is used, so the iterator is lazy, and won't perform
//! // any real work until used.
//!
//! // `reverse_complement` takes ownership of the object, so if you
//! // want to use `seq` again, first call `iter()`.
//!
//! // println!("{:?}", seq); // would panic.
//!
//! let seq = vec![DNA::A, DNA::T, DNA::G, DNA::C];
//! let rc: Vec<DNA> = seq.iter().reverse_complement().collect();
//!
//! assert_eq!(rc, vec![DNA::G, DNA::C, DNA::A, DNA::T]);
//! println!("{:?}", seq);
//! ```

use std::ops::Try;


pub trait Complement {
    type Compl;
    fn complement(self) -> Self::Compl;
}


pub trait IntoReverseComplement: Sized {
    type Iter;

    fn reverse_complement(self) -> ReverseComplement<Self>;
}


/// A wrapper around map and rev.
#[derive(Debug, Clone)]
pub struct ReverseComplement<I> {
    iter: I,
}


impl<I, T> IntoReverseComplement for I
    where T: Complement,
          I: DoubleEndedIterator<Item=T>,
{
    type Iter = I;

    fn reverse_complement(self) -> ReverseComplement<Self> {
        ReverseComplement { iter: self }
    }
}


impl<I, T> Iterator for ReverseComplement<I>
    where I: DoubleEndedIterator<Item=T>,
          T: Complement,
{
    type Item = <T as Complement>::Compl;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|b| b.complement())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn try_fold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> R,
              R: Try<Ok=Acc>,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = g(accum, x)?;
        }
        Try::from_ok(accum)
    }
}


impl<'a, I, T> DoubleEndedIterator for ReverseComplement<I>
    where I: DoubleEndedIterator<Item=T>,
          T: Complement,
{
    #[inline]
    fn next_back(&mut self) -> Option<<T as Complement>::Compl> {
        self.iter.next().map(|b| b.complement())
    }

    #[inline]
    fn try_rfold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> R,
              R: Try<Ok=Acc>,
    {
        let mut accum = init;
        while let Some(x) = self.next_back() {
            accum = g(accum, x)?;
        }
        Try::from_ok(accum)
    }
}


impl<I, T> ExactSizeIterator for ReverseComplement<I>
    where I: ExactSizeIterator<Item=T> + DoubleEndedIterator<Item=T>,
          T: Complement,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

