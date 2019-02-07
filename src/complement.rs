use std::ops::Try;

pub trait Complement {
    fn complement(&self) -> Self;
}

pub trait ReverseComplement<I> {
    type Item;
    type Iter;

    fn reverse_complement(self) -> ReverseComplementIterator<I>;
}

impl<I, T, U> ReverseComplement<I> for U
    where U: IntoIterator<Item=T, IntoIter=I>,
          T: Complement,
          I: DoubleEndedIterator<Item=T>,
{
    type Item = T;
    type Iter = I;

    fn reverse_complement(self) -> ReverseComplementIterator<I> {
        ReverseComplementIterator {iter: self.into_iter() }
    }
}

/// A wrapper around map and rev.
#[derive(Debug, Clone)]
pub struct ReverseComplementIterator<I> {
    iter: I,
}

impl<I, T> Iterator for ReverseComplementIterator<I>
    where I: DoubleEndedIterator<Item=T>,
          T: Complement,
{
    type Item = T;

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

impl<I, T> DoubleEndedIterator for ReverseComplementIterator<I>
    where I: DoubleEndedIterator<Item=T>,
          T: Complement,
{
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> {
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

impl<I, T> ExactSizeIterator for ReverseComplementIterator<I>
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

