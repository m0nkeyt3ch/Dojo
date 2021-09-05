fn main() {
    assert_eq!(dna_to_rna("TTTT"), "UUUU");
    assert_eq!(dna_to_rna("GCAT"), "GCAU");
    println!("result: {}", dna_to_rna("TTTTTTTT"));
}

fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}