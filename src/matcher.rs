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
//! # Examples:
//!
//! Say we wanted a 5-character DNA alphabet with "N" representing a general
//! redundant base. We can implement [`Match`] like this.
//!
//! ```
//! use seqrs::matcher::Match;
//!
//! #[derive(PartialEq, Eq, Debug)]
//! pub enum DNA5 { A, T, G, C, N };
//!
//! impl Match<DNA5> for DNA5 {
//!     fn matches(&self, other: &DNA5) -> bool {
//!         match (self, other) {
//!             (DNA5::N, _) => true,
//!             (_, DNA5::N) => true,
//!             (a,       b) => a == b,
//!         }
//!     }
//! }
//!
//! assert!(DNA5::A.matches(&DNA5::A));
//! assert!(DNA5::A.matches(&DNA5::N));
//! assert!(DNA5::A.doesnt_match(&DNA5::T));
//! ```
//!
//! Now say we wanted to implement a 7-letter DNA alphabet with `N` representing
//! any base, `W` representing `T` or `A`, and `S` representing `C` or `G`.
//! We could define some set operations with [`RedundantAlphabet`] like this.
//!
//! ```
//! use seqrs::matcher::RedundantAlphabet;
//!
//! // Cheating a bit by using Copy to get around borrowing trickyness
//! #[derive(PartialEq, Copy, Clone, Debug)]
//! pub enum DNA { A, T, G, C, W, S, N };
//!
//! impl RedundantAlphabet for DNA {
//!     // NB Using guard match pattens to keep this succinct and clear.
//!     // You should probably find a more efficient way of doing this.
//!
//!     fn union(&self, other: &Self) -> Self {
//!         match (self, other) {
//!             (a, b) if a == b => *a,
//!             (DNA::A, DNA::T) => DNA::W,
//!             (DNA::T, DNA::A) => DNA::W,
//!             (DNA::C, DNA::G) => DNA::S,
//!             (DNA::G, DNA::C) => DNA::S,
//!             _                => DNA::N,
//!         }
//!     }
//!
//!     fn intersection(&self, other: &Self) -> Option<Self> {
//!         match (self, other) {
//!             (a, b) if a == b                              => Some(*a),
//!             (DNA::N, b)                                   => Some(*b),
//!             (a, DNA::N)                                   => Some(*a),
//!             (DNA::W, b) if (b == &DNA::A || b == &DNA::T) => Some(*b),
//!             (a, DNA::W) if (a == &DNA::A || a == &DNA::T) => Some(*a),
//!             (DNA::S, b) if (b == &DNA::C || b == &DNA::G) => Some(*b),
//!             (a, DNA::S) if (a == &DNA::C || a == &DNA::G) => Some(*a),
//!             _                                             => None,
//!         }
//!     }
//!
//!     fn difference(&self, other: &Self) -> Option<Self> {
//!         match (self, other) {
//!             (a, b) if a == b                              => None,
//!             (_, DNA::N)                                   => None,
//!             (a, DNA::W) if (a == &DNA::A || a == &DNA::T) => None,
//!             (a, DNA::S) if (a == &DNA::C || a == &DNA::G) => None,
//!             (DNA::N, DNA::W)                              => Some(DNA::S),
//!             (DNA::N, DNA::S)                              => Some(DNA::W),
//!             (DNA::N, _)                                   => Some(DNA::N),
//!             (DNA::W, DNA::A)                              => Some(DNA::T),
//!             (DNA::W, DNA::T)                              => Some(DNA::A),
//!             (DNA::S, DNA::C)                              => Some(DNA::G),
//!             (DNA::S, DNA::G)                              => Some(DNA::C),
//!             (a, _)                                        => Some(*a),
//!         }
//!     }
//!
//!     fn is_redundant(&self) -> bool {
//!         (self == &DNA::W) || (self == &DNA::S) || (self == &DNA::N)
//!     }
//! }
//!
//! // Unions return a character that can represent both members.
//! assert_eq!(DNA::A.union(&DNA::T), DNA::W);
//! // If no exact representative, use a more general representative
//! assert_eq!(DNA::A.union(&DNA::C), DNA::N);
//! // Unions should be commutative.
//! assert_eq!(DNA::A.union(&DNA::C), DNA::C.union(&DNA::A));
//!
//! // Intersections help tease apart redundancy.
//! assert_eq!(DNA::W.intersection(&DNA::A), Some(DNA::A));
//! // But they can result in an empty set if no overlap.
//! assert_eq!(DNA::W.intersection(&DNA::S), None);
//! // Intersections should be commutative.
//! assert_eq!(DNA::W.union(&DNA::A), DNA::A.union(&DNA::W));
//!
//! // Differences can be used to find complements, or tease apart redunancy.
//! assert_eq!(DNA::N.difference(&DNA::S), Some(DNA::W));
//! // If no exact representative, use a more general representative
//! assert_eq!(DNA::N.difference(&DNA::A), Some(DNA::N));
//! // But they can result in an empty set if they overlap completely.
//! assert_eq!(DNA::A.difference(&DNA::A), None);
//! // Difference between sets with no overlap yields same character.
//! assert_eq!(DNA::S.difference(&DNA::W), Some(DNA::S));
//! ```
//!
//! [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
//! [`Match`]: trait.Match.html
//! [`RedundantAlphabet`]: trait.RedundantAlphabet.html

/// A variant of the [`PartialEq`] trait specificically for handling redundant
/// alphabets, and to provide an universal interface for pattern matching.
///
/// NB. both redundant and non-redundant alphabets should implement this trait.
/// For non-redundant ones, this will be equivalent to `==` and `!=` operators.
///
/// See the [module level documentation] for implementation examples.
///
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [module level documentation]: ../index.html#examples
pub trait Match<T> {
    /// Defines "equality" semantics between elements.
    fn matches(&self, other: &T) -> bool;

    /// The logical inverse of the [`matches`] method.
    /// Has a default implemetation.
    ///
    /// [`matches`]: #method.matches
    fn doesnt_match(&self, other: &T) -> bool {
        !self.matches(other)
    }
}

/// Provides partial set operations for redundant alphabets.
///
/// You might encounter situations where you wanted to create a redundant
/// character from non-redundant ones, or you wanted to find the overlap
/// between two redundant bases. This trait implements some operations borrowing
/// the syntax of [sets] that can do this.
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
/// See the [module level documentation] for implementation examples.
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
