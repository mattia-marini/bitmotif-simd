// pub mod compressed_motif;
pub mod compressed_motif2;
pub mod compressed_node_set;
pub mod fingerprint;
pub mod motifs;
pub mod sorting_network;
pub mod util;

#[allow(unused)]
pub mod test;

///         #labelings #classes
/// Order 3 12          6
/// Order 4 1990        171
/// Order 5 67_098_648  565_464
fn main() -> Result<(), Box<dyn std::error::Error>> {
    test::order5()?;

    Ok(())
}
