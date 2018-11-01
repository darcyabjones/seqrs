/// Definitions for the Protein alphabet


use errors::SeqError;
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
    Lys, // Lysine, K
    Leu, // Leucine, L
    Met, // Methionine, M
    Asn, // Asparagine, N
    Pro, // Proline, P
    Gln, // Glutamine, Q
    Arg, // Arginine, R
    Ser, // Serine, S
    Thr, // Threonine, T
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
        match aa {
            b'A' | b'a' => Ok(AminoAcid::Ala),
            b'B' | b'b' => Ok(AminoAcid::Asx),
            b'C' | b'c' => Ok(AminoAcid::Cys),
            b'D' | b'd' => Ok(AminoAcid::Asp),
            b'E' | b'e' => Ok(AminoAcid::Glu),
            b'F' | b'f' => Ok(AminoAcid::Phe),
            b'G' | b'g' => Ok(AminoAcid::Gly),
            b'H' | b'h' => Ok(AminoAcid::His),
            b'I' | b'i' => Ok(AminoAcid::Ile),
            b'K' | b'k' => Ok(AminoAcid::Lys),
            b'L' | b'l' => Ok(AminoAcid::Leu),
            b'M' | b'm' => Ok(AminoAcid::Met),
            b'N' | b'n' => Ok(AminoAcid::Asn),
            b'P' | b'p' => Ok(AminoAcid::Pro),
            b'Q' | b'q' => Ok(AminoAcid::Gln),
            b'R' | b'r' => Ok(AminoAcid::Arg),
            b'S' | b's' => Ok(AminoAcid::Ser),
            b'T' | b't' => Ok(AminoAcid::Thr),
            b'V' | b'v' => Ok(AminoAcid::Val),
            b'W' | b'w' => Ok(AminoAcid::Trp),
            b'X' | b'x' => Ok(AminoAcid::Xaa),
            b'Y' | b'y' => Ok(AminoAcid::Tyr),
            b'Z' | b'z' => Ok(AminoAcid::Glx),
            b'*'        => Ok(AminoAcid::Stop),
            a           => Err(SeqError::AlphabetReadError { base: a as char }),
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
            AminoAcid::Lys  => b'K',
            AminoAcid::Leu  => b'L',
            AminoAcid::Met  => b'M',
            AminoAcid::Asn  => b'N',
            AminoAcid::Pro  => b'P',
            AminoAcid::Gln  => b'Q',
            AminoAcid::Arg  => b'R',
            AminoAcid::Ser  => b'S',
            AminoAcid::Thr  => b'T',
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
            (AminoAcid::Stop, AminoAcid::Stop) => true,
            (AminoAcid::Stop, _              ) => false,
            (              _, AminoAcid::Stop) => false,
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
            ( AminoAcid::Pro, AminoAcid::Pro ) => true,
            ( AminoAcid::Gln, AminoAcid::Gln ) => true,
            ( AminoAcid::Arg, AminoAcid::Arg ) => true,
            ( AminoAcid::Ser, AminoAcid::Ser ) => true,
            ( AminoAcid::Thr, AminoAcid::Thr ) => true,
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
