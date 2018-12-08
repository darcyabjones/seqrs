/// Complement trait.
///


pub trait Complement {
    fn complement(&self) -> Self;
}


pub trait ReverseComplement {
    fn reverse_complement(&self) -> Self;
}
