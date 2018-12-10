/// Definitions for the Protein alphabet

use errors::SeqError;
use std::convert::TryFrom;

/// Proteins are represented as an enum.
#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum AminoAcid {
    Ala, // Alanine, A
    Asx, // Aspartic acid or Asparagine, B
    Cys, // Cysteine, C
    Asp, // Aspartic acid, D
    Glu, // Glutamic acid, E
    Phe, // Phenylalanine, F
    Gly, // Glycine, G
    His, // Histidine, H
    Ile, // Isoleucine, I
    Xle, // Isoleucine or leucine, J
    Lys, // Lysine, K
    Leu, // Leucine, L
    Met, // Methionine, M
    Asn, // Asparagine, N
    Pyr, // Pyroleucine, O non-standard
    Pro, // Proline, P
    Gln, // Glutamine, Q
    Arg, // Arginine, R
    Ser, // Serine, S
    Thr, // Threonine, T
    Sel, // Selenocysteine, U non-standard
    Val, // Valine, V
    Trp, // Tryptophan, W
    Xaa, // Any amino acid, X
    Tyr, // Tyrosine, Y
    Glx, // Glutamine or Glutamic acid, Z
    Stop, // *
}

/// Use try from because it's possible to receive invalid input.
impl TryFrom<char> for AminoAcid {
    type Error = SeqError;

    /// Convert a char to an AminoAcid enum variant.
    /// Returns `Err` if character is invalid.
    ///
    /// # Examples:
    ///
    /// WARNING: try_from is currently unstable, so this example cannot be
    /// tested.
    ///
    /// ```rust,ignore
    /// use seqrs::alphabets::AminoAcid;
    /// use std::convert::{TryFrom, TryInto};
    ///
    /// let aa = AminoAcid::try_from('A').unwrap();
    /// assert_eq!(aa, AminoAcid::Ala);
    ///
    /// // Bytes can be automatically converted into chars.
    /// let aa = AminoAcid::try_from(b'A').unwrap();
    /// assert_eq!(aa, AminoAcid::Ala);
    /// ```
    fn try_from(aa: char) -> Result<Self, Self::Error> {
        match aa.to_ascii_uppercase() {
            'A' => Ok(AminoAcid::Ala),
            'B' => Ok(AminoAcid::Asx),
            'C' => Ok(AminoAcid::Cys),
            'D' => Ok(AminoAcid::Asp),
            'E' => Ok(AminoAcid::Glu),
            'F' => Ok(AminoAcid::Phe),
            'G' => Ok(AminoAcid::Gly),
            'H' => Ok(AminoAcid::His),
            'I' => Ok(AminoAcid::Ile),
            'J' => Ok(AminoAcid::Xle),
            'K' => Ok(AminoAcid::Lys),
            'L' => Ok(AminoAcid::Leu),
            'M' => Ok(AminoAcid::Met),
            'N' => Ok(AminoAcid::Asn),
            'O' => Ok(AminoAcid::Pyr),
            'P' => Ok(AminoAcid::Pro),
            'Q' => Ok(AminoAcid::Gln),
            'R' => Ok(AminoAcid::Arg),
            'S' => Ok(AminoAcid::Ser),
            'T' => Ok(AminoAcid::Thr),
            'U' => Ok(AminoAcid::Sel),
            'V' => Ok(AminoAcid::Val),
            'W' => Ok(AminoAcid::Trp),
            'X' => Ok(AminoAcid::Xaa),
            'Y' => Ok(AminoAcid::Tyr),
            'Z' => Ok(AminoAcid::Glx),
            '*' => Ok(AminoAcid::Stop),
            a   => Err(SeqError::AlphabetReadError { base: a }),
        }
    }
}


impl From<AminoAcid> for char {

    /// Convert AminoAcid to char.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabets::AminoAcid;
    /// use std::convert::{From, Into};
    ///
    /// assert_eq!(char::from(AminoAcid::Ala), 'A');
    ///
    /// // Into is also implicitly defined.
    /// let aa: char = AminoAcid::Ala.into();
    /// assert_eq!(aa, 'A');
    ///
    /// let aa: u8 = AminoAcid::Ala.into();
    /// assert_eq!(aa, b'A');
    /// ```
    fn from(aa: AminoAcid) -> Self {
        match aa {
            AminoAcid::Ala  => 'A',
            AminoAcid::Asx  => 'B',
            AminoAcid::Cys  => 'C',
            AminoAcid::Asp  => 'D',
            AminoAcid::Glu  => 'E',
            AminoAcid::Phe  => 'F',
            AminoAcid::Gly  => 'G',
            AminoAcid::His  => 'H',
            AminoAcid::Ile  => 'I',
            AminoAcid::Xle  => 'J',
            AminoAcid::Lys  => 'K',
            AminoAcid::Leu  => 'L',
            AminoAcid::Met  => 'M',
            AminoAcid::Asn  => 'N',
            AminoAcid::Pyr  => 'O',
            AminoAcid::Pro  => 'P',
            AminoAcid::Gln  => 'Q',
            AminoAcid::Arg  => 'R',
            AminoAcid::Ser  => 'S',
            AminoAcid::Thr  => 'T',
            AminoAcid::Sel  => 'U',
            AminoAcid::Val  => 'V',
            AminoAcid::Trp  => 'W',
            AminoAcid::Xaa  => 'X',
            AminoAcid::Tyr  => 'Y',
            AminoAcid::Glx  => 'Z',
            AminoAcid::Stop => '*',
        }
    }
}

/// Tryfrom for byte.
impl TryFrom<u8> for AminoAcid {
    type Error = SeqError;

    /// Convert a u8 to an AminoAcid enum variant.
    /// Returns `Err` if character is invalid.
    ///
    /// # Examples:
    ///
    /// WARNING: try_from is currently unstable, so this example cannot be
    /// tested.
    ///
    /// ```rust,ignore
    /// use seqrs::alphabets::AminoAcid;
    /// use std::convert::{TryFrom, TryInto};
    ///
    /// let aa = AminoAcid::try_from(b'A').unwrap();
    /// assert_eq!(aa, AminoAcid::Ala);
    /// ```
    fn try_from(aa: u8) -> Result<Self, Self::Error> {
        Self::try_from(aa as char)
    }
}


impl From<AminoAcid> for u8 {

    /// Convert AminoAcid to u8.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabets::AminoAcid;
    /// use std::convert::{From, Into};
    ///
    /// assert_eq!(u8::from(AminoAcid::Ala), b'A');
    ///
    /// // Into is also implicitly defined.
    /// let aa: u8 = AminoAcid::Ala.into();
    /// assert_eq!(aa, b'A');
    /// ```
    fn from(aa: AminoAcid) -> Self {
        // Casting char -> u32 -> u8 is safe in this instance because we
        // control what `char`s it could be.
        (char::from(aa) as u32) as u8
    }
}

impl PartialEq for AminoAcid {

    /// Define equality for AminoAcids.
    /// For redundant AAs, if the sets intersect then evaluates as true.
    ///
    /// # Examples:
    ///
    /// ```
    /// use seqrs::alphabets::AminoAcid;
    ///
    /// assert!(AminoAcid::Ala == AminoAcid::Ala);
    /// assert_eq!(AminoAcid::Ala, AminoAcid::Ala);
    /// assert_eq!(AminoAcid::Ala, AminoAcid::Xaa);
    /// assert_eq!(AminoAcid::Asp, AminoAcid::Asx); // Asx == Asp | Asn
    ///
    /// // != is implicitly defined.
    /// assert_ne!(AminoAcid::Ala, AminoAcid::Asp);
    /// assert_ne!(AminoAcid::Asx, AminoAcid::Glx);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            ( AminoAcid::Ala, AminoAcid::Ala ) => true,
            ( AminoAcid::Asx, AminoAcid::Asx ) => true,
            ( AminoAcid::Cys, AminoAcid::Cys ) => true,
            ( AminoAcid::Asp, AminoAcid::Asp ) => true,
            ( AminoAcid::Glu, AminoAcid::Glu ) => true,
            ( AminoAcid::Phe, AminoAcid::Phe ) => true,
            ( AminoAcid::Gly, AminoAcid::Gly ) => true,
            ( AminoAcid::His, AminoAcid::His ) => true,
            ( AminoAcid::Ile, AminoAcid::Ile ) => true,
            ( AminoAcid::Lys, AminoAcid::Lys ) => true,
            ( AminoAcid::Leu, AminoAcid::Leu ) => true,
            ( AminoAcid::Met, AminoAcid::Met ) => true,
            ( AminoAcid::Asn, AminoAcid::Asn ) => true,
            ( AminoAcid::Pyr, AminoAcid::Pyr ) => true,
            ( AminoAcid::Pro, AminoAcid::Pro ) => true,
            ( AminoAcid::Gln, AminoAcid::Gln ) => true,
            ( AminoAcid::Arg, AminoAcid::Arg ) => true,
            ( AminoAcid::Ser, AminoAcid::Ser ) => true,
            ( AminoAcid::Thr, AminoAcid::Thr ) => true,
            ( AminoAcid::Sel, AminoAcid::Sel ) => true,
            ( AminoAcid::Val, AminoAcid::Val ) => true,
            ( AminoAcid::Trp, AminoAcid::Trp ) => true,
            ( AminoAcid::Xaa, AminoAcid::Xaa ) => true,
            ( AminoAcid::Tyr, AminoAcid::Tyr ) => true,
            ( AminoAcid::Glx, AminoAcid::Glx ) => true,
            ( AminoAcid::Glx, AminoAcid::Glu ) => true,
            ( AminoAcid::Glx, AminoAcid::Gln ) => true,
            ( AminoAcid::Glu, AminoAcid::Glx ) => true,
            ( AminoAcid::Gln, AminoAcid::Glx ) => true,
            ( AminoAcid::Asx, AminoAcid::Asp ) => true,
            ( AminoAcid::Asx, AminoAcid::Asn ) => true,
            ( AminoAcid::Asp, AminoAcid::Asx ) => true,
            ( AminoAcid::Asn, AminoAcid::Asx ) => true,
            ( AminoAcid::Xle, AminoAcid::Xle ) => true,
            ( AminoAcid::Xle, AminoAcid::Ile ) => true,
            ( AminoAcid::Xle, AminoAcid::Leu ) => true,
            ( AminoAcid::Ile, AminoAcid::Xle ) => true,
            ( AminoAcid::Leu, AminoAcid::Xle ) => true,
            (AminoAcid::Stop, AminoAcid::Stop) => true,
            (AminoAcid::Stop, _              ) => false,
            (              _, AminoAcid::Stop) => false,
            ( AminoAcid::Xaa, _              ) => true,
            (              _, AminoAcid::Xaa ) => true,
            _                                  => false,
        }
    }
}

/// Uses definition from `PartialEq`.
impl Eq for AminoAcid {}

impl Default for AminoAcid {
    /// Returns [`Xaa`][AminoAcid::Xaa].
    #[inline]
    fn default() -> Self { AminoAcid::Xaa }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(AminoAcid::try_from('A').unwrap(), AminoAcid::Ala);
        assert_eq!(AminoAcid::try_from('C').unwrap(), AminoAcid::Cys);
        assert_eq!(AminoAcid::try_from('Z').unwrap(), AminoAcid::Glx);
        assert_eq!(AminoAcid::try_from('G').unwrap(), AminoAcid::Gly);

        assert_eq!(AminoAcid::try_from(b'G').unwrap(), AminoAcid::Gly);
    }
}
