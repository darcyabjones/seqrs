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
use std::convert::TryFrom;

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


pub trait BitPack<A, I=u32, E=BigEndian> {
    const CAPACITY: usize;
    const NBITS: usize;
    const MASK: usize;

    // These should probably be unsafe unless we do bounds checks.
    fn get(&self, index: usize) -> A;

    fn set(&mut self, index: usize, value: A);
}


/// # Examples:
///
/// ```
/// use seqrs::packed::Packed;
/// use seqrs::packed::BitPack;
/// use seqrs::alphabet::DNA;
///
/// let mut b: Packed<DNA, u8> = Packed::from([DNA::N, DNA::N]);
///
/// b.set(0, DNA::A);
/// assert_eq!(b.get(0), DNA::A);
///
/// b.set(1, DNA::T);
///
/// assert_eq!(b.get(1), DNA::T);
/// assert_eq!(b.get(0), DNA::A);
/// ```
impl BitPack<DNA, u8> for Packed<DNA, u8> {
    const CAPACITY: usize = 2;
    const NBITS: usize = 4;
    const MASK: usize = 0b00001111;

    fn get(&self, index: usize) -> DNA {
        debug_assert!(index < Self::CAPACITY);

        let bitshift = index * Self::NBITS;
        let mask: u8 = (Self::MASK << bitshift) as u8;
        let i: u8 = (self.bits & mask) >> bitshift;

        debug_assert!(i != 0);
        unsafe { std::mem::transmute::<u8, DNA>(i) }
    }

    fn set(&mut self, index: usize, value: DNA) {
        debug_assert!(index < Self::CAPACITY);

        let bitshift = index * Self::NBITS;
        let shifted_val = (value as u8) << bitshift;
        let shifted_mask = (Self::MASK as u8) << bitshift;
        self.bits = (self.bits & !shifted_mask) | (shifted_val & shifted_mask);
    }
}


impl From<DNA> for Packed<DNA, u8> {

    /// # Examples:
    ///
    /// ```
    /// use seqrs::packed::Packed;
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
    /// use seqrs::packed::Packed;
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


impl From<[DNA; 2]> for Packed<DNA, u8> {

    /// # Examples:
    ///
    /// ```
    /// use seqrs::packed::Packed;
    /// use seqrs::alphabet::DNA;
    ///
    /// let b: Packed<DNA, u8> = [DNA::A, DNA::C].into();
    ///
    /// assert_eq!(b.bits, 0b00100001);
    /// ```
    fn from(x: [DNA; 2]) -> Packed<DNA, u8> {
        Packed {
            bits: (x[0] as u8) | (x[1] as u8) << 4,
            _alphabet: PhantomData,
            _endian: PhantomData,
        }
    }
}


impl From<Packed<DNA, u8>> for [DNA; 2] {

    /// # Examples:
    ///
    /// ```
    /// use seqrs::packed::Packed;
    /// use seqrs::alphabet::DNA;
    ///
    /// let b: Packed<DNA, u8> = [DNA::A, DNA::C].into();
    /// let c: [DNA; 2] = b.into();
    ///
    /// assert_eq!(c, [DNA::A, DNA::C]);
    ///
    /// assert_eq!(std::mem::size_of::<Packed<DNA, u8>>(), 1);
    /// ```
    fn from(x: Packed<DNA, u8>) -> [DNA; 2] {
        let mask = 0b1111;
        let one = x.bits & mask;
        let two = (x.bits & mask << 4) >> 4;
        [unsafe { std::mem::transmute::<u8, DNA>(one) },
         unsafe { std::mem::transmute::<u8, DNA>(two) }]
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

