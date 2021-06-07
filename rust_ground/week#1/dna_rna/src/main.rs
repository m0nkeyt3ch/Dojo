fn main() {
    println!("result: {}", dna_to_rna("TTTTTTTT"));
}

fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}