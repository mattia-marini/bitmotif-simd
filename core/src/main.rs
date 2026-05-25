pub mod compressed_motif;
pub mod compressed_motif2;
pub mod fingerprint;
pub mod motifs;
pub mod util;

#[allow(unused)]
pub mod test;

///         #labelings #classes
/// Order 3 12          6
/// Order 4 1990        171
/// Order 5 67_098_648  565_464
fn main() {
    test::order4();
    // for p in Itertools::permutations(vec![0, 1, 2, 3]) {
    //     println!("{}", p);
    // }
    // test::order5();
}
