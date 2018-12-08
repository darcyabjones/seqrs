/// Definitions for the Protein alphabet

use errors::SeqError;
use charcase::to_upper;
use std::convert::TryFrom;

/// Proteins are represented as an enum.
#[derive(Debug, Clone, Copy, Hash)]
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

/// Read AAs from bytes.
impl TryFrom<u8> for AminoAcid {
    type Error = SeqError;

    fn try_from(aa: u8) -> Result<Self, Self::Error> {
        match to_upper(aa) {
            b'A' => Ok(AminoAcid::Ala),
            b'B' => Ok(AminoAcid::Asx),
            b'C' => Ok(AminoAcid::Cys),
            b'D' => Ok(AminoAcid::Asp),
            b'E' => Ok(AminoAcid::Glu),
            b'F' => Ok(AminoAcid::Phe),
            b'G' => Ok(AminoAcid::Gly),
            b'H' => Ok(AminoAcid::His),
            b'I' => Ok(AminoAcid::Ile),
            b'J' => Ok(AminoAcid::Xle),
            b'K' => Ok(AminoAcid::Lys),
            b'L' => Ok(AminoAcid::Leu),
            b'M' => Ok(AminoAcid::Met),
            b'N' => Ok(AminoAcid::Asn),
            b'O' => Ok(AminoAcid::Pyr),
            b'P' => Ok(AminoAcid::Pro),
            b'Q' => Ok(AminoAcid::Gln),
            b'R' => Ok(AminoAcid::Arg),
            b'S' => Ok(AminoAcid::Ser),
            b'T' => Ok(AminoAcid::Thr),
            b'U' => Ok(AminoAcid::Sel),
            b'V' => Ok(AminoAcid::Val),
            b'W' => Ok(AminoAcid::Trp),
            b'X' => Ok(AminoAcid::Xaa),
            b'Y' => Ok(AminoAcid::Tyr),
            b'Z' => Ok(AminoAcid::Glx),
            b'*' => Ok(AminoAcid::Stop),
            a    => Err(SeqError::AlphabetReadError { base: a as char }),
        }
    }
}


/// Convert amino-acid to byte
impl From<AminoAcid> for u8 {
    fn from(aa: AminoAcid) -> Self {
        match aa {
            AminoAcid::Ala  => b'A',
            AminoAcid::Asx  => b'B',
            AminoAcid::Cys  => b'C',
            AminoAcid::Asp  => b'D',
            AminoAcid::Glu  => b'E',
            AminoAcid::Phe  => b'F',
            AminoAcid::Gly  => b'G',
            AminoAcid::His  => b'H',
            AminoAcid::Ile  => b'I',
            AminoAcid::Xle  => b'J',
            AminoAcid::Lys  => b'K',
            AminoAcid::Leu  => b'L',
            AminoAcid::Met  => b'M',
            AminoAcid::Asn  => b'N',
            AminoAcid::Pyr  => b'O',
            AminoAcid::Pro  => b'P',
            AminoAcid::Gln  => b'Q',
            AminoAcid::Arg  => b'R',
            AminoAcid::Ser  => b'S',
            AminoAcid::Thr  => b'T',
            AminoAcid::Sel  => b'U',
            AminoAcid::Val  => b'V',
            AminoAcid::Trp  => b'W',
            AminoAcid::Xaa  => b'X',
            AminoAcid::Tyr  => b'Y',
            AminoAcid::Glx  => b'Z',
            AminoAcid::Stop => b'*',
        }
    }
}


impl PartialEq for AminoAcid {
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


impl Eq for AminoAcid {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(AminoAcid::try_from(b'A').unwrap(), AminoAcid::Ala);
        assert_eq!(AminoAcid::try_from(b'C').unwrap(), AminoAcid::Cys);
        assert_eq!(AminoAcid::try_from(b'Z').unwrap(), AminoAcid::Glx);
        assert_eq!(AminoAcid::try_from(b'G').unwrap(), AminoAcid::Gly);
    }
}
