pub mod compressed_motif;
pub mod compressed_node_set;
pub mod fingerprint;
pub mod motifs;
pub mod sorting_network;

#[macro_use]
pub mod util;

#[allow(unused)]
pub mod test;

///         #labelings #classes
/// Order 3 12          6
/// Order 4 1990        171
/// Order 5 67_098_648  611_846
fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", test::compute_all_fingerprints::<4>()?);
    print!("{}", test::compute_all_fingerprints::<5>()?);

    Ok(())
}
