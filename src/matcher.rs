//! Partial equality  operations for redundant alphabets.
//!
//! Rather than overloading built in operations that describe strict numerical
//! equality reltionships, `seqrs` uses the [`Match`] to provide some custom
//! semantics for redundant alphabets. This allows non-redundant and redundant
//! alphabets to share the same matching interface, while retaining the
//! useful built in traits.
//!
//! # Examples:
//!
//! ```
//! use seqrs::alphabet::DNA;
//! use seqrs::matcher::Match;
//!
//! // Matches identical characters.
//! assert!(DNA::A.matches(&DNA::A));
//!
//! // Matches redundant characters.
//! assert!(DNA::A.matches(&DNA::N));
//!
//! // If two redundant bases overlap in what they can match, they "match".
//! // W = A or T
//! // R = A or G
//! assert!(DNA::W.matches(&DNA::R)); // Matches because both contain "A"
//!
//! // But wont match if they don't overlap.
//! // S = C or G
//! assert!(DNA::W.doesnt_match(&DNA::S));
//! ```


/// A variant of the [`PartialEq`] trait specificically for handling redundant
/// alphabets, and to provide an universal interface for pattern matching.
///
/// NB. both redundant and non-redundant alphabets should implement this trait.
/// For non-redundant ones, this will be equivalent to `==` and `!=` operators.
///
/// See the [module level documentation] for usage examples with existing
/// alphabets.
///
/// # Examples:
///
/// Say we wanted a 5-character DNA alphabet with "N" representing a general
/// redundant base. We can implement [`Match`] like this.
///
/// ```
/// use seqrs::matcher::Match;
///
/// #[derive(PartialEq, Eq, Debug)]
/// pub enum DNA5 { A, T, G, C, N };
///
/// impl Match for DNA5 {
///     fn matches(&self, other: &DNA5) -> bool {
///         match (self, other) {
///             (DNA5::N, _) => true,
///             (_, DNA5::N) => true,
///             (a,       b) => a == b,
///         }
///     }
/// }
///
/// assert!(DNA5::A.matches(&DNA5::A));
/// assert!(DNA5::A.matches(&DNA5::N));
/// assert!(DNA5::A.doesnt_match(&DNA5::T));
/// ```
///
/// Note that match uses `Self` as a default type for other.
/// This means that you don't have to provide the type parameter for the most
/// common use case where you're matching two of the same type.
/// It is possible to define matching between two different types, see the
/// [`Add`] trait docs for some good examples of using default type parameters.
///
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [`Match`]: trait.Match.html
/// [module level documentation]: ../index.html#examples
/// ['Add']: https://doc.rust-lang.org/std/ops/trait.Add.html
pub trait Match<T = Self> {
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
