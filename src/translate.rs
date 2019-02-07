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
