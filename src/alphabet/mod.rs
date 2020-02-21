//! Partial equality and set-like operations for redundant alphabets.
//!
//! Rather than overloading built in operations that describe strict numerical
//! equality reltionships, `seqrs` uses the [`Match`] and [`RedundantAlphabet`]
//! traits, to provide some custom semantics for redundant alphabets.
//! This allows non-redundant and redundant alphabets to share the same
//! matching interface, while retaining the useful built in traits.
//!
//! Both non-redundant and redundant alphabets should implement the [`Match`]
//! trait (for non-redundant, this will be equivalent to the [`PartialEq`]
//! trait). Only redundant alphabets need to implement the
//! [`RedundantAlphabet`] trait.
//!
//!
//!
//! [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
//! [`Match`]: trait.Match.html
//! [`RedundantAlphabet`]: trait.RedundantAlphabet.html


mod aa;
mod dna;
mod dna4;
mod tables;
mod tags;

pub use self::aa::AA;
pub use self::dna::DNA;
pub use self::dna4::DNA4;
pub use self::tags::CodonTag;

pub trait Alphabet: Sized {
    /// The size of the alphabet.
    /// E.G. A non-redundant dna alphabet has a cardinality of 4 (A, T, G, C).
    fn cardinality() -> usize;

    /// The unique numeric identifier for this character in the alphabet.
    /// Must start at 0 and always be less than `cardinality`.
    fn rank(&self) -> usize;

    /// Convert a numeric rank back to the alphabet variant, panicking if
    /// the rank doesn't exist in the alphabet.
    /// In general you should use the `from_rank` method unless you know that
    /// the rank will always be valid.
    unsafe fn from_rank_unsafe(r: usize) -> Self;

    /// Convert a numeric rank back to an alphabet variant, yielding `None`
    /// if the rank doesn't exist in the alphabet.
    ///
    /// This has a default implementation based on the assumption that any
    /// rank < cardinality will be valid.
    fn from_rank(r: usize) -> Option<Self> {
        if r < Self::cardinality() {
            Some(unsafe { Self::from_rank_unsafe(r) })
        } else {
            None
        }
    }

    /// Returns a Vec of all variants in the alphabet.
    fn variants() -> Vec<Self>;
}


/// Partial set operations for redundant alphabets.
///
/// You might encounter situations where you wanted to create a redundant
/// character from non-redundant ones, or you wanted to find the overlap
/// between two redundant bases. This trait implements some operations
/// borrowing the syntax of [sets] that can do this.
///
/// Note that some redundant alphabets (e.g. IUPAC amino acid) don't contain
/// every possible combination of elements, nor can there be an "Empty Set",
/// so the laws ruling set operations don't always quite hold.
/// Here for operations that could result in an empty set, results are wrapped
/// in an [`Option`] with [`None`] representing the empty set.
/// In the case of missing redundant combinations, the results should be
/// promoted to a more general variant (e.g. the full set). With these
/// caveats in mind, types implementing `RedundantAlphabet` should try to
/// satisfy the basic relationships and properties of sets.
///
/// See the [module level documentation] for usage examples with existing
/// alphabets.
///
/// # Examples:
///
/// Say we wanted to implement a 7-letter DNA alphabet with `N` representing
/// any base, `W` representing `T` or `A`, and `S` representing `C` or `G`.
/// We could define some set operations with [`RedundantAlphabet`] like this.
///
/// ```
/// use seqrs::alphabet::RedundantAlphabet;
///
/// // Cheating a bit by using Copy to get around borrowing trickyness
/// #[derive(PartialEq, Copy, Clone, Debug)]
/// pub enum DNA { A, T, G, C, W, S, N };
///
/// impl RedundantAlphabet for DNA {
///     // NB Using guard match pattens to keep this succinct and clear.
///     // You should probably find a more efficient way of doing this.
///
///     fn union(&self, other: &Self) -> Self {
///         match (self, other) {
///             (a, b) if a == b => *a,
///             (DNA::A, DNA::T) => DNA::W,
///             (DNA::T, DNA::A) => DNA::W,
///             (DNA::C, DNA::G) => DNA::S,
///             (DNA::G, DNA::C) => DNA::S,
///             _                => DNA::N,
///         }
///     }
///
///     fn intersection(&self, other: &Self) -> Option<Self> {
///         match (self, other) {
///             (a, b) if a == b                              => Some(*a),
///             (DNA::N, b)                                   => Some(*b),
///             (a, DNA::N)                                   => Some(*a),
///             (DNA::W, b) if (b == &DNA::A || b == &DNA::T) => Some(*b),
///             (a, DNA::W) if (a == &DNA::A || a == &DNA::T) => Some(*a),
///             (DNA::S, b) if (b == &DNA::C || b == &DNA::G) => Some(*b),
///             (a, DNA::S) if (a == &DNA::C || a == &DNA::G) => Some(*a),
///             _                                             => None,
///         }
///     }
///
///     fn difference(&self, other: &Self) -> Option<Self> {
///         match (self, other) {
///             (a, b) if a == b                              => None,
///             (_, DNA::N)                                   => None,
///             (a, DNA::W) if (a == &DNA::A || a == &DNA::T) => None,
///             (a, DNA::S) if (a == &DNA::C || a == &DNA::G) => None,
///             (DNA::N, DNA::W)                              => Some(DNA::S),
///             (DNA::N, DNA::S)                              => Some(DNA::W),
///             (DNA::N, _)                                   => Some(DNA::N),
///             (DNA::W, DNA::A)                              => Some(DNA::T),
///             (DNA::W, DNA::T)                              => Some(DNA::A),
///             (DNA::S, DNA::C)                              => Some(DNA::G),
///             (DNA::S, DNA::G)                              => Some(DNA::C),
///             (a, _)                                        => Some(*a),
///         }
///     }
///
///     fn is_redundant(&self) -> bool {
///         (self == &DNA::W) || (self == &DNA::S) || (self == &DNA::N)
///     }
/// }
///
/// // Unions return a character that can represent both members.
/// assert_eq!(DNA::A.union(&DNA::T), DNA::W);
/// // If no exact representative, use a more general representative
/// assert_eq!(DNA::A.union(&DNA::C), DNA::N);
/// // Unions should be commutative.
/// assert_eq!(DNA::A.union(&DNA::C), DNA::C.union(&DNA::A));
///
/// // Intersections help tease apart redundancy.
/// assert_eq!(DNA::W.intersection(&DNA::A), Some(DNA::A));
/// // But they can result in an empty set if no overlap.
/// assert_eq!(DNA::W.intersection(&DNA::S), None);
/// // Intersections should be commutative.
/// assert_eq!(DNA::W.union(&DNA::A), DNA::A.union(&DNA::W));
///
/// // Differences can be used to find complements, or tease apart redunancy.
/// assert_eq!(DNA::N.difference(&DNA::S), Some(DNA::W));
/// // If no exact representative, use a more general representative
/// assert_eq!(DNA::N.difference(&DNA::A), Some(DNA::N));
/// // But they can result in an empty set if they overlap completely.
/// assert_eq!(DNA::A.difference(&DNA::A), None);
/// // Difference between sets with no overlap yields same character.
/// assert_eq!(DNA::S.difference(&DNA::W), Some(DNA::S));
/// ```
///
/// [sets]: https://en.wikipedia.org/wiki/Set_(mathematics)#Basic_operations
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
/// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
/// [module level documentation]: ../index.html#examples
pub trait RedundantAlphabet: Sized {
    /// The set union of two characters.
    fn union(&self, other: &Self) -> Self;

    /// The set intersection of two characters, with None being the empty set.
    fn intersection(&self, other: &Self) -> Option<Self>;

    /// The set difference of two characters, with None being the empty set.
    fn difference(&self, other: &Self) -> Option<Self>;

    /// Is the character redundant?.
    fn is_redundant(&self) -> bool;
}
