use seq_macro::seq;
use std::{
    fmt::Display,
    ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, Index, Mul, Shl},
};

use num_traits::{AsPrimitive, FromPrimitive, One, PrimInt, Zero};

use crate::fingerprint::{Fingerprint2, Fingerprint3, Fingerprint4, Fingerprint5};

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

macro_rules! define_compact_motif {
    ($ct:ty, $order:literal, $max_edge_count:literal, $fingerprint: ty) => {
        impl CompactMotifConfigurator for CompactMotif<$order> {
            type ContainerType = $ct;
            type FingerprintType = $fingerprint;

            const MAX_EDGE_COUNT: usize = $max_edge_count;
            const SIZE: usize = $order;

            const CONTAINER_ZERO: Self::ContainerType = 0;
            const CONTAINER_ONE: Self::ContainerType = 1;

            const ZERO: Self = Self::new(Self::CONTAINER_ZERO);
            const ONE: Self = Self::new(Self::CONTAINER_ONE);

            type AdjType = [Self; $order];
            type FullOverlapsType = [Self; $max_edge_count];
            type PartOverlapsType = [Self; $max_edge_count];
            type NodeMapType = [CompressedNodeSet; $max_edge_count];

            // Replaced the todo!() calls with the newly constructed const evaluator functions
            const ADJ: Self::AdjType = Self::get_adj_const_bitmasks();
            const FULL_OVERLAPS: Self::FullOverlapsType = Self::get_full_overlaps_const_bitmasks();
            const PART_OVERLAPS: Self::PartOverlapsType = Self::get_part_overlaps_const_bitmasks();
            const NODE_MAP: Self::NodeMapType = Self::get_node_map_const_bitmasks();
        }

        impl CompactMotif<$order> {
            pub const fn bitor(self, other: Self) -> Self {
                Self::new(self.container | other.container)
            }

            pub const fn bitand(self, other: Self) -> Self {
                Self::new(self.container & other.container)
            }

            pub const fn const_bitor_assign(&mut self, other: Self) {
                self.container |= other.container;
            }

            pub const fn bitand_assign(&mut self, other: Self) {
                self.container &= other.container;
            }

            pub const fn shl(self, rhs: usize) -> Self {
                Self::new(self.container << rhs)
            }

            pub const fn shl_assign(&mut self, rhs: usize) {
                self.container <<= rhs
            }

            pub const fn shr(self, rhs: usize) -> Self {
                Self::new(self.container >> rhs)
            }

            pub const fn shr_assign(&mut self, rhs: usize) {
                self.container >>= rhs
            }

            pub const fn adj() -> <Self as CompactMotifConfigurator>::AdjType {
                Self::ADJ
            }
            pub const fn full_overlaps() -> <Self as CompactMotifConfigurator>::FullOverlapsType {
                Self::FULL_OVERLAPS
            }
            pub const fn part_overlaps() -> <Self as CompactMotifConfigurator>::PartOverlapsType {
                Self::PART_OVERLAPS
            }
            pub const fn node_map() -> <Self as CompactMotifConfigurator>::NodeMapType {
                Self::NODE_MAP
            }


            // -------------------------------------------------------------
            // Adjacency Matrix Builder
            // -------------------------------------------------------------
            const fn build_adj_rec<const M: usize>(
                mut pos: usize,
                level: usize,
                rv: &mut [u8; M],
                adj: &mut [Self; $order],
                edge_count: &mut usize,
            ) {
                if level == M {
                    let mut i = 0;
                    while i < M {
                        adj[rv[i] as usize].const_bitor_assign(Self::ONE.shl(*edge_count));
                        i += 1;
                    }

                    *edge_count += 1;
                    return;
                }
                while pos < $order - (M - level - 1) {
                    rv[level] = pos as u8;
                    Self::build_adj_rec::<M>(pos + 1, level + 1, rv, adj, edge_count);
                    pos += 1;
                }
            }

            pub const fn get_adj_const_bitmasks() -> <Self as CompactMotifConfigurator>::AdjType {
                let mut adj_raw = [Self::ZERO; $order];
                let mut edge_count = 0;
                seq!(M in 2..=$order {
                    let mut bucket~M = [0; M];
                    Self::build_adj_rec::<M>(0, 0, &mut bucket~M, &mut adj_raw, &mut edge_count);
                });
                adj_raw
            }

            // -------------------------------------------------------------
            // Full Overlaps Builder
            // -------------------------------------------------------------
            const fn build_full_overlaps_rec<const M: usize>(
                mut pos: usize,
                level: usize,
                rv: &mut [u8; M],
                adj: &[Self; $order],
                full_overlaps: &mut [Self; $max_edge_count],
                edge_count: &mut usize,
            ) {
                if level == M {
                    let mut node_set: u8 = 0;
                    let mut i = 0;
                    while i < M {
                        node_set |= 1 << rv[i];
                        i += 1;
                    }
                    node_set = !node_set;
                    node_set = node_set & ((1 << $order) - 1);

                    let mut curr_full_overlaps: $ct = (!0) as $ct;
                    while node_set != 0 {
                        let node = node_set.trailing_zeros() as usize;

                        node_set &= !(1 << node);
                        curr_full_overlaps &= !adj[node].container;
                    }

                    // Safely truncate bits avoiding bit-shift overflow
                    curr_full_overlaps = curr_full_overlaps & (((1 as $ct) << $max_edge_count) - 1);

                    full_overlaps[*edge_count] = Self::new(curr_full_overlaps);
                    *edge_count += 1;
                    return;
                }

                while pos < $order - (M - level - 1) {
                    rv[level] = pos as u8;
                    Self::build_full_overlaps_rec::<M>(pos + 1, level + 1, rv, adj, full_overlaps, edge_count);
                    pos += 1;
                }
            }

            pub const fn get_full_overlaps_const_bitmasks() -> <Self as CompactMotifConfigurator>::FullOverlapsType {
                let mut full_overlaps_raw = [Self::ZERO; $max_edge_count];
                let mut edge_count = 0;
                let adj_raw = Self::get_adj_const_bitmasks();

                seq!(M in 2..=$order {
                    let mut bucket~M = [0; M];
                    Self::build_full_overlaps_rec::<M>(0, 0, &mut bucket~M, &adj_raw, &mut full_overlaps_raw, &mut edge_count);
                });

                full_overlaps_raw
            }

            // -------------------------------------------------------------
            // Node Map Builder
            // -------------------------------------------------------------
            const fn build_node_map_rec<const M: usize>(
                mut pos: usize,
                level: usize,
                rv: &mut [u8; M],
                node_map: &mut [CompressedNodeSet; $max_edge_count],
                edge_count: &mut usize,
            ) {
                if level == M {
                    let mut map: u8 = 0;
                    let mut i = 0;
                    while i < M {
                        map |= 1 << rv[i];
                        i += 1;
                    }
                    node_map[*edge_count] = CompressedNodeSet { nodes: map };
                    *edge_count += 1;
                    return;
                }

                while pos < $order - (M - level - 1) {
                    rv[level] = pos as u8;
                    Self::build_node_map_rec::<M>(pos + 1, level + 1, rv, node_map, edge_count);
                    pos += 1;
                }
            }

            pub const fn get_node_map_const_bitmasks() -> <Self as CompactMotifConfigurator>::NodeMapType {
                let mut node_map_raw = [CompressedNodeSet { nodes: 0 }; $max_edge_count];
                let mut edge_count = 0;

                seq!(M in 2..=$order {
                    let mut bucket~M = [0; M];
                    Self::build_node_map_rec::<M>(0, 0, &mut bucket~M, &mut node_map_raw, &mut edge_count);
                });

                node_map_raw
            }

            // -------------------------------------------------------------
            // Partial Overlaps Builder
            // -------------------------------------------------------------
            pub const fn get_part_overlaps_const_bitmasks() -> <Self as CompactMotifConfigurator>::PartOverlapsType {
                let mut part_overlaps_raw = [Self::ZERO; $max_edge_count];
                let adj_raw = Self::get_adj_const_bitmasks();
                let mut n = 0;

                while n < $order {
                    let edges = adj_raw[n].container;
                    let mut edges_iter = edges;

                    while edges_iter != 0 {
                        let edge = edges_iter.trailing_zeros() as usize;
                        // Avoid overflow by casting to $ct before shifting
                        edges_iter &= !((1 as $ct) << edge);
                        part_overlaps_raw[edge].container |= edges;
                    }
                    n += 1;
                }
                part_overlaps_raw
            }

        }

        impl CMAssociated for $fingerprint {
            type CMType = CompactMotif<$order>;
        }
    };
}

pub trait CompactMotifConfigurator
where
    Self::ContainerType: Eq
        + Zero
        + PrimInt
        + AsPrimitive<usize>
        + One
        + BitAndAssign
        + BitOrAssign
        + Div
        + Mul
        + Add
        + Shl<Output = Self::ContainerType>
        + FromPrimitive,
    Self::AdjType: IntoIterator<Item = Self> + Index<usize, Output = Self>,
    Self::FullOverlapsType: IntoIterator<Item = Self> + Index<usize, Output = Self>,
    Self::PartOverlapsType: IntoIterator<Item = Self> + Index<usize, Output = Self>,
    Self::NodeMapType:
        IntoIterator<Item = CompressedNodeSet> + Index<usize, Output = CompressedNodeSet>,
{
    type ContainerType;
    type FingerprintType;

    const MAX_EDGE_COUNT: usize;
    const SIZE: usize;

    const CONTAINER_ZERO: Self::ContainerType;
    const CONTAINER_ONE: Self::ContainerType;

    const ZERO: Self;
    const ONE: Self;

    type AdjType;
    type FullOverlapsType;
    type PartOverlapsType;
    type NodeMapType;

    const ADJ: Self::AdjType = todo!();
    // Self::generate_adj_const_bitmasks();
    const FULL_OVERLAPS: Self::FullOverlapsType = todo!();
    const PART_OVERLAPS: Self::PartOverlapsType = todo!();
    const NODE_MAP: Self::NodeMapType = todo!();
}

pub trait CMAssociated {
    type CMType;
}

#[derive(Copy, Clone)]
pub struct CompactMotif<const N: usize>
where
    Self: CompactMotifConfigurator,
{
    container: <Self as CompactMotifConfigurator>::ContainerType,
}

impl<const N: usize> CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
    pub const fn new(container: <Self as CompactMotifConfigurator>::ContainerType) -> Self {
        Self { container }
    }

    /// Yields the number of each edge contained in the motif
    pub fn iter_edges(&self) -> CompactMotifEdgeIter<N> {
        self.into_iter()
    }

    /// Yields the set of nodes that each edge in the motifs has
    pub fn iter_nodes(&self) -> CompactMotifNodeIter<N> {
        CompactMotifNodeIter {
            remaining_edges: self.container,
        }
    }

    pub fn edge_count(&self) -> u32 {
        self.container.count_ones()
    }

    pub fn add_edge(&mut self, edge_number: usize) {
        self.container |= Self::CONTAINER_ONE << edge_number;
    }

    pub fn remove_edge(&mut self, edge_number: usize) {
        self.container &= !(Self::CONTAINER_ONE << edge_number);
    }

    pub fn part_ovelaps(&self, edge_number: usize) -> Self {
        *self & Self::PART_OVERLAPS[edge_number]
    }

    pub fn full_ovelaps(&self, edge_number: usize) -> Self {
        *self & Self::FULL_OVERLAPS[edge_number]
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_zero()
    }

    pub const fn one() -> Self {
        <Self as CompactMotifConfigurator>::ZERO
    }

    pub const fn zero() -> Self {
        <Self as CompactMotifConfigurator>::ONE
    }

    pub fn fingerprint(&self) -> <Self as CompactMotifConfigurator>::FingerprintType
    where
        <Self as CompactMotifConfigurator>::FingerprintType: From<Self>,
    {
        (*self).into()
    }

    pub fn enum_labelings<F>(show_progress: bool, mut f: F)
    where
        F: FnMut(Self),
    {
        let is_connected = |motif: Self| -> bool {
            if motif.is_empty() {
                return false;
            }

            // 1. Quick Spanning Check: Ensure the motif covers all 'size' nodes
            let mut covered_nodes = 0u8;
            for e in motif {
                covered_nodes |= Self::NODE_MAP[e].nodes;
            }

            if covered_nodes != (1 << Self::SIZE) - 1 {
                return false;
            }

            // 2. Connectivity Check: Run a bitwise BFS to ensure all edges form 1 component
            let first_edge = motif.iter_edges().next();
            if first_edge.is_none() {
                return false;
            }
            let first_edge = first_edge.unwrap();

            // let first_edge = motif.trailing_zeros() as usize;
            let mut visited_edges = Self::CONTAINER_ONE << first_edge;
            let mut queue = Self::CONTAINER_ONE << first_edge;

            while !queue.is_zero() {
                let e = queue.trailing_zeros() as usize;
                queue &= queue - Self::CONTAINER_ONE;

                // Get neighbors that are in the motif but haven't been visited yet
                // let x = motif.part_ovelaps(e);
                let neighbors = motif.part_ovelaps(e).container & !visited_edges;
                visited_edges |= neighbors;
                queue |= neighbors;
            }

            // If we visited every edge in the motif, it is fully connected
            visited_edges == motif.container
        };

        let motif_count = (1 << Self::MAX_EDGE_COUNT) as usize;

        let block_size = motif_count / 100;
        let mut curr_progress = 0;
        let mut block_progress = 0;

        for i in 0..motif_count {
            // print_motif(&motif);
            let motif = Self::new(
                <Self as CompactMotifConfigurator>::ContainerType::from_usize(i).unwrap(),
            );
            if is_connected(motif) {
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
            let mut nodes = Vec::with_capacity(Self::SIZE);
            for n in Self::NODE_MAP[e] {
                nodes.push(n);
            }
            edges.push(nodes);
        }
        edges
    }
}

impl<const N: usize> BitAnd for CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            container: self.container & rhs.container,
        }
    }
}

impl<const N: usize> BitAndAssign for CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
    fn bitand_assign(&mut self, rhs: Self) {
        self.container &= rhs.container;
    }
}

impl<const N: usize> BitOr for CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            container: self.container | rhs.container,
        }
    }
}

impl<const N: usize> BitOrAssign for CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.container |= rhs.container;
    }
}

impl<const N: usize> Shl<usize> for CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self {
            container: self.container << rhs,
        }
    }
}

impl<const N: usize> Display for CompactMotif<N>
where
    Self: CompactMotifConfigurator,
{
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

pub struct CompactMotifEdgeIter<const N: usize>
where
    CompactMotif<N>: CompactMotifConfigurator,
{
    remaining_edges: <CompactMotif<N> as CompactMotifConfigurator>::ContainerType,
}

impl<const N: usize> Iterator for CompactMotifEdgeIter<N>
where
    CompactMotif<N>: CompactMotifConfigurator,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // type T = ;
        if self.remaining_edges.is_zero() {
            None
        } else {
            // 1. Grab the index of the lowest set bit
            let index = self.remaining_edges.trailing_zeros() as usize;

            // 2. Clear the lowest set bit blazingly fast via bitwise AND
            self.remaining_edges &= self.remaining_edges
                - <CompactMotif<N> as CompactMotifConfigurator>::ContainerType::one();

            Some(index)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.remaining_edges.count_ones() as usize;
        (count, Some(count))
    }
}

impl<const N: usize> IntoIterator for CompactMotif<N>
where
    CompactMotif<N>: CompactMotifConfigurator,
{
    type Item = usize;
    type IntoIter = CompactMotifEdgeIter<N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        CompactMotifEdgeIter {
            remaining_edges: self.container,
        }
    }
}

impl<'a, const N: usize> IntoIterator for &'a CompactMotif<N>
where
    CompactMotif<N>: CompactMotifConfigurator,
{
    type Item = usize;
    type IntoIter = CompactMotifEdgeIter<N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        CompactMotifEdgeIter {
            remaining_edges: self.container,
        }
    }
}

pub struct CompactMotifNodeIter<const N: usize>
where
    CompactMotif<N>: CompactMotifConfigurator,
{
    remaining_edges: <CompactMotif<N> as CompactMotifConfigurator>::ContainerType,
}

impl<const N: usize> Iterator for CompactMotifNodeIter<N>
where
    CompactMotif<N>: CompactMotifConfigurator,
{
    type Item = CompressedNodeSet;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // type T = ;
        if self.remaining_edges.is_zero() {
            None
        } else {
            // 1. Grab the index of the lowest set bit
            let index = self.remaining_edges.trailing_zeros() as usize;

            // 2. Clear the lowest set bit blazingly fast via bitwise AND
            self.remaining_edges &= self.remaining_edges - CompactMotif::<N>::CONTAINER_ONE;
            Some(CompactMotif::<N>::NODE_MAP[index])
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.remaining_edges.count_ones() as usize;
        (count, Some(count))
    }
}

define_compact_motif!(u8, 2, 1, Fingerprint2);
define_compact_motif!(u8, 3, 4, Fingerprint3);
define_compact_motif!(u16, 4, 11, Fingerprint4);
define_compact_motif!(u32, 5, 26, Fingerprint5);
