use std::ops::Try;


///
///
/// The match trait
pub trait Match<T> {
    fn matches(&self, other: &T) -> bool;

    fn doesnt_match(&self, other: &T) -> bool {
        !self.matches(other)
    }
}

pub trait RedundantAlphabet: Sized {

    fn union(&self, other: &Self) -> Self;

    fn intersection(&self, other: &Self) -> Option<Self>;

    fn difference(&self, other: &Self) -> Option<Self>;

    fn is_redundant(&self) -> bool;
}

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

    fn try_fold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> R,
              R: Try<Ok=Acc>,
    {
        self.iter.try_rfold(init, move |acc, elt| g(acc, elt.complement()))
    }

    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.rfold(init, move |acc, elt| g(acc, elt.complement()))
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool
    {
        self.iter.rfind(predicate)
    }

    #[inline]
    fn rposition<P>(&mut self, predicate: P) -> Option<usize>
        where P: FnMut(Self::Item) -> bool,
    {
        self.iter.position(predicate)
    }

    /*
    // Currently not impl because requirement for DoubleEndedIterator +
    // ExactSizeIterator, might restrict impl.
    #[inline]
    fn position<P>(&mut self, predicate: P) -> Option<usize>
        where P: FnMut(Self::Item) -> bool,
    {
        self.iter.rposition(predicate)
    }
    */
}

impl<I, T> DoubleEndedIterator for ReverseComplementIterator<I>
    where I: DoubleEndedIterator<Item=T>,
          T: Complement,
{
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> {
        self.iter.next().map(|b| b.complement())
    }

    fn try_rfold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> R,
              R: Try<Ok=Acc>,
    {
        self.iter.try_fold(init, move |acc, elt| g(acc, elt.complement()))
    }

    fn rfold<Acc, G>(self, init: Acc, mut g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, move |acc, elt| g(acc, elt.complement()))
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool
    {
        self.iter.find(predicate)
    }
}

impl<I, T> ExactSizeIterator for ReverseComplementIterator<I>
    where I: ExactSizeIterator<Item=T> + DoubleEndedIterator<Item=T>,
          T: Complement,
{
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

pub trait Translate<T> {
    fn translate(&self) -> T;
}

/*
impl<I, T, U> Translate<Box<std::iter::Map<Iterator<Item=U>, Fn(T) -> U>>> for I
    where I: IntoIterator<Item=T, IntoIter=Iterator<Item=T>>,
          T: Translate<U>,
          U: Sized,
{
    fn translate(&self) -> Box<std::iter::Map<Iterator<Item=U>, Fn(T) -> U>> {
        fn trans<T: Translate<U>, U>(a: T) -> U {
            a.translate()
        }
        Box::new(self.into_iter().map(trans))
    }
}
*/
