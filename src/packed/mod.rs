// Bit packed datastructures for enums and their wrappers.
//
// ```
// let seq: Vec<Gapped<DNA>> = vec![Base(A), Base(T), Gap, Base(G), Base(C)];
// let pseq: Vec<Packed<Gapped<DNA>, SIZE=1, REPR=u8>> = seq
//     .into_iter()
//     .map(|b| b.pack())
//     .collect();
//
// let reseq: Vec<Gapped<DNA>> = pseq
//     .iter()
//     .flat_map(|b| b.unpack())
//     .collect();
//
// assert_eq!(pseq.len(), 5);
// assert_eq!(reseq, seq);
//
//
//
// ```



use crate::alphabet::DNA;
use std::marker::PhantomData;

pub trait EndianNess {}

pub struct BigEndian;
pub struct LittleEndian;

impl EndianNess for BigEndian {}
impl EndianNess for LittleEndian {}

pub struct Packed<A, I=u32, E=BigEndian> {
    pub bits: I,
    _alphabet: PhantomData<*const A>,
    _endian: PhantomData<*const E>,
}


pub trait Pack<T> {
    fn pack(self) -> T;
}


pub trait Unpack<T> {
    fn unpack(self) -> T;
}


impl From<DNA> for Packed<DNA, u8> {

    /// # Examples:
    ///
    /// ```
    /// use seqrs::packed::{Packed, Pack};
    /// use seqrs::alphabet::DNA;
    ///
    /// let b: Packed<DNA, u8> = DNA::A.into();
    ///
    /// assert_eq!(b.bits, 0b0001);
    /// ```
    fn from(x: DNA) -> Packed<DNA, u8> {
        Packed {
            bits: x as u8,
            _alphabet: PhantomData,
            _endian: PhantomData,
        }
    }
}


impl From<Packed<DNA, u8>> for DNA {

    /// # Examples:
    ///
    /// ```
    /// use seqrs::packed::{Packed, Pack, Unpack};
    /// use seqrs::alphabet::DNA;
    ///
    /// let b: Packed<DNA, u8> = DNA::A.into();
    /// let c: DNA = b.into();
    ///
    /// assert_eq!(c, DNA::A);
    ///
    /// assert_eq!(std::mem::size_of::<Packed<DNA, u8>>(), 1);
    /// ```
    fn from(x: Packed<DNA, u8>) -> DNA {
        unsafe { std::mem::transmute::<u8, DNA>(x.bits) }
    }
}


/*
pub trait Pack {
    fn pack() -> Packed<>;

    fn unpack(&self) -> Iter<A>;

    fn into_unpack(self) -> IntoIter<A>;

    fn unpack_mut(&mut self) -> IterMut<A>;

    fn map(self, f) -> Self;

    fn bits(&self) -> B;

    fn get(&self, index) -> A;
}
*/

