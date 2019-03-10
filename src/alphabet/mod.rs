mod aa;
mod dna;
mod dna4;
mod tags;

pub use self::aa::AA;
pub use self::dna::DNA;
pub use self::dna4::DNA4;
pub use self::tags::CodonTag;


pub trait Alphabet: Sized {

    /// The size of the alphabet.
    /// E.G. A non-redundant dna alphabet has a cardinality of 4 (A, T, G, C).
    fn cardinality() -> u8;

    /// The unique numeric identifier for this character in the alphabet.
    /// Must start at 0 and always be less than `cardinality`.
    fn rank(&self) -> u8;

    /// Convert a numeric rank back to the alphabet variant, panicking if
    /// the rank doesn't exist in the alphabet.
    /// In general you should use the `from_rank` method unless you know that
    /// the rank will always be valid.
    unsafe fn from_rank_unsafe(r: u8) -> Self;

    /// Convert a numeric rank back to an alphabet variant, yielding `None`
    /// if the rank doesn't exist in the alphabet.
    ///
    /// This has a default implementation based on the assumption that any
    /// rank < cardinality will be valid.
    fn from_rank(r: u8) -> Option<Self> {
        if r < Self::cardinality() {
            Some( unsafe { Self::from_rank_unsafe(r) } )
        } else {
            None
        }
    }

    /// Returns a Vec of all variants in the alphabet.
    fn variants() -> Vec<Self>;
}
