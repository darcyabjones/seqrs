
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
    Gap, // -
}
