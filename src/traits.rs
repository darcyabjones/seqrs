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
}

pub trait Complement {
    fn complement(&self) -> Self;
}

pub trait Translate<T> {
    fn translate(&self) -> T;
}
