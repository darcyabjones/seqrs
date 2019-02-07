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


