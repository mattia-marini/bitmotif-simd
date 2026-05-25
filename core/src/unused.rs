// #[duplicate_item(
//     f_name                      e_bitset    n_bitset   max_hx_size;
//     [compute_constants_u32]     [ u32 ]     [ u8 ]     [4];
//     [compute_constants_u64]     [ u64 ]     [ u8 ]     [6];
// )]
// pub fn f_name<const N: usize>() -> ([e_bitset; 64], [n_bitset; 64]) {
//     // v[i] = all edges touched by the i hyperedge
//     let mut overlaps = [0 as e_bitset; 64];
//     // V[i] = all nodes in the i hyperedge
//     let mut nodes = [0 as n_bitset; 64];
//
//     // 2-edges
//     // let offset = N - 1;
//
//     let mut pivot_start = N - 1;
//     let mut scat_values = 1;
//     let mut msb = 1;
//
//     overlaps[0] = ((1 << (N - 1)) - 1);
//
//     println!("{:032b}", overlaps[0]);
//     let mut i = 1;
//     while i < N {
//         // println!("{:032b}", ((1 << (N - i))-1));
//         let fixed_values = ((1 << (N - i - 1)) - 1) << pivot_start;
//         // println!("{:032b}", scat_values | fixed_values);
//         overlaps[i] = scat_values | fixed_values;
//
//         msb = msb << (N - i);
//         scat_values = scat_values << 1 | msb;
//         pivot_start += N - i - 1;
//         println!("{:032b}", overlaps[i]);
//
//         i += 1;
//     }
//
//     // let x: e_bitset = 14;
//     (overlaps, nodes)
// }
// Defining type aliases for clarity (adjust u64/u128 depending on your capacity needs)
// type EBitset = u128;
// type NBitset = u64;
//
// /// Generates incidence bitsets for hyperedges of a specific order (uniformity).
// /// ORDER = 2 (standard edges), 3 (triplets), 4 (quads), 5 (quints).
// pub fn generate_hypergraph_masks<const N: usize, const ORDER: usize>()
// -> (Vec<EBitset>, Vec<NBitset>) {
//     // Calculate total number of hyperedges using Binomial Coefficient (N choose ORDER)
//     let total_edges = choose(N, ORDER);
//
//     // In a real scenario, you'd want dynamic allocation (Vec) because
//     // the number of edges grows explosively with the ORDER.
//     let mut overlaps = vec![0 as EBitset; total_edges];
//     let mut nodes = vec![0 as NBitset; total_edges];
//
//     let mut current_combination = Vec::with_capacity(ORDER);
//     let mut edge_index = 0;
//
//     // Helper closure to recursively find all combinations of nodes
//     fn find_combinations<const ORDER: usize>(
//         start_node: usize,
//         n: usize,
//         current: &mut Vec<usize>,
//         overlaps: &mut [EBitset],
//         nodes: &mut [NBitset],
//         edge_index: &mut usize,
//     ) {
//         if current.len() == ORDER {
//             // 1. Map nodes to this hyperedge
//             let mut node_mask: NBitset = 0;
//             for &node in current.iter() {
//                 node_mask |= 1 << node;
//             }
//             nodes[*edge_index] = node_mask;
//
//             // 2. Map which edges touch each other.
//             // For now, we flag that this hyperedge exists at its own index bit.
//             overlaps[*edge_index] |= 1 << *edge_index;
//
//             *edge_index += 1;
//             return;
//         }
//
//         for next_node in start_node..n {
//             current.push(next_node);
//             find_combinations::<ORDER>(next_node + 1, n, current, overlaps, nodes, edge_index);
//             current.pop(); // Backtrack
//         }
//     }
//
//     find_combinations::<ORDER>(
//         0,
//         N,
//         &mut current_combination,
//         &mut overlaps,
//         &mut nodes,
//         &mut edge_index,
//     );
//
//     // After generating the base nodes, we compute the structural overlaps.
//     // Two hyperedges overlap if they share AT LEAST ONE node.
//     for i in 0..total_edges {
//         for j in 0..total_edges {
//             if (nodes[i] & nodes[j]) != 0 {
//                 overlaps[i] |= 1 << j; // Edge i overlaps with Edge j
//             }
//         }
//     }
//
//     (overlaps, nodes)
// }
//
// // Simple compile-time safe binomial coefficient helper
// fn choose(n: usize, k: usize) -> usize {
//     if k > n {
//         return 0;
//     }
//     let mut res = 1;
//     for i in 0..k {
//         res = res * (n - i) / (i + 1);
//     }
//     res
// }
//
// pub trait SelectBitsetType<const N: usize> {
//     type Type: Copy + Default + Shl + BitOr + BitOrAssign + PrimInt + One + Zero;
// }
