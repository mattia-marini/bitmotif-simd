use std::{fmt::Display, ops::BitAnd};

use duplicate::duplicate_item;
use macros::hoist_mod;
use seq_macro::seq;

use crate::fingerprint::{Fingerprint4, Fingerprint5};

pub trait BitContainer {
    type ContainerType;
}

#[derive(Copy, Clone)]
pub struct CompressedNodeSet {
    pub nodes: u8,
}

impl CompressedNodeSet {
    pub fn new(nodes: u8) -> Self {
        Self { nodes }
    }

    pub fn len(&self) -> u32 {
        self.nodes.count_ones()
    }

    pub fn iter(&self) -> CompressetNodeSetIter {
        CompressetNodeSetIter {
            remaining_nodes: self.nodes,
        }
    }

    pub fn contains(&self, node: usize) -> bool {
        (self.nodes & (1 << node)) != 0
    }
}

pub struct CompressetNodeSetIter {
    remaining_nodes: u8,
}


impl Iterator for CompressetNodeSetIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_nodes == 0 {
            None
        } else {
            let index = self.remaining_nodes.trailing_zeros() as usize;

            // 2. Clear the lowest set bit
            self.remaining_nodes &= self.remaining_nodes - 1;

            Some(index)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.remaining_nodes.count_ones() as usize;
        (count, Some(count))
    }
}

impl IntoIterator for CompressedNodeSet {
    type Item = usize;
    type IntoIter = CompressetNodeSetIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        CompressetNodeSetIter {
            remaining_nodes: self.nodes,
        }
    }
}

#[hoist_mod(
attr(duplicate_item(
    struct_name            iter_name                fingerprint_name        const_f              e_bitset  size     e_count;
    // [CompressedMotif2]     [CompressedMotif2Iter]   [Fingerprint2]  [generate_bitmask2]  [u8]      [2]      [1];
    // [CompressedMotif3]     [CompressedMotif3Iter]   [Fingerprint3]  [generate_bitmask3]  [u8]      [3]      [4];      
    [CompressedMotif4]     [CompressedMotif4Iter]   [Fingerprint4]  [generate_bitmask4]  [u16]     [4]      [11];      
    [CompressedMotif5]     [CompressedMotif5Iter]   [Fingerprint5]  [generate_bitmask5]  [u32]     [5]      [26];    
    // [CompressedMotif6]     [CompressedMotif6Iter]   [Fingerprint6]  [generate_bitmask6]  [u64]     [6]      [57];      
))
)]
mod __ {

    #[derive(Copy, Clone)]
    pub struct struct_name {
        pub edges: e_bitset,
    }

    impl struct_name {
        pub const SIZE: usize = size;
        pub const MAX_EDGE_COUNT: usize = e_count;

        // Uses your macro-generated struct names and CompressedNodeSet directly
        pub const ADJ: [struct_name; size] = const_f().0;
        pub const FULL_OVERLAPS: [struct_name; e_count] = const_f().1;
        pub const PART_OVERLAPS: [struct_name; e_count] = const_f().2;
        pub const NODE_MAP: [CompressedNodeSet; e_count] = const_f().3;

        pub fn new(edges: e_bitset) -> Self {
            Self { edges }
        }

        pub fn iter(&self) -> iter_name {
            iter_name {
                remaining_edges: self.edges,
            }
        }

        pub fn fingerprint(&self) -> fingerprint_name {
            fingerprint_name::from_cm(self)
        }

        pub fn enum_labelings<F>(show_progress: bool, mut f: F)
        where
            F: FnMut(struct_name),
        {
            let is_connected = |motif: &e_bitset| -> bool {
                if *motif == 0 {
                    return false;
                }

                // 1. Quick Spanning Check: Ensure the motif covers all 'size' nodes
                let mut covered_nodes = 0u8;
                let mut e_iter = *motif;
                while e_iter != 0 {
                    let e = e_iter.trailing_zeros() as usize;
                    e_iter &= e_iter - 1;
                    covered_nodes |= Self::NODE_MAP[e].nodes;
                }
                if covered_nodes != (1 << size) - 1 {
                    return false;
                }

                // 2. Connectivity Check: Run a bitwise BFS to ensure all edges form 1 component
                let first_edge = motif.trailing_zeros() as usize;
                let mut visited_edges = (1 as e_bitset) << first_edge;
                let mut queue = (1 as e_bitset) << first_edge;

                while queue != 0 {
                    let e = queue.trailing_zeros() as usize;
                    queue &= queue - 1;

                    // Get neighbors that are in the motif but haven't been visited yet
                    let neighbors = Self::PART_OVERLAPS[e].edges & *motif & !visited_edges;
                    visited_edges |= neighbors;
                    queue |= neighbors;
                }

                // If we visited every edge in the motif, it is fully connected
                visited_edges == *motif
            };

            let motif_count = 1 << Self::MAX_EDGE_COUNT;

            let block_size = motif_count / 100;
            let mut curr_progress = 0;
            let mut block_progress = 0;

            for motif in (0 as e_bitset)..motif_count {
                // print_motif(&motif);
                if is_connected(&motif) {
                    let motif = struct_name::new(motif);
                    f(motif);
                }

                if show_progress {
                    if block_progress < block_size {
                        block_progress += 1;
                    } else {
                        block_progress = 0;
                        curr_progress += 1;
                        println!("Progress: {}%", curr_progress);
                    }
                }
            }
        }

        pub fn to_vec(&self) -> Vec<Vec<usize>> {
            let mut edges = Vec::new();
            for e in self {
                let mut nodes = Vec::with_capacity(8);
                for n in Self::NODE_MAP[e] {
                    nodes.push(n);
                }
                edges.push(nodes);
            }
            edges
        }
    }

    impl BitContainer for struct_name {
        type ContainerType = e_bitset;
    }

    impl BitAnd for struct_name {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            Self {
                edges: self.edges & rhs.edges,
            }
        }
    }

    impl Display for struct_name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut edges = Vec::new();
            for e in self {
                let mut nodes = Vec::with_capacity(8);
                for n in Self::NODE_MAP[e] {
                    nodes.push(n);
                }
                edges.push(nodes);
            }
            f.write_str(format!("{:?}", edges).as_str())
        }
    }

    pub struct iter_name {
        remaining_edges: e_bitset,
    }

    impl Iterator for iter_name {
        type Item = usize;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining_edges == 0 {
                None
            } else {
                // 1. Grab the index of the lowest set bit
                let index = self.remaining_edges.trailing_zeros() as usize;

                // 2. Clear the lowest set bit blazingly fast via bitwise AND
                self.remaining_edges &= self.remaining_edges - 1;

                Some(index)
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let count = self.remaining_edges.count_ones() as usize;
            (count, Some(count))
        }
    }

    impl IntoIterator for struct_name {
        type Item = usize;
        type IntoIter = iter_name;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            iter_name {
                remaining_edges: self.edges,
            }
        }
    }

    impl<'a> IntoIterator for &'a struct_name {
        type Item = usize;
        type IntoIter = iter_name;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            iter_name {
                remaining_edges: self.edges,
            }
        }
    }
}


// pub trait CompactMotifContainer{
//     type ContainerType;
//     const MAX_EDGE_COUNT: usize;
//     const SIZE: usize;
//
//     const ADJ: [struct_name; size] = const_f().0;
//     const FULL_OVERLAPS: [struct_name; e_count] = const_f().1;
//     const PART_OVERLAPS: [struct_name; e_count] = const_f().2;
//     const NODE_MAP: [CompressedNodeSet; e_count] = const_f().3;
// }
//
//
// impl CompactMotifContainer for CompactMotif<2>{
//     type ContainerType = u8;
//     const MAX_EDGE_COUNT: usize = 1;
//     const SIZE: usize = 2;
// }
//
// struct CompactMotif<const N: usize>
//     where Self: CompactMotifContainer
//     {
//     container: <Self as CompactMotifContainer>::ContainerType,
// }



// struct MotifContainer<const N: usize, T>{
//     container: T,
// }

#[duplicate_item(
    f_name                struct_name          size      e_bitset    e_count;
    // [generate_bitmask2]   [CompressedMotif2]   [2]       [u8]        [1];
    // [generate_bitmask3]   [CompressedMotif3]   [3]       [u8]        [4];
    [generate_bitmask4]   [CompressedMotif4]   [4]       [u16]       [11];
    [generate_bitmask5]   [CompressedMotif5]   [5]       [u32]       [26];
    // [generate_bitmask6]   [CompressedMotif6]   [6]       [u64]       [57];
)]
pub const fn f_name() -> (
    [struct_name; size],
    [struct_name; e_count],
    [struct_name; e_count],
    [CompressedNodeSet; e_count],
) {
    const fn build_adj<const N: usize>(
        mut pos: usize,
        level: usize,
        rv: &mut [u8; N],
        adj: &mut [e_bitset; size],
        edge_count: &mut usize,
    ) {
        if level == N {
            let mut i = 0;
            while i < N {
                adj[rv[i] as usize] |= 1 << *edge_count;
                i += 1;
            }

            *edge_count += 1;
            return;
        }
        while pos < size - (N - level - 1) {
            rv[level] = pos as u8;
            build_adj(pos + 1, level + 1, rv, adj, edge_count);
            pos += 1;
        }
    }

    const fn build_full_overlaps<const N: usize>(
        mut pos: usize,
        level: usize,
        rv: &mut [u8; N],
        adj: &[e_bitset; size],
        full_overlaps: &mut [e_bitset; e_count],
        edge_count: &mut usize,
    ) {
        if level == N {
            let mut node_set: u8 = 0;
            let mut i = 0;
            while i < N {
                node_set |= 1 << rv[i];
                i += 1;
            }
            node_set = !node_set;
            node_set = node_set & ((1 << size) - 1);

            let mut curr_full_overlaps = !0;
            while node_set != 0 {
                let node = node_set.trailing_zeros() as usize;

                node_set &= !(1 << node);
                curr_full_overlaps &= !adj[node];
            }
            curr_full_overlaps = curr_full_overlaps & ((1 << e_count) - 1);

            full_overlaps[*edge_count] = curr_full_overlaps;
            *edge_count += 1;
            return;
        }

        while pos < size - (N - level - 1) {
            rv[level] = pos as u8;
            build_full_overlaps(pos + 1, level + 1, rv, adj, full_overlaps, edge_count);
            pos += 1;
        }
    }

    const fn build_node_map<const N: usize>(
        mut pos: usize,
        level: usize,
        rv: &mut [u8; N],
        node_map: &mut [u8; e_count],
        edge_count: &mut usize,
    ) {
        if level == N {
            let mut map: u8 = 0;
            let mut i = 0;
            while i < N {
                map |= 1 << rv[i];
                i += 1;
            }
            node_map[*edge_count] = map;
            *edge_count += 1;
            return;
        }

        while pos < size - (N - level - 1) {
            rv[level] = pos as u8;
            build_node_map(pos + 1, level + 1, rv, node_map, edge_count);
            pos += 1;
        }
    }

    // building compressed adj list
    let mut adj_raw: [e_bitset; size] = [0; size];
    let mut edge_count = 0;
    seq!(N in 2..=size {
        let mut bucket~N = [0; N];
        build_adj(0, 0, &mut bucket~N, &mut adj_raw, &mut edge_count);
    });

    // building full overlaps
    let mut edge_count = 0;
    let mut full_overlaps_raw = [0 as e_bitset; e_count];
    seq!(N in 2..=size {
        let mut bucket~N = [0; N];
        build_full_overlaps(0, 0, &mut bucket~N,  &adj_raw, &mut full_overlaps_raw, &mut edge_count);
    });

    // building partial overlaps
    let mut part_overlaps_raw = [0 as e_bitset; e_count];
    let mut n = 0;
    while n < size {
        let edges = adj_raw[n];
        let mut edges_iter = adj_raw[n];

        while edges_iter != 0 {
            let edge = edges_iter.trailing_zeros() as usize;
            edges_iter &= !(1 << edge);
            part_overlaps_raw[edge] |= edges;
        }
        n += 1;
    }

    // building node map
    let mut node_map_raw: [u8; e_count] = [0; e_count];
    let mut edge_count = 0;
    seq!(N in 2..=size {
        let mut bucket~N = [0; N];
        build_node_map(0, 0, &mut bucket~N, &mut node_map_raw, &mut edge_count);
    });

    // Convert raw bitsets into our new wrapping abstractions
    let mut adj = [struct_name { edges: 0 }; size];
    let mut i = 0;
    while i < size {
        adj[i] = struct_name { edges: adj_raw[i] };
        i += 1;
    }

    let mut full_overlaps = [struct_name { edges: 0 }; e_count];
    let mut i = 0;
    while i < e_count {
        full_overlaps[i] = struct_name {
            edges: full_overlaps_raw[i],
        };
        i += 1;
    }

    let mut part_overlaps = [struct_name { edges: 0 }; e_count];
    let mut i = 0;
    while i < e_count {
        part_overlaps[i] = struct_name {
            edges: part_overlaps_raw[i],
        };
        i += 1;
    }

    let mut node_map = [CompressedNodeSet { nodes: 0 }; e_count];
    let mut i = 0;
    while i < e_count {
        node_map[i] = CompressedNodeSet {
            nodes: node_map_raw[i],
        };
        i += 1;
    }

    (adj, full_overlaps, part_overlaps, node_map)
}

