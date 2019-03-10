mod trans {
    use crate::alphabet::AA;
    use crate::alphabet::AA::*;
    use crate::alphabet::DNA4;
    use crate::codon::Codon;
    use crate::stopped::Stopped;
    use crate::stopped::Stopped::{Res, Stop, StopOr};
    use crate::translate::NCBITransTable;
    use crate::translate::NCBITransTable::*;
    use crate::translate::TranslationTable;

    impl<T> TranslationTable<Codon<DNA4>, Stopped<AA>> for NCBITransTable {
        fn get(&self, k: &T) -> Stopped<AA> {
            let index = k.rank();
            match self {
                Standard => CODONS_STANDARD[index],
                VertebrateMito => CODONS_VERTEBRATE_MITO[index],
                YeastMito => CODONS_YEAST_MITO[index],
                MoldProtozoanCoelenterateMito => CODONS_MOLD_PROTOZOAN_COELENTERATE_MITO[index],
                InvertebrateMito => CODONS_INVERTEBRATE_MITO[index],
                CiliateDasycladaceanHexamita => CODONS_CILIATE_DASYCLADACEAN_HEXAMITA[index],
                EchinodermFlatwormMito => CODONS_ECHINODERM_FLATWORM_MITO[index],
                Euplotid => CODONS_EUPLOTID[index],
                BacterialArchaealPlastid => CODONS_BACTERIAL_ARCHAEAL_PLASTID[index],
                AltYeast => CODONS_ALT_YEAST[index],
                AscidianMito => CODONS_ASCIDIAN_MITO[index],
                AltFlatwormMito => CODONS_ALT_FLATWORM_MITO[index],
                BlepharismaMacronuclear => CODONS_BLEPHARISMA_MACRONUCLEAR[index],
                ChlorophyceanMito => CODONS_CHLOROPHYCEAN_MITO[index],
                TrematodeMito => CODONS_TREMATODE_MITO[index],
                ScenedesmusMito => CODONS_SCENEDESMUS_MITO[index],
                ThraustochytriumMito => CODONS_THRAUSTOCHYTRIUM_MITO[index],
                PterobranchiaMito => CODONS_PTEROBRANCHIA_MITO[index],
                SR1Gracilibacteria => CODONS_S_R1_GRACILIBACTERIA[index],
                Pachysolen => CODONS_PACHYSOLEN[index],
                Karyorelict => CODONS_KARYORELICT[index],
                Condylostoma => CODONS_CONDYLOSTOMA[index],
                Mesodinium => CODONS_MESODINIUM[index],
                Peritrich => CODONS_PERITRICH[index],
                Blastocrithidia => CODONS_BLASTOCRITHIDIA[index],
                BalanophoraceaePlastid => CODONS_BALANOPHORACEAE_PLASTID[index],
                CephalodiscidaeMito => CODONS_CEPHALODISCIDAE_MITO[index],
            }
        }
    }

    // TABLES!
    const CODONS_STANDARD: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_VERTEBRATE_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Stop, Res(S), Stop, Res(S), Res(M), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_YEAST_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(M), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(T), Res(T), Res(T), Res(T), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_MOLD_PROTOZOAN_COELENTERATE_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_INVERTEBRATE_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(S), Res(S), Res(S), Res(S), Res(M), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_CILIATE_DASYCLADACEAN_HEXAMITA: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Res(Q), Res(Y), Res(Q), Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_ECHINODERM_FLATWORM_MITO: [Stopped<AA>; 64] = [Res(N), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(S), Res(S), Res(S), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_EUPLOTID: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(C), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_BACTERIAL_ARCHAEAL_PLASTID: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_ALT_YEAST: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(S), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_ASCIDIAN_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(G), Res(S), Res(G), Res(S), Res(M), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_ALT_FLATWORM_MITO: [Stopped<AA>; 64] = [Res(N), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(S), Res(S), Res(S), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Res(Y), Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_BLEPHARISMA_MACRONUCLEAR: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Res(Q), Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_CHLOROPHYCEAN_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Res(L), Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_TREMATODE_MITO: [Stopped<AA>; 64] = [Res(N), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(S), Res(S), Res(S), Res(S), Res(M), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_SCENEDESMUS_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Res(L), Res(Y), Stop, Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_THRAUSTOCHYTRIUM_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Stop, Res(F), Res(L), Res(F)];
    const CODONS_PTEROBRANCHIA_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(S), Res(S), Res(K), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_S_R1_GRACILIBACTERIA: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(G), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_PACHYSOLEN: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(A), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Stop, Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_KARYORELICT: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Res(Q), Res(Y), Res(Q), Res(Y), Res(S), Res(S), Res(S), Res(S), StopOr(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_CONDYLOSTOMA: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), StopOr(Q), Res(Y), StopOr(Q), Res(Y), Res(S), Res(S), Res(S), Res(S), StopOr(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_MESODINIUM: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Res(Y), Res(Y), Res(Y), Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_PERITRICH: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Res(E), Res(Y), Res(E), Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_BLASTOCRITHIDIA: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), StopOr(E), Res(Y), StopOr(E), Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_BALANOPHORACEAE_PLASTID: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(R), Res(S), Res(R), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), StopOr(W), Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Stop, Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
    const CODONS_CEPHALODISCIDAE_MITO: [Stopped<AA>; 64] = [Res(K), Res(N), Res(K), Res(N), Res(T), Res(T), Res(T), Res(T), Res(S), Res(S), Res(K), Res(S), Res(I), Res(I), Res(M), Res(I), Res(Q), Res(H), Res(Q), Res(H), Res(P), Res(P), Res(P), Res(P), Res(R), Res(R), Res(R), Res(R), Res(L), Res(L), Res(L), Res(L), Res(E), Res(D), Res(E), Res(D), Res(A), Res(A), Res(A), Res(A), Res(G), Res(G), Res(G), Res(G), Res(V), Res(V), Res(V), Res(V), Res(Y), Res(Y), Stop, Res(Y), Res(S), Res(S), Res(S), Res(S), Res(W), Res(C), Res(W), Res(C), Res(L), Res(F), Res(L), Res(F)];
}

mod tags {
    use crate::alphabet::CodonTag;
    use crate::alphabet::CodonTag::*;
    use crate::alphabet::DNA4;
    use crate::codon::Codon;
    use crate::translate::CodonTagTable;
    use crate::translate::NCBITransTable;
    use crate::translate::NCBITransTable::*;

    impl CodonTagTable<Codon<DNA4>, CodonTag> for NCBITransTable {
        fn get_tag(&self, k: &Codon<DNA4>) -> CodonTag {
            let index = k.rank();
            match self {
                Standard => TAGS_STANDARD[index],
                VertebrateMito => TAGS_VERTEBRATE_MITO[index],
                YeastMito => TAGS_YEAST_MITO[index],
                MoldProtozoanCoelenterateMito => TAGS_MOLD_PROTOZOAN_COELENTERATE_MITO[index],
                InvertebrateMito => TAGS_INVERTEBRATE_MITO[index],
                CiliateDasycladaceanHexamita => TAGS_CILIATE_DASYCLADACEAN_HEXAMITA[index],
                EchinodermFlatwormMito => TAGS_ECHINODERM_FLATWORM_MITO[index],
                Euplotid => TAGS_EUPLOTID[index],
                BacterialArchaealPlastid => TAGS_BACTERIAL_ARCHAEAL_PLASTID[index],
                AltYeast => TAGS_ALT_YEAST[index],
                AscidianMito => TAGS_ASCIDIAN_MITO[index],
                AltFlatwormMito => TAGS_ALT_FLATWORM_MITO[index],
                BlepharismaMacronuclear => TAGS_BLEPHARISMA_MACRONUCLEAR[index],
                ChlorophyceanMito => TAGS_CHLOROPHYCEAN_MITO[index],
                TrematodeMito => TAGS_TREMATODE_MITO[index],
                ScenedesmusMito => TAGS_SCENEDESMUS_MITO[index],
                ThraustochytriumMito => TAGS_THRAUSTOCHYTRIUM_MITO[index],
                PterobranchiaMito => TAGS_PTEROBRANCHIA_MITO[index],
                SR1Gracilibacteria => TAGS_S_R1_GRACILIBACTERIA[index],
                Pachysolen => TAGS_PACHYSOLEN[index],
                Karyorelict => TAGS_KARYORELICT[index],
                Condylostoma => TAGS_CONDYLOSTOMA[index],
                Mesodinium => TAGS_MESODINIUM[index],
                Peritrich => TAGS_PERITRICH[index],
                Blastocrithidia => TAGS_BLASTOCRITHIDIA[index],
                BalanophoraceaePlastid => TAGS_BALANOPHORACEAE_PLASTID[index],
                CephalodiscidaeMito => TAGS_CEPHALODISCIDAE_MITO[index],
            }
        }
    }


    const TAGS_STANDARD: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_VERTEBRATE_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Stop, Res, Start, Start, Start, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_YEAST_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_MOLD_PROTOZOAN_COELENTERATE_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Start, Start, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Start, Res];
    const TAGS_INVERTEBRATE_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Start, Start, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_CILIATE_DASYCLADACEAN_HEXAMITA: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_ECHINODERM_FLATWORM_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_EUPLOTID: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_BACTERIAL_ARCHAEAL_PLASTID: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Start, Start, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_ALT_YEAST: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_ASCIDIAN_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_ALT_FLATWORM_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_BLEPHARISMA_MACRONUCLEAR: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_CHLOROPHYCEAN_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_TREMATODE_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_SCENEDESMUS_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Stop, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_THRAUSTOCHYTRIUM_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Stop, Res, Res, Res];
    const TAGS_PTEROBRANCHIA_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_S_R1_GRACILIBACTERIA: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_PACHYSOLEN: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Stop, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_KARYORELICT: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, StopRes, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_CONDYLOSTOMA: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, StopRes, Res, StopRes, Res, Res, Res, Res, Res, StopRes, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_MESODINIUM: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_PERITRICH: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_BLASTOCRITHIDIA: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, StopRes, Res, StopRes, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res];
    const TAGS_BALANOPHORACEAE_PLASTID: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Start, Start, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, StopRes, Res, Stop, Res, Res, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Start, Res];
    const TAGS_CEPHALODISCIDAE_MITO: [CodonTag; 64] = [Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res, Res, Res, Stop, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Res, Start, Res];
}

#[cfg(test)]
mod tests {
    use crate::alphabet::DNA4;
    use crate::codon::Codon;
    use crate::translate::CodonTagTable;
    use crate::translate::NCBITransTable;
    use crate::translate::TranslationTable;
    use crate::alphabet::Alphabet;

    use proptest::sample::select;
    use proptest::{proptest, proptest_helper};

    proptest! {
        #[test]
        fn test_trans_table_get_doesnt_crash(
            b1 in select(DNA4::variants()),
            b2 in select(DNA4::variants()),
            b3 in select(DNA4::variants()),
            table in select(NCBITransTable::variants()),
        ) {
            let codon = Codon(b1, b2, b3);
            let _dummy = table.get(&codon);
        }

        #[test]
        fn test_tag_table_get_tag_doesnt_crash(
            b1 in select(DNA4::variants()),
            b2 in select(DNA4::variants()),
            b3 in select(DNA4::variants()),
            table in select(NCBITransTable::variants()),
        ) {
            let codon = Codon(b1, b2, b3);
            let _dummy = table.get_tag(&codon);
        }
    }
}
