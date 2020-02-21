use std::marker::PhantomData;
use typenum::{UInt, Unsigned};

use crate::alphabet::Alphabet;

// See status of generic consts.
// https://github.com/rust-lang/rust/pull/53645




#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SimpleKmer<A, S, T=usize> {
    inner: T,
    size: PhantomData<S>,
    alphabet: PhantomData<A>,
}


impl<A, S, T> SimpleKmer<A, S, T>
where
    A: Alphabet,
    S: UInt + Unsigned,
{

    pub fn cardinality() -> usize {
        let size = A::cardinality();
        S::to_usize().pow(size as u32)
    }

    pub fn rank(&self) -> usize {
        self.inner as usize
    }

    pub fn from_rank(r: usize) -> Option<Self> {
        if r < Self::cardinality() {
            Some(Self {
                inner: r as T,
                size: PhantomData(),
                alphabet: PhantomData(),
            })
        } else {
            None
        }
    }

    // fn variants(&self) -> KmerIterator<Self>;
}

trait Kmer {}

trait RollingKmer {}

/*
 * seq.iter().kmers(U5);
 *
 * seq.iter().spaced_kmers(pattern, U5)
 *
 * let index: KmerIndex<K, > = seq.iter().kmers(U5).collect()
 * let counts: KmerCounts<K, > 
 *
 *
 *
 */
